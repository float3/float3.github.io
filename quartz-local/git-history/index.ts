import path from "path"
import type { Root } from "mdast"
import type { VFile } from "vfile"
import type { QuartzTransformerPlugin } from "../../quartz/plugins/types"
import { defaultBranch, git, originRemote, repoRoot, webUrl } from "../shared/git"
import { warn } from "../shared/warn"
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

function readRepo(cwd: string): Repo | undefined {
  const base = webUrl(originRemote(cwd) ?? "")
  return base === undefined ? undefined : { base, branch: defaultBranch(cwd) }
}

const encodePath = (file: string) => file.split("/").map(encodeURIComponent).join("/")

// CI pushes generated/maintenance commits as `github-actions[bot]` and Dependabot
// pushes as `dependabot[bot]`. Neither is an edit I made, so neither counts as a
// version or moves the "updated" date. My own GitHub noreply address lives on the
// same domain without the `[bot]` marker, so it still counts.
const isBotAuthor = (email: string) =>
  email.endsWith("@users.noreply.github.com") && email.includes("[bot]")

// One `git log` for the entire repository, rather than one per file. `--name-status`
// with `-M` gives rename records, which is what threads a file back through its
// old names without a per-file `--follow`.
function readHistory(cwd: string): Map<string, Commit[]> | undefined {
  const output = git(
    cwd,
    ["log", "--format=%x00%H%x1f%aI%x1f%ae", "--name-status", "-M", "--no-show-signature"],
    512 * 1024 * 1024,
  )
  if (output === undefined) return undefined

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
    warn(
      `excluded-commits.ts keeps ${strayPaths.length} path(s) no file has: ` +
        strayPaths.join(", "),
    )
  }

  const missing = excludedShas().filter((sha) => !seen.has(sha))
  if (missing.length > 0) {
    warn(
      `excluded-commits.ts lists ${missing.length} commit(s) not in this history: ` +
        missing.map((sha) => sha.slice(0, 8)).join(", "),
    )
  }

  return history
}

// Quartz may parse in worker threads; keep one map per process.
const cached = new Map<string, Map<string, Commit[]> | undefined>()
const cachedRepos = new Map<string, Repo | undefined>()

export const GitHistory: QuartzTransformerPlugin = () => ({
  name: "GitHistory",
  markdownPlugins(ctx) {
    const root = repoRoot(ctx.argv.directory)
    let history: Map<string, Commit[]> | undefined
    let repo: Repo | undefined

    if (root !== undefined) {
      if (!cached.has(root)) {
        cached.set(root, readHistory(root))
        cachedRepos.set(root, readRepo(root))
      }
      history = cached.get(root)
      repo = cachedRepos.get(root)
    }
    if (history === undefined) {
      warn("no git history available, falling back to frontmatter dates")
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
