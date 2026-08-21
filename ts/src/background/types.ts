/**
 * Shared types for the background shader system.
 *
 * A background is either the original DOM/CSS one (`dappled-light`) or a
 * WebGL2 fragment shader. Both are described by the same record so the menu can
 * list them together and the store can persist a choice without caring which
 * kind it is.
 */

/** A knob a shader exposes to the menu, bound to a `float u_<key>` uniform. */
export interface ShaderParam {
  key: string
  label: string
  min: number
  max: number
  step: number
  value: number
}

export type BackgroundKind = "dom" | "glsl" | "fluid"

export interface BackgroundDef {
  id: string
  name: string
  /** One line shown under the name in the menu. */
  blurb: string
  kind: BackgroundKind
  /** Reads `--bg-theme` so it repaints when the site theme flips. */
  themeReactive: boolean
  /** Reads `iMouse`, so pointer movement changes the image. */
  mouseReactive: boolean
  /** GLSL ES 3.00 body. Omitted for `dom` and `fluid`. */
  fragment?: string
  params: ShaderParam[]
  /** True for user-pasted shaders, which the menu lets you delete. */
  custom?: boolean
}

/** Everything persisted to localStorage. */
export interface BackgroundSettings {
  /** `id` of the selected background. */
  selected: string
  /** Per-background parameter overrides, keyed by background id then param key. */
  params: Record<string, Record<string, number>>
  /** Whether the background animates at all. */
  enabled: boolean
  /** Fraction of full speed, 0–2. */
  speed: number
  /** Fraction of full opacity, 0–1. */
  opacity: number
  /** User-pasted shaders, kept verbatim so they can be re-edited. */
  custom: CustomShaderRecord[]
}

export interface CustomShaderRecord {
  id: string
  name: string
  source: string
}

export const DEFAULT_SETTINGS: BackgroundSettings = {
  selected: "dappled-light",
  params: {},
  enabled: true,
  speed: 1,
  opacity: 1,
  custom: [],
}
