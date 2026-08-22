/**
 * Attaches each page's comment thread, and the address of the repository the
 * compose box should open its pull request against.
 *
 * The scan is a single walk of the content directory plus a single `git log`,
 * cached per process, for the same reason GitHistory caches its own: Quartz
 * parses in worker threads, and re-reading every comment file once per page
 * would turn a linear cost into a quadratic one.
 */

import { execFileSync } from "child_process"
import path from "path"
import { styleText } from "util"
import type { Root } from "mdast"
import type { VFile } from "vfile"
import type { QuartzTransformerPlugin } from "../../quartz/plugins/types"
import { scanComments } from "./parse"
import type { CommentRecord } from "./types"

export type { CommentAuthor, CommentRecord, CommentTarget } from "./types"

interface Options {
  /** `owner/repo` on GitHub. Read from the `origin` remote when omitted. */
  repo?: string
  /** Branch pull requests should target. Read from the remote's HEAD when omitted. */
  branch?: string
}

interface Repo {
  repo: string
  branch: string
  /** Absolute path of the repository root. */
  root: string
  /** Where the content directory sits inside it, e.g. `content`. */
  prefix: string
}

/** Pulls `owner/repo` out of whichever URL form the remote happens to use. */
function parseRemote(remote: string): string | undefined {
  const match = /github\.com[:/]+([^/]+)\/(.+?)(?:\.git)?\/?$/.exec(remote.trim())
  return match ? `${match[1]}/${match[2]}` : undefined
}

function readRepo(contentDir: string, options: Options): Repo | undefined {
  const git = (args: string[]): string | undefined => {
    try {
      return execFileSync("git", args, { cwd: contentDir, encoding: "utf8" }).trim()
    } catch {
      return undefined
    }
  }

  const root = git(["rev-parse", "--show-toplevel"])
  if (root === undefined) return undefined

  const repo = options.repo ?? parseRemote(git(["remote", "get-url", "origin"]) ?? "")
  if (repo === undefined) return undefined

  // The remote's default branch, not the checked-out one: a pull request opened
  // against a branch that only ever existed on my machine helps nobody.
  const head = git(["symbolic-ref", "--short", "refs/remotes/origin/HEAD"])

  return {
    repo,
    branch: options.branch ?? head?.replace(/^origin\//, "") ?? "master",
    root,
    prefix: path.relative(root, contentDir).split(path.sep).join("/"),
  }
}

// Quartz parses in worker threads; one cache per process, keyed by content dir.
const cachedThreads = new Map<string, Map<string, CommentRecord[]>>()
const cachedRepos = new Map<string, Repo | undefined>()

export const Comments: QuartzTransformerPlugin<Partial<Options>> = (userOptions) => {
  const options: Options = { ...userOptions }

  return {
    name: "Comments",
    markdownPlugins(ctx) {
      const contentDir = path.resolve(ctx.argv.directory)

      let repo: Repo | undefined
      if (cachedRepos.has(contentDir)) {
        repo = cachedRepos.get(contentDir)
      } else {
        repo = readRepo(contentDir, options)
        cachedRepos.set(contentDir, repo)
        if (repo === undefined) {
          console.log(
            styleText(
              "yellow",
              "\nWarning: no GitHub remote found, so comment threads will render read-only",
            ),
          )
        }
      }

      let threads = cachedThreads.get(contentDir)
      if (threads === undefined) {
        threads = scanComments(contentDir, repo?.root ?? contentDir)
        cachedThreads.set(contentDir, threads)
      }

      return [
        () => (_tree: Root, file: VFile) => {
          const filePath = file.data.filePath
          if (filePath === undefined) return

          const absolute = path.resolve(filePath)
          file.data.comments = threads.get(absolute) ?? []

          if (repo === undefined) return
          const relative = path.relative(contentDir, absolute).split(path.sep).join("/")
          file.data.commentTarget = {
            repo: repo.repo,
            branch: repo.branch,
            path: repo.prefix === "" ? relative : `${repo.prefix}/${relative}`,
            parent: relative,
          }
        },
      ]
    },
  }
}

export default Comments
