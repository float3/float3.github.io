"use strict"

/** As much of the Web MIDI API as this page actually looks at. */
interface MidiPort {
  name?: string
  manufacturer?: string
  state?: string
}

interface MidiPortMap {
  forEach(callback: (value: MidiPort, key: string) => void): void
}

interface MidiAccess {
  inputs: MidiPortMap
  outputs: MidiPortMap
}

interface ReportState {
  generatedAt: string
  data: Record<string, unknown>
}

interface PlainObject {
  [key: string]: unknown
}

type NavigatorWithExtras = Navigator & {
  deviceMemory?: number
  pdfViewerEnabled?: boolean
  userAgentData?: {
    mobile: boolean
    platform: string
    brands: Array<{ brand: string; version: string }>
    getHighEntropyValues?: (hints: string[]) => Promise<Record<string, unknown>>
  }
  connection?: {
    effectiveType?: string
    downlink?: number
    rtt?: number
    saveData?: boolean
    type?: string
  }
  wakeLock?: unknown
  bluetooth?: unknown
  usb?: unknown
  serial?: unknown
  hid?: unknown
  gpu?: {
    requestAdapter?: () => Promise<unknown>
  }
  requestMIDIAccess?: (options?: { sysex?: boolean }) => Promise<MidiAccess>
  getBattery?: () => Promise<{
    charging: boolean
    level: number
    chargingTime: number
    dischargingTime: number
  }>
}

interface PerformanceWithMemory extends Performance {
  memory?: unknown
}

interface WindowWithExtras extends Window {
  webkitAudioContext?: typeof AudioContext
  webkitOfflineAudioContext?: typeof OfflineAudioContext
  openDatabase?: unknown
}

const nav = navigator as NavigatorWithExtras
const perf = performance as PerformanceWithMemory
const win = window as WindowWithExtras

const state: ReportState = {
  generatedAt: "",
  data: {},
}

function requiredElement<T extends Element>(selector: string): T {
  const element = document.querySelector<T>(selector)

  if (!element) {
    throw new Error(`Missing required element: ${selector}`)
  }

  return element
}

const report = requiredElement<HTMLElement>(".browser-report")
const summaryEl = requiredElement<HTMLElement>("#summary")
const sectionsEl = requiredElement<HTMLElement>("#sections")
const refreshBtn = requiredElement<HTMLButtonElement>("#refreshBtn")
const expandBtn = requiredElement<HTMLButtonElement>("#expandBtn")
const collapseBtn = requiredElement<HTMLButtonElement>("#collapseBtn")
const copyBtn = requiredElement<HTMLButtonElement>("#copyBtn")

const toPlain = (value: unknown): unknown => {
  if (value instanceof Error) {
    return {
      name: value.name,
      message: value.message,
      stack: value.stack,
    }
  }

  if (value instanceof Map) {
    return Object.fromEntries(value.entries())
  }

  if (value instanceof Set) {
    return Array.from(value.values()).map(toPlain)
  }

  if (Array.isArray(value)) {
    return value.map(toPlain)
  }

  if (value && typeof value === "object") {
    const out: PlainObject = {}

    for (const [key, item] of Object.entries(value)) {
      out[key] = toPlain(item)
    }

    return out
  }

  return value
}

const simpleHash = (str: string): string => {
  let hash = 0

  for (let i = 0; i < str.length; i++) {
    hash = (hash * 31 + str.charCodeAt(i)) >>> 0
  }

  return `0x${hash.toString(16).padStart(8, "0")}`
}

function addSummary(label: string, value: unknown): void {
  const card = document.createElement("div")
  card.className = "card"

  const labelEl = document.createElement("div")
  labelEl.className = "label"
  labelEl.textContent = label

  const valueEl = document.createElement("div")
  valueEl.className = "value"
  valueEl.textContent = String(value)

  card.append(labelEl, valueEl)
  summaryEl.appendChild(card)
}

