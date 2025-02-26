export let wasm: typeof import("wasm")

import("wasm").then((module) => {
  wasm = module
  wasm.main()
  const startButton = document.getElementById("start-button") as HTMLButtonElement
  startButton.addEventListener("click", () => {
    const baseInput = document.getElementById("base") as HTMLInputElement
    const tempoInput = document.getElementById("tempo") as HTMLInputElement
    const subdivisionsInput = document.getElementById("subdivisions") as HTMLInputElement
    const pitchInput = document.getElementById("pitch") as HTMLInputElement

    const base = parseInt(baseInput.value) || 4
    const tempo = parseInt(tempoInput.value) || 120
    const subdivisions = subdivisionsInput.value || "3:4"
    const pitch = parseInt(pitchInput.value) || 440

    wasm.start_with_settings(base, tempo, subdivisions, pitch)
  })

  const stopButton = document.getElementById("stop-button") as HTMLButtonElement
  stopButton.addEventListener("click", () => {
    wasm.stop()
  })
})
