/**
 * The movie lists: search links beside every film, and a wheel that picks one
 * not yet watched.
 *
 * What is on each list, and whether it has been watched, is read out of
 * `content/notes/movies.md` by `site movies` (`tools/site/src/movies.rs`)
 * into `movies.json` beside this page; this only puts it on the page.
 */

interface Movie {
  /** Absent for a line that names no film, such as a `BREAK` week. */
  label?: string
  watched: boolean
  imdb?: string
  letterboxd?: string
}

interface MovieList {
  heading: string
  /** Whether it has anything left unwatched to spin for. */
  pickable: boolean
  /** Every task-list line under the heading, in page order. */
  movies: Movie[]
}

const timeout = 0.25

function randomIndex(count: number): number {
  return crypto.getRandomValues(new Uint32Array(1))[0] % count
}

function link(text: string, href: string): HTMLAnchorElement {
  const anchor = document.createElement("a")
  anchor.href = href
  anchor.textContent = text
  anchor.target = "_blank"
  anchor.rel = "noopener"
  return anchor
}

/** A film's two search links, for a list line or the wheel's pick. */
function searchLinks(movie: Movie): HTMLSpanElement {
  const links = document.createElement("span")
  links.className = "movie-links"
  if (movie.imdb !== undefined && movie.letterboxd !== undefined) {
    links.append(link("imdb", movie.imdb), " · ", link("letterboxd", movie.letterboxd))
  }
  return links
}

function spin(candidates: Movie[], button: HTMLButtonElement): void {
  document.getElementById("wheel")?.remove()
  document.getElementById("result")?.remove()

  const canvas = document.createElement("canvas")
  canvas.id = "wheel"
  canvas.width = 500
  canvas.height = 500
  button.insertAdjacentElement("afterend", canvas)

  const wheel = new SpinningWheel(
    "wheel",
    candidates.map((movie) => movie.label ?? ""),
  )
  wheel.drawWheel()
  wheel.spin()

  setTimeout(() => {
    const pick = candidates[randomIndex(candidates.length)]
    const result = document.createElement("h2")
    result.id = "result"
    result.textContent = "random movie: "
    result.append(pick.label ?? "", " ", searchLinks(pick))
    canvas.insertAdjacentElement("afterend", result)
  }, timeout * 1000)
}

/** The task-list items between a heading and the next one, in page order. */
function itemsUnder(heading: HTMLHeadingElement): HTMLLIElement[] {
  const items: HTMLLIElement[] = []
  for (
    let node = heading.nextElementSibling;
    node !== null && node.tagName !== "H1";
    node = node.nextElementSibling
  ) {
    items.push(...node.querySelectorAll<HTMLLIElement>("li"))
  }
  return items
}

function decorate(heading: HTMLHeadingElement, list: MovieList): void {
  if (heading.dataset.moviesDecorated === "true") return
  heading.dataset.moviesDecorated = "true"

  const items = itemsUnder(heading)
  if (items.length !== list.movies.length) {
    console.error(
      `movies: "${list.heading}" has ${items.length} lines on the page and ${list.movies.length} in movies.json`,
    )
    return
  }

  list.movies.forEach((movie, index) => {
    if (movie.label === undefined) return
    items[index].append(searchLinks(movie))
  })

  if (list.pickable) {
    const candidates = list.movies.filter((movie) => !movie.watched && movie.label !== undefined)
    const button = document.createElement("button")
    button.textContent = "I'm feelin' lucky"
    button.addEventListener("click", () => spin(candidates, button))
    heading.insertAdjacentElement("afterend", button)
  }
}

async function start(): Promise<void> {
  const article = document.querySelector("article")
  if (article === null) return

  let lists: MovieList[]
  try {
    const response = await fetch("/notes/movies.json", { cache: "no-cache" })
    if (!response.ok) throw new Error(`${response.status}`)
    lists = (await response.json()) as MovieList[]
  } catch (error) {
    console.error("movies: could not read /notes/movies.json", error)
    return
  }

  const byHeading = new Map(lists.map((list) => [list.heading, list]))
  for (const heading of article.querySelectorAll<HTMLHeadingElement>("h1")) {
    const list = byHeading.get(heading.textContent?.trim() ?? "")
    if (list !== undefined) decorate(heading, list)
  }
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", () => void start(), { once: true })
} else {
  void start()
}

class SpinningWheel {
  private canvas: HTMLCanvasElement
  private ctx: CanvasRenderingContext2D
  private segments: string[]
  private angle: number = 0
  private baseSpeed: number = Math.PI / 16
  private currentSpinTime: number = 0
  public spinTime: number = 0

  constructor(canvasId: string, segments: string[]) {
    this.canvas = document.getElementById(canvasId) as HTMLCanvasElement
    this.ctx = this.canvas.getContext("2d")!
    this.segments = segments
  }

  drawWheel() {
    const { ctx, canvas, segments } = this
    const numSegments = segments.length
    const anglePerSeg = (2 * Math.PI) / numSegments
    const centerX = canvas.width / 2
    const centerY = canvas.height / 2
    const radius = Math.min(centerX, centerY)

    ctx.clearRect(0, 0, canvas.width, canvas.height)

    for (let i = 0; i < numSegments; i++) {
      const angle = this.angle + i * anglePerSeg
      ctx.beginPath()
      ctx.moveTo(centerX, centerY)
      ctx.arc(centerX, centerY, radius, angle, angle + anglePerSeg)
      ctx.closePath()
      ctx.fillStyle = i % 2 === 0 ? "#ffffff" : "#ffcc00"
      ctx.fill()

      ctx.save()
      ctx.translate(centerX, centerY)
      ctx.rotate(angle + anglePerSeg / 2)
      ctx.textAlign = "right"
      ctx.fillStyle = "#333"
      ctx.font = "14px Arial"
      ctx.fillText(segments[i], radius - 10, 0)
      ctx.restore()
    }

    this.angle += this.baseSpeed
  }

  spin() {
    this.spinTime = timeout
    this.currentSpinTime = 0
    this.rotateWheel()
  }

  rotateWheel() {
    this.currentSpinTime += 20
    if (this.currentSpinTime >= this.spinTime * 1000) {
      return
    }
    this.drawWheel()
    setTimeout(() => this.rotateWheel(), 20)
  }
}
