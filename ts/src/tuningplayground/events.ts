import { heldKeys, noteOff, noteOn, shift, stopAllTones, wasm } from "./index.js"
import { markKey, showMarkedButtons, unmarkAll } from "./UI.js"

export function visibilityChange(): void {
  if (document.hidden) stopAllTones()
}

/** The marked notes travel in the hash, as a comma-separated list of steps. */
export function readMarkedFromHash(): void {
  unmarkAll()
  const hash = window.location.hash.substring(1)
  if (!hash) {
    showMarkedButtons(false)
    return
  }
  for (const note of hash.split(",")) {
    const step = parseInt(note, 10)
    if (Number.isFinite(step)) markKey(step)
  }
  showMarkedButtons(true)
}

function typing(): boolean {
  const active = document.activeElement
  if (!active) return false
  return (
    ["INPUT", "TEXTAREA", "SELECT"].includes(active.tagName) ||
    (active as HTMLElement).isContentEditable
  )
}

export function keydown(event: KeyboardEvent): void {
  if (!document.hasFocus() || event.repeat || typing()) return
  if (event.metaKey || event.ctrlKey || event.altKey) return
  if (heldKeys.has(event.code)) return

  const step = wasm.from_keymap(event.code)
  if (step === -1) return
  event.preventDefault()
  heldKeys.add(event.code)
  noteOn(step + shift())
}

export function keyup(event: KeyboardEvent): void {
  if (!heldKeys.has(event.code)) return
  heldKeys.delete(event.code)
  const step = wasm.from_keymap(event.code)
  if (step === -1) return
  noteOff(step + shift())
}
