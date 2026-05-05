import { main, start_with_settings, stop } from "wasm"

type PolyrhythmSettings = {
  base: number
  tempo: number
  subdivisions: string
  pitch: number
}

main()

const baseInput = requiredInput("base")
const tempoInput = requiredInput("tempo")
const subdivisionsInput = requiredInput("subdivisions")
const pitchInput = requiredInput("pitch")
const startButton = requiredButton("start-button")
const stopButton = requiredButton("stop-button")
const statusElement = document.getElementById("polyrhythm-status")
const presetButtons = document.querySelectorAll<HTMLButtonElement>("[data-polyrhythm-preset]")

applySettings(readSettingsFromHash() ?? readSettingsFromControls())
setRunning(false)

startButton.addEventListener("click", () => {
  const settings = readSettingsFromControls()
  applySettings(settings)
  updateHash(settings)

  try {
    start_with_settings(settings.base, settings.tempo, settings.subdivisions, settings.pitch)
    setRunning(true)
    setStatus(`${settings.subdivisions.replaceAll(":", " against ")} at ${settings.tempo} BPM`)
  } catch (error) {
    setRunning(false)
    setStatus(error instanceof Error ? error.message : "Could not start polyrhythm")
  }
})

stopButton.addEventListener("click", () => {
  try {
    stop()
    window.stop_audio?.()
    setStatus("Stopped")
  } catch (error) {
    console.error("Could not stop polyrhythm:", error)
    setStatus("Stopped with cleanup warning")
  } finally {
    setRunning(false)
  }
})

presetButtons.forEach((button) => {
  button.addEventListener("click", () => {
    const preset = button.dataset.polyrhythmPreset
    if (!preset) {
      return
    }

    subdivisionsInput.value = preset
    const settings = readSettingsFromControls()
    applySettings(settings)
    updateHash(settings)
    setStatus(`${settings.subdivisions.replaceAll(":", " against ")} ready`)
  })
})

for (const input of [baseInput, tempoInput, subdivisionsInput, pitchInput]) {
  input.addEventListener("change", () => {
    const settings = readSettingsFromControls()
    applySettings(settings)
    updateHash(settings)
  })
}

function requiredInput(id: string) {
  const input = document.getElementById(id)
  if (!(input instanceof HTMLInputElement)) {
    throw new Error(`Missing input #${id}`)
  }
  return input
}

function requiredButton(id: string) {
  const button = document.getElementById(id)
  if (!(button instanceof HTMLButtonElement)) {
    throw new Error(`Missing button #${id}`)
  }
  return button
}

function readSettingsFromControls(): PolyrhythmSettings {
  return {
    base: readInteger(baseInput.value, 4, 1, 16),
    tempo: readInteger(tempoInput.value, 120, 20, 280),
    subdivisions: normalizeSubdivisions(subdivisionsInput.value),
    pitch: readInteger(pitchInput.value, 440, 80, 1400),
  }
}

function readSettingsFromHash(): PolyrhythmSettings | null {
  const hash = window.location.hash.slice(1)
  if (!hash) {
    return null
  }

  const params = new URLSearchParams(hash)
  return {
    base: readInteger(params.get("base"), 4, 1, 16),
    tempo: readInteger(params.get("tempo"), 120, 20, 280),
    subdivisions: normalizeSubdivisions(params.get("subdivisions") ?? "3:4"),
    pitch: readInteger(params.get("pitch"), 440, 80, 1400),
  }
}

function applySettings(settings: PolyrhythmSettings) {
  baseInput.value = String(settings.base)
  tempoInput.value = String(settings.tempo)
  subdivisionsInput.value = settings.subdivisions
  pitchInput.value = String(settings.pitch)
}

function updateHash(settings: PolyrhythmSettings) {
  const params = new URLSearchParams({
    base: String(settings.base),
    tempo: String(settings.tempo),
    subdivisions: settings.subdivisions,
    pitch: String(settings.pitch),
  })
  history.replaceState(null, "", `#${params}`)
}

function readInteger(value: string | null, fallback: number, min: number, max: number) {
  const parsed = Number.parseInt(value ?? "", 10)
  if (!Number.isFinite(parsed)) {
    return fallback
  }
  return Math.min(max, Math.max(min, parsed))
}

function normalizeSubdivisions(value: string) {
  const parts = value
    .split(/[,: ]+/)
    .map((part) => readInteger(part, 0, 0, 32))
    .filter((part) => part > 0)

  return parts.length > 0 ? parts.join(":") : "3:4"
}

function setRunning(isRunning: boolean) {
  startButton.disabled = isRunning
  stopButton.disabled = !isRunning
}

function setStatus(message: string) {
  if (statusElement) {
    statusElement.textContent = message
  }
}
