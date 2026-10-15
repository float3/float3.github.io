/**
 * The three pictures on the AI incident timeline.
 *
 * All of them are drawn from the entries in the markup, never from a second
 * copy of the data: an entry's date comes off its `<time datetime>`, its kind
 * off the chip beside it, its contribution to a running total off the same
 * `data-tally-<key>` the panel reads. So the charts cannot disagree with the
 * prose, and adding an entry redraws them. Every number in every chart is also
 * written out in the entry it came from, so none of this is the only way to
 * read a value.
 *
 * Two series carry colour and nothing else does: a model acting on its own,
 * and a person using one. The two hues are the reference categorical blue and
 * orange, stepped per theme, checked against both of this site's surfaces with
 * the palette validator rather than by eye — the site's own `--secondary` and
 * `--tertiary` fail it outright in the dark theme, 9.6 apart to normal vision
 * where 15 is the floor.
 *
 * Nothing here scales text with a viewBox. Each picture is drawn at the width
 * its container actually has and redrawn when that changes, so 10px type is
 * 10px type on a phone as well as on a desktop, and the margins reserved for
 * the labels mean what they say.
 */

const NS = "http://www.w3.org/2000/svg"

/** 2026, as the whole-day numbers the entries are parsed into. */
const YEAR_FROM = Date.parse("2026-01-01") / 86_400_000
const YEAR_TO = Date.parse("2026-12-31") / 86_400_000

const MONTHS = ["J", "F", "M", "A", "M", "J", "J", "A", "S", "O", "N", "D"]

/** Roughly how wide a character of the 10px label type is. */
const GLYPH = 5.4

/** What a chart needs to know about one entry. */
export interface Plotted {
  element: HTMLElement
  /** The entry's heading, for the tooltip. */
  title: string
  /** The date as the markup writes it. */
  date: string
  kind: string
  /** Whole-day number of the day it happened. */
  at: number
  /** Whole-day number of the day it was published, where the two differ. */
  disclosed?: number
  /** A few words for a chart axis, on the entries that appear on one. */
  short?: string
  /** Running totals as of this entry, by tally key. */
  totals: ReadonlyMap<string, number>
}

type Attrs = Record<string, string | number>

function node<K extends keyof SVGElementTagNameMap>(
  name: K,
  attrs: Attrs = {},
): SVGElementTagNameMap[K] {
  const made = document.createElementNS(NS, name)
  for (const [key, value] of Object.entries(attrs)) made.setAttribute(key, String(value))
  return made
}

function text(x: number, y: number, body: string, className: string): SVGTextElement {
  const made = node("text", { x, y, class: className })
  made.textContent = body
  return made
}

/** The day number of the first of a month, for the axis ticks. */
function month(index: number): number {
  return Date.parse(`2026-${String(index + 1).padStart(2, "0")}-01`) / 86_400_000
}

/** The class that paints a mark, which is the only thing colour says here. */
function kindClass(kind: string): string {
  if (kind === "agents" || kind === "accomplices") return `is-${kind}`
  return "is-other"
}

/**
 * Redraws at the element's own width whenever that changes. A figure starts
 * hidden, so the first measurement is zero and the real one arrives from the
 * observer once the figure is shown.
 */
function responsive(host: HTMLElement, draw: (width: number) => SVGSVGElement): void {
  let frame: number | undefined
  let last = -1

  const render = (): void => {
    frame = undefined
    const width = Math.round(host.clientWidth)
    if (width <= 0 || width === last) return

    last = width
    host.querySelector("svg")?.remove()
    host.prepend(draw(width))
  }

  new ResizeObserver(() => {
    if (frame === undefined) frame = requestAnimationFrame(render)
  }).observe(host)

  render()
}

/** One tooltip per chart, positioned against the chart's own box. */
interface Tip {
  show: (x: number, y: number, lines: string[]) => void
  hide: () => void
}

