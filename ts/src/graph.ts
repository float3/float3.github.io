/**
 * Starting the Elm graph, and being the parts of the page it cannot be.
 *
 * The graph itself -- the neighbourhood, the force simulation, the drawing --
 * is `quartz-local/elm-graph/src/Main.elm`, compiled to `/js/elm.js`. What is
 * left here is what belongs to the browser rather than to the graph: reading
 * the content index, measuring the box, remembering which pages have been
 * visited, opening the whole-site graph, and handing a click back to Quartz's
 * router.
 *
 * The compiled Elm is fetched only where a graph is actually drawn. It is 54
 * KB, the sidebar it lives in is desktop-only, and the page this replaced
 * pulled d3 and pixi.js off a CDN on every page view whether it drew anything
 * or not.
 */

interface ElmPort<T> {
  send(value: T): void
}

interface ElmSubscription<T> {
  subscribe(handler: (value: T) => void): void
}

interface Size {
  width: number
  height: number
}

interface ElmApp {
  ports: {
    follow: ElmSubscription<string>
    failed: ElmSubscription<string>
    resized: ElmPort<Size>
    halt: ElmPort<null>
  }
}

/** One graph: the app, and the observer that keeps it the size of its box. */
interface Mount {
  app: ElmApp
  observer: ResizeObserver
}

interface ElmRuntime {
  Main: { init(config: { node: HTMLElement; flags: unknown }): ElmApp }
}

declare global {
  interface Window {
    Elm?: ElmRuntime
    spaNavigate?: (url: URL) => void
  }
}

/** What the graph needs out of one page of the content index. */
interface Entry {
  title?: string
  links?: string[]
  tags?: string[]
}

interface Page {
  id: string
  title: string
  links: string[]
  tags: string[]
}

/** The key Quartz's graph has always kept its visited pages under. */
const VISITED = "graph-visited"

const mounts = new WeakMap<HTMLElement, Mount>()

/**
 * Containers with a graph on them or on the way to one.
 *
 * Marked here rather than in `mounts` because starting one takes two fetches,
 * and Quartz announces the first page with a `nav` event of its own on top of
 * `DOMContentLoaded`: without a mark taken synchronously, both calls get past
 * the check and two apps end up drawing into the same box.
 */
const taken = new WeakSet<HTMLElement>()

/**
 * A slug as the graph names it: no slashes on either end and no `index` at the
 * end, so `blog/index` and `blog/` are the one node a reader takes them for.
 * The front page comes out as the empty string.
 */
function simplify(slug: string): string {
  const trimmed = slug.replace(/^\/+|\/+$/g, "")
  if (trimmed === "index") return ""
  return trimmed.replace(/\/index$/, "")
}

function basePath(): string {
  return document.body.dataset.basepath ?? ""
}

function currentSlug(): string {
  return simplify(document.body.dataset.slug ?? "")
}

function visited(): string[] {
  try {
    const stored: unknown = JSON.parse(localStorage.getItem(VISITED) ?? "[]")
    return Array.isArray(stored) ? stored.filter((slug) => typeof slug === "string") : []
  } catch {
    // Storage can be turned off, and then it throws rather than returning
    // nothing. A graph that does not remember where the reader has been is
    // still a graph.
    return []
  }
}

function remember(slug: string): void {
  try {
    const all = new Set(visited())
    all.add(slug)
    localStorage.setItem(VISITED, JSON.stringify([...all]))
  } catch {
    /* see above */
  }
}

let runtime: Promise<ElmRuntime> | null = null

/** The compiled Elm, loaded once and only where there is a graph to draw. */
function elm(): Promise<ElmRuntime> {
  runtime ??= new Promise<ElmRuntime>((resolve, reject) => {
    const script = document.createElement("script")
    script.src = `${basePath()}/js/elm.js`
    script.onload = () => {
      const loaded = window.Elm
      if (loaded === undefined) reject(new Error("/js/elm.js defined no Elm"))
      else resolve(loaded)
    }
    script.onerror = () => reject(new Error("/js/elm.js did not load"))
    document.head.append(script)
  })

  return runtime
}

let index: Promise<Page[]> | null = null

/**
 * The content index, which the search loads too, read once per page load and
 * cut down to what a graph is drawn from. Every page's full text is in there,
 * and handing that to Elm would mean copying a megabyte of prose across the
 * boundary in order to count links with it.
 */
function pages(): Promise<Page[]> {
  index ??= fetch(`${basePath()}/static/contentIndex.json`)
    .then((response) => {
      if (!response.ok) throw new Error(`contentIndex.json: ${response.status}`)
      return response.json() as Promise<Record<string, Entry>>
    })
    .then((data) =>
      Object.entries(data).map(([slug, entry]) => ({
        id: simplify(slug),
        title: entry.title ?? "",
        links: (entry.links ?? []).map(simplify),
        tags: entry.tags ?? [],
      })),
    )

  return index
}

