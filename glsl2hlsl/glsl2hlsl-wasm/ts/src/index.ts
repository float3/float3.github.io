export let wasm: typeof import("glsl2hlsl-wasm")

declare global {
  interface Window {
    downloadFile: (name: string, contents: string) => void
    downloadImage: (name: string, contents: string) => void
    reset: () => void
  }
}

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

let textFile: string | null = null

let makeTextFile = (text: string): string => {
  let data = new Blob([text], { type: "text/plain" })

  if (textFile !== null) {
    window.URL.revokeObjectURL(textFile)
  }

  textFile = window.URL.createObjectURL(data)
  return textFile
}

window.reset = reset
window.downloadFile = downloadFile
window.downloadImage = downloadImage

export function downloadFile(name: string, contents: string): void {
  let a = document.createElement("a")
  a.style.display = "none"
  a.href = makeTextFile(contents)
  a.download = name
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
}

export function downloadImage(name: string, contents: string): void {
  let c = document.createElement("br")
  document.querySelector("#links")?.appendChild(c)

  let a = document.createElement("a")
  a.innerHTML = name
  a.href = contents
  a.download = name
  document.querySelector("#links")?.appendChild(a)

  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
}

export function reset(): void {
  let links = document.querySelector("#links")
  if (links) {
    links.innerHTML = "<p></p><h2>Textures (Ctrl+Click and Save-As):</h2><br>"
  }
}
