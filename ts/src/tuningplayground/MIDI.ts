import { noteOff, noteOn, scale, shift, wasm } from "./index.js"
import { midiMultiplier } from "./config.js"
import { setMidiStatus } from "./UI.js"

const WANTED = "tuningplayground-midi"

let access: MIDIAccess | null = null

/**
 * Whether the reader connected a MIDI device on an earlier visit. The browser
 * asks permission the first time, so it is only asked for on request and
 * remembered after that.
 */
export function midiWanted(): boolean {
  try {
    return window.localStorage.getItem(WANTED) === "1"
  } catch {
    return false
  }
}

function rememberMidi(wanted: boolean): void {
  try {
    if (wanted) window.localStorage.setItem(WANTED, "1")
    else window.localStorage.removeItem(WANTED)
  } catch {
    return
  }
}

/** A MIDI key as a step: middle C is the root, and each key up is one degree. */
function midiToStep(note: number): number {
  return scale.twelve_tone ? note : 5 * scale.count + (note - 60)
}

export async function connectMidi(): Promise<void> {
  if (!navigator.requestMIDIAccess) {
    setMidiStatus("This browser has no Web MIDI. Chrome, Edge and Opera do.", false)
    return
  }
  try {
    access ??= await navigator.requestMIDIAccess({ sysex: false })
    access.onstatechange = wireInputs
    rememberMidi(true)
    wireInputs()
  } catch (error: unknown) {
    rememberMidi(false)
    setMidiStatus(`MIDI: ${error instanceof Error ? error.message : String(error)}`, false)
  }
}

function wireInputs(): void {
  if (!access) return
  let count = 0
  for (const input of access.inputs.values()) {
    input.onmidimessage = onMIDIMessage
    count += 1
  }
  setMidiStatus(
    count === 0
      ? "No MIDI device found. Plug one in and it is picked up."
      : `${count} MIDI input${count === 1 ? "" : "s"} connected. Middle C is the root; each key up is one degree.`,
    true,
  )
}

function onMIDIMessage(event: MIDIMessageEvent): void {
  const data = event.data
  if (!data || data.length < 3) return

  const status = data[0] & 0xf0
  const note = data[1]
  const velocity = data[2]

  if (status === 0x90 && velocity > 0) {
    noteOn(midiToStep(note) + shift(), velocity)
  } else if (status === 0x80 || (status === 0x90 && velocity === 0)) {
    noteOff(midiToStep(note) + shift())
  }
}

let timeoutIds: ReturnType<typeof setTimeout>[] = []

export function stopMIDIFile(): void {
  timeoutIds.forEach((id) => clearTimeout(id))
  timeoutIds = []
}

export function playMIDIFile(midiFile: ArrayBuffer): void {
  // The wasm reads the file and hands back the notes flat: key, velocity,
  // start and end, four numbers at a time.
  const notes = wasm.parse_midi(new Uint8Array(midiFile))

  for (let at = 0; at < notes.length; at += 4) {
    const step = midiToStep(notes[at]) + shift()
    const velocity = notes[at + 1]
    const start = notes[at + 2] * midiMultiplier
    const end = notes[at + 3] * midiMultiplier

    timeoutIds.push(setTimeout(() => noteOn(step, velocity), start))
    timeoutIds.push(setTimeout(() => noteOff(step), end))
  }
}
