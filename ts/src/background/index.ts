/**
 * Background shader orchestration.
 *
 * Owns the canvas, the render loop, the pointer and theme state, and the glue
 * between the menu and the store. The DOM background (`dappled-light`) stays in
 * the markup and is simply shown or hidden — it costs nothing when visible and
 * keeps working with JavaScript disabled, which is why it is the default.
 */

import { FluidSimulation } from "./fluid.js"
import { BackgroundMenu } from "./menu.js"
import { buildFragmentSource, linkProgram, supportsWebGPU, UniformCache } from "./renderer.js"
import { BUILTIN_BACKGROUNDS } from "./shaders.js"
import { loadSettings, saveSettings } from "./store.js"
import { BackgroundDef, BackgroundSettings } from "./types.js"

const DOM_BACKGROUND_ID = "dappled-light"

class BackgroundController {
  private settings: BackgroundSettings
  private canvas: HTMLCanvasElement
  private gl: WebGL2RenderingContext | null = null
  private program: WebGLProgram | null = null
  private uniforms: UniformCache | null = null
  private fluid: FluidSimulation | null = null
  private menu: BackgroundMenu | null = null

  private frame = 0
  private startTime = performance.now()
  private lastTime = this.startTime
  private rafHandle = 0
  private running = false

  private pointer = { x: 0, y: 0, px: 0, py: 0, down: 0, moved: false }
  private themeTarget = 0
  private themeCurrent = 0
  private reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)")

  constructor() {
    this.settings = loadSettings()

    this.canvas = adoptCanvas()

    this.themeTarget = currentTheme()
    this.themeCurrent = this.themeTarget

    this.watchTheme()
    this.watchPointer()
    this.watchResize()
    this.watchVisibility()
    this.watchNav()

    this.menu = new BackgroundMenu(
      () => this.allBackgrounds(),
      () => this.settings,
      {
        onSelect: (id) => this.select(id),
        onParamChange: (backgroundId, key, value) => {
          const params = this.settings.params[backgroundId] ?? {}
          params[key] = value
          this.settings.params[backgroundId] = params
          this.persist()
        },
        onGlobalChange: (patch) => {
          Object.assign(this.settings, patch)
          this.persist()
          this.applyGlobals()
        },
        onAddCustom: (name, source) => this.addCustom(name, source),
        onDeleteCustom: (id) => this.deleteCustom(id),
        validate: (source) => this.validate(source),
      },
      supportsWebGPU(),
    )

    this.applyGlobals()
    this.select(this.settings.selected, true)
  }

  /**
   * Whether the render loop should run.
   *
   * Reduced motion is deliberately not folded into `settings.enabled`: it is a
   * property of the visitor's machine, not a choice they made here, and writing
   * it into storage would leave the background switched off for good on a
   * machine that never asked for that.
   */
  private get animating(): boolean {
    return this.settings.enabled && !this.reducedMotion.matches
  }

  private allBackgrounds(): BackgroundDef[] {
    const custom: BackgroundDef[] = this.settings.custom.map((record) => ({
      id: record.id,
      name: record.name,
      blurb: "Your shader.",
      kind: "glsl",
      themeReactive: /uTheme/.test(record.source),
      mouseReactive: /iMouse/.test(record.source),
      fragment: record.source,
      params: [
        { key: "speed", label: "Speed", min: 0, max: 3, step: 0.05, value: 1 },
        { key: "scale", label: "Scale", min: 0.3, max: 3, step: 0.05, value: 1 },
      ],
      custom: true,
    }))
    return [...BUILTIN_BACKGROUNDS, ...custom]
  }

  private definition(id: string): BackgroundDef | undefined {
    return this.allBackgrounds().find((background) => background.id === id)
  }

  private persist(): void {
    saveSettings(this.settings)
  }

  private applyGlobals(): void {
    // Kept on <html> rather than on the canvas: the SPA router strips inline
    // styles from everything inside <body>, but never touches the root
    // element, so state parked there survives a navigation for free.
    document.documentElement.style.setProperty("--bg-opacity", String(this.settings.opacity))
    if (!this.animating) this.stop()
    else if (this.program || this.fluid) this.start()
  }

  /** Switches background, falling back to the DOM one if the GPU path fails. */
  private select(id: string, initial = false): void {
    const background = this.definition(id) ?? this.definition(DOM_BACKGROUND_ID)!
    this.settings.selected = background.id
    if (!initial) this.persist()

    this.teardownGl()

    // CSS decides which of the two backgrounds is on screen, keyed off an
    // attribute on <html>, for the same reason the opacity lives there.
    const useDom = background.kind === "dom"
    document.documentElement.dataset.bg = useDom ? "dom" : "gl"

    if (useDom) {
      this.stop()
      this.menu?.refresh()
      return
    }

    const gl = this.acquireContext()
    if (!gl) {
      // Falling back keeps the page readable, but doing it without a word is
      // indistinguishable from a menu that ignores clicks, so say why.
      this.fallback("This browser has no WebGL2, so only Dappled Light works here.")
      return
    }

    if (background.kind === "fluid") {
      this.fluid = new FluidSimulation(gl)
      if (this.fluid.unavailable) {
        this.fluid = null
        this.fallback("Fluid needs float render targets, which this GPU does not expose.")
        return
      }
      // Forced: a sim switched in at an unchanged canvas size would otherwise
      // never be given its render targets, and the first step would fault.
      this.resize(true)
    } else {
      const { program, error } = linkProgram(gl, buildFragmentSource(background.fragment ?? ""))
      if (!program) {
        console.error(`background shader "${background.id}" failed to compile:\n${error}`)
        this.fallback(`"${background.name}" did not compile on this GPU.`)
        return
      }
      this.program = program
      this.uniforms = new UniformCache(gl, program)
      this.resize(true)
    }

    this.frame = 0
    this.startTime = performance.now()
    this.lastTime = this.startTime
    this.menu?.setNotice(null)
    // Paint one frame now rather than waiting on the loop. A background tab
    // schedules no animation frames at all, and a background held still —
    // reduced motion, or Animate switched off — would never get one either, so
    // picking a background would look like nothing happened.
    this.renderOnce()
    if (this.animating) this.start()
    this.menu?.refresh()
  }

  /** Reverts to the DOM background and tells the menu what went wrong. */
  private fallback(reason: string): void {
    this.select(DOM_BACKGROUND_ID)
    this.menu?.setNotice(reason)
  }

  private acquireContext(): WebGL2RenderingContext | null {
    if (this.gl) return this.gl
    this.gl = this.canvas.getContext("webgl2", {
      alpha: false,
      antialias: false,
      depth: false,
      stencil: false,
      powerPreference: "low-power",
      preserveDrawingBuffer: false,
    })
    return this.gl
  }

  private teardownGl(): void {
    if (this.gl && this.program) this.gl.deleteProgram(this.program)
    this.program = null
    this.uniforms = null
    // The sim holds a dozen float render targets. Dropping the reference alone
    // leaks them for as long as the context lives, which is the whole session.
    this.fluid?.dispose()
    this.fluid = null
  }

  /** Compiles a source without installing it, for the menu's paste box. */
  private validate(source: string): string | null {
    const gl = this.acquireContext()
    if (!gl) return "WebGL2 is not available in this browser."
    const { program, error } = linkProgram(gl, buildFragmentSource(source))
    if (program) gl.deleteProgram(program)
    return program ? null : (error ?? "unknown compile error")
  }

  private addCustom(name: string, source: string): string | null {
    const id = `custom-${Date.now().toString(36)}`
    this.settings.custom.push({ id, name, source })
    this.persist()
    this.select(id)
    return id
  }

  private deleteCustom(id: string): void {
    this.settings.custom = this.settings.custom.filter((record) => record.id !== id)
    delete this.settings.params[id]
    if (this.settings.selected === id) this.select(DOM_BACKGROUND_ID)
    this.persist()
  }

  private watchTheme(): void {
    const read = () => {
      this.themeTarget = currentTheme()
    }
    // Quartz stores the choice on <html saved-theme>, and also emits an event.
    new MutationObserver(read).observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["saved-theme"],
    })
    document.addEventListener("themechange", read)
    window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", read)
  }

  private watchPointer(): void {
    const update = (x: number, y: number) => {
      const rect = this.canvas.getBoundingClientRect()
      this.pointer.px = this.pointer.x
      this.pointer.py = this.pointer.y
      this.pointer.x = x - rect.left
      this.pointer.y = rect.height - (y - rect.top)
      this.pointer.moved = true
    }
    window.addEventListener("pointermove", (event) => update(event.clientX, event.clientY), {
      passive: true,
    })
    window.addEventListener("pointerdown", (event) => {
      this.pointer.down = 1
      update(event.clientX, event.clientY)
    })
    window.addEventListener("pointerup", () => {
      this.pointer.down = 0
    })
  }

  private watchResize(): void {
    let pending = 0
    const schedule = () => {
      cancelAnimationFrame(pending)
      pending = requestAnimationFrame(() => this.resize())
    }
    window.addEventListener("resize", schedule, { passive: true })
    // A window that has not been laid out yet — a hidden tab, a pane opened
    // later — reports zero size and never fires `resize`, which would leave
    // the canvas stuck at 1x1. Watching the element covers that too.
    new ResizeObserver(schedule).observe(document.documentElement)
  }

  private watchNav(): void {
    // Quartz's SPA router morphs <body> against the incoming page. The canvas
    // and the menu host are in that markup, so the diff lines up, but the
    // router still strips the attributes this class wrote and empties the
    // menu. Put both back once the new page is in place.
    document.addEventListener("nav", () => {
      const canvas = document.getElementById("shader-bg")
      if (canvas instanceof HTMLCanvasElement && canvas !== this.canvas) {
        // A replaced canvas takes the WebGL context with it; swap ours back in
        // rather than paying to rebuild the context on every navigation.
        canvas.replaceWith(this.canvas)
      }
      this.menu?.mount(document.getElementById("bg-menu"))
      this.resize(true)
    })
  }

  private watchVisibility(): void {
    // A background nobody can see should not be burning battery.
    document.addEventListener("visibilitychange", () => {
      if (document.hidden) this.stop()
      else if (this.animating && (this.program || this.fluid)) this.start()
      else if (this.program || this.fluid) this.renderOnce()
    })
  }

  /** `force` re-applies the size even when it looks unchanged, which it does
   * after a navigation strips the canvas's width and height attributes. */
  private resize(force = false): void {
    const gl = this.gl
    if (!gl) return
    const ratio = Math.min(window.devicePixelRatio || 1, 2)
    const width = Math.max(1, Math.floor(window.innerWidth * ratio))
    const height = Math.max(1, Math.floor(window.innerHeight * ratio))
    if (!force && this.canvas.width === width && this.canvas.height === height) return

    this.canvas.width = width
    this.canvas.height = height
    gl.viewport(0, 0, width, height)
    this.fluid?.resize(width, height)
    // Re-sizing clears the drawing buffer, so repaint straight away. Waiting
    // for the loop costs a frame of black on every navigation, and a
    // background held still would never get that frame at all.
    if (this.program || this.fluid) this.renderOnce()
  }

  private start(): void {
    if (this.running) return
    this.running = true
    this.lastTime = performance.now()
    this.rafHandle = requestAnimationFrame(this.tick)
  }

  private stop(): void {
    this.running = false
    cancelAnimationFrame(this.rafHandle)
  }

  /** Draws exactly one frame, for the backgrounds that are not animating. */
  private renderOnce(): void {
    const now = performance.now()
    this.themeCurrent = this.themeTarget
    if (this.fluid) this.renderFluid(1 / 60)
    else this.renderShader(((now - this.startTime) / 1000) * this.settings.speed, 1 / 60)
  }

  private tick = (now: number): void => {
    if (!this.running) return
    this.rafHandle = requestAnimationFrame(this.tick)

    const rawDelta = (now - this.lastTime) / 1000
    this.lastTime = now
    // Clamp so a backgrounded tab returning does not advance the sim by a
    // second in one step and blow it up.
    const delta = Math.min(rawDelta, 1 / 20) * this.settings.speed
    const elapsed = ((now - this.startTime) / 1000) * this.settings.speed

    // Ease the theme so a light/dark flip cross-fades with the page transition.
    this.themeCurrent += (this.themeTarget - this.themeCurrent) * Math.min(1, delta * 4)

    if (this.fluid) this.renderFluid(delta)
    else this.renderShader(elapsed, delta)

    this.frame += 1
  }

  private renderShader(elapsed: number, delta: number): void {
    const gl = this.gl
    if (!gl || !this.program || !this.uniforms) return

    gl.useProgram(this.program)
    this.uniforms.vec3("iResolution", this.canvas.width, this.canvas.height, 1)
    this.uniforms.float("iTime", elapsed)
    this.uniforms.float("iTimeDelta", delta)
    this.uniforms.int("iFrame", this.frame)
    this.uniforms.vec4(
      "iMouse",
      this.pointer.x * (this.canvas.width / window.innerWidth),
      this.pointer.y * (this.canvas.height / window.innerHeight),
      this.pointer.down,
      0,
    )
    this.uniforms.float("uTheme", this.themeCurrent)

    const background = this.definition(this.settings.selected)
    const overrides = this.settings.params[this.settings.selected] ?? {}
    for (const param of background?.params ?? []) {
      this.uniforms.float(`u_${param.key}`, overrides[param.key] ?? param.value)
    }

    gl.bindFramebuffer(gl.FRAMEBUFFER, null)
    gl.viewport(0, 0, this.canvas.width, this.canvas.height)
    gl.drawArrays(gl.TRIANGLES, 0, 3)
  }

  private renderFluid(delta: number): void {
    if (!this.fluid) return
    const background = this.definition(this.settings.selected)
    const overrides = this.settings.params[this.settings.selected] ?? {}
    const value = (key: string, fallback: number) =>
      overrides[key] ?? background?.params.find((p) => p.key === key)?.value ?? fallback

    const force = value("force", 5200)
    const radius = value("radius", 0.3)

    if (this.pointer.moved) {
      const dx = (this.pointer.x - this.pointer.px) / window.innerWidth
      const dy = (this.pointer.y - this.pointer.py) / window.innerHeight
      if (Math.abs(dx) > 1e-5 || Math.abs(dy) > 1e-5) {
        const hue = (performance.now() / 3000) % 1
        this.fluid.splat(
          this.pointer.x / window.innerWidth,
          this.pointer.y / window.innerHeight,
          dx * force,
          dy * force,
          hueToRgb(hue),
          radius,
        )
      }
      this.pointer.moved = false
    }

    this.fluid.step(delta * 60, {
      dissipation: value("dissipation", 0.985),
      force,
      radius,
    })
    this.fluid.render(this.themeCurrent)
  }
}

