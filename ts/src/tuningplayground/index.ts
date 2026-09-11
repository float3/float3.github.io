/**
 * The tuning playground: a keyboard under any of the scales music21-rs knows.
 *
 * The wasm owns the scale and answers what each step sounds and is called;
 * this file owns the audio engine and the notes being held, and UI.ts draws
 * the page. A step is a degree of the scale counted from five periods below
 * the root, which in a twelve-tone octave scale makes it a MIDI note number.
 */

import { connectMidi, midiWanted } from "./MIDI.js"
import { DEFAULT_ROOT_HZ } from "./config.js"
import { keydown, keyup, readMarkedFromHash, visibilityChange } from "./events.js"
import * as ui from "./UI.js"

export let wasm: typeof import("wasm-tuningplayground")

export interface Degree {
  ratio_label: string
  ratio: number
  cents: number
  from_equal: number
  frequency: number
}

export interface Generator {
  ratio: string
  cents: number
}

export interface TemperamentFacts {
  name: string
  page: string
  subgroup: string
  rank: number
  periods_per_equave: number
  generators: Generator[]
  optimization: string
  commas: string[]
  published_moments: string[]
  moments: number[]
}

export interface Scale {
  id: string
  name: string
  family: string
  description: string
  count: number
  period_ratio: number
  period_cents: number
  root_hz: number
  adaptive: boolean
  twelve_tone: boolean
  degrees: Degree[]
  temperament: TemperamentFacts | null
}

export interface SystemEntry {
  id: string
  name: string
  family: string
  description: string
  count: number
}

export interface EqualPreset {
  id: string
  label: string
  divisions: number
  numerator: number
  denominator: number
  note: string
}

export interface ScalaEntry {
  id: string
  file: string
  description: string
  count: number
}

export interface Library {
  systems: SystemEntry[]
  temperaments: TemperamentFacts[]
  equal: EqualPreset[]
  scala_count: number
}

export interface Key {
  step: number
  degree: number
  label: string
  cents: number
  frequency: number
  black: boolean
}

/** A note being held: its step and what the wasm said about it when it began. */
export interface Tone {
  step: number
  freq: number
  name: string
}

/** The scale under the keys, as the wasm last described it. */
export let scale: Scale
export let library: Library

/** Currently sounding notes, keyed by step, for the staff and the log. */
export const playingTones = new Map<number, Tone>()
/** Physical keyboard keys held down, so auto-repeat does not retrigger. */
export const heldKeys = new Set<string>()
/** Steps the reader has marked to play or share as a chord. */
export const markedKeys: number[] = []

/** Whole periods the computer keyboard and MIDI input are shifted by. */
export let octaveShift = 0
export function setOctaveShift(value: number): void {
  octaveShift = Math.max(-4, Math.min(4, Math.round(value)))
}
/** The step offset the octave control amounts to in the current scale. */
export function shift(): number {
  return octaveShift * scale.count
}

// ---------------------------------------------------------------------------
// Startup

void (async () => {
  try {
    wasm = await import("wasm-tuningplayground")
    wasm.main()
    library = JSON.parse(wasm.library_json()) as Library

    const shared = ui.readUrl()
    selectScale(shared.id, shared.root)
    ui.build()
    readMarkedFromHash()

    document.addEventListener("keydown", keydown)
    document.addEventListener("keyup", keyup)
    if (midiWanted()) void connectMidi()
    ui.setStatus("", "ready")
  } catch (error: unknown) {
    ui.setStatus(`Could not start the tuning playground: ${formatError(error)}`, "error")
    window.setTimeout(() => {
      throw error
    }, 0)
  }
})()

document.addEventListener("visibilitychange", visibilityChange)
window.addEventListener("blur", stopAllTones)
window.addEventListener("hashchange", readMarkedFromHash)

export function formatError(error: unknown): string {
  return error instanceof Error ? error.message : String(error)
}

/**
 * Puts the scale a share id names under the keys. An id nothing answers to
 * falls back to twelve-tone equal temperament rather than leaving the page
 * with no scale at all.
 */
export function selectScale(id: string, rootHz: number): void {
  if (scale) stopAllTones()
  try {
    scale = JSON.parse(wasm.select_scale(id, rootHz)) as Scale
  } catch (error: unknown) {
    ui.setStatus(formatError(error), "error")
    scale = JSON.parse(wasm.select_scale("EqualTemperament", DEFAULT_ROOT_HZ)) as Scale
  }
}

/** Puts a scale from the text of a `.scl` file under the keys. */
export function selectScl(name: string, contents: string, rootHz: number): boolean {
  if (scale) stopAllTones()
  try {
    scale = JSON.parse(wasm.select_scl(name, contents, rootHz)) as Scale
    return true
  } catch (error: unknown) {
    ui.setStatus(formatError(error), "error")
    return false
  }
}

// ---------------------------------------------------------------------------
// Audio engine
//
// One shared AudioContext feeds a master gain and a compressor, so a chord is
// tamed rather than clipped. Every note is a short attack-release envelope on
// its own gain, which is what keeps a square wave from clicking on and off and
// what lets a sample stop when the key is released.

const ATTACK = 0.008
const RELEASE = 0.09
const SAMPLE_BASE_HZ = 220
const SAMPLE_URL = "/misc/media/a1.wav"

type SoundMethod = "sample" | "native"

interface Voice {
  gain: GainNode
  source: OscillatorNode | AudioBufferSourceNode | null
  frequency: number
  release: () => void
}

const voices = new Map<number, Voice>()

let audioContext: AudioContext | null = null
let masterGain: GainNode | null = null
let sampleBuffer: AudioBuffer | null = null
let sampleRequest: Promise<AudioBuffer> | null = null

let engine: SoundMethod = "sample"
let waveform: OscillatorType = "sine"
let volume = 0.4

