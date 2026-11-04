/** A 32-bit string hash, printed as `0x` and eight hex digits. Not cryptographic. */
export function simpleHash(text: string): string {
  let hash = 0

  for (let i = 0; i < text.length; i++) {
    hash = (hash * 31 + text.charCodeAt(i)) >>> 0
  }

  return `0x${hash.toString(16).padStart(8, "0")}`
}
