import * as abcjs from "abcjs"
import { _noteOn, markedKeys, noteOff, noteOn, playingTones, stopAllTones } from "./index.js"
import { playMIDIFile, stopMIDIFile } from "./MIDI.js"
import { wasm } from "./index.js"

const octaveSize = document.getElementById("octaveSize") as HTMLInputElement
const stepSize = document.getElementById("stepSize") as HTMLInputElement
const fileInput = document.getElementById("fileInput") as HTMLInputElement
export const soundMethod = document.getElementById("soundMethod") as HTMLSelectElement
const keymapSelect = document.getElementById("keymapSelect") as HTMLSelectElement
// const linkInput = document.getElementById("linkInput") as HTMLInputElement;

const logContainer = document.getElementById("logContainer") as HTMLDivElement
const stepSizeParent = stepSize.parentElement as HTMLDivElement
export const markedButtons = document.getElementById("markedButtons") as HTMLDivElement

export const playButton = document.getElementById("playButton") as HTMLButtonElement
export const shareMarked = document.getElementById("shareMarked") as HTMLButtonElement
export const playMarked = document.getElementById("playMarked") as HTMLButtonElement

const stopButton = document.getElementById("stopButton") as HTMLButtonElement

export const tuningSelect = document.getElementById("tuningSelect") as HTMLSelectElement

export const volumeSlider = document.getElementById("volumeSlider") as HTMLInputElement

const transpose = document.getElementById("transpose") as HTMLInputElement

export const output = document.getElementById("output") as HTMLElement
const chordInput = document.getElementById("chordInput") as HTMLInputElement
const nameChord = document.getElementById("nameChord") as HTMLButtonElement
const clearChord = document.getElementById("clearChord") as HTMLButtonElement
const chordNameOutput = document.getElementById("chordNameOutput") as HTMLDivElement
const chordDetailsOutput = document.getElementById("chordDetailsOutput") as HTMLDivElement

type TuningPlaygroundStatusState = "loading" | "ready" | "error"

export function setTuningPlaygroundStatus(
  message: string,
  state: TuningPlaygroundStatusState = "loading",
): void {
  const status = document.getElementById("tuningPlaygroundStatus")
  if (!status) return
  status.dataset.state = state
  status.textContent = message
}

octaveSize.onchange = handleTuningSelectChange
tuningSelect.onchange = handleTuningSelectChange
stepSize.onchange = handleTuningSelectChange
fileInput.onchange = fileInputChange
transpose.onchange = transposeChange
volumeSlider.onchange = volumeChange
keymapSelect.onchange = keymapChange
chordInput.oninput = updateChordName
// linkInput.onchange = linkInputChange;

stopButton.onclick = stop
playMarked.onclick = playMarkedKeys
shareMarked.onclick = sharedMarkedKeys
nameChord.onclick = updateChordName
clearChord.onclick = clearChordInput

export let tranposeValue = 0
function transposeChange(): void {
  tranposeValue = parseInt(transpose.value)
}

export let volumeValue = 0.25
function volumeChange(): void {
  volumeValue = parseFloat(volumeSlider.value)
}

let midiFile: ArrayBuffer
let midiFilePromise: Promise<ArrayBuffer> | null = null

function initOrGetMidiFile(): Promise<ArrayBuffer> {
  if (!midiFilePromise) {
    midiFilePromise = fetch("/misc/blobs/jm_mozdi.mid")
      .then((response) => {
        if (!response.ok) {
          throw new Error(`Could not load default MIDI file: ${response.status}`)
        }
        return response.arrayBuffer()
      })
      .then((buffer) => {
        midiFile = buffer
        return midiFile
      })
      .catch((error) => {
        setTuningPlaygroundStatus(`Could not load default MIDI file: ${formatError(error)}`, "error")
        throw error
      })
  }
  return midiFilePromise
}

function fileInputChange(event: Event): Promise<void> {
  return new Promise((resolve, reject) => {
    const files = (event.target as HTMLInputElement).files
    if (files && files.length > 0) {
      const reader = new FileReader()
      reader.onload = (e) => {
        midiFile = e.target!.result as ArrayBuffer
        midiFilePromise = Promise.resolve(midiFile)
        resolve()
      }
      reader.onerror = reject
      reader.readAsArrayBuffer(files[0])
    } else {
      reject(new Error("No file selected"))
    }
  })
}

