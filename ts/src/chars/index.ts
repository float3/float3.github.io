import { Char } from "wasm"

export let wasm: typeof import("wasm")

import("wasm").then((module) => {
  wasm = module
  wasm.main()

  const generator: Char = Char.new()
  const output = document.getElementById("output")
  if (!output) return
  ;(function appendChar() {
    const char = generator.next_char()
    output.textContent += char
    setTimeout(appendChar, 10)
  })()
})

async function run() {}

run()
