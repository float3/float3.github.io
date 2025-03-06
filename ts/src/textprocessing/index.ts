import * as wasm from "wasm"

wasm.main()

enum Side {
  LEFT,
  RIGHT,
}

export function transformLeftToRight(index: number) {
  const leftEl = document.getElementById(`left${index}`) as HTMLInputElement | null
  const rightEl = document.getElementById(`right${index}`) as HTMLInputElement | null
  if (leftEl && rightEl) {
    const left = leftEl.value
    rightEl.value = transform(left, index, Side.LEFT)
  }
}

export function transformRightToLeft(index: number) {
  const rightEl = document.getElementById(`right${index}`) as HTMLInputElement | null
  const leftEl = document.getElementById(`left${index}`) as HTMLInputElement | null
  if (rightEl && leftEl) {
    const right = rightEl.value
    leftEl.value = transform(right, index, Side.RIGHT)
  }
}

function transform(text: string, index: number, side: Side): string {
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
          return wasm.hanja_to_hangeul(text)
        case Side.RIGHT:
          return wasm.hangeul_to_hanja(text)
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
          return wasm.hanja_to_hangeul_all_variants(text)
        case Side.RIGHT:
          return text
        default:
          return text
      }
    case 7:
      switch (side) {
        case Side.LEFT:
          return wasm.arabic_to_roman(text)
        case Side.RIGHT:
          return wasm.roman_to_arabic(text)
        default:
          return text
      }
    case 8:
      switch (side) {
        case Side.LEFT:
          return wasm.to_zhuyin_wasm(text)
        case Side.RIGHT:
          return text
        default:
          return text
      }
    case 9:
      switch (side) {
        case Side.LEFT:
          return wasm.to_zhuyin_multi_wasm(text)
        case Side.RIGHT:
          return text
        default:
          return text
      }
    case 10:
      switch (side) {
        case Side.LEFT:
          return wasm.number_to_chinese_f128(text, true, 0)
        case Side.RIGHT:
          return text
        default:
          return text
      }
    case 11:
      switch (side) {
        case Side.LEFT:
          return wasm.number_to_chinese_f128(text, true, 1)
        case Side.RIGHT:
          return text
        default:
          return text
      }
    case 12:
      switch (side) {
        case Side.LEFT:
          return wasm.number_to_chinese_f128(text, true, 2)
        case Side.RIGHT:
          return text
        default:
          return text
      }
    case 13:
      switch (side) {
        case Side.LEFT:
          return wasm.number_to_chinese_f128(text, true, 3)
        case Side.RIGHT:
          return text
        default:
          return text
      }
    case 14:
      switch (side) {
        case Side.LEFT:
          return wasm.number_to_chinese_f128(text, false, 0)
        case Side.RIGHT:
          return text
        default:
          return text
      }
    case 15:
      switch (side) {
        case Side.LEFT:
          return wasm.number_to_chinese_f128(text, false, 1)
        case Side.RIGHT:
          return text
        default:
          return text
      }
    case 16:
      switch (side) {
        case Side.LEFT:
          return wasm.number_to_chinese_f128(text, false, 2)
        case Side.RIGHT:
          return text
        default:
          return text
      }
    case 17:
      switch (side) {
        case Side.LEFT:
          return wasm.number_to_chinese_f128(text, false, 3)
        case Side.RIGHT:
          return text
        default:
          return text
      }
    case 18:
      switch (side) {
        case Side.LEFT:
          return wasm.number_to_japanese(text)
        case Side.RIGHT:
          return text
        default:
          return text
      }
    case 19:
      switch (side) {
        case Side.LEFT:
          return wasm.romanize_hangeul(text)
        case Side.RIGHT:
          return wasm.roman_to_hangeul(text)
        default:
          return text
      }
    default:
      return text
  }
}
