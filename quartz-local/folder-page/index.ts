import path from "path"
import { h } from "preact"
import { Fragment, jsx, jsxs } from "preact/jsx-runtime"
import { toJsxRuntime } from "hast-util-to-jsx-runtime"
import type {
  QuartzComponent,
  QuartzComponentConstructor,
  QuartzComponentProps,
} from "../../quartz/components/types"
import type { QuartzPageTypePlugin, VirtualPage } from "../../quartz/plugins/types"
import type { QuartzPluginData } from "../../quartz/plugins/vfile"
import { type FullSlug, isFolderPath, joinSegments, resolveRelative } from "../../quartz/util/path.ts"
import type { Root } from "hast"

type SortFn = (f1: PageEntry, f2: PageEntry) => number

type FolderContentOptions = {
  showFolderCount: boolean
  showSubfolders: boolean
  sort?: SortFn
}

export type FolderPageOptions = Partial<FolderContentOptions> & {
  prefixFolders?: boolean
}

type TrieNode = {
  isFolder: boolean
  children: TrieNode[]
  data: unknown
  slug: string
  displayName: string
  findNode(path: string[]): TrieNode | undefined
}

type PageEntry = QuartzPluginData & Record<string, unknown>

const defaultOptions: FolderContentOptions = {
  showFolderCount: true,
  showSubfolders: true,
}

const styles = `
ul.section-ul {
  list-style: none;
  margin-top: 2em;
  padding-left: 0;
}

li.section-li {
  margin-bottom: 1em;
}

li.section-li > .section {
  display: grid;
  grid-template-columns: fit-content(8em) 3fr 1fr;
}

@media all and (max-width: 600px) {
  li.section-li > .section > .tags {
    display: none;
  }
}

li.section-li > .section > .desc > h3 > a {
  background-color: transparent;
}

li.section-li > .section .meta {
  margin: 0 1em 0 0;
  opacity: 0.6;
}

.popover .section {
  grid-template-columns: fit-content(8em) 1fr !important;
}

.popover .section > .tags {
  display: none;
}
`

function concatenateResources(
  ...resources: Array<string | string[] | undefined>
): string | string[] | undefined {
  const result = resources.filter((resource): resource is string | string[] => resource !== undefined).flat()
  return result.length === 0 ? undefined : result
}

function folderLabel(): string {
  return "Folder"
}

function itemsUnderFolder(count: number): string {
  return count === 1 ? "1 item under this folder." : `${count} items under this folder.`
}

function htmlToJsx(tree: Root) {
  return toJsxRuntime(tree, {
    Fragment,
    jsx,
    jsxs,
    elementAttributeNameCase: "html",
  })
}

function mostRecentDatesFromChildren(children: TrieNode[]): PageEntry["dates"] {
  let maybeDates: PageEntry["dates"] | undefined
  for (const child of children) {
    const childDates = (child.data as { dates?: PageEntry["dates"] } | null)?.dates
    if (!childDates) continue

    if (!maybeDates) {
      maybeDates = { ...childDates }
      continue
    }

    if (childDates.created > maybeDates.created) maybeDates.created = childDates.created
    if (childDates.modified > maybeDates.modified) maybeDates.modified = childDates.modified
    if (childDates.published > maybeDates.published) maybeDates.published = childDates.published
  }
  return maybeDates ?? { created: new Date(), modified: new Date(), published: new Date() }
}

function mostRecentDatesFromEntries(entries: PageEntry[]): PageEntry["dates"] {
  let maybeDates: PageEntry["dates"] | undefined
  for (const entry of entries) {
    if (!entry.dates) continue

    if (!maybeDates) {
      maybeDates = { ...entry.dates }
      continue
    }

    if (entry.dates.created > maybeDates.created) maybeDates.created = entry.dates.created
    if (entry.dates.modified > maybeDates.modified) maybeDates.modified = entry.dates.modified
    if (entry.dates.published > maybeDates.published) maybeDates.published = entry.dates.published
  }
  return maybeDates ?? { created: new Date(), modified: new Date(), published: new Date() }
}

function pagesFromTrie(folder: TrieNode, showSubfolders: boolean): PageEntry[] {
  return folder.children
    .map((node) => {
      const nodeData = node.data as PageEntry | null
      if (nodeData) {
        if (nodeData.unlisted === true) return undefined
        return nodeData
      }

      if (node.isFolder && showSubfolders) {
        return {
          slug: node.slug as FullSlug,
          dates: mostRecentDatesFromChildren(node.children),
          frontmatter: { title: node.displayName, tags: [] },
        }
      }

      return undefined
    })
    .filter((page): page is PageEntry => page !== undefined)
}

