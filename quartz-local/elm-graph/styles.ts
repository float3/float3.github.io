/**
 * What the graph looks like, in each theme.
 *
 * The Elm side draws circles and lines with classes on them and says nothing
 * about colour. That is the point of the port: the graph this replaced read
 * six custom properties out of `getComputedStyle` when it started and painted
 * them into a WebGL scene, so it had to be told to rebuild itself whenever the
 * theme changed. These are `var(--secondary)` and friends, resolved by the
 * browser, on nodes that are already in the document.
 */

export const styles = `
.elm-graph h3 {
  font-size: 1rem;
  margin: 0 0 0.5rem;
}

.elm-graph-outer {
  position: relative;
  border: 1px solid var(--lightgray);
  border-radius: 5px;
  height: 250px;
  overflow: hidden;
}

.elm-graph-container {
  width: 100%;
  height: 100%;
}

.elm-graph-svg {
  display: block;
  width: 100%;
  height: 100%;
}

/* The background, which is what a drag that misses a node lands on. It has to
   be painted to be hit at all, since fill:none is not hit-tested, so it is
   painted in nothing. */
.elm-graph-field {
  fill: transparent;
  cursor: grab;
}

.elm-graph-svg:active .elm-graph-field {
  cursor: grabbing;
}

.elm-graph-link {
  stroke: var(--lightgray);
  stroke-width: 1px;
  transition:
    stroke 0.2s ease,
    opacity 0.2s ease;
}

.elm-graph-link.is-lit {
  stroke: var(--gray);
}

.elm-graph-link.is-dim {
  opacity: 0.2;
}

.elm-graph-node {
  cursor: pointer;
  transition: opacity 0.2s ease;
}

/* A node being dragged should not also be a piece of text being selected. */
.elm-graph-svg {
  user-select: none;
  touch-action: none;
}

.elm-graph-node circle {
  fill: var(--gray);
  transition: fill 0.2s ease;
}

.elm-graph-node.is-visited circle {
  fill: var(--tertiary);
}

/* A tag is drawn as an outline rather than a disc, so that a page and the tag
   it carries are told apart at a glance without reading either label. */
.elm-graph-node.is-tag circle {
  fill: var(--light);
  stroke: var(--tertiary);
  stroke-width: 1.5px;
}

.elm-graph-node.is-current circle {
  fill: var(--secondary);
}

.elm-graph-node.is-dim {
  opacity: 0.25;
}

.elm-graph-label {
  fill: var(--dark);
  font-family: var(--bodyFont);
  font-size: 8px;
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.2s ease;
}

.elm-graph-node.is-hovered .elm-graph-label {
  opacity: 1;
}

.elm-graph-expand {
  position: absolute;
  right: 0.4rem;
  bottom: 0.4rem;
  display: flex;
  padding: 0.2rem;
  border: none;
  border-radius: 4px;
  background: none;
  color: var(--gray);
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.2s ease;
}

.elm-graph-outer:hover .elm-graph-expand,
.elm-graph-expand:focus-visible {
  opacity: 1;
}

.elm-graph-modal {
  position: fixed;
  inset: 0;
  z-index: 100;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.55);
}

.elm-graph-modal[hidden] {
  display: none;
}

/* No background of its own: the page's own words go on showing through the
   dimming behind it, which is what makes this the site's graph rather than a
   window laid over the top of it. */
.elm-graph-modal .elm-graph-container {
  width: min(85vw, 60rem);
  height: min(85vh, 45rem);
}
`