function addSection(title: string, obj: unknown): void {
  const details = document.createElement("details")
  details.open = true

  const summary = document.createElement("summary")
  summary.textContent = title

  const body = document.createElement("div")
  body.className = "section-body"

  if (Array.isArray(obj)) {
    const pre = document.createElement("pre")
    pre.textContent = JSON.stringify(toPlain(obj), null, 2)
    body.appendChild(pre)
  } else if (obj && typeof obj === "object") {
    const table = document.createElement("table")

    for (const [key, value] of Object.entries(obj)) {
      const tr = document.createElement("tr")

      const tdKey = document.createElement("td")
      tdKey.className = "key"
      tdKey.textContent = key

      const tdVal = document.createElement("td")
      const pre = document.createElement("pre")

      pre.textContent = typeof value === "string" ? value : JSON.stringify(toPlain(value), null, 2)

      tdVal.appendChild(pre)
      tr.append(tdKey, tdVal)
      table.appendChild(tr)
    }

    body.appendChild(table)
  } else {
    const pre = document.createElement("pre")
    pre.textContent = String(obj)
    body.appendChild(pre)
  }

  details.append(summary, body)
  sectionsEl.appendChild(details)
}

function getNavigatorInfo(): PlainObject {
  return {
    userAgent: nav.userAgent,
    platform: nav.platform,
    vendor: nav.vendor,
    product: nav.product,
    productSub: nav.productSub,
    appCodeName: nav.appCodeName,
    appName: nav.appName,
    appVersion: nav.appVersion,
    language: nav.language,
    languages: nav.languages,
    cookieEnabled: nav.cookieEnabled,
    onLine: nav.onLine,
    doNotTrack: nav.doNotTrack,
    hardwareConcurrency: nav.hardwareConcurrency,
    deviceMemory: nav.deviceMemory,
    maxTouchPoints: nav.maxTouchPoints,
    webdriver: nav.webdriver,
    pdfViewerEnabled: nav.pdfViewerEnabled,
    userActivation: nav.userActivation
      ? {
          hasBeenActive: nav.userActivation.hasBeenActive,
          isActive: nav.userActivation.isActive,
        }
      : "Unavailable",
    clipboard: !!nav.clipboard,
    geolocation: !!nav.geolocation,
    storage: !!nav.storage,
    serviceWorker: "serviceWorker" in nav,
    share: !!nav.share,
    wakeLock: !!nav.wakeLock,
    bluetooth: !!nav.bluetooth,
    usb: !!nav.usb,
    serial: !!nav.serial,
    hid: !!nav.hid,
    gpu: !!nav.gpu,
    mediaDevices: !!nav.mediaDevices,
    permissions: !!nav.permissions,
    connection: nav.connection
      ? {
          effectiveType: nav.connection.effectiveType,
          downlink: nav.connection.downlink,
          rtt: nav.connection.rtt,
          saveData: nav.connection.saveData,
          type: nav.connection.type,
        }
      : "Unavailable",
  }
}

function getScreenInfo(): PlainObject {
  return {
    screenWidth: screen.width,
    screenHeight: screen.height,
    availableWidth: screen.availWidth,
    availableHeight: screen.availHeight,
    colorDepth: screen.colorDepth,
    pixelDepth: screen.pixelDepth,
    devicePixelRatio: window.devicePixelRatio,
    innerWidth: window.innerWidth,
    innerHeight: window.innerHeight,
    outerWidth: window.outerWidth,
    outerHeight: window.outerHeight,
    orientation: screen.orientation?.type ?? "Unavailable",
    orientationAngle: screen.orientation?.angle ?? "Unavailable",
  }
}

