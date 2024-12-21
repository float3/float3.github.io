import { codeToHtml } from 'shiki'

export interface TabConfig {
  years: number
  days: number
  problems: number
}

export async function createTabs(container: HTMLElement, config: TabConfig, wasm: typeof import("wasm")) {
  const { years, days, problems } = config

  let activeYear = 10 // 2024
  let activeDay = 1
  let activeProblem = 1

  const urlParams = new URLSearchParams(window.location.search)

  const initialYearParam = urlParams.get("year")
  const initialDayParam = urlParams.get("day")
  const initialProblemParam = urlParams.get("problem")

  if (initialYearParam) {
    const parsedYear = parseInt(initialYearParam, 10)
    if (!isNaN(parsedYear)) {
      const relativeY = parsedYear - 2014
      if (relativeY >= 1 && relativeY <= years) {
        activeYear = relativeY
      }
    }
  }

  if (initialDayParam) {
    const parsedDay = parseInt(initialDayParam, 10)
    if (!isNaN(parsedDay) && parsedDay >= 1 && parsedDay <= days) {
      activeDay = parsedDay
    }
  }

  if (initialProblemParam) {
    const parsedProblem = parseInt(initialProblemParam, 10)
    if (!isNaN(parsedProblem) && parsedProblem >= 1 && parsedProblem <= problems) {
      activeProblem = parsedProblem
    }
  }

  const yearsWrapper = document.createElement("div")
  yearsWrapper.className = "years tabs"

  const daysWrapper = document.createElement("div")
  daysWrapper.className = "days tabs"

  const problemsWrapper = document.createElement("div")
  problemsWrapper.className = "problems tabs"

  const contentWrapper = document.createElement("div")
  contentWrapper.className = "content"

  const fieldsMap = new Map<string, HTMLDivElement>()

  // Pre-create all fields sets
  for (let y = 2015; y < years + 2015; y++) {
    for (let d = 1; d <= days; d++) {
      for (let p = 1; p <= problems; p++) {
        const fields = document.createElement("div")
        fields.className = "fields hidden"

        const descriptionArea = document.createElement("textarea")
        descriptionArea.className = "big-field"
        descriptionArea.value = wasm.retrieve_problem(y, d, p)
        descriptionArea.disabled = true

        const codeArea = document.createElement("div")
        codeArea.className = "big-field"
        codeArea.innerHTML = await codeToHtml(wasm.retrieve_solution(y, d, p), { lang: "rust", theme: "vitesse-dark" })

        const inputArea = document.createElement("textarea")
        inputArea.id = "inputArea"
        inputArea.className = "big-field"
        inputArea.placeholder = "Input here..."

        inputArea.oninput = () => {
          console.log("inputArea.oninput")
          const t = p === 1 ? 2 : 1
          const key = `${y}-${d}-${t}`
          const fields = fieldsMap.get(key)
          if (fields) {
            const otherInputArea = fields.querySelector("textarea#inputArea") as HTMLTextAreaElement
            if (otherInputArea) otherInputArea.value = inputArea.value
          }
        }

        const outputArea = document.createElement("textarea")
        outputArea.className = "small-field"
        outputArea.disabled = true
        outputArea.value = ""

        const solveButton = document.createElement("button")
        solveButton.textContent = "Solve"
        solveButton.addEventListener("click", () => {
          outputArea.value = wasm.solve(inputArea.value, y, d, p)
        })

        fields.appendChild(descriptionArea)
        fields.appendChild(codeArea)
        fields.appendChild(inputArea)
        fields.appendChild(outputArea)
        fields.appendChild(solveButton)

        const key = `${y}-${d}-${p}`
        fieldsMap.set(key, fields)
        contentWrapper.appendChild(fields)
      }
    }
  }

  // Create year tabs
  for (let y = 1; y <= years; y++) {
    const btn = document.createElement("button")
    btn.textContent = (2014 + y).toString()
    if (y === activeYear) btn.classList.add("active")
    btn.addEventListener("click", () => {
      activeYear = y
      updateActiveTab(yearsWrapper, btn)
      showCurrentFields()
      updateURL()
    })
    yearsWrapper.appendChild(btn)
  }

  // Create day tabs
  for (let d = 1; d <= days; d++) {
    const btn = document.createElement("button")
    btn.textContent = `day ${d}`
    if (d === activeDay) btn.classList.add("active")
    btn.addEventListener("click", () => {
      activeDay = d
      updateActiveTab(daysWrapper, btn)
      showCurrentFields()
      updateURL()
    })
    daysWrapper.appendChild(btn)
  }

  // Create problem tabs
  for (let p = 1; p <= problems; p++) {
    const btn = document.createElement("button")
    btn.textContent = `problem ${p}`
    if (p === activeProblem) btn.classList.add("active")
    btn.addEventListener("click", () => {
      activeProblem = p
      updateActiveTab(problemsWrapper, btn)
      showCurrentFields()
      updateURL()
    })
    problemsWrapper.appendChild(btn)
  }

  container.appendChild(yearsWrapper)
  container.appendChild(daysWrapper)
  container.appendChild(problemsWrapper)
  container.appendChild(contentWrapper)

  function updateActiveTab(wrapper: HTMLElement, activeButton: HTMLButtonElement) {
    wrapper.querySelectorAll("button").forEach((btn) => btn.classList.remove("active"))
    activeButton.classList.add("active")
  }

  function showCurrentFields() {
    fieldsMap.forEach((fieldsDiv) => fieldsDiv.classList.add("hidden"))
    const actualYear = 2014 + activeYear
    const key = `${actualYear}-${activeDay}-${activeProblem}`
    const currentFields = fieldsMap.get(key)
    if (currentFields) currentFields.classList.remove("hidden")
  }

  function updateURL() {
    const actualYear = 2014 + activeYear
    const url = new URL(window.location.href)
    url.searchParams.set("year", actualYear.toString())
    url.searchParams.set("day", activeDay.toString())
    url.searchParams.set("problem", activeProblem.toString())
    history.replaceState(null, "", url.toString())
  }

  // Initialize
  showCurrentFields()
  updateURL() // so initial load also sets URL parameters
}