// export function linkInputChange(): void {
//   const link = linkInput.value;
//   fetch(link)
//     .then((response) => response.arrayBuffer())
//     .then((buffer) => {
//       midiFile = buffer;
//     });
// }

function playMarkedKeys(): void {
  markedKeys.forEach((note) => _noteOn(note, undefined, true))
  playingTonesChanged()
}

function sharedMarkedKeys(): void {
  createAndCopyUrl(markedKeys)()
}

function stop(): void {
  stopMIDIFile()
}

export function play(): void {
  initOrGetMidiFile()
    .then(playMIDIFile)
    .catch((error: unknown) => {
      setTuningPlaygroundStatus(`Could not play MIDI file: ${formatError(error)}`, "error")
    })
}

export function DOMContentLoaded(): void {
  if (!wasm) return

  applySharedTuningFromUrl()
  handleTuningSelectChange()
  keymapChange()
  updateChordName()
}

function keymapChange(): void {
  wasm.set_keymap(keymapSelect.value)
  stopAllTones()
}

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

function clearChordInput(): void {
  chordInput.value = ""
  updateChordName()
}

export function handleTuningSelectChange(): void {
  const octave = positiveInteger(octaveSize.value, 12)
  const step = positiveInteger(stepSize.value, 7)
  octaveSize.value = octave.toString()
  stepSize.value = step.toString()

  switch (tuningSelect.value) {
    case "StepMethod":
      stepSizeParent.hidden = false
      stepSize.hidden = false
      stepSize.readOnly = false
      octaveSize.readOnly = false
      break
    case "EqualTemperament":
      stepSizeParent.hidden = true
      stepSize.hidden = false
      stepSize.readOnly = true
      octaveSize.readOnly = false
      break
    default:
      stepSizeParent.hidden = true
      octaveSize.readOnly = true
      stepSize.hidden = false
      stepSize.readOnly = true
      break
  }

  wasm.set_tuning_system(tuningSelect.value, octave, step)

  if (tuningSelect.value !== "StepMethod" && tuningSelect.value !== "EqualTemperament") {
    octaveSize.value = wasm.get_tuning_size().toString()
  }

  stopAllTones()
}

function adjustOutputSize(): void {
  output.style.width = "300px"
  output.style.height = "200px"
}

export function playingTonesChanged(): void {
  const notes = Object.keys(playingTones).map(Number)

  if (notes.length === 0) {
    abcjs.renderAbc("output", 'X: 1\nL: 1/1\n|""[u]|')
    adjustOutputSize()
    return
  }

  let chordName
  const tones = Object.values(playingTones)
    .map((tone) => tone.name)
    .join(" ")

  if (octaveSize.value === "12") {
    const formatted_notes = wasm.convert_notes(tones.split(" "))
    chordName = wasm.get_chord_name()
    abcjs.renderAbc("output", formatted_notes)
    adjustOutputSize()
  }

  logToDiv(`${tones} | ${chordName}`, notes)
}

function createAndCopyUrl(keys: number[]): () => void {
  const hash = generateHash(keys)
  const url = tuningUrl(hash)
  return function () {
    copyUrl(url).catch((error: unknown) => {
      setTuningPlaygroundStatus(`Could not copy link: ${formatError(error)}`, "error")
    })
  }
}

function generateHash(keys: number[]) {
  const hash = keys.join(",")
  return hash
}

export function logToDiv(message: string, notes: number[]): void {
  const p = document.createElement("p")
  p.textContent = message

  const shareButton = document.createElement("button")
  shareButton.textContent = "Share"
  shareButton.onclick = createAndCopyUrl(notes)

  shareButton.style.marginRight = "10px"
  p.style.marginLeft = "10px"

  const div = document.createElement("div")
  div.style.display = "flex"
  div.style.justifyContent = "left"
  div.style.alignItems = "center"
  div.appendChild(shareButton)
  div.appendChild(p)
  logContainer.insertBefore(div, logContainer.firstChild)
}

export function keyActive(tone_index: number, active: boolean) {
  const keyElement = document.querySelector(`div[data-note="${tone_index}"]`)
  if (keyElement) {
    if (active) keyElement.classList.add("key-active")
    else keyElement.classList.remove("key-active")
  }
}

