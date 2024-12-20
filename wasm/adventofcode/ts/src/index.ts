import { createTabs, TabConfig } from "./ui.js"

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

export async function initWasm(): Promise<void> {
  await loadWasm()
}

document.addEventListener("DOMContentLoaded", async () => {
  await initWasm()

  const container = document.getElementById("adventofcode") as HTMLElement
  createTabs(container, { years: 10, days: 25, problems: 2 })
})
