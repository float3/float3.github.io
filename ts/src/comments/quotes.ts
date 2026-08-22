/**
 * Tying a comment's quote back to the passage it quotes.
 *
 * The comment file stores the quoted text verbatim rather than an offset or a
 * generated anchor id, because the page is edited and any position-based
 * reference would rot silently on the first rewrite. Finding the text again at
 * read time costs a pass over the article and fails loudly instead: if the
 * sentence is gone, the quote is still shown in the thread and simply stops
 * claiming to point anywhere.
 */

/** A character in the article, mapped back to where it came from. */
interface Position {
  node: Text
  offset: number
}

interface FlatText {
  /** Whitespace-collapsed text of the whole article. */
  text: string
  /** `positions[i]` is where `text[i]` lives in the DOM. */
  positions: Position[]
}

/**
 * Collapses the article's text nodes into one searchable string.
 *
 * Whitespace is normalised because markdown wraps lines wherever it likes: the
 * quote was captured from a rendered selection, and the same sentence can be
 * one line in the source and three in the DOM.
 */
function flatten(root: Element): FlatText {
  const walker = document.createTreeWalker(root, NodeFilter.SHOW_TEXT, {
    acceptNode(node) {
      const parent = node.parentElement
      if (parent === null) return NodeFilter.FILTER_REJECT
      // The thread quotes the article; letting it match itself would mark the
      // quote inside the comment and link the passage to nothing.
      if (parent.closest(".comments, script, style, code, pre") !== null) {
        return NodeFilter.FILTER_REJECT
      }
      return NodeFilter.FILTER_ACCEPT
    },
  })

  let text = ""
  const positions: Position[] = []
  let lastWasSpace = true

  for (let node = walker.nextNode(); node !== null; node = walker.nextNode()) {
    const value = (node as Text).data
    for (let offset = 0; offset < value.length; offset++) {
      const character = value[offset]
      if (/\s/.test(character)) {
        if (lastWasSpace) continue
        lastWasSpace = true
        text += " "
      } else {
        lastWasSpace = false
        text += character
      }
      positions.push({ node: node as Text, offset })
    }
  }

  return { text, positions }
}

const normalise = (value: string) => value.replace(/\s+/g, " ").trim()

/**
 * Wraps one run of the article in a `<mark>`.
 *
 * A quote regularly spans element boundaries — half of an emphasised phrase,
 * the end of one paragraph — so this splits the run per text node rather than
 * trying to wrap a Range, which throws the moment the range is not balanced.
 */
function mark(
  flat: FlatText,
  start: number,
  end: number,
  id: string,
): { first: HTMLElement; last: HTMLElement } | undefined {
  const runs = new Map<Text, { from: number; to: number }>()
  for (let index = start; index < end; index++) {
    const { node, offset } = flat.positions[index]
    const run = runs.get(node)
    if (run === undefined) runs.set(node, { from: offset, to: offset + 1 })
    else run.to = offset + 1
  }

  let first: HTMLElement | undefined
  let last: HTMLElement | undefined
  // Splitting a text node invalidates the positions that point past the split,
  // so the runs are applied back to front and each node is touched once. That
  // reversal is why the first element built is the one furthest down the page.
  for (const [node, run] of [...runs].reverse()) {
    const target = run.from > 0 ? node.splitText(run.from) : node
    if (target.data.length > run.to - run.from) target.splitText(run.to - run.from)

    const element = document.createElement("mark")
    element.className = "comment-mark"
    target.replaceWith(element)
    element.append(target)
    first = element
    last ??= element
  }

  if (first === undefined || last === undefined) return undefined
  // The scroll target goes on the start of the passage and the backlink after
  // its end, so a quote spanning an emphasised word does not end up with an
  // arrow wedged into the middle of the sentence.
  first.id = id
  return { first, last }
}

/** Where in the flattened text the search should start, given a heading hint. */
function searchFrom(flat: FlatText, root: Element, heading: string | undefined): number {
  if (heading === undefined) return 0
  const target = root.querySelector(`#${CSS.escape(heading)}`)
  if (target === null) return 0

  // The heading's own text is in the flattened string; find where it starts and
  // search from there, so a phrase repeated in two sections lands in the right one.
  for (let index = 0; index < flat.positions.length; index++) {
    if (target.contains(flat.positions[index].node)) return index
  }
  return 0
}

/**
 * Marks every quoted passage and cross-links it with its comment.
 *
 * Returns the ids of the comments whose quote was actually located, so the
 * thread can enable only the links that lead somewhere.
 */
export function linkQuotes(root: Element, comments: HTMLElement[]): Set<string> {
  const located = new Set<string>()
  // Recomputed after each mark: splitting text nodes moves everything after it.
  let flat = flatten(root)

  for (const comment of comments) {
    const id = comment.dataset.commentId
    const quote = comment.dataset.quote
    if (id === undefined || quote === undefined) continue

    const needle = normalise(quote)
    if (needle === "") continue

    const from = searchFrom(flat, root, comment.dataset.quoteHeading)
    let at = flat.text.indexOf(needle, from)
    // The heading hint is a hint: a passage that has since moved to another
    // section is still the passage that was quoted.
    if (at === -1 && from > 0) at = flat.text.indexOf(needle)
    if (at === -1) continue

    const marked = mark(flat, at, at + needle.length, `quote-${id}`)
    if (marked === undefined) continue

    const backlink = document.createElement("a")
    backlink.className = "comment-backlink"
    backlink.href = `#comment-${id}`
    backlink.textContent = "¶"
    backlink.title = "a comment quotes this"
    marked.last.after(backlink)

    located.add(id)
    flat = flatten(root)
  }

  return located
}

/** Undoes `linkQuotes`, so a re-run does not nest marks inside marks. */
export function unlinkQuotes(root: Element): void {
  for (const backlink of root.querySelectorAll("a.comment-backlink")) backlink.remove()
  for (const marked of root.querySelectorAll("mark.comment-mark")) {
    marked.replaceWith(...marked.childNodes)
  }
  // Splitting left the article full of adjacent text nodes; rejoin them so the
  // next flatten sees the same shape the page was built with.
  root.normalize()
}

export interface CapturedQuote {
  text: string
  heading?: string
}

/** The selected text, plus the nearest heading above it to disambiguate later. */
export function captureSelection(root: Element): CapturedQuote | undefined {
  const selection = window.getSelection()
  if (selection === null || selection.isCollapsed || selection.rangeCount === 0) return undefined

  const range = selection.getRangeAt(0)
  const container =
    range.commonAncestorContainer instanceof Element
      ? range.commonAncestorContainer
      : range.commonAncestorContainer.parentElement
  if (container === null || !root.contains(container)) return undefined
  // Quoting the thread back at itself is never what was meant.
  if (container.closest(".comments") !== null) return undefined

  const text = normalise(selection.toString())
  if (text.length < 2) return undefined

  return { text, heading: nearestHeading(container, root) }
}

function nearestHeading(from: Element, root: Element): string | undefined {
  // Walk backwards through the document from the selection until a heading with
  // an id turns up — that is the section the reader was looking at.
  const headings = [...root.querySelectorAll("h1[id], h2[id], h3[id], h4[id], h5[id], h6[id]")]
  let found: string | undefined
  for (const heading of headings) {
    const position = heading.compareDocumentPosition(from)
    if ((position & Node.DOCUMENT_POSITION_FOLLOWING) !== 0) found = heading.id
    else break
  }
  return found
}
