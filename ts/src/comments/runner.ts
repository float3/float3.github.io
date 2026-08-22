/**
 * Running a comment's code, in a box it cannot get out of.
 *
 * The document was assembled at build time and shipped inert in a `data-run`
 * attribute. All this does is copy it into an iframe's `srcdoc` when asked, and
 * throw the iframe away when asked again.
 *
 * The sandbox list is the important line in this file. `allow-scripts` without
 * `allow-same-origin` puts the frame in an opaque origin: same-origin reads
 * against the parent fail, `localStorage` and `document.cookie` are not the
 * site's, and there is no handle back to this window. Adding
 * `allow-same-origin` alongside `allow-scripts` would undo all of that in one
 * word, which is why it is spelled out here rather than left to be inferred.
 */

const SANDBOX = [
  // The point of the feature.
  "allow-scripts",
  // `alert` and `prompt`, which half of all small demos are built on.
  "allow-modals",
  // Deliberately absent: allow-same-origin, allow-top-navigation,
  // allow-popups, allow-forms, allow-downloads, allow-pointer-lock.
].join(" ")

function frameFor(document_: string): HTMLIFrameElement {
  const frame = document.createElement("iframe")
  frame.setAttribute("sandbox", SANDBOX)
  frame.setAttribute("referrerpolicy", "no-referrer")
  // Emphatically not `loading="lazy"`: the frame is created by a click, and a
  // comment far enough down the page to be worth scrolling to is exactly the
  // one a lazy frame would refuse to start until it drifted into view.
  frame.title = "output of this comment's code"
  frame.srcdoc = document_
  return frame
}

/**
 * Wires one run button.
 *
 * Returns a teardown, because a navigation must take any running frame with
 * it — a demo left spinning in a `requestAnimationFrame` loop on a page the
 * reader has left is a bug that only shows up as a warm laptop.
 */
export function wireRunner(button: HTMLElement, onToggle: () => void): () => void {
  const stage = button.parentElement?.querySelector<HTMLElement>(".comment-stage")
  const source = button.dataset.run
  if (stage === undefined || stage === null || source === undefined) return () => {}

  const label = button.textContent ?? "run this"

  const stop = () => {
    stage.replaceChildren()
    button.textContent = label
  }

  const click = () => {
    if (stage.firstChild !== null) {
      stop()
    } else {
      stage.append(frameFor(source))
      button.textContent = "stop"
    }
    onToggle()
  }

  button.addEventListener("click", click)

  return () => {
    button.removeEventListener("click", click)
    stop()
  }
}
