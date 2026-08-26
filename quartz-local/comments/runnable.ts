/**
 * Assembling the little page a comment's code runs as.
 *
 * Two shapes of comment are runnable, and they are told apart by what the
 * author wrote rather than by a flag they have to remember:
 *
 *   - one that contains a `<script>` or `<style>` tag runs *as itself* — the
 *     whole comment becomes the page, markup and all, which is what someone
 *     writing a demo means by writing a demo;
 *   - one that contains fenced `html`, `css` or `js` blocks has them stitched
 *     together into a page, which is what someone pasting a snippet means.
 *
 * The result goes into an iframe's `srcdoc` with `sandbox="allow-scripts"` and
 * no `allow-same-origin`, so it runs in an opaque origin: it can do as it likes
 * inside its own box and can reach nothing of this site's — not the DOM, not
 * localStorage, not a cookie. Nothing starts until the reader presses run.
 */

/** ```js … ``` and friends, with the language on the fence. */
const FENCE =
  /^[ \t]*(?:```|~~~)[ \t]*([A-Za-z0-9+#-]*)[ \t]*\r?\n([\s\S]*?)^[ \t]*(?:```|~~~)[ \t]*$/gm

type Language = "html" | "css" | "js"

function language(tag: string): Language | undefined {
  switch (tag.toLowerCase()) {
    case "html":
    case "htm":
      return "html"
    case "css":
      return "css"
    case "js":
    case "javascript":
    case "mjs":
      return "js"
    default:
      return undefined
  }
}

/**
 * A stylesheet thin enough to stay out of the way, but not so thin that a demo
 * lands as black-on-white in the middle of a dark page. The frame cannot read
 * the site's theme — that is the sandbox working — so it follows the reader's
 * system preference instead, which is usually the same answer.
 */
const FRAME_STYLE = `
:root { color-scheme: light dark; }
* { box-sizing: border-box; }
body {
  font: 15px/1.5 system-ui, sans-serif;
  margin: 0;
  padding: 0.75rem;
}
body > :first-child { margin-top: 0; }
body > :last-child { margin-bottom: 0; }
`

/**
 * Tells the parent how tall the frame's content is, so the box can be the size
 * of the thing in it rather than a fixed 22rem with the demo scrolling inside.
 *
 * It measures the *body*, not `documentElement.scrollHeight`. The root's scroll
 * height is never less than the viewport, so feeding it back as the frame's
 * height is a ratchet: the box can grow and can then never shrink again. The
 * body is `height: auto` with no margins of its own, so its border box is the
 * content and nothing else, and it does not move when the frame is resized
 * around it.
 *
 * `postMessage` is the only channel there is. The frame has no
 * `allow-same-origin`, so it cannot touch `frameElement` or reach the parent
 * document; the parent, for its part, treats what arrives as a number from a
 * stranger and clamps it.
 */
const RESIZE_REPORTER = `
(() => {
  const send = () => {
    parent.postMessage(
      { commentFrameHeight: Math.ceil(document.body.getBoundingClientRect().height) },
      "*",
    )
  }
  new ResizeObserver(send).observe(document.body)
  addEventListener("load", send)
  send()
})()
`

function document(body: string, css: string, js: string): string {
  return [
    "<!doctype html>",
    '<html><head><meta charset="utf-8">',
    // Relative links in a srcdoc frame resolve against the parent, and a demo
    // navigating this page out from under the reader is never what was wanted.
    '<base target="_blank">',
    `<style>${FRAME_STYLE}${css}</style>`,
    "</head><body>",
    body,
    js === "" ? "" : `<script>${js}</script>`,
    `<script>${RESIZE_REPORTER}</script>`,
    "</body></html>",
  ].join("\n")
}

/**
 * The comment as a page, from its already-serialised HTML.
 *
 * The input is the tree *before* sanitising, so the author's `<script>` and
 * `<style>` are still in it and stay where they were written.
 */
export function runnableFromHtml(html: string): string {
  return document(html, "", "")
}

/** The comment's fenced code blocks, stitched into a page. */
export function runnableFromFences(source: string): string | undefined {
  const parts: Record<Language, string[]> = { html: [], css: [], js: [] }
  let found = false

  for (const match of source.matchAll(FENCE)) {
    const kind = language(match[1] ?? "")
    if (kind === undefined) continue
    parts[kind].push(match[2])
    found = true
  }

  if (!found) return undefined
  return document(parts.html.join("\n"), parts.css.join("\n"), parts.js.join("\n"))
}
