function randomRange(min: number, max: number): number {
  if (!Number.isFinite(min) || !Number.isFinite(max) || max <= min) {
    return min
  }

  return min + Math.random() * (max - min)
}

function initializeRenderingEquation() {
  const svg = document.getElementById("interactiveSvg")
  if (!(svg instanceof SVGSVGElement) || svg.dataset.interactiveReady === "true") {
    return
  }

  svg.dataset.interactiveReady = "true"

  const elements: NodeListOf<SVGPathElement | SVGRectElement> = svg.querySelectorAll("path, rect")
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

    const frequency = randomRange(2, 5)
    const amplitude = randomRange(8, 13)

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
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", initializeRenderingEquation, { once: true })
} else {
  initializeRenderingEquation()
}
