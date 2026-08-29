/**
 * The policy every page carries, as a `<meta http-equiv>`.
 *
 * GitHub Pages serves static files and no headers of its own, so the document
 * is the only place a policy can be hung. That costs the three directives a
 * meta element may not carry — `frame-ancestors`, `report-uri` and `sandbox`,
 * all of which are ignored there — and leaves the rest, which is most of it.
 *
 * `script-src` keeps `'unsafe-inline'`, because Quartz bootstraps each page
 * from half a dozen inline scripts it writes itself. So this is not a policy
 * that stops an injected `<script>alert(1)</script>`, and it is not pretending
 * to be one. What it stops is everything around that: a script pulled in from
 * somewhere else, a page reparented under a new `<base>`, a form posted off to
 * a host of somebody's choosing, an `<object>`, and — the one that matters most
 * here — a connection opened to anywhere but the two CDNs that are supposed to
 * get one. The comment system deliberately allows a wide span of HTML and
 * arbitrary inline CSS on the theory that a person reads every comment before
 * it merges; this is what stands behind that theory when the reading misses
 * something.
 *
 * A comment's own demo frame inherits this. `srcdoc` is a local scheme, so the
 * browser hands it the parent's policy on top of the one `runnable.ts` writes
 * into the frame, and the two intersect. That is why `connect-src` names
 * jsDelivr and `script-src` keeps `'wasm-unsafe-eval'` even though nothing in
 * this document needs either: narrowing them here would narrow them in there,
 * and a demo that fetches its own wasm would stop working with no obvious
 * reason why.
 */

/** Where the page's own code comes from, plus what the demo frames need. */
const SCRIPTS = ["'self'", "'unsafe-inline'", "'wasm-unsafe-eval'"]

/** The two CDNs: katex and d3 and pixi from one, mermaid from the other. */
const CDN = ["https://cdn.jsdelivr.net", "https://cdnjs.cloudflare.com"]

/** What `you.ts` asks for the reader's own address. */
const IP_LOOKUP = ["https://api.ip.sb", "https://api.ipify.org", "https://api64.ipify.org"]

/** Embeds that live in the writing itself. */
const EMBEDS = ["https://www.shadertoy.com", "https://graphtoy.com", "https://www.youtube.com"]

/**
 * Pictures, video and fonts stay open to `https:`.
 *
 * A comment may link an image from anywhere, which has always been true and is
 * the price of letting comments have pictures at all; narrowing it here would
 * break the ones already published. `connect-src` is where the bulk of the harm
 * would have gone, and that one is shut.
 */
const ANY_MEDIA = ["'self'", "data:", "blob:", "https:"]

/**
 * The two things a development build needs and a published one must not have.
 *
 * webpack's default devtool in development mode is `eval`, so every module in
 * `content/js` arrives wrapped in one and the page cannot run at all under a
 * policy without `'unsafe-eval'`. In production the devtool is off and the
 * bundles contain no eval, which is the only reason it is safe to forbid: the
 * strict policy is the one that ships, and it is enforced by the same builds
 * that serve the site rather than trusted to hold.
 *
 * The other is the websocket `--serve` reloads the page over.
 */
function development(wsPort: number): { script: string[]; connect: string[] } {
  return {
    script: ["'unsafe-eval'"],
    connect: [`ws://localhost:${wsPort}`, `ws://127.0.0.1:${wsPort}`],
  }
}

export function contentSecurityPolicy(argv: { serve: boolean; wsPort: number }): string {
  const extra = argv.serve ? development(argv.wsPort) : { script: [], connect: [] }

  return [
    "default-src 'self'",
    "base-uri 'self'",
    "object-src 'none'",
    "form-action 'self'",
    `script-src ${[...SCRIPTS, ...extra.script, ...CDN].join(" ")}`,
    `style-src 'self' 'unsafe-inline' https://fonts.googleapis.com ${CDN.join(" ")}`,
    `font-src ${ANY_MEDIA.join(" ")} https://fonts.gstatic.com`,
    `img-src ${ANY_MEDIA.join(" ")}`,
    `media-src ${ANY_MEDIA.join(" ")}`,
    `connect-src ${["'self'", "data:", "blob:", ...extra.connect, ...CDN, ...IP_LOOKUP].join(" ")}`,
    `frame-src 'self' blob: ${EMBEDS.join(" ")}`,
    "worker-src 'self' blob:",
  ].join("; ")
}
