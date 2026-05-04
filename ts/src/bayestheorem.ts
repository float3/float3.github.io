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
  const latex = createLatexOutput()

  const error = document.createElement("p")
  error.className = "bayes-solver-error"
  error.setAttribute("role", "status")

  form.append(eventGrid, definition, fieldGrid, modeLabel, output, latex.element, error)
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
    const formulaValues = {
      prior: clampProbability(Number.parseFloat(prior.input.value) / 100),
      likelihood: clampProbability(Number.parseFloat(likelihood.input.value) / 100),
      evidence: evidenceValue,
      numerator: numeratorValue,
    }
    const latexEquation = buildBayesLatex(formulaValues)
    latex.output.value = latexEquation.source
    latex.rendered.replaceChildren(createBayesMath(formulaValues))
    latex.status.textContent = ""

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
  latex.copyButton.addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText(latex.output.value)
      latex.status.textContent = "copied"
    } catch {
      latex.status.textContent = "copy failed"
    }
  })
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

function clampProbability(value: number) {
  if (!Number.isFinite(value)) return 0
  return Math.min(1, Math.max(0, value))
}

function latexColor(role: keyof typeof bayesColors, value: string) {
  return `\\textcolor{${bayesColors[role]}}{${value}}`
}

function latexNumber(value: number) {
  if (!Number.isFinite(value)) return "\\text{undefined}"
  const rounded = value.toFixed(6).replace(/(\.\d*?)0+$/, "$1").replace(/\.$/, "")
  return rounded === "-0" ? "0" : rounded
}

function buildBayesLatex(values: BayesFormulaValues) {
  const posterior = values.evidence > 0 ? values.numerator / values.evidence : Number.NaN
  const body = [
    "\\begin{aligned}",
    `${latexColor("posterior", "P(A \\mid B)")}`,
    `&= \\frac{${latexColor("likelihood", "P(B \\mid A)")}\\,${latexColor("prior", "P(A)")}}{${latexColor("evidence", "P(B)")}} \\\\`,
    `&= \\frac{${latexColor("likelihood", latexNumber(values.likelihood))}\\cdot` +
      `${latexColor("prior", latexNumber(values.prior))}}` +
      `{${latexColor("evidence", latexNumber(values.evidence))}} \\\\`,
    `&= ${latexColor("posterior", latexNumber(posterior))}`,
    "\\end{aligned}",
  ].join("\n")

  const source = [
    "\\[",
    body,
    "\\]",
  ].join("\n")

  return { body, source }
}

function createBayesMath(values: BayesFormulaValues) {
  const posterior = values.evidence > 0 ? values.numerator / values.evidence : Number.NaN
  return mathElement(
    "math",
    { class: "bayes-rendered-math", display: "block" },
    mathElement(
      "mtable",
      {},
      mathRow(
        probabilityAGivenB(),
        fraction(multiply(probabilityBGivenA(), probabilityA()), probabilityB()),
      ),
      mathRow(
        null,
        fraction(
          multiply(mathNumber(values.likelihood, "likelihood"), mathNumber(values.prior, "prior")),
          mathNumber(values.evidence, "evidence"),
        ),
      ),
      mathRow(null, mathNumber(posterior, "posterior")),
    ),
  )
}

function mathRow(left: Element | null, right: Element) {
  return mathElement(
    "mtr",
    {},
    mathElement("mtd", {}, left ?? mathElement("mrow")),
    mathElement("mtd", {}, mathElement("mo", {}, "=")),
    mathElement("mtd", {}, right),
  )
}

function fraction(numerator: Element, denominator: Element) {
  return mathElement("mfrac", {}, numerator, denominator)
}

function multiply(left: Element, right: Element) {
  return mathElement("mrow", {}, left, mathElement("mo", {}, "\u00b7"), right)
}

function probabilityAGivenB() {
  return coloredMath(
    "posterior",
    mathElement("mi", {}, "P"),
    mathElement("mo", {}, "("),
    mathElement("mi", {}, "A"),
    mathElement("mo", {}, "\u2223"),
    mathElement("mi", {}, "B"),
    mathElement("mo", {}, ")"),
  )
}

function probabilityBGivenA() {
  return coloredMath(
    "likelihood",
    mathElement("mi", {}, "P"),
    mathElement("mo", {}, "("),
    mathElement("mi", {}, "B"),
    mathElement("mo", {}, "\u2223"),
    mathElement("mi", {}, "A"),
    mathElement("mo", {}, ")"),
  )
}

function probabilityA() {
  return coloredMath(
    "prior",
    mathElement("mi", {}, "P"),
    mathElement("mo", {}, "("),
    mathElement("mi", {}, "A"),
    mathElement("mo", {}, ")"),
  )
}

function probabilityB() {
  return coloredMath(
    "evidence",
    mathElement("mi", {}, "P"),
    mathElement("mo", {}, "("),
    mathElement("mi", {}, "B"),
    mathElement("mo", {}, ")"),
  )
}

function mathNumber(value: number, role: keyof typeof bayesColors) {
  return coloredMath(role, mathElement(Number.isFinite(value) ? "mn" : "mtext", {}, latexNumber(value)))
}

function coloredMath(role: keyof typeof bayesColors, ...children: Array<Element | string>) {
  const element = mathElement("mrow", { mathcolor: bayesColors[role] }, ...children)
  ;(element as HTMLElement).style.color = bayesColors[role]
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

function createLatexOutput() {
  const element = document.createElement("section")
  element.className = "bayes-latex"

  const heading = document.createElement("div")
  heading.className = "bayes-latex-heading"

  const label = document.createElement("span")
  label.textContent = "LaTeX"

  const copyButton = document.createElement("button")
  copyButton.type = "button"
  copyButton.textContent = "Copy LaTeX"

  const status = document.createElement("span")
  status.className = "bayes-latex-status"
  status.setAttribute("aria-live", "polite")

  heading.append(label, copyButton, status)

  const rendered = document.createElement("div")
  rendered.className = "bayes-latex-rendered"
  rendered.setAttribute("aria-label", "Rendered Bayes theorem equation with current numbers")

  const output = document.createElement("textarea")
  output.className = "bayes-latex-output"
  output.readOnly = true
  output.rows = 8
  output.spellcheck = false
  output.setAttribute("aria-label", "Bayes theorem LaTeX with current numbers")

  element.append(heading, rendered, output)
  return { element, rendered, output, copyButton, status }
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
