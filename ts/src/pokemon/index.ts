let wasm: typeof import("wasm")

import("wasm").then((module) => {
  wasm = module
  wasm.main()

  const pokemon = wasm.random_n_pokemon(15, 0)

  const pre = document.createElement("pre")
  pre.style.fontFamily = '"Courier New", Courier, monospace'
  pre.style.letterSpacing = "0px"
  // pre.style.whiteSpace = "pre-wrap"
  pre.style.overflowX = "hidden"

  document.body.prepend(pre)
  const dappledLightDiv = document.getElementById("dappled-light")
  if (dappledLightDiv) {
    dappledLightDiv.style.top = "0px"
  } else {
    console.error('Element with id "dappled-light" not found.')
  }

  pokemon.forEach(element => {
    setTimeout(() => {
      pre.innerHTML += element
      pre.innerHTML += "\n"
    }, 3333);
  });
})