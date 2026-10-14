/**
 * The 2026 AI incident timeline.
 *
 * The entries are the page: every date, headline, number and source lives in
 * the markdown, so the post reads top to bottom with this script missing. What
 * the script adds is the panel beside it — whichever entry the reader is level
 * with, and the totals of everything above it, counting up as they scroll.
 *
 * The running sums are computed here rather than written into the markup
 * because they depend on where the reader is, and because a tally typed by hand
 * into twenty-three entries would be wrong by the second edit. Each entry
 * carries only its own contribution, as `data-tally-<key>`.
 */

const TIMELINE = "ai-incidents-2026"

/** How long a number takes to travel from its old value to its new one. */
const DURATION = 420

/**
 * A one percent band across the middle of the viewport. The entry crossing it
 * is the one the reader is looking at; between two entries nothing crosses it
 * and the last answer stands.
 */
const BAND = "-45% 0px -54% 0px"

interface Tally {
  /** Read off an entry as `data-tally-<key>`. */
  key: string
  label: string
  /** What every entry adds, for a count that is only the entries themselves. */
  each?: number
}

const TALLIES: readonly Tally[] = [
  { key: "events", label: "events so far", each: 1 },
  { key: "intrusions", label: "third parties an agent got into" },
  { key: "campaigns", label: "campaigns by people using one" },
  { key: "legal", label: "suits, inquiries, filings" },
  { key: "deaths", label: "deaths named in filings" },
]

interface Figure {
  value: number
  label: string
  /** As many decimals as the markup wrote, so 29.2% does not become 29. */
  decimals: number
}

interface Entry {
  element: HTMLElement
  /** The date as the markup writes it, so nothing here has to format one. */
  date: string
  kind: string
  /** The entry's own headline number, where it has one. */
  figure?: Figure
  /** Every tally as of this entry: the sums over it and everything above it. */
  totals: ReadonlyMap<string, number>
}

/** A number on the panel, and where it is on its way from one value to another. */
interface Counter {
  element: HTMLElement
  from: number
  to: number
  decimals: number
  /** Timestamp the current travel started, or undefined when it is at rest. */
  started?: number
  frame?: number
}

interface Dash {
  date: HTMLElement
  kind: HTMLElement
  figure: HTMLElement
  figureValue: Counter
  figureLabel: HTMLElement
  counters: Map<string, Counter>
  fill: HTMLElement
  position: HTMLElement
}

const still = window.matchMedia("(prefers-reduced-motion: reduce)")

function text(root: ParentNode, selector: string): string {
  return root.querySelector(selector)?.textContent?.trim() ?? ""
}

/** What this entry adds to one tally, which for most entries is nothing. */
function contribution(element: HTMLElement, tally: Tally): number {
  if (tally.each !== undefined) return tally.each

  const written = element.dataset[`tally${tally.key[0].toUpperCase()}${tally.key.slice(1)}`]
  const value = Number(written ?? 0)
  return Number.isFinite(value) ? value : 0
}

function figureOf(element: HTMLElement): Figure | undefined {
  const written = element.dataset.figure
  if (written === undefined) return undefined

  const value = Number(written)
  if (!Number.isFinite(value)) return undefined

  const point = written.indexOf(".")
  return {
    value,
    label: element.dataset.figureLabel ?? "",
    decimals: point === -1 ? 0 : written.length - point - 1,
  }
}

/** The entries in document order, each carrying the totals as of itself. */
function readEntries(track: HTMLElement): Entry[] {
  const running = new Map<string, number>(TALLIES.map((tally) => [tally.key, 0]))

  return [...track.querySelectorAll<HTMLElement>(".ait-entry")].map((element) => {
    for (const tally of TALLIES) {
      running.set(tally.key, (running.get(tally.key) ?? 0) + contribution(element, tally))
    }

    return {
      element,
      date: text(element, "time"),
      kind: text(element, ".ait-kind"),
      figure: figureOf(element),
      totals: new Map(running),
    }
  })
}

function counter(element: HTMLElement, value: number): Counter {
  const count: Counter = { element, from: value, to: value, decimals: 0 }
  paint(count, value)
  return count
}

/** Fills the panel's list of totals, one row per tally. */
function buildTallies(list: HTMLElement): Map<string, Counter> {
  const counters = new Map<string, Counter>()

  for (const tally of TALLIES) {
    const row = document.createElement("div")
    row.className = "ait-tally"

    const term = document.createElement("dt")
    term.textContent = tally.label
    const value = document.createElement("dd")

    row.append(term, value)
    list.append(row)
    counters.set(tally.key, counter(value, 0))
  }

  return counters
}

