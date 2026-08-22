/**
 * WebGL2 renderer for the background shaders.
 *
 * Why WebGL2 and not WebGPU: this paints on every page load, so the cost of
 * being wrong is a blank or janky background on someone's first visit. WebGL2
 * is available essentially everywhere, and — more importantly — the shader
 * ecosystem people actually paste from (Shadertoy, oimo.io) is GLSL. Accepting
 * a pasted shader is a headline feature, so GLSL is the format that has to work
 * first.
 *
 * Everything renders as a single full-screen triangle. The vertex stage never
 * changes, so switching backgrounds only recompiles a fragment shader.
 */

import { GLSL_HELPERS } from "./shaders.js"

const VERTEX_SOURCE = `#version 300 es
precision highp float;
out vec2 vUv;
void main() {
  // One oversized triangle covers the clip volume with no index buffer.
  vec2 pos = vec2((gl_VertexID << 1) & 2, gl_VertexID & 2);
  vUv = pos;
  gl_Position = vec4(pos * 2.0 - 1.0, 0.0, 1.0);
}
`

/** Knob names always declared, so pasted shaders can use them too. */
const KNOB_UNIFORMS = ["speed", "scale", "amount", "size", "grain", "density"]

const FRAGMENT_PRELUDE = `#version 300 es
precision highp float;
precision highp int;

in vec2 vUv;
out vec4 outColor;

uniform vec3 iResolution;
uniform float iTime;
uniform float iTimeDelta;
uniform int iFrame;
uniform vec4 iMouse;
uniform float uTheme;
${KNOB_UNIFORMS.map((knob) => `uniform float u_${knob};`).join("\n")}
`

/** Wraps a Shadertoy-style body so it can be used as a real fragment shader. */
export function buildFragmentSource(body: string): string {
  const hasMainImage = /\bvoid\s+mainImage\s*\(/.test(body)
  const hasMain = /\bvoid\s+main\s*\(/.test(body)

  // A shader that already defines main() is used as-is; one that defines
  // mainImage() gets the Shadertoy entry wrapper. Shadertoy shaders are the
  // common case, so they need no editing at all.
  const epilogue = hasMainImage
    ? `
void main() {
  vec4 color = vec4(0.0, 0.0, 0.0, 1.0);
  mainImage(color, vUv * iResolution.xy);
  outColor = color;
}
`
    : hasMain
      ? ""
      : `
void main() { outColor = vec4(1.0, 0.0, 1.0, 1.0); }
`

  return `${FRAGMENT_PRELUDE}\n${GLSL_HELPERS}\n${body}\n${epilogue}`
}

export interface CompileResult {
  program: WebGLProgram | null
  error: string | null
}

/** Compiles one shader stage, returning the info log rather than throwing. */
function compileStage(gl: WebGL2RenderingContext, type: number, source: string): CompileResult {
  const shader = gl.createShader(type)
  if (!shader) return { program: null, error: "could not create shader" }

  gl.shaderSource(shader, source)
  gl.compileShader(shader)
  if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
    const log = gl.getShaderInfoLog(shader) ?? "unknown compile error"
    gl.deleteShader(shader)
    return { program: null, error: formatShaderError(log, source) }
  }
  return { program: shader as unknown as WebGLProgram, error: null }
}

/**
 * Rewrites a driver info log so line numbers point at the user's source.
 *
 * The prelude is prepended invisibly, so raw numbers are always wrong by its
 * length — which makes an otherwise fine error message actively misleading.
 */
function formatShaderError(log: string, source: string): string {
  const preludeLines = source.slice(0, source.indexOf(GLSL_HELPERS)).split("\n").length
  const helperLines = GLSL_HELPERS.split("\n").length
  const offset = preludeLines + helperLines - 2

  return log
    .split("\n")
    .filter((line) => line.trim().length > 0)
    .map((line) =>
      line.replace(/(\d+):(\d+)/, (match, column: string, row: string) => {
        const adjusted = Number(row) - offset
        return adjusted > 0 ? `${column}:${adjusted}` : match
      }),
    )
    .join("\n")
}

export function linkProgram(
  gl: WebGL2RenderingContext,
  fragmentSource: string,
): { program: WebGLProgram | null; error: string | null } {
  const vertex = compileStage(gl, gl.VERTEX_SHADER, VERTEX_SOURCE)
  if (!vertex.program) return { program: null, error: vertex.error }

  const fragment = compileStage(gl, gl.FRAGMENT_SHADER, fragmentSource)
  if (!fragment.program) {
    gl.deleteShader(vertex.program as unknown as WebGLShader)
    return { program: null, error: fragment.error }
  }

  const program = gl.createProgram()
  if (!program) return { program: null, error: "could not create program" }

  gl.attachShader(program, vertex.program as unknown as WebGLShader)
  gl.attachShader(program, fragment.program as unknown as WebGLShader)
  gl.linkProgram(program)
  gl.deleteShader(vertex.program as unknown as WebGLShader)
  gl.deleteShader(fragment.program as unknown as WebGLShader)

  if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
    const log = gl.getProgramInfoLog(program) ?? "unknown link error"
    gl.deleteProgram(program)
    return { program: null, error: log }
  }
  return { program, error: null }
}

/** Caches uniform locations so the draw loop does no string lookups. */
export class UniformCache {
  private locations = new Map<string, WebGLUniformLocation | null>()

  constructor(
    private gl: WebGL2RenderingContext,
    private program: WebGLProgram,
  ) {}

  location(name: string): WebGLUniformLocation | null {
    if (!this.locations.has(name)) {
      this.locations.set(name, this.gl.getUniformLocation(this.program, name))
    }
    return this.locations.get(name) ?? null
  }

  float(name: string, value: number): void {
    const location = this.location(name)
    if (location) this.gl.uniform1f(location, value)
  }

  int(name: string, value: number): void {
    const location = this.location(name)
    if (location) this.gl.uniform1i(location, value)
  }

  vec3(name: string, x: number, y: number, z: number): void {
    const location = this.location(name)
    if (location) this.gl.uniform3f(location, x, y, z)
  }

  vec4(name: string, x: number, y: number, z: number, w: number): void {
    const location = this.location(name)
    if (location) this.gl.uniform4f(location, x, y, z, w)
  }
}
