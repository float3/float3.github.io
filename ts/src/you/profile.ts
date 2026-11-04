/**
 * The report read back as prose: who a site would take the reader to be,
 * from what the page already collected and nothing that asks permission.
 *
 * Every fact is a chip whose title names the API it came from, so a reader can
 * check any claim against the tables below it.
 */

import { browserName, graphicsOf, Rendering, systemName, type HighEntropy } from "./device.js"
import { readFonts, type FontReading } from "./fonts.js"
import { simpleHash } from "./hash.js"
import {
  addressesOf,
  exitCount,
  Family,
  familyOf,
  isHosting,
  ownerOf,
  type GeoIP,
  type Network,
} from "./network.js"

export enum Tone {
  Plain = "plain",
  Warn = "warn",
  Muted = "muted",
}

interface Chip {
  text: string
  source: string
  tone: Tone
  flag?: string
}

type Part = string | Chip
type Paragraph = Part[]

/** What `you.ts` has already read, handed over rather than read twice. */
export interface Readings {
  network: Network
  highEntropy: unknown
  webgl: unknown
  battery: unknown
  canvas: unknown
  audio: unknown
}

export interface ProfileTarget {
  badge: HTMLElement
  body: HTMLElement
}

const ENGLISH = "en"
const languageNames = new Intl.DisplayNames([ENGLISH], { type: "language" })
const regionNames = new Intl.DisplayNames([ENGLISH], { type: "region" })

const REGIONAL_INDICATOR_A = 0x1f1e6
const LETTER_A = "A".charCodeAt(0)

const SECONDS_PER_MINUTE = 60
const MINUTES_PER_HOUR = 60
const PERCENT = 100

/** ISO weekday numbers, as `Intl.Locale` week info uses them, from 1. */
const WEEKDAYS = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"]

/** Class names ad blockers hide on sight. */
const AD_BAIT = "ad ads adsbox ad-banner ad-placeholder pub_300x250 textads banner-ads sponsored-ad"

/** Long enough for a blocker's cosmetic rules to reach a new element. */
const AD_BAIT_WAIT_MS = 100

function chip(text: string, source: string, tone = Tone.Plain, flag?: string): Chip {
  return { text, source, tone, flag }
}

function isChip(part: Part): part is Chip {
  return typeof part !== "string"
}

function flagOf(code: string | undefined): string | undefined {
  if (!code || !/^[a-z]{2}$/i.test(code)) return undefined

  const letters = [...code.toUpperCase()]
  return String.fromCodePoint(
    ...letters.map((letter) => REGIONAL_INDICATOR_A + letter.charCodeAt(0) - LETTER_A),
  )
}

function regionOf(tag: string): string | undefined {
  try {
    return new Intl.Locale(tag).maximize().region
  } catch {
    return undefined
  }
}

function languageName(tag: string): string {
  try {
    return languageNames.of(tag) ?? tag
  } catch {
    return tag
  }
}

function regionName(code: string): string {
  try {
    return regionNames.of(code) ?? code
  } catch {
    return code
  }
}

function timeZoneName(timeZone: string): string {
  const parts = new Intl.DateTimeFormat(ENGLISH, {
    timeZone,
    timeZoneName: "longGeneric",
  }).formatToParts()

  return parts.find((part) => part.type === "timeZoneName")?.value ?? timeZone
}

function cityOf(timeZone: string): string {
  return (timeZone.split("/").pop() ?? timeZone).replaceAll("_", " ")
}

function clockOf(): string {
  const { hourCycle } = new Intl.DateTimeFormat(undefined, { hour: "numeric" }).resolvedOptions()

  return hourCycle === "h11" || hourCycle === "h12" ? "12-hour" : "24-hour"
}

function weekStart(tag: string): string | undefined {
  try {
    const locale = new Intl.Locale(tag) as Intl.Locale & {
      getWeekInfo?: () => { firstDay: number }
      weekInfo?: { firstDay: number }
    }
    const info = locale.getWeekInfo?.() ?? locale.weekInfo

    return info ? WEEKDAYS[info.firstDay - 1] : undefined
  } catch {
    return undefined
  }
}

function localOffset(): number {
  return -new Date().getTimezoneOffset() * SECONDS_PER_MINUTE
}

/** `3600` reads as `UTC+01:00`. */
function utcOffset(seconds: number): string {
  const sign = seconds < 0 ? "−" : "+"
  const minutes = Math.abs(seconds) / SECONDS_PER_MINUTE
  const hours = Math.floor(minutes / MINUTES_PER_HOUR)
  const rest = minutes % MINUTES_PER_HOUR

  return `UTC${sign}${String(hours).padStart(2, "0")}:${String(rest).padStart(2, "0")}`
}

function asRecord(value: unknown): Record<string, unknown> | undefined {
  return value && typeof value === "object" ? (value as Record<string, unknown>) : undefined
}

function hintsOf(readings: Readings): HighEntropy | undefined {
  return asRecord(readings.highEntropy) as HighEntropy | undefined
}

