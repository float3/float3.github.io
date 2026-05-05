import { bayes_number, solve_bayes_percent } from "wasm"

type BayesMode = "computed-evidence" | "known-evidence"

type BayesEventLabels = {
  eventA: string
  eventB: string
}

type BayesColorRole = "posterior" | "likelihood" | "prior" | "evidence" | "numerator" | "neutral"
type InlineContent = string | Node
type BayesFormulaValues = {
  prior: number
  likelihood: number
  evidence: number
  numerator: number
}

const bayesColors = {
  posterior: "#0466e7",
  likelihood: "#ffbc3f",
  prior: "#fe7fb3",
  evidence: "#dd6fff",
} as const

const percentFormatter = new Intl.NumberFormat(undefined, {
  maximumFractionDigits: 6,
})
const mathMlNamespace = "http://www.w3.org/1998/Math/MathML"

function probabilityToPercent(value: number): string {
  return percentFormatter.format(value * 100)
}

function setResult(element: HTMLElement, value: number, suffix = "%") {
  element.textContent = `${probabilityToPercent(value)}${suffix}`
}

function renderBayesSolver() {
  const root = document.getElementById("bayes-solver")
  if (!root) {
    return
  }

  const solverRoot = root
  solverRoot.textContent = ""

  const heading = document.createElement("h2")
  heading.id = "bayes-solver-heading"
  heading.textContent = "solver"

  const form = document.createElement("form")
  form.className = "bayes-solver-form"

  const eventGrid = document.createElement("div")
  eventGrid.className = "bayes-event-fields"

  const eventA = createEventField("event A", "a person has the disease", "a person has the disease")
  const eventB = createEventField(
    "event B",
    "the screening test is positive",
    "the screening test is positive",
  )
  eventGrid.append(eventA.label, eventB.label)

  const definition = document.createElement("p")
  definition.className = "bayes-solver-definition"

  const fieldGrid = document.createElement("div")
  fieldGrid.className = "bayes-solver-fields"

  const prior = createPercentField(
    [bayesTerm("P(A)", "prior"), ", ", bayesPhrase("the prior probability of event A", "prior")],
    "1",
    "prior",
  )
  const likelihood = createPercentField(
    [
      bayesTerm("P(B | A)", "likelihood"),
      ", ",
      bayesPhrase("the likelihood of event B given event A", "likelihood"),
    ],
    "90",
    "likelihood",
  )
  const falsePositive = createPercentField(
    [
      bayesTerm("P(B | not A)", "neutral"),
      ", ",
      bayesPhrase("the probability of event B given event A did not occur", "neutral"),
    ],
    "5",
    "neutral",
  )
  const evidence = createPercentField(
    [
      bayesTerm("P(B)", "evidence"),
      ", ",
      bayesPhrase("the marginal probability of event B", "evidence"),
    ],
    "5.85",
    "evidence",
  )

  fieldGrid.append(prior.label, likelihood.label, falsePositive.label, evidence.label)

  const modeLabel = document.createElement("label")
  modeLabel.className = "bayes-solver-toggle"

  const modeToggle = document.createElement("input")
  modeToggle.type = "checkbox"
  modeToggle.checked = true

  const modeText = document.createElement("span")
  modeText.className = "bayes-solver-toggle-text"
  setInlineContent(
    modeText,
    "compute ",
    bayesTerm("P(B)", "evidence"),
    " from ",
    bayesTerm("P(B | not A)", "neutral"),
  )
  modeLabel.append(modeToggle, modeText)

  const output = document.createElement("div")
  output.className = "bayes-solver-output"

  const posterior = createOutputItem(
    [
      bayesTerm("P(A | B)", "posterior"),
      ", ",
      bayesPhrase("the probability of event A occurring given event B has occurred", "posterior"),
    ],
    "posterior",
    "posterior",
  )
  const numerator = createOutputItem(
    [
      bayesTerm("P(B | A)", "likelihood"),
      " times ",
      bayesTerm("P(A)", "prior"),
      ", ",
      bayesPhrase("likelihood times prior", "numerator"),
    ],
    "numerator",
    "numerator",
  )
  const evidenceOutput = createOutputItem(
    [
      bayesTerm("P(B)", "evidence"),
      ", ",
      bayesPhrase("the marginal probability of event B", "evidence"),
    ],
    "evidence",
    "evidence",
  )
  const odds = createOutputItem(
    [bayesPhrase("posterior odds", "posterior"), " from ", bayesTerm("P(A | B)", "posterior")],
    "odds",
    "posterior",
  )

  output.append(posterior.element, numerator.element, evidenceOutput.element, odds.element)

  const solution = createSolutionOutput()

  const error = document.createElement("p")
  error.className = "bayes-solver-error"
  error.setAttribute("role", "status")

  form.append(eventGrid, definition, fieldGrid, modeLabel, output, solution.element, error)
  solverRoot.append(heading, form)

  function getMode(): BayesMode {
    return modeToggle.checked ? "computed-evidence" : "known-evidence"
  }

  function getEventLabels(): BayesEventLabels {
    return {
      eventA: eventName(eventA.input.value, "event A"),
      eventB: eventName(eventB.input.value, "event B"),
    }
  }

  function updateCopy(labels: BayesEventLabels, values: BayesFormulaValues) {
    const notEventA = `it is not true that ${labels.eventA}`
    const priorPercent = formatProbabilityPercent(values.prior)
    const likelihoodPercent = formatProbabilityPercent(values.likelihood)
    const evidencePercent = formatProbabilityPercent(values.evidence)
    const posteriorPercent = formatPosteriorPercent(values)

    setInlineContent(
      definition,
      "To find ",
      bayesPhrase(
        `the probability of the event "${labels.eventA}" occurring given the event "${labels.eventB}" has occurred`,
        "posterior",
      ),
      ", we multiply ",
      bayesPhrase(
        `the ${likelihoodPercent} likelihood of the event "${labels.eventB}" given the event "${labels.eventA}"`,
        "likelihood",
      ),
      " by ",
      bayesPhrase(`the ${priorPercent} prior probability of the event "${labels.eventA}"`, "prior"),
      " and divide by ",
      bayesPhrase(
        `the ${evidencePercent} marginal probability of the event "${labels.eventB}"`,
        "evidence",
      ),
      ". This gives ",
      bayesPhrase(posteriorPercent, "posterior"),
      ".",
    )

    prior.setDefinition(
      bayesTerm("P(A)", "prior"),
      ", ",
      bayesPhrase(`the prior probability that ${labels.eventA}`, "prior"),
    )
    likelihood.setDefinition(
      bayesTerm("P(B | A)", "likelihood"),
      ", ",
      bayesPhrase(`the likelihood that ${labels.eventB} given that ${labels.eventA}`, "likelihood"),
    )
    falsePositive.setDefinition(
      bayesTerm("P(B | not A)", "neutral"),
      ", ",
      bayesPhrase(`the probability that ${labels.eventB} given that ${notEventA}`, "neutral"),
    )
    evidence.setDefinition(
      bayesTerm("P(B)", "evidence"),
      ", ",
      bayesPhrase(`the marginal probability that ${labels.eventB}`, "evidence"),
    )

    if (getMode() === "computed-evidence") {
      setInlineContent(
        modeText,
        "compute ",
        bayesTerm("P(B)", "evidence"),
        " from the ",
        bayesPhrase("false positive rate", "neutral"),
      )
    } else {
      setInlineContent(modeText, "use a known marginal probability ", bayesTerm("P(B)", "evidence"))
    }

    posterior.setDefinition(
      bayesTerm("P(A | B)", "posterior"),
      ", ",
      bayesPhrase(`the probability that ${labels.eventA} given that ${labels.eventB}`, "posterior"),
    )
    numerator.setDefinition(
      bayesTerm("P(B | A)", "likelihood"),
      " times ",
      bayesTerm("P(A)", "prior"),
      ", ",
      bayesPhrase("likelihood times prior", "numerator"),
    )
    evidenceOutput.setDefinition(
      bayesTerm("P(B)", "evidence"),
      ", ",
      bayesPhrase(`the marginal probability that ${labels.eventB}`, "evidence"),
    )
    odds.setDefinition(
      bayesPhrase("posterior odds", "posterior"),
      " from ",
      bayesTerm("P(A | B)", "posterior"),
    )
  }

  function update() {
    const mode = getMode()
    const labels = getEventLabels()

    const result = solve_bayes_percent(
      Number.parseFloat(prior.input.value),
      Number.parseFloat(likelihood.input.value),
      Number.parseFloat(falsePositive.input.value),
      Number.parseFloat(evidence.input.value),
      mode === "computed-evidence",
    )
    const evidenceValue = result.evidence
    const numeratorValue = result.numerator
    const posteriorValue = result.posterior
    const oddsValue = result.odds
    const errorCode = result.error_code
    result.free()

    falsePositive.input.disabled = mode !== "computed-evidence"
    evidence.input.disabled = mode === "computed-evidence"

    if (mode === "computed-evidence") {
      evidence.input.value = probabilityToPercent(evidenceValue)
    }

    setResult(numerator.value, numeratorValue)
    setResult(evidenceOutput.value, evidenceValue)

    const formulaValues = {
      prior: clampProbability(Number.parseFloat(prior.input.value) / 100),
      likelihood: clampProbability(Number.parseFloat(likelihood.input.value) / 100),
      evidence: evidenceValue,
      numerator: numeratorValue,
    }

    updateCopy(labels, formulaValues)
    updateSolution(solution, formulaValues)

    if (errorCode === 1) {
      posterior.value.textContent = "-"
      odds.value.textContent = "-"
      solution.equation.textContent = "-"
      error.textContent = `The marginal probability that ${labels.eventB} must be greater than 0.`
      solverRoot.classList.add("has-error")
      return
    }

    if (errorCode === 2) {
      error.textContent =
        `The likelihood times prior is larger than the marginal probability of ` +
        `${labels.eventB}, so these probabilities are inconsistent.`
      solverRoot.classList.add("has-error")
    } else {
      error.textContent = ""
      solverRoot.classList.remove("has-error")
    }

    setResult(posterior.value, posteriorValue)
    odds.value.textContent = Number.isFinite(oddsValue)
      ? `${percentFormatter.format(oddsValue)}:1`
      : "certain"
  }

  form.addEventListener("input", update)
  form.addEventListener("submit", (event) => event.preventDefault())
  modeToggle.addEventListener("change", update)
  update()
}

