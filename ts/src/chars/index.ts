import * as wasm from "wasm"

wasm.main()

const output = document.getElementById("output")
if (output) {
  ;(function appendChar() {
    output.textContent += wasm.random_weighted_char(true)
    setTimeout(appendChar, 1)
  })()
}