function platformOf(hints: HighEntropy | undefined): string {
  if (hints?.platform) return hints.platform

  return /Win/.test(navigator.platform) ? "Windows" : navigator.platform
}

function otherLanguages(languages: readonly string[]): string[] {
  const primary = languages[0]?.split("-")[0]
  const bases = languages.map((tag) => tag.split("-")[0]).filter((base) => base !== primary)

  return [...new Set(bases)].map(languageName)
}

function whereNow(geo: GeoIP | undefined, timeZone: string): Chip {
  if (!geo?.country) return chip(cityOf(timeZone), "Intl time zone, by your clock")

  const place = geo.city ? `${geo.city}, ${geo.country}` : geo.country
  return chip(place, "api.ip.sb, by your address", Tone.Plain, flagOf(geo.country_code))
}

function placeParagraph(network: Network): Paragraph {
  const languages = navigator.languages.length ? navigator.languages : [navigator.language]
  const primary = languages[0]
  const home = regionOf(primary)
  const timeZone = Intl.DateTimeFormat().resolvedOptions().timeZone
  const parts: Paragraph = []

  if (home) {
    const from = chip(regionName(home), "navigator.languages", Tone.Plain, flagOf(home))
    parts.push("Probably from ", from, ", now in ")
  } else {
    parts.push("Now in ")
  }

  parts.push(whereNow(network.geo, timeZone), ". You speak ")
  parts.push(chip(languageName(primary), "navigator.language"))

  const others = otherLanguages(languages)
  if (others.length) parts.push(" and read ", chip(others.join(", "), "navigator.languages"))

  parts.push(", on ", chip(timeZoneName(timeZone), "Intl time zone"))
  parts.push(" with a ", chip(clockOf(), "Intl hourCycle"), " clock")

  const firstDay = weekStart(primary)
  if (firstDay) parts.push(" and weeks that start on ", chip(firstDay, "Intl.Locale weekInfo"))

  parts.push(".")
  return parts
}

function screenSize(): string {
  const width = Math.round(screen.width * devicePixelRatio)
  const height = Math.round(screen.height * devicePixelRatio)

  return `${width}×${height}`
}

function powerParts(battery: unknown): Paragraph {
  const record = asRecord(battery)
  if (!record || typeof record.level !== "number") return []

  const mains =
    record.charging === true && record.level === 1 && record.dischargingTime === Infinity
  if (mains) return [" No battery, so ", chip("probably a desktop", "getBattery"), "."]

  const level = chip(`${Math.round(record.level * PERCENT)}%`, "getBattery")
  const state = record.charging ? " and charging" : ""
  return [" Your battery is at ", level, `${state}.`]
}

function deviceParagraph(readings: Readings): Paragraph {
  const hints = hintsOf(readings)
  const parts: Paragraph = [
    "You use ",
    chip(browserName(hints), "userAgentData brands"),
    " on ",
    chip(systemName(hints), "userAgentData platform"),
    ": a ",
    chip(screenSize(), "screen × devicePixelRatio"),
    " screen",
  ]

  const renderer = asRecord(readings.webgl)?.renderer
  if (typeof renderer === "string") {
    const graphics = graphicsOf(renderer)
    parts.push(" and ", chip(graphics.name, renderer), " graphics")

    if (graphics.rendering === Rendering.Software) {
      parts.push(" (", chip("drawn in software", renderer, Tone.Warn), ")")
    }
  }

  parts.push(", with ", chip(`${navigator.hardwareConcurrency} threads`, "hardwareConcurrency"))

  const memory = (navigator as Navigator & { deviceMemory?: number }).deviceMemory
  if (memory) parts.push(" and ", chip(`at least ${memory} GB`, "deviceMemory"), " of memory")

  parts.push(".", ...powerParts(readings.battery))

  if (navigator.maxTouchPoints > 0) {
    parts.push(" It has a ", chip("touchscreen", "maxTouchPoints"), ".")
  }

  return parts
}

function familiesOf(addresses: string[]): string {
  const families = new Set(addresses.map(familyOf))

  if (families.size > 1) return "IPv4 and IPv6"
  return families.has(Family.V6) ? "IPv6 only" : "IPv4 only"
}

function clockParts(geo: GeoIP | undefined): Paragraph {
  if (geo?.offset === undefined) return []

  const mine = localOffset()
  if (mine === geo.offset) {
    return [
      " Your clock and your address agree on ",
      chip(utcOffset(mine), "getTimezoneOffset"),
      ".",
    ]
  }

  return [
    " Your clock is on ",
    chip(utcOffset(mine), "getTimezoneOffset"),
    " but your address is on ",
    chip(utcOffset(geo.offset), "api.ip.sb", Tone.Warn),
    ": something between you and this page is moving your traffic.",
  ]
}

