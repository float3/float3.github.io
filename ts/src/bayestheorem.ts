import { solve_bayes_percent } from "wasm"

type BayesMode = "computed-evidence" | "known-evidence"

type BayesEventLabels = {
  eventA: string
  eventB: string
}

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

  const prior = createPercentField("P(A)", "the prior probability of event A", "1")
  const likelihood = createPercentField("P(B | A)", "the likelihood of event B given event A", "90")
  const falsePositive = createPercentField(
    "P(B | not A)",
    "the probability of event B given event A did not occur",
    "5",
  )
  const evidence = createPercentField("P(B)", "the marginal probability of event B", "5.85")

  fieldGrid.append(prior.label, likelihood.label, falsePositive.label, evidence.label)

  const modeLabel = document.createElement("label")
  modeLabel.className = "bayes-solver-toggle"

  const modeToggle = document.createElement("input")
  modeToggle.type = "checkbox"
  modeToggle.checked = true

  const modeText = document.createElement("span")
  modeText.textContent = "compute P(B) from P(B | not A)"
  modeLabel.append(modeToggle, modeText)

  const output = document.createElement("div")
  output.className = "bayes-solver-output"
  output.setAttribute("aria-live", "polite")

  const posterior = createOutputItem(
    "P(A | B)",
    "the probability of event A occurring given event B has occurred",
    "posterior",
  )
  const numerator = createOutputItem(
    "P(B | A) * P(A)",
    "the likelihood of event B given event A times the prior probability of event A",
    "numerator",
  )
  const evidenceOutput = createOutputItem("P(B)", "the marginal probability of event B", "evidence")
  const odds = createOutputItem("odds", "posterior odds for event A", "odds")

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

    definition.textContent =
      `Example: ${priorPercent} is the prior probability that ${labels.eventA}; ` +
      `${likelihoodPercent} is the likelihood that ${labels.eventB} given that ` +
      `${labels.eventA}; and ${falsePositivePercent} is the probability that ` +
      `${labels.eventB} given that ${notEventA}. The solver finds the probability that ` +
      `${labels.eventA} after observing that ${labels.eventB}.`

    prior.setDefinition(`the prior probability that ${labels.eventA}`)
    likelihood.setDefinition(`the likelihood that ${labels.eventB} given that ${labels.eventA}`)
    falsePositive.setDefinition(`the probability that ${labels.eventB} given that ${notEventA}`)
    evidence.setDefinition(`the marginal probability that ${labels.eventB}`)

    modeText.textContent =
      getMode() === "computed-evidence"
        ? `compute the marginal probability that ${labels.eventB} from the false positive rate`
        : `use a known marginal probability that ${labels.eventB}`

    posterior.setDefinition(
      `the probability that ${labels.eventA} given that ${labels.eventB}`,
    )
    numerator.setDefinition(
      `the likelihood that ${labels.eventB} given that ${labels.eventA} times ` +
        `the prior probability that ${labels.eventA}`,
    )
    evidenceOutput.setDefinition(`the marginal probability that ${labels.eventB}`)
    odds.setDefinition(`posterior odds that ${labels.eventA}`)
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

function createPercentField(symbol: string, labelText: string, value: string) {
  const label = document.createElement("label")
  label.className = "bayes-solver-field"

  const text = document.createElement("span")
  text.className = "bayes-solver-label"

  const notation = document.createElement("span")
  notation.className = "bayes-solver-symbol"
  notation.textContent = symbol

  const definition = document.createElement("span")
  definition.className = "bayes-solver-definition-text"
  definition.textContent = labelText

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
    setDefinition(value: string) {
      definition.textContent = value
    },
  }
}

function createOutputItem(symbol: string, labelText: string, title: string) {
  const element = document.createElement("article")
  element.className = "bayes-solver-result"

  const label = document.createElement("span")
  label.className = "bayes-solver-label"

  const notation = document.createElement("span")
  notation.className = "bayes-solver-symbol"
  notation.textContent = symbol

  const definition = document.createElement("span")
  definition.className = "bayes-solver-definition-text"
  definition.textContent = labelText

  const value = document.createElement("strong")
  value.title = title

  label.append(notation, definition)
  element.append(label, value)
  return {
    element,
    value,
    setDefinition(value: string) {
      definition.textContent = value
    },
  }
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", renderBayesSolver, { once: true })
} else {
  renderBayesSolver()
}
