/**
 * The recursive tuning post: a global scale that places roots, a local scale
 * that tunes what sounds above each of them, and three ways to hear the pair.
 *
 * The wasm owns the pair and answers every frequency and name; this file draws
 * what it says and keeps track of the notes being held. A held note is a step
 * of the local scale, so picking another root moves it to that root's row.
 */

import * as sound from "./audio.js"
import { picker, type Picker } from "./picker.js"
import type { Chord, Library, Matrix, ScalaEntry, Voice } from "./types.js"

type Wasm = typeof import("wasm-tuningplayground")
type Rendering = keyof Pick<Voice, "recursive" | "fixed" | "equal">

const DEFAULT_ROOT_HZ = 261.6255653005986
const DEFAULT_ID = "FiveLimit"
const CHORD_SECONDS = 1.8
const SCALA_LIMIT = 60

const NOTE_CLASSES = [
  "note-c",
  "note-c-sharp",
  "note-d",
  "note-d-sharp",
  "note-e",
  "note-f",
  "note-f-sharp",
  "note-g",
  "note-g-sharp",
  "note-a",
  "note-a-sharp",
  "note-b",
]

/** Every physical key the page listens to; the wasm says what each one does. */
const CODES = [
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
]

const PRINTED: Record<string, string> = {
  Comma: ",",
  Period: ".",
  Slash: "/",
  Semicolon: ";",
  Quote: "'",
  BracketLeft: "[",
  BracketRight: "]",
  Minus: "-",
  Equal: "=",
}

