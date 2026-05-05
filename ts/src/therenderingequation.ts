import { random_range } from "wasm"

document.addEventListener("DOMContentLoaded", () => {
  const elements: NodeListOf<SVGPathElement | SVGRectElement> = document.querySelectorAll(
    "#interactiveSvg path, #interactiveSvg rect",
  )
  const groupMap: Record<string, (SVGPathElement | SVGRectElement)[]> = {}

  elements.forEach((element) => {
    const fillColor = window.getComputedStyle(element).fill

    if (!groupMap[fillColor]) {
      groupMap[fillColor] = []
    }

    groupMap[fillColor].push(element)

    if (fillColor === "rgb(123, 233, 255)" || fillColor === "#7BE9FF") {
      return
    }

    const frequency = random_range(2, 5)
    const amplitude = random_range(8, 13)

    element.style.animation = `moveUpDown ${frequency}s ease-in-out infinite alternate`
    element.style.setProperty("--amplitude", `${amplitude}px`)
  })

  elements.forEach((element) => {
    const fillColor = window.getComputedStyle(element).fill

    element.addEventListener("mouseenter", () => {
      groupMap[fillColor]?.forEach((el) => el.classList.add("hovered"))
    })

    element.addEventListener("mouseleave", () => {
      groupMap[fillColor]?.forEach((el) => el.classList.remove("hovered"))
    })
  })
})
