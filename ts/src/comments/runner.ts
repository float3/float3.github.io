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

  const label = button.textContent ?? "run it"

  const stop = () => {
    stage.replaceChildren()
    button.textContent = label
  }

  const start = () => {
    if (stage.firstChild !== null) return
    stage.append(frameFor(source))
    button.textContent = "stop"
  }

  const click = () => {
    if (stage.firstChild !== null) stop()
    else start()
    onToggle()
  }

  button.addEventListener("click", click)

  // Started on sight rather than on a click: the reader should find the thing
  // running, not a button promising it. Starting only what is on screen is what
  // keeps a page of these from opening every frame at once.
  const MARGIN = 128

  const onScreen = () => {
    const box = button.getBoundingClientRect()
    return box.top < window.innerHeight + MARGIN && box.bottom > -MARGIN
  }

  const begin = () => {
    if (stage.firstChild !== null) return
    start()
    onToggle()
  }

  // Measured directly for what is already in view, rather than waiting to be
  // told. An observer is the right tool for the comment further down the page,
  // but it reports nothing at all while the document is not being rendered, and
  // a game that only runs in a foreground tab is a game that sometimes does not
  // run. The measurement works either way.
  if (onScreen()) begin()

  const watcher = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) {
        if (!entry.isIntersecting) continue
        begin()
        watcher.disconnect()
      }
    },
    { rootMargin: `${MARGIN}px` },
  )
  // The button, not the stage: the stage is an empty div until something is put
  // in it, and an element with no height has no intersection to report — so
  // observing it would mean waiting for a frame that only arrives once the
  // frame is already there.
  watcher.observe(button)

  return () => {
    watcher.disconnect()
    button.removeEventListener("click", click)
    stop()
  }
}
