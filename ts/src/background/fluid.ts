/**
 * A pointer-driven fluid simulation.
 *
 * This is the one background that cannot be a single fragment shader: it keeps
 * state between frames and needs several passes per frame. The scheme is the
 * standard Stam "Stable Fluids" split — advect velocity, add a force splat,
 * compute divergence, relax pressure with Jacobi iterations, subtract the
 * pressure gradient to make the field divergence-free, then advect dye through
 * the result.
 *
 * It runs at half resolution for the simulation and full resolution only for
 * the dye, which is where all the visible detail is. On a background that
 * nobody is looking at directly, that trade is invisible and roughly quarters
 * the per-frame cost.
 */

import { buildFragmentSource, linkProgram } from "./renderer.js"
import { FluidStyle } from "./types.js"

const ADVECT = `
uniform sampler2D uSource;
uniform sampler2D uVelocity;
uniform vec2 uTexelSize;
uniform float uDt;
uniform float uDissipation;

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
  vec2 uv = fragCoord / iResolution.xy;
  // Trace backwards along the velocity field and sample where the parcel came
  // from; that is what makes the scheme unconditionally stable.
  vec2 coord = uv - uDt * texture(uVelocity, uv).xy * uTexelSize;
  fragColor = uDissipation * texture(uSource, coord);
  fragColor.a = 1.0;
}
`

const DIVERGENCE = `
uniform sampler2D uVelocity;
uniform vec2 uTexelSize;

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
  vec2 uv = fragCoord / iResolution.xy;
  float left = texture(uVelocity, uv - vec2(uTexelSize.x, 0.0)).x;
  float right = texture(uVelocity, uv + vec2(uTexelSize.x, 0.0)).x;
  float bottom = texture(uVelocity, uv - vec2(0.0, uTexelSize.y)).y;
  float top = texture(uVelocity, uv + vec2(0.0, uTexelSize.y)).y;
  fragColor = vec4(0.5 * (right - left + top - bottom), 0.0, 0.0, 1.0);
}
`

const PRESSURE = `
uniform sampler2D uPressure;
uniform sampler2D uDivergence;
uniform vec2 uTexelSize;

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
  vec2 uv = fragCoord / iResolution.xy;
  float left = texture(uPressure, uv - vec2(uTexelSize.x, 0.0)).x;
  float right = texture(uPressure, uv + vec2(uTexelSize.x, 0.0)).x;
  float bottom = texture(uPressure, uv - vec2(0.0, uTexelSize.y)).x;
  float top = texture(uPressure, uv + vec2(0.0, uTexelSize.y)).x;
  float divergence = texture(uDivergence, uv).x;
  fragColor = vec4((left + right + bottom + top - divergence) * 0.25, 0.0, 0.0, 1.0);
}
`

const GRADIENT_SUBTRACT = `
uniform sampler2D uPressure;
uniform sampler2D uVelocity;
uniform vec2 uTexelSize;

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
  vec2 uv = fragCoord / iResolution.xy;
  float left = texture(uPressure, uv - vec2(uTexelSize.x, 0.0)).x;
  float right = texture(uPressure, uv + vec2(uTexelSize.x, 0.0)).x;
  float bottom = texture(uPressure, uv - vec2(0.0, uTexelSize.y)).x;
  float top = texture(uPressure, uv + vec2(0.0, uTexelSize.y)).x;
  vec2 velocity = texture(uVelocity, uv).xy - vec2(right - left, top - bottom);
  fragColor = vec4(velocity, 0.0, 1.0);
}
`

const SPLAT = `
uniform sampler2D uTarget;
uniform vec2 uPoint;
uniform vec3 uColor;
uniform float uRadius;
uniform float uAspect;

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
  vec2 uv = fragCoord / iResolution.xy;
  vec2 offset = uv - uPoint;
  offset.x *= uAspect;
  vec3 splat = exp(-dot(offset, offset) / uRadius) * uColor;
  fragColor = vec4(texture(uTarget, uv).xyz + splat, 1.0);
}
`

