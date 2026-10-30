/**
 * The wager table on the Pascal post.
 *
 * Rows are traditions taken as true, columns are traditions followed, and a
 * cell is what the row says becomes of a follower of the column, as
 * `eternal · ∞ + finite`. The reader puts a probability on each row and a
 * cost of practice on each column, and the wasm does the expectation and the
 * ranking; this file only draws what it says.
 */

import {
  wager_adherents,
  wager_cell,
  wager_cost,
  wager_count,
  wager_expected,
  wager_family,
  wager_format,
  wager_name,
  wager_prior,
  wager_ranking,
  wager_source_count,
  wager_source_label,
  wager_source_url,
  wager_summary,
} from "wasm-wager"

const ROOT = "pascal-wager"
const STORAGE = "pascal-wager"

interface Saved {
  priors: number[]
  costs: number[]
}

interface Column {
  header: HTMLTableCellElement
  cost: HTMLInputElement
  expected: HTMLTableCellElement
  cells: HTMLTableCellElement[]
}

interface Row {
  prior: HTMLInputElement
}

function element<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  className?: string,
  text?: string,
): HTMLElementTagNameMap[K] {
  const node = document.createElement(tag)
  if (className !== undefined) node.className = className
  if (text !== undefined) node.textContent = text
  return node
}

function numberInput(value: number, step: string, min: string): HTMLInputElement {
  const input = element("input")
  input.type = "number"
  input.inputMode = "decimal"
  input.min = min
  input.step = step
  input.value = String(value)
  return input
}

function read(input: HTMLInputElement): number {
  const value = Number.parseFloat(input.value)
  return Number.isFinite(value) && value >= 0 ? value : 0
}

function load(count: number): Saved | undefined {
  try {
    const raw = window.localStorage.getItem(STORAGE)
    if (raw === null) return undefined
    const saved: unknown = JSON.parse(raw)
    if (
      typeof saved !== "object" ||
      saved === null ||
      !Array.isArray((saved as Saved).priors) ||
      !Array.isArray((saved as Saved).costs) ||
      (saved as Saved).priors.length !== count ||
      (saved as Saved).costs.length !== count
    ) {
      return undefined
    }
    return saved as Saved
  } catch {
    return undefined
  }
}

function save(priors: number[], costs: number[]) {
  try {
    window.localStorage.setItem(STORAGE, JSON.stringify({ priors, costs }))
  } catch {
    return
  }
}

function percent(value: number): string {
  return `${value.toLocaleString(undefined, { maximumFractionDigits: 2 })}%`
}

function sourceLinks(index: number): HTMLAnchorElement[] {
  return Array.from({ length: wager_source_count(index) }, (_, source) => {
    const link = element("a", undefined, wager_source_label(index, source))
    link.href = wager_source_url(index, source)
    link.rel = "noreferrer"
    link.target = "_blank"
    return link
  })
}

function citation(links: HTMLAnchorElement[]): HTMLElement {
  const cited = element("span", "pascal-cited")
  cited.append("Sources: ")
  links.forEach((link, position) => {
    if (position > 0) cited.append(", ")
    cited.append(link)
  })
  cited.append(".")
  return cited
}

/** Every row's reading, listed under the table so the whole table can be checked. */
function buildReferences(names: string[]): HTMLDetailsElement {
  const references = element("details", "pascal-references")
  const summary = element("summary", undefined, "where each row comes from")
  const list = element("ul")

  names.forEach((name, index) => {
    const links = sourceLinks(index)
    if (links.length === 0) return
    const item = element("li")
    item.append(element("strong", undefined, name), ": ")
    links.forEach((link, position) => {
      if (position > 0) item.append(", ")
      item.append(link)
    })
    list.append(item)
  })

  references.append(summary, list)
  return references
}

