/**
 * Assembling the little page a comment's code runs as.
 *
 * Two shapes of comment are runnable, and they are told apart by what the
 * author wrote rather than by a flag they have to remember:
 *
 *   - one that contains a `<script>` or `<style>` tag runs *as itself* — the
 *     whole comment becomes the page, markup and all, which is what someone
 *     writing a demo means by writing a demo;
 *   - one that contains fenced `html`, `css`, `js` or `ts` blocks has them
 *     stitched together into a page, which is what someone pasting a snippet
 *     means.
 *
 * Either shape may be written in TypeScript -- a `ts` fence, or a script tag
 * that says `lang="ts"` -- and is compiled here, while the site is built. What
 * reaches the frame is always JavaScript: a comment cannot ask the reader's
 * browser to fetch a compiler, and the sandbox it runs in could not reach one.
 *
 * The result goes into an iframe's `srcdoc` with `sandbox="allow-scripts"` and
 * no `allow-same-origin`, so it runs in an opaque origin: it can do as it likes
 * inside its own box and can reach nothing of this site's — not the DOM, not
 * localStorage, not a cookie. Nothing starts until the reader presses run.
 */

import { transformSync, type TransformFailure } from "esbuild"

/** ```js … ``` and friends, with the language on the fence. */
const FENCE =
  /^[ \t]*(?:```|~~~)[ \t]*([A-Za-z0-9+#-]*)[ \t]*\r?\n([\s\S]*?)^[ \t]*(?:```|~~~)[ \t]*$/gm

type Language = "html" | "css" | "js"

/** A `ts` fence is a `js` one that has to go past esbuild on the way. */
function language(tag: string): Language | "ts" | undefined {
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
    case "ts":
    case "typescript":
    case "mts":
      return "ts"
    default:
      return undefined
  }
}

/**
 * TypeScript with the types taken off, which is the whole of what a browser
 * needs from it.
 *
 * esbuild is already here -- Quartz builds its client bundle and its own parse
 * worker with it -- and it strips types rather than checking them. That is the
 * right trade for a comment: a demo is not a pull request, and nobody writing
 * one should have to satisfy a compiler configured for someone else's project.
 * Code that `tsc` would have refused still runs, exactly as the JavaScript it
 * is.
 *
 * A syntax error is the other kind of wrong, and it cannot be quietly ignored:
 * there is no page to make out of it. It goes into the frame where the author
 * will see it, rather than up as an exception -- a comment arriving with a
 * mistyped fence must not be able to stop the site building.
 */
function compile(source: string): { js: string; error?: string } {
  try {
    return { js: transformSync(source, { loader: "ts" }).code }
  } catch (failure) {
    return { js: "", error: describe(failure) }
  }
}

