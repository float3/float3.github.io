import { createTabs } from "./ui.js"

let wasmModulePromise: Promise<typeof import("wasm")>

function loadWasm(): Promise<typeof import("wasm")> {
  if (!wasmModulePromise) {
    wasmModulePromise = import("wasm").then(async (module) => {
      await module.main()
      return module
    })
  }
  return wasmModulePromise
}

document.addEventListener("DOMContentLoaded", async () => {
  const wasm = await loadWasm()

  const container = document.getElementById("adventofcode") as HTMLElement
  await createTabs(container, { year: 10, day: 25, problem: 2 }, wasm)
})
