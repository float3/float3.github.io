export let wasm: typeof import("glsl2hlsl-wasm")

import("glsl2hlsl-wasm").then((module) => {
  wasm = module
  wasm.default()
})

const inp = document.getElementById("in") as HTMLTextAreaElement
const outp = document.getElementById("out") as HTMLTextAreaElement
const shader = document.getElementById("shader") as HTMLInputElement
const raymarch = document.getElementById("raymarch") as HTMLInputElement
const extract = document.getElementById("extract") as HTMLInputElement

const convertButton = document.getElementById("convert") as HTMLButtonElement
const downloadButton = document.getElementById("download") as HTMLButtonElement

convertButton.addEventListener("click", (e: MouseEvent) => {
  if (inp && outp && extract && raymarch) {
    outp.value = wasm.transpile(inp.value, extract.checked, raymarch.checked)
  }
})

downloadButton.addEventListener("click", (e: MouseEvent) => {
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
