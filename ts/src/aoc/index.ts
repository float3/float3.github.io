import { createTabs } from "./ui.js"

import { main } from "wasm-aoc"

main()

function initAdventOfCode() {
  const container = document.getElementById("adventofcode")
  if (!container || container.dataset.aocInitialized === "true") return

  container.dataset.aocInitialized = "true"
  createTabs(container, { years: 11, days: 25, problems: 2 })
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", initAdventOfCode, { once: true })
} else {
  initAdventOfCode()
}

document.addEventListener("nav", initAdventOfCode)
