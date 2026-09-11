/**
 * The tuning playground's page: the library the scale is picked from, what
 * the page says about it, the keyboard, the staff, the degree table and the
 * log. index.ts owns the sound; this file only draws and wires.
 */

import {
  Key,
  Library,
  ScalaEntry,
  Scale,
  TemperamentFacts,
  _noteOn,
  audio,
  formatError,
  library,
  markedKeys,
  noteOff,
  noteOn,
  octaveShift,
  playingTones,
  scale,
  selectScale,
  selectScl,
  setEngine,
  setOctaveShift,
  setVolume,
  setWaveform,
  shift,
  stopAllTones,
  wasm,
} from "./index.js"
import { connectMidi, playMIDIFile, stopMIDIFile } from "./MIDI.js"
import { DEFAULT_ROOT_HZ } from "./config.js"

// ---------------------------------------------------------------------------
// Elements

const el = <T extends HTMLElement>(id: string) => document.getElementById(id) as T

const search = el<HTMLInputElement>("tuningSearch")
const browseToggle = el<HTMLButtonElement>("browseToggle")
const libraryPanel = el<HTMLDivElement>("libraryPanel")
const chips = el<HTMLDivElement>("libraryChips")
const list = el<HTMLDivElement>("libraryList")

const title = el<HTMLElement>("tuningTitle")
const facts = el<HTMLElement>("tuningFacts")
const sizes = el<HTMLDivElement>("tuningSizes")
const description = el<HTMLParagraphElement>("tuningDescription")

const output = el<HTMLElement>("output")
const keyboardEl = el<HTMLDivElement>("keyboard")

const soundMethod = el<HTMLSelectElement>("soundMethod")
const waveform = el<HTMLSelectElement>("waveform")
const volumeSlider = el<HTMLInputElement>("volumeSlider")
const octaveInput = el<HTMLInputElement>("octave")
const playScaleButton = el<HTMLButtonElement>("playScale")
const stopButton = el<HTMLButtonElement>("stopAll")
const shareButton = el<HTMLButtonElement>("shareLink")

const degreesBody = el<HTMLTableSectionElement>("degreesBody")

const layoutSelect = el<HTMLSelectElement>("keymapSelect")
const midiButton = el<HTMLButtonElement>("midiButton")
const midiStatus = el<HTMLElement>("midiStatus")
const fileInput = el<HTMLInputElement>("fileInput")
const playFileButton = el<HTMLButtonElement>("playButton")
const stopFileButton = el<HTMLButtonElement>("stopButton")
const chordInput = el<HTMLInputElement>("chordInput")
const nameChord = el<HTMLButtonElement>("nameChord")
const clearChord = el<HTMLButtonElement>("clearChord")
const chordNameOutput = el<HTMLDivElement>("chordNameOutput")
const chordDetailsOutput = el<HTMLDivElement>("chordDetailsOutput")

const markedButtons = el<HTMLDivElement>("markedButtons")
const playMarked = el<HTMLButtonElement>("playMarked")
const shareMarked = el<HTMLButtonElement>("shareMarked")
const logContainer = el<HTMLDivElement>("logContainer")

export function setStatus(message: string, state: "loading" | "ready" | "error" = "loading"): void {
  const status = document.getElementById("tuningPlaygroundStatus")
  if (!status) return
  status.dataset.state = state
  status.textContent = message
}

