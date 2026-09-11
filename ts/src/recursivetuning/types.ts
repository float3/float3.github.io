/** The shapes the wasm hands over as JSON, named as their Rust types are. */

export interface SystemEntry {
  id: string
  name: string
  family: string
  description: string
  count: number
}

export interface EqualPreset {
  id: string
  label: string
  divisions: number
  numerator: number
  denominator: number
  note: string
}

export interface Generator {
  ratio: string
  cents: number
}

export interface TemperamentFacts {
  name: string
  page: string
  subgroup: string
  rank: number
  periods_per_equave: number
  generators: Generator[]
  optimization: string
  commas: string[]
  published_moments: string[]
  moments: number[]
}

export interface ScalaEntry {
  id: string
  file: string
  description: string
  count: number
}

export interface Library {
  systems: SystemEntry[]
  temperaments: TemperamentFacts[]
  equal: EqualPreset[]
  scala_count: number
}

/** Either scale of a pair. */
export interface Side {
  id: string
  name: string
  family: string
  description: string
  count: number
  period_cents: number
}

export interface Column {
  step: number
  ratio_label: string
  cents: number
}

export interface Cell {
  step: number
  frequency: number
  cents: number
  from_fixed: number
  note: number
  name: string
}

export interface Row {
  root: number
  label: string
  ratio_label: string
  frequency: number
  cells: Cell[]
}

export interface Matrix {
  global: Side
  local: Side
  root_hz: number
  aligned: boolean
  twelve_tone: boolean
  largest_shift: number
  truncated: boolean
  columns: Column[]
  rows: Row[]
}

/** A note of the progression, as each of the three renderings plays it. */
export interface Voice {
  step: number
  name: string
  recursive: number
  fixed: number
  equal: number
  from_fixed: number
}

export interface Chord {
  name: string
  root: number
  voices: Voice[]
}