function readDash(element: HTMLElement): Dash | undefined {
  const date = element.querySelector<HTMLElement>(".ait-dash-date")
  const kind = element.querySelector<HTMLElement>(".ait-dash-kind")
  const figure = element.querySelector<HTMLElement>(".ait-dash-figure")
  const figureValue = element.querySelector<HTMLElement>(".ait-figure-value")
  const figureLabel = element.querySelector<HTMLElement>(".ait-figure-label")
  const tallies = element.querySelector<HTMLElement>(".ait-tallies")
  const fill = element.querySelector<HTMLElement>(".ait-progress-fill")
  const position = element.querySelector<HTMLElement>(".ait-position")

  if (
    date === null ||
    kind === null ||
    figure === null ||
    figureValue === null ||
    figureLabel === null ||
    tallies === null ||
    fill === null ||
    position === null
  ) {
    return undefined
  }

  return {
    date,
    kind,
    figure,
    figureValue: counter(figureValue, 0),
    figureLabel,
    counters: buildTallies(tallies),
    fill,
    position,
  }
}

function easeOut(fraction: number): number {
  return 1 - (1 - fraction) ** 3
}

function paint(count: Counter, value: number): void {
  count.element.textContent = value.toLocaleString(undefined, {
    minimumFractionDigits: count.decimals,
    maximumFractionDigits: count.decimals,
  })
}

/** Sends a counter to a new value, picking up from wherever it currently is. */
function retarget(count: Counter, to: number): void {
  if (count.to === to) {
    // Already there, but the figure it shares its slot with may have had a
    // different number of decimals.
    if (count.frame === undefined) paint(count, to)
    return
  }

  if (count.frame !== undefined) cancelAnimationFrame(count.frame)

  if (still.matches) {
    count.from = to
    count.to = to
    count.started = undefined
    count.frame = undefined
    paint(count, to)
    return
  }

  const now = performance.now()
  const elapsed = count.started === undefined ? DURATION : now - count.started
  const progress = Math.min(elapsed / DURATION, 1)

  count.from = count.from + (count.to - count.from) * easeOut(progress)
  count.to = to
  count.started = now

  const step = (time: number): void => {
    const fraction = Math.min((time - (count.started ?? time)) / DURATION, 1)
    paint(count, count.from + (count.to - count.from) * easeOut(fraction))

    if (fraction < 1) {
      count.frame = requestAnimationFrame(step)
      return
    }

    count.from = count.to
    count.started = undefined
    count.frame = undefined
  }

  count.frame = requestAnimationFrame(step)
}

function show(dash: Dash, entries: readonly Entry[], index: number): void {
  const entry = entries[index]

  dash.date.textContent = entry.date
  dash.kind.textContent = entry.kind

  dash.figure.hidden = entry.figure === undefined
  if (entry.figure !== undefined) {
    dash.figureValue.decimals = entry.figure.decimals
    retarget(dash.figureValue, entry.figure.value)
    dash.figureLabel.textContent = entry.figure.label
  }

  for (const [key, count] of dash.counters) {
    retarget(count, entry.totals.get(key) ?? 0)
  }

  const span = Math.max(entries.length - 1, 1)
  dash.fill.style.width = `${(index / span) * 100}%`
  dash.position.textContent = `${index + 1} / ${entries.length}`

  for (const [position, other] of entries.entries()) {
    other.element.classList.toggle("is-active", position === index)
    other.element.classList.toggle("is-past", position < index)
  }
}

/**
 * The figure slot holds the only number that does not grow monotonically, so
 * it starts at the first entry's rather than counting up to it from zero.
 */
function start(timeline: HTMLElement): void {
  const dashElement = timeline.querySelector<HTMLElement>(".ait-dash")
  const track = timeline.querySelector<HTMLElement>(".ait-track")
  if (dashElement === null || track === null) return

  const dash = readDash(dashElement)
  if (dash === undefined) return

  const entries = readEntries(track)
  if (entries.length === 0) return

  const first = entries[0].figure
  if (first !== undefined) {
    dash.figureValue.from = first.value
    dash.figureValue.to = first.value
    dash.figureValue.decimals = first.decimals
    paint(dash.figureValue, first.value)
  }

  timeline.classList.add("is-live")
  show(dash, entries, 0)

  let active = 0
  const observer = new IntersectionObserver(
    (records) => {
      for (const record of records) {
        if (!record.isIntersecting) continue

        const index = entries.findIndex((entry) => entry.element === record.target)
        if (index !== -1 && index !== active) {
          active = index
          show(dash, entries, index)
        }
      }
    },
    { rootMargin: BAND, threshold: 0 },
  )

  for (const entry of entries) observer.observe(entry.element)
}

const timeline = document.getElementById(TIMELINE)
if (timeline !== null) start(timeline)
