/**
 * What a comment is allowed to put directly into the page.
 *
 * Comments may contain HTML, and may contain scripts, and both are wanted —
 * but only one of them belongs in this document. Markup renders inline, with a
 * wide allowlist: a comment can lay out a table, draw a details/summary, style
 * its own text. Anything that *executes* is pulled out here and handed to
 * `runnable.ts`, which puts it in a sandboxed frame the reader has to press a
 * button to start.
 *
 * That split is the whole security model, and it is not about distrusting the
 * commenter — every comment is read before it merges. It is that a merge is a
 * judgement made once, by eye, on code that may be minified or clever, while
 * the consequence of getting it wrong is script running on this origin for
 * every visitor afterwards. The sandbox makes a mistaken merge cost a silly
 * iframe rather than the site's cookies.
 */

import type { Element, ElementContent, Root } from "hast"

/**
 * Tags that render inline, mapped to the attributes each may keep.
 *
 * Everything gets `ALWAYS` on top. A tag not listed here is unwrapped rather
 * than deleted, so an unrecognised element costs its markup and not its text.
 */
const ALLOWED: Record<string, readonly string[]> = {
  // Text
  p: [],
  br: [],
  hr: [],
  span: [],
  div: [],
  em: [],
  strong: [],
  b: [],
  i: [],
  u: [],
  s: [],
  del: [],
  ins: [],
  mark: [],
  small: [],
  sub: [],
  sup: [],
  abbr: ["title"],
  cite: [],
  q: ["cite"],
  kbd: [],
  samp: [],
  var: [],
  time: ["datetime"],
  code: [],
  pre: [],
  blockquote: ["cite"],

  // Structure
  section: [],
  article: [],
  aside: [],
  header: [],
  footer: [],
  figure: [],
  figcaption: [],
  details: ["open"],
  summary: [],
  h4: [],
  h5: [],
  h6: [],

  // Lists
  ul: [],
  ol: ["start", "reversed", "type"],
  li: ["value"],
  dl: [],
  dt: [],
  dd: [],

  // Tables
  table: [],
  caption: [],
  colgroup: ["span"],
  col: ["span"],
  thead: [],
  tbody: [],
  tfoot: [],
  tr: [],
  th: ["colspan", "rowspan", "scope", "abbr"],
  td: ["colspan", "rowspan"],

  // Media
  a: ["href", "title"],
  img: ["src", "alt", "title", "width", "height"],
  picture: [],
  source: ["src", "srcset", "type", "media", "sizes"],
  video: ["src", "poster", "controls", "loop", "muted", "playsinline", "width", "height"],
  audio: ["src", "controls", "loop"],
  track: ["src", "kind", "srclang", "label"],
}

/** Cosmetic, and safe on any of the above. */
const ALWAYS = ["class", "style", "title", "lang", "dir"]

/**
 * Executable in this document, so it goes to the sandbox instead.
 *
 * These are not stripped for being dangerous in themselves — a `<script>` is
 * the point of the feature — but because inline is the wrong place to put them.
 */
export const EXECUTABLE = new Set(["script", "style", "iframe", "object", "embed", "form"])

// Headings inside a comment are demoted rather than dropped: a comment sits
// under the page's own headings and must not compete with them in the outline
// (or in the table of contents, which reads the document's heading levels).
const DEMOTE: Record<string, string> = { h1: "h4", h2: "h5", h3: "h6" }

const SAFE_LINK = /^(https?:|mailto:|#|\/)/i
// No `data:` here. A data URL is a payload the reviewer cannot read in a diff,
// and there is nowhere in a comment that needs one.
const SAFE_MEDIA = /^https?:/i

/**
 * Inline CSS is allowed, because a comment that wants to colour a word should
 * be able to. These few declarations are not styling, though — they are ways to
 * leave the comment's own box, phone home, or execute.
 */
const FORBIDDEN_CSS =
  /(position\s*:\s*(fixed|sticky|absolute)|url\s*\(|expression\s*\(|@import|behaviou?r\s*:|-moz-binding)/i

function cleanStyle(value: string): string | undefined {
  const kept = value
    .split(";")
    .map((declaration) => declaration.trim())
    .filter((declaration) => declaration !== "" && !FORBIDDEN_CSS.test(declaration))
  return kept.length > 0 ? kept.join("; ") : undefined
}

function cleanValue(
  property: string,
  value: unknown,
): string | number | boolean | (string | number)[] | undefined {
  if (property === "style") {
    return typeof value === "string" ? cleanStyle(value) : undefined
  }

  if (property === "href") {
    return typeof value === "string" && SAFE_LINK.test(value) ? value : undefined
  }

  if (property === "src" || property === "srcset" || property === "poster") {
    return typeof value === "string" && SAFE_MEDIA.test(value) ? value : undefined
  }

  if (typeof value === "string" || typeof value === "number" || typeof value === "boolean") {
    return value
  }
  return Array.isArray(value) ? (value as (string | number)[]) : undefined
}

function cleanElement(node: Element): ElementContent[] {
  // Executable elements are removed whole — children included. Keeping the text
  // of a script would spill its source into the middle of the prose.
  if (EXECUTABLE.has(node.tagName)) return []

  const children = cleanChildren(node.children)

  const demoted = DEMOTE[node.tagName]
  const tagName = demoted ?? node.tagName
  const allowed = ALLOWED[tagName]

  // Unknown tag: keep what it said, drop the fact that it said it.
  if (allowed === undefined) return children

  const properties: Element["properties"] = {}
  for (const property of [...allowed, ...ALWAYS]) {
    const raw = node.properties?.[property]
    if (raw === undefined || raw === null) continue
    const value = cleanValue(property, raw)
    if (value === undefined) continue
    properties[property] = value
  }

  // Every link leaves the site as far as this page is concerned, and a comment
  // author does not get to choose how the reader's browser treats the referrer.
  if (tagName === "a") {
    properties.rel = ["nofollow", "ugc", "noopener", "noreferrer"]
    properties.target = "_blank"
  }
  if (tagName === "img" || tagName === "video" || tagName === "audio") {
    properties.loading = "lazy"
    properties.referrerpolicy = "no-referrer"
  }

  return [{ type: "element", tagName, properties, children }]
}

function cleanChildren(children: ElementContent[]): ElementContent[] {
  const cleaned: ElementContent[] = []
  for (const child of children) {
    if (child.type === "element") {
      cleaned.push(...cleanElement(child))
    } else if (child.type === "text") {
      cleaned.push(child)
    }
    // `raw`, `comment` and `doctype` nodes are dropped. By this point the tree
    // has been through a real HTML parser, so a `raw` node cannot occur — and
    // if one did it would be unparsed markup, which is precisely the thing not
    // to hand back to the renderer.
  }
  return cleaned
}

export function sanitize(tree: Root): Root {
  return { type: "root", children: cleanChildren(tree.children as ElementContent[]) }
}

/** True if the tree carries anything that would want to run. */
export function hasExecutable(tree: Root): boolean {
  let found = false
  const visit = (nodes: ElementContent[]) => {
    for (const node of nodes) {
      if (found || node.type !== "element") continue
      if (EXECUTABLE.has(node.tagName)) {
        found = true
        return
      }
      visit(node.children)
    }
  }
  visit(tree.children as ElementContent[])
  return found
}
