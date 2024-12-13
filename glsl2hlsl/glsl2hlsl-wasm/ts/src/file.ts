declare global {
  interface Window {
    downloadFile: (name: string, contents: string) => void
    downloadImage: (name: string, contents: string) => void
    reset: () => void
  }
}

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

let links = document.querySelector("#links")

export function downloadFile(name: string, contents: string): void {
  let c = document.createElement("br")
  links?.appendChild(c)

  let a = document.createElement("a")
  // a.style.display = "none"
  a.href = makeTextFile(contents)
  a.download = name
  links?.appendChild(a)

  document.body.appendChild(a)
  // a.click()
  // document.body.removeChild(a)
}

export function downloadImage(name: string, contents: string): void {
  let c = document.createElement("br")
  links?.appendChild(c)

  let a = document.createElement("a")
  a.innerHTML = name
  a.href = contents
  a.download = name
  links?.appendChild(a)

  document.body.appendChild(a)
  // a.click()
  // document.body.removeChild(a)
}

export function reset(): void {
  if (links) {
    links.innerHTML = "<p></p><h2>Textures (Ctrl+Click and Save-As):</h2><br>"
  }
}