import * as abcjs from "abcjs"
import { recursive_ji_chord_progression_abc, recursive_ji_note_splits_abc } from "wasm"

type AbcRenderer = {
  renderAbc: (target: string, abc: string, options?: { responsive?: string }) => unknown
}

const renderer = abcjs as AbcRenderer

const abcForKind: Record<string, () => string> = {
  progression: recursive_ji_chord_progression_abc,
  "note-splits": recursive_ji_note_splits_abc,
}

document.querySelectorAll<HTMLElement>("[data-recursive-ji-abc]").forEach((container, index) => {
  const kind = container.dataset.recursiveJiAbc
  const abc = kind ? abcForKind[kind]?.() : undefined

  if (!abc) {
    return
  }

  if (!container.id) {
    container.id = `recursive-ji-abc-${index}`
  }

  renderer.renderAbc(container.id, abc, { responsive: "resize" })
})
