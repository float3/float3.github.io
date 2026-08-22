/**
 * Reading the comment files off disk.
 *
 * One walk of the content directory per process, paired with one `git log` for
 * the authorship — see `authors.ts` for why none of that is in the files.
 */

import fs from "fs"
import path from "path"
import YAML from "yaml"
import { unified } from "unified"
import remarkParse from "remark-parse"
import remarkRehype from "remark-rehype"
import type { Root as HastRoot } from "hast"
import { fromHtml } from "hast-util-from-html"
import { hasExecutable, sanitize } from "./sanitize"
import { serialize } from "./serialize"
import { runnableFromFences, runnableFromHtml } from "./runnable"
import { readCommentAuthors, type CommentCommit } from "./authors"
import type { CommentRecord } from "./types"

const COMMENT_FILE = /^(.*)\.comment\.([A-Za-z0-9_-]{1,32})\.md$/
const FRONTMATTER = /^---\r?\n([\s\S]*?)\r?\n---\r?\n?([\s\S]*)$/

// `allowDangerousHtml` is the whole point: a comment may contain HTML, and a
// comment may contain a script. What it may *not* do is put either of those
// into this document unread — see `sanitize.ts` for where the two part ways.
const markdown = unified().use(remarkParse).use(remarkRehype, { allowDangerousHtml: true })

interface RenderedBody {
  body: HastRoot
  /** A complete document to run in a sandboxed frame, if there is one. */
  runnable?: string
}

function renderBody(source: string): RenderedBody {
  const mixed = markdown.runSync(markdown.parse(source)) as HastRoot

  // The author's HTML is still opaque `raw` text at this point, and unbalanced
  // wherever a tag opened in one node and closed in another. Writing the tree
  // back out and letting a real parser read it is what makes it a tree at all.
  const html = serialize(mixed)
  const parsed = fromHtml(html, { fragment: true })

  return {
    body: sanitize(parsed),
    // A comment that wrote a script runs as itself; one that pasted fenced code
    // runs that. Checking the parsed tree rather than the source means a
    // `<script>` shown inside a code fence does not count as one written.
    runnable: hasExecutable(parsed) ? runnableFromHtml(html) : runnableFromFences(source),
  }
}

function optionalString(value: unknown): string | undefined {
  return typeof value === "string" && value.trim() !== "" ? value.trim() : undefined
}

const toPosix = (file: string) => file.split(path.sep).join("/")

function readComment(
  file: string,
  contentDir: string,
  commits: Map<string, CommentCommit>,
  repoRoot: string,
): { record: CommentRecord; parentPath: string } | undefined {
  const match = COMMENT_FILE.exec(path.basename(file))
  if (match === null) return undefined
  const [, stem, id] = match

  const parentPath = path.resolve(path.dirname(file), `${stem}.md`)
  // A comment on a page that has since been deleted or renamed has nothing to
  // attach to. Dropping it silently is right: the file is still in the repo and
  // still in history, it just has no page left to appear on.
  if (!fs.existsSync(parentPath)) return undefined

  const parsed = FRONTMATTER.exec(fs.readFileSync(file, "utf8"))
  if (parsed === null) return undefined
  const [, header, source] = parsed

  let frontmatter: unknown
  try {
    frontmatter = YAML.parse(header)
  } catch {
    return undefined
  }
  if (typeof frontmatter !== "object" || frontmatter === null) return undefined
  const fields = frontmatter as Record<string, unknown>

  const body = source.trim()
  if (body === "") return undefined

  const commit = commits.get(toPosix(path.relative(repoRoot, file)))

  // The commit's date is what the comment was actually written on. The date in
  // the file is the browser's guess at the same moment, and only survives while
  // the file is uncommitted — which, locally, is the whole time it is being
  // tested.
  const date = commit?.date ?? optionalString(fields.date)
  if (date === undefined || Number.isNaN(Date.parse(date))) return undefined

  return {
    parentPath,
    record: {
      id,
      parent: toPosix(path.relative(contentDir, parentPath)),
      date,
      author: commit?.author,
      replyTo: optionalString(fields.replyTo),
      quote: optionalString(fields.quote),
      quoteHeading: optionalString(fields.quoteHeading),
      ...renderBody(body),
    },
  }
}

function walk(dir: string, found: string[]): void {
  let entries: fs.Dirent[]
  try {
    entries = fs.readdirSync(dir, { withFileTypes: true })
  } catch {
    return
  }
  for (const entry of entries) {
    const full = path.join(dir, entry.name)
    if (entry.isDirectory()) {
      if (entry.name === "node_modules" || entry.name.startsWith(".")) continue
      walk(full, found)
    } else if (COMMENT_FILE.test(entry.name)) {
      found.push(full)
    }
  }
}

/** Every comment in the content tree, keyed by the absolute path of its page. */
export function scanComments(contentDir: string, repoRoot: string): Map<string, CommentRecord[]> {
  const files: string[] = []
  walk(contentDir, files)

  const commits = readCommentAuthors(repoRoot)
  const byParent = new Map<string, CommentRecord[]>()

  for (const file of files) {
    const read = readComment(file, contentDir, commits, repoRoot)
    if (read === undefined) continue

    const existing = byParent.get(read.parentPath)
    if (existing) existing.push(read.record)
    else byParent.set(read.parentPath, [read.record])
  }

  // Oldest first, the way a thread reads.
  for (const thread of byParent.values()) {
    thread.sort((a, b) => Date.parse(a.date) - Date.parse(b.date))
  }

  return byParent
}