function tooltip(host: HTMLElement): Tip {
  const box = document.createElement("div")
  box.className = "ait-tip"
  box.hidden = true
  host.append(box)

  return {
    show(x, y, lines) {
      box.replaceChildren(
        ...lines.map((line, index) => {
          const span = document.createElement("span")
          span.textContent = line
          if (index === 0) span.className = "ait-tip-head"
          return span
        }),
      )
      box.hidden = false
      // Keep it inside the chart rather than off the side of the page.
      const room = host.clientWidth - box.offsetWidth
      box.style.left = `${Math.min(Math.max(x - box.offsetWidth / 2, 0), Math.max(room, 0))}px`
      box.style.top = `${y}px`
    },
    hide() {
      box.hidden = true
    },
  }
}

/**
 * A layer over the whole plot that answers with the nearest entry, because the
 * marks are a few pixels apart and nobody can be asked to hit one. `distance`
 * decides what nearest means: along the year for the two time charts, down the
 * rows for the third.
 */
interface Pointed {
  entries: readonly Plotted[]
  distance: (entry: Plotted, x: number, y: number) => number
  place: (entry: Plotted) => { x: number; y: number }
  lines: (entry: Plotted) => string[]
  onPick?: (entry: Plotted | undefined) => void
}

function nearest(svg: SVGSVGElement, host: HTMLElement, tip: Tip, spec: Pointed): void {
  const find = (event: { clientX: number; clientY: number }): Plotted | undefined => {
    const box = svg.getBoundingClientRect()
    const x = event.clientX - box.left
    const y = event.clientY - box.top

    let best: Plotted | undefined
    let closest = Infinity
    for (const entry of spec.entries) {
      const away = spec.distance(entry, x, y)
      if (away < closest) {
        closest = away
        best = entry
      }
    }
    return best
  }

  svg.addEventListener("pointermove", (event) => {
    const entry = find(event)
    if (entry === undefined) return

    const at = spec.place(entry)
    tip.show(at.x, at.y, spec.lines(entry))
    spec.onPick?.(entry)
  })
  svg.addEventListener("pointerleave", () => {
    tip.hide()
    spec.onPick?.(undefined)
  })
  svg.addEventListener("click", (event) => {
    find(event)?.element.scrollIntoView({ behavior: "smooth", block: "center" })
  })
  host.style.cursor = "pointer"
}

/* ------------------------------------------------------------------ ruler */

/** Set by `drawRuler`, called by the page script when the active entry moves. */
export type Mark = (index: number) => void

/**
 * The year in the panel: a tick per entry at its real date, so the shape of
 * the year shows at a glance — almost nothing until May, then a pile-up in
 * September. Doubles as the progress bar it replaces.
 */
export function drawRuler(host: HTMLElement, entries: readonly Plotted[]): Mark {
  const HEIGHT = 30
  const BASE = 20
  const EDGE = 3
  const tip = tooltip(host)
  let active = 0
  let paint: Mark = () => {}

  responsive(host, (width) => {
    const svg = node("svg", {
      width,
      height: HEIGHT,
      viewBox: `0 0 ${width} ${HEIGHT}`,
      class: "ait-chart",
    })
    svg.setAttribute("role", "img")
    svg.setAttribute("aria-label", "the year, one tick per entry")

    // The last month letter has to fit, so the year stops short of the edge.
    const span = Math.max(width - EDGE * 2 - GLYPH * 2, 10)
    const x = (at: number): number => EDGE + ((at - YEAR_FROM) / (YEAR_TO - YEAR_FROM)) * span

    svg.append(node("line", { x1: EDGE, y1: BASE, x2: EDGE + span, y2: BASE, class: "ait-axis" }))

    for (const [index, label] of MONTHS.entries()) {
      const at = x(month(index))
      svg.append(node("line", { x1: at, y1: BASE, x2: at, y2: BASE + 3, class: "ait-axis" }))
      svg.append(text(at + 1, HEIGHT - 1, label, "ait-tick"))
    }

    const marks = entries.map((entry) => {
      const at = x(entry.at)
      const mark = node("line", {
        x1: at,
        y1: BASE - 10,
        x2: at,
        y2: BASE,
        class: `ait-mark ${kindClass(entry.kind)}`,
      })
      svg.append(mark)
      return mark
    })

    nearest(svg, host, tip, {
      entries,
      distance: (entry, at) => Math.abs(x(entry.at) - at),
      place: (entry) => ({ x: x(entry.at), y: BASE - 10 }),
      lines: (entry) => [entry.title, entry.date],
    })

    paint = (index) => {
      for (const [position, mark] of marks.entries()) {
        mark.classList.toggle("is-ahead", position > index)
        mark.classList.toggle("is-here", position === index)
      }
    }
    paint(active)
    return svg
  })

  return (index) => {
    active = index
    paint(index)
  }
}

