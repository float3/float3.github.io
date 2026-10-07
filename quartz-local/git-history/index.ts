/**
 * Each page's version count, dates and history link, from the index
 * `site git-history` writes (`tools/site/src/git_history.rs`, run by
 * `site build` before Quartz). Which commits count, the bots and the sweeps
 * that do not, and the threading through renames all happen there.
 */

import fs from "fs"
import path from "path"
import type { Root } from "mdast"
import type { VFile } from "vfile"
import type { QuartzTransformerPlugin } from "../../quartz/plugins/types"
import { repoRoot } from "../shared/git"
import { warn } from "../shared/warn"

declare module "vfile" {
  interface DataMap {
    versions: number
    historyUrl: string
  }
}

/** Where `site git-history` writes, relative to the repository root. */
const GIT_HISTORY_INDEX = ".quartz/git-history.json"

interface PageHistory {
  versions: number
  created: string
  modified: string
  historyUrl?: string
}

/** `pages` is null when the index was written without any git history to read. */
interface Index {
  pages: Record<string, PageHistory> | null
}

function readIndex(root: string): Index {
  const indexPath = path.join(root, GIT_HISTORY_INDEX)
  if (!fs.existsSync(indexPath)) {
    throw new Error(
      `${indexPath} is missing; run \`site git-history\` (part of \`site build\`) first`,
    )
  }
  return JSON.parse(fs.readFileSync(indexPath, "utf8")) as Index
}

// Quartz may parse in worker threads; keep one index per process.
const cached = new Map<string, Index>()

export const GitHistory: QuartzTransformerPlugin = () => ({
  name: "GitHistory",
  markdownPlugins(ctx) {
    const contentDir = path.resolve(ctx.argv.directory)
    const root = repoRoot(contentDir) ?? path.dirname(contentDir)

    let index = cached.get(root)
    if (index === undefined) {
      index = readIndex(root)
      cached.set(root, index)
    }
    const pages = index.pages
    if (pages === null) {
      warn("no git history available, falling back to frontmatter dates")
    }

    return [
      () => (_tree: Root, file: VFile) => {
        if (pages === null) return

        const filePath = file.data.filePath
        if (filePath === undefined) return

        const relative = path.relative(root, path.resolve(filePath)).split(path.sep).join("/")
        const page = pages[relative]
        if (page === undefined) {
          // A draft not committed yet. Saying so beats silently dropping the
          // count, which reads as a broken build while writing locally, and
          // the dates from the frontmatter/filesystem fallback still stand.
          file.data.versions = 0
          return
        }

        file.data.versions = page.versions
        if (page.historyUrl !== undefined) file.data.historyUrl = page.historyUrl

        const modified = new Date(page.modified)
        file.data.dates = {
          created: new Date(page.created),
          modified,
          published: file.data.dates?.published ?? modified,
        }
      },
    ]
  },
})

export default GitHistory
