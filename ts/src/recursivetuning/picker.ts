/**
 * A search over every scale the playground knows, for one side of a pair:
 * music21-rs's tuning systems, the equal divisions it suggests, its regular
 * temperaments at a moment-of-symmetry size, and, once something is typed,
 * the Scala archive.
 */

import type { Library, ScalaEntry, Side, TemperamentFacts } from "./types.js"

/** The playground's adaptive scale, which is a pair already and so no side of one. */
const ADAPTIVE_ID = "adaptive:recursive"

export interface PickerOptions {
  root: HTMLElement
  library: Library
  search: (query: string) => ScalaEntry[]
  onPick: (id: string) => void
}

export interface Picker {
  show: (side: Side) => void
}

function part<T extends HTMLElement>(root: HTMLElement, selector: string): T {
  const node = root.querySelector<T>(selector)
  if (!node) throw new Error(`the picker has no ${selector}`)
  return node
}

function element<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  className?: string,
  text?: string,
): HTMLElementTagNameMap[K] {
  const node = document.createElement(tag)
  if (className) node.className = className
  if (text !== undefined) node.textContent = text
  return node
}

/** The size a temperament is offered at: its first moment of symmetry of five to twelve notes. */
function preferredSize(entry: TemperamentFacts): number | null {
  if (entry.moments.length === 0) return null
  return entry.moments.find((size) => size >= 5 && size <= 12) ?? entry.moments[0]
}

function periodName(cents: number): string {
  if (Math.abs(cents - 1200) < 1e-6) return "the octave"
  if (Math.abs(cents - 1901.955) < 0.01) return "the tritave"
  return `${cents.toFixed(1)}¢`
}

export function picker({ root, library, search, onPick }: PickerOptions): Picker {
  const input = part<HTMLInputElement>(root, "input")
  const results = part<HTMLElement>(root, ".rt-results")
  const chosen = part<HTMLElement>(root, ".rt-chosen")
  let currentId = ""

  const close = (): void => {
    results.hidden = true
    input.value = ""
    input.setAttribute("aria-expanded", "false")
  }

  const item = (name: string, aside: string, sub: string, id: string): HTMLButtonElement => {
    const node = element("button", `rt-item${id === currentId ? " is-active" : ""}`)
    node.type = "button"
    node.append(element("span", "rt-item-name", name), element("span", "rt-item-aside", aside))
    if (sub) node.append(element("span", "rt-item-sub", sub))
    node.addEventListener("click", () => {
      close()
      onPick(id)
    })
    return node
  }

  const render = (): void => {
    const query = input.value.trim().toLowerCase()
    const matches = (text: string): boolean => !query || text.toLowerCase().includes(query)
    const fragment = document.createDocumentFragment()
    const add = (name: string, rows: HTMLElement[]): void => {
      if (rows.length === 0) return
      const heading = element("div", "rt-group")
      heading.append(
        element("span", undefined, name),
        element("span", undefined, String(rows.length)),
      )
      fragment.append(heading, ...rows)
    }

    add(
      "Tuning systems",
      library.systems
        .filter((system) => system.id !== ADAPTIVE_ID)
        .filter((system) => matches(`${system.name} ${system.family} ${system.description}`))
        .map((system) => item(system.name, `${system.count} / octave`, system.family, system.id)),
    )
    add(
      "Equal divisions",
      library.equal
        .filter((preset) => matches(`${preset.label} ${preset.note} edo equal`))
        .map((preset) =>
          item(
            preset.label,
            `${preset.divisions} / ${preset.numerator}:${preset.denominator}`,
            preset.note,
            preset.id,
          ),
        ),
    )
    add(
      "Regular temperaments",
      library.temperaments.flatMap((entry) => {
        const size = preferredSize(entry)
        if (size === null || !matches(`${entry.name} ${entry.page} ${entry.subgroup}`)) return []
        return [
          item(entry.page, `${size} notes`, entry.subgroup, `temperament:${entry.name}:${size}`),
        ]
      }),
    )
    if (query) {
      add(
        "Scala archive",
        search(query).map((entry) =>
          item(
            entry.file.replace(/\.scl$/, ""),
            `${entry.count} / period`,
            entry.description,
            entry.id,
          ),
        ),
      )
    } else {
      fragment.append(
        element(
          "div",
          "rt-note",
          `Type to search the ${library.scala_count.toLocaleString()} scales of the Scala archive as well.`,
        ),
      )
    }
    if (fragment.childElementCount === 0) {
      fragment.append(element("div", "rt-note", `Nothing matches "${input.value.trim()}".`))
    }
    results.replaceChildren(fragment)
  }

  const open = (): void => {
    render()
    results.hidden = false
    input.setAttribute("aria-expanded", "true")
  }

  input.addEventListener("focus", open)
  input.addEventListener("input", open)
  input.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      close()
      input.blur()
    } else if (event.key === "Enter") {
      results.querySelector<HTMLButtonElement>(".rt-item")?.click()
    }
  })
  document.addEventListener("pointerdown", (event) => {
    if (!results.hidden && !root.contains(event.target as Node)) close()
  })

  return {
    show(side: Side): void {
      currentId = side.id
      chosen.replaceChildren(
        element("strong", undefined, side.name),
        document.createTextNode(
          ` · ${side.count} notes to ${periodName(side.period_cents)}. ${side.description}`,
        ),
      )
    },
  }
}
