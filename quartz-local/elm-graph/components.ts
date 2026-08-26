/**
 * The box the Elm graph is drawn in.
 *
 * Everything here is empty: two containers carrying the settings the graph
 * should be built with, and a button that opens the second one. `/js/graph.js`
 * reads the content index and starts `/js/elm.js` on them -- see
 * `src/Main.elm` for the graph itself, and the README for why it is in Elm.
 *
 * The containers are empty on purpose rather than by accident of being drawn
 * later: with the script blocked there is no half-graph and no empty framed
 * box, because the CSS gives the box its border only once something is in it.
 */

import { h } from "preact"
import type {
  QuartzComponent,
  QuartzComponentConstructor,
  QuartzComponentProps,
} from "../../quartz/components/types"
import { classNames } from "@quartz-community/utils"
import { styles } from "./styles"

/**
 * The knobs the old graph had, kept name for name so that a config written
 * against it means the same thing here. Two are gone: `drag` and `zoom`, which
 * this does not do yet, and `scale`, which was a pixi camera setting.
 */
export interface GraphConfig {
  /** How many links away from this page to draw. Negative draws the site. */
  depth: number
  repelForce: number
  centerForce: number
  linkDistance: number
  /** Dim everything that is not the hovered node or a neighbour of it. */
  focusOnHover: boolean
  /** Pull the nodes onto a ring, which keeps the site graph from knotting. */
  enableRadial: boolean
  showTags: boolean
  removeTags: string[]
}

export interface GraphOptions {
  localGraph?: Partial<GraphConfig>
  globalGraph?: Partial<GraphConfig>
}

const local: GraphConfig = {
  depth: 1,
  repelForce: 0.5,
  centerForce: 0.3,
  linkDistance: 30,
  focusOnHover: false,
  enableRadial: false,
  showTags: true,
  removeTags: [],
}

const global: GraphConfig = {
  depth: -1,
  repelForce: 0.5,
  centerForce: 0.2,
  linkDistance: 30,
  focusOnHover: true,
  enableRadial: true,
  showTags: true,
  removeTags: [],
}

/** Three nodes and two links: the picture the button opens, in 16 pixels. */
const icon = h("svg", { viewBox: "0 0 16 16", width: "16", height: "16", "aria-hidden": "true" }, [
  h("line", { x1: "4", y1: "12", x2: "8", y2: "4", stroke: "currentColor", "stroke-width": "1" }),
  h("line", { x1: "8", y1: "4", x2: "12", y2: "11", stroke: "currentColor", "stroke-width": "1" }),
  h("circle", { cx: "8", cy: "3.5", r: "2.5", fill: "currentColor" }),
  h("circle", { cx: "3.5", cy: "12.5", r: "2.5", fill: "currentColor" }),
  h("circle", { cx: "12.5", cy: "11.5", r: "2.5", fill: "currentColor" }),
])

export const ElmGraph: QuartzComponentConstructor<GraphOptions> = (opts) => {
  const Component: QuartzComponent = ({ displayClass }: QuartzComponentProps) => {
    const near = JSON.stringify({ ...local, ...opts?.localGraph })
    const whole = JSON.stringify({ ...global, ...opts?.globalGraph })

    return h("div", { class: classNames(displayClass, "elm-graph") }, [
      h("h3", null, "Graph"),
      h("div", { class: "elm-graph-outer" }, [
        h("div", { class: "elm-graph-container", "data-cfg": near }),
        h(
          "button",
          { type: "button", class: "elm-graph-expand", "aria-label": "Graph of the whole site" },
          icon,
        ),
      ]),
      h(
        "div",
        { class: "elm-graph-modal", hidden: true },
        h("div", { class: "elm-graph-container is-global", "data-cfg": whole }),
      ),
    ])
  }

  Component.css = styles
  return Component
}

export default ElmGraph
