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
  await createTabs(container, { years: 10, days: 25, problems: 2 }, wasm)
})