/* ------------------------------------------------------------------ shape */

interface Series {
  /** The tally the page script keeps, which is where the numbers come from. */
  key: string
  /** Two words for the end of the line. */
  short: string
  /** The whole thing, for the tooltip. */
  label: string
  className: string
}

const SHAPE: readonly Series[] = [
  {
    key: "intrusions",
    short: "break-ins",
    label: "third parties an agent got into",
    className: "is-agents",
  },
  {
    key: "campaigns",
    short: "campaigns",
    label: "campaigns by people using one",
    className: "is-accomplices",
  },
]

/**
 * What the year did: the two running totals as step lines. They count the same
 * kind of thing, so they share one axis, and the step is honest about a total
 * that only moves when an entry moves it.
 */
export function drawShape(figure: HTMLElement, entries: readonly Plotted[]): void {
  const host = document.createElement("div")
  host.className = "ait-plot"
  figure.prepend(host)

  const tip = tooltip(host)
  const TOP = 10
  const FLOOR = 150
  const AXIS = 18
  const GUTTER = 24

  const last = entries[entries.length - 1]
  const peak = Math.max(...SHAPE.map((series) => last?.totals.get(series.key) ?? 0), 1)
  const step = peak > 30 ? 10 : 5
  // Enough room for "23 campaigns" and its dot, whichever label is longer.
  const reserve = 14 + GLYPH * Math.max(...SHAPE.map((s) => s.short.length + 4))

  responsive(host, (width) => {
    const full = FLOOR + AXIS
    const svg = node("svg", {
      width,
      height: full,
      viewBox: `0 0 ${width} ${full}`,
      class: "ait-chart",
    })
    svg.setAttribute("role", "img")
    svg.setAttribute("aria-label", "running totals across 2026")

    const span = Math.max(width - GUTTER - reserve, 10)
    const x = (at: number): number => GUTTER + ((at - YEAR_FROM) / (YEAR_TO - YEAR_FROM)) * span
    const y = (value: number): number => FLOOR - (value / peak) * (FLOOR - TOP)

    for (let value = 0; value <= peak; value += step) {
      svg.append(
        node("line", {
          x1: GUTTER,
          y1: y(value),
          x2: GUTTER + span,
          y2: y(value),
          class: "ait-grid",
        }),
      )
      svg.append(text(GUTTER - 5, y(value) + 3, String(value), "ait-tick ait-tick-end"))
    }

    for (const [index, label] of MONTHS.entries()) {
      svg.append(text(x(month(index)) + 1, full - 5, label, "ait-tick"))
    }

    const guide = node("line", { x1: 0, y1: TOP, x2: 0, y2: FLOOR, class: "ait-guide", opacity: 0 })
    svg.append(guide)

    for (const series of SHAPE) {
      let path = `M ${x(YEAR_FROM)} ${y(0)}`
      let height = 0
      for (const entry of entries) {
        const value = entry.totals.get(series.key) ?? 0
        if (value === height) continue
        path += ` L ${x(entry.at)} ${y(height)} L ${x(entry.at)} ${y(value)}`
        height = value
      }
      path += ` L ${x(YEAR_TO)} ${y(height)}`

      svg.append(node("path", { d: path, class: `ait-line ${series.className}` }))
      svg.append(
        node("circle", {
          cx: x(YEAR_TO),
          cy: y(height),
          r: 4,
          class: `ait-dot ${series.className}`,
        }),
      )
      svg.append(
        text(
          x(YEAR_TO) + 9,
          y(height) + 3,
          `${height} ${series.short}`,
          `ait-label ${series.className}`,
        ),
      )
    }

    nearest(svg, host, tip, {
      entries,
      distance: (entry, at) => Math.abs(x(entry.at) - at),
      place: (entry) => ({ x: x(entry.at), y: y(entry.totals.get("intrusions") ?? 0) }),
      lines: (entry) => [
        entry.title,
        entry.date,
        ...SHAPE.map((series) => `${entry.totals.get(series.key) ?? 0} ${series.label}`),
      ],
      onPick: (entry) => {
        guide.setAttribute("opacity", entry === undefined ? "0" : "1")
        if (entry !== undefined) {
          guide.setAttribute("x1", String(x(entry.at)))
          guide.setAttribute("x2", String(x(entry.at)))
        }
      },
    })

    return svg
  })

  figure.hidden = false
}

