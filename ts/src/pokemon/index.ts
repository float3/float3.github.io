import { random_n_pokemon, main } from "wasm-pokemon"

main()

const pokemon = random_n_pokemon(15, 0)

const container = document.createElement("div")
container.setAttribute("aria-hidden", "true")
container.style.position = "fixed"
container.style.top = "0"
container.style.left = "0"
container.style.right = "0"
container.style.pointerEvents = "none"
container.style.overflow = "hidden"
container.style.contain = "layout paint"
document.body.prepend(container)
const shadowRoot = container.attachShadow({ mode: "open" })

const pre = document.createElement("pre")
pre.style.margin = "0"
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
    const scrollX = window.scrollX
    const scrollY = window.scrollY
    pre.insertAdjacentHTML("beforeend", `${element}\n`)
    window.scrollTo(scrollX, scrollY)
  }, 6666)
})