const DISPLAY = `
uniform sampler2D uDye;

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
  vec2 uv = fragCoord / iResolution.xy;
  vec3 dye = texture(uDye, uv).rgb;
  float density = clamp(length(dye), 0.0, 1.0);

  vec3 background = themed(vec3(0.98, 0.97, 0.94), vec3(0.03, 0.04, 0.07));
  // In light mode the ink darkens the page; in dark mode it glows.
  vec3 ink = themed(vec3(0.42, 0.30, 0.18), vec3(0.55, 0.85, 1.00));
  vec3 tint = normalize(dye + 1e-4) * 0.5 + 0.5;

  vec3 col = mix(background, ink * tint * 1.6, density);
  fragColor = vec4(col, 1.0);
}
`

interface Framebuffer {
  texture: WebGLTexture
  fbo: WebGLFramebuffer
  width: number
  height: number
}

interface DoubleBuffer {
  read: Framebuffer
  write: Framebuffer
  swap(): void
}

export interface FluidOptions {
  dissipation: number
  force: number
  radius: number
  /** taiji: how fast the dye is pulled back to the image it was seeded with. */
  restore: number
  /** convection: upward force per unit of heat. */
  buoyancy: number
  /** convection: strength of the hot floor and the cold ceiling. */
  heat: number
}

/**
 * Seeds the dye with a taijitu.
 *
 * One signed channel carries the whole image: +1 is white ink, -1 is black,
 * and 0 is bare page. That is what lets the sim smear the two inks through
 * each other and still know which is which — a three-channel colour would go
 * muddy grey at the boundary instead of staying a boundary.
 */
const SEED_TAIJI = `
float taijiAt(vec2 p) {
  float r = 0.30;
  if (length(p) > r) return 0.0;

  float upper = length(p - vec2(0.0, r * 0.5));
  float lower = length(p + vec2(0.0, r * 0.5));

  // Split the disc down the middle, let two half-radius circles carry each
  // half back across the seam, then punch the two eyes.
  float v = p.x > 0.0 ? 1.0 : -1.0;
  if (upper < r * 0.5) v = 1.0;
  if (lower < r * 0.5) v = -1.0;
  if (upper < r * 0.16) v = -1.0;
  if (lower < r * 0.16) v = 1.0;
  return v;
}

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
  vec2 p = (fragCoord - 0.5 * iResolution.xy) / min(iResolution.x, iResolution.y);
  // Supersampled, because every edge here is a hard step and this pass runs
  // once — the cost is nothing and the jaggies would advect around for ever.
  vec2 e = vec2(0.5) / min(iResolution.x, iResolution.y);
  float v = 0.25 * (taijiAt(p + vec2(-e.x, -e.y)) + taijiAt(p + vec2(e.x, -e.y)) +
                    taijiAt(p + vec2(-e.x, e.y)) + taijiAt(p + vec2(e.x, e.y)));
  fragColor = vec4(v, 0.0, 0.0, 1.0);
}
`

const DISPLAY_TAIJI = `
uniform sampler2D uDye;

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
  vec2 uv = fragCoord / iResolution.xy;
  float v = texture(uDye, uv).r;

  // Magnitude is how much ink is here, sign is which ink.
  float coverage = smoothstep(0.10, 0.38, abs(v));
  float tone = smoothstep(-0.10, 0.10, v);

  vec3 page = themed(vec3(0.97, 0.96, 0.94), vec3(0.04, 0.05, 0.07));
  vec3 col = mix(page, mix(vec3(0.03), vec3(0.97), tone), coverage);

  // A hairline where the two inks meet, so the boundary stays readable however
  // far the flow has pulled it out of shape.
  float seam = coverage * (1.0 - smoothstep(0.0, 0.14, abs(v)));
  col = mix(col, themed(vec3(0.45), vec3(0.55)), seam * 0.6);
  fragColor = vec4(col, 1.0);
}
`

/**
 * Advects the dye, then pulls it back toward the image it was seeded with.
 *
 * The plain advect pass decays toward zero, which for a picture means it fades
 * to blank. Decaying toward a stored target instead means the field has a rest
 * state to fall into, so a taijitu left alone reassembles itself.
 */
const ADVECT_RESTORE = `
uniform sampler2D uSource;
uniform sampler2D uVelocity;
uniform sampler2D uRest;
uniform vec2 uTexelSize;
uniform float uDt;
uniform float uRestore;

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
  vec2 uv = fragCoord / iResolution.xy;
  vec2 coord = uv - uDt * texture(uVelocity, uv).xy * uTexelSize;
  float advected = texture(uSource, coord).r;
  float settled = mix(advected, texture(uRest, uv).r, clamp(uRestore * uDt, 0.0, 1.0));
  fragColor = vec4(settled, 0.0, 0.0, 1.0);
}
`

