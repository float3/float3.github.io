import("./polyrhythm/index.js").catch((e) => console.error("Error importing `.js`:", e))
declare global {
  interface Window {
    play_beep: (frequency: number, duration: number) => void
  }
}

window.play_beep = play_beep

function play_beep(frequency: number, duration: number) {
  const context = new (window.AudioContext || window.AudioContext)()
  const oscillator = context.createOscillator()
  const gainNode = context.createGain()

  oscillator.frequency.value = frequency
  oscillator.connect(gainNode)
  gainNode.connect(context.destination)

  oscillator.start()
  gainNode.gain.exponentialRampToValueAtTime(0.00001, context.currentTime + duration)
  oscillator.stop(context.currentTime + duration)
}

function updateHash() {
  const baseElement = document.getElementById("base")
  const tempoElement = document.getElementById("tempo")
  const subdivisionsElement = document.getElementById("subdivisions")
  const pitchElement = document.getElementById("pitch")

  const base = baseElement ? (baseElement as HTMLInputElement).value : ""
  const tempo = tempoElement ? (tempoElement as HTMLInputElement).value : ""
  const subdivisions = subdivisionsElement ? (subdivisionsElement as HTMLInputElement).value : ""
  const pitch = pitchElement ? (pitchElement as HTMLInputElement).value : ""
  const hashStr = `base=${encodeURIComponent(base)}&tempo=${encodeURIComponent(tempo)}&subdivisions=${encodeURIComponent(subdivisions)}&pitch=${encodeURIComponent(pitch)}`
  window.location.hash = hashStr
}

// Load settings from URL hash if available
function loadFromHash() {
  const hash = window.location.hash.substring(1) // remove the '#' character
  if (!hash) return
  hash.split("&").forEach((param) => {
    const [key, value] = param.split("=")
    if (key && value) {
      const elem = document.getElementById(key)
      if (elem) {
        ;(elem as HTMLInputElement).value = decodeURIComponent(value)
      }
    }
  })
}

// Placeholder functions for starting/stopping polyrhythm
function startPolyrhythm() {
  console.log("Polyrhythm started with settings:", {
    base: (document.getElementById("base") as HTMLInputElement)?.value || "",
    tempo: (document.getElementById("tempo") as HTMLInputElement)?.value,
    subdivisions: (document.getElementById("subdivisions") as HTMLInputElement)?.value,
    pitch: (document.getElementById("pitch") as HTMLInputElement)?.value,
  })
  // Insert your polyrhythm start logic here
}

function stopPolyrhythm() {
  console.log("Polyrhythm stopped.")
  // Insert your polyrhythm stop logic here
}

// Attach event listeners
document.getElementById("start-button")?.addEventListener("click", () => {
  updateHash()
  startPolyrhythm()
})
document.getElementById("stop-button")?.addEventListener("click", stopPolyrhythm)

// On page load, apply settings from the URL hash
window.addEventListener("load", loadFromHash)
