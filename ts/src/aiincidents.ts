/**
 * The 2026 AI incident timeline.
 *
 * The entries are the page: every date, headline, number and source lives in
 * the markdown, so the post reads top to bottom with this script missing. What
 * the script adds is the panel beside it — whichever entry the reader is level
 * with, and the totals of everything above it, counting up as they scroll —
 * and the three pictures in `aiincidents/charts.ts`, which are drawn from the
 * same entries rather than from a second copy of the data.
 *
 * The running sums are computed here rather than written into the markup
 * because they depend on where the reader is, and because a tally typed by
 * hand into every entry would be wrong by the second edit. Each entry carries
 * only its own contribution, as `data-tally-<key>`.
 */

import { followHeader } from "./shared/sticky.js"
import {
  drawRuler,
  drawShape,
  drawSilence,
  legend,
  type Mark,
  type Plotted,
} from "./aiincidents/charts.js"

const TIMELINE = "ai-incidents-2026"

/** How long a number takes to travel from its old value to its new one. */
const DURATION = 420

/**
 * A one percent band across the middle of whatever the entries scroll inside.
 * The entry crossing it is the one the reader is looking at; between two
 * entries nothing crosses it and the last answer stands. Laid down the page
 * the band is a line across the viewport; laid across the track it is a line
 * down the track, so the same idea needs the margins in a different order.
 */
const BANDS: Record<Layout, string> = {
  vertical: "-45% 0px -54% 0px",
  horizontal: "0px -54% 0px -45%",
}

/** Where the reader's choice of layout is kept between visits. */
const STORED = "ai-incidents-2026:layout"

type Layout = "vertical" | "horizontal"

function isLayout(value: string | undefined): value is Layout {
  return value === "vertical" || value === "horizontal"
}

/** What the two coloured series are, said once per picture. */
const KEYS: readonly [string, string][] = [
  ["is-agents", "a model, unasked"],
  ["is-accomplices", "a person using one"],
]

/** The ruler has a third mark the charts do not. */
const RULER_KEYS: readonly [string, string][] = [...KEYS, ["is-told", "when it came out"]]

interface Tally {
  /** Read off an entry as `data-tally-<key>`. */
  key: string
  label: string
  /** What every entry adds, for a count that is only the entries themselves. */
  each?: number
  /** For a count the markup never states because it follows from the dates. */
  derive?: (element: HTMLElement) => number
}

const TALLIES: readonly Tally[] = [
  // How many entries there have been is already on the panel, as `n / 40`.
  {
    key: "firsts",
    label: "firsts of their kind",
    derive: (element) => (element.dataset.first === undefined ? 0 : 1),
  },
  { key: "intrusions", label: "third parties an agent got into" },
  { key: "campaigns", label: "campaigns by people using one" },
  { key: "quiet", label: "days nobody knew, added up", derive: gapDays },
  { key: "legal", label: "suits, inquiries, filings" },
  { key: "deaths", label: "deaths named in filings" },
]

interface Figure {
  value: number
  label: string
  /** As many decimals as the markup wrote, so 29.2% does not become 29. */
  decimals: number
}

interface Entry extends Plotted {
  /** The entry's own headline number, where it has one. */
  figure?: Figure
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
  /** Moves the ruler's lit tick to the entry being read. */
  mark: Mark
  position: HTMLElement
}

const still = window.matchMedia("(prefers-reduced-motion: reduce)")

function textOf(root: ParentNode, selector: string): string {
  return root.querySelector(selector)?.textContent?.trim() ?? ""
}

/** A written date as a whole day number, or nothing if it is not one. */
function dayOf(written: string | undefined): number | undefined {
  if (written === undefined) return undefined

  const parsed = Date.parse(written)
  return Number.isNaN(parsed) ? undefined : parsed / 86_400_000
}

/** The same, for a `<time datetime>` that may not be there at all. */
function day(element: Element | null): number | undefined {
  return dayOf(element?.getAttribute("datetime") ?? undefined)
}

/**
 * How long the thing went unknown: the days from when it happened to when it
 * was made public. An entry reported as it happened contributes nothing, and
 * so does one dated to a report rather than to the event, which is why this is
 * derived from the dates in the markup instead of being typed in beside them.
 */
function gapDays(element: HTMLElement): number {
  const happened = day(element.querySelector(".ait-at"))
  const disclosed = day(element.querySelector(".ait-disclosed time"))
  if (happened === undefined || disclosed === undefined) return 0

  return Math.max(Math.round(disclosed - happened), 0)
}

/**
 * The year keeps producing things that had not happened before, so an entry
 * that was one says which beside its date.
 */
function showFirst(element: HTMLElement): void {
  const claim = element.dataset.first
  const meta = element.querySelector(".ait-meta")
  if (claim === undefined || meta === null) return

  const badge = document.createElement("span")
  badge.className = "ait-first"
  badge.textContent = claim
  meta.append(badge)
}

/** Puts the gap in the entry too, beside the two dates it comes from. */
function showGap(element: HTMLElement, days: number): void {
  const when = element.querySelector(".ait-when")
  if (when === null || days === 0) return

  const span = document.createElement("span")
  span.className = "ait-gap"
  span.textContent = `${days.toLocaleString()} days unknown`
  when.append(span)
}