function size(container: HTMLElement): Size {
  return {
    width: Math.max(container.clientWidth, 100),
    // A container that has not been laid out yet -- the whole-site graph,
    // before its dialog is opened -- measures zero, and a graph in a box of no
    // height is a graph nobody can see.
    height: Math.max(container.clientHeight, 250),
  }
}

function start(container: HTMLElement): void {
  taken.add(container)

  void Promise.all([elm(), pages()])
    .then(([loaded, sitePages]) => {
      let config: unknown = {}
      try {
        config = JSON.parse(container.dataset.cfg ?? "{}")
      } catch (error: unknown) {
        console.error("[graph] unreadable data-cfg", error)
      }

      // Elm replaces the node it is given rather than filling it, so it is
      // given one of its own. The container itself has to survive: it carries
      // the settings, and it is the box whose size is being watched.
      const root = document.createElement("div")
      container.replaceChildren(root)

      const app = loaded.Main.init({
        node: root,
        flags: {
          ...(config as Record<string, unknown>),
          slug: currentSlug(),
          base: basePath(),
          ...size(container),
          visited: visited(),
          pages: sitePages,
          // A reader who has asked for less movement gets the graph laid out
          // rather than the layout being performed at them.
          reducedMotion: window.matchMedia("(prefers-reduced-motion: reduce)").matches,
        },
      })

      app.ports.failed.subscribe((reason) => {
        console.error("[graph]", reason)
      })

      app.ports.follow.subscribe((id) => {
        remember(id)
        const url = new URL(`${basePath()}/${id}`, window.location.href)
        // Quartz's own router, so that a node behaves like a link in the
        // prose: no reload, and the back button still works.
        if (window.spaNavigate !== undefined) window.spaNavigate(url)
        else window.location.assign(url)
      })

      const observer = new ResizeObserver(() => {
        app.ports.resized.send(size(container))
      })
      observer.observe(container)

      mounts.set(container, { app, observer })
    })
    .catch((error: unknown) => {
      taken.delete(container)
      console.error("[graph]", error)
    })
}

/**
 * Mount the graph, and mount it again after a soft navigation.
 *
 * Quartz patches the document rather than replacing it, so the container is
 * the same element after a navigation as it was before -- but micromorph
 * patches it back into the empty div the new page's HTML has, taking the
 * drawing with it. The app that was rendering there is left holding a view
 * that is no longer in the page, so it is stopped and another one started
 * around the page the reader has arrived at.
 *
 * A container that still has its drawing has been through none of that, and is
 * left alone.
 */
function initialise(): void {
  for (const container of document.querySelectorAll<HTMLElement>(".elm-graph-container")) {
    const mount = mounts.get(container)

    if (mount !== undefined) {
      if (container.firstElementChild !== null) continue
      mount.app.ports.halt.send(null)
      mount.observer.disconnect()
      mounts.delete(container)
      taken.delete(container)
    }

    // The whole-site graph waits until someone opens it.
    if (!taken.has(container) && container.closest(".elm-graph-modal") === null) start(container)
  }
}

let listening = false

/**
 * The whole-site graph, drawn only when it is asked for: it is the same
 * simulation over every page rather than a dozen, and there is no reason to
 * run it behind a closed dialog.
 */
function expandable(): void {
  const button = document.querySelector<HTMLButtonElement>(".elm-graph-expand")
  const modal = document.querySelector<HTMLElement>(".elm-graph-modal")
  if (button === null || modal === null) return

  // Assigned rather than added: a soft navigation can hand back the same
  // elements, and adding would stack a second handler on each of them.
  button.onclick = () => {
    modal.hidden = false
    const container = modal.querySelector<HTMLElement>(".elm-graph-container")
    if (container !== null && !taken.has(container)) start(container)
  }

  modal.onclick = (event) => {
    if (event.target === modal) modal.hidden = true
  }

  // The document outlives every navigation, so this one is added once and left
  // alone -- and added rather than assigned, because `onkeydown` is a single
  // slot and the search box wants keys too.
  if (!listening) {
    listening = true
    document.addEventListener("keydown", (event) => {
      const open = document.querySelector<HTMLElement>(".elm-graph-modal")
      if (event.key === "Escape" && open !== null && !open.hidden) open.hidden = true
    })
  }
}

function boot(): void {
  initialise()
  expandable()
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", boot, { once: true })
} else {
  boot()
}

document.addEventListener("nav", boot)
