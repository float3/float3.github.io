import { solve_bayes_percent } from "wasm"

type BayesMode = "computed-evidence" | "known-evidence"

type BayesEventLabels = {
  eventA: string
  eventB: string
}

type BayesColorRole =
  | "posterior"
  | "likelihood"
  | "prior"
  | "evidence"
  | "numerator"
  | "neutral"
type InlineContent = string | Node

const percentFormatter = new Intl.NumberFormat(undefined, {
  maximumFractionDigits: 6,
})

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
  heading.textContent = "bayes theorem solver"

  const form = document.createElement("form")
  form.className = "bayes-solver-form"

  const eventGrid = document.createElement("div")
  eventGrid.className = "bayes-event-fields"

  const eventA = createEventField(
    "A",
    "event A",
    "a person has the disease",
    "a person has the disease",
  )
  const eventB = createEventField(
    "B",
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
    [bayesTerm("P(A)", "prior")],
    [bayesTerm("P(A)", "prior"), ", the prior probability of event A"],
    "1",
    "prior",
  )
  const likelihood = createPercentField(
    [bayesTerm("P(B | A)", "likelihood")],
    [bayesTerm("P(B | A)", "likelihood"), ", the likelihood of event B given event A"],
    "90",
    "likelihood",
  )
  const falsePositive = createPercentField(
    [bayesTerm("P(B | not A)", "neutral")],
    [
      bayesTerm("P(B | not A)", "neutral"),
      ", the probability of event B given event A did not occur",
    ],
    "5",
    "neutral",
  )
  const evidence = createPercentField(
    [bayesTerm("P(B)", "evidence")],
    [bayesTerm("P(B)", "evidence"), ", the marginal probability of event B"],
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
  output.setAttribute("aria-live", "polite")

  const posterior = createOutputItem(
    [bayesTerm("P(A | B)", "posterior")],
    [
      bayesTerm("P(A | B)", "posterior"),
      ", the probability of event A occurring given event B has occurred",
    ],
    "posterior",
    "posterior",
  )
  const numerator = createOutputItem(
    [bayesTerm("P(B | A)", "likelihood"), " * ", bayesTerm("P(A)", "prior")],
    [
      bayesTerm("P(B | A)", "likelihood"),
      " times ",
      bayesTerm("P(A)", "prior"),
      ", likelihood times prior",
    ],
    "numerator",
    "numerator",
  )
  const evidenceOutput = createOutputItem(
    [bayesTerm("P(B)", "evidence")],
    [bayesTerm("P(B)", "evidence"), ", the marginal probability of event B"],
    "evidence",
    "evidence",
  )
  const odds = createOutputItem(
    ["odds"],
    ["posterior odds from ", bayesTerm("P(A | B)", "posterior")],
    "odds",
    "posterior",
  )

  output.append(posterior.element, numerator.element, evidenceOutput.element, odds.element)

  const error = document.createElement("p")
  error.className = "bayes-solver-error"
  error.setAttribute("role", "status")

  form.append(eventGrid, definition, fieldGrid, modeLabel, output, error)
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

  function updateCopy(labels: BayesEventLabels) {
    const notEventA = `it is not true that ${labels.eventA}`
    const priorPercent = percentInputText(prior.input)
    const likelihoodPercent = percentInputText(likelihood.input)
    const falsePositivePercent = percentInputText(falsePositive.input)

    setInlineContent(
      definition,
      "Example: ",
      priorPercent,
      " is ",
      bayesTerm("P(A)", "prior"),
      ", the prior probability that ",
      labels.eventA,
      "; ",
      likelihoodPercent,
      " is ",
      bayesTerm("P(B | A)", "likelihood"),
      ", the likelihood that ",
      labels.eventB,
      " given that ",
      labels.eventA,
      "; and ",
      falsePositivePercent,
      " is ",
      bayesTerm("P(B | not A)", "neutral"),
      ", the probability that ",
      labels.eventB,
      " given that ",
      notEventA,
      ". The solver finds ",
      bayesTerm("P(A | B)", "posterior"),
      ", the probability that ",
      labels.eventA,
      " after observing that ",
      labels.eventB,
      ".",
    )

    prior.setDefinition(bayesTerm("P(A)", "prior"), ", the prior probability that ", labels.eventA)
    likelihood.setDefinition(
      bayesTerm("P(B | A)", "likelihood"),
      ", the likelihood that ",
      labels.eventB,
      " given that ",
      labels.eventA,
    )
    falsePositive.setDefinition(
      bayesTerm("P(B | not A)", "neutral"),
      ", the probability that ",
      labels.eventB,
      " given that ",
      notEventA,
    )
    evidence.setDefinition(
      bayesTerm("P(B)", "evidence"),
      ", the marginal probability that ",
      labels.eventB,
    )

    if (getMode() === "computed-evidence") {
      setInlineContent(
        modeText,
        "compute ",
        bayesTerm("P(B)", "evidence"),
        " from the false positive rate",
      )
    } else {
      setInlineContent(modeText, "use a known marginal probability ", bayesTerm("P(B)", "evidence"))
    }

    posterior.setDefinition(
      bayesTerm("P(A | B)", "posterior"),
      ", the probability that ",
      labels.eventA,
      " given that ",
      labels.eventB,
    )
    numerator.setDefinition(
      bayesTerm("P(B | A)", "likelihood"),
      " times ",
      bayesTerm("P(A)", "prior"),
      ", likelihood times prior",
    )
    evidenceOutput.setDefinition(
      bayesTerm("P(B)", "evidence"),
      ", the marginal probability that ",
      labels.eventB,
    )
    odds.setDefinition("posterior odds from ", bayesTerm("P(A | B)", "posterior"))
  }

  function update() {
    const mode = getMode()
    const labels = getEventLabels()
    updateCopy(labels)

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

    if (errorCode === 1) {
      posterior.value.textContent = "-"
      odds.value.textContent = "-"
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

function percentInputText(input: HTMLInputElement) {
  const value = Number.parseFloat(input.value)
  return Number.isFinite(value) ? `${percentFormatter.format(value)}%` : "an unknown percentage"
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

function createEventField(
  symbol: string,
  labelText: string,
  placeholder: string,
  value: string,
) {
  const label = document.createElement("label")
  label.className = "bayes-event-field"

  const symbolText = document.createElement("span")
  symbolText.className = "bayes-solver-symbol"
  symbolText.textContent = symbol

  const descriptor = document.createElement("span")
  descriptor.className = "bayes-solver-definition-text"
  descriptor.textContent = labelText

  const input = document.createElement("input")
  input.type = "text"
  input.placeholder = placeholder
  input.value = value
  input.autocomplete = "off"

  label.append(symbolText, descriptor, input)
  return { label, input }
}

function createPercentField(
  symbol: InlineContent[],
  labelContent: InlineContent[],
  value: string,
  role: BayesColorRole,
) {
  const label = document.createElement("label")
  label.className = "bayes-solver-field"
  label.dataset.bayesRole = role

  const text = document.createElement("span")
  text.className = "bayes-solver-label"

  const notation = document.createElement("span")
  notation.className = "bayes-solver-symbol"
  setInlineContent(notation, ...symbol)

  const definition = document.createElement("span")
  definition.className = "bayes-solver-definition-text"
  setInlineContent(definition, ...labelContent)

  text.append(notation, definition)

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
  label.append(text, control)

  return {
    label,
    input,
    setDefinition(...value: InlineContent[]) {
      setInlineContent(definition, ...value)
    },
  }
}

function createOutputItem(
  symbol: InlineContent[],
  labelContent: InlineContent[],
  title: string,
  role: BayesColorRole,
) {
  const element = document.createElement("article")
  element.className = "bayes-solver-result"
  element.dataset.bayesRole = role

  const label = document.createElement("span")
  label.className = "bayes-solver-label"

  const notation = document.createElement("span")
  notation.className = "bayes-solver-symbol"
  setInlineContent(notation, ...symbol)

  const definition = document.createElement("span")
  definition.className = "bayes-solver-definition-text"
  setInlineContent(definition, ...labelContent)

  const value = document.createElement("strong")
  value.title = title

  label.append(notation, definition)
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
