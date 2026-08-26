# elm-graph

The site graph, drawn in Elm.

Quartz's graph is a community plugin, `github:quartz-community/graph`. It draws
a good picture, and to draw it the page fetches **d3 and pixi.js from jsDelivr**
— about a megabyte and a half of somebody else's JavaScript, from a third party,
on every page view — and paints into a WebGL canvas. Its script opens with
`@ts-nocheck`, so none of it is type-checked, and the reader's IP address goes
to a CDN before a single node is drawn.

This draws the same picture out of SVG, from a compiled Elm program served from
this site. **59 KB, 20 KB over the wire**, fetched only on a page that has a
graph on it.

## What is where

| file                          | what it is                                                                        |
| ----------------------------- | --------------------------------------------------------------------------------- |
| `src/Main.elm`                | the graph: neighbourhood, force simulation, SVG, hover, click                     |
| `components.ts`               | the Quartz component — two empty containers and the button that opens the big one |
| `styles.ts`                   | what a node, a link and a label look like in each theme                           |
| `../../ts/src/graph.ts`       | the page's side: the content index, the box's size, visited pages, navigation     |
| `../../tools/site/src/elm.rs` | `site elm`, which compiles `Main.elm` into `content/js/elm.js`                    |

Nothing here is hand-compiled: `site build` and `site wasm` both run `site elm`,
and `content/js` is a build output.

## The layout

The forces are d3-force's, deliberately, so the graph settles into the shape the
old one settled into: charge at `-100 × repelForce`, links pulling towards
`linkDistance` with d3's own strength and bias, a centring force that moves
positions rather than velocities, one pass of collision, and the same alpha
decay of 0.0228 per tick down to 0.001.

Two things are not d3's:

- **Nothing is random.** d3 seeds positions with `Math.random`; this uses the
  phyllotaxis spiral d3 falls back to for a node with no position, so the same
  page draws the same graph every time it is opened.
- **The first layout is run before anything is drawn**, against a budget of
  about 110,000 node-pair steps. A simulation that starts from its seed
  positions spends its first second looking like an explosion, which in a
  sidebar is movement for its own sake; and a graph in a background tab gets no
  animation frames at all, so without this it would sit there as a spiral of
  untouched seed positions. The budget is what keeps that cheap: a step compares
  every node with every other, so the whole-site graph — 94 nodes against the
  sidebar's three — gets twelve steps where the sidebar gets its full three
  hundred and settles completely.

A reader who has asked for `prefers-reduced-motion` gets the layout settled and
still, rather than performed at them.

## Colour

There is none in the Elm. A node is a `circle` with `is-current`, `is-tag`,
`is-visited`, `is-hovered` or `is-dim` on it, and `styles.ts` says what those
mean in `var(--secondary)` and friends. The graph this replaced read six custom
properties out of `getComputedStyle` when it started and painted them into a
WebGL scene, which is why it had to be told to rebuild itself when the theme
changed; this one is repainted by the browser like anything else on the page.

## The four ports

Elm owns the graph and nothing else. Everything that is the page's job crosses a
port:

| port      | direction | what it carries                                                        |
| --------- | --------- | ---------------------------------------------------------------------- |
| `follow`  | out       | the id of a clicked node, which the page hands to Quartz's router      |
| `failed`  | out       | why flags could not be read, for the console                           |
| `resized` | in        | where the container is and how big, from a `ResizeObserver` and scroll |
| `halt`    | in        | stop: this app's view has been patched out of the page                 |

`resized` carries the corner as well as the size because a mouse event says
where it happened in the window, and the graph works in the units of its own
drawing. An Elm event decoder can read properties off an event but cannot call
`getBoundingClientRect`, so the page measures and sends. Scrolling moves the
corner without changing the size, which is why the page watches for that too:
without it, zooming a graph the reader had scrolled to would zoom about the
wrong point.

`halt` exists because of how Quartz navigates. It patches the document rather
than replacing it, so after a soft navigation the container is the same element
— with the drawing taken out of it by micromorph, because the incoming page's
HTML has an empty one. The app rendering into it is left holding a view that is
no longer in the page, so `graph.ts` stops it and starts another around the page
the reader has arrived at.

Clicks are the other place the two sides meet. A node is an SVG `<a href>`, so
it can be focused and read as a link — but Quartz's router reads `href` off
whatever anchor a click came from, and on an SVG anchor that is an
`SVGAnimatedString` rather than a string. The anchor carries `data-router-ignore`
to keep the router's hands off it, and Elm sends the id out through `follow`
instead, which the page turns into a URL the router can read.

## The mouse

Dragging a node pins it: the forces go on pushing at it, it does not move, and
everything else rearranges itself around where it has been put. Letting go
unpins it. A node is also a link, so the click that ends a drag would otherwise
follow it — the drag remembers whether the mouse travelled more than three
pixels, and the click after one that did is swallowed.

Dragging the background pans, and the wheel zooms about the pointer, between a
quarter and four times, on d3's own curve of two to the power of the delta over
five hundred. Both are the `viewBox` and nothing else: no transform on the
nodes, no second coordinate system, and a stylesheet that goes on measuring
strokes and font sizes in the units it was written in. Labels fade in above
life size, which is the old graph's arithmetic, so that zooming into a corner
is what makes it readable rather than fifty labels at once.

## What is not ported

Touch. The sidebar graph is desktop-only and the whole-site one is opened from
it, so this listens for mouse events rather than pointer events, and there is
no pinch. `scale` is gone for good: it was a pixi camera setting with nothing
to configure in an SVG.

## Tests

```sh
cargo run --locked --manifest-path tools/site/Cargo.toml -- elm-test
```

Forty-nine of them, and `site check` runs them. They cover the half of this
that a screenshot cannot show: that a link settles at about the length it asked
for, that the neighbourhood walks a link in both directions, that a drag
carries a node by exactly the distance the mouse moved and half that at twice
the zoom, that the wheel keeps whatever is under the pointer under the pointer,
and that the click ending a drag is not a link being followed.

Elm will not let a test look inside a `Cmd`, so where a message's only job is to
send one, what is checked is the state that decides whether it is sent.

## Building it

```sh
cargo run --locked --manifest-path tools/site/Cargo.toml -- elm --prod
```

`--dev` skips `--optimize` and the esbuild pass, so the JavaScript in the
debugger is the JavaScript on disk. The compiler and the test runner are
`elmPackages.elm` and `elmPackages.elm-test` in the flake's dev shell; on a
machine without nix, `site` reaches for elm-test through `bun x` at the pinned
`0.19.1-revision12`, because plain `latest` is built for a compiler that is not
out yet. `elm-stuff/` beside this file is the package cache and is ignored.
