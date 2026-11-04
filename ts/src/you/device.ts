/**
 * Names for the browser, operating system and graphics card, in the words a
 * person would use rather than the strings the APIs return.
 */

/** The parts of `userAgentData.getHighEntropyValues` read here. */
export interface HighEntropy {
  brands?: Brand[]
  fullVersionList?: Brand[]
  platform?: string
  platformVersion?: string
  architecture?: string
  model?: string
}

interface Brand {
  brand: string
  version: string
}

/** Brands ahead of plain Chromium, most specific first. */
const BRANDS: Array<[string, string]> = [
  ["Microsoft Edge", "Edge"],
  ["Opera", "Opera"],
  ["Brave", "Brave"],
  ["Vivaldi", "Vivaldi"],
  ["YaBrowser", "Yandex Browser"],
  ["Google Chrome", "Chrome"],
  ["Chromium", "Chromium"],
]

/** Browsers told apart by user agent alone, for those without client hints. */
const AGENTS: Array<[RegExp, string]> = [
  [/Edg\/(\d+)/, "Edge"],
  [/OPR\/(\d+)/, "Opera"],
  [/Firefox\/(\d+)/, "Firefox"],
  [/Chrome\/(\d+)/, "Chrome"],
  [/Version\/(\d+(?:\.\d+)?).*Safari/, "Safari"],
]

/**
 * Windows reports 10 and 11 alike in the user agent; client hints tell them
 * apart by `platformVersion`, which starts at 13 on Windows 11.
 */
const WINDOWS_11 = 13

/** Renderers that draw on the processor because no graphics card was given. */
const SOFTWARE = /SwiftShader|Basic Render Driver|llvmpipe|softpipe/i

export enum Rendering {
  Hardware,
  Software,
}

export interface Graphics {
  name: string
  rendering: Rendering
}

function trimVersion(version: string): string {
  return version.replace(/(\.0)+$/, "")
}

export function browserName(hints: HighEntropy | undefined): string {
  const list = hints?.fullVersionList ?? hints?.brands ?? []

  for (const [brand, name] of BRANDS) {
    const match = list.find((entry) => entry.brand === brand)
    if (!match) continue

    return `${name} ${match.version.split(".")[0]}`
  }

  for (const [pattern, name] of AGENTS) {
    const match = pattern.exec(navigator.userAgent)
    if (match) return `${name} ${match[1]}`
  }

  return "an unknown browser"
}

function systemFromHints(hints: HighEntropy): string | undefined {
  const version = trimVersion(hints.platformVersion ?? "")
  const major = Number.parseInt(version, 10)

  switch (hints.platform) {
    case "Windows":
      if (major >= WINDOWS_11) return "Windows 11"
      return major > 0 ? "Windows 10" : "Windows"
    case "macOS":
      return version ? `a Mac (macOS ${version})` : "a Mac"
    case "Android":
      return hints.model ? `${hints.model} (Android ${version})` : `Android ${version}`
    case "Chrome OS":
    case "ChromeOS":
      return "a Chromebook"
    case "Linux":
      return "Linux"
    default:
      return undefined
  }
}

function systemFromAgent(agent: string): string {
  const iphone = /iPhone OS (\d+)_(\d+)/.exec(agent)
  if (iphone) return `an iPhone (iOS ${iphone[1]}.${iphone[2]})`

  const ipad = /iPad.*OS (\d+)_(\d+)/.exec(agent)
  if (ipad) return `an iPad (iPadOS ${ipad[1]}.${ipad[2]})`

  const android = /Android (\d+(?:\.\d+)?)/.exec(agent)
  if (android) return `Android ${android[1]}`

  if (/Windows NT 10/.test(agent)) return "Windows 10 or 11"
  if (/Windows/.test(agent)) return "Windows"
  if (/Mac OS X/.test(agent)) return "a Mac"
  if (/CrOS/.test(agent)) return "a Chromebook"
  if (/Linux/.test(agent)) return "Linux"

  return "an unknown system"
}

export function systemName(hints: HighEntropy | undefined): string {
  const fromHints = hints ? systemFromHints(hints) : undefined

  return fromHints ?? systemFromAgent(navigator.userAgent)
}

/**
 * The card's name out of a WebGL renderer string.
 *
 * `ANGLE (Apple, ANGLE Metal Renderer: Apple M1 Max, Unspecified Version)`
 * becomes `Apple M1 Max`, and
 * `ANGLE (NVIDIA, NVIDIA GeForce RTX 4090 (0x00002684) Direct3D11 vs_5_0 ps_5_0, D3D11)`
 * becomes `NVIDIA GeForce RTX 4090`.
 */
export function graphicsOf(renderer: string): Graphics {
  const angle = /^ANGLE \((.*)\)$/.exec(renderer)
  const device = angle ? (angle[1].split(", ")[1] ?? angle[1]) : renderer

  const name = device
    .replace(/^ANGLE \w+ Renderer: /, "")
    .replace(/ \(0x[0-9a-f]+\)/i, "")
    .replace(/ Direct3D.*$/, "")
    .replace(/\/PCIe.*$/, "")
    .replace(/ OpenGL Engine$/, "")
    .replace(/\((R|TM)\)/g, "")
    .trim()

  return {
    name,
    rendering: SOFTWARE.test(renderer) ? Rendering.Software : Rendering.Hardware,
  }
}