/**
 * Rayleigh-Benard convection: heat transported by the flow it is driving.
 *
 * The cell is heated along the floor and cooled along the ceiling, and nothing
 * else pushes the fluid at all — every plume here is the fluid overturning
 * under its own buoyancy.
 */
const ADVECT_HEAT = `
uniform sampler2D uSource;
uniform sampler2D uVelocity;
uniform vec2 uTexelSize;
uniform float uDt;
uniform float uHeat;

// Slow bulk cooling, so a plume gives its heat up to the fluid on the way and
// the cell does not simply fill with warm.
const float COOL = 0.01;

// How fast a plate imposes its temperature on the fluid touching it. Fixed,
// because above about a half it stops changing what the cell looks like — the
// knob worth exposing is how hot the plate is, not how quickly it wins.
const float PLATE_RATE = 0.5;

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
  vec2 uv = fragCoord / iResolution.xy;
  vec2 coord = uv - uDt * texture(uVelocity, uv).xy * uTexelSize;
  float heat = texture(uSource, coord).r;

  // Plates held at a temperature, rather than plates that add heat without
  // limit. The difference decides whether this convects at all: unbounded
  // heating drives every column into the clamp, a floor that is uniformly at
  // the clamp has no lateral variation left, and a buoyancy force with no
  // lateral variation is curl-free — the projection removes all of it. What
  // makes a plume is an *uneven* plate, not a hot one.
  float bias = (0.75 + 0.5 * hash21(vec2(floor(uv.x * 48.0), 7.0))) * uHeat;
  float floorBand = smoothstep(0.06, 0.0, uv.y);
  float ceilBand = smoothstep(0.94, 1.0, uv.y);
  float rate = clamp(PLATE_RATE * uDt, 0.0, 1.0);
  heat = mix(heat, bias, floorBand * rate);
  heat = mix(heat, -uHeat, ceilBand * rate);
  heat -= heat * clamp(COOL * uDt, 0.0, 1.0);

  fragColor = vec4(clamp(heat, -2.0, 2.0), 0.0, 0.0, 1.0);
}
`

const BUOYANCY = `
uniform sampler2D uVelocity;
uniform sampler2D uHeat;
uniform float uDt;
uniform float uBuoyancy;

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
  vec2 uv = fragCoord / iResolution.xy;
  vec2 velocity = texture(uVelocity, uv).xy;
  // Boussinesq: density differences matter only through the force they exert,
  // so warm fluid is simply pushed up in proportion to how warm it is.
  velocity.y += texture(uHeat, uv).r * uBuoyancy * uDt;
  fragColor = vec4(velocity, 0.0, 1.0);
}
`

const SEED_HEAT = `
void mainImage(out vec4 fragColor, in vec2 fragCoord) {
  vec2 uv = fragCoord / iResolution.xy;
  // Start with a warm, already uneven lower layer, so the first overturn
  // happens within a second of the page opening instead of a minute later.
  float base = smoothstep(0.55, 0.0, uv.y) * 0.6;
  fragColor = vec4(base * (0.6 + 0.8 * noise(uv * 12.0)), 0.0, 0.0, 1.0);
}
`

const DISPLAY_CONVECTION = `
uniform sampler2D uDye;

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
  vec2 uv = fragCoord / iResolution.xy;
  float level = clamp(texture(uDye, uv).r * 0.5 + 0.5, 0.0, 1.0);

  vec3 cold = themed(vec3(0.95, 0.96, 0.98), vec3(0.02, 0.03, 0.06));
  vec3 mid = themed(vec3(0.82, 0.83, 0.88), vec3(0.08, 0.11, 0.20));
  vec3 warm = themed(vec3(0.86, 0.66, 0.46), vec3(0.30, 0.19, 0.26));
  vec3 hot = themed(vec3(0.96, 0.85, 0.62), vec3(0.62, 0.42, 0.26));

  vec3 col = level < 0.5
    ? mix(cold, mid, level * 2.0)
    : mix(mid, mix(warm, hot, smoothstep(0.72, 1.0, level)), (level - 0.5) * 2.0);
  fragColor = vec4(col, 1.0);
}
`

const DISPLAYS: Record<FluidStyle, string> = {
  ink: DISPLAY,
  taiji: DISPLAY_TAIJI,
  convection: DISPLAY_CONVECTION,
}

