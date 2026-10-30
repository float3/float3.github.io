import { createTabs } from "./ui.js"

import { main } from "wasm-aoc"

main()

function initAdventOfCode() {
  const container = document.getElementById("adventofcode")
  if (!container || container.dataset.aocInitialized === "true") return

  container.dataset.aocInitialized = "true"
  createTabs(container)
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", initAdventOfCode, { once: true })
} else {
  initAdventOfCode()
}

document.addEventListener("nav", initAdventOfCode)