function getLocaleInfo(): PlainObject {
  const resolved = new Intl.DateTimeFormat().resolvedOptions()

  return {
    language: nav.language,
    languages: nav.languages,
    locale: resolved.locale,
    timeZone: resolved.timeZone,
    calendar: resolved.calendar,
    numberingSystem: resolved.numberingSystem,
    hourCycle: resolved.hourCycle,
    dateTimeResolvedOptions: resolved,
    supportedCalendars:
      typeof Intl.supportedValuesOf === "function"
        ? Intl.supportedValuesOf("calendar").slice(0, 40)
        : "Unavailable",
    supportedCollations:
      typeof Intl.supportedValuesOf === "function"
        ? Intl.supportedValuesOf("collation").slice(0, 40)
        : "Unavailable",
    supportedTimeZonesCount:
      typeof Intl.supportedValuesOf === "function"
        ? Intl.supportedValuesOf("timeZone").length
        : "Unavailable",
  }
}

function getWindowInfo(): PlainObject {
  return {
    isSecureContext,
    crossOriginIsolated,
    historyLength: history.length,
    screenX: window.screenX,
    screenY: window.screenY,
    scrollX: window.scrollX,
    scrollY: window.scrollY,
    pageXOffset: window.pageXOffset,
    pageYOffset: window.pageYOffset,
    locationHref: location.href,
    origin: location.origin,
    protocol: location.protocol,
    host: location.host,
    hostname: location.hostname,
    port: location.port,
    pathname: location.pathname,
    search: location.search,
    hash: location.hash,
    referrer: document.referrer,
    title: document.title,
    visibilityState: document.visibilityState,
    prerendering:
      "prerendering" in document
        ? Boolean((document as Document & { prerendering?: boolean }).prerendering)
        : false,
  }
}

function getCSSFeatureInfo(): PlainObject {
  const queries: Array<[string, string]> = [
    ["prefers-color-scheme: dark", "(prefers-color-scheme: dark)"],
    ["prefers-color-scheme: light", "(prefers-color-scheme: light)"],
    ["prefers-reduced-motion: reduce", "(prefers-reduced-motion: reduce)"],
    ["hover: hover", "(hover: hover)"],
    ["any-hover: hover", "(any-hover: hover)"],
    ["pointer: fine", "(pointer: fine)"],
    ["pointer: coarse", "(pointer: coarse)"],
    ["forced-colors: active", "(forced-colors: active)"],
  ]

  const out: PlainObject = {}

  for (const [key, query] of queries) {
    out[key] = matchMedia(query).matches
  }

  out.touchEvents = "ontouchstart" in window
  out.pointerEvent = "PointerEvent" in window
  out.matchMedia = typeof window.matchMedia === "function"

  out.localStorage = (() => {
    try {
      return !!window.localStorage
    } catch {
      return false
    }
  })()

  out.sessionStorage = (() => {
    try {
      return !!window.sessionStorage
    } catch {
      return false
    }
  })()

  out.indexedDB = !!window.indexedDB
  out.broadcastChannel = !!window.BroadcastChannel
  out.offscreenCanvas = "OffscreenCanvas" in window
  out.sharedArrayBuffer = typeof SharedArrayBuffer !== "undefined"
  out.audioContext = "AudioContext" in window || "webkitAudioContext" in window
  out.webAssembly = "WebAssembly" in window
  out.webSocket = "WebSocket" in window
  out.webRTC = "RTCPeerConnection" in window

  return out
}

function getPerformanceInfo(): PlainObject {
  const entries = performance.getEntriesByType?.("navigation") ?? []

  return {
    timeOrigin: performance.timeOrigin,
    now: performance.now(),
    memory: perf.memory ? toPlain(perf.memory) : "Unavailable",
    navigationEntries: entries.map((entry) => {
      const e = entry as PerformanceNavigationTiming

      return {
        name: e.name,
        entryType: e.entryType,
        startTime: e.startTime,
        duration: e.duration,
        type: e.type,
        domContentLoadedEventEnd: e.domContentLoadedEventEnd,
        loadEventEnd: e.loadEventEnd,
        transferSize: e.transferSize,
        encodedBodySize: e.encodedBodySize,
        decodedBodySize: e.decodedBodySize,
      }
    }),
  }
}

