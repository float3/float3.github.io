/**
 * The questions the build asks git about the repository it is running in.
 *
 * Every answer is optional. A checkout without a remote, a shallow CI fetch
 * that never wrote `origin/HEAD`, or no git at all each leave a plugin with
 * less to show, never with a broken build.
 */

import { execFileSync } from "child_process"

/**
 * One git command's trimmed output, or undefined if it fails.
 *
 * `maxBuffer` is for a log of the whole repository, which outgrows Node's
 * default.
 */
export function git(cwd: string, args: string[], maxBuffer?: number): string | undefined {
  try {
    return execFileSync("git", args, { cwd, encoding: "utf8", maxBuffer }).trim()
  } catch {
    return undefined
  }
}

/** Absolute path of the repository root. */
export const repoRoot = (cwd: string) => git(cwd, ["rev-parse", "--show-toplevel"])

/** The `origin` remote's URL, as written in the config. */
export const originRemote = (cwd: string) => git(cwd, ["remote", "get-url", "origin"])

/**
 * The remote's default branch, not the one checked out: feature branches
 * disappear and would leave links 404ing, and a pull request opened against a
 * branch that only ever existed on one machine helps nobody.
 *
 * CI checkouts often fetch a single ref and never write `origin/HEAD`, which
 * is what the fallback is for.
 */
export function defaultBranch(cwd: string): string {
  const head = git(cwd, ["symbolic-ref", "--short", "refs/remotes/origin/HEAD"])
  return head?.replace(/^origin\//, "") || "master"
}

/** `owner/repo` out of whichever URL form a GitHub remote happens to use. */
export function ownerRepo(remote: string): string | undefined {
  const match = /github\.com[:/]+([^/]+)\/(.+?)(?:\.git)?\/?$/.exec(remote.trim())
  return match ? `${match[1]}/${match[2]}` : undefined
}
