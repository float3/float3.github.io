---
parent: "_index.md"
date: "2026-08-24T13:10:00.000Z"
author: "float3"
authorId: 86748455
history:
  - date: "2026-08-24T13:10:00.000Z"
---

Comments here can carry a `<script>`, so here is DOOM in my comment section.

<div id="doom"></div>

<script>
  ;(() => {
    /*
     * A comment runs in an iframe with `allow-scripts` and no
     * `allow-same-origin`, which puts it in an opaque origin: every request it
     * makes is cross-origin, this site's own `/js` included, and arrives
     * bearing `Origin: null`. So the binary comes from jsDelivr, which answers
     * that with `Access-Control-Allow-Origin: *` and serves it as
     * `application/wasm` -- the two things `instantiateStreaming` wants.
     */
    const WASM_URL = "https://cdn.jsdelivr.net/npm/wasm-doom@1.2.0/wasm/doom.wasm"

    /* This build renders at twice DOOM's own 320x200 and hands back a single
     * RGBA buffer of that size. Read it as 320x200 and it overruns the
     * ImageData four times over. */
    const WIDTH = 640
    const HEIGHT = 400

    const container = document.getElementById("doom")

    /* One game per document, whatever happens. `once` on the click listener
     * covers a second press, but not this script being evaluated twice -- and
     * a second DOOM is not merely a second picture: both would read the same
     * keyboard and both would hold 6.9 MB of wasm memory open. */
    if (!container || container.dataset.doom === "on") return
    container.dataset.doom = "on"

    const fail = (message) => {
      container.replaceChildren(
        Object.assign(document.createElement("p"), { textContent: message }),
      )
    }

    /** Browser keyCodes to the codes DOOM's own event queue reads. */
    const doomKey = (code) => {
      switch (code) {
        case 8:
          return 127 // backspace
        case 17:
          return 157 // ctrl, fire
        case 18:
          return 184 // alt, strafe
        case 37:
          return 172 // left
        case 38:
          return 173 // up
        case 39:
          return 174 // right
        case 40:
          return 175 // down
        default:
          if (code >= 65 && code <= 90) return code + 32 // A-Z to a-z
          if (code >= 112 && code <= 123) return code + 75 // F1-F12
          return code
      }
    }

    const start = async (button) => {
      button.disabled = true
      button.textContent = "loading…"

      const canvas = document.createElement("canvas")
      canvas.className = "doom-screen"
      canvas.width = WIDTH
      canvas.height = HEIGHT
      canvas.tabIndex = 0

      const context = canvas.getContext("2d")
      if (!context) return fail("this browser has no 2d canvas")
      const frame = context.createImageData(WIDTH, HEIGHT)

      /* The module imports its memory rather than exporting one, so it is
       * built out here and handed in. 108 pages is what the binary was linked
       * against; give it fewer and instantiation fails outright. */
      const memory = new WebAssembly.Memory({ initial: 108 })

      /* Set for real just before `main()`; until then the engine is not
         running and nothing asks the time. */
      let epoch = performance.now()

      const ignore = () => {}
      const imports = {
        js: {
          js_console_log: ignore,
          js_stdout: ignore,
          js_stderr: ignore,
          /* Called once a frame with a pointer into that memory. The view is
           * made fresh each time: growing the memory detaches the old buffer,
           * and a stale view onto it reads as zeroes. */
          js_draw_screen: (pointer) => {
            frame.data.set(new Uint8ClampedArray(memory.buffer, pointer, WIDTH * HEIGHT * 4))
            context.putImageData(frame, 0, 0)
          },
          /* Milliseconds since the *game* started, which is what the name
           * promises and not what `performance.now()` measures. The clock in
           * here starts when the frame loads, and `main()` does not run until
           * 6.8 MB later, so passing it straight through tells DOOM that a
           * minute has already gone by. Its screen melts between the title,
           * the credits and the menu are driven off those tics, so the wipe
           * never advances and all of them end up on screen at once. */
          js_milliseconds_since_start: () => performance.now() - epoch,
        },
        env: { memory },
      }

      let doom
      try {
        doom = await WebAssembly.instantiateStreaming(fetch(WASM_URL), imports)
      } catch (error) {
        return fail(`could not load doom: ${error}`)
      }

      button.remove()
      container.append(canvas)
      canvas.focus()

      const exports = doom.instance.exports

      /* 0 is a press and 1 a release. Bound to this frame's own window rather
       * than to the canvas: the frame is nothing but the game, so anything
       * typed into it is meant for the game, and the arrows and space have to
       * stop scrolling it either way. */
      const send = (kind) => (event) => {
        exports.add_browser_event(kind, doomKey(event.keyCode))
        event.preventDefault()
      }
      addEventListener("keydown", send(0))
      addEventListener("keyup", send(1))

      epoch = performance.now()
      exports.main()

      const step = () => {
        exports.doom_loop_step()
        requestAnimationFrame(step)
      }
      requestAnimationFrame(step)
    }

    const button = document.createElement("button")
    button.type = "button"
    button.className = "doom-start"
    button.textContent = "play doom (6.8 MB)"
    button.addEventListener("click", () => void start(button), { once: true })
    container.append(button)
  })()
</script>

<style>
  /* No margin: the frame sizes itself to what is in here, so a margin on the
     only child is just a gap inside the box. */
  #doom { line-height: 0; }
  .doom-start { font: inherit; line-height: normal; padding: 0.4rem 0.9rem; }
  .doom-screen { display: block; height: auto; image-rendering: pixelated; width: 100%; }
  .doom-screen:focus-visible { outline: 2px solid #4a8fd6; outline-offset: -2px; }
</style>
