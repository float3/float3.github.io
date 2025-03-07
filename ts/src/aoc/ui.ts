const START_YEAR = 2015
const STAR = "⭐"

import { retrieve_html, retrieve_problem, solve } from "wasm"

export interface TabConfig {
  years: number
  days: number
  problems: number
}

export function createTabs(container: HTMLElement, config: TabConfig) {
  const { years, days, problems } = config

  let activeYear = START_YEAR + years - 1
  let activeDay = 1,
    activeProblem = 1

  const urlParams = new URLSearchParams(window.location.search)

  const initialYearParam = urlParams.get("year")
  const initialDayParam = urlParams.get("day")
  const initialProblemParam = urlParams.get("problem")

  if (initialYearParam) {
    const parsedYear = parseInt(initialYearParam, 10)
    if (!isNaN(parsedYear) && parsedYear >= START_YEAR && parsedYear <= years) {
      activeYear = parsedYear
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

  const complete: boolean[][][] = Array.from({ length: years }, () =>
    Array.from({ length: days }, () => Array.from({ length: problems }, () => false)),
  )

  let isDark = document.documentElement.getAttribute("saved-theme") === "dark"

  // Pre-create all fields sets
  for (let y = START_YEAR; y < START_YEAR + years; y++) {
    for (let d = 1; d <= days; d++) {
      for (let p = 1; p <= problems; p++) {
        const fields = document.createElement("div")
        fields.className = "fields hidden"

        const descriptionArea = document.createElement("textarea")
        descriptionArea.className = "big-field"
        descriptionArea.value = retrieve_problem(y, d, p)
        descriptionArea.disabled = true

        const codeArea = document.createElement("div")
        codeArea.className = "big-field"
        const code = retrieve_html(y, d, p, isDark)
        complete[y - START_YEAR][d - 1][p - 1] = !code.includes("todo!")
        codeArea.innerHTML = code
        // {
        //   (codeArea.firstChild as HTMLElement).style.margin = "0";
        // }

        const inputArea = document.createElement("textarea")
        inputArea.id = "inputArea"
        inputArea.className = "big-field"
        inputArea.placeholder = "Input here..."

        inputArea.oninput = () => {
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
          outputArea.value = solve(inputArea.value, y, d, p)
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
  for (let y = START_YEAR; y < START_YEAR + years; y++) {
    const btn = document.createElement("button")
    let completeCount = 0
    for (let d = 1; d <= days; d++) {
      for (let p = 1; p <= problems; p++) {
        if (complete[y - START_YEAR][d - 1][p - 1]) completeCount++
      }
    }
    const percentage = Math.floor((completeCount / (days * problems)) * 100)
    btn.textContent = y.toString() + (percentage === 100 ? ` ${STAR}` : ` ${percentage}%`)
    if (y === activeYear) btn.classList.add("active")
    btn.addEventListener("click", () => {
      activeYear = y
      activeDay = 1
      activeProblem = 1
      updateActiveTab(yearsWrapper, btn)
      updateActiveTab(daysWrapper, daysWrapper.firstChild as HTMLButtonElement)
      updateActiveTab(problemsWrapper, problemsWrapper.firstChild as HTMLButtonElement)
      updateDayTabs()
      updateProblemTabs()
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
      activeProblem = 1
      updateActiveTab(daysWrapper, btn)
      updateActiveTab(problemsWrapper, problemsWrapper.firstChild as HTMLButtonElement)
      updateProblemTabs()
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
    const key = `${activeYear}-${activeDay}-${activeProblem}`
    const currentFields = fieldsMap.get(key)
    if (currentFields) currentFields.classList.remove("hidden")
  }

  function updateURL() {
    const url = new URL(window.location.href)
    url.searchParams.set("year", activeYear.toString())
    url.searchParams.set("day", activeDay.toString())
    url.searchParams.set("problem", activeProblem.toString())
    history.replaceState(null, "", url.toString())
  }

  function updateDayTabs() {
    const dayButtons = daysWrapper.querySelectorAll("button")
    dayButtons.forEach((btn, index) => {
      const d = index + 1
      let starCount = 0
      for (let p = 1; p <= problems; p++) {
        if (complete[activeYear - START_YEAR][d - 1][p - 1]) {
          starCount++
        }
      }
      let starString = ""
      if (starCount >= 2) {
        starString = STAR.repeat(2)
      } else if (starCount === 1) {
        starString = STAR
      }
      btn.textContent = `day ${d} ${starString}`
    })
  }

  function updateProblemTabs() {
    const problemButtons = problemsWrapper.querySelectorAll("button")
    problemButtons.forEach((btn, index) => {
      const p = index + 1
      if (complete[activeYear - START_YEAR][activeDay - 1][p - 1]) {
        btn.textContent = `problem ${p} ${STAR}`
      } else {
        btn.textContent = `problem ${p}`
      }
    })
  }

  function updateThemeForAllFields(newIsDark: boolean) {
    fieldsMap.forEach((fieldsDiv, key) => {
      const [yStr, dStr, pStr] = key.split("-")
      const y = parseInt(yStr, 10)
      const d = parseInt(dStr, 10)
      const p = parseInt(pStr, 10)

      const newCode = retrieve_html(y, d, p, newIsDark)
      const codeArea = fieldsDiv.querySelector("div.big-field") as HTMLDivElement
      if (codeArea) codeArea.innerHTML = newCode
    })
    isDark = newIsDark
  }

  function setupThemeObserver() {
    const observer = new MutationObserver(() => {
      const newIsDark = document.documentElement.getAttribute("saved-theme") === "dark"
      if (newIsDark !== isDark) {
        updateThemeForAllFields(newIsDark)
      }
    })
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["saved-theme"],
    })
  }

  setupThemeObserver()

  showCurrentFields()
  updateDayTabs()
  updateProblemTabs()
}