function eventName(value: string, fallback: string) {
  const trimmed = value.trim()
  return trimmed.length > 0 ? trimmed : fallback
}

function clampProbability(value: number) {
  if (!Number.isFinite(value)) return 0
  return Math.min(1, Math.max(0, value))
}

function formatProbabilityPercent(value: number) {
  return Number.isFinite(value) ? `${probabilityToPercent(value)}%` : "an unknown percentage"
}

function updateSolution(
  solution: ReturnType<typeof createSolutionOutput>,
  values: BayesFormulaValues,
) {
  solution.equation.replaceChildren(createBayesEquation(values))
}

function formatPosteriorPercent(values: BayesFormulaValues) {
  if (values.evidence <= 0) {
    return "-"
  }

  return `${probabilityToPercent(values.numerator / values.evidence)}%`
}

function createBayesEquation(values: BayesFormulaValues) {
  return mathElement(
    "math",
    { class: "bayes-rendered-math", display: "block" },
    mathElement(
      "mrow",
      {},
      coloredMath("posterior", mathElement("mtext", {}, formatPosteriorPercent(values))),
      mathElement("mo", {}, "="),
      fraction(
        multiply(mathNumber(values.likelihood, "likelihood"), mathNumber(values.prior, "prior")),
        mathNumber(values.evidence, "evidence"),
      ),
    ),
  )
}

