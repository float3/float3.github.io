/**
 * Styles for the thread, the compose box, and the marks the script leaves in
 * the prose where a comment quotes it.
 *
 * Kept next to the component rather than in `custom.scss` because the whole
 * plugin is meant to be liftable into another Quartz site in one directory.
 */

export const styles = `
.comments {
  border-top: 1px solid var(--lightgray);
  margin-top: 3rem;
  padding-top: 1.5rem;
}

.comments-heading {
  align-items: baseline;
  display: flex;
  gap: 0.5rem;
  font-size: 1.2rem;
  margin: 0 0 1rem;
}

.comment-count {
  background: var(--lightgray);
  border-radius: 999px;
  color: var(--darkgray);
  font-size: 0.75rem;
  font-weight: 400;
  padding: 0.1em 0.6em;
}

.comments-empty {
  color: var(--gray);
  font-size: 0.9rem;
  margin: 0 0 1.5rem;
}

.comment-list {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
  list-style: none;
  margin: 0 0 2rem;
  padding: 0;
}

/* Replies indent from the left edge and carry a rule, so the nesting reads
   without relying on the indentation alone at narrow widths. */
.comment-replies {
  border-left: 2px solid var(--lightgray);
  margin: 1rem 0 0;
  padding-left: 1rem;
}

.comment {
  scroll-margin-top: 2rem;
}

/* A comment arrived at by its permalink flashes rather than sits highlighted:
   the marker is meant to answer "which one" and then get out of the way. */
.comment:target {
  animation: comment-found 1.6s ease-out;
}

@keyframes comment-found {
  from {
    background: var(--highlight);
  }
  to {
    background: transparent;
  }
}

.comment-head {
  align-items: center;
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin-bottom: 0.4rem;
}

.comment-avatar {
  border-radius: 50%;
  height: 32px;
  object-fit: cover;
  width: 32px;
}

.comment-avatar-empty {
  align-items: center;
  background: var(--lightgray);
  color: var(--darkgray);
  display: flex;
  font-size: 0.9rem;
  justify-content: center;
  user-select: none;
}

.comment-author {
  font-weight: 600;
}

/* Links to the GitHub profile, but reads as a name rather than as a link. */
a.comment-author {
  color: inherit;
  text-decoration: none;
}

a.comment-author:hover {
  color: var(--secondary);
  text-decoration: underline;
}

/* Only ever seen locally, on a comment file that is not committed yet. */
.comment-author.is-pending {
  color: var(--gray);
  font-style: italic;
  font-weight: 400;
}

.comment-date,
.comment-permalink {
  color: var(--gray);
  font-size: 0.8rem;
}

.comment-permalink {
  margin-left: auto;
  opacity: 0;
  text-decoration: none;
  transition: opacity 150ms ease;
}

.comment:hover .comment-permalink,
.comment-permalink:focus-visible {
  opacity: 1;
}

.comment-quote {
  border-left: 3px solid var(--secondary);
  color: var(--darkgray);
  font-size: 0.9rem;
  margin: 0 0 0.5rem;
  padding: 0.2rem 0 0.2rem 0.75rem;
}

.comment-quote-text::before,
.comment-quote-draft::before {
  content: "\\201c";
}

.comment-quote-text::after,
.comment-quote-draft::after {
  content: "\\201d";
}

.comment-quote-jump {
  font-size: 0.75rem;
  margin-left: 0.5rem;
  white-space: nowrap;
}

/* Until the script has located the passage there is nothing to jump to, and a
   link that scrolls nowhere is worse than a line of plain text. */
.comment-quote-jump[data-inactive="true"] {
  display: none;
}

.comment-body > :first-child {
  margin-top: 0;
}

.comment-body > :last-child {
  margin-bottom: 0;
}

.comment-tools {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin-top: 0.5rem;
}

/* The revision list is a footnote to the comment, not a feature of it: closed,
   quiet, and only present at all once there is more than one version. */
.comment-history {
  font-size: 0.78rem;
  margin-top: 0.4rem;
}

.comment-history summary {
  color: var(--gray);
  cursor: pointer;
}

.comment-history ol {
  color: var(--darkgray);
  margin: 0.3rem 0 0;
  padding-left: 1.2rem;
}

.comment-history-diffs {
  display: inline-block;
  margin-top: 0.3rem;
}

.comment-editing {
  border-left: 3px solid var(--tertiary);
  padding-left: 0.75rem;
}

.comment-editing-note {
  font-size: 0.85rem;
}

/* Red because losing what you just wrote is the failure it prevents, and it
   sits against the button rather than at the top of the box so it cannot be
   scrolled past on the way to pressing it. */
.comment-fork-warning {
  border: 1px solid #b4553f;
  border-radius: 4px;
  background: color-mix(in srgb, #b4553f 10%, transparent);
  font-size: 0.82rem;
  line-height: 1.5;
  padding: 0.6rem 0.75rem;
}

.comment-fork-warning[hidden] {
  display: none;
}

.comment-fork-warning p {
  margin: 0;
}

.comment-fork-warning ol {
  margin: 0.4rem 0 0;
  padding-left: 1.2rem;
}

.comment-fork-warning li + li {
  margin-top: 0.15rem;
}

/* ------------------------------------------------------------------ */
/* The post button, split: the action, and the choice of action. */

.comment-post {
  display: inline-flex;
  position: relative;
}

/* The two halves are one control, so only the outer corners round and the
   seam between them is a single shared line rather than two borders meeting. */
.comment-post .comment-submit {
  border-radius: 4px 0 0 4px;
}

.comment-post-toggle {
  align-items: center;
  background: var(--secondary);
  border-color: var(--secondary);
  border-left: 1px solid color-mix(in srgb, var(--light) 35%, transparent);
  border-radius: 0 4px 4px 0;
  color: var(--light);
  display: flex;
  margin-left: -1px;
  padding: 0.3rem 0.5rem;
}

.comment-post-toggle:hover {
  background: var(--dark);
  border-color: var(--dark);
  color: var(--light);
}

/* Disabled as a pair: the caret opening a menu of things that cannot be done
   yet would be a menu that lies about what pressing it achieves. */
.comment-post:has(.comment-submit[aria-disabled="true"]) .comment-post-toggle {
  cursor: not-allowed;
  filter: grayscale(1);
  opacity: 0.5;
  pointer-events: none;
}

.comment-post-menu {
  background: var(--light);
  border: 1px solid var(--lightgray);
  border-radius: 6px;
  box-shadow: 0 6px 24px rgb(0 0 0 / 25%);
  display: flex;
  flex-direction: column;
  left: 0;
  min-width: min(24rem, 80vw);
  position: absolute;
  top: calc(100% + 0.35rem);
  z-index: 6;
}

.comment-post-menu[hidden] {
  display: none;
}

.comment-post-option {
  background: none;
  border: none;
  border-bottom: 1px solid var(--lightgray);
  color: inherit;
  cursor: pointer;
  display: flex;
  font: inherit;
  gap: 0.5rem;
  padding: 0.6rem 0.7rem;
  text-align: left;
}

.comment-post-option:last-child {
  border-bottom: none;
}

.comment-post-option:hover,
.comment-post-option:focus-visible {
  background: var(--highlight);
  outline: none;
}

/* The tick keeps its column whether or not it is showing, so the titles line
   up down the menu instead of shifting by a character. */
.comment-post-check {
  color: var(--secondary);
  flex: none;
  visibility: hidden;
  width: 1em;
}

.comment-post-option[aria-checked="true"] .comment-post-check {
  visibility: visible;
}

.comment-post-text {
  display: flex;
  flex-direction: column;
  gap: 0.15rem;
}

.comment-post-title {
  font-weight: 600;
}

.comment-post-blurb {
  color: var(--darkgray);
  font-size: 0.78rem;
  line-height: 1.4;
}

/* Behind the same fold as the file they act on. */
.comment-preview-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin-top: 0.5rem;
}

/* A comment body may contain the author's own HTML, which means it may contain
   a wide block or a long unbroken string. Neither gets to widen the page. */
.comment-body {
  max-width: 100%;
  overflow-wrap: break-word;
}

.comment-body pre,
.comment-body table {
  max-width: 100%;
  overflow-x: auto;
}

/* ------------------------------------------------------------------ */
/* Running a comment's code */

.comment-runner {
  align-items: center;
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin-top: 0.5rem;
}

/* The frame is the only child that wants the full width, and it only exists
   once the reader has pressed the button. */
.comment-stage {
  flex-basis: 100%;
}

.comment-stage iframe {
  background: var(--light);
  border: 1px solid var(--gray);
  border-radius: 4px;
  display: block;
  height: 22rem;
  width: 100%;
}

/* Reads as an action rather than as a warning: the sandbox is what makes this
   safe enough to offer, so saying so should not look like a disclaimer. */
.comment-run {
  font-weight: 600;
}

/* ------------------------------------------------------------------ */
/* The passage a comment quotes, marked up in the prose by the script. */

mark.comment-mark {
  background: var(--textHighlight);
  border-radius: 2px;
  color: inherit;
  padding: 0.05em 0;
  scroll-margin-top: 2rem;
}

/* One per comment, so a passage two people quoted carries two markers rather
   than one arrow pointing at whichever of them was found first. */
.comment-backlink {
  color: var(--secondary);
  font-size: 0.7em;
  margin-left: 0.15em;
  text-decoration: none;
  vertical-align: super;
}

.comment-backlink:hover {
  text-decoration: underline;
}

/* ------------------------------------------------------------------ */
/* Compose box */

.comment-composer {
  background: var(--lightgray);
  border-radius: 6px;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  padding: 1rem;
}

.comment-composer-heading {
  font-size: 1rem;
  margin: 0;
}

.comment-composer-note,
.comment-hint {
  color: var(--darkgray);
  font-size: 0.8rem;
  margin: 0;
}

.comment-field {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.comment-label {
  font-size: 0.8rem;
  font-weight: 600;
  letter-spacing: 0.02em;
  text-transform: lowercase;
}

/* Width is left to the flex column that holds it, which stretches its children.
   A textarea carries an intrinsic width from its cols attribute, and pinning it
   to 100% instead measured against the content box while its own padding and
   border sat outside that, so it hung over the edge of the composer. */
.comment-input {
  background: var(--light);
  border: 1px solid var(--gray);
  border-radius: 4px;
  color: var(--dark);
  font-family: inherit;
  font-size: 0.9rem;
  padding: 0.4rem 0.5rem;
  width: auto;
}

.comment-text {
  font-family: var(--codeFont);
  resize: vertical;
}

.comment-input:focus-visible {
  border-color: var(--secondary);
  outline: none;
}

.comment-quoting,
.comment-replying,
.comment-actions {
  align-items: center;
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.comment-button {
  background: var(--light);
  border: 1px solid var(--gray);
  border-radius: 4px;
  color: var(--dark);
  cursor: pointer;
  font-family: inherit;
  font-size: 0.8rem;
  padding: 0.3rem 0.7rem;
  text-decoration: none;
}

.comment-button:hover {
  border-color: var(--secondary);
  color: var(--secondary);
}

.comment-submit {
  background: var(--secondary);
  border-color: var(--secondary);
  color: var(--light);
  font-weight: 600;
}

.comment-submit:hover {
  background: var(--dark);
  border-color: var(--dark);
  color: var(--light);
}

/* Disabled rather than hidden: the button is the explanation of what the form
   is for, and it should stay visible while the reason it cannot fire is shown
   next to it. */
.comment-submit[aria-disabled="true"] {
  cursor: not-allowed;
  filter: grayscale(1);
  opacity: 0.5;
  pointer-events: none;
}

.comment-preview summary {
  color: var(--darkgray);
  cursor: pointer;
  font-size: 0.8rem;
}

.comment-preview-body {
  background: var(--light);
  border-radius: 4px;
  font-size: 0.75rem;
  margin: 0.5rem 0 0;
  max-height: 20rem;
  overflow: auto;
  padding: 0.5rem;
  white-space: pre-wrap;
  word-break: break-word;
}

/* One status line for both halves of the job: why the button will not fire,
   and what happened when one of the other two did. Only the first is a
   complaint, so only the first is coloured like one. */
.comment-error {
  color: #b4553f;
  font-size: 0.85rem;
  margin: 0;
}

.comment-error[data-tone="note"] {
  color: var(--darkgray);
}

.comment-selection-toolbar {
  position: fixed;
  z-index: 5;
}

.comment-selection-toolbar .comment-button {
  box-shadow: 0 2px 8px rgb(0 0 0 / 25%);
}

@media (max-width: 600px) {
  /* Anchored to the button's left edge and capped at the viewport. Hanging it
     off the right edge instead put it off the left of a narrow screen, because
     the button it is measuring from is only as wide as its own label. */
  .comment-post-menu {
    left: 0;
    max-width: calc(100vw - 2.5rem);
    min-width: 0;
    right: auto;
    width: max-content;
  }
}
`
