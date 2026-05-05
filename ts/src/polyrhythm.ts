import("./polyrhythm/index.js").catch((error) =>
  console.error("Error importing polyrhythm:", error),
)

type AudioContextConstructor = typeof AudioContext
type BrowserWindow = Window & {
  webkitAudioContext?: AudioContextConstructor
}

type PendingHit = {
  frequency: number
  duration: number
}

declare global {
  interface Window {
    play_beep: (frequency: number, duration: number) => void
    stop_audio: () => void
  }
}

let audioContext: AudioContext | null = null
let masterGain: GainNode | null = null
let compressor: DynamicsCompressorNode | null = null
let pendingHits: PendingHit[] = []
let flushScheduled = false

window.play_beep = playBeep
window.stop_audio = stopAudio

function playBeep(frequency: number, duration: number) {
  if (!Number.isFinite(frequency) || frequency <= 0) {
    return
  }

  pendingHits.push({
    frequency,
    duration: Number.isFinite(duration) && duration > 0 ? duration : 0.09,
  })

  if (!flushScheduled) {
    flushScheduled = true
    queueMicrotask(flushPendingHits)
  }
}

function stopAudio() {
  pendingHits = []
  flushScheduled = false
  if (audioContext) {
    void audioContext.close()
  }
  audioContext = null
  masterGain = null
  compressor = null
}

function flushPendingHits() {
  flushScheduled = false
  const hits = pendingHits
  pendingHits = []
  if (hits.length === 0) {
    return
  }

  void playHitGroup(hits)
}

async function playHitGroup(hits: PendingHit[]) {
  const context = getAudioContext()
  if (!context) {
    return
  }

  if (context.state === "suspended") {
    await context.resume()
  }

  const startTime = context.currentTime + 0.004
  const duration = Math.max(...hits.map((hit) => hit.duration))
  const voicePeak = Math.min(0.14, 0.22 / Math.sqrt(hits.length))
  const groupDuration = Math.max(0.055, Math.min(0.16, duration))

  for (const hit of hits) {
    const oscillator = context.createOscillator()
    const gain = context.createGain()
    oscillator.type = hits.length > 1 ? "sine" : "triangle"
    oscillator.frequency.setValueAtTime(hit.frequency, startTime)

    gain.gain.setValueAtTime(0.0001, startTime)
    gain.gain.linearRampToValueAtTime(voicePeak, startTime + 0.006)
    gain.gain.exponentialRampToValueAtTime(0.0001, startTime + groupDuration)

    oscillator.connect(gain)
    gain.connect(masterGain!)

    oscillator.start(startTime)
    oscillator.stop(startTime + groupDuration + 0.02)
    oscillator.addEventListener("ended", () => {
      oscillator.disconnect()
      gain.disconnect()
    })
  }
}

function getAudioContext() {
  if (audioContext && masterGain && compressor) {
    return audioContext
  }

  const AudioCtor = window.AudioContext ?? (window as BrowserWindow).webkitAudioContext
  if (!AudioCtor) {
    return null
  }

  audioContext = new AudioCtor()
  masterGain = audioContext.createGain()
  compressor = audioContext.createDynamicsCompressor()

  masterGain.gain.value = 0.82
  compressor.threshold.value = -18
  compressor.knee.value = 18
  compressor.ratio.value = 6
  compressor.attack.value = 0.004
  compressor.release.value = 0.12

  masterGain.connect(compressor)
  compressor.connect(audioContext.destination)

  return audioContext
}
