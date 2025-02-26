export let wasm: typeof import("wasm")

import("wasm").then((module) => {
  wasm = module
  wasm.main()

  const output = document.getElementById("output")
  if (!output) return
  ;(function appendChar() {
    output.textContent += wasm.random_weighted_char(true)
    setTimeout(appendChar, 1)
  })()
})