async function getHighEntropyHints(): Promise<unknown> {
  const uaData = nav.userAgentData

  if (!uaData?.getHighEntropyValues) {
    return "Unavailable"
  }

  try {
    return await uaData.getHighEntropyValues([
      "architecture",
      "bitness",
      "brands",
      "fullVersionList",
      "mobile",
      "model",
      "platform",
      "platformVersion",
      "uaFullVersion",
      "wow64",
    ])
  } catch (error) {
    return {
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

async function getClientHints(): Promise<PlainObject> {
  return {
    userAgentData: nav.userAgentData
      ? {
          mobile: nav.userAgentData.mobile,
          platform: nav.userAgentData.platform,
          brands: nav.userAgentData.brands,
        }
      : "Unavailable",
    highEntropyHints: await getHighEntropyHints(),
  }
}

function getWebGLInfo(): unknown {
  const canvas = document.createElement("canvas")

  const gl = canvas.getContext("webgl") ?? canvas.getContext("experimental-webgl")

  if (!gl || !(gl instanceof WebGLRenderingContext)) {
    return "Unavailable"
  }

  const debugInfo = gl.getExtension("WEBGL_debug_renderer_info")

  return {
    version: gl.getParameter(gl.VERSION),
    shadingLanguageVersion: gl.getParameter(gl.SHADING_LANGUAGE_VERSION),
    vendor: debugInfo ? gl.getParameter(debugInfo.UNMASKED_VENDOR_WEBGL) : "Hidden",
    renderer: debugInfo ? gl.getParameter(debugInfo.UNMASKED_RENDERER_WEBGL) : "Hidden",
    maxTextureSize: gl.getParameter(gl.MAX_TEXTURE_SIZE),
    maxCubeMapTextureSize: gl.getParameter(gl.MAX_CUBE_MAP_TEXTURE_SIZE),
    maxViewportDims: gl.getParameter(gl.MAX_VIEWPORT_DIMS),
    aliasedLineWidthRange: gl.getParameter(gl.ALIASED_LINE_WIDTH_RANGE),
    aliasedPointSizeRange: gl.getParameter(gl.ALIASED_POINT_SIZE_RANGE),
  }
}

function canvasFingerprint(): unknown {
  const canvas = document.getElementById("fingerprintCanvas") as HTMLCanvasElement | null

  if (!canvas) return "Unavailable"

  const ctx = canvas.getContext("2d")
  if (!ctx) return "Unavailable"

  ctx.textBaseline = "top"
  ctx.font = "16px Arial"
  ctx.fillStyle = "#f60"
  ctx.fillRect(10, 10, 100, 40)

  ctx.fillStyle = "#069"
  ctx.fillText("Browser fingerprint sample", 12, 18)

  ctx.strokeStyle = "#0f0"
  ctx.lineWidth = 2
  ctx.beginPath()
  ctx.arc(140, 40, 20, 0, Math.PI * 1.7)
  ctx.stroke()

  ctx.fillStyle = "rgba(255,0,0,0.6)"
  ctx.fillText("AaBbCc123", 12, 56)

  const dataURL = canvas.toDataURL()

  return {
    hash: simpleHash(dataURL),
    length: dataURL.length,
  }
}

async function audioFingerprint(): Promise<unknown> {
  const AudioContextConstructor = window.OfflineAudioContext ?? win.webkitOfflineAudioContext

  if (!AudioContextConstructor) return "Unavailable"

  try {
    const ctx = new AudioContextConstructor(1, 44100, 44100)
    const osc = ctx.createOscillator()
    const compressor = ctx.createDynamicsCompressor()

    osc.type = "triangle"
    osc.frequency.value = 1000

    compressor.threshold.value = -50
    compressor.knee.value = 40
    compressor.ratio.value = 12
    compressor.attack.value = 0
    compressor.release.value = 0.25

    osc.connect(compressor)
    compressor.connect(ctx.destination)

    osc.start(0)

    const rendered = await ctx.startRendering()
    const channel = rendered.getChannelData(0).slice(4500, 4600)

    return {
      sampleCount: channel.length,
      hash: simpleHash(
        Array.from(channel)
          .map((n) => n.toFixed(6))
          .join(","),
      ),
    }
  } catch (error) {
    return {
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

async function getPublicIP(): Promise<unknown> {
  const services = [
    "https://api.ipify.org?format=json",
    "https://api64.ipify.org?format=json",
    "https://api.ip.sb/jsonip",
  ]

  for (const url of services) {
    try {
      const response = await fetch(url, {
        cache: "no-store",
      })

      if (!response.ok) continue

      return await response.json()
    } catch (error) {
      console.log(url, error)
    }
  }

  return {
    error: "Unable to determine public IP",
  }
}

async function getBatteryInfo(): Promise<unknown> {
  if (!nav.getBattery) return "Unavailable"

  try {
    const battery = await nav.getBattery()

    return {
      charging: battery.charging,
      level: battery.level,
      chargingTime: battery.chargingTime,
      dischargingTime: battery.dischargingTime,
    }
  } catch (error) {
    return {
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

async function getStorageInfo(): Promise<unknown> {
  if (!nav.storage?.estimate) return "Unavailable"

  try {
    return await nav.storage.estimate()
  } catch (error) {
    return {
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

async function getPermissionsInfo(): Promise<unknown> {
  if (!nav.permissions?.query) return "Unavailable"

  const names = [
    "geolocation",
    "notifications",
    "camera",
    "microphone",
    "clipboard-read",
    "clipboard-write",
    "midi",
    "background-sync",
    "persistent-storage",
    "screen-wake-lock",
    "payment",
  ]

  const out: Record<string, string> = {}

  for (const name of names) {
    try {
      const permission = await nav.permissions.query({
        name: name as PermissionName,
      })

      out[name] = permission.state
    } catch {
      out[name] = "unsupported"
    }
  }

  return out
}

async function getMediaDevices(): Promise<unknown> {
  if (!nav.mediaDevices?.enumerateDevices) {
    return "Unavailable"
  }

  try {
    const devices = await nav.mediaDevices.enumerateDevices()

    return devices.map((device, index) => ({
      index,
      kind: device.kind,
      label: device.label || "(label hidden until permission granted)",
      deviceId: device.deviceId ? `${device.deviceId.slice(0, 12)}…` : "",
      groupId: device.groupId ? `${device.groupId.slice(0, 12)}…` : "",
    }))
  } catch (error) {
    return {
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

async function getMIDIInfo(): Promise<unknown> {
  if (!nav.requestMIDIAccess) return "Unavailable"

  try {
    const access = await nav.requestMIDIAccess({ sysex: false })
    const inputs: unknown[] = []
    const outputs: unknown[] = []

    access.inputs.forEach((value, key) => {
      inputs.push({
        id: key,
        name: value.name,
        manufacturer: value.manufacturer,
        state: value.state,
      })
    })

    access.outputs.forEach((value, key) => {
      outputs.push({
        id: key,
        name: value.name,
        manufacturer: value.manufacturer,
        state: value.state,
      })
    })

    return {
      sysexEnabled: access.sysexEnabled,
      inputs,
      outputs,
    }
  } catch (error) {
    return {
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

async function getGamepads(): Promise<unknown> {
  try {
    const pads = nav.getGamepads ? Array.from(nav.getGamepads()) : []

    return pads
      .filter((gamepad): gamepad is Gamepad => gamepad !== null)
      .map((gamepad, index) => ({
        index,
        id: gamepad.id,
        mapping: gamepad.mapping,
        connected: gamepad.connected,
        buttons: gamepad.buttons.length,
        axes: gamepad.axes.length,
        vibrationActuator:
          "vibrationActuator" in gamepad &&
          !!(gamepad as Gamepad & { vibrationActuator?: unknown }).vibrationActuator,
      }))
  } catch (error) {
    return {
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

async function getWebGPUInfo(): Promise<unknown> {
  if (!nav.gpu?.requestAdapter) return "Unavailable"

  try {
    const adapter = await nav.gpu.requestAdapter()

    if (!adapter) {
      return { available: false }
    }

    return {
      available: true,
      isFallbackAdapter: !!(adapter as GPUAdapter & { isFallbackAdapter?: unknown })
        .isFallbackAdapter,
      features: Array.from(adapter.features),
      limits: toPlain(adapter.limits),
    }
  } catch (error) {
    return {
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

async function getVoices(): Promise<unknown> {
  if (!window.speechSynthesis) return "Unavailable"

  try {
    let voices = speechSynthesis.getVoices()

    if (!voices.length) {
      voices = await new Promise<SpeechSynthesisVoice[]>((resolve) => {
        const timeout = window.setTimeout(() => resolve(speechSynthesis.getVoices()), 1200)

        speechSynthesis.onvoiceschanged = () => {
          clearTimeout(timeout)
          resolve(speechSynthesis.getVoices())
        }
      })
    }

    return voices.map((voice) => ({
      name: voice.name,
      lang: voice.lang,
      default: voice.default,
      localService: voice.localService,
      voiceURI: voice.voiceURI,
    }))
  } catch (error) {
    return {
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

function getFonts(): unknown {
  if (!document.fonts) return "Unavailable"

  try {
    return Array.from(document.fonts).map((font) => ({
      family: font.family,
      style: font.style,
      weight: font.weight,
      stretch: font.stretch,
      status: font.status,
    }))
  } catch (error) {
    return {
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

function getPluginInfo(): unknown {
  try {
    return Array.from(nav.plugins ?? []).map((plugin) => ({
      name: plugin.name,
      filename: plugin.filename,
      description: plugin.description,
    }))
  } catch (error) {
    return {
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

function getMimeInfo(): unknown {
  try {
    return Array.from(nav.mimeTypes ?? []).map((mime) => ({
      type: mime.type,
      suffixes: mime.suffixes,
      description: mime.description,
      enabledPlugin: mime.enabledPlugin ? mime.enabledPlugin.name : null,
    }))
  } catch (error) {
    return {
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

function getInstalledCapabilities(): PlainObject {
  return {
    localStorage: (() => {
      try {
        return !!window.localStorage
      } catch {
        return false
      }
    })(),
    sessionStorage: (() => {
      try {
        return !!window.sessionStorage
      } catch {
        return false
      }
    })(),
    indexedDB: !!window.indexedDB,
    webSQL: !!win.openDatabase,
    broadcastChannel: !!window.BroadcastChannel,
    serviceWorker: "serviceWorker" in nav,
    share: !!nav.share,
    clipboard: !!nav.clipboard,
    wakeLock: !!nav.wakeLock,
    bluetooth: !!nav.bluetooth,
    usb: !!nav.usb,
    serial: !!nav.serial,
    hid: !!nav.hid,
    gpu: !!nav.gpu,
    mediaDevices: !!nav.mediaDevices,
    webRTC: "RTCPeerConnection" in window,
    webSocket: "WebSocket" in window,
    webAssembly: "WebAssembly" in window,
    offscreenCanvas: "OffscreenCanvas" in window,
    sharedArrayBuffer: typeof SharedArrayBuffer !== "undefined",
  }
}

async function buildReport(): Promise<void> {
  summaryEl.innerHTML = ""
  sectionsEl.innerHTML = ""

  state.generatedAt = new Date().toISOString()

  const [
    clientHints,
    publicIP,
    battery,
    storage,
    permissions,
    mediaDevices,
    midi,
    gamepads,
    webgpu,
    voices,
  ] = await Promise.all([
    getClientHints(),
    getPublicIP(),
    getBatteryInfo(),
    getStorageInfo(),
    getPermissionsInfo(),
    getMediaDevices(),
    getMIDIInfo(),
    getGamepads(),
    getWebGPUInfo(),
    getVoices(),
  ])

  const locale = getLocaleInfo()
  const screenInfo = getScreenInfo()

  state.data = {
    generatedAt: state.generatedAt,
    navigator: getNavigatorInfo(),
    clientHints,
    screen: screenInfo,
    locale,
    window: getWindowInfo(),
    cssFeatures: getCSSFeatureInfo(),
    capabilities: getInstalledCapabilities(),
    performance: getPerformanceInfo(),
    publicIP,
    battery,
    storage,
    permissions,
    mediaDevices,
    midi,
    gamepads,
    webgpu,
    voices,
    webgl: getWebGLInfo(),
    canvasFingerprint: canvasFingerprint(),
    audioFingerprint: await audioFingerprint(),
    plugins: getPluginInfo(),
    mimeTypes: getMimeInfo(),
    fonts: getFonts(),
  }

  addSummary("Generated at", state.generatedAt)
  addSummary("Locale", locale.locale ?? nav.language ?? "Unknown")
  addSummary("Time zone", locale.timeZone ?? "Unknown")
  addSummary("Screen", `${screenInfo.screenWidth} × ${screenInfo.screenHeight}`)
  addSummary("Touch", nav.maxTouchPoints > 0 ? `Yes (${nav.maxTouchPoints})` : "No")

  const publicIPRecord =
    publicIP && typeof publicIP === "object" ? (publicIP as Record<string, unknown>) : {}

  addSummary(
    "Public IP",
    publicIPRecord.ip ?? publicIPRecord.responseText ?? publicIPRecord.error ?? "Unknown",
  )

  addSection("Navigator", state.data.navigator)
  addSection("Client Hints", state.data.clientHints)
  addSection("Screen", state.data.screen)
  addSection("Locale and Time", state.data.locale)
  addSection("Window and Location", state.data.window)
  addSection("CSS and Feature Detection", state.data.cssFeatures)
  addSection("Capabilities", state.data.capabilities)
  addSection("Performance", state.data.performance)
  addSection("Public IP", state.data.publicIP)
  addSection("Battery", state.data.battery)
  addSection("Storage", state.data.storage)
  addSection("Permissions", state.data.permissions)
  addSection("Media Devices", state.data.mediaDevices)
  addSection("MIDI", state.data.midi)
  addSection("Gamepads", state.data.gamepads)
  addSection("WebGPU", state.data.webgpu)
  addSection("Speech Voices", state.data.voices)
  addSection("WebGL", state.data.webgl)
  addSection("Canvas Fingerprint", state.data.canvasFingerprint)
  addSection("Audio Fingerprint", state.data.audioFingerprint)
  addSection("Plugins", state.data.plugins)
  addSection("MIME Types", state.data.mimeTypes)
  addSection("Loaded Fonts", state.data.fonts)
}

function expandAll(): void {
  report.querySelectorAll<HTMLDetailsElement>("details").forEach((details) => {
    details.open = true
  })
}

function collapseAll(): void {
  report.querySelectorAll<HTMLDetailsElement>("details").forEach((details) => {
    details.open = false
  })
}

async function copyJSON(): Promise<void> {
  try {
    await nav.clipboard.writeText(JSON.stringify(state.data, null, 2))

    copyBtn.textContent = "Copied"

    window.setTimeout(() => {
      copyBtn.textContent = "Copy JSON"
    }, 1200)
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error)

    alert(`Copy failed: ${message}`)
  }
}

refreshBtn.addEventListener("click", () => {
  void buildReport()
})

expandBtn.addEventListener("click", expandAll)
collapseBtn.addEventListener("click", collapseAll)

copyBtn.addEventListener("click", () => {
  void copyJSON()
})

void buildReport()

// Nothing here is imported anywhere; this only keeps the file a module, so its
// top-level names stay its own rather than joining the global scope.
export {}
