/** The shapes the wasm hands over as JSON, named as their Rust types are. */

export interface Degree {
  ratio_label: string
  ratio: number
  cents: number
  from_equal: number
  frequency: number
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
  moments: Moment[]
  preferred: Moment | null
}

/** A temperament's moment-of-symmetry scale of one size, and the id that selects it. */
export interface Moment {
  size: number
  id: string
}

export interface Scale {
  id: string
  name: string
  family: string
  description: string
  count: number
  period_ratio: number
  period_cents: number
  /** `octave`, `tritave`, or the period in cents. */
  period_name: string
  root_hz: number
  adaptive: boolean
  twelve_tone: boolean
  degrees: Degree[]
  temperament: TemperamentFacts | null
}

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

export interface Key {
  step: number
  degree: number
  label: string
  cents: number
  frequency: number
  black: boolean
}
