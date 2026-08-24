/**
 * Turning a filled-in form into the file a pull request would add, and into
 * the routes that get it there.
 *
 * There is no server in any of them. The default route opens a prefilled
 * GitHub issue, and a workflow in the repository turns that into a branch, a
 * commit in the issue opener's name, and a pull request — which is the part
 * that settles who wrote the comment. The other two routes exist because the
 * first one can fail for reasons the reader cannot fix: a pull request opened
 * by hand for anyone who would rather see the diff, and an email for anyone
 * without a GitHub account at all.
 */

export interface CommentTarget {
  repo: string
  branch: string
  /** Repo-relative path of the page, e.g. `content/blog/theism.md`. */
  path: string
  /** Content-relative path, which is what goes in the frontmatter. */
  parent: string
  /** Where the email route sends to. Absent hides that option. */
  email?: string
  /** Web URL of the page, for the issue title and the mail subject. */
  page: string
}

export interface CommentDraft {
  id: string
  date: string
  body: string
  replyTo?: string
  quote?: string
  quoteHeading?: string
  /** id of the comment this replaces, when the reader pressed edit. */
  editing?: string
}

/** The three ways a comment can be submitted, in the order the menu offers them. */
export type Route = "issue" | "pull-request" | "email"

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
 * The file as the pull-request route would add it.
 *
 * Note what is *not* here: no author and no history. Both are the workflow's to
 * write, from the account that opened the issue — a browser cannot be trusted
 * to say who is using it. A file submitted through the pull-request route
 * therefore carries no author claim at all, and falls back to its commit.
 */
export function buildCommentFile(target: CommentTarget, draft: CommentDraft): string {
  const lines = ["---", `parent: ${scalar(target.parent)}`, `date: ${scalar(draft.date)}`]

  if (draft.replyTo !== undefined) lines.push(`replyTo: ${scalar(draft.replyTo)}`)
  if (draft.quote !== undefined) lines.push(`quote: ${scalar(draft.quote)}`)
  if (draft.quoteHeading !== undefined) lines.push(`quoteHeading: ${scalar(draft.quoteHeading)}`)

  lines.push("---", "", draft.body.trim(), "")
  return lines.join("\n")
}

// ---------------------------------------------------------------------------
// The issue route

/**
 * The marker the workflow looks for.
 *
 * It is an HTML comment, so GitHub renders the issue as just the comment's own
 * text with the machine-readable part invisible — the issue reads as what it
 * is. The workflow keys off this string rather than off a label, because a
 * label set through `?labels=` is silently dropped for anyone without triage
 * permission on the repository, which is everyone this feature is for.
 */
export const ISSUE_MARKER = "hilll.dev:comment"

interface IssuePayload {
  parent: string
  replyTo?: string
  quote?: string
  quoteHeading?: string
  /** Present for an edit; the workflow rewrites that file instead of adding one. */
  editing?: string
}

export function buildIssueBody(target: CommentTarget, draft: CommentDraft): string {
  const payload: IssuePayload = {
    parent: target.parent,
    replyTo: draft.replyTo,
    quote: draft.quote,
    quoteHeading: draft.quoteHeading,
    editing: draft.editing,
  }

  return [`<!--${ISSUE_MARKER}`, JSON.stringify(payload), "-->", "", draft.body.trim(), ""].join(
    "\n",
  )
}

function issueTitle(target: CommentTarget, draft: CommentDraft): string {
  const what = draft.editing !== undefined ? "Edit comment on" : "Comment on"
  return `${what} ${target.parent.replace(/\.md$/, "")}`
}

// ---------------------------------------------------------------------------
// Routes

function issueUrl(target: CommentTarget, draft: CommentDraft): string {
  const query = new URLSearchParams({
    title: issueTitle(target, draft),
    body: buildIssueBody(target, draft),
    labels: "comment",
  })
  return `https://github.com/${target.repo}/issues/new?${query.toString()}`
}

/**
 * GitHub's "create a new file" editor, prefilled.
 *
 * Someone without push access gets offered a fork automatically, and the
 * editor's own commit form is what opens the pull request.
 */
function pullRequestUrl(target: CommentTarget, path: string, content: string): string {
  const query = new URLSearchParams({ filename: path, value: content })
  return `https://github.com/${target.repo}/new/${target.branch}?${query.toString()}`
}

function emailUrl(target: CommentTarget, draft: CommentDraft): string {
  const body = [
    draft.quote !== undefined ? `Quoting: ${draft.quote}` : undefined,
    draft.editing !== undefined ? `This edits comment ${draft.editing}.` : undefined,
    "",
    draft.body.trim(),
    "",
    `— on ${target.page}`,
  ]
    .filter((line) => line !== undefined)
    .join("\n")

  const query = new URLSearchParams({ subject: issueTitle(target, draft), body })
  // `mailto:` wants percent-encoded spaces, and URLSearchParams writes `+`,
  // which mail clients paste through literally.
  return `mailto:${target.email}?${query.toString().replace(/\+/g, "%20")}`
}

export interface Submission {
  url: string
  /** Shown under the button, so the reader knows what pressing it does. */
  explanation: string
  /** The file that would be added, for the preview and the copy button. */
  content: string
  path: string
}

export function buildSubmission(
  route: Route,
  target: CommentTarget,
  draft: CommentDraft,
): Submission {
  const content = buildCommentFile(target, draft)
  const path = commentPath(target, draft.id)

  switch (route) {
    case "issue":
      return {
        url: issueUrl(target, draft),
        explanation:
          "Opens an issue. A workflow turns it into a pull request in your name, and closes the issue.",
        content,
        path,
      }
    case "pull-request":
      return {
        url: pullRequestUrl(target, path, content),
        explanation: `Opens GitHub's file editor at ${path}, forking ${target.repo} if you cannot push to it.`,
        content,
        path,
      }
    case "email":
      return {
        url: emailUrl(target, draft),
        explanation: `Opens your mail client. No GitHub account needed; it gets added by hand.`,
        content,
        path,
      }
  }
}

/**
 * Past roughly this much, a prefilled URL stops being reliable — the limit is
 * GitHub's and it is not documented, so the number is a conservative guess
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
