/**
 * Who wrote each comment.
 *
 * There are two sources and they agree by construction. The workflow that turns
 * an issue into a pull request writes the opener's login into the file *and*
 * makes the commit in their name, so the frontmatter is a convenience and the
 * commit is the corroboration. Files written before the workflow existed, or by
 * hand, carry no author at all and fall back to the commit alone.
 *
 * The frontmatter is what renders, because an edit changes the file's author
 * field not at all while it does change who last touched the commit. What it
 * cannot do is make an unreviewed claim true: a pull request opened by hand can
 * put any login in the file, and the merge is what checks it — the same merge
 * that has always been the only gate here.
 */

import { execFileSync } from "child_process"
import type { CommentAuthor } from "./types"

/**
 * GitHub's private-email form, in both the shapes it has had.
 *
 * The modern one carries the account's numeric id, which is the stable handle:
 * a login can be changed or reused, an id cannot.
 */
const NOREPLY = /^(?:(\d+)\+)?([A-Za-z0-9-]+)@users\.noreply\.github\.com$/

/**
 * Someone whose commits use their real email gives no login to look up, so
 * they get the name git recorded and no picture. Everyone arriving through the
 * web editor — which is everyone the compose box sends — gets the noreply form.
 */
function toAuthor(name: string, email: string): CommentAuthor {
  const match = NOREPLY.exec(email)
  if (match === null) return { name }

  const [, id, login] = match
  return {
    login,
    name: login,
    // The id-based URL is stable across renames; the login-based one is all
    // there is for accounts old enough to predate the id in the address.
    avatar:
      id !== undefined
        ? `https://avatars.githubusercontent.com/u/${id}?s=64&v=4`
        : `https://github.com/${login}.png?size=64`,
    profile: `https://github.com/${login}`,
  }
}

export interface CommentCommit {
  author: CommentAuthor
  /** The commit's author date, which is when the comment was actually written. */
  date: string
}

/**
 * Builds an author from whatever identity the file records.
 *
 * That is a GitHub login for anything submitted through the issue or
 * pull-request routes, and an email address for a comment that arrived as mail
 * and was added by hand. The two are told apart by the `@`, which a GitHub
 * login cannot contain.
 */
export function authorFromIdentity(identity: string, id?: number): CommentAuthor {
  if (identity.includes("@")) return authorFromEmail(identity)

  return {
    login: identity,
    name: identity,
    // The id-based URL is stable across renames; the login-based one is all
    // there is when the file records no id.
    avatar:
      id !== undefined
        ? `https://avatars.githubusercontent.com/u/${id}?s=64&v=4`
        : `https://github.com/${identity}.png?size=64`,
    profile: `https://github.com/${identity}`,
  }
}

/**
 * An email author, shown by the part before the `@`.
 *
 * The whole address stays in the file, because that is the only thing an edit
 * can be checked against later. It is not rendered: the file is public either
 * way, but a page is what gets crawled, and printing an address into one is
 * how it ends up on a list.
 */
function authorFromEmail(email: string): CommentAuthor {
  const local = email.slice(0, email.indexOf("@")).trim()
  return { email, name: local === "" ? "by email" : local }
}

/**
 * One `git log` for every comment file in the repository.
 *
 * Only the commit that *added* each file matters: later commits are edits by
 * whoever tidied it, most often me, and attributing the comment to them would
 * be worse than showing nothing. The log is newest-first, so the first record
 * seen for a path is the one that stands — which is also what makes a deleted
 * and re-added file resolve to its current author rather than its first.
 */
export function readCommentAuthors(cwd: string): Map<string, CommentCommit> {
  const authors = new Map<string, CommentCommit>()

  let output: string
  try {
    output = execFileSync(
      "git",
      [
        "log",
        "--format=%x00%aI%x1f%an%x1f%ae",
        "--name-only",
        "--diff-filter=A",
        "--no-show-signature",
        "--",
        "*.comment.*.md",
      ],
      { cwd, encoding: "utf8", maxBuffer: 64 * 1024 * 1024 },
    )
  } catch {
    // No git, no shallow history, no authors. The thread still renders.
    return authors
  }

  for (const block of output.split("\0")) {
    if (block.trim() === "") continue

    const [header, ...files] = block.split("\n")
    const [date, name, email] = header.split("\x1f")
    if (date === undefined || name === undefined || email === undefined) continue

    const commit: CommentCommit = { author: toAuthor(name, email), date }
    for (const file of files) {
      if (file === "" || authors.has(file)) continue
      authors.set(file, commit)
    }
  }

  return authors
}
