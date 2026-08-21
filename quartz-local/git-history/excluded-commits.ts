/**
 * Commits that should not count as edits to a page.
 *
 * A sweep that touches thirty files to fix one typo, rename a directory or
 * restyle frontmatter is one act of maintenance, not thirty revisions. Left in,
 * these dominate the version counts and drag every page's "updated" date to
 * whichever day I last ran a find-and-replace, which makes both numbers say
 * nothing about the page they sit under.
 *
 * Bot commits are already dropped by author in `index.ts`. This list is only
 * for sweeps I made by hand.
 *
 * Two things this deliberately does not do. It never changes a page's "created"
 * date — a page added by a migration was still created then, whatever else the
 * commit touched. And it is a list of specific commits rather than a heuristic
 * on file count or diff size, because "broad and mechanical" is a judgement
 * about intent that a threshold gets wrong in both directions.
 */

export type CommitExclusion = {
  /** Full 40-character SHA. Abbreviations are not matched. */
  sha: string
  /**
   * The same patch under a second SHA.
   *
   * Most of this history exists twice: a rebased lineage was merged back, so
   * 1546 of 1858 commits have an identical-patch twin. Excluding one of a pair
   * leaves the other doing all the damage by itself.
   */
  twin?: string
  /** What the sweep did, so a later reader can re-judge it. */
  reason: string
  /**
   * Paths this commit genuinely changed, which go on counting, for a sweep
   * that also did one piece of real work in passing.
   *
   * These are the file's path *today*, not its path in the commit: history is
   * threaded through renames before this is consulted. A path that matches
   * nothing is reported at build time rather than silently excluding.
   */
  keep?: string[]
}

export const EXCLUDED_COMMITS: CommitExclusion[] = [
  // --- frontmatter and metadata sweeps -----------------------------------
  {
    sha: "2ab131a745f52253618676431127601540806e18",
    reason: "drop date frontmatter now that dates come from git history — 31 files, 2 lines each",
  },
  {
    sha: "576e2bf88528fb7dce8f2941aadabba36bdc9e3b",
    twin: "58f7acff7d67b276ee6efedf4461da7c6539eef4",
    reason: "set every `updated:` to the same day — 14 files, one line each",
  },
  {
    sha: "152fd7f9fa2dac8f19f6453085b5a1047390883b",
    twin: "43d0c5f73a9c27aa276471b0ec18fe3d8d708e21",
    reason: "add `tags:` frontmatter — 18 files",
    // This one also added a Misc section and a link while it was in there.
    keep: ["content/notes/blogs.md"],
  },
  {
    sha: "6d3a7f8604f6c2fa717eeee84e06dc72f7c26d7c",
    twin: "39b09b2cd4852b5bf6aebc03f9e923f78f7f5778",
    reason: "add a `date:` line to the thoughts pages — 4 files, one line each",
  },
  {
    sha: "f6d7220c04032beba213d6a0dd537ed16879cbb1",
    twin: "9804200369b47f68ea40fbb236e461a18ba75d00",
    reason: "strip date frontmatter and reflow some URLs — 4 files",
  },

  // --- identical edits to a shared line ----------------------------------
  {
    sha: "c967dea69a3ed04611fc690721b362bda51ac256",
    reason: "fix the wasm credit typo on the tool pages — 11 files, the same one-line fix",
  },
  {
    sha: "15de5575ab179efdcbbe1dcbe112c21c028f29d3",
    reason: "reword the wasm credit line on the tool pages — 11 files, the same replacement",
  },

  // --- formatting only ----------------------------------------------------
  {
    sha: "1f85f57fe8066ba88395f947d665d0e5cafa680f",
    reason: "reformat the tag lists — 9 files, no prose changed",
  },
  {
    sha: "36c5660c0cb707d66417d5f3193d8ff915f91719",
    twin: "b04cfee4e8a80229e37793d17370cacb0a9687a6",
    reason: "whitespace reformatting — 19 files, every line removed and re-added identically",
  },

  // --- moves and the link rewrites that followed them ---------------------
  {
    sha: "f26dc1310cb6dd2d715b4ff913a79824f895b60d",
    twin: "62db4fc16f117821e4601107412d2c25dc2d6b20",
    reason: "move blog/ to posts/ and notes/ to thoughts/ — 12 renames plus 3 link fixes",
  },
  {
    sha: "6df46de3ffe2d0f98bc7beea2a5224ccb5fef73e",
    twin: "4d8288f6ea53fde3285089e7e1f44ad63d7591d2",
    reason: "move posts/ back to blog/ — 26 files, almost all pure renames",
  },
  {
    sha: "12e615cf57885be875487cfb90b2468cb8ac8ac1",
    twin: "4119e2a933ef27b40822c464b1083afb9f48c5d4",
    reason: "rewrite tuningplayground/ links to piano/ — 5 files",
  },
  {
    sha: "22632b42b5b924554c928863678c9d69f5ac85a7",
    twin: "d0e08a89924715bc7bac6e2634e5715793db17d7",
    reason: "swap the tool `<script src>` paths after a module rename — 4 files",
  },

  // --- the site migration -------------------------------------------------
  {
    sha: "2624fd57b50c2a3975a8548527dc2c75939fbdb9",
    twin: "bd4ab6a470cf0f8d9f98a487a809e7f64f85a67c",
    reason: "convert to quartz: TOML to YAML frontmatter across everything that existed then",
    // agi.md was written in this commit rather than converted by it. Listed
    // under the name it has now, notes/, not the thoughts/ it had then.
    keep: ["content/notes/agi.md"],
  },
]

const bySha = new Map(
  EXCLUDED_COMMITS.flatMap((exclusion) =>
    exclusion.twin === undefined
      ? [[exclusion.sha, exclusion] as const]
      : [[exclusion.sha, exclusion] as const, [exclusion.twin, exclusion] as const],
  ),
)

/** Whether `sha` should be ignored when counting and dating `file`. */
export function isExcluded(sha: string, file: string): boolean {
  const exclusion = bySha.get(sha)
  if (exclusion === undefined) return false
  return !exclusion.keep?.includes(file)
}

/** Every listed SHA, so the caller can warn about ones history no longer has. */
export function excludedShas(): string[] {
  return [...bySha.keys()]
}

/** Every `keep` path, so the caller can warn about ones that match no file. */
export function excludedKeepPaths(): string[] {
  return EXCLUDED_COMMITS.flatMap((exclusion) => exclusion.keep ?? [])
}
