/**
 * The submit button on a gallery page, and the issue it opens.
 *
 * There is no upload here, and no server to upload to. The button opens a
 * prefilled GitHub issue, the reader drops pictures into it, and GitHub — which
 * already does this well, with a progress bar and a size limit and a login —
 * hosts each one and writes a link to it in the body. A workflow reads those
 * links back, adds the files to the gallery, opens a pull request and closes
 * the issue; merging that pull request is what publishes them.
 *
 * The same shape as the comment compose box, for the same reasons: the account
 * is the identity, so there is nothing to type and no key to keep, and the
 * review step is a diff rather than a moderation queue.
 */

/**
 * The marker the workflow looks for.
 *
 * An HTML comment, so GitHub renders the issue as just the pictures with the
 * machine-readable part invisible. The workflow keys off this string rather
 * than off a label, because a label set through `?labels=` is silently dropped
 * for anyone without triage permission on the repository, which is everyone
 * this feature is for.
 */
export const ISSUE_MARKER = "hilll.dev:gallery"

export interface SubmitTarget {
  /** `owner/repo` on GitHub. */
  repo: string
  /** Directory under `content/misc`, e.g. `trolley`. */
  collection: string
  /** What one of them is called: "trolley problem", "entry". */
  noun: string
  /** What the gallery is called, for the issue's title. */
  label: string
}

/**
 * What the reader sees in the compose box before they add anything.
 *
 * GitHub shows the raw markdown while an issue is being written and renders
 * nothing of an HTML comment once it exists, so this is read exactly where it
 * is useful and disappears where it would only be a stale instruction.
 */
function guidance(noun: string): string {
  return (
    `<!-- Drop your ${noun}s in below, under this line: drag the files onto the box, or\n` +
    "     paste them in. GitHub uploads each one and writes a link to it. As many as you\n" +
    "     like in one issue.\n" +
    "\n" +
    "     Then press the green Create button. A workflow adds them and closes this issue;\n" +
    "     they appear on the page once the pull request it opens has been merged. -->"
  )
}

export function buildIssueBody(target: SubmitTarget): string {
  return [
    guidance(target.noun),
    `<!--${ISSUE_MARKER}`,
    JSON.stringify({ collection: target.collection }),
    "-->",
    "",
    "",
  ].join("\n")
}

export function issueUrl(target: SubmitTarget): string {
  const query = new URLSearchParams({
    title: `${target.label} submission`,
    body: buildIssueBody(target),
  })
  return `https://github.com/${target.repo}/issues/new?${query.toString()}`
}

/** "a trolley problem", "an entry". */
export function withArticle(noun: string): string {
  return `${/^[aeiou]/i.test(noun) ? "an" : "a"} ${noun}`
}

/**
 * Puts the button in the gallery's header, beside the count.
 *
 * An anchor rather than a button because it navigates, which is also what makes
 * it work with a middle click and worth hovering over to see where it goes.
 */
export function renderSubmitButton(section: ParentNode, target: SubmitTarget): void {
  const header = section.querySelector<HTMLElement>(".photo-gallery-header")
  if (header === null || header.querySelector(".gallery-submit") !== null) return

  const link = document.createElement("a")
  link.className = "gallery-submit"
  link.href = issueUrl(target)
  link.rel = "noopener"
  link.textContent = `submit ${withArticle(target.noun)}`
  header.append(link)
}
