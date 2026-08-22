/**
 * Who wrote each comment, according to git.
 *
 * The comment file carries no name, no picture and no key, because none of
 * those could be trusted: the file is composed by a stranger's browser and
 * anything in it is whatever they typed. What cannot be typed is the commit —
 * a comment only appears on the site once a pull request merges, and GitHub
 * records the pull request's author as the author of the commit that added the
 * file. So authorship is read from `git log` and never from the frontmatter.
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
