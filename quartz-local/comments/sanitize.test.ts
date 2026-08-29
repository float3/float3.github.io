import test, { describe } from "node:test"
import assert from "node:assert"
import { fromHtml } from "hast-util-from-html"
import type { Element, ElementContent, Root } from "hast"
import { sanitize, decodeCss } from "./sanitize"

const clean = (html: string): Root => sanitize(fromHtml(html, { fragment: true }))

/** Every element in the tree, so a test can ask what survived. */
function elements(nodes: ElementContent[]): Element[] {
  const found: Element[] = []
  for (const node of nodes) {
    if (node.type !== "element") continue
    found.push(node)
    found.push(...elements(node.children))
  }
  return found
}

const only = (html: string): Element => {
  const all = elements(clean(html).children as ElementContent[])
  assert.strictEqual(all.length, 1)
  return all[0]
}

/** The `style` a declaration is left with, or undefined if it was all dropped. */
const styleOf = (declaration: string): unknown =>
  only(`<span style="${declaration.replace(/"/g, "&quot;")}">x</span>`).properties.style

describe("what a comment may put in the page", () => {
  test("an event handler does not survive the attribute allowlist", () => {
    const img = only(`<img src="https://x/y.png" onerror="alert(1)">`)
    assert.strictEqual(img.properties.onerror, undefined)
    assert.strictEqual(only(`<a href="javascript:alert(1)">x</a>`).properties.href, undefined)
    assert.strictEqual(only(`<a href="https://x/y">x</a>`).properties.href, "https://x/y")
  })

  test("a script goes to the sandbox rather than into the document", () => {
    const kept = elements(
      clean(`<p>before</p><script>alert(1)</script><p>after</p>`).children as ElementContent[],
    )
    assert.deepStrictEqual(
      kept.map((node) => node.tagName),
      ["p", "p"],
    )
  })

  test("styling a word is still allowed", () => {
    assert.strictEqual(styleOf("color: red; font-weight: bold"), "color: red; font-weight: bold")
  })

  test("a declaration that fetches is dropped however it is spelled", () => {
    for (const declaration of [
      "background: url(https://elsewhere.example/pixel)",
      // The bypass this test exists for: an identifier may spell any of its
      // characters as an escape, and `\75 rl(` is `url(` to every browser.
      "background: \\75 rl(https://elsewhere.example/pixel)",
      "background: \\000075rl(https://elsewhere.example/pixel)",
      "background: u\\72 l(https://elsewhere.example/pixel)",
      "background: URL(https://elsewhere.example/pixel)",
      "background-image: image-set('https://elsewhere.example/pixel' 1x)",
      "background-image: -webkit-image-set('https://elsewhere.example/pixel' 1x)",
      "background: cross-fade(url(https://elsewhere.example/a), 50%)",
      "background: element(#x)",
      "width: expression(alert(1))",
      "position: fixed",
      "posi/* nothing to see */tion: fixed",
    ]) {
      assert.strictEqual(styleOf(declaration), undefined, declaration)
    }
  })

  test("dropping the bad declaration keeps the good ones", () => {
    assert.strictEqual(
      styleOf("color: red; background: \\75 rl(https://x/p); font-style: italic"),
      "color: red; font-style: italic",
    )
  })

  test("the escape decoder reads what the browser reads", () => {
    assert.strictEqual(decodeCss("\\75 rl(x)"), "url(x)")
    assert.strictEqual(decodeCss("\\000075rl(x)"), "url(x)")
    assert.strictEqual(decodeCss("u\\72 l(x)"), "url(x)")
    assert.strictEqual(decodeCss("posi/*hide*/tion: fixed"), "position: fixed")
    // A code point that is not one comes back as a placeholder, not a crash.
    assert.doesNotThrow(() => decodeCss("\\ffffff x"))
  })

  test("a link leaves with the rel and target it is given, not its own", () => {
    const link = only(`<a href="https://x/y" target="_self" rel="dofollow">x</a>`)
    assert.deepStrictEqual(link.properties.rel, ["nofollow", "ugc", "noopener", "noreferrer"])
    assert.strictEqual(link.properties.target, "_blank")
  })
})
