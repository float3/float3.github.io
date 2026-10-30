/**
 * The sound of the recursive tuning post: one AudioContext, a master gain into
 * a compressor, and voices that can be retuned while they sound, since picking
 * another root moves every note held over the old one.
 */

export type Timbre = "harmonic" | "sine"

const ATTACK = 0.01
const RELEASE = 0.12
const PARTIALS = 8

interface Sound {
  oscillator: OscillatorNode
  gain: GainNode
}

let context: AudioContext | null = null
let master: GainNode | null = null
let harmonics: PeriodicWave | null = null
let timbre: Timbre = "harmonic"
let volume = 0.5

const held = new Map<string, Sound>()
const scheduled = new Set<Sound>()

export function setTimbre(value: string): void {
  timbre = value === "sine" ? "sine" : "harmonic"
}

export function setVolume(value: number): void {
  volume = Math.min(1, Math.max(0, value))
  if (master && context) master.gain.setTargetAtTime(volume, context.currentTime, 0.02)
}

export function audio(): AudioContext {
  if (!context) {
    context = new AudioContext()
    master = context.createGain()
    master.gain.value = volume
    const compressor = context.createDynamicsCompressor()
    master.connect(compressor)
    compressor.connect(context.destination)

    const real = new Float32Array(PARTIALS + 1)
    const imag = new Float32Array(PARTIALS + 1)
    for (let partial = 1; partial <= PARTIALS; partial++) imag[partial] = 1 / partial
    harmonics = context.createPeriodicWave(real, imag)
  }
  if (context.state === "suspended") void context.resume()
  return context
}

function begin(frequency: number, at: number, peak: number): Sound {
  const ctx = audio()
  const gain = ctx.createGain()
  gain.gain.setValueAtTime(0, at)
  gain.gain.linearRampToValueAtTime(peak, at + ATTACK)
  if (master) gain.connect(master)

  const oscillator = ctx.createOscillator()
  if (timbre === "harmonic" && harmonics) oscillator.setPeriodicWave(harmonics)
  else oscillator.type = "sine"
  oscillator.frequency.setValueAtTime(frequency, at)
  oscillator.connect(gain)
  oscillator.start(at)
  return { oscillator, gain }
}

function release(sound: Sound, at: number): void {
  const level = sound.gain.gain
  level.cancelScheduledValues(at)
  level.setValueAtTime(level.value, at)
  level.linearRampToValueAtTime(0, at + RELEASE)
  sound.oscillator.stop(at + RELEASE + 0.02)
}

/** Starts a note that sounds until {@link stop} is called with the same id. */
export function start(id: string, frequency: number, peak = 0.3): void {
  stop(id)
  held.set(id, begin(frequency, audio().currentTime, peak))
}

export function retune(id: string, frequency: number): void {
  const sound = held.get(id)
  if (sound && context) {
    sound.oscillator.frequency.setTargetAtTime(frequency, context.currentTime, 0.015)
  }
}

export function stop(id: string): void {
  const sound = held.get(id)
  if (!sound || !context) return
  held.delete(id)
  release(sound, context.currentTime)
}

/** Plays a note from `at` for `duration` seconds. */
export function schedule(frequency: number, at: number, duration: number, peak = 0.22): void {
  const sound = begin(frequency, at, peak)
  const end = at + duration
  const level = sound.gain.gain
  level.setValueAtTime(peak, Math.max(at + ATTACK, end - RELEASE))
  level.linearRampToValueAtTime(0, end)
  sound.oscillator.stop(end + 0.02)
  scheduled.add(sound)
  sound.oscillator.addEventListener("ended", () => scheduled.delete(sound))
}

export function stopAll(): void {
  if (!context) return
  const now = context.currentTime
  for (const id of [...held.keys()]) stop(id)
  for (const sound of scheduled) release(sound, now)
  scheduled.clear()
}