function fraction(numerator: Element, denominator: Element) {
  return mathElement("mfrac", {}, numerator, denominator)
}

function multiply(left: Element, right: Element) {
  return mathElement("mrow", {}, left, mathElement("mo", {}, "\u00b7"), right)
}

function mathNumber(value: number, role: keyof typeof bayesColors) {
  return coloredMath(
    role,
    mathElement(Number.isFinite(value) ? "mn" : "mtext", {}, bayes_number(value)),
  )
}

function coloredMath(role: keyof typeof bayesColors, ...children: Array<Element | string>) {
  const element = mathElement("mrow", { mathcolor: bayesColors[role] }, ...children)
    ; (element as HTMLElement).style.color = bayesColors[role]
  return element
}

function mathElement(
  tag: string,
  attrs: Record<string, string> = {},
  ...children: Array<Element | string>
) {
  const element = document.createElementNS(mathMlNamespace, tag)
  for (const [key, value] of Object.entries(attrs)) {
    element.setAttribute(key, value)
  }
  for (const child of children) {
    element.append(child)
  }
  return element
}

function createSolutionOutput() {
  const element = document.createElement("section")
  element.className = "bayes-solution"
  element.setAttribute("aria-live", "polite")

  const equation = document.createElement("div")
  equation.className = "bayes-solution-equation"
  equation.setAttribute("aria-label", "Bayes theorem equation with current numbers")

  element.append(equation)
  return { element, equation }
}

