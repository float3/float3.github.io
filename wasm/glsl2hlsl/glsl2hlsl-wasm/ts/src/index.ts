import { DEFAULT_SHADER, DEFAULT_SHADER_SOURCE } from "./default_shader.js"

export let wasm: typeof import("glsl2hlsl-wasm")

import("glsl2hlsl-wasm").then((module) => {
  wasm = module
  wasm.main()
})

const inp = document.getElementById("in") as HTMLTextAreaElement
const outp = document.getElementById("out") as HTMLTextAreaElement
const shader = document.getElementById("shader") as HTMLInputElement
const raymarch = document.getElementById("raymarch") as HTMLInputElement
const extract = document.getElementById("extract") as HTMLInputElement

const convertButton = document.getElementById("convert") as HTMLButtonElement
const downloadButton = document.getElementById("download") as HTMLButtonElement

convertButton.addEventListener("click", () => {
  if (inp && outp && extract && raymarch) {
    outp.value = wasm.transpile(inp.value, extract.checked, raymarch.checked)
  }
})

downloadButton.addEventListener("click", () => {
  if (shader && extract && raymarch) {
    const arr = shader.value.split("/").filter((x) => x.length > 0)

    const xhttp = new XMLHttpRequest()
    xhttp.onload = function () {
      if (this.responseText) {
        wasm.download(this.responseText, extract.checked, raymarch.checked)
      }
    }

    const shaderId = arr[arr.length - 1]
    if (shaderId) {
      xhttp.open("GET", `https://www.shadertoy.com/api/v1/shaders/${shaderId}?key=NtHtMm`)
      xhttp.send()
    }
  }
})

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

document.addEventListener("DOMContentLoaded", () => {
  shader.value = DEFAULT_SHADER
  inp.value = DEFAULT_SHADER_SOURCE
})

shader.addEventListener("input", () => {
  const xhttp = new XMLHttpRequest()
  xhttp.onload = function () {
    if (this.responseText) {
      inp.value = JSON.parse(this.responseText).Shader.renderpass[0].code
    }
  }
  const shaderId = shader.value.split("/").pop()
  xhttp.open("GET", `https://www.shadertoy.com/api/v1/shaders/${shaderId}?key=NtHtMm`)
  xhttp.send()
})