/** esbuild's complaint, one line per error. */
function describe(failure: unknown): string {
  const errors = (failure as Partial<TransformFailure> | null | undefined)?.errors
  if (!Array.isArray(errors) || errors.length === 0) return String(failure)

  return errors
    .map(({ text, location }) => (location === null ? text : `line ${location.line}: ${text}`))
    .join("\n")
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
.comment-frame-error {
  margin: 0 0 0.75rem;
  padding: 0.5rem 0.6rem;
  border-left: 3px solid light-dark(#b3261e, #f2b8b5);
  color: light-dark(#b3261e, #f2b8b5);
  font: 13px/1.45 ui-monospace, SFMono-Regular, Menlo, monospace;
  white-space: pre-wrap;
}
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

/**
 * Hosts a demo's own code may open a connection to -- `connect-src`, which is
 * `fetch`, `XMLHttpRequest`, `WebSocket` and `EventSource` at once.
 *
 * This is the one channel with no part in *showing* anything, and so the one an
 * attacker's comment reaches for: to map the reader's own network by timing
 * requests at `localhost` and `192.168.x`, to fire a state-changing request at
 * a site the reader is signed in to, or simply to post home whatever it has
 * managed to scrape together. Everything here is here because some comment
 * needs it, and the burden is on the comment to justify a new entry: DOOM
 * fetches its 6.8 MB binary from jsDelivr.
 */
const CONNECT_ALLOW = ["https://cdn.jsdelivr.net"]

/**
 * The frame's own content security policy, carried in the document because a
 * `srcdoc` frame has no response of its own to hang a header on.
 *
 * The sandbox walls the frame off from this site; this walls it off from the
 * reader. Code still runs -- `script-src` keeps `'unsafe-inline'`, since the
 * whole document is inline, and gains `'wasm-unsafe-eval'` so `WebAssembly`
 * still compiles without opening `eval` back up -- and pictures still draw. But
 * the frame can only reach out to the few hosts above, so a comment cannot
 * quietly turn the reader's browser into a network scanner, an exfiltration
 * point, or a cross-site request they never made.
 *
 * `img-src` and friends stay open to `https:` and `data:` so a demo can show
 * what it likes; a determined comment can still trickle a little out through an
 * image URL, and that is the standing price of letting demos load images at
 * all. `connect-src` is where the bulk of the harm would have gone, and it is
 * shut.
 */
const FRAME_CSP = [
  "default-src 'none'",
  "script-src 'unsafe-inline' 'wasm-unsafe-eval'",
  "style-src 'unsafe-inline'",
  "img-src data: blob: https:",
  "media-src data: blob: https:",
  "font-src data: https:",
  `connect-src ${CONNECT_ALLOW.join(" ")}`,
].join("; ")

const escape = (value: string) => value.replace(/&/g, "&amp;").replace(/</g, "&lt;")

/**
 * A script tag holding code that cannot close it early.
 *
 * `</script` ends a script element wherever the HTML parser meets it -- inside
 * a string, inside a comment, inside a template literal, it makes no
 * difference, because the parser is not reading JavaScript. A demo that prints
 * one would have the rest of itself spilled into the page as text. The
 * backslash is invisible to JavaScript in every position the sequence can
 * legally occupy.
 */
function script(js: string): string {
  if (js.trim() === "") return ""
  return `<script>${js.replace(/<\/script/gi, "<\/script")}</script>`
}

function document(body: string, css: string, js: string, errors: string[]): string {
  return [
    "<!doctype html>",
    '<html><head><meta charset="utf-8">',
    `<meta http-equiv="Content-Security-Policy" content="${FRAME_CSP}">`,
    // Relative links in a srcdoc frame resolve against the parent, and a demo
    // navigating this page out from under the reader is never what was wanted.
    '<base target="_blank">',
    `<style>${FRAME_STYLE}${css}</style>`,
    "</head><body>",
    // Above the demo rather than below it: whatever did compile has already
    // drawn something, and the reader should not have to scroll past it to be
    // told that a piece is missing.
    errors.length === 0
      ? ""
      : `<pre class="comment-frame-error">${escape(errors.join("\n\n"))}</pre>`,
    body,
    script(js),
    script(RESIZE_REPORTER),
    "</body></html>",
  ].join("\n")
}

/**
 * A script element, matched the way the HTML parser reads one: everything up to
 * the first `</script` is its content, whatever that content looks like.
 */
const SCRIPT = /<script\b([^>]*)>([\s\S]*?)<\/script\s*>/gi

/** The attributes that say a script is TypeScript, and nothing else does. */
const MARKER = /\s+(lang|type)\s*=\s*(?:"([^"]*)"|'([^']*)'|([^\s>]+))/gi
const TYPESCRIPT_LANG = /^(?:ts|typescript|mts)$/i
const TYPESCRIPT_TYPE = /^(?:text|application)\/(?:x-)?typescript$/i

/**
 * The tag's remaining attributes, or `undefined` if it never claimed to be
 * TypeScript.
 *
 * Only the claim itself is taken out. `type="module"` beside a `lang="ts"` is
 * still a module once it is JavaScript, and a demo that says so means it.
 */
function withoutTypeScript(attributes: string): string | undefined {
  let found = false

  const rest = attributes.replace(
    MARKER,
    (whole: string, name: string, quoted?: string, single?: string, bare?: string) => {
      const value = (quoted ?? single ?? bare ?? "").trim()
      const typescript =
        name.toLowerCase() === "lang" ? TYPESCRIPT_LANG.test(value) : TYPESCRIPT_TYPE.test(value)

      if (!typescript) return whole
      found = true
      return ""
    },
  )

  return found ? rest : undefined
}

/**
 * The same HTML, with every TypeScript script compiled where it stands.
 *
 * A browser runs a `<script>` and ignores a `<script type="text/typescript">`
 * without a word, so a comment written that way is inert today rather than
 * broken; this is what makes it run. Position is kept, because a demo's scripts
 * can depend on running in the order they were written.
 */
function compileScripts(html: string): { html: string; errors: string[] } {
  const errors: string[] = []

  const compiled = html.replace(
    SCRIPT,
    (whole: string, attributes: string, body: string): string => {
      const rest = withoutTypeScript(attributes)
      if (rest === undefined) return whole

      const { js, error } = compile(body)
      if (error !== undefined) {
        errors.push(error)
        return ""
      }

      return `<script${rest}>${js}</script>`
    },
  )

  return { html: compiled, errors }
}

/**
 * The comment as a page, from its already-serialised HTML.
 *
 * The input is the tree *before* sanitising, so the author's `<script>` and
 * `<style>` are still in it and stay where they were written.
 */
export function runnableFromHtml(html: string): string {
  const compiled = compileScripts(html)
  return document(compiled.html, "", "", compiled.errors)
}

/** The comment's fenced code blocks, stitched into a page. */
export function runnableFromFences(source: string): string | undefined {
  const parts: Record<Language, string[]> = { html: [], css: [], js: [] }
  const errors: string[] = []
  let found = false

  for (const match of source.matchAll(FENCE)) {
    const kind = language(match[1] ?? "")
    if (kind === undefined) continue
    found = true

    if (kind !== "ts") {
      parts[kind].push(match[2])
      continue
    }

    // A `ts` fence joins the `js` ones where it was written. Which fence a line
    // arrived in stops mattering the moment the types come off it, and a demo
    // that opens in TypeScript and finishes in JavaScript still runs top to
    // bottom.
    const { js, error } = compile(match[2])
    if (error === undefined) parts.js.push(js)
    else errors.push(error)
  }

  if (!found) return undefined
  return document(parts.html.join("\n"), parts.css.join("\n"), parts.js.join("\n"), errors)
}
