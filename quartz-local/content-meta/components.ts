import { h } from "preact"
import type { ComponentChildren } from "preact"
import type {
  QuartzComponent,
  QuartzComponentConstructor,
  QuartzComponentProps,
} from "../../quartz/components/types"
import { classNames, formatDate } from "@quartz-community/utils"
import { i18n } from "../../quartz/i18n"

const styles = `
.content-meta {
  align-items: baseline;
  color: var(--darkgray);
  column-gap: 0.5em;
  display: flex;
  flex-wrap: wrap;
  font-size: 0.85rem;
  letter-spacing: 0.01em;
  line-height: 1.5;
  margin-top: 0;
  row-gap: 0.1em;
}

/* A dot between neighbours instead of commas, so a wrapped line never starts
   with dangling punctuation. Sits on the wrapper span, never on the link inside. */
.content-meta > span + span::before {
  color: var(--gray);
  content: "\\00b7";
  margin-right: 0.5em;
}

/* Links here are metadata, not prose: no bold, no accent colour until hovered. */
.content-meta a {
  color: inherit;
  font-weight: inherit;
  text-decoration: underline;
  text-decoration-color: var(--gray);
  text-underline-offset: 0.2em;
}

.content-meta a:hover {
  color: var(--secondary);
  text-decoration-color: currentColor;
}

.content-meta .uncommitted {
  color: var(--gray);
  font-style: italic;
}
`

const sameDay = (a: Date, b: Date) => a.toDateString() === b.toDateString()

const WORDS_PER_MINUTE = 200

// Latin-script runs count as one word each; CJK has no spaces, so every character
// counts on its own. Both are read at roughly the same rate per "word".
const CJK = /[぀-ヿ㐀-䶿一-鿿豈-﫿가-힯]/gu
const WORD = /[^\s぀-ヿ㐀-䶿一-鿿豈-﫿가-힯]+/gu

function countWords(text: string): number {
  return (text.match(CJK)?.length ?? 0) + (text.match(WORD)?.length ?? 0)
}

export const ContentMeta: QuartzComponentConstructor = () => {
  const Component: QuartzComponent = ({ cfg, fileData, displayClass }: QuartzComponentProps) => {
    const segments: ComponentChildren[] = []

    const time = (key: string, label: string, date: Date) =>
      h("time", { key, datetime: date.toISOString() }, `${label} ${formatDate(date, cfg.locale)}`)

    const created = fileData.dates?.created
    const modified = fileData.dates?.modified

    if (created) {
      segments.push(time("created", "created", created))
    }

    // Only worth showing once the page has actually moved on from its first commit.
    if (modified && (!created || !sameDay(created, modified))) {
      segments.push(time("modified", "updated", modified))
    }

    // Derived from git by the GitHistory transformer, counting only my own commits.
    // No URL alongside a zero count means the file has no commits at all; a URL with
    // a zero count means only the bot has ever touched it, which is worth no mention.
    const versions = fileData.versions
    const history = fileData.historyUrl
    if (versions === 0 && history === undefined) {
      segments.push(h("span", { key: "versions", class: "uncommitted" }, "uncommitted"))
    } else if (typeof versions === "number" && versions > 0) {
      const label = `${versions} version${versions === 1 ? "" : "s"}`
      segments.push(
        history
          ? h(
              "a",
              {
                key: "versions",
                href: history,
                title: "commit history on GitHub",
                target: "_blank",
                rel: "noopener noreferrer",
              },
              label,
            )
          : h("span", { key: "versions" }, label),
      )
    }

    // `text` is the plain-text body extracted by the Description transformer.
    const text = fileData.text as string | undefined
    const words = text ? countWords(text) : 0
    if (words > 0) {
      const minutes = Math.max(1, Math.round(words / WORDS_PER_MINUTE))
      // Plain text: the wrapper span added below is element enough.
      segments.push(i18n(cfg.locale).components.contentMeta.readingTime({ minutes }))
    }

    if (segments.length === 0) return null

    // Each segment gets a wrapper to hang the separator off: a `::before` on the
    // version link itself would draw the dot inside the link, underlined and
    // clickable. The wrapper also keeps a dot from wrapping away from its segment.
    return h(
      "p",
      { class: classNames(displayClass, "content-meta") },
      segments.map((segment, index) => h("span", { key: `segment-${index}` }, segment)),
    )
  }

  Component.css = styles
  return Component
}

export default ContentMeta
export function init(): void {}
