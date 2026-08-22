/**
 * Shared shapes for the comment system.
 *
 * A comment is a markdown file checked in next to the page it belongs to, named
 * `<stem>.comment.<id>.md`. Readers never write one directly: the compose box
 * builds the file's text in the browser and hands it to GitHub's "new file"
 * editor, which forks and opens a pull request. Merging the pull request is
 * what publishes the comment, so moderation is the merge button.
 */

import type { Root as HastRoot } from "hast"

/**
 * Who wrote a comment, taken from the commit that added the file.
 *
 * Nothing about the author lives in the file itself, which is the point: the
 * frontmatter is written by a stranger's browser and could say anything, while
 * the commit author is whatever GitHub recorded when the pull request merged.
 */
export interface CommentAuthor {
  /** GitHub login, when the commit's email identifies one. */
  login?: string
  /** Display name from the commit, used when there is no login to show. */
  name: string
  /** Profile picture on GitHub. */
  avatar?: string
  /** Profile page on GitHub. */
  profile?: string
}

export interface CommentRecord {
  /** The `<id>` from the filename; unique per page, not globally. */
  id: string
  /** Content-relative path of the page being commented on, e.g. `blog/theism.md`. */
  parent: string
  /** ISO 8601. The commit's author date where there is one, else the frontmatter. */
  date: string
  /** Absent until the file has been committed, which is the only source for it. */
  author?: CommentAuthor
  /** `id` of the comment this answers, for a few levels of threading. */
  replyTo?: string
  /** Text quoted verbatim from the page, which the client re-finds and marks. */
  quote?: string
  /** Slug of the heading the quote sat under, to disambiguate repeated text. */
  quoteHeading?: string
  /** Sanitised body, ready to hand to `htmlToJsx`. */
  body: HastRoot
  /**
   * A complete HTML document to run in a sandboxed frame, for a comment that
   * carries code. Present means the comment gets a run button; nothing starts
   * until the reader presses it.
   */
  runnable?: string
}

/** Everything the compose box needs to address a pull request at the repo. */
export interface CommentTarget {
  /** `owner/repo` on GitHub. */
  repo: string
  /** Branch the pull request should target. */
  branch: string
  /** Repo-relative path of the page, e.g. `content/blog/theism.md`. */
  path: string
  /** Content-relative path of the page, which goes in the comment frontmatter. */
  parent: string
}

declare module "vfile" {
  interface DataMap {
    comments: CommentRecord[]
    commentTarget: CommentTarget
  }
}
