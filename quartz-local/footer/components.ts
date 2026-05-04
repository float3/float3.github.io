import { h } from "preact"
import type {
  QuartzComponent,
  QuartzComponentConstructor,
  QuartzComponentProps,
} from "../../quartz/components/types"

type FooterOptions = {
  links?: Record<string, string>
}

const styles = `
footer {
  text-align: left;
  margin-bottom: 4rem;
  opacity: 0.7;
}

footer ul {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: row;
  gap: 1rem;
}
`

export const Footer: QuartzComponentConstructor<FooterOptions> = (opts) => {
  const Component: QuartzComponent = ({ displayClass }: QuartzComponentProps) => {
    const links = Object.entries(opts?.links ?? {})
    if (links.length === 0) return null

    return h(
      "footer",
      { class: displayClass ?? "" },
      h(
        "ul",
        null,
        links.map(([text, link]) => h("li", { key: text }, h("a", { href: link }, text))),
      ),
    )
  }

  Component.css = styles
  return Component
}

export default Footer