/**
 * The canvas is in every page's markup, at a fixed position in `<body>`.
 *
 * That position is the point: the SPA router pairs the old and new body's
 * children by index, so a canvas prepended at runtime shifts every sibling by
 * one and the morph rebuilds — and discards — the entire page, background and
 * picker included.
 */
function adoptCanvas(): HTMLCanvasElement {
  const existing = document.getElementById("shader-bg")
  if (existing instanceof HTMLCanvasElement) return existing
  const created = document.createElement("canvas")
  created.id = "shader-bg"
  created.setAttribute("aria-hidden", "true")
  document.body.prepend(created)
  return created
}

function currentTheme(): number {
  const saved = document.documentElement.getAttribute("saved-theme")
  if (saved === "dark") return 1
  if (saved === "light") return 0
  return window.matchMedia("(prefers-color-scheme: dark)").matches ? 1 : 0
}

/** Cheap HSV-to-RGB at full saturation, for the fluid's rotating ink colour. */
function hueToRgb(hue: number): [number, number, number] {
  const f = (n: number) => {
    const k = (n + hue * 6) % 6
    return Math.max(0, Math.min(1, Math.min(k, 4 - k, 1)))
  }
  return [f(5) * 0.6, f(3) * 0.6, f(1) * 0.6]
}

function boot(): void {
  new BackgroundController()
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", boot, { once: true })
} else {
  boot()
}
