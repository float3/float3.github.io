let textFile: string | null = null

const makeTextFile = (text: string): string => {
  const data = new Blob([text], { type: "text/plain" })

  if (textFile !== null) {
    window.URL.revokeObjectURL(textFile)
  }

  textFile = window.URL.createObjectURL(data)
  return textFile
}

// Set up downloading
export function downloadFile(name: string, contents: string): void {
  const a = document.createElement("a")
  a.style.display = "none"
  a.href = makeTextFile(contents)
  a.download = name
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a) // Cleanup
}

export function downloadImage(name: string, contents: string): void {
  const c = document.createElement("br")
  document.querySelector("#links")?.appendChild(c)

  const a = document.createElement("a")
  a.innerHTML = name
  a.href = contents
  a.download = name
  document.querySelector("#links")?.appendChild(a)
}

export function reset(): void {
  const links = document.querySelector("#links")
  if (links) {
    links.innerHTML = "<p></p><h2>Textures (Ctrl+Click and Save-As):</h2><br>"
  }
}
