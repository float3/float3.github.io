import { noteOn, noteOff, wasm } from "./index.js"
import { midiMultiplier } from "./config.js"

type MidiMessageLike = {
  data?: ArrayLike<number> | null
}

type MidiInputLike = {
  onmidimessage: ((event: MidiMessageLike) => void) | null
}

type MidiAccessLike = {
  inputs?: {
    values?: () => Iterator<unknown>
  }
}

export function requestMIDI(): void {
  if (!navigator.requestMIDIAccess) {
    alert("WebMIDI is not supported in this browser.")
    return
  }

  navigator.requestMIDIAccess().then(onMIDISuccess).catch(onMIDIFailure)
}

function onMIDISuccess(midiAccess: unknown): void {
  const input = firstMidiInput(midiAccess)

  if (input) {
    input.onmidimessage = onMIDIMessage
  } else {
    alert("No MIDI input devices found.")
  }
}

function onMIDIFailure(error: DOMException): void {
  console.error("MIDI Access failed:", error)
}

function firstMidiInput(midiAccess: unknown): MidiInputLike | undefined {
  const inputs = (midiAccess as MidiAccessLike).inputs
  const input = inputs?.values?.().next().value

  if (isMidiInputLike(input)) {
    return input
  }

  return undefined
}

function isMidiInputLike(value: unknown): value is MidiInputLike {
  return !!value && typeof value === "object" && "onmidimessage" in value
}

function onMIDIMessage(event: MidiMessageLike): void {
  const data = event.data
  if (!data || data.length < 3) return

  const status = data[0]
  const tone_index = data[1]
  const velocity = data[2]
  const is_note_on = (status & 240) === 144
  const is_note_off = (status & 240) === 128

  if (is_note_off) {
    noteOff(tone_index)
  }
  if (is_note_on) {
    noteOn(tone_index, velocity)
  }
}

let timeoutIds: NodeJS.Timeout[] = []

export function stopMIDIFile(): void {
  timeoutIds.forEach((id) => clearTimeout(id))
  timeoutIds = []
}

export function playMIDIFile(midiFile: ArrayBuffer): void {
  // The wasm reads the file and hands back the notes flat: key, velocity,
  // start and end, four numbers at a time. Parsing it in the browser used to
  // mean @tonejs/midi, a full object model of tracks and controllers built so
  // that this loop could read four numbers off each note.
  const notes = wasm.parse_midi(new Uint8Array(midiFile))

  for (let at = 0; at < notes.length; at += 4) {
    const key = notes[at]
    const velocity = notes[at + 1]
    const start = notes[at + 2] * midiMultiplier
    const end = notes[at + 3] * midiMultiplier

    timeoutIds.push(setTimeout(() => noteOn(key, velocity), start))
    timeoutIds.push(setTimeout(() => noteOff(key), end))
  }
}