export class FluidSimulation {
  private programs = new Map<string, WebGLProgram>()
  private velocity!: DoubleBuffer
  private dye!: DoubleBuffer
  private divergence!: Framebuffer
  private pressure!: DoubleBuffer
  /** The image the taiji style relaxes back toward. Null for other styles. */
  private rest: Framebuffer | null = null
  private simWidth = 0
  private simHeight = 0
  private dyeWidth = 0
  private dyeHeight = 0
  private failed = false
  private sized = false

  constructor(
    private gl: WebGL2RenderingContext,
    private style: FluidStyle = "ink",
  ) {
    // Float render targets are what make the sim stable; without them the
    // velocity field quantises and the flow visibly stair-steps.
    if (!gl.getExtension("EXT_color_buffer_float")) {
      this.failed = true
      return
    }
    gl.getExtension("OES_texture_float_linear")

    const passes: [string, string][] = [
      ["advect", ADVECT],
      ["divergence", DIVERGENCE],
      ["pressure", PRESSURE],
      ["gradient", GRADIENT_SUBTRACT],
      ["splat", SPLAT],
      ["display", DISPLAYS[style]],
    ]
    if (style === "taiji") passes.push(["seed", SEED_TAIJI], ["restore", ADVECT_RESTORE])
    if (style === "convection")
      passes.push(["seed", SEED_HEAT], ["heat", ADVECT_HEAT], ["buoyancy", BUOYANCY])

    for (const [name, body] of passes) {
      const program = this.build(body)
      if (!program) {
        this.failed = true
        return
      }
      this.programs.set(name, program)
    }
  }

  get unavailable(): boolean {
    return this.failed
  }

  private build(body: string): WebGLProgram | null {
    // Reuses the main renderer's Shadertoy wrapper so these passes get the same
    // prelude (iResolution, uTheme, the `themed` helper) as everything else.
    const { program } = linkProgram(this.gl, buildFragmentSource(body))
    return program
  }

  resize(width: number, height: number): void {
    if (this.failed) return
    const gl = this.gl
    // New render targets mean the old ones are unreachable; without this the
    // sim leaks its whole working set once per resize event.
    this.releaseTargets()
    const simScale = 0.25
    this.simWidth = Math.max(2, Math.floor(width * simScale))
    this.simHeight = Math.max(2, Math.floor(height * simScale))
    this.dyeWidth = Math.max(2, Math.floor(width * 0.5))
    this.dyeHeight = Math.max(2, Math.floor(height * 0.5))

    this.velocity = this.createDouble(this.simWidth, this.simHeight, gl.RG16F, gl.RG, gl.HALF_FLOAT)
    this.dye = this.createDouble(this.dyeWidth, this.dyeHeight, gl.RGBA16F, gl.RGBA, gl.HALF_FLOAT)
    this.divergence = this.createSingle(
      this.simWidth,
      this.simHeight,
      gl.R16F,
      gl.RED,
      gl.HALF_FLOAT,
    )
    this.pressure = this.createDouble(this.simWidth, this.simHeight, gl.R16F, gl.RED, gl.HALF_FLOAT)
    if (this.style === "taiji") {
      this.rest = this.createSingle(
        this.dyeWidth,
        this.dyeHeight,
        gl.RGBA16F,
        gl.RGBA,
        gl.HALF_FLOAT,
      )
    }
    this.sized = true
    this.seed()
  }

  /**
   * Draws the starting image into the dye, if this style has one.
   *
   * Runs on every resize because the buffers are new — which also means a
   * window resize hands back a fresh, unsmeared taijitu.
   */
  private seed(): void {
    const program = this.programs.get("seed")
    if (!program) return
    this.gl.useProgram(program)
    // Once into the rest target the restore pass pulls back toward, and once
    // into the live dye so the page opens on the finished image.
    if (this.rest !== null) this.draw(this.rest, program)
    this.draw(this.dye.write, program)
    this.dye.swap()
  }

  /** Frees every render target, leaving the compiled programs in place. */
  private releaseTargets(): void {
    if (!this.sized) return
    const gl = this.gl
    for (const target of [
      this.velocity.read,
      this.velocity.write,
      this.dye.read,
      this.dye.write,
      this.pressure.read,
      this.pressure.write,
      this.divergence,
    ]) {
      gl.deleteTexture(target.texture)
      gl.deleteFramebuffer(target.fbo)
    }
    if (this.rest !== null) {
      gl.deleteTexture(this.rest.texture)
      gl.deleteFramebuffer(this.rest.fbo)
      this.rest = null
    }
    this.sized = false
  }