function networkParagraph(network: Network): Paragraph {
  const geo = network.geo
  const addresses = addressesOf(network)
  if (!addresses.length) return ["Your address could not be looked up from here."]

  const route = addresses.join(" · ")
  const parts: Paragraph = []

  if (geo) {
    const owner = ownerOf(geo) ?? "an unnamed network"
    parts.push("You connect through ", chip(owner, geo.asn ? `AS${geo.asn}` : "api.ip.sb"))

    if (isHosting(geo)) parts.push(", ", chip("a data center, not a home line", owner, Tone.Warn))
  } else {
    parts.push("You connect from ", chip(addresses[0], "ipify"))
  }

  const exits = exitCount(network)
  if (exits > 1) parts.push(", ", chip(`split across ${exits} exits`, route, Tone.Warn))

  parts.push(", over ", chip(familiesOf(addresses), route), ".")
  parts.push(...clockParts(geo))

  return parts
}

/** Whether a decoy shaped like an ad survives on the page. */
async function blocksAds(): Promise<boolean> {
  const bait = document.createElement("div")
  bait.className = AD_BAIT
  bait.setAttribute("aria-hidden", "true")
  bait.style.cssText = "position:absolute;left:-9999px;top:0;width:1px;height:1px"
  bait.textContent = " "
  document.body.append(bait)

  await new Promise((resolve) => window.setTimeout(resolve, AD_BAIT_WAIT_MS))

  const hidden =
    !bait.isConnected || bait.offsetHeight === 0 || getComputedStyle(bait).display === "none"
  bait.remove()

  return hidden
}

function trackingParts(): Paragraph {
  const extras = navigator as Navigator & { globalPrivacyControl?: boolean }
  const asks = extras.globalPrivacyControl === true || navigator.doNotTrack === "1"

  if (asks) return [" You ask sites ", chip("not to track you", "Sec-GPC · DNT"), "."]
  return [" You ", chip("never ask", "Sec-GPC · DNT", Tone.Muted), " sites not to track you."]
}

function historyParts(): Paragraph {
  const parts: Paragraph = []
  const pages = history.length === 1 ? "1 page" : `${history.length} pages`
  parts.push(" This tab has been to ", chip(pages, "history.length"))

  if (document.referrer) {
    parts.push(", and you came from ", chip(new URL(document.referrer).host, "document.referrer"))
  }

  parts.push(".")
  return parts
}

async function habitsParagraph(fonts: FontReading): Promise<Paragraph> {
  const dark = matchMedia("(prefers-color-scheme: dark)").matches
  const parts: Paragraph = [
    "You prefer ",
    chip(dark ? "dark mode" : "light mode", "prefers-color-scheme"),
  ]

  if (matchMedia("(prefers-reduced-motion: reduce)").matches) {
    parts.push(" and ", chip("less motion", "prefers-reduced-motion"))
  }

  const ads = (await blocksAds())
    ? chip("you block ads", "bait element")
    : chip("you don't block ads", "bait element", Tone.Muted)
  parts.push(", and ", ads, ".")
  parts.push(...trackingParts())

  const said = fonts.hints.length
    ? chip(fonts.hints.join(", "), fonts.found.join(", "))
    : chip("nothing in particular", "canvas measureText", Tone.Muted)
  parts.push(" Your fonts say: ", said, ".")

  if (navigator.webdriver) {
    parts.push(
      " A ",
      chip("program", "navigator.webdriver", Tone.Warn),
      " is driving this browser.",
    )
  }

  parts.push(...historyParts())
  return parts
}

function identityParagraph(readings: Readings, fonts: FontReading): Paragraph {
  const signals = [
    navigator.userAgent,
    navigator.languages.join(),
    Intl.DateTimeFormat().resolvedOptions().timeZone,
    screenSize(),
    navigator.hardwareConcurrency,
    JSON.stringify(readings.webgl),
    JSON.stringify(readings.canvas),
    JSON.stringify(readings.audio),
    fonts.found.join(),
  ]

  return [
    "Put together, a site can call you ",
    chip(simpleHash(signals.join("|")), "hash of the signals above"),
    " without setting a cookie, and it usually stays the same in a private window.",
  ]
}

function chipElement(part: Chip): HTMLElement {
  const span = document.createElement("span")
  span.className = `chip ${part.tone}`
  span.title = part.source

  if (part.flag) span.append(`${part.flag} `)
  span.append(part.text)

  return span
}

function paragraphElement(paragraph: Paragraph): HTMLParagraphElement {
  const element = document.createElement("p")

  for (const part of paragraph) {
    element.append(isChip(part) ? chipElement(part) : part)
  }

  return element
}

function plainText(paragraph: Paragraph): string {
  return paragraph.map((part) => (isChip(part) ? part.text : part)).join("")
}

/** Draws the prose into `target` and returns it as plain text, one paragraph per entry. */
export async function renderProfile(target: ProfileTarget, readings: Readings): Promise<string[]> {
  const fonts = readFonts(platformOf(hintsOf(readings)))

  const paragraphs = [
    placeParagraph(readings.network),
    deviceParagraph(readings),
    networkParagraph(readings.network),
    await habitsParagraph(fonts),
    identityParagraph(readings, fonts),
  ]

  const signals = paragraphs.flat().filter(isChip).length
  target.badge.textContent = `${signals} signals · none asked for`
  target.body.replaceChildren(...paragraphs.map(paragraphElement))

  return paragraphs.map(plainText)
}
