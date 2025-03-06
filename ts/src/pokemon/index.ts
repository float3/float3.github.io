export let wasm: typeof import("wasm")

import("wasm").then((module) => {
  wasm = module
  wasm.main()

  const pokemon = wasm.random_n_pokemon(15, 0)

  // Create a pre element to preserve all whitespace exactly.
  const pre = document.createElement("pre")
  const header = document.createElement("header")
  pre.style.fontFamily = '"Courier New", Courier, monospace'
  pre.style.letterSpacing = "0px"
  // pre.style.whiteSpace = "pre-wrap"
  pre.style.overflowX = "hidden"
  pre.innerHTML = pokemon

  // header.appendChild(pre)
  document.body.prepend(pre)
  const dappledLightDiv = document.getElementById("dappled-light")
  if (dappledLightDiv) {
    dappledLightDiv.style.top = "0px"
  } else {
    console.error('Element with id "dappled-light" not found.')
  }
})