function byId<T extends HTMLElement>(id: string): T {
  const node = document.getElementById(id)
  if (!node) throw new Error(`the page has no #${id}`)
  return node as T
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

function caption(code: string): string {
  if (code.startsWith("Key")) return code.slice(3).toLowerCase()
  if (code.startsWith("Digit")) return code.slice(5)
  return PRINTED[code] ?? code
}

function hz(value: number): string {
  return value.toFixed(value >= 1000 ? 1 : value >= 100 ? 2 : 3)
}

function signed(value: number, digits = 1): string {
  const rounded = Number(value.toFixed(digits))
  if (rounded === 0) return (0).toFixed(digits)
  return rounded > 0 ? `+${rounded.toFixed(digits)}` : `−${Math.abs(rounded).toFixed(digits)}`
}

function clamp(text: string, min: number, max: number, fallback: number): number {
  const value = Number.parseFloat(text)
  return Number.isFinite(value) ? Math.min(max, Math.max(min, value)) : fallback
}

const status = byId<HTMLElement>("recursiveTuningStatus")
const linkBox = byId<HTMLInputElement>("rtLink")
const rootInput = byId<HTMLInputElement>("rtRoot")
const octaveInput = byId<HTMLInputElement>("rtOctave")
const timbreSelect = byId<HTMLSelectElement>("rtTimbre")
const volumeInput = byId<HTMLInputElement>("rtVolume")
const shareButton = byId<HTMLButtonElement>("rtShare")
const pairsEl = byId<HTMLElement>("rtPairs")
const summaryEl = byId<HTMLElement>("rtSummary")
const rootsEl = byId<HTMLElement>("rtRoots")
const notesEl = byId<HTMLElement>("rtNotes")
const chordButton = byId<HTMLButtonElement>("rtChord")
const stopButton = byId<HTMLButtonElement>("rtStop")
const droneBox = byId<HTMLInputElement>("rtDrone")
const progressionEl = byId<HTMLElement>("rtProgression")
const matrixEl = byId<HTMLElement>("rtMatrix")

let wasm: Wasm
let globalPicker: Picker
let localPicker: Picker
let matrix: Matrix
let progression: Chord[] = []
let globalId = DEFAULT_ID
let localId = DEFAULT_ID
let rootHz = DEFAULT_ROOT_HZ
let currentRoot = 0
let octave = 0

/** Local steps being held, by what holds them: a key code or a pointer. */
const held = new Map<string, number>()
/** The matrix's cells, by the root of their row. */
const rowCells = new Map<number, HTMLElement[]>()
const progressionTimers: number[] = []

function setStatus(message: string, state: "loading" | "ready" | "error"): void {
  status.dataset.state = state
  status.textContent = message
}

function octaveFactor(): number {
  return 2 ** octave
}

function frequency(step: number): number {
  return wasm.recursive_frequency(currentRoot, step) * octaveFactor()
}

/** `?global=` and `?local=` name the scales, `?root=` the root in hertz. */
function readUrl(): void {
  const params = new URLSearchParams(window.location.search)
  globalId = params.get("global") ?? DEFAULT_ID
  localId = params.get("local") ?? globalId
  rootHz = clamp(params.get("root") ?? "", 20, 2000, DEFAULT_ROOT_HZ)
}

function shareUrl(): string {
  const url = new URL(window.location.href)
  url.search = ""
  url.hash = ""
  url.searchParams.set("global", globalId)
  url.searchParams.set("local", localId)
  if (Math.abs(rootHz - DEFAULT_ROOT_HZ) > 1e-9) url.searchParams.set("root", String(rootHz))
  return url.toString()
}

function select(global: string, local: string): boolean {
  try {
    matrix = JSON.parse(wasm.recursive_select(global, local, rootHz)) as Matrix
  } catch (error: unknown) {
    setStatus(error instanceof Error ? error.message : String(error), "error")
    return false
  }
  globalId = global
  localId = local
  progression = JSON.parse(wasm.recursive_progression_json()) as Chord[]
  currentRoot = Math.min(currentRoot, matrix.global.count - 1)
  setStatus("", "ready")
  render()
  window.history.replaceState(window.history.state, "", shareUrl())
  return true
}

function pick(side: "global" | "local", id: string): void {
  if (linkBox.checked) select(id, id)
  else if (side === "global") select(id, localId)
  else select(globalId, id)
}

function render(): void {
  globalPicker.show(matrix.global)
  localPicker.show(matrix.local)
  for (const node of pairsEl.querySelectorAll<HTMLButtonElement>("button")) {
    node.classList.toggle(
      "is-active",
      node.dataset.global === globalId && node.dataset.local === localId,
    )
  }
  renderSummary()
  renderRoots()
  renderNotes()
  renderProgression()
  renderMatrix()
  retuneHeld()
}

function renderSummary(): void {
  const { global, local, largest_shift: shift } = matrix
  let text: string
  if (shift < 0.001) {
    text = `Nothing moves: every note of this pair is a note ${global.name} already has on its own.`
  } else if (matrix.aligned) {
    text = `Recursing moves notes up to ${shift.toFixed(3)} cents from where ${global.name} has them on its own.`
  } else {
    text = `${global.name} has ${global.count} notes to its period and ${local.name} has ${local.count}, so a column is not the same note in every row. Each note is compared with the note of ${global.name} nearest it, and the farthest is ${shift.toFixed(3)} cents away.`
  }
  if (matrix.truncated) {
    text += ` The matrix shows the first ${matrix.rows.length} roots and ${matrix.columns.length - 1} steps; the keyboard plays the rest.`
  }
  summaryEl.textContent = text
}

function captions(keyOf: (code: string) => number): Map<number, string> {
  const map = new Map<number, string>()
  for (const code of CODES) {
    const index = keyOf(code)
    if (index >= 0 && !map.has(index)) map.set(index, caption(code))
  }
  return map
}

function renderRoots(): void {
  const hints = captions((code) => wasm.recursive_root_key(code))
  rootsEl.replaceChildren(
    ...matrix.rows.map((row) => {
      const node = button("", `rt-root${row.root === currentRoot ? " is-active" : ""}`)
      node.append(
        element("strong", undefined, row.label),
        element("small", undefined, row.ratio_label),
      )
      const hint = hints.get(row.root)
      if (hint) node.append(element("kbd", undefined, hint))
      node.addEventListener("click", () => setRoot(row.root))
      return node
    }),
  )
}

/** The label of a local step: its ratio, and how many periods up it is past the first. */
function stepLabel(step: number): string {
  if (step < matrix.columns.length) return matrix.columns[step].ratio_label
  const count = matrix.local.count
  const degree = step % count
  return `${matrix.columns[degree]?.ratio_label ?? degree} +${Math.floor(step / count)}`
}

function renderNotes(): void {
  const count = matrix.local.count
  const last = Math.max(count, Math.min(2 * count, 32))
  const hints = captions((code) => wasm.recursive_note_key(code))
  const steps = new Set(held.values())
  const keys: HTMLElement[] = []
  for (let step = 0; step <= last; step++) {
    const note = NOTE_CLASSES[wasm.recursive_note_class(currentRoot, step)]
    const classes = ["rt-key", note]
    if (step % count === 0) classes.push("is-period")
    if (steps.has(step)) classes.push("is-active")
    const node = element("div", classes.join(" "))
    node.dataset.step = String(step)
    node.append(
      element("span", "rt-key-name", wasm.recursive_note_name(currentRoot, step)),
      element("span", "rt-key-ratio", stepLabel(step)),
      element("small", undefined, hz(frequency(step))),
    )
    const hint = hints.get(step)
    if (hint) node.append(element("kbd", undefined, hint))
    node.addEventListener("pointerdown", (event) => {
      event.preventDefault()
      noteOn(`pointer:${event.pointerId}`, step)
    })
    keys.push(node)
  }
  notesEl.replaceChildren(...keys)
}

function setRoot(root: number): void {
  if (root === currentRoot) return
  currentRoot = root
  for (const [index, node] of [...rootsEl.children].entries()) {
    node.classList.toggle("is-active", index === root)
  }
  for (const row of matrixEl.querySelectorAll<HTMLTableRowElement>("tbody tr")) {
    row.classList.toggle("is-root", Number(row.dataset.root) === root)
  }
  renderNotes()
  retuneHeld()
  markActive()
}

function noteOn(holder: string, step: number): void {
  if (held.has(holder)) return
  held.set(holder, step)
  sound.start(holder, frequency(step))
  markActive()
}

function noteOff(holder: string): void {
  if (!held.delete(holder)) return
  sound.stop(holder)
  markActive()
}

function retuneHeld(): void {
  for (const [holder, step] of held) sound.retune(holder, frequency(step))
}

function markActive(): void {
  const steps = new Set(held.values())
  for (const node of notesEl.querySelectorAll<HTMLElement>(".rt-key")) {
    node.classList.toggle("is-active", steps.has(Number(node.dataset.step)))
  }
  for (const [root, cells] of rowCells) {
    for (const cell of cells) {
      cell.classList.toggle(
        "is-active",
        root === currentRoot && steps.has(Number(cell.dataset.step)),
      )
    }
  }
}

function releaseAll(): void {
  for (const holder of [...held.keys()]) noteOff(holder)
}

function typing(): boolean {
  const active = document.activeElement
  if (active instanceof HTMLInputElement) {
    return !["checkbox", "radio", "range", "button"].includes(active.type)
  }
  return (
    active instanceof HTMLTextAreaElement ||
    active instanceof HTMLSelectElement ||
    (active instanceof HTMLElement && active.isContentEditable)
  )
}

function renderProgression(): void {
  progressionEl.replaceChildren(
    ...progression.map((chord, index) => {
      const node = element("div", "rt-chord")
      node.dataset.index = String(index)
      node.append(element("strong", undefined, chord.name))
      const moved = chord.voices.filter((voice) => Math.abs(voice.from_fixed) >= 0.05)
      if (moved.length === 0) node.append(element("small", undefined, "unmoved"))
      for (const voice of moved) {
        node.append(element("small", undefined, `${voice.name} ${signed(voice.from_fixed)}¢`))
      }
      return node
    }),
  )
}

function markChord(index: number): void {
  for (const node of progressionEl.querySelectorAll<HTMLElement>(".rt-chord")) {
    node.classList.toggle("is-playing", Number(node.dataset.index) === index)
  }
}

function stopProgression(): void {
  for (const timer of progressionTimers) window.clearTimeout(timer)
  progressionTimers.length = 0
  markChord(-1)
}

function playProgression(rendering: Rendering): void {
  stopProgression()
  sound.stopAll()
  const context = sound.audio()
  const begin = context.currentTime + 0.08
  const factor = octaveFactor()
  const at = (seconds: number): number => Math.max(0, (seconds - context.currentTime) * 1000)

  progression.forEach((chord, index) => {
    const start = begin + index * CHORD_SECONDS
    for (const voice of chord.voices) {
      sound.schedule(voice[rendering] * factor, start, CHORD_SECONDS - 0.06)
    }
    progressionTimers.push(window.setTimeout(() => markChord(index), at(start)))
  })

  const total = progression.length * CHORD_SECONDS
  if (droneBox.checked) sound.schedule((matrix.root_hz * factor) / 2, begin, total, 0.14)
  progressionTimers.push(window.setTimeout(() => markChord(-1), at(begin + total)))
}

function renderMatrix(): void {
  rowCells.clear()
  const factor = octaveFactor()
  const table = element("table", "rt-matrix")

  const head = element("tr")
  head.append(element("th", undefined, "root / step"))
  for (const column of matrix.columns) {
    const th = element("th")
    th.append(
      element("span", undefined, column.ratio_label),
      element("small", undefined, `${column.cents.toFixed(1)}¢`),
    )
    head.append(th)
  }
  const thead = element("thead")
  thead.append(head)

  const tbody = element("tbody")
  for (const row of matrix.rows) {
    const tr = element("tr", row.root === currentRoot ? "is-root" : undefined)
    tr.dataset.root = String(row.root)
    const th = element("th")
    const rowButton = button(`${row.label} · ${row.ratio_label}`, "rt-row-head")
    rowButton.addEventListener("click", () => setRoot(row.root))
    th.append(rowButton)
    tr.append(th)

    const cells: HTMLElement[] = []
    for (const cell of row.cells) {
      const td = element("td")
      const moved = Math.abs(cell.from_fixed) >= 0.001
      const span = element(
        "span",
        `recursive-note-cell ${NOTE_CLASSES[cell.note]}${moved ? "" : " is-unmoved"}`,
      )
      span.dataset.note = cell.name
      span.dataset.step = String(cell.step)
      span.title = `${cell.name}, ${hz(cell.frequency * factor)} Hz, ${signed(cell.from_fixed, 3)}¢ from ${matrix.global.name} alone`
      span.append(
        element("code", undefined, `${hz(cell.frequency * factor)} Hz`),
        element("small", "tet-cents", `${signed(cell.from_fixed)}¢`),
      )
      span.addEventListener("pointerdown", (event) => {
        event.preventDefault()
        setRoot(row.root)
        noteOn(`pointer:${event.pointerId}`, cell.step)
      })
      cells.push(span)
      td.append(span)
      tr.append(td)
    }
    rowCells.set(row.root, cells)
    tbody.append(tr)
  }

  table.append(thead, tbody)
  matrixEl.replaceChildren(table)
}

function wire(): void {
  linkBox.addEventListener("change", () => {
    if (linkBox.checked && globalId !== localId) select(globalId, globalId)
  })
  rootInput.addEventListener("change", () => {
    rootHz = clamp(rootInput.value, 20, 2000, DEFAULT_ROOT_HZ)
    rootInput.value = String(rootHz)
    select(globalId, localId)
  })
  octaveInput.addEventListener("change", () => {
    octave = Math.round(clamp(octaveInput.value, -3, 3, 0))
    octaveInput.value = String(octave)
    renderNotes()
    renderMatrix()
    retuneHeld()
  })
  timbreSelect.addEventListener("change", () => sound.setTimbre(timbreSelect.value))
  volumeInput.addEventListener("input", () => sound.setVolume(Number(volumeInput.value)))
  shareButton.addEventListener("click", () => {
    void navigator.clipboard.writeText(shareUrl()).then(() => {
      shareButton.textContent = "Copied"
      window.setTimeout(() => (shareButton.textContent = "Copy link"), 1500)
    })
  })

  for (const node of pairsEl.querySelectorAll<HTMLButtonElement>("button")) {
    node.addEventListener("click", () => {
      const global = node.dataset.global ?? DEFAULT_ID
      const local = node.dataset.local ?? global
      linkBox.checked = global === local
      select(global, local)
    })
  }

  chordButton.addEventListener("click", () => {
    const start = sound.audio().currentTime + 0.02
    for (const step of wasm.recursive_triad()) sound.schedule(frequency(step), start, 1.6)
  })
  for (const node of document.querySelectorAll<HTMLButtonElement>(
    "#recursiveTuning [data-render]",
  )) {
    node.addEventListener("click", () => playProgression(node.dataset.render as Rendering))
  }
  stopButton.addEventListener("click", () => {
    stopProgression()
    releaseAll()
    sound.stopAll()
  })

  document.addEventListener("keydown", (event) => {
    if (event.repeat || event.metaKey || event.ctrlKey || event.altKey || typing()) return
    const root = wasm.recursive_root_key(event.code)
    if (root >= 0) {
      event.preventDefault()
      if (root < matrix.global.count) setRoot(root)
      return
    }
    const step = wasm.recursive_note_key(event.code)
    if (step >= 0) {
      event.preventDefault()
      noteOn(`key:${event.code}`, step)
    }
  })
  document.addEventListener("keyup", (event) => noteOff(`key:${event.code}`))
  window.addEventListener("pointerup", (event) => noteOff(`pointer:${event.pointerId}`))
  window.addEventListener("pointercancel", (event) => noteOff(`pointer:${event.pointerId}`))
  window.addEventListener("blur", releaseAll)
  document.addEventListener("visibilitychange", () => {
    if (document.hidden) releaseAll()
  })
}

void (async () => {
  try {
    wasm = await import("wasm-tuningplayground")
    wasm.main()
    const library = JSON.parse(wasm.library_json()) as Library
    const search = (query: string): ScalaEntry[] =>
      JSON.parse(wasm.scala_search_json(query, SCALA_LIMIT)) as ScalaEntry[]

    readUrl()
    rootInput.value = String(rootHz)
    linkBox.checked = globalId === localId
    globalPicker = picker({
      root: byId("rtGlobal"),
      library,
      search,
      onPick: (id) => pick("global", id),
    })
    localPicker = picker({
      root: byId("rtLocal"),
      library,
      search,
      onPick: (id) => pick("local", id),
    })
    wire()
    if (!select(globalId, localId)) select(DEFAULT_ID, DEFAULT_ID)
  } catch (error: unknown) {
    setStatus(
      `Could not start the recursive tuning explorer: ${error instanceof Error ? error.message : String(error)}`,
      "error",
    )
    window.setTimeout(() => {
      throw error
    }, 0)
  }
})()
