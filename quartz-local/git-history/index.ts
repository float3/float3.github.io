import { execFileSync } from "child_process"
import path from "path"
import { styleText } from "util"
import type { Root } from "mdast"
import type { VFile } from "vfile"
import type { QuartzTransformerPlugin } from "../../quartz/plugins/types"

declare module "vfile" {
  interface DataMap {
    versions: number
    historyUrl: string
  }
}

type Commit = {
  date: Date
  human: boolean
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
    ["log", "--format=%x00%aI%x1f%ae", "--name-status", "-M", "--no-show-signature"],
    { cwd, encoding: "utf8", maxBuffer: 512 * 1024 * 1024 },
  )

  const history = new Map<string, Commit[]>()
  const renamedTo = new Map<string, string>()

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
    const separator = header.indexOf("\x1f")
    if (separator < 0) continue

    const date = new Date(header.slice(0, separator))
    if (Number.isNaN(date.getTime())) continue
    const commit: Commit = { date, human: !isBotAuthor(header.slice(separator + 1)) }

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

      const commits = history.get(file)
      if (commits) {
        commits.push(commit)
      } else {
        history.set(file, [commit])
      }
    }
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
        file.data.versions = mine.length

        if (repo !== undefined) {
          file.data.historyUrl = `${repo.base}/commits/${repo.branch}/${encodePath(relative)}`
        }

        // Pages that only automation has ever touched (generated indices) still
        // deserve real dates, so fall back to the unfiltered history for those.
        const dated = mine.length > 0 ? mine : commits
        file.data.dates = {
          created: dated[dated.length - 1].date,
          modified: dated[0].date,
          published: file.data.dates?.published ?? dated[0].date,
        }
      },
    ]
  },
})

export default GitHistory
