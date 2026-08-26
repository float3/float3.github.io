import { DEFAULT_SHADER, DEFAULT_SHADER_SOURCE } from "./default_shader.js"

import * as wasm from "wasm-glsl"

wasm.main()

const inp = document.getElementById("in") as HTMLTextAreaElement
const outp = document.getElementById("out") as HTMLTextAreaElement
const shader = document.getElementById("shader") as HTMLInputElement
const raymarch = document.getElementById("raymarch") as HTMLInputElement
const extract = document.getElementById("extract") as HTMLInputElement

const previewToggle = document.getElementById("preview-bg") as HTMLInputElement | null
const previewStatus = document.getElementById("preview-status") as HTMLParagraphElement | null

const convertButton = document.getElementById("convert") as HTMLButtonElement
const downloadButton = document.getElementById("download") as HTMLButtonElement

convertButton.addEventListener("click", () => {
  if (inp && outp && extract && raymarch) {
    outp.value = wasm.transpile(inp.value, extract.checked, raymarch.checked)
  }
})

downloadButton.addEventListener("click", () => {
  if (shader && extract && raymarch) {
    const xhttp = new XMLHttpRequest()
    xhttp.onload = function () {
      if (this.responseText) {
        wasm.download(this.responseText, extract.checked, raymarch.checked)
      }
    }

    const shaderId = wasm.shader_id_from_url(shader.value)
    if (shaderId) {
      xhttp.open("GET", `https://www.shadertoy.com/api/v1/shaders/${shaderId}?key=NtHtMm`)
      xhttp.send()
    }
  }
})

/**
 * Renders whatever is in the Shadertoy box as the page background.
 *
 * The background system already takes GLSL and already keeps rendering the
 * last program that linked, so this is only the wiring: debounce the typing,
 * hand the source over, and say so when it did not compile. Off is remembered
 * per browser, because a shader behind the editor is a matter of taste.
 */
const PREVIEW_KEY = "float3:glsl2hlsl:preview"
const PREVIEW_DEBOUNCE_MS = 400

let previewTimer = 0

function previewEnabled(): boolean {
  try {
    return localStorage.getItem(PREVIEW_KEY) !== "off"
  } catch {
    // Private mode, or storage switched off. Default on, same as everyone else.
    return true
  }
}

function rememberPreview(enabled: boolean): void {
  try {
    localStorage.setItem(PREVIEW_KEY, enabled ? "on" : "off")
  } catch {
    // Not worth telling anyone about; the checkbox still works this session.
  }
}

function setStatus(text: string): void {
  if (!previewStatus) return
  previewStatus.textContent = text
  previewStatus.hidden = text.length === 0
}

/** Runs `use` once the background bundle has published its API. */
function withBackground(use: (api: NonNullable<Window["float3Background"]>) => void): void {
  if (window.float3Background) {
    use(window.float3Background)
    return
  }
  document.addEventListener(
    "float3:background-ready",
    () => {
      if (window.float3Background) use(window.float3Background)
    },
    { once: true },
  )
}

/** The driver's info log is many lines; the first one says what is wrong. */
function firstLine(text: string): string {
  return text.split("\n", 1)[0]
}

function applyPreview(): void {
  const source = inp?.value ?? ""
  if (!source.trim()) {
    setStatus("")
    return
  }
  withBackground((api) => {
    const error = api.preview(source)
    // A shader mid-edit fails far more often than it succeeds, so the failure
    // is the ordinary case and reads as a note, not an alarm.
    setStatus(error === null ? "" : `not rendering this one: ${firstLine(error)}`)
  })
}

function schedulePreview(): void {
  if (!previewEnabled()) return
  window.clearTimeout(previewTimer)
  previewTimer = window.setTimeout(applyPreview, PREVIEW_DEBOUNCE_MS)
}

function stopPreview(): void {
  window.clearTimeout(previewTimer)
  setStatus("")
  withBackground((api) => api.endPreview())
}

if (previewToggle) {
  previewToggle.checked = previewEnabled()
  previewToggle.addEventListener("change", () => {
    rememberPreview(previewToggle.checked)
    if (previewToggle.checked) applyPreview()
    else stopPreview()
  })
}

inp.addEventListener("input", schedulePreview)

const makeTextFile = (text: string): { textFile: string; cleanup: () => void } => {
  const data = new Blob([text], { type: "text/plain" })

  const textFile = window.URL.createObjectURL(data)

  const cleanup = () => {
    window.URL.revokeObjectURL(textFile)
  }

  return { textFile, cleanup }
}

declare global {
  interface Window {
    downloadFile: (name: string, contents: string) => void
    downloadImage: (name: string, contents: string) => void
    reset: () => void
  }
}

window.reset = reset
window.downloadFile = downloadFile
window.downloadImage = downloadImage

const links: HTMLDivElement = document.querySelector("#links") as HTMLDivElement

export function downloadFile(name: string, contents: string): void {
  const a = document.createElement("a")
  a.style.display = "none"
  const { textFile, cleanup } = makeTextFile(contents)
  a.href = textFile
  a.download = name
  links.appendChild(a)

  document.body.appendChild(a)
  a.click()

  document.body.removeChild(a)
  cleanup()
}

export function downloadImage(name: string, contents: string): void {
  const c = document.createElement("br")
  links.appendChild(c)

  const a = document.createElement("a")
  a.innerHTML = name
  a.href = contents
  a.download = name
  links.appendChild(a)

  // document.body.appendChild(a)
  // a.click()
  // document.body.removeChild(a)
}

export function reset(): void {
  if (links) {
    links.innerHTML = "<p></p><h2>Textures (Ctrl+Click and Save-As):</h2><br>"
  }
}

function fillDefaults(): void {
  shader.value = DEFAULT_SHADER
  inp.value = DEFAULT_SHADER_SOURCE
  schedulePreview()
}

// The SPA router re-runs this script on navigation, by which point
// DOMContentLoaded has long since fired and would never come again.
if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", fillDefaults, { once: true })
} else {
  fillDefaults()
}

shader.addEventListener("input", () => {
  const xhttp = new XMLHttpRequest()
  xhttp.onload = function () {
    if (this.responseText) {
      inp.value = JSON.parse(this.responseText).Shader.renderpass[0].code
      // Setting `value` fires no input event, so ask for the repaint by hand.
      schedulePreview()
    }
  }
  const shaderId = wasm.shader_id_from_url(shader.value)
  xhttp.open("GET", `https://www.shadertoy.com/api/v1/shaders/${shaderId}?key=NtHtMm`)
  xhttp.send()
})
