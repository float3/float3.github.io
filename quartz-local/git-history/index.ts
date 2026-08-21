import { execFileSync } from "child_process"
import path from "path"
import { styleText } from "util"
import type { Root } from "mdast"
import type { VFile } from "vfile"
import type { QuartzTransformerPlugin } from "../../quartz/plugins/types"
import { excludedKeepPaths, excludedShas, isExcluded } from "./excluded-commits"

declare module "vfile" {
  interface DataMap {
    versions: number
    historyUrl: string
  }
}

type Commit = {
  sha: string
  date: Date
  human: boolean
  /** This commit created the file, under this name or one it was renamed from. */
  added: boolean
}

type Repo = {
  base: string
  branch: string
}

// `git@github.com:owner/repo.git` and `https://github.com/owner/repo.git` both need
// to become a browsable `https://github.com/owner/repo`.
function webUrl(remote: string): string | undefined {
  // Checked before the scp-like form below, whose `host:path` shape would otherwise
  // swallow the `https:` scheme as a hostname.
  const url = /^(?:https?|ssh|git):\/\/(?:[^@/]+@)?(.+?)(?:\.git)?\/?$/.exec(remote)
  if (url) return `https://${url[1]}`

  const scp = /^(?:[^@/]+@)?([^:/]+):(.+?)(?:\.git)?$/.exec(remote)
  if (scp) return `https://${scp[1]}/${scp[2]}`

  return undefined
}