export function setEngine(value: string): void {
  engine = value === "native" ? "native" : "sample"
}
export function setWaveform(value: string): void {
  waveform = value as OscillatorType
}
export function setVolume(value: number): void {
  volume = Math.min(1, Math.max(0, value))
  if (masterGain && audioContext) {
    masterGain.gain.setTargetAtTime(volume, audioContext.currentTime, 0.01)
  }
}

export function audio(): AudioContext {
  if (!audioContext) {
    audioContext = new AudioContext()
    masterGain = audioContext.createGain()
    masterGain.gain.value = volume
    const compressor = audioContext.createDynamicsCompressor()
    masterGain.connect(compressor)
    compressor.connect(audioContext.destination)
  }
  if (audioContext.state === "suspended") {
    void audioContext.resume()
  }
  return audioContext
}

function ensureSample(context: AudioContext): Promise<AudioBuffer> {
  if (sampleBuffer) return Promise.resolve(sampleBuffer)
  sampleRequest ??= fetch(SAMPLE_URL)
    .then((response) => {
      if (!response.ok) throw new Error(`sample ${response.status}`)
      return response.arrayBuffer()
    })
    .then((bytes) => context.decodeAudioData(bytes))
    .then((buffer) => {
      sampleBuffer = buffer
      return buffer
    })
  return sampleRequest
}

/** The lowest step held, which is what an adaptive scale tunes from. */
function lowestHeld(): number {
  let lowest = -1
  for (const step of playingTones.keys()) {
    if (lowest < 0 || step < lowest) lowest = step
  }
  return lowest
}

/** The pitch a step sounds now, given what else is held. */
export function frequencyOf(step: number): number {
  return wasm.step_frequency(step, scale.adaptive ? lowestHeld() : -1)
}

function setSourceFrequency(voice: Voice, frequency: number, at: number): void {
  const source = voice.source
  if (source instanceof OscillatorNode) {
    source.frequency.setTargetAtTime(frequency, at, 0.01)
  } else if (source instanceof AudioBufferSourceNode) {
    source.playbackRate.setTargetAtTime(frequency / SAMPLE_BASE_HZ, at, 0.01)
  }
  voice.frequency = frequency
}

/** In an adaptive scale the lowest key held is the root, so the others move when it changes. */
function retune(): void {
  if (!scale.adaptive || !audioContext) return
  const now = audioContext.currentTime
  for (const [step, voice] of voices) {
    const frequency = frequencyOf(step)
    const tone = playingTones.get(step)
    if (tone) tone.freq = frequency
    if (Math.abs(frequency - voice.frequency) > 1e-6) setSourceFrequency(voice, frequency, now)
  }
}

function startVoice(step: number, frequency: number, peak: number): void {
  const context = audio()
  const master = masterGain
  if (!master) return

  voices.get(step)?.release()

  const gain = context.createGain()
  gain.gain.value = 0
  gain.connect(master)

  const voice: Voice = {
    gain,
    source: null,
    frequency,
    release: () => {
      const at = context.currentTime
      gain.gain.cancelScheduledValues(at)
      gain.gain.setValueAtTime(gain.gain.value, at)
      gain.gain.linearRampToValueAtTime(0, at + RELEASE)
      try {
        voice.source?.stop(at + RELEASE + 0.02)
      } catch {
        /* already stopped */
      }
    },
  }
  voices.set(step, voice)

  const begin = (source: OscillatorNode | AudioBufferSourceNode) => {
    voice.source = source
    source.connect(gain)
    source.start()
    const now = context.currentTime
    gain.gain.setValueAtTime(0, now)
    gain.gain.linearRampToValueAtTime(peak, now + ATTACK)
  }

  const oscillator = (type: OscillatorType) => {
    const node = context.createOscillator()
    node.type = type
    node.frequency.setValueAtTime(voice.frequency, context.currentTime)
    begin(node)
  }

  if (engine === "native") {
    oscillator(waveform)
    return
  }
  ensureSample(context)
    .then((buffer) => {
      if (!voices.has(step) || voices.get(step) !== voice) return
      const source = context.createBufferSource()
      source.buffer = buffer
      source.playbackRate.value = voice.frequency / SAMPLE_BASE_HZ
      begin(source)
    })
    .catch(() => {
      // No sample, no fallback needed: a synth voice keeps the key playable.
      if (voices.get(step) === voice) oscillator("triangle")
    })
}

function velocityToPeak(velocity?: number): number {
  if (velocity === undefined) return 0.9
  return 0.3 + 0.6 * Math.min(1, Math.max(0, velocity / 127))
}

/** Starts a note and redraws the staff. */
export function noteOn(step: number, velocity?: number): void {
  _noteOn(step, velocity)
  ui.playingTonesChanged()
}

/** Starts a note without redrawing, so a chord can be built then drawn once. */
export function _noteOn(step: number, velocity?: number): void {
  if (playingTones.has(step)) return
  const tone: Tone = { step, freq: 0, name: wasm.step_name(step) }
  playingTones.set(step, tone)
  tone.freq = frequencyOf(step)
  startVoice(step, tone.freq, velocityToPeak(velocity))
  retune()
  ui.keyActive(step, true)
}

export function noteOff(step: number): void {
  const voice = voices.get(step)
  if (voice) {
    voice.release()
    voices.delete(step)
  }
  if (!playingTones.has(step)) return
  playingTones.delete(step)
  retune()
  ui.keyActive(step, false)
  ui.playingTonesChanged()
}

export function stopAllTones(): void {
  for (const step of [...playingTones.keys()]) {
    voices.get(step)?.release()
    voices.delete(step)
    ui.keyActive(step, false)
    playingTones.delete(step)
  }
  ui.playingTonesChanged()
}