function pagesFromAllFiles(
  allFiles: unknown[],
  folderSlug: string,
  showSubfolders: boolean,
): PageEntry[] {
  const folderPrefix = folderSlug.endsWith("/index")
    ? folderSlug.slice(0, -"index".length)
    : folderSlug.endsWith("/")
      ? folderSlug
      : `${folderSlug}/`

  const directChildren: PageEntry[] = []
  const subfolderFiles = new Map<string, PageEntry[]>()

  for (const file of allFiles as PageEntry[]) {
    if (file.unlisted === true) continue

    const fileSlug = file.slug
    if (!fileSlug || !fileSlug.startsWith(folderPrefix)) continue

    const relativePath = fileSlug.slice(folderPrefix.length)
    if (!relativePath || relativePath === "index") continue

    const segments = relativePath.split("/")
    if (segments.length === 1) {
      directChildren.push(file)
    } else if (showSubfolders) {
      const subfolderName = segments[0]!
      if (!subfolderFiles.has(subfolderName)) subfolderFiles.set(subfolderName, [])
      subfolderFiles.get(subfolderName)!.push(file)
    }
  }

  for (const [subfolderName, files] of subfolderFiles) {
    const indexFile = files.find((file) => file.slug === `${folderPrefix}${subfolderName}/index`)
    if (indexFile) continue

    directChildren.push({
      slug: `${folderPrefix}${subfolderName}/index` as FullSlug,
      dates: mostRecentDatesFromEntries(files),
      frontmatter: { title: subfolderName, tags: [] },
    })
  }

  return directChildren
}

function dateFor(page: PageEntry): Date | undefined {
  const dateType = page.defaultDateType
  const dates = page.dates
  const value =
    (dateType ? dates?.[dateType as keyof typeof dates] : undefined) ??
    dates?.modified ??
    dates?.created ??
    dates?.published
  if (!value) return undefined

  const date = value instanceof Date ? value : new Date(value)
  return Number.isNaN(date.valueOf()) ? undefined : date
}

function byDateAndAlphabeticalFolderFirst(): SortFn {
  return (f1, f2) => {
    const f1IsFolder = isFolderPath(f1.slug ?? "")
    const f2IsFolder = isFolderPath(f2.slug ?? "")
    if (f1IsFolder && !f2IsFolder) return -1
    if (!f1IsFolder && f2IsFolder) return 1

    const f1Date = dateFor(f1)
    const f2Date = dateFor(f2)
    if (f1Date && f2Date) return f2Date.getTime() - f1Date.getTime()
    if (f1Date && !f2Date) return -1
    if (!f1Date && f2Date) return 1

    const f1Title = f1.frontmatter?.title?.toLowerCase() ?? ""
    const f2Title = f2.frontmatter?.title?.toLowerCase() ?? ""
    return f1Title.localeCompare(f2Title)
  }
}

function renderPageList(
  cfg: QuartzComponentProps["cfg"],
  currentSlug: FullSlug,
  pages: PageEntry[],
  sort: SortFn | undefined,
) {
  const sorter = sort ?? byDateAndAlphabeticalFolderFirst()
  return h(
    "ul",
    { class: "section-ul" },
    [...pages].sort(sorter).map((page) => {
      const title = page.frontmatter?.title
      const tags = page.frontmatter?.tags ?? []
      const date = dateFor(page)

      return h(
        "li",
        { class: "section-li", key: page.slug },
        h(
          "div",
          { class: "section" },
          h(
            "p",
            { class: "meta" },
            date
              ? h(
                  "time",
                  { datetime: date.toISOString() },
                  date.toLocaleDateString(cfg.locale, {
                    year: "numeric",
                    month: "short",
                    day: "2-digit",
                  }),
                )
              : null,
          ),
          h(
            "div",
            { class: "desc" },
            h(
              "h3",
              null,
              h(
                "a",
                {
                  href: resolveRelative(currentSlug, page.slug!),
                  class: "internal internal-link",
                },
                title,
              ),
            ),
          ),
          h(
            "ul",
            { class: "tags" },
            tags.map((tag) =>
              h(
                "li",
                { key: tag },
                h(
                  "a",
                  {
                    class: "internal tag-link",
                    href: resolveRelative(currentSlug, `tags/${tag}` as FullSlug),
                  },
                  tag,
                ),
              ),
            ),
          ),
        ),
      )
    }),
  )
}

