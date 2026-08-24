/**
 * A minimal hast-to-HTML serializer.
 *
 * This exists for one narrow job. `remark-rehype` with `allowDangerousHtml`
 * leaves the author's HTML as opaque `raw` strings scattered through the tree,
 * unbalanced wherever a tag opened in one node and closed in another — which is
 * every inline `<b>x</b>`. The way out is to write the tree back to text and let
 * a real HTML parser read it, which is what `hast-util-raw` does; it is not a
 * dependency here, and this is the part of it that is actually needed.
 *
 * It runs only *before* that reparse, which is what keeps it small: at that
 * point the only elements carrying properties are the ones markdown generated,
 * so the handful of names below is the entire vocabulary. Everything the author
 * wrote is still a `raw` string and passes through untouched, `<script>` and
 * all — the sanitiser downstream is what decides which of it survives.
 */

import type { Element, Node, Parent, Root } from "hast"

// Tags that never take a closing tag. Anything else gets one, even when empty.
const VOID = new Set([
  "area",
  "base",
  "br",
  "col",
  "embed",
  "hr",
  "img",
  "input",
  "link",
  "meta",
  "source",
  "track",
  "wbr",
])

/**
 * hast stores properties under DOM names; HTML wants attribute names. Only
 * these two differ among the properties markdown can produce, and the fallback
 * below covers anything a future plugin might add.
 */
const ATTRIBUTE: Record<string, string> = { className: "class", htmlFor: "for" }

const escapeText = (value: string) =>
  value.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;")

const escapeAttribute = (value: string) => value.replace(/&/g, "&amp;").replace(/"/g, "&quot;")

function attributes(node: Element): string {
  const properties = node.properties ?? {}
  let out = ""

  for (const [property, value] of Object.entries(properties)) {
    if (value === undefined || value === null || value === false) continue

    const name =
      ATTRIBUTE[property] ?? property.replace(/[A-Z]/g, (letter) => `-${letter.toLowerCase()}`)

    // `hidden`, `checked` and friends are present or absent, never `="true"`.
    if (value === true) {
      out += ` ${name}`
      continue
    }

    // Space-separated lists — `class` most of all — arrive as arrays.
    const text = Array.isArray(value) ? value.join(" ") : String(value)
    out += ` ${name}="${escapeAttribute(text)}"`
  }

  return out
}

export function serialize(node: Node): string {
  switch (node.type) {
    case "root":
      return (node as Root).children.map(serialize).join("")

    case "element": {
      const element = node as Element
      const open = `<${element.tagName}${attributes(element)}>`
      if (VOID.has(element.tagName)) return open
      return `${open}${element.children.map(serialize).join("")}</${element.tagName}>`
    }

    case "text":
      return escapeText((node as unknown as { value: string }).value)

    // The author's own markup, still exactly as they typed it.
    case "raw":
      return (node as unknown as { value: string }).value

    case "comment":
      return `<!--${(node as unknown as { value: string }).value}-->`

    default: {
      // Nothing else should reach here, but dropping a node's children along
      // with the node itself would lose text silently.
      const children = (node as Parent).children
      return Array.isArray(children) ? children.map(serialize).join("") : ""
    }
  }
}

/**
 * Only the author's own HTML, with the markdown-generated prose left out.
 *
 * This is the split between what a runnable comment shows on the page and what
 * it shows in its frame: prose written as markdown stays on the page, and the
 * markup written as HTML — which is the thing being built — goes in the frame.
 * Serialising the whole comment into the frame instead renders the prose twice,
 * once on the page and again inside the box.
 */
export function serializeRaw(node: Node): string {
  if (node.type === "raw") return (node as unknown as { value: string }).value

  const children = (node as Parent).children
  return Array.isArray(children) ? children.map(serializeRaw).join("") : ""
}

/** The same tree with the author's HTML taken out, leaving the prose. */
export function withoutRaw<T extends Node>(node: T): T {
  const children = (node as unknown as Parent).children
  if (!Array.isArray(children)) return node

  return {
    ...node,
    children: children.filter((child) => child.type !== "raw").map(withoutRaw),
  }
}