export function setMidiStatus(message: string, connected: boolean): void {
  midiStatus.textContent = message
  midiButton.classList.toggle("is-on", connected)
  midiButton.textContent = connected ? "MIDI connected" : "Connect a MIDI device"
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

function button(text: string, className?: string): HTMLButtonElement {
  const node = element("button", className, text)
  node.type = "button"
  return node
}

function hz(value: number): string {
  return value.toFixed(value >= 100 ? 2 : 3)
}

function signed(value: number, digits = 1): string {
  const text = value.toFixed(digits)
  return value > 0 ? `+${text}` : text
}

function clamp(value: string | null, min: number, max: number, fallback: number): number {
  const parsed = Number.parseFloat(value ?? "")
  if (!Number.isFinite(parsed)) return fallback
  return Math.min(max, Math.max(min, parsed))
}

// ---------------------------------------------------------------------------
// The URL
//
// `?system=` names the scale, `?root=` its root in hertz, `?octave=` the
// shift, and the hash carries the marked notes. `?tuning=` is what links used
// to say and still works.

let rootHz = DEFAULT_ROOT_HZ
let pasted: { name: string; contents: string } | null = null

export function readUrl(): { id: string; root: number } {
  const params = new URLSearchParams(window.location.search)
  rootHz = clamp(params.get("root"), 20, 2000, DEFAULT_ROOT_HZ)
  setOctaveShift(clamp(params.get("octave"), -4, 4, 0))
  octaveInput.value = String(octaveShift)
  return { id: params.get("system") ?? params.get("tuning") ?? "EqualTemperament", root: rootHz }
}

function shareUrl(steps: number[]): string {
  const url = new URL(window.location.href)
  url.search = ""
  if (!scale.id.startsWith("pasted:")) url.searchParams.set("system", scale.id)
  if (Math.abs(rootHz - DEFAULT_ROOT_HZ) > 1e-9) url.searchParams.set("root", String(rootHz))
  if (octaveShift !== 0) url.searchParams.set("octave", String(octaveShift))
  url.hash = steps.length > 0 ? wasm.tuning_marked_hash(steps.join(",")) : ""
  return url.toString()
}

/**
 * Writes the state into the address bar without navigating. Assigning
 * `location.hash` fires `popstate`, which the site's router answers by
 * re-fetching the page and re-running this script.
 */
function syncUrl(): void {
  history.replaceState(null, "", shareUrl(markedKeys))
}

function copyLink(steps: number[]): void {
  const url = shareUrl(steps)
  navigator.clipboard
    ?.writeText(url)
    .then(() => setStatus("Link copied.", "ready"))
    .catch(() => setStatus("Could not copy the link; it is in the address bar.", "error"))
}

// ---------------------------------------------------------------------------
// Choosing a scale

export function choose(id: string): void {
  if (id.startsWith("pasted:") && pasted) {
    selectScl(pasted.name, pasted.contents, rootHz)
  } else {
    selectScale(id, rootHz)
  }
  scaleChanged()
  closeLibrary()
  syncUrl()
}

function scaleChanged(): void {
  setStatus("", "ready")
  applyLayout()
  renderHead()
  renderKeyboard()
  renderDegrees()
  playingTonesChanged()
  if (!libraryPanel.hidden) renderLibrary()
}

// ---------------------------------------------------------------------------
// The library

type Kind = "all" | "system" | "temperament" | "equal" | "scala" | "mine"
let kind: Kind = "all"
const SCALA_LIMIT = 400

function openLibrary(): void {
  if (!libraryPanel.hidden) return
  libraryPanel.hidden = false
  browseToggle.setAttribute("aria-expanded", "true")
  renderLibrary()
}

function closeLibrary(): void {
  libraryPanel.hidden = true
  browseToggle.setAttribute("aria-expanded", "false")
  search.value = ""
}

function matches(text: string, query: string): boolean {
  return !query || text.toLowerCase().includes(query)
}

function item(name: string, aside: string, sub: string, active: boolean, onClick: () => void) {
  const node = button("", `tp-item${active ? " is-active" : ""}`)
  node.append(element("span", "tp-item-name", name), element("span", "tp-item-aside", aside))
  if (sub) node.append(element("span", "tp-item-sub", sub))
  node.addEventListener("click", onClick)
  return node
}

function group(name: string, count: number | string): HTMLElement {
  const node = element("div", "tp-group")
  node.append(element("span", undefined, name), element("span", undefined, String(count)))
  return node
}

function preferredSize(entry: TemperamentFacts): number | null {
  if (entry.moments.length === 0) return null
  return entry.moments.find((size) => size >= 5 && size <= 12) ?? entry.moments[0]
}

function renderLibrary(): void {
  const query = search.value.trim().toLowerCase()
  const fragment = document.createDocumentFragment()
  const lib: Library = library

  if (kind === "equal") fragment.appendChild(equalForm())
  if (kind === "mine") fragment.appendChild(mineForm())

  if (kind === "all" || kind === "system") {
    const rows = lib.systems
      .filter((system) => matches(`${system.name} ${system.family} ${system.description}`, query))
      .map((system) =>
        item(
          system.name,
          `${system.count} / ${system.id === "adaptive:recursive" ? "oct, adaptive" : "oct"}`,
          `${system.family} · ${system.description}`,
          system.id === scale.id,
          () => choose(system.id),
        ),
      )
    if (rows.length > 0) {
      fragment.appendChild(group("Tuning systems", rows.length))
      fragment.append(...rows)
    }
  }

  if (kind === "all" || kind === "temperament") {
    const rows: HTMLElement[] = []
    for (const entry of lib.temperaments) {
      const haystack = `${entry.name} ${entry.page} ${entry.subgroup} ${entry.commas.join(" ")} ${entry.published_moments.join(" ")}`
      if (!matches(haystack, query)) continue
      const generators = entry.generators
        .map((generator) => `${generator.ratio} ${generator.cents.toFixed(1)}¢`)
        .join(", ")
      const aside =
        entry.moments.length > 0
          ? `${entry.moments.slice(0, 6).join(" ")}${entry.moments.length > 6 ? " …" : ""}`
          : `rank ${entry.rank}`
      const active = scale.temperament?.name === entry.name
      rows.push(
        item(entry.page, aside, `${entry.subgroup} · generator ${generators}`, active, () => {
          const size = preferredSize(entry)
          if (size === null) {
            setStatus(
              `${entry.page} is rank ${entry.rank}: it stacks two generators, so it has no single moment-of-symmetry scale to play.`,
              "error",
            )
            return
          }
          choose(`temperament:${entry.name}:${size}`)
        }),
      )
    }
    if (rows.length > 0) {
      fragment.appendChild(group("Regular temperaments", rows.length))
      fragment.append(...rows)
    }
  }

  if (kind === "all" || kind === "equal") {
    const rows = lib.equal
      .filter((preset) => matches(`${preset.label} ${preset.note} edo edt equal`, query))
      .map((preset) =>
        item(
          preset.label,
          `${preset.divisions} / ${preset.numerator}:${preset.denominator}`,
          preset.note,
          preset.id === scale.id,
          () => choose(preset.id),
        ),
      )
    if (rows.length > 0) {
      fragment.appendChild(group("Equal divisions", rows.length))
      fragment.append(...rows)
    }
  }

  if ((kind === "all" && query) || kind === "scala") {
    const found = JSON.parse(wasm.scala_search_json(query, SCALA_LIMIT)) as ScalaEntry[]
    const total = query ? found.length : lib.scala_count
    fragment.appendChild(group("Scala archive", total.toLocaleString()))
    for (const entry of found) {
      fragment.appendChild(
        item(
          entry.file.replace(/\.scl$/, ""),
          `${entry.count} / period`,
          entry.description || "(no description)",
          entry.id === scale.id,
          () => choose(entry.id),
        ),
      )
    }
    if (found.length >= SCALA_LIMIT) {
      fragment.appendChild(
        element("div", "tp-note", `The first ${SCALA_LIMIT} matches; type more to narrow them.`),
      )
    }
  } else if (kind === "all") {
    fragment.appendChild(group("Scala archive", lib.scala_count.toLocaleString()))
    fragment.appendChild(
      element("div", "tp-note", "Type to search the archive, or pick Scala to list every scale."),
    )
  }

  if (!fragment.childElementCount) {
    fragment.appendChild(element("div", "tp-note", `Nothing matches "${search.value.trim()}".`))
  }
  list.replaceChildren(fragment)
}

function field(label: string, control: HTMLElement): HTMLLabelElement {
  const node = element("label", "tool-field")
  node.append(element("span", undefined, label), control)
  return node
}

function equalForm(): HTMLElement {
  const form = element("div", "tp-form tool-bar")
  const divisions = element("input")
  divisions.type = "number"
  divisions.min = "1"
  divisions.max = "1000"
  divisions.value = "19"
  const ratio = element("input")
  ratio.type = "text"
  ratio.value = "2/1"
  ratio.placeholder = "2/1, 3/1, 3/2"
  const go = button("Divide it")
  go.addEventListener("click", () => {
    const count = Math.round(clamp(divisions.value, 1, 1000, 12))
    const [top, bottom = "1"] = ratio.value.trim().split("/")
    const numerator = Number.parseInt(top, 10)
    const denominator = Number.parseInt(bottom, 10)
    if (!(numerator > 0) || !(denominator > 0) || numerator === denominator) {
      setStatus(`${ratio.value} is not an interval such as 3/2.`, "error")
      return
    }
    choose(`equal:${count}:${numerator}:${denominator}`)
  })
  form.append(field("Divisions", divisions), field("Of the interval", ratio), go)
  return form
}

function mineForm(): HTMLElement {
  const form = element("div", "tp-form")
  const file = element("input")
  file.type = "file"
  file.accept = ".scl,text/plain"
  file.addEventListener("change", () => {
    const chosen = file.files?.[0]
    if (!chosen) return
    void chosen.text().then((contents) => loadPasted(chosen.name, contents))
  })
  const text = element("textarea")
  text.rows = 6
  text.spellcheck = false
  text.placeholder = "! my.scl\n!\nMy scale\n 3\n!\n 9/8\n 3/2\n 2/1"
  const go = button("Use this scale")
  go.addEventListener("click", () => loadPasted("pasted.scl", text.value))
  const actions = element("div", "tool-actions")
  actions.append(go)
  form.append(field("A Scala file", file), field("Or its text", text), actions)
  return form
}

function loadPasted(name: string, contents: string): void {
  if (!contents.trim()) {
    setStatus("Paste the contents of a .scl file first.", "error")
    return
  }
  if (!selectScl(name, contents, rootHz)) return
  pasted = { name, contents }
  scaleChanged()
  closeLibrary()
  syncUrl()
}

// ---------------------------------------------------------------------------
// What the page says about the scale

function periodText(system: Scale): string {
  if (Math.abs(system.period_ratio - 2) < 1e-9) return "octave"
  if (Math.abs(system.period_ratio - 3) < 1e-9) return "tritave"
  return `period of ${system.period_cents.toFixed(1)}¢`
}

function renderHead(): void {
  title.textContent = scale.name

  const root = element("input")
  root.type = "number"
  root.min = "20"
  root.max = "2000"
  root.step = "0.0001"
  root.value = String(Number(rootHz.toFixed(4)))
  root.title = "The frequency of the root, which is C4 in a twelve-tone scale"
  root.addEventListener("change", () => {
    rootHz = clamp(root.value, 20, 2000, DEFAULT_ROOT_HZ)
    choose(scale.id)
  })
  const rootFact = element("span", "tp-fact")
  rootFact.append("root ", root, " Hz")
  facts.replaceChildren(
    element("span", "tp-fact", scale.family),
    element("span", "tp-fact", `${scale.count} steps per ${periodText(scale)}`),
    rootFact,
  )

  sizes.replaceChildren()
  const entry = scale.temperament
  if (entry) {
    sizes.append(element("span", "tool-hint", "notes per equave"))
    for (const size of entry.moments) {
      const id = `temperament:${entry.name}:${size}`
      const chip = button(String(size), id === scale.id ? "is-active" : undefined)
      chip.addEventListener("click", () => choose(id))
      sizes.appendChild(chip)
    }
  }

  description.replaceChildren()
  if (scale.description) description.append(scale.description)
  if (entry) {
    const periods = `${entry.periods_per_equave} period${entry.periods_per_equave === 1 ? "" : "s"}`
    const generators = entry.generators
      .map((generator) => `${generator.ratio} = ${generator.cents.toFixed(3)}¢`)
      .join(", ")
    description.append(
      ` Subgroup ${entry.subgroup}, rank ${entry.rank}, ${periods} to the equave; generators ${generators} (${entry.optimization}); tempers out ${entry.commas.join(", ")}.`,
    )
    if (entry.published_moments.length > 0) {
      description.append(` Scales the wiki lists: ${entry.published_moments.join(", ")}.`)
    }
    const link = element("a", undefined, `${entry.page} on the Xenharmonic Wiki`)
    link.href = `https://en.xen.wiki/w/${encodeURIComponent(entry.page)}`
    link.target = "_blank"
    link.rel = "noreferrer"
    description.append(" ", link, ".")
  }
}

// ---------------------------------------------------------------------------
// Keyboard

const keyElements = new Map<number, HTMLElement>()
const pointers = new Map<number, number>()

const WHITE_WIDTH = 34
const BLACK_WIDTH = 20

/** Every physical key a layout might use, so the keys can show which plays them. */
const CODES = [
  "KeyZ",
  "KeyX",
  "KeyC",
  "KeyV",
  "KeyB",
  "KeyN",
  "KeyM",
  "Comma",
  "Period",
  "Slash",
  "IntlBackslash",
  "KeyA",
  "KeyS",
  "KeyD",
  "KeyF",
  "KeyG",
  "KeyH",
  "KeyJ",
  "KeyK",
  "KeyL",
  "Semicolon",
  "Quote",
  "Backslash",
  "KeyQ",
  "KeyW",
  "KeyE",
  "KeyR",
  "KeyT",
  "KeyY",
  "KeyU",
  "KeyI",
  "KeyO",
  "KeyP",
  "BracketLeft",
  "BracketRight",
  "Digit1",
  "Digit2",
  "Digit3",
  "Digit4",
  "Digit5",
  "Digit6",
  "Digit7",
  "Digit8",
  "Digit9",
  "Digit0",
  "Minus",
  "Equal",
]
const CAPTIONS: Record<string, string> = {
  Comma: ",",
  Period: ".",
  Slash: "/",
  Semicolon: ";",
  Quote: "'",
  BracketLeft: "[",
  BracketRight: "]",
  Minus: "-",
  Equal: "=",
  Backslash: "\\",
  IntlBackslash: "<",
}

function caption(code: string): string {
  if (CAPTIONS[code]) return CAPTIONS[code]
  if (code.startsWith("Key")) return code.slice(3).toLowerCase()
  if (code.startsWith("Digit")) return code.slice(5)
  return code
}

/** Which physical key plays each step under the layout in force. */
function hints(): Map<number, string> {
  const map = new Map<number, string>()
  for (const code of CODES) {
    const step = wasm.from_keymap(code)
    if (step === -1) continue
    const shifted = step + shift()
    if (!map.has(shifted)) map.set(shifted, caption(code))
  }
  return map
}

function applyLayout(): void {
  const chosen = layoutSelect.value
  wasm.set_keymap(chosen === "auto" ? (scale.twelve_tone ? "us" : "rows") : chosen)
}

function renderKeyboard(): void {
  const keys = JSON.parse(wasm.keyboard_json()) as Key[]
  keyboardEl.replaceChildren()
  keyElements.clear()
  const captions = hints()

  if (scale.twelve_tone) buildPiano(keys, captions)
  else buildFlat(keys, captions)

  for (const step of markedKeys) keyElements.get(step)?.classList.add("is-marked")
  for (const step of playingTones.keys()) keyElements.get(step)?.classList.add("is-active")
}

function buildPiano(keys: Key[], captions: Map<number, string>): void {
  keyboardEl.className = "tp-keyboard is-piano"
  const piano = element("div", "tp-piano")

  let x = 0
  for (const key of keys) {
    const div = element("div", `tp-key ${key.black ? "black" : "white"}`)
    div.dataset.step = String(key.step)
    div.title = `${key.label} · ${hz(key.frequency)} Hz · ${signed(key.cents)}¢ from equal`
    if (key.black) {
      div.style.left = `${x - BLACK_WIDTH / 2}px`
      div.style.width = `${BLACK_WIDTH}px`
    } else {
      div.style.left = `${x}px`
      div.style.width = `${WHITE_WIDTH}px`
      x += WHITE_WIDTH
    }
    if (key.degree === 0) div.append(element("span", "tp-key-label", key.label))
    const hint = captions.get(key.step)
    if (hint) div.append(element("kbd", undefined, hint))
    attachKey(div, key.step)
    keyElements.set(key.step, div)
    piano.appendChild(div)
  }
  piano.style.width = `${x}px`
  keyboardEl.appendChild(piano)
  scrollKeyboardTo(60)
}

function buildFlat(keys: Key[], captions: Map<number, string>): void {
  keyboardEl.className = "tp-keyboard is-flat"
  for (const key of keys) {
    const div = element("div", "tp-key")
    div.dataset.step = String(key.step)
    div.dataset.degree = String(key.degree)
    div.title = `${key.label} · ${hz(key.frequency)} Hz`

    const label = element("span", "tp-key-label", key.label)
    const cents = element("span", "tp-key-cents", key.cents === 0 ? "0" : signed(key.cents))
    div.append(label, cents)
    const hint = captions.get(key.step)
    if (hint) div.append(element("kbd", undefined, hint))
    attachKey(div, key.step)
    keyElements.set(key.step, div)
    keyboardEl.appendChild(div)
  }
  scrollKeyboardTo(5 * scale.count)
}

/** Scrolls the keyboard so the given step sits a little left of centre. */
function scrollKeyboardTo(step: number): void {
  const target = keyElements.get(step)
  if (!target) return
  const left = target.offsetLeft - keyboardEl.clientWidth / 3
  keyboardEl.scrollLeft = Math.max(0, left)
}

/**
 * One key. A note starts on pointer-down and stops on the matching pointer-up,
 * tracked per pointer id so several fingers hold several notes and neither a
 * bare hover nor a drag starts anything. Shift-click marks the key instead.
 */
function attachKey(div: HTMLElement, step: number): void {
  div.addEventListener("pointerdown", (event) => {
    event.preventDefault()
    if (event.shiftKey) {
      markOrUnmarkKey(step)
      return
    }
    pointers.set(event.pointerId, step)
    noteOn(step)
  })
}

function releasePointer(event: PointerEvent): void {
  const step = pointers.get(event.pointerId)
  if (step === undefined) return
  pointers.delete(event.pointerId)
  noteOff(step)
}

export function keyActive(step: number, active: boolean): void {
  keyElements.get(step)?.classList.toggle("is-active", active)
  const degree = ((step % scale.count) + scale.count) % scale.count
  for (const row of degreesBody.querySelectorAll<HTMLTableRowElement>("tr")) {
    if (Number(row.dataset.degree) !== degree) continue
    const stillHeld = [...playingTones.keys()].some(
      (held) => ((held % scale.count) + scale.count) % scale.count === degree,
    )
    row.classList.toggle("is-playing", active || stillHeld)
  }
}

// ---------------------------------------------------------------------------
// Marking and sharing

export function markKey(step: number): void {
  if (!markedKeys.includes(step)) markedKeys.push(step)
  markedKeys.sort((a, b) => a - b)
  keyElements.get(step)?.classList.add("is-marked")
  showMarkedButtons(true)
}

export function unmarkKey(step: number): void {
  const index = markedKeys.indexOf(step)
  if (index > -1) markedKeys.splice(index, 1)
  keyElements.get(step)?.classList.remove("is-marked")
  if (markedKeys.length === 0) showMarkedButtons(false)
}

export function unmarkAll(): void {
  for (const step of [...markedKeys]) unmarkKey(step)
}

export function showMarkedButtons(shown: boolean): void {
  markedButtons.hidden = !shown
}

function markOrUnmarkKey(step: number): void {
  if (markedKeys.includes(step)) unmarkKey(step)
  else markKey(step)
  syncUrl()
}

// ---------------------------------------------------------------------------
// Staff and log

let emptyStaffSvg: string | null = null
let lastLogged = ""

export function playingTonesChanged(): void {
  const steps = [...playingTones.keys()].sort((a, b) => a - b)

  output.hidden = !scale.twelve_tone
  if (scale.twelve_tone) {
    if (steps.length === 0) {
      emptyStaffSvg ??= wasm.empty_staff()
      output.innerHTML = emptyStaffSvg
    } else {
      output.innerHTML = wasm.convert_notes(steps.map((step) => playingTones.get(step)!.name))
    }
  }

  if (steps.length === 0) {
    lastLogged = ""
    return
  }
  const label = steps.map((step) => prettyName(playingTones.get(step)!.name)).join(" ")
  if (label !== lastLogged) {
    lastLogged = label
    log(label, steps)
  }
}

/** music21 writes a natural note as `CN4`; a reader wants `C4`. */
function prettyName(name: string): string {
  return name.replace(/N(-?\d+)$/, "$1")
}

function log(label: string, steps: number[]): void {
  const row = element("div", "tool-log-row")

  const share = button("share")
  share.addEventListener("click", () => copyLink(steps))

  const text = element("p")
  const chord = scale.twelve_tone && steps.length > 1 ? ` · ${wasm.get_chord_name()}` : ""
  const pitches = steps.map((step) => hz(playingTones.get(step)!.freq)).join(" ")
  text.textContent = `${label}${chord}`
  text.title = `${pitches} Hz`

  row.append(share, text)
  logContainer.insertBefore(row, logContainer.firstChild)
  while (logContainer.children.length > 40) logContainer.lastChild?.remove()
}

// ---------------------------------------------------------------------------
// Degrees

function renderDegrees(): void {
  degreesBody.replaceChildren()
  const root = 5 * scale.count
  scale.degrees.forEach((degree, index) => {
    const row = element("tr")
    row.dataset.degree = String(index % scale.count)
    const step = root + index
    const name = scale.twelve_tone
      ? prettyName(wasm.step_name(step))
      : index === scale.count
        ? `1+1`
        : String(index + 1)
    row.append(
      element("td", undefined, String(index)),
      element("td", undefined, name),
      element("td", "tp-ratio", degree.ratio_label),
      element("td", undefined, hz(degree.frequency)),
    )
    const cents = element("td")
    cents.appendChild(centsBar(degree.from_equal))
    row.appendChild(cents)
    row.addEventListener("click", () => playOnce(step))
    degreesBody.appendChild(row)
  })
}

function centsBar(cents: number): HTMLElement {
  const wrapper = element("span", "tp-cents")
  const track = element("span", "tp-cents-track")
  const fill = element("span", `tp-cents-fill${cents < 0 ? " is-negative" : ""}`)
  fill.style.width = `${Math.min(50, Math.abs(cents) * 1.5)}%`
  track.appendChild(fill)
  wrapper.append(element("span", undefined, `${signed(cents)}¢`), track)
  return wrapper
}

// ---------------------------------------------------------------------------
// Playing without a keyboard

let sequence: ReturnType<typeof setTimeout>[] = []

function stopSequence(): void {
  for (const id of sequence) clearTimeout(id)
  sequence = []
}

function playOnce(step: number): void {
  audio()
  noteOn(step)
  sequence.push(setTimeout(() => noteOff(step), 450))
}

function playScale(): void {
  stopSequence()
  stopAllTones()
  audio()
  const root = 5 * scale.count
  const degrees = scale.twelve_tone
    ? [0, 2, 4, 5, 7, 9, 11, 12]
    : Array.from({ length: scale.count + 1 }, (_, index) => index)
  const step = 360
  degrees.forEach((degree, index) => {
    sequence.push(setTimeout(() => noteOn(root + degree), index * step))
    sequence.push(setTimeout(() => noteOff(root + degree), index * step + step * 0.9))
  })
}

// ---------------------------------------------------------------------------
// Chord namer

function updateChordName(): void {
  const notes = chordInput.value.trim()
  if (!notes) {
    chordNameOutput.textContent = ""
    chordDetailsOutput.textContent = ""
    return
  }
  chordNameOutput.textContent = wasm.chordname(notes)
  chordDetailsOutput.textContent = wasm.chord_details(notes)
}

// ---------------------------------------------------------------------------
// MIDI file

let midiFile: ArrayBuffer | null = null

function readMidiFile(): Promise<ArrayBuffer> {
  if (midiFile) return Promise.resolve(midiFile)
  const files = fileInput.files
  if (!files || files.length === 0) return Promise.reject(new Error("choose a file first"))
  return files[0].arrayBuffer().then((buffer) => {
    midiFile = buffer
    return buffer
  })
}

function playFile(): void {
  audio()
  readMidiFile()
    .then(playMIDIFile)
    .catch((error: unknown) =>
      setStatus(`Could not play the MIDI file: ${formatError(error)}`, "error"),
    )
}

// ---------------------------------------------------------------------------
// Wiring

export function build(): void {
  search.addEventListener("focus", openLibrary)
  search.addEventListener("input", () => {
    openLibrary()
    renderLibrary()
  })
  search.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      closeLibrary()
      search.blur()
    }
    if (event.key === "Enter") {
      const first = list.querySelector<HTMLButtonElement>(".tp-item")
      first?.click()
    }
  })
  browseToggle.addEventListener("click", () => {
    if (libraryPanel.hidden) {
      openLibrary()
      search.focus()
    } else {
      closeLibrary()
    }
  })
  chips.addEventListener("click", (event) => {
    const chip = (event.target as HTMLElement).closest<HTMLButtonElement>("button")
    if (!chip) return
    kind = (chip.dataset.kind ?? "all") as Kind
    for (const node of chips.querySelectorAll("button")) {
      node.classList.toggle("is-active", node === chip)
    }
    renderLibrary()
  })
  for (const chip of chips.querySelectorAll<HTMLElement>("button")) {
    const count = chip.querySelector("small")
    if (!count) continue
    switch (chip.dataset.kind) {
      case "system":
        count.textContent = String(library.systems.length)
        break
      case "temperament":
        count.textContent = String(library.temperaments.length)
        break
      case "equal":
        count.textContent = String(library.equal.length)
        break
      case "scala":
        count.textContent = library.scala_count.toLocaleString()
        break
    }
  }

  soundMethod.addEventListener("change", () => {
    setEngine(soundMethod.value)
    stopAllTones()
  })
  waveform.addEventListener("change", () => setWaveform(waveform.value))
  volumeSlider.addEventListener("input", () => setVolume(parseFloat(volumeSlider.value)))
  setEngine(soundMethod.value)
  setWaveform(waveform.value)
  setVolume(parseFloat(volumeSlider.value))
  octaveInput.addEventListener("change", () => {
    setOctaveShift(clamp(octaveInput.value, -4, 4, 0))
    octaveInput.value = String(octaveShift)
    stopAllTones()
    renderKeyboard()
    syncUrl()
  })
  layoutSelect.addEventListener("change", () => {
    stopAllTones()
    applyLayout()
    renderKeyboard()
  })

  playScaleButton.addEventListener("click", playScale)
  stopButton.addEventListener("click", () => {
    stopSequence()
    stopMIDIFile()
    stopAllTones()
  })
  shareButton.addEventListener("click", () => copyLink(markedKeys))

  midiButton.addEventListener("click", () => void connectMidi())
  playFileButton.addEventListener("click", playFile)
  stopFileButton.addEventListener("click", () => {
    stopMIDIFile()
    stopAllTones()
  })
  fileInput.addEventListener("change", () => {
    midiFile = null
  })

  nameChord.addEventListener("click", updateChordName)
  clearChord.addEventListener("click", () => {
    chordInput.value = ""
    updateChordName()
  })
  chordInput.addEventListener("input", updateChordName)

  playMarked.addEventListener("click", () => {
    audio()
    markedKeys.forEach((step) => _noteOn(step))
    playingTonesChanged()
    sequence.push(setTimeout(stopAllTones, 1500))
  })
  shareMarked.addEventListener("click", () => copyLink(markedKeys))

  document.addEventListener("pointerup", releasePointer)
  document.addEventListener("pointercancel", releasePointer)

  scaleChanged()
}
