import type { JSX } from "preact"
import { jsx, jsxs } from "preact/jsx-runtime"
import type { QuartzComponent, QuartzComponentProps } from "../../quartz/components/types"

type DateValue = Date | string | number

type RecentNotePage = {
  slug?: unknown
  defaultDateType?: string
  dates?: Record<string, DateValue | undefined>
  frontmatter?: {
    title?: string
    tags?: string[] | string
    noindex?: boolean
  }
}

type RecentNotesFilteredOptions = {
  title: string
  limit: number
  prefix: string
  excludeSlug: string
  excludeNoindex: boolean
  linkToMore: string
  showTags: boolean
  collapsible: boolean
  defaultCollapsed: boolean
}

const defaultOptions: RecentNotesFilteredOptions = {
  title: "Recent notes",
  limit: 3,
  prefix: "",
  excludeSlug: "",
  excludeNoindex: false,
  linkToMore: "",
  showTags: true,
  collapsible: true,
  defaultCollapsed: false,
}

const styles = `
.recent-notes {
  min-width: 0;
}

.recent-notes > h3 {
  margin: 0.5rem 0 0 0;
  font-size: 1rem;
}

.recent-notes > summary {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.5rem;
  margin: 0.5rem 0 0 0;
  cursor: pointer;
  list-style: none;
  user-select: none;
}

.recent-notes > summary::-webkit-details-marker {
  display: none;
}

.recent-notes > summary > h3 {
  margin: 0;
  font-size: 1rem;
}

.recent-notes > summary > .fold {
  flex: 0 0 auto;
  transform: rotate(-90deg);
  transition: transform 0.2s ease;
}

.recent-notes[open] > summary > .fold {
  transform: rotate(0deg);
}

.recent-notes > ul.recent-ul,
.recent-notes .recent-ul {
  list-style: none;
  margin-top: 1rem;
  padding-left: 0;
}

.recent-notes .recent-li {
  margin: 1rem 0;
}

.recent-notes .section > .desc > h3 > a {
  background-color: transparent;
}

.recent-notes .section > .meta {
  margin: 0 0 0.5rem 0;
  opacity: 0.6;
}
`

function classNames(...names: Array<string | false | null | undefined>): string {
  return names.filter(Boolean).join(" ")
}

function normalizeSlug(slug: unknown): string {
  return String(slug ?? "")
}

function href(slug: unknown): string {
  const normalized = normalizeSlug(slug)
  return normalized.length > 0 ? `./${normalized}` : "./"
}

function tagHref(tag: string): string {
  const normalized = tag.replace(/^#/, "").trim().toLowerCase().replace(/\s+/g, "-")
  return href(`tags/${normalized}`)
}

function titleFor(page: RecentNotePage): string {
  return page.frontmatter?.title ?? normalizeSlug(page.slug).split("/").pop() ?? "Untitled"
}

function dateFor(page: RecentNotePage): Date | undefined {
  const dateType = page.defaultDateType
  const value =
    (dateType ? page.dates?.[dateType] : undefined) ??
    page.dates?.modified ??
    page.dates?.created ??
    page.dates?.published

  if (!value) return undefined
  const date = value instanceof Date ? value : new Date(value)
  return Number.isNaN(date.valueOf()) ? undefined : date
}

function sortRecent(a: RecentNotePage, b: RecentNotePage): number {
  const aDate = dateFor(a)?.valueOf() ?? 0
  const bDate = dateFor(b)?.valueOf() ?? 0
  if (aDate !== bDate) return bDate - aDate
  return titleFor(a).localeCompare(titleFor(b))
}

function tagsFor(page: RecentNotePage): string[] {
  const tags = page.frontmatter?.tags
  if (Array.isArray(tags)) return tags
  if (typeof tags === "string") return [tags]
  return []
}

function includePage(page: RecentNotePage, opts: RecentNotesFilteredOptions): boolean {
  const slug = normalizeSlug(page.slug)
  if (!slug) return false
  if (opts.prefix && !slug.startsWith(opts.prefix)) return false
  if (opts.excludeSlug && slug === opts.excludeSlug) return false
  if (opts.excludeNoindex && page.frontmatter?.noindex) return false
  return true
}

function FoldIcon(): JSX.Element {
  return jsx("svg", {
    xmlns: "http://www.w3.org/2000/svg",
    width: "14",
    height: "14",
    viewBox: "5 8 14 8",
    fill: "none",
    stroke: "currentColor",
    "stroke-width": "2",
    "stroke-linecap": "round",
    "stroke-linejoin": "round",
    class: "fold",
    "aria-hidden": "true",
    children: jsx("polyline", { points: "6 9 12 15 18 9" }),
  })
}

function renderContent(
  pages: RecentNotePage[],
  opts: RecentNotesFilteredOptions,
  locale: string | undefined,
): JSX.Element {
  const visible = pages.slice(0, opts.limit)
  const remaining = Math.max(0, pages.length - opts.limit)

  return jsxs("div", {
    children: [
      jsx("ul", {
        class: "recent-ul",
        children: visible.map((page) => {
          const date = dateFor(page)
          const tags = tagsFor(page)

          return jsx(
            "li",
            {
              class: "recent-li",
              children: jsxs("div", {
                class: "section",
                children: [
                  jsx("div", {
                    class: "desc",
                    children: jsx("h3", {
                      children: jsx("a", {
                        href: href(page.slug),
                        class: "internal",
                        children: titleFor(page),
                      }),
                    }),
                  }),
                  date &&
                    jsx("p", {
                      class: "meta",
                      children: jsx("time", {
                        datetime: date.toISOString(),
                        children: date.toLocaleDateString(locale ?? "en-US", {
                          year: "numeric",
                          month: "short",
                          day: "2-digit",
                        }),
                      }),
                    }),
                  opts.showTags &&
                    tags.length > 0 &&
                    jsx("ul", {
                      class: "tags",
                      children: tags.map((tag) =>
                        jsx(
                          "li",
                          {
                            children: jsx("a", {
                              class: "internal tag-link",
                              href: tagHref(tag),
                              children: tag,
                            }),
                          },
                          tag,
                        ),
                      ),
                    }),
                ],
              }),
            },
            normalizeSlug(page.slug),
          )
        }),
      }),
      opts.linkToMore &&
        remaining > 0 &&
        jsx("p", {
          children: jsx("a", {
            href: href(opts.linkToMore),
            children: `See ${remaining} more`,
          }),
        }),
    ],
  })
}

export const RecentNotesFiltered = (
  userOpts: Partial<RecentNotesFilteredOptions> = {},
): QuartzComponent => {
  const opts = { ...defaultOptions, ...userOpts }

  const Component = ({ allFiles, fileData, displayClass, cfg }: QuartzComponentProps) => {
    if (normalizeSlug(fileData.slug) !== "index") return null

    const pages = allFiles
      .map((page) => page as RecentNotePage)
      .filter((page) => includePage(page, opts))
      .sort(sortRecent)
    const content = renderContent(pages, opts, cfg.locale)

    if (!opts.collapsible) {
      return jsxs("section", {
        class: classNames(displayClass, "recent-notes"),
        children: [jsx("h3", { children: opts.title }), content],
      })
    }

    return jsxs("details", {
      class: classNames(displayClass, "recent-notes"),
      open: !opts.defaultCollapsed,
      children: [
        jsxs("summary", { children: [jsx("h3", { children: opts.title }), jsx(FoldIcon, {})] }),
        content,
      ],
    })
  }

  Component.css = styles
  return Component
}

export default RecentNotesFiltered