export function markKey(tone_index: number) {
  if (markedKeys.includes(tone_index)) return
  markedKeys.push(tone_index)
  const keyElement = document.querySelector(`div[data-note="${tone_index}"]`)
  if (keyElement) {
    keyElement.classList.add("key-marked")
  }
  markedButtons.style.display = "block"
}

export function unmarkKey(tone_index: number): void {
  const index = markedKeys.indexOf(tone_index)
  if (index > -1) {
    markedKeys.splice(index, 1)
  }
  const keyElement = document.querySelector(`div[data-note="${tone_index}"]`)
  if (keyElement) {
    keyElement.classList.remove("key-marked")
  }
  if (markedKeys.length === 0) {
    markedButtons.style.display = "none"
  }
}

export function markOrUnmarkKey(tone_index: number) {
  const index = markedKeys.indexOf(tone_index)
  if (index > -1) {
    unmarkKey(tone_index)
  } else {
    markKey(tone_index)
  }
  markedKeys.sort((a, b) => a - b)
  window.location.hash = generateHash(markedKeys)
}

export function addEvents(key: Element) {
  const note = parseInt(key.getAttribute("data-note")!)

  const addEvent = (eventName: string, callback: () => void) => {
    key.addEventListener(eventName, callback)
  }

  key.addEventListener("mousedown", (event) => {
    const mouseEvent = event as MouseEvent
    if (mouseEvent.shiftKey) {
      markOrUnmarkKey(note - tranposeValue)
    } else {
      noteOn(note - tranposeValue)
    }
  })

  // key.addEventListener("mouseup", (event) => {
  //   let mouseEvent = event as MouseEvent;
  //   if (mouseEvent.ctrlKey) {
  //     unmarkKey(note);
  //   } else {
  //     noteOff(note);
  //   }
  // });

  addEvent("mouseup", () => noteOff(note - tranposeValue))

  key.addEventListener("mouseenter", (event) => {
    const mouseEvent = event as MouseEvent
    if (mouseEvent.ctrlKey) {
      return
    }
    noteOn(note - tranposeValue)
  })

  addEvent("mouseleave", () => noteOff(note - tranposeValue))
  addEvent("touchstart", () => noteOn(note - tranposeValue))
  addEvent("touchend", () => noteOff(note - tranposeValue))
}

let sharedTuningApplied = false

function applySharedTuningFromUrl(): void {
  if (sharedTuningApplied) return
  sharedTuningApplied = true

  const params = new URLSearchParams(window.location.search)
  const tuning = params.get("tuning")
  const octave = params.get("octaveSize")
  const step = params.get("stepSize")

  if (tuning && tuningOptionExists(tuning)) {
    tuningSelect.value = tuning
  }

  if (octave) {
    octaveSize.value = positiveInteger(octave, 12).toString()
  }

  if (step) {
    stepSize.value = positiveInteger(step, 7).toString()
  }
}

function tuningOptionExists(value: string): boolean {
  return Array.from(tuningSelect.options).some((option) => option.value === value)
}

function positiveInteger(value: string, fallback: number): number {
  const parsed = Number.parseInt(value, 10)
  if (Number.isFinite(parsed) && parsed > 0) {
    return parsed
  }
  return fallback
}

function currentMarkedHash(): string {
  return markedKeys.length > 0 ? generateHash(markedKeys) : window.location.hash.substring(1)
}

function tuningUrl(hash = currentMarkedHash()): string {
  const url = new URL(window.location.href)
  url.search = ""
  url.searchParams.set("tuning", tuningSelect.value)
  url.searchParams.set("octaveSize", octaveSize.value)
  url.searchParams.set("stepSize", stepSize.value)
  url.hash = hash
  return url.toString()
}

async function copyUrl(url: string): Promise<void> {
  if (navigator.clipboard) {
    await navigator.clipboard.writeText(url)
    return
  }

  const textarea = document.createElement("textarea")
  textarea.value = url
  textarea.style.position = "fixed"
  textarea.style.left = "-9999px"
  document.body.appendChild(textarea)
  textarea.select()
  document.execCommand("copy")
  textarea.remove()
}

function formatError(error: unknown): string {
  if (error instanceof Error) {
    return error.message
  }
  return String(error)
}