// The remote's default branch, not the one checked out: feature branches disappear
// and would leave the links 404ing.
function readRepo(cwd: string): Repo | undefined {
  let base: string | undefined
  try {
    base = webUrl(
      execFileSync("git", ["remote", "get-url", "origin"], { cwd, encoding: "utf8" }).trim(),
    )
  } catch {
    return undefined
  }
  if (base === undefined) return undefined

  let branch = "master"
  try {
    // CI checkouts often fetch a single ref and never write origin/HEAD.
    branch =
      execFileSync("git", ["symbolic-ref", "--short", "refs/remotes/origin/HEAD"], {
        cwd,
        encoding: "utf8",
      })
        .trim()
        .replace(/^origin\//, "") || branch
  } catch {
    /* keep the fallback */
  }

  return { base, branch }
}

const encodePath = (file: string) => file.split("/").map(encodeURIComponent).join("/")

// CI pushes generated/maintenance commits as `github-actions[bot]` and Dependabot
// pushes as `dependabot[bot]`. Neither is an edit I made, so neither counts as a
// version or moves the "updated" date. My own GitHub noreply address lives on the
// same domain without the `[bot]` marker, so it still counts.
const isBotAuthor = (email: string) =>
  email.endsWith("@users.noreply.github.com") && email.includes("[bot]")

// One `git log` for the entire repository, rather than one per file. `--name-status`
// with `-M` gives us rename records, which is what lets us thread a file back through
// its old names without the per-file `--follow` this used to cost.
function readHistory(cwd: string): Map<string, Commit[]> {
  const output = execFileSync(
    "git",
    ["log", "--format=%x00%H%x1f%aI%x1f%ae", "--name-status", "-M", "--no-show-signature"],
    { cwd, encoding: "utf8", maxBuffer: 512 * 1024 * 1024 },
  )

  const history = new Map<string, Commit[]>()
  const renamedTo = new Map<string, string>()
  const seen = new Set<string>()

  // git log is newest-first, so every rename is recorded before we reach the older
  // commits that still refer to the previous name.
  const currentName = (file: string): string => {
    const seen = new Set<string>()
    while (renamedTo.has(file) && !seen.has(file)) {
      seen.add(file)
      file = renamedTo.get(file)!
    }
    return file
  }

  for (const block of output.split("\0")) {
    if (block.trim() === "") continue

    const [header, ...entries] = block.split("\n")
    const [sha, authored, email] = header.split("\x1f")
    if (sha === undefined || authored === undefined || email === undefined) continue

    const date = new Date(authored)
    if (Number.isNaN(date.getTime())) continue
    const shared = { sha, date, human: !isBotAuthor(email) }
    seen.add(sha)

    for (const entry of entries) {
      if (entry === "") continue

      // "M\tpath", "A\tpath", or "R100\told\tnew"
      const fields = entry.split("\t")
      if (fields.length < 2) continue

      let file: string
      if (fields[0].startsWith("R") && fields.length >= 3) {
        file = currentName(fields[2])
        renamedTo.set(fields[1], file)
      } else {
        file = currentName(fields[1])
      }

      // Per file rather than per commit: one commit adds some paths and merely
      // touches others, and only the add means authorship of this page.
      const record: Commit = { ...shared, added: fields[0].startsWith("A") }

      const commits = history.get(file)
      if (commits) {
        commits.push(record)
      } else {
        history.set(file, [record])
      }
    }
  }

  // A listed SHA that history no longer contains excludes nothing, silently.
  // That happens after a rebase, or from a typo, and either way the list has
  // rotted rather than done its job.
  const strayPaths = excludedKeepPaths().filter((file) => !history.has(file))
  if (strayPaths.length > 0) {
    console.log(
      styleText(
        "yellow",
        `
Warning: excluded-commits.ts keeps ${strayPaths.length} path(s) no file has: ` +
          strayPaths.join(", "),
      ),
    )
  }

  const missing = excludedShas().filter((sha) => !seen.has(sha))
  if (missing.length > 0) {
    console.log(
      styleText(
        "yellow",
        `\nWarning: excluded-commits.ts lists ${missing.length} commit(s) not in this history: ` +
          missing.map((sha) => sha.slice(0, 8)).join(", "),
      ),
    )
  }

  return history
}

// Quartz may parse in worker threads; keep one map per process.
const cached = new Map<string, Map<string, Commit[]>>()
const cachedRepos = new Map<string, Repo | undefined>()

export const GitHistory: QuartzTransformerPlugin = () => ({
  name: "GitHistory",
  markdownPlugins(ctx) {
    let root: string | undefined
    let history: Map<string, Commit[]> | undefined
    let repo: Repo | undefined

    try {
      root = execFileSync("git", ["rev-parse", "--show-toplevel"], {
        cwd: ctx.argv.directory,
        encoding: "utf8",
      }).trim()

      history = cached.get(root)
      if (!history) {
        history = readHistory(root)
        cached.set(root, history)
      }

      if (cachedRepos.has(root)) {
        repo = cachedRepos.get(root)
      } else {
        repo = readRepo(root)
        cachedRepos.set(root, repo)
      }
    } catch {
      console.log(
        styleText(
          "yellow",
          "\nWarning: no git history available, falling back to frontmatter dates",
        ),
      )
    }

    return [
      () => (_tree: Root, file: VFile) => {
        if (root === undefined || history === undefined) return

        const filePath = file.data.filePath
        if (filePath === undefined) return

        const relative = path.relative(root, path.resolve(filePath)).split(path.sep).join("/")

        const commits = history.get(relative)
        if (commits === undefined || commits.length === 0) {
          // A draft I haven't committed yet. Saying so beats silently dropping the
          // count, which reads as a broken build while writing locally, and the
          // dates from the frontmatter/filesystem fallback still stand.
          file.data.versions = 0
          return
        }

        const mine = commits.filter((commit) => commit.human)
        // Sweeps stay in `commits` and go on feeding the created date below;
        // they just do not count as revisions of this page, or move its
        // updated date to whenever the find-and-replace happened to run.
        // A commit that created the page is authorship by definition, never a
        // sweep, however many other files it swept on the way past. Without
        // this a page written during a migration reports zero versions.
        const edits = mine.filter((commit) => commit.added || !isExcluded(commit.sha, relative))
        file.data.versions = edits.length

        if (repo !== undefined) {
          file.data.historyUrl = `${repo.base}/commits/${repo.branch}/${encodePath(relative)}`
        }

        // Pages that only automation has ever touched (generated indices) still
        // deserve real dates, so fall back to the unfiltered history for those.
        const dated = mine.length > 0 ? mine : commits
        // A page every one of whose edits was a sweep has no meaningful
        // "updated" left, so it falls back rather than reporting nothing.
        const touched = edits.length > 0 ? edits : dated
        file.data.dates = {
          created: dated[dated.length - 1].date,
          modified: touched[0].date,
          published: file.data.dates?.published ?? touched[0].date,
        }
      },
    ]
  },
})

export default GitHistory
