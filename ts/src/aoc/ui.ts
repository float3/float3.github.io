const START_YEAR = 2015
const STAR = "\u2B50"

import {
  aoc_completion_percentage,
  aoc_day_count_for_year,
  aoc_day_status,
  aoc_problem_count_for_day,
  retrieve_html,
  retrieve_problem,
  solve,
} from "wasm"

export interface TabConfig {
  years: number
  days: number
  problems: number
}

type Progress = {
  completeCount: number
  totalProblems: number
}

type DayStatus = "complete" | "partial" | "todo"

export function createTabs(container: HTMLElement, config: TabConfig) {
  const { years, days, problems } = config

  function dayCountForYear(year: number) {
    return aoc_day_count_for_year(year, days)
  }

  function problemCountForDay(year: number, day: number) {
    return aoc_problem_count_for_day(year, day, problems)
  }

  let activeYear = START_YEAR + years - 1
  let activeDay = 1
  let activeProblem = 1

  const urlParams = new URLSearchParams(window.location.search)

  const initialYearParam = urlParams.get("year")
  const initialDayParam = urlParams.get("day")
  const initialProblemParam = urlParams.get("problem")

  if (initialYearParam) {
    const parsedYear = parseInt(initialYearParam, 10)
    if (!isNaN(parsedYear) && parsedYear >= START_YEAR && parsedYear < START_YEAR + years) {
      activeYear = parsedYear
    }
  }

  if (initialDayParam) {
    const parsedDay = parseInt(initialDayParam, 10)
    if (!isNaN(parsedDay) && parsedDay >= 1 && parsedDay <= dayCountForYear(activeYear)) {
      activeDay = parsedDay
    }
  }

  if (initialProblemParam) {
    const parsedProblem = parseInt(initialProblemParam, 10)
    if (
      !isNaN(parsedProblem) &&
      parsedProblem >= 1 &&
      parsedProblem <= problemCountForDay(activeYear, activeDay)
    ) {
      activeProblem = parsedProblem
    }
  }

  const yearsWrapper = document.createElement("div")
  yearsWrapper.className = "years tabs"
  yearsWrapper.setAttribute("aria-label", "Advent of Code year")

  const progressSummary = document.createElement("div")
  progressSummary.className = "aoc-progress-summary"
  progressSummary.setAttribute("aria-live", "polite")

  const daysWrapper = document.createElement("div")
  daysWrapper.className = "days aoc-calendar"
  daysWrapper.setAttribute("aria-label", "Advent of Code calendar")

  const problemsWrapper = document.createElement("div")
  problemsWrapper.className = "problems tabs"
  problemsWrapper.setAttribute("aria-label", "Advent of Code problem")

  const contentWrapper = document.createElement("div")
  contentWrapper.className = "content"

  const fieldsMap = new Map<string, HTMLDivElement>()
  const yearButtons: HTMLButtonElement[] = []
  const dayButtons: HTMLButtonElement[] = []
  const problemButtons: HTMLButtonElement[] = []

  const complete: boolean[][][] = Array.from({ length: years }, () =>
    Array.from({ length: days }, () => Array.from({ length: problems }, () => false)),
  )

  let isDark = document.documentElement.getAttribute("saved-theme") === "dark"

  for (let y = START_YEAR; y < START_YEAR + years; y++) {
    for (let d = 1; d <= dayCountForYear(y); d++) {
      for (let p = 1; p <= problemCountForDay(y, d); p++) {
        const fields = document.createElement("div")
        fields.className = "fields hidden"

        const descriptionArea = document.createElement("textarea")
        descriptionArea.className = "big-field description-field"
        descriptionArea.value = retrieve_problem(y, d, p)
        descriptionArea.disabled = true
        descriptionArea.setAttribute(
          "aria-label",
          `Problem description for ${y} day ${d} part ${p}`,
        )

        const codeArea = document.createElement("div")
        codeArea.className = "big-field code-field"
        const code = retrieve_html(y, d, p, isDark)
        complete[y - START_YEAR][d - 1][p - 1] = !code.includes("todo!")
        codeArea.innerHTML = code

        const inputArea = document.createElement("textarea")
        inputArea.className = "big-field input-field"
        inputArea.placeholder = "Input here..."
        inputArea.dataset.aocInput = "true"
        inputArea.setAttribute("aria-label", `Puzzle input for ${y} day ${d} part ${p}`)

        inputArea.oninput = () => {
          const otherProblem = p === 1 ? 2 : 1
          const key = `${y}-${d}-${otherProblem}`
          const fields = fieldsMap.get(key)
          if (!fields) return

          const otherInputArea = fields.querySelector(
            "textarea[data-aoc-input='true']",
          ) as HTMLTextAreaElement | null
          if (otherInputArea) otherInputArea.value = inputArea.value
        }

        const outputArea = document.createElement("textarea")
        outputArea.className = "small-field output-field"
        outputArea.disabled = true
        outputArea.value = ""
        outputArea.setAttribute("aria-label", `Solution output for ${y} day ${d} part ${p}`)

        const solveButton = document.createElement("button")
        solveButton.className = "solve-button"
        solveButton.type = "button"
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

  for (let y = START_YEAR; y < START_YEAR + years; y++) {
    const btn = document.createElement("button")
    btn.type = "button"
    btn.addEventListener("click", () => {
      activeYear = y
      activeDay = 1
      activeProblem = 1
      updateYearTabs()
      updateDayTabs()
      updateProblemTabs()
      showCurrentFields()
      updateURL()
    })
    yearButtons.push(btn)
    yearsWrapper.appendChild(btn)
  }

  for (let d = 1; d <= days; d++) {
    const btn = document.createElement("button")
    btn.className = "aoc-day"
    btn.type = "button"
    btn.appendChild(createDayNumber(d))
    btn.appendChild(createDayMeta())
    btn.addEventListener("click", () => {
      if (d > dayCountForYear(activeYear)) return

      activeDay = d
      activeProblem = 1
      updateDayTabs()
      updateProblemTabs()
      showCurrentFields()
      updateURL()
    })
    dayButtons.push(btn)
    daysWrapper.appendChild(btn)
  }

  for (let p = 1; p <= problems; p++) {
    const btn = document.createElement("button")
    btn.type = "button"
    btn.addEventListener("click", () => {
      activeProblem = p
      updateProblemTabs()
      showCurrentFields()
      updateURL()
    })
    problemButtons.push(btn)
    problemsWrapper.appendChild(btn)
  }

  container.appendChild(yearsWrapper)
  container.appendChild(progressSummary)
  container.appendChild(daysWrapper)
  container.appendChild(problemsWrapper)
  container.appendChild(contentWrapper)

  function getYearProgress(year: number): Progress {
    let completeCount = 0
    let totalProblems = 0

    for (let d = 1; d <= dayCountForYear(year); d++) {
      const dayProblemCount = problemCountForDay(year, d)
      totalProblems += dayProblemCount

      for (let p = 1; p <= dayProblemCount; p++) {
        if (complete[year - START_YEAR][d - 1][p - 1]) completeCount++
      }
    }

    return { completeCount, totalProblems }
  }

  function getDayProgress(year: number, day: number): Progress {
    let completeCount = 0
    const totalProblems = problemCountForDay(year, day)

    for (let p = 1; p <= totalProblems; p++) {
      if (complete[year - START_YEAR][day - 1][p - 1]) completeCount++
    }

    return { completeCount, totalProblems }
  }

  function getDayStatus(year: number, day: number): DayStatus {
    const { completeCount, totalProblems } = getDayProgress(year, day)
    return dayStatusName(aoc_day_status(completeCount, totalProblems))
  }

  function updateYearTabs() {
    yearButtons.forEach((btn, index) => {
      const year = START_YEAR + index
      const { completeCount, totalProblems } = getYearProgress(year)
      const percentage = aoc_completion_percentage(completeCount, totalProblems)
      btn.textContent = `${year} ${percentage === 100 ? STAR : `${percentage}%`}`
      btn.classList.toggle("active", year === activeYear)
      btn.setAttribute("aria-label", `${year}: ${completeCount} of ${totalProblems} stars complete`)
      btn.setAttribute("aria-pressed", String(year === activeYear))
    })
  }

  function updateDayTabs() {
    dayButtons.forEach((btn, index) => {
      const day = index + 1
      const isAvailable = day <= dayCountForYear(activeYear)
      btn.hidden = !isAvailable
      btn.disabled = !isAvailable
      btn.classList.toggle("active", isAvailable && day === activeDay)

      if (!isAvailable) return

      const { completeCount, totalProblems } = getDayProgress(activeYear, day)
      const status = getDayStatus(activeYear, day)
      const meta = btn.querySelector(".aoc-day-meta")
      btn.dataset.status = status
      btn.setAttribute(
        "aria-label",
        `Day ${day}: ${completeCount} of ${totalProblems} stars complete`,
      )
      btn.setAttribute("aria-pressed", String(day === activeDay))
      btn.title = `Day ${day}: ${completeCount}/${totalProblems} complete`

      if (meta) {
        meta.textContent =
          completeCount === totalProblems
            ? STAR.repeat(totalProblems)
            : `${completeCount}/${totalProblems}`
      }
    })

    updateProgressSummary()
  }

  function updateProblemTabs() {
    const activeProblemCount = problemCountForDay(activeYear, activeDay)

    if (activeProblem > activeProblemCount) {
      activeProblem = activeProblemCount
    }

    problemButtons.forEach((btn, index) => {
      const problem = index + 1
      const isAvailable = problem <= activeProblemCount
      btn.hidden = !isAvailable
      btn.disabled = !isAvailable
      btn.classList.toggle("active", isAvailable && problem === activeProblem)

      if (!isAvailable) return

      const isComplete = complete[activeYear - START_YEAR][activeDay - 1][problem - 1]
      btn.textContent = `part ${problem}${isComplete ? ` ${STAR}` : ""}`
      btn.setAttribute("aria-label", `Part ${problem}${isComplete ? ", complete" : ", incomplete"}`)
      btn.setAttribute("aria-pressed", String(problem === activeProblem))
    })
  }

  function updateProgressSummary() {
    const yearProgress = getYearProgress(activeYear)
    const dayProgress = getDayProgress(activeYear, activeDay)
    progressSummary.textContent = `${activeYear}: ${yearProgress.completeCount}/${yearProgress.totalProblems} stars complete. Day ${activeDay}: ${dayProgress.completeCount}/${dayProgress.totalProblems}.`
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

  function updateThemeForAllFields(newIsDark: boolean) {
    fieldsMap.forEach((fieldsDiv, key) => {
      const [yStr, dStr, pStr] = key.split("-")
      const y = parseInt(yStr, 10)
      const d = parseInt(dStr, 10)
      const p = parseInt(pStr, 10)

      const newCode = retrieve_html(y, d, p, newIsDark)
      const codeArea = fieldsDiv.querySelector(".code-field") as HTMLDivElement | null
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

  updateYearTabs()
  updateDayTabs()
  updateProblemTabs()
  showCurrentFields()
}

function createDayNumber(day: number) {
  const number = document.createElement("span")
  number.className = "aoc-day-number"
  number.textContent = day.toString()
  return number
}

function createDayMeta() {
  const meta = document.createElement("span")
  meta.className = "aoc-day-meta"
  meta.textContent = "0/2"
  return meta
}

function dayStatusName(status: number): DayStatus {
  if (status === 2) return "complete"
  if (status === 1) return "partial"
  return "todo"
}
