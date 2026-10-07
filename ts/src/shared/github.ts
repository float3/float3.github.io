/**
 * The GitHub issue a page opens on the reader's behalf.
 *
 * A comment and a gallery submission both travel as an issue: the page
 * prefills one, the reader presses Create, and a workflow in the repository
 * reads it back. What the workflow looks for is a marker in an HTML comment,
 * so GitHub renders the issue as just what the reader wrote with the
 * machine-readable part invisible. The workflow keys off the marker rather
 * than off a label, because a label set through `?labels=` is silently
 * dropped for anyone without triage permission on the repository, which is
 * everyone this is for.
 */

/** The lines that carry a marker and its payload, one line each. */
export function markerLines(marker: string, payload: unknown): string[] {
  return [`<!--${marker}`, JSON.stringify(payload), "-->"]
}

/** GitHub's new-issue page for `owner/repo`, prefilled with `fields`. */
export function newIssueUrl(repo: string, fields: Record<string, string>): string {
  return `https://github.com/${repo}/issues/new?${new URLSearchParams(fields).toString()}`
}