/** What this entry adds to one tally, which for most entries is nothing. */
function contribution(element: HTMLElement, tally: Tally): number {
  if (tally.each !== undefined) return tally.each
  if (tally.derive !== undefined) return tally.derive(element)

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
    showGap(element, gapDays(element))
    showFirst(element)

    return {
      element,
      title: textOf(element, "h3"),
      date: textOf(element, ".ait-at"),
      kind: textOf(element, ".ait-kind"),
      at: day(element.querySelector(".ait-at")) ?? 0,
      until: dayOf(element.dataset.until),
      disclosed: day(element.querySelector(".ait-disclosed time")),
      short: element.dataset.short,
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

function readDash(element: HTMLElement, entries: readonly Entry[]): Dash | undefined {
  const date = element.querySelector<HTMLElement>(".ait-dash-date")
  const kind = element.querySelector<HTMLElement>(".ait-dash-kind")
  const figure = element.querySelector<HTMLElement>(".ait-dash-figure")
  const figureValue = element.querySelector<HTMLElement>(".ait-figure-value")
  const figureLabel = element.querySelector<HTMLElement>(".ait-figure-label")
  const tallies = element.querySelector<HTMLElement>(".ait-tallies")
  const ruler = element.querySelector<HTMLElement>(".ait-ruler")
  const position = element.querySelector<HTMLElement>(".ait-position")

  if (
    date === null ||
    kind === null ||
    figure === null ||
    figureValue === null ||
    figureLabel === null ||
    tallies === null ||
    ruler === null ||
    position === null
  ) {
    return undefined
  }

  const counters = buildTallies(tallies)
  const mark = drawRuler(ruler, entries)
  legend(ruler, RULER_KEYS)

  return {
    date,
    kind,
    figure,
    figureValue: counter(figureValue, 0),
    figureLabel,
    counters,
    mark,
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

  dash.mark(index)
  dash.position.textContent = `${index + 1} / ${entries.length}`

  for (const [position, other] of entries.entries()) {
    other.element.classList.toggle("is-active", position === index)
    other.element.classList.toggle("is-past", position < index)
  }
}

/**
 * Watches for the entry crossing the band and says so. Returns the way to
 * stop, because changing the layout changes both the thing the entries scroll
 * inside and which way the band lies, and an observer cannot be told either
 * after the fact.
 */
function watch(
  track: HTMLElement,
  entries: readonly Entry[],
  how: Layout,
  onActive: (index: number) => void,
): () => void {
  const observer = new IntersectionObserver(
    (records) => {
      for (const record of records) {
        if (!record.isIntersecting) continue

        const index = entries.findIndex((entry) => entry.element === record.target)
        if (index !== -1) onActive(index)
      }
    },
    {
      root: how === "horizontal" ? track : null,
      rootMargin: BANDS[how],
      threshold: 0,
    },
  )

  for (const entry of entries) observer.observe(entry.element)
  return () => observer.disconnect()
}

function remembered(): Layout {
  try {
    const kept = localStorage.getItem(STORED) ?? undefined
    return isLayout(kept) ? kept : "vertical"
  } catch {
    // Storage can be denied outright, and a layout is not worth failing over.
    return "vertical"
  }
}

function remember(how: Layout): void {
  try {
    localStorage.setItem(STORED, how)
  } catch {
    /* as above */
  }
}

/**
 * The two layouts, and the buttons that choose between them. The buttons are
 * in the markup but hidden until `is-live`, because without this script they
 * would be a pair of controls that do nothing.
 */
function layouts(timeline: HTMLElement, onPick: (how: Layout, byHand: boolean) => void): void {
  const buttons = [...timeline.querySelectorAll<HTMLButtonElement>(".ait-mode")]

  const settle = (how: Layout, byHand: boolean): void => {
    for (const button of buttons) {
      button.setAttribute("aria-pressed", String(button.dataset.layout === how))
    }
    onPick(how, byHand)
  }

  for (const button of buttons) {
    button.addEventListener("click", () => {
      const how = button.dataset.layout
      if (!isLayout(how)) return

      remember(how)
      settle(how, true)
    })
  }

  settle(remembered(), false)
}

/**
 * The figure slot holds the only number that does not grow monotonically, so
 * it starts at the first entry's rather than counting up to it from zero.
 */
function start(timeline: HTMLElement): void {
  const dashElement = timeline.querySelector<HTMLElement>(".ait-dash")
  const track = timeline.querySelector<HTMLElement>(".ait-track")
  if (dashElement === null || track === null) return

  const entries = readEntries(track)
  if (entries.length === 0) return

  const dash = readDash(dashElement, entries)
  if (dash === undefined) return

  const first = entries[0].figure
  if (first !== undefined) {
    dash.figureValue.from = first.value
    dash.figureValue.to = first.value
    dash.figureValue.decimals = first.decimals
    paint(dash.figureValue, first.value)
  }

  for (const [id, draw] of [
    ["ait-shape", drawShape],
    ["ait-silence", drawSilence],
  ] as const) {
    const figure = document.getElementById(id)
    if (figure !== null) {
      draw(figure, entries)
      legend(figure, KEYS)
    }
  }

  followHeader()
  timeline.classList.add("is-live")
  show(dash, entries, 0)

  let active = 0
  let stop = (): void => {}

  layouts(timeline, (how, byHand) => {
    timeline.classList.toggle("is-horizontal", how === "horizontal")
    stop()
    stop = watch(track, entries, how, (index) => {
      if (index === active) return
      active = index
      show(dash, entries, index)
    })

    // Changing the layout should not also lose the reader's place.
    if (byHand) {
      entries[active].element.scrollIntoView({ block: "center", inline: "center" })
    }
  })
}

const timeline = document.getElementById(TIMELINE)
if (timeline !== null) start(timeline)
