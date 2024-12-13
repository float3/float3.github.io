let wasmModulePromise: Promise<typeof import("wasm")>

console.debug("Loading WASM module...")

function loadWasm(): Promise<typeof import("wasm")> {
  if (!wasmModulePromise) {
    wasmModulePromise = import("wasm").then(async (module) => {
      await module.main()
      return module
    })
  }
  return wasmModulePromise
}

export async function initWasm(): Promise<void> {
  await loadWasm()
}

enum Side {
  LEFT,
  RIGHT,
}

export async function transformLeftToRight(index: number): Promise<void> {
  const leftEl = document.getElementById(`left${index}`) as HTMLInputElement | null
  const rightEl = document.getElementById(`right${index}`) as HTMLInputElement | null
  if (leftEl && rightEl) {
    const left = leftEl.value
    rightEl.value = await mockTransform(left, index, Side.LEFT)
  }
}

export async function transformRightToLeft(index: number): Promise<void> {
  const rightEl = document.getElementById(`right${index}`) as HTMLInputElement | null
  const leftEl = document.getElementById(`left${index}`) as HTMLInputElement | null
  if (rightEl && leftEl) {
    const right = rightEl.value
    leftEl.value = await mockTransform(right, index, Side.RIGHT)
  }
}

async function mockTransform(text: string, index: number, side: Side): Promise<string> {
  const wasm = await loadWasm()
  switch (index) {
    case 0:
      switch (side) {
        case Side.LEFT:
          return wasm.pinyin_to_zhuyin_wasm_extended(text)
        case Side.RIGHT:
          return wasm.zhuyin_to_pinyin_wasm_extended(text)
        default:
          return text
      }
    case 1:
      switch (side) {
        case Side.LEFT:
          return wasm.traditional_to_simplified_wasm(text)
        case Side.RIGHT:
          return wasm.simplified_to_traditional_wasm(text)
        default:
          return text
      }
    case 2:
      switch (side) {
        case Side.LEFT:
          return wasm.convert_hiragana_to_katakana(text)
        case Side.RIGHT:
          return wasm.convert_katakana_to_hiragana(text)
        default:
          return text
      }
    case 3:
      switch (side) {
        case Side.LEFT:
          return wasm.hanja_to_hangul(text)
        case Side.RIGHT:
          return wasm.hangul_to_hanja(text)
        default:
          return text
      }
    case 4:
      switch (side) {
        case Side.LEFT:
          return wasm.to_pinyin_wasm(text)
        case Side.RIGHT:
          return text
        default:
          return text
      }
    case 5:
      switch (side) {
        case Side.LEFT:
          return wasm.to_pinyin_multi_wasm(text)
        case Side.RIGHT:
          return text
        default:
          return text
      }
    case 6:
      switch (side) {
        case Side.LEFT:
          return wasm.hanja_to_hangul_all_variants(text)
        case Side.RIGHT:
          return text
        default:
          return text
      }
    default:
      return text
  }
}