function setInlineContent(element: HTMLElement, ...content: InlineContent[]) {
  element.textContent = ""
  for (const item of content) {
    element.append(item)
  }
}

function bayesTerm(text: string, role: BayesColorRole) {
  const term = document.createElement("span")
  term.className = "bayes-term"
  term.dataset.bayesRole = role
  term.textContent = text
  return term
}

function bayesPhrase(text: string, role: BayesColorRole) {
  const phrase = document.createElement("span")
  phrase.className = "bayes-phrase"
  phrase.dataset.bayesRole = role
  phrase.textContent = text
  return phrase
}

function createEventField(labelText: string, placeholder: string, value: string) {
  const label = document.createElement("label")
  label.className = "bayes-event-field"

  const descriptor = document.createElement("span")
  descriptor.className = "bayes-solver-definition-text"
  descriptor.textContent = labelText

  const input = document.createElement("input")
  input.type = "text"
  input.placeholder = placeholder
  input.value = value
  input.autocomplete = "off"

  label.append(descriptor, input)
  return { label, input }
}

function createPercentField(labelContent: InlineContent[], value: string, role: BayesColorRole) {
  const label = document.createElement("label")
  label.className = "bayes-solver-field"
  label.dataset.bayesRole = role

  const definition = document.createElement("span")
  definition.className = "bayes-solver-definition-text"
  setInlineContent(definition, ...labelContent)

  const control = document.createElement("span")
  control.className = "bayes-solver-control"

  const input = document.createElement("input")
  input.type = "number"
  input.min = "0"
  input.max = "100"
  input.step = "0.01"
  input.value = value
  input.inputMode = "decimal"

  const percent = document.createElement("span")
  percent.textContent = "%"

  control.append(input, percent)
  label.append(definition, control)

  return {
    label,
    input,
    setDefinition(...value: InlineContent[]) {
      setInlineContent(definition, ...value)
    },
  }
}

function createOutputItem(labelContent: InlineContent[], title: string, role: BayesColorRole) {
  const element = document.createElement("article")
  element.className = "bayes-solver-result"
  element.dataset.bayesRole = role

  const label = document.createElement("span")
  label.className = "bayes-solver-label"

  const definition = document.createElement("span")
  definition.className = "bayes-solver-definition-text"
  setInlineContent(definition, ...labelContent)

  const value = document.createElement("strong")
  value.title = title

  label.append(definition)
  element.append(label, value)
  return {
    element,
    value,
    setDefinition(...value: InlineContent[]) {
      setInlineContent(definition, ...value)
    },
  }
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", renderBayesSolver, { once: true })
} else {
  renderBayesSolver()
}
