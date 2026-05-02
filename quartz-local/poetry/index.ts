import type { Code } from "mdast"
import { visit } from "unist-util-visit"
import type { QuartzTransformerPlugin } from "../../quartz/plugins/types"

type HtmlCodeNode = {
  type: "html"
  value: string
}

export const Poetry: QuartzTransformerPlugin = () => ({
  name: "Poetry",
  markdownPlugins() {
    return [
      () => (tree) => {
        visit(tree, "code", (node: Code) => {
          if (node.lang === "poetry") {
            const htmlNode = node as unknown as HtmlCodeNode
            htmlNode.type = "html"
            htmlNode.value = `<pre class="poetry">${node.value}</pre>`
          }
        })
      },
    ]
  },
})

export default Poetry