  /** Releases everything. The instance is unusable afterwards. */
  dispose(): void {
    const gl = this.gl
    this.releaseTargets()
    for (const program of this.programs.values()) gl.deleteProgram(program)
    this.programs.clear()
    // Reported as unavailable so a stray call after teardown is a no-op rather
    // than a draw against deleted objects.
    this.failed = true
  }

  private createSingle(
    width: number,
    height: number,
    internal: number,
    format: number,
    type: number,
  ): Framebuffer {
    const gl = this.gl
    const texture = gl.createTexture()!
    gl.bindTexture(gl.TEXTURE_2D, texture)
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR)
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR)
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)
    gl.texImage2D(gl.TEXTURE_2D, 0, internal, width, height, 0, format, type, null)

    const fbo = gl.createFramebuffer()!
    gl.bindFramebuffer(gl.FRAMEBUFFER, fbo)
    gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, texture, 0)
    gl.clearColor(0, 0, 0, 1)
    gl.clear(gl.COLOR_BUFFER_BIT)
    return { texture, fbo, width, height }
  }

  private createDouble(
    width: number,
    height: number,
    internal: number,
    format: number,
    type: number,
  ): DoubleBuffer {
    const first = this.createSingle(width, height, internal, format, type)
    const second = this.createSingle(width, height, internal, format, type)
    return {
      read: first,
      write: second,
      swap() {
        const temporary = this.read
        this.read = this.write
        this.write = temporary
      },
    }
  }

  private draw(target: Framebuffer | null, program: WebGLProgram): void {
    const gl = this.gl
    gl.bindFramebuffer(gl.FRAMEBUFFER, target ? target.fbo : null)
    const width = target ? target.width : gl.drawingBufferWidth
    const height = target ? target.height : gl.drawingBufferHeight
    gl.viewport(0, 0, width, height)
    gl.useProgram(program)
    gl.uniform3f(gl.getUniformLocation(program, "iResolution"), width, height, 1)
    gl.drawArrays(gl.TRIANGLES, 0, 3)
  }

  private bind(program: WebGLProgram, name: string, texture: WebGLTexture, unit: number): void {
    const gl = this.gl
    gl.activeTexture(gl.TEXTURE0 + unit)
    gl.bindTexture(gl.TEXTURE_2D, texture)
    gl.uniform1i(gl.getUniformLocation(program, name), unit)
  }

  /** Injects momentum and colour at a point, in 0–1 screen coordinates. */
  splat(
    x: number,
    y: number,
    dx: number,
    dy: number,
    color: [number, number, number],
    radius: number,
  ): void {
    if (this.failed) return
    const gl = this.gl
    const program = this.programs.get("splat")!
    const aspect = this.dyeWidth / this.dyeHeight

    gl.useProgram(program)
    this.bind(program, "uTarget", this.velocity.read.texture, 0)
    gl.uniform2f(gl.getUniformLocation(program, "uPoint"), x, y)
    gl.uniform3f(gl.getUniformLocation(program, "uColor"), dx, dy, 0)
    gl.uniform1f(gl.getUniformLocation(program, "uRadius"), radius / 100)
    gl.uniform1f(gl.getUniformLocation(program, "uAspect"), aspect)
    this.draw(this.velocity.write, program)
    this.velocity.swap()

    gl.useProgram(program)
    this.bind(program, "uTarget", this.dye.read.texture, 0)
    gl.uniform3f(gl.getUniformLocation(program, "uColor"), color[0], color[1], color[2])
    this.draw(this.dye.write, program)
    this.dye.swap()
  }

  step(dt: number, options: FluidOptions): void {
    if (this.failed) return
    const gl = this.gl
    const texelX = 1 / this.simWidth
    const texelY = 1 / this.simHeight

    const advect = this.programs.get("advect")!
    gl.useProgram(advect)
    gl.uniform2f(gl.getUniformLocation(advect, "uTexelSize"), texelX, texelY)
    gl.uniform1f(gl.getUniformLocation(advect, "uDt"), dt)
    gl.uniform1f(gl.getUniformLocation(advect, "uDissipation"), options.dissipation)
    this.bind(advect, "uVelocity", this.velocity.read.texture, 0)
    this.bind(advect, "uSource", this.velocity.read.texture, 1)
    this.draw(this.velocity.write, advect)
    this.velocity.swap()

    // Buoyancy goes in before the projection, so the pressure solve gets to
    // enforce incompressibility on a field that already knows about the heat.
    if (this.style === "convection") {
      const buoyancy = this.programs.get("buoyancy")!
      gl.useProgram(buoyancy)
      gl.uniform1f(gl.getUniformLocation(buoyancy, "uDt"), dt)
      gl.uniform1f(gl.getUniformLocation(buoyancy, "uBuoyancy"), options.buoyancy)
      this.bind(buoyancy, "uVelocity", this.velocity.read.texture, 0)
      this.bind(buoyancy, "uHeat", this.dye.read.texture, 1)
      this.draw(this.velocity.write, buoyancy)
      this.velocity.swap()
    }

    const divergence = this.programs.get("divergence")!
    gl.useProgram(divergence)
    gl.uniform2f(gl.getUniformLocation(divergence, "uTexelSize"), texelX, texelY)
    this.bind(divergence, "uVelocity", this.velocity.read.texture, 0)
    this.draw(this.divergence, divergence)

    const pressure = this.programs.get("pressure")!
    gl.useProgram(pressure)
    gl.uniform2f(gl.getUniformLocation(pressure, "uTexelSize"), texelX, texelY)
    this.bind(pressure, "uDivergence", this.divergence.texture, 1)
    // Twenty Jacobi iterations is the usual visual-quality plateau; more costs
    // frames without looking different.
    for (let i = 0; i < 20; i++) {
      this.bind(pressure, "uPressure", this.pressure.read.texture, 0)
      this.draw(this.pressure.write, pressure)
      this.pressure.swap()
    }

    const gradient = this.programs.get("gradient")!
    gl.useProgram(gradient)
    gl.uniform2f(gl.getUniformLocation(gradient, "uTexelSize"), texelX, texelY)
    this.bind(gradient, "uPressure", this.pressure.read.texture, 0)
    this.bind(gradient, "uVelocity", this.velocity.read.texture, 1)
    this.draw(this.velocity.write, gradient)
    this.velocity.swap()

    // What the flow carries, which is the only part that differs by style.
    if (this.style === "taiji") {
      const restore = this.programs.get("restore")!
      gl.useProgram(restore)
      gl.uniform2f(gl.getUniformLocation(restore, "uTexelSize"), texelX, texelY)
      gl.uniform1f(gl.getUniformLocation(restore, "uDt"), dt)
      gl.uniform1f(gl.getUniformLocation(restore, "uRestore"), options.restore)
      this.bind(restore, "uVelocity", this.velocity.read.texture, 0)
      this.bind(restore, "uSource", this.dye.read.texture, 1)
      this.bind(restore, "uRest", this.rest!.texture, 2)
      this.draw(this.dye.write, restore)
    } else if (this.style === "convection") {
      const heat = this.programs.get("heat")!
      gl.useProgram(heat)
      gl.uniform2f(gl.getUniformLocation(heat, "uTexelSize"), texelX, texelY)
      gl.uniform1f(gl.getUniformLocation(heat, "uDt"), dt)
      gl.uniform1f(gl.getUniformLocation(heat, "uHeat"), options.heat)
      this.bind(heat, "uVelocity", this.velocity.read.texture, 0)
      this.bind(heat, "uSource", this.dye.read.texture, 1)
      this.draw(this.dye.write, heat)
    } else {
      gl.useProgram(advect)
      gl.uniform2f(gl.getUniformLocation(advect, "uTexelSize"), texelX, texelY)
      gl.uniform1f(gl.getUniformLocation(advect, "uDissipation"), options.dissipation)
      this.bind(advect, "uVelocity", this.velocity.read.texture, 0)
      this.bind(advect, "uSource", this.dye.read.texture, 1)
      this.draw(this.dye.write, advect)
    }
    this.dye.swap()
  }

  render(theme: number): void {
    if (this.failed) return
    const gl = this.gl
    const display = this.programs.get("display")!
    gl.useProgram(display)
    gl.uniform1f(gl.getUniformLocation(display, "uTheme"), theme)
    this.bind(display, "uDye", this.dye.read.texture, 0)
    this.draw(null, display)
  }
}
