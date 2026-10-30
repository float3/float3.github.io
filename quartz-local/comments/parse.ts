/**
 * Rendering the comments `site comments-index` found.
 *
 * Finding them, reading their frontmatter and settling who wrote each one is
 * done in Rust (`tools/site/src/comment_index.rs`), which writes the result to
 * `.quartz/comments.json`; `site build` runs it before Quartz. What is left
 * here is the part that needs unified: turning each body into sanitised HTML.
 */

import fs from "fs"
import path from "path"
import { unified } from "unified"
import remarkParse from "remark-parse"
import remarkRehype from "remark-rehype"
import type { Root as HastRoot } from "hast"
import { fromHtml } from "hast-util-from-html"
import { hasExecutable, sanitize } from "./sanitize"
import { serialize, serializeRaw, withoutRaw } from "./serialize"
import { runnableFromFences, runnableFromHtml } from "./runnable"
import type { CommentRecord } from "./types"

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
  const parsed = fromHtml(serialize(mixed), { fragment: true })

  // Checking the parsed tree rather than the source means a `<script>` shown
  // inside a code fence does not count as one written.
  if (!hasExecutable(parsed)) {
    return { body: sanitize(parsed), runnable: runnableFromFences(source) }
  }

  // A runnable comment is two things at once, and they are told apart by how
  // they were written: the prose is markdown and belongs on the page, the HTML
  // is the thing being built and belongs in the frame. Putting the whole
  // comment in the frame renders the prose a second time inside the box, and
  // leaves the markup it needs sitting dead on the page — a "new puzzle" button
  // wired to nothing.
  const prose = fromHtml(serialize(withoutRaw(mixed)), { fragment: true })
  return { body: sanitize(prose), runnable: runnableFromHtml(serializeRaw(mixed)) }
}

/** A comment as the index records it: everything but the rendered body. */
type IndexedComment = Omit<CommentRecord, "body" | "runnable">

/** Where `site comments-index` writes, relative to the repository root. */
export const COMMENTS_INDEX = ".quartz/comments.json"

/**
 * Every comment in the index, rendered, keyed by the absolute path of its page.
 *
 * A missing index is an error rather than an empty thread on every page: the
 * build that forgot to write it would otherwise publish the site with every
 * comment gone.
 */
export function loadComments(indexPath: string, contentDir: string): Map<string, CommentRecord[]> {
  if (!fs.existsSync(indexPath)) {
    throw new Error(
      `${indexPath} is missing; run \`site comments-index\` (part of \`site build\`) first`,
    )
  }
  const index = JSON.parse(fs.readFileSync(indexPath, "utf8")) as Record<string, IndexedComment[]>

  const byParent = new Map<string, CommentRecord[]>()
  for (const [parent, thread] of Object.entries(index)) {
    byParent.set(
      path.resolve(contentDir, parent),
      thread.map((comment) => ({ ...comment, ...renderBody(comment.source) })),
    )
  }
  return byParent
}
