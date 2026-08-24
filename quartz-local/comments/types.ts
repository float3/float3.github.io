/**
 * Shared shapes for the comment system.
 *
 * A comment is a markdown file checked in next to the page it belongs to, named
 * `<stem>.comment.<id>.md`. Readers never write one directly: the compose box
 * builds the file's text in the browser and submits it by whichever of three
 * routes they pick — a prefilled GitHub issue that a workflow turns into a pull
 * request, a pull request opened by hand, or an email. Merging the pull request
 * is what publishes the comment, so moderation is the merge button.
 */

import type { Root as HastRoot } from "hast"

/**
 * Who wrote a comment.
 *
 * Written into the file by the workflow, from the account that opened the
 * issue, and corroborated by the commit the workflow makes in that account's
 * name. Files that predate the workflow carry no author and fall back to the
 * commit that added them — see `authors.ts`.
 */
export interface CommentAuthor {
  /** GitHub login, for a comment that came through GitHub. */
  login?: string
  /**
   * Email address, for one that arrived as mail.
   *
   * Kept so an edit can be matched against it, and deliberately not rendered —
   * see `authorFromIdentity`.
   */
  email?: string
  /** Display name: the login, the local part of the address, or what git said. */
  name: string
  /** Profile picture on GitHub. */
  avatar?: string
  /** Profile page on GitHub. */
  profile?: string
}

/** One submission against a comment: the first one, then each edit. */
export interface CommentRevision {
  date: string
  /** Issue the revision arrived as, so the page can link to where it was made. */
  issue?: number
  /** False for the original, true for every revision after it. */
  edited: boolean
}

export interface CommentRecord {
  /** The `<id>` from the filename; unique per page, not globally. */
  id: string
  /** Repo-relative path of this comment's own file, for linking to its history. */
  file: string
  /** Content-relative path of the page being commented on, e.g. `blog/theism.md`. */
  parent: string
  /** ISO 8601. When the comment was first submitted. */
  date: string
  /** When it was last edited, if it ever was. */
  edited?: string
  author?: CommentAuthor
  /** Every submission, oldest first. Empty for a file written by hand. */
  history: CommentRevision[]
  /** `id` of the comment this answers, for a few levels of threading. */
  replyTo?: string
  /** Text quoted verbatim from the page, which the client re-finds and marks. */
  quote?: string
  /** Slug of the heading the quote sat under, to disambiguate repeated text. */
  quoteHeading?: string
  /** The body as written, so the edit button can prefill it again. */
  source: string
  /** Sanitised body, ready to hand to `htmlToJsx`. */
  body: HastRoot
  /**
   * A complete HTML document to run in a sandboxed frame, for a comment that
   * carries code. Present means the comment gets a run button; nothing starts
   * until the reader presses it.
   */
  runnable?: string
}

/** Everything the compose box needs to submit against this repository. */
export interface CommentTarget {
  /** `owner/repo` on GitHub. */
  repo: string
  /** Branch pull requests should target. */
  branch: string
  /** Repo-relative path of the page, e.g. `content/blog/theism.md`. */
  path: string
  /** Content-relative path of the page, which goes in the comment frontmatter. */
  parent: string
  /** Where the email route sends to. */
  email?: string
  /** Web URL of the page, for the issue title and the email subject. */
  page: string
}

declare module "vfile" {
  interface DataMap {
    comments: CommentRecord[]
    commentTarget: CommentTarget
  }
}
