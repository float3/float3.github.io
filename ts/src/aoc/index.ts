import { createTabs } from "./ui.js"

import { main } from "wasm"

main()

document.addEventListener("DOMContentLoaded", () => {
  const container = document.getElementById("adventofcode") as HTMLElement
  createTabs(container, { years: 10, days: 25, problems: 2 })
})
