import { noteOn, noteOff } from "./index.js"
import { Midi } from "@tonejs/midi"
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
  if (navigator.requestMIDIAccess) {
    navigator
      .requestMIDIAccess()
      .then((midiAccess: unknown) => onMIDISuccess(midiAccess), onMIDIFailure)
  } else {
    alert("WebMIDI is not supported in this browser.")
  }
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
  const midi = new Midi(midiFile)

  // const tempo = midi.header.tempos[0].bpm;

  midi.tracks.forEach((track) => {
    const startTime: number = 0
    // track.notes.forEach((note) => {
    //   startTime = note.time * midiMultiplier;
    //   return;
    // });
    track.notes.forEach((note) => {
      const noteOnTime = note.time * midiMultiplier - startTime
      const noteOffTime = (note.time + note.duration) * midiMultiplier - startTime
      const velocity = note.velocity
      if (velocity === 1) note.velocity = 127 // fix for some midi files
      const midiNote = note.midi

      timeoutIds.push(setTimeout(() => noteOn(midiNote, velocity), noteOnTime))
      timeoutIds.push(setTimeout(() => noteOff(midiNote), noteOffTime))
    })
  })
}
