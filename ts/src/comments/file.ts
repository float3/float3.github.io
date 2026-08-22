/**
 * Turning a filled-in form into the file a pull request would add, and into
 * the three ways of getting that file to the repository.
 *
 * There is no server anywhere in this. GitHub's web editor accepts a path and
 * a body as query parameters, forks the repository on the reader's behalf if
 * they cannot push to it, and offers to open the pull request — which covers
 * the entire round trip that the usual recipe for this spends a serverless
 * function on. It also settles who wrote the comment, since the commit it
 * makes is attributed to whichever account opened the pull request.
 */

export interface CommentTarget {
  repo: string
  branch: string
  /** Repo-relative path of the page, e.g. `content/blog/theism.md`. */
  path: string
  /** Content-relative path, which is what goes in the frontmatter. */
  parent: string
}

export interface CommentDraft {
  id: string
  date: string
  body: string
  replyTo?: string
  quote?: string
  quoteHeading?: string
}

/**
 * Where the comment file lands: beside the page, sharing its name.
 *
 * `page.md` and `page.comment.<id>.md` sort next to each other in every file
 * listing there is, which is the whole reason for the naming scheme.
 */
export function commentPath(target: CommentTarget, id: string): string {
  return target.path.replace(/\.md$/, `.comment.${id}.md`)
}

// Double-quoted YAML scalars accept exactly the JSON string escapes, so JSON
// is both the safest and the shortest way to write a value that might contain
// a colon, a newline, or a leading dash.
const scalar = (value: string) => JSON.stringify(value)

/**
 * Note what is *not* here: no name, no picture, nothing about who is writing.
 * All of that is read back off the commit at build time, where it cannot be
 * typed into a text box.
 */
export function buildCommentFile(target: CommentTarget, draft: CommentDraft): string {
  const lines = ["---", `parent: ${scalar(target.parent)}`, `date: ${scalar(draft.date)}`]

  if (draft.replyTo !== undefined) lines.push(`replyTo: ${scalar(draft.replyTo)}`)
  if (draft.quote !== undefined) lines.push(`quote: ${scalar(draft.quote)}`)
  if (draft.quoteHeading !== undefined) lines.push(`quoteHeading: ${scalar(draft.quoteHeading)}`)

  lines.push("---", "", draft.body.trim(), "")
  return lines.join("\n")
}

/**
 * GitHub's "create a new file" editor, prefilled.
 *
 * Someone without push access gets offered a fork automatically, and the
 * editor's own commit form is what opens the pull request — so this one link
 * is the entire submission path.
 */
export function newFileUrl(target: CommentTarget, path: string, content: string): string {
  const query = new URLSearchParams({ filename: path, value: content })
  return `https://github.com/${target.repo}/new/${target.branch}?${query.toString()}`
}

/**
 * Past roughly this much, the prefilled URL stops being reliable — the limit
 * is GitHub's and it is not documented, so the number is a conservative guess
 * rather than a boundary anyone promised. Beyond it the copy-and-paste path is
 * offered instead of a link that would fail after the reader had committed to
 * clicking it.
 */
export const MAX_URL_LENGTH = 7000

/** A `git apply`-able patch, for anyone who would rather not use the web editor. */
export function buildPatch(path: string, content: string): string {
  const body = content.endsWith("\n") ? content : content + "\n"
  const lines = body.slice(0, -1).split("\n")
  return [
    `diff --git a/${path} b/${path}`,
    "new file mode 100644",
    "--- /dev/null",
    `+++ b/${path}`,
    `@@ -0,0 +1,${lines.length} @@`,
    ...lines.map((line) => `+${line}`),
    "",
  ].join("\n")
}

/**
 * An id unique within one page's thread.
 *
 * Only the page's own comments need distinguishing — the filename already
 * carries the page — so this is short enough to read out and still has room
 * for far more comments than any page here will collect.
 */
export function newCommentId(taken: Set<string>): string {
  for (let attempt = 0; attempt < 100; attempt++) {
    const bytes = crypto.getRandomValues(new Uint8Array(4))
    const id = [...bytes].map((byte) => byte.toString(16).padStart(2, "0")).join("")
    if (!taken.has(id)) return id
  }
  // Four bytes colliding a hundred times running is not a case worth handling
  // cleverly, but it is worth not returning a duplicate.
  return Date.now().toString(16)
}
