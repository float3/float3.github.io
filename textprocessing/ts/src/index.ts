let wasmModulePromise: Promise<typeof import("wasm")>

console.debug("Loading WASM module...")

function loadWasm(): Promise<typeof import("wasm")> {
  if (!wasmModulePromise) {
    wasmModulePromise = import("wasm").then(async (module) => {
      await module.default()
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
          return wasm.convert_japanese_to_kana(text)
        case Side.RIGHT:
          return wasm.convert_japanese_to_kanji(text)
        default:
          return text
      }
    case 2:
      switch (side) {
        case Side.LEFT:
          return wasm.number_to_chinese_f128(text, true, 1)
        case Side.RIGHT:
          return text
        default:
          return text
      }
    default:
      return text
  }
}
