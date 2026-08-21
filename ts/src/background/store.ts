/**
 * Persistence for background settings.
 *
 * Everything lives under one localStorage key so a single read at startup
 * restores the whole state — which matters because the background is painted
 * before anything else and must not flash the wrong thing.
 */

import { BackgroundSettings, CustomShaderRecord, DEFAULT_SETTINGS } from "./types.js"

const STORAGE_KEY = "float3:background"

/** Reads settings, falling back to defaults for anything missing or corrupt. */
export function loadSettings(): BackgroundSettings {
  if (typeof localStorage === "undefined") return { ...DEFAULT_SETTINGS }

  let parsed: unknown
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (!raw) return { ...DEFAULT_SETTINGS }
    parsed = JSON.parse(raw)
  } catch {
    // A malformed value is not worth surfacing; fall back and move on.
    return { ...DEFAULT_SETTINGS }
  }

  if (typeof parsed !== "object" || parsed === null) return { ...DEFAULT_SETTINGS }
  const record = parsed as Partial<BackgroundSettings>

  return {
    selected: typeof record.selected === "string" ? record.selected : DEFAULT_SETTINGS.selected,
    params: isParamMap(record.params) ? record.params : {},
    enabled: typeof record.enabled === "boolean" ? record.enabled : true,
    speed: clamp(numberOr(record.speed, 1), 0, 2),
    opacity: clamp(numberOr(record.opacity, 1), 0, 1),
    custom: Array.isArray(record.custom) ? record.custom.filter(isCustomRecord) : [],
  }
}

export function saveSettings(settings: BackgroundSettings): void {
  if (typeof localStorage === "undefined") return
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings))
  } catch {
    // Private-mode quota failures should never break the page.
  }
}

function numberOr(value: unknown, fallback: number): number {
  return typeof value === "number" && Number.isFinite(value) ? value : fallback
}

function clamp(value: number, low: number, high: number): number {
  return Math.min(high, Math.max(low, value))
}

function isParamMap(value: unknown): value is Record<string, Record<string, number>> {
  return typeof value === "object" && value !== null && !Array.isArray(value)
}

function isCustomRecord(value: unknown): value is CustomShaderRecord {
  if (typeof value !== "object" || value === null) return false
  const record = value as Partial<CustomShaderRecord>
  return (
    typeof record.id === "string" &&
    typeof record.name === "string" &&
    typeof record.source === "string"
  )
}
