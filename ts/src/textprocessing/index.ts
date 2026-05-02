import * as wasm from "wasm"

wasm.main()

export function transformLeftToRight(index: number) {
  const leftEl = document.getElementById(`left${index}`) as HTMLInputElement | null
  const rightEl = document.getElementById(`right${index}`) as HTMLInputElement | null
  if (leftEl && rightEl) {
    const left = leftEl.value
    rightEl.value = transform(left, index, true)
  }
}

export function transformRightToLeft(index: number) {
  const rightEl = document.getElementById(`right${index}`) as HTMLInputElement | null
  const leftEl = document.getElementById(`left${index}`) as HTMLInputElement | null
  if (rightEl && leftEl) {
    const right = rightEl.value
    leftEl.value = transform(right, index, false)
  }
}

function transform(text: string, index: number, leftToRight: boolean): string {
  return wasm.transform_text(index, leftToRight, text)
}
