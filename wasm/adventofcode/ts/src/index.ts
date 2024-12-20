import { createTabs } from "./ui.js"

let wasmModulePromise: Promise<typeof import("wasm")>

function loadWasm(): Promise<typeof import("wasm")> {
  if (!wasmModulePromise) {
    wasmModulePromise = import("wasm").then(async (module) => {
      // await module.main()
      createTabs()
      return module
    })
  }
  return wasmModulePromise
}

export async function initWasm(): Promise<void> {
  await loadWasm()
}
