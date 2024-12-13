export let wasm: typeof import("glsl2hlsl-wasm")

import("glsl2hlsl-wasm").then((module) => {
  wasm = module
  wasm.main()
})

let inp = document.getElementById("in") as HTMLTextAreaElement
let outp = document.getElementById("out") as HTMLTextAreaElement
let shader = document.getElementById("shader") as HTMLInputElement
let raymarch = document.getElementById("raymarch") as HTMLInputElement
let extract = document.getElementById("extract") as HTMLInputElement

let convertButton = document.getElementById("convert") as HTMLButtonElement
let downloadButton = document.getElementById("download") as HTMLButtonElement

convertButton.addEventListener("click", () => {
  if (inp && outp && extract && raymarch) {
    outp.value = wasm.transpile(inp.value, extract.checked, raymarch.checked)
  }
})

downloadButton.addEventListener("click", () => {
  if (shader && extract && raymarch) {
    let arr = shader.value.split("/").filter((x) => x.length > 0)

    let xhttp = new XMLHttpRequest()
    xhttp.onload = function () {
      if (this.responseText) {
        wasm.download(this.responseText, extract.checked, raymarch.checked)
      }
    }

    let shaderId = arr[arr.length - 1]
    if (shaderId) {
      xhttp.open("GET", `https://www.shadertoy.com/api/v1/shaders/${shaderId}?key=NtHtMm`)
      xhttp.send()
    }
  }
})