/* ---------------------------------------------------------------- silence */

interface Row {
  entry: Plotted
  days: number
}

/**
 * How long each one went unknown, longest first. Only the entries carrying
 * both a date and a disclosure date are here; the rest were reported as they
 * happened, or are aggregates dated to a report, and have no gap to draw.
 * `cargo test -p site` refuses an entry that belongs here without the short
 * label this uses, so nothing drops out of it quietly.
 */
export function drawSilence(figure: HTMLElement, entries: readonly Plotted[]): void {
  const rows: Row[] = entries
    .filter((entry) => entry.disclosed !== undefined && entry.short !== undefined)
    .map((entry) => ({ entry, days: Math.max(Math.round((entry.disclosed ?? 0) - entry.at), 0) }))
    .filter((row) => row.days > 0)
    .sort((a, b) => b.days - a.days)

  if (rows.length === 0) return

  const host = document.createElement("div")
  host.className = "ait-plot"
  figure.prepend(host)

  const tip = tooltip(host)
  const ROW = 20
  const BAR = 10
  const AXIS = 20
  const longest = rows[0].days
  // "140 days", plus the gap before it.
  const reserve = 10 + GLYPH * (String(longest).length + 6)

  responsive(host, (width) => {
    const gutter = Math.min(width < 420 ? 104 : 148, width * 0.45)
    const full = rows.length * ROW + AXIS
    const svg = node("svg", {
      width,
      height: full,
      viewBox: `0 0 ${width} ${full}`,
      class: "ait-chart",
    })
    svg.setAttribute("role", "img")
    svg.setAttribute("aria-label", "days between the thing happening and its disclosure")

    const span = Math.max(width - gutter - reserve, 10)
    const x = (days: number): number => gutter + (days / longest) * span

    for (let days = 0; days <= longest; days += 30) {
      svg.append(
        node("line", { x1: x(days), y1: 0, x2: x(days), y2: full - AXIS, class: "ait-grid" }),
      )
      svg.append(text(x(days), full - 6, String(days), "ait-tick ait-tick-mid"))
    }

    for (const [index, row] of rows.entries()) {
      const top = index * ROW + (ROW - BAR) / 2
      svg.append(
        node("rect", {
          x: gutter,
          y: top,
          width: Math.max(x(row.days) - gutter, 2),
          height: BAR,
          rx: 4,
          class: `ait-bar ${kindClass(row.entry.kind)}`,
        }),
      )
      svg.append(text(gutter - 8, top + BAR - 1, row.entry.short ?? "", "ait-row"))
      // Only the ends get a number; the rest are read off the axis or hovered.
      if (index === 0 || index === rows.length - 1) {
        svg.append(text(x(row.days) + 7, top + BAR - 1, `${row.days} days`, "ait-label"))
      }
    }

    const place = (entry: Plotted): { x: number; y: number } => {
      const index = rows.findIndex((row) => row.entry === entry)
      return { x: (gutter + x(rows[index].days)) / 2, y: index * ROW + ROW / 2 }
    }

    nearest(svg, host, tip, {
      entries: rows.map((row) => row.entry),
      distance: (entry, _x, y) => Math.abs(place(entry).y - y),
      place,
      lines: (entry) => {
        const row = rows.find((candidate) => candidate.entry === entry)
        return [entry.title, `${row?.days ?? 0} days unknown`, entry.date]
      },
    })

    return svg
  })

  figure.hidden = false
}

/* ----------------------------------------------------------------- legend */

/** Colour means one thing on this page, so it is spelled out beside each one. */
export function legend(host: HTMLElement, classes: readonly [string, string][]): void {
  const list = document.createElement("ul")
  list.className = "ait-legend"

  for (const [className, label] of classes) {
    const item = document.createElement("li")
    const swatch = document.createElement("span")
    swatch.className = `ait-swatch ${className}`
    item.append(swatch, label)
    list.append(item)
  }

  host.append(list)
}
