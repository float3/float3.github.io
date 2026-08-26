/**
 * The coin flip on the insurance post.
 *
 * Heads takes everything the reader owns, tails pays them a reward, and the
 * three numbers underneath say how much of each. The post's argument is that
 * the expected value of that bet is not the point, so the calculator has to
 * let anyone put their own numbers in and see the expectation come out
 * positive while the bet stays one they would not take.
 *
 * It used to be a `<script>` in the markdown, which is the one place in this
 * repository where code is neither compiled nor checked. It also read
 * `document.querySelectorAll("input")` and listened to every one of them: the
 * search box at the top of the page recalculated the bet on each keystroke.
 * The fields are found inside the calculator now, and it hears only them.
 */

const CALCULATOR = "insurance-calculator"

interface Fields {
  /** Chance of tails, as a percentage. */
  tails: HTMLInputElement
  /** What the reader owns, and stands to lose on heads. */
  wealth: HTMLInputElement
  /** What tails pays. */
  reward: HTMLInputElement
  results: HTMLElement
}

function money(amount: number): string {
  return `€${amount.toLocaleString(undefined, { maximumFractionDigits: 2 })}`
}

function fields(root: HTMLElement): Fields | undefined {
  const tails = root.querySelector<HTMLInputElement>("#tailsProb")
  const wealth = root.querySelector<HTMLInputElement>("#netWorth")
  const reward = root.querySelector<HTMLInputElement>("#reward")
  const results = root.querySelector<HTMLElement>("#results")

  if (tails === null || wealth === null || reward === null || results === null) return undefined
  return { tails, wealth, reward, results }
}

/** One `<b>label</b> value` line, the shape the original wrote by hand. */
function line(label: string, value: string | Node): DocumentFragment {
  const bold = document.createElement("b")
  bold.textContent = label

  const fragment = document.createDocumentFragment()
  fragment.append(bold, " ", value)
  return fragment
}

/** A paragraph of those lines, which is what the blank line between groups was. */
function group(...lines: [label: string, value: string | Node][]): HTMLParagraphElement {
  const paragraph = document.createElement("p")

  for (const [label, value] of lines) {
    if (paragraph.childNodes.length > 0) paragraph.append(document.createElement("br"))
    paragraph.append(line(label, value))
  }

  return paragraph
}

function update(parts: Fields): void {
  const chance = Number(parts.tails.value) / 100
  const wealth = Number(parts.wealth.value)
  const reward = Number(parts.reward.value)

  const onHeads = 0
  const onTails = wealth + reward
  const expected = (1 - chance) * onHeads + chance * onTails
  const gain = expected - wealth

  // Tails at no chance at all can never pay the loss back, whatever it pays.
  const breakEven = chance > 0 ? wealth / chance - wealth : Infinity

  const change = document.createElement("span")
  change.style.color = gain >= 0 ? "green" : "red"
  change.textContent = money(gain)

  parts.results.replaceChildren(
    group(["If Heads:", money(onHeads)], ["If Tails:", money(onTails)]),
    group(["Expected Wealth:", money(expected)], ["Expected Gain/Loss:", change]),
    group(["Break-even Reward:", Number.isFinite(breakEven) ? money(breakEven) : "Impossible"]),
  )
}

function initialise(): void {
  const root = document.getElementById(CALCULATOR)
  if (root === null) return

  const parts = fields(root)
  if (parts === undefined) return

  // Assigned rather than added, and once on the box rather than three times on
  // the fields: `input` bubbles, and a navigation can hand back the same nodes,
  // where `addEventListener` would stack a second handler on each of them.
  root.oninput = () => update(parts)
  update(parts)
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", initialise, { once: true })
} else {
  initialise()
}

document.addEventListener("nav", initialise)
