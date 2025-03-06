import { random_n_pokemon, main } from "wasm"

main()

const pokemon = random_n_pokemon(15, 0)

const container = document.createElement("div")
document.body.prepend(container)
const shadowRoot = container.attachShadow({ mode: "open" })

const pre = document.createElement("pre")
pre.style.userSelect = "none"
pre.style.fontFamily = '"Courier New", Courier, monospace'
pre.style.letterSpacing = "0px"
pre.style.overflowX = "hidden"
shadowRoot.appendChild(pre)

const dappledLightDiv = document.getElementById("dappled-light")
if (dappledLightDiv) {
  dappledLightDiv.style.top = "0px"
  dappledLightDiv.style.bottom = "0px"
  dappledLightDiv.style.left = "0px"
  dappledLightDiv.style.right = "0px"
} else {
  console.error('Element with id "dappled-light" not found.')
}

// Step 3: add Pokémon text, styling remains isolated
pokemon.forEach((element) => {
  setTimeout(() => {
    pre.innerHTML += element + "\n"
  }, 6666)
})
