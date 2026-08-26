import { DOOM } from "wasm-doom"

/**
 * Copied beside the bundle by `site wasm`. The library would otherwise fetch it
 * from a CDN at read time, and the package's `exports` map will not let webpack
 * import it. 6.8 MB, so nothing asks for it until the button is pressed.
 */
const DOOM_WASM_URL = "/js/doom.wasm"

/**
 * The framebuffer wasm-doom hands back, which is twice DOOM's own 320x200. Read
 * it as 320x200 and `set` overruns the ImageData by four times over.
 */
const DOOM_WIDTH = 640
const DOOM_HEIGHT = 400

function start(container: HTMLElement, button: HTMLButtonElement): void {
  const canvas = document.createElement("canvas")
  canvas.className = "doom-screen"
  canvas.width = DOOM_WIDTH
  canvas.height = DOOM_HEIGHT
  canvas.tabIndex = 0

  const context = canvas.getContext("2d")
  if (!context) {
    button.textContent = "this browser has no 2d canvas"
    return
  }

  const frame = context.createImageData(DOOM_WIDTH, DOOM_HEIGHT)

  button.remove()
  container.append(canvas)
  canvas.focus()

  const doom = new DOOM({
    screenWidth: DOOM_WIDTH,
    screenHeight: DOOM_HEIGHT,
    wasmURL: DOOM_WASM_URL,
    keyboardTarget: canvas,
    onFrameRender: ({ screen }) => {
      frame.data.set(screen)
      context.putImageData(frame, 0, 0)
    },
  })

  void doom.start().catch((error: unknown) => {
    const message = error instanceof Error ? error.message : String(error)
    canvas.replaceWith(Object.assign(document.createElement("p"), { textContent: message }))
  })
}

const container = document.getElementById("doom")
if (container) {
  const button = document.createElement("button")
  button.type = "button"
  button.className = "doom-start"
  button.textContent = "play doom (6.8 MB)"
  button.addEventListener("click", () => start(container, button), { once: true })
  container.append(button)
}
