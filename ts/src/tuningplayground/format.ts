/** Number formatting and parsing shared by the tuning playground and the recursive tuning post. */

/** A frequency in hertz, to as many places as stay readable. */
export function hz(value: number): string {
  return value.toFixed(value >= 1000 ? 1 : value >= 100 ? 2 : 3)
}

/** A signed amount, with a true minus sign and no negative zero. */
export function signed(value: number, digits = 1): string {
  const rounded = Number(value.toFixed(digits))
  if (rounded === 0) return (0).toFixed(digits)
  return rounded > 0 ? `+${rounded.toFixed(digits)}` : `−${Math.abs(rounded).toFixed(digits)}`
}

/** A number read from text and held within bounds, or the fallback when it is none. */
export function clamp(text: string | null, min: number, max: number, fallback: number): number {
  const value = Number.parseFloat(text ?? "")
  return Number.isFinite(value) ? Math.min(max, Math.max(min, value)) : fallback
}