function render() {
  const root = document.getElementById(ROOT)
  if (root === null) return
  root.textContent = ""

  const count = wager_count()
  const names = Array.from({ length: count }, (_, index) => wager_name(index))
  const saved = load(count)

  const controls = element("div", "pascal-controls")
  const mine = element("button", undefined, "back to my prior")
  const uniform = element("button", undefined, "every row equally likely")
  const crowd = element("button", undefined, "weight rows by how many people follow them")
  const nobody = element("button", undefined, "clear every row")
  const defaults = element("button", undefined, "reset costs")
  for (const button of [mine, uniform, crowd, nobody, defaults]) button.type = "button"
  controls.append(mine, uniform, crowd, nobody, defaults)

  const scroll = element("div", "pascal-scroll")
  const table = element("table", "pascal-table")
  const head = element("thead")
  const body = element("tbody")
  table.append(head, body)
  scroll.append(table)

  const nameRow = element("tr", "pascal-names")
  const corner = element("th", "pascal-corner")
  corner.append(
    element("span", undefined, "if this row is true ↓"),
    element("span", undefined, "and you followed this column →"),
  )
  nameRow.append(corner)

  const costRow = element("tr", "pascal-costs")
  costRow.append(element("th", "pascal-label", "cost of a lifetime of practice, in lifetimes"))

  const expectedRow = element("tr", "pascal-expected")
  expectedRow.append(element("th", "pascal-label", "expected utility"))

  const columns: Column[] = names.map((name, index) => {
    const header = element("th", "pascal-column")
    const label = element("span", undefined, name)
    header.append(label)
    header.title = `${name} (${wager_family(index)}): ${wager_summary(index)}`
    nameRow.append(header)

    const cost = numberInput(saved?.costs[index] ?? wager_cost(index), "0.01", "0")
    cost.setAttribute("aria-label", `cost of practising ${name}`)
    const costCell = element("td")
    costCell.append(cost)
    costRow.append(costCell)

    const expected = element("td", "pascal-value")
    expectedRow.append(expected)

    return { header, cost, expected, cells: [] }
  })
  head.append(nameRow, costRow, expectedRow)

  const rows: Row[] = names.map((name, truth) => {
    const row = element("tr")
    const header = element("th", "pascal-row")
    const prior = numberInput(saved?.priors[truth] ?? wager_prior(truth), "any", "0")
    prior.setAttribute("aria-label", `probability that ${name} is true, in percent`)
    const label = element("span", "pascal-row-name", name)
    label.title = `${wager_family(truth)}: ${wager_summary(truth)}`
    header.append(prior, element("span", "pascal-unit", "%"), label)
    row.append(header)

    for (let followed = 0; followed < count; followed += 1) {
      const cell = wager_cell(truth, followed)
      const td = element("td", "pascal-cell", cell.text)
      td.dataset.truth = String(truth)
      td.dataset.followed = String(followed)
      td.dataset.note = cell.note
      const eternal = cell.eternal
      if (eternal > 0) td.classList.add("heaven")
      else if (eternal < 0) td.classList.add("hell")
      else td.classList.add("mortal")
      td.style.setProperty("--mix", `${Math.round(Math.abs(eternal) * 100)}%`)
      cell.free()
      columns[followed].cells.push(td)
      row.append(td)
    }
    body.append(row)
    return { prior }
  })

  const reading = element("p", "pascal-reading")
  reading.setAttribute("aria-live", "polite")
  reading.textContent = "Point at a cell to read the teaching it is based on."

  const ranking = element("ol", "pascal-ranking")
  const references = buildReferences(names)
  const verdict = element("p", "pascal-verdict")
  verdict.setAttribute("aria-live", "polite")

  root.append(controls, scroll, reading, verdict, ranking, references)

  function explain(td: HTMLTableCellElement) {
    const truth = Number(td.dataset.truth)
    const followed = Number(td.dataset.followed)
    reading.textContent = ""
    reading.append(
      "If ",
      element("strong", undefined, names[truth]),
      " is true and you followed ",
      element("strong", undefined, names[followed]),
      `, you get ${td.textContent}. ${td.dataset.note ?? ""}`,
    )
    const cited = sourceLinks(truth)
    if (cited.length > 0) {
      reading.append(" ")
      reading.append(citation(cited))
    }
  }

  function update() {
    const priors = rows.map((row) => read(row.prior))
    const costs = columns.map((column) => read(column.cost))
    save(priors, costs)

    const total = priors.reduce((sum, prior) => sum + prior, 0)
    const expected = wager_expected(Float64Array.from(priors), Float64Array.from(costs))
    const order = wager_ranking(Float64Array.from(priors), Float64Array.from(costs))
    const best = order[0]

    columns.forEach((column, index) => {
      column.expected.textContent = wager_format(expected[2 * index], expected[2 * index + 1])
      const isBest = index === best
      column.header.classList.toggle("best", isBest)
      column.expected.classList.toggle("best", isBest)
      for (const cell of column.cells) cell.classList.toggle("best", isBest)
    })

    ranking.textContent = ""
    for (const index of order) {
      const item = element("li")
      item.append(
        element("span", "pascal-rank-name", names[index]),
        element(
          "span",
          "pascal-rank-value",
          wager_format(expected[2 * index], expected[2 * index + 1]),
        ),
      )
      ranking.append(item)
    }

    verdict.textContent = ""
    if (total <= 0) {
      verdict.append(
        "Every row is at zero, so nothing after death counts and the cheapest practice wins: ",
        element("strong", undefined, names[best]),
        ".",
      )
      return
    }
    const runnerUp = order[1]
    verdict.append(
      "With these probabilities the highest expected utility is ",
      element("strong", undefined, names[best]),
      ` at ${wager_format(expected[2 * best], expected[2 * best + 1])}, ahead of `,
      element("strong", undefined, names[runnerUp]),
      ` at ${wager_format(expected[2 * runnerUp], expected[2 * runnerUp + 1])}. `,
      `The rows sum to ${percent(total)} and are normalised.`,
    )
  }

  function setPriors(values: (index: number) => number) {
    rows.forEach((row, index) => {
      row.prior.value = String(values(index))
    })
    update()
  }

  mine.addEventListener("click", () => setPriors((index) => wager_prior(index)))
  uniform.addEventListener("click", () => setPriors(() => 1))
  crowd.addEventListener("click", () => setPriors((index) => wager_adherents(index)))
  nobody.addEventListener("click", () => setPriors(() => 0))
  defaults.addEventListener("click", () => {
    columns.forEach((column, index) => {
      column.cost.value = String(wager_cost(index))
    })
    update()
  })

  table.addEventListener("input", update)
  table.addEventListener("mouseover", (event) => {
    const td = (event.target as Element).closest<HTMLTableCellElement>("td.pascal-cell")
    if (td !== null) explain(td)
  })
  table.addEventListener("click", (event) => {
    const td = (event.target as Element).closest<HTMLTableCellElement>("td.pascal-cell")
    if (td !== null) explain(td)
  })

  update()
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", render, { once: true })
} else {
  render()
}