const FolderContent = ((userOpts?: Partial<FolderContentOptions>) => {
  const options: FolderContentOptions = { ...defaultOptions, ...userOpts }

  const Component: QuartzComponent = (props: QuartzComponentProps) => {
    const { tree, fileData, allFiles, cfg } = props
    const ctx = props.ctx as { trie?: TrieNode } | undefined
    const slug = fileData.slug
    if (!slug) return null

    const trie = ctx?.trie
    let allPagesInFolder: PageEntry[]
    if (trie) {
      const folder = trie.findNode(slug.split("/"))
      if (!folder) return null
      allPagesInFolder = pagesFromTrie(folder, options.showSubfolders)
    } else {
      allPagesInFolder = pagesFromAllFiles(allFiles ?? [], slug, options.showSubfolders)
    }

    const cssClasses = fileData.frontmatter?.cssclasses
    const classes = Array.isArray(cssClasses) ? cssClasses.join(" ") : ""
    const hastRoot = tree as Root
    const content =
      hastRoot.children.length === 0
        ? fileData.description
        : htmlToJsx(hastRoot)
    const pageListContent = renderPageList(cfg, slug, allPagesInFolder, options.sort)
    const count =
      options.showFolderCount && allPagesInFolder.length > 0
        ? h(
            "p",
            null,
            itemsUnderFolder(allPagesInFolder.length),
          )
        : null

    return h(
      "div",
      { class: "popover-hint" },
      h(
        "article",
        { class: classes },
        h("div", { class: "markdown-preview-view markdown-rendered" }, content),
      ),
      h("div", { class: "page-listing" }, count, h("div", null, pageListContent)),
    )
  }

  Component.css = concatenateResources(styles)
  return Component
}) satisfies QuartzComponentConstructor<Partial<FolderContentOptions>>

function folderMatcher({ slug }: { slug: string }): boolean {
  return slug.endsWith("/index")
}

function getFolders(slug: string): string[] {
  let folderName = path.dirname(slug ?? "")
  const parentFolderNames = [folderName]
  while (folderName !== ".") {
    folderName = path.dirname(folderName ?? "")
    parentFolderNames.push(folderName)
  }
  return parentFolderNames
}

export const FolderPage: QuartzPageTypePlugin<FolderPageOptions> = (opts) => {
  const body: QuartzComponentConstructor = () => FolderContent(opts)

  return {
    name: "FolderPage",
    priority: 10,
    match: folderMatcher,
    generate({ content }) {
      const allFiles = content
        .map((entry) => entry[1].data)
        .filter((data) => (data as { unlisted?: unknown } | undefined)?.unlisted !== true)
      const folders = new Set<string>()
      const folderDisplayNames = new Map<string, string>()

      for (const file of allFiles) {
        const slug = (file as { slug?: string } | undefined)?.slug
        if (!slug) continue

        const fileFolders = getFolders(slug).filter((folder) => folder !== "." && folder !== "tags")
        for (const folder of fileFolders) folders.add(folder)

        const relativePath = (file as { relativePath?: string } | undefined)?.relativePath
        if (!relativePath) continue

        const slugParts = path.dirname(slug).split("/").filter((part) => part !== ".")
        const pathParts = path.dirname(relativePath).split("/").filter((part) => part !== ".")
        for (let index = 0; index < slugParts.length && index < pathParts.length; index++) {
          const slugPart = slugParts[index]
          const pathPart = pathParts[index]
          if (slugPart && pathPart && !folderDisplayNames.has(slugPart)) {
            folderDisplayNames.set(slugPart, pathPart)
          }
        }
      }

      const foldersWithIndex = new Set<string>()
      for (const [, file] of content) {
        const data = file.data as { slug?: string; unlisted?: unknown } | undefined
        if (data?.unlisted === true) continue

        const slug = data?.slug
        if (slug?.endsWith("/index")) foldersWithIndex.add(slug.slice(0, -"/index".length))
      }

      for (const [, file] of content) {
        const data = file.data as { slug?: string; frontmatter?: { title?: string } }
        const slug = data.slug
        if (!slug?.endsWith("/index")) continue
        if (!data.frontmatter || (data.frontmatter.title && data.frontmatter.title !== "index")) {
          continue
        }

        const folder = slug.slice(0, -"/index".length)
        const slugSegment = folder.split("/").pop() ?? folder
        const folderName = folderDisplayNames.get(slugSegment) ?? slugSegment
        data.frontmatter.title = opts?.prefixFolders
          ? `${folderLabel()}: ${folderName}`
          : folderName
      }

      const virtualPages: VirtualPage[] = []
      for (const folder of folders) {
        if (foldersWithIndex.has(folder)) continue

        const slug = joinSegments(folder, "index") as FullSlug
        const slugSegment = folder.split("/").pop() ?? folder
        const folderName = folderDisplayNames.get(slugSegment) ?? slugSegment
        const title = opts?.prefixFolders
          ? `${folderLabel()}: ${folderName}`
          : folderName

        virtualPages.push({
          slug,
          title,
          data: {},
        })
      }

      return virtualPages
    },
    layout: "folder",
    body,
  }
}

export default FolderPage
