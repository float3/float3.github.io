/**
 * The built-in background shaders.
 *
 * Every shader is written against the Shadertoy contract — a
 * `mainImage(out vec4 fragColor, in vec2 fragCoord)` entry point with
 * `iResolution` / `iTime` / `iMouse` in scope — because that is the format
 * anything you paste in from Shadertoy or oimo.io will already be in. The
 * renderer supplies the prelude and the `main()` wrapper, so these bodies and a
 * pasted one go through exactly the same path.
 *
 * `uTheme` is this site's addition: 0.0 in light mode, 1.0 in dark, eased
 * between the two so a theme flip cross-fades instead of snapping.
 */

import { BackgroundDef } from "./types.js"

/** Shared helpers injected ahead of every shader body. */
export const GLSL_HELPERS = `
float hash11(float p) {
  p = fract(p * 0.1031);
  p *= p + 33.33;
  return fract(p * (p + p));
}

float hash21(vec2 p) {
  vec3 p3 = fract(vec3(p.xyx) * 0.1031);
  p3 += dot(p3, p3.yzx + 33.33);
  return fract((p3.x + p3.y) * p3.z);
}

float noise(vec2 p) {
  vec2 i = floor(p);
  vec2 f = fract(p);
  vec2 u = f * f * (3.0 - 2.0 * f);
  return mix(
    mix(hash21(i + vec2(0.0, 0.0)), hash21(i + vec2(1.0, 0.0)), u.x),
    mix(hash21(i + vec2(0.0, 1.0)), hash21(i + vec2(1.0, 1.0)), u.x),
    u.y);
}

float fbm(vec2 p) {
  float total = 0.0;
  float amplitude = 0.5;
  for (int i = 0; i < 5; i++) {
    total += noise(p) * amplitude;
    p *= 2.02;
    amplitude *= 0.5;
  }
  return total;
}

/** Mixes a light-mode and a dark-mode colour by the current theme. */
vec3 themed(vec3 lightColor, vec3 darkColor) {
  return mix(lightColor, darkColor, uTheme);
}
`

const AURORA = `
void mainImage(out vec4 fragColor, in vec2 fragCoord) {
  vec2 uv = fragCoord / iResolution.xy;
  vec2 p = (fragCoord - 0.5 * iResolution.xy) / iResolution.y;

  float t = iTime * 0.06 * u_speed;
  float bands = 0.0;
  for (float i = 0.0; i < 4.0; i++) {
    float offset = i * 0.35;
    float wave = fbm(vec2(p.x * 1.6 + t + offset, p.y * 0.9 - t * 0.6));
    float line = smoothstep(0.55, 0.0, abs(p.y + wave * u_amount - 0.15 + offset * 0.18));
    bands += line * (0.4 - i * 0.06);
  }

  vec3 warm = themed(vec3(0.98, 0.86, 0.68), vec3(0.13, 0.20, 0.36));
  vec3 cool = themed(vec3(0.86, 0.90, 0.98), vec3(0.05, 0.07, 0.12));
  vec3 glow = themed(vec3(1.00, 0.82, 0.58), vec3(0.36, 0.58, 0.86));

  vec3 col = mix(cool, warm, uv.y * 0.7 + 0.15);
  col += glow * bands;
  fragColor = vec4(col, 1.0);
}
`

const METABALLS = `
void mainImage(out vec4 fragColor, in vec2 fragCoord) {
  vec2 p = (fragCoord - 0.5 * iResolution.xy) / iResolution.y;
  vec2 mouse = (iMouse.xy - 0.5 * iResolution.xy) / iResolution.y;

  float t = iTime * 0.35 * u_speed;
  float field = 0.0;

  for (float i = 0.0; i < 6.0; i++) {
    float a = t + i * 1.0472;
    vec2 c = vec2(cos(a * 0.9 + i) * 0.55, sin(a * 1.1 + i * 1.7) * 0.35);
    field += u_size * 0.02 / (dot(p - c, p - c) + 0.006);
  }
  // The pointer is just one more, heavier, ball.
  field += u_size * 0.035 / (dot(p - mouse, p - mouse) + 0.008);

  float mask = smoothstep(0.85, 1.25, field);
  float rim = smoothstep(1.25, 0.9, field) * smoothstep(0.6, 1.0, field);

  vec3 bg = themed(vec3(0.97, 0.96, 0.93), vec3(0.05, 0.06, 0.09));
  vec3 body = themed(vec3(0.93, 0.79, 0.55), vec3(0.20, 0.38, 0.62));
  vec3 edge = themed(vec3(0.85, 0.55, 0.35), vec3(0.55, 0.80, 1.00));

  fragColor = vec4(mix(bg, body, mask) + edge * rim * 0.5, 1.0);
}
`

const PLASMA = `
void mainImage(out vec4 fragColor, in vec2 fragCoord) {
  vec2 uv = fragCoord / iResolution.xy;
  vec2 p = uv * u_scale * 3.0;
  float t = iTime * 0.25 * u_speed;

  float v = sin(p.x + t)
          + sin(p.y * 1.3 + t * 1.1)
          + sin((p.x + p.y) * 0.7 + t * 0.8)
          + sin(length(p - u_scale * 1.5) * 1.4 - t * 1.3);
  v *= 0.25;

  vec3 a = themed(vec3(0.99, 0.95, 0.88), vec3(0.04, 0.05, 0.10));
  vec3 b = themed(vec3(0.94, 0.80, 0.62), vec3(0.16, 0.26, 0.46));
  vec3 c = themed(vec3(0.80, 0.86, 0.95), vec3(0.36, 0.24, 0.48));

  vec3 col = mix(a, b, 0.5 + 0.5 * v);
  col = mix(col, c, 0.5 + 0.5 * sin(v * 3.14159 + t));
  fragColor = vec4(col, 1.0);
}
`

const VORONOI_DRIFT = `
void mainImage(out vec4 fragColor, in vec2 fragCoord) {
  vec2 p = fragCoord / iResolution.y * u_scale * 4.0;
  float t = iTime * 0.15 * u_speed;

  vec2 cell = floor(p);
  vec2 frac = fract(p);
  float nearest = 8.0;
  float second = 8.0;

  for (int y = -1; y <= 1; y++) {
    for (int x = -1; x <= 1; x++) {
      vec2 offset = vec2(float(x), float(y));
      float seed = hash21(cell + offset);
      vec2 site = offset + 0.5 + 0.42 * vec2(
        sin(t + seed * 6.2831),
        cos(t * 1.2 + seed * 6.2831));
      float d = length(site - frac);
      if (d < nearest) { second = nearest; nearest = d; }
      else if (d < second) { second = d; }
    }
  }

  float edge = smoothstep(0.0, 0.18, second - nearest);
  vec3 bg = themed(vec3(0.98, 0.97, 0.94), vec3(0.05, 0.06, 0.09));
  vec3 line = themed(vec3(0.78, 0.66, 0.48), vec3(0.34, 0.52, 0.76));
  fragColor = vec4(mix(line, bg, edge), 1.0);
}
`

const STARFIELD = `
void mainImage(out vec4 fragColor, in vec2 fragCoord) {
  vec2 p = (fragCoord - 0.5 * iResolution.xy) / iResolution.y;
  vec2 mouse = (iMouse.xy - 0.5 * iResolution.xy) / iResolution.y;

  vec3 bg = themed(vec3(0.98, 0.97, 0.95), vec3(0.02, 0.03, 0.06));
  vec3 col = bg;
  float t = iTime * 0.05 * u_speed;

  // Three parallax layers; the pointer shifts each by a different amount.
  for (float layer = 0.0; layer < 3.0; layer++) {
    float depth = 1.0 + layer * 0.9;
    vec2 q = p * depth + mouse * (0.05 * (3.0 - layer)) + vec2(t * (0.4 + layer * 0.2), 0.0);
    vec2 cell = floor(q * 12.0);
    vec2 frac = fract(q * 12.0) - 0.5;
    float seed = hash21(cell + layer * 41.0);
    if (seed > 0.86) {
      float twinkle = 0.6 + 0.4 * sin(iTime * (1.0 + seed * 3.0) + seed * 20.0);
      float star = smoothstep(0.34, 0.0, length(frac)) * twinkle * u_density;
      vec3 tint = themed(vec3(0.55, 0.48, 0.38), vec3(0.90, 0.94, 1.00));
      col += tint * star * (1.0 - layer * 0.25);
    }
  }
  fragColor = vec4(col, 1.0);
}
`

const GRADIENT_MESH = `
void mainImage(out vec4 fragColor, in vec2 fragCoord) {
  vec2 uv = fragCoord / iResolution.xy;
  float t = iTime * 0.08 * u_speed;

  vec2 a = vec2(0.3 + 0.2 * sin(t * 1.1), 0.35 + 0.2 * cos(t * 0.9));
  vec2 b = vec2(0.7 + 0.2 * cos(t * 0.8), 0.30 + 0.2 * sin(t * 1.3));
  vec2 c = vec2(0.5 + 0.3 * sin(t * 0.6), 0.80 + 0.1 * cos(t * 1.5));

  float wa = 1.0 / (distance(uv, a) + 0.25);
  float wb = 1.0 / (distance(uv, b) + 0.25);
  float wc = 1.0 / (distance(uv, c) + 0.25);
  float sum = wa + wb + wc;

  vec3 ca = themed(vec3(0.99, 0.93, 0.83), vec3(0.10, 0.14, 0.26));
  vec3 cb = themed(vec3(0.88, 0.93, 0.99), vec3(0.22, 0.13, 0.29));
  vec3 cc = themed(vec3(0.96, 0.88, 0.92), vec3(0.07, 0.19, 0.22));

  vec3 col = (ca * wa + cb * wb + cc * wc) / sum;
  col += (hash21(fragCoord) - 0.5) * u_grain * 0.06;
  fragColor = vec4(col, 1.0);
}
`

function speedParam(value = 1) {
  return { key: "speed", label: "Speed", min: 0, max: 3, step: 0.05, value }
}

/** The backgrounds that ship with the site. */
export const BUILTIN_BACKGROUNDS: BackgroundDef[] = [
  {
    id: "dappled-light",
    name: "Dappled Light",
    blurb: "CSS blinds, leaves and progressive blur. No GPU work",
    kind: "dom",
    themeReactive: true,
    mouseReactive: false,
    params: [],
  },
  {
    id: "fluid",
    name: "Fluid",
    blurb: "Navier-Stokes ink that follows the pointer. Drag to stir it.",
    kind: "fluid",
    themeReactive: true,
    mouseReactive: true,
    params: [
      { key: "dissipation", label: "Fade", min: 0.9, max: 1.0, step: 0.002, value: 0.985 },
      { key: "force", label: "Force", min: 100, max: 12000, step: 100, value: 5200 },
      { key: "radius", label: "Splat size", min: 0.05, max: 1.2, step: 0.01, value: 0.3 },
    ],
  },
  {
    id: "aurora",
    name: "Aurora",
    blurb: "Layered fbm ribbons drifting across a warm-to-cool wash.",
    kind: "glsl",
    themeReactive: true,
    mouseReactive: false,
    fragment: AURORA,
    params: [
      speedParam(),
      { key: "amount", label: "Sway", min: 0, max: 1.5, step: 0.02, value: 0.6 },
    ],
  },
  {
    id: "metaballs",
    name: "Metaballs",
    blurb: "Blobs orbiting a heavier one pinned to your cursor.",
    kind: "glsl",
    themeReactive: true,
    mouseReactive: true,
    fragment: METABALLS,
    params: [speedParam(), { key: "size", label: "Size", min: 0.3, max: 3, step: 0.05, value: 1 }],
  },
  {
    id: "gradient-mesh",
    name: "Gradient Mesh",
    blurb: "Three wandering colour poles, inverse-distance blended.",
    kind: "glsl",
    themeReactive: true,
    mouseReactive: false,
    fragment: GRADIENT_MESH,
    params: [
      speedParam(),
      { key: "grain", label: "Grain", min: 0, max: 1, step: 0.02, value: 0.35 },
    ],
  },
  {
    id: "plasma",
    name: "Plasma",
    blurb: "Summed sine fields.",
    kind: "glsl",
    themeReactive: true,
    mouseReactive: false,
    fragment: PLASMA,
    params: [
      speedParam(),
      { key: "scale", label: "Scale", min: 0.3, max: 3, step: 0.05, value: 1 },
    ],
  },
  {
    id: "voronoi",
    name: "Voronoi Drift",
    blurb: "Cell edges from drifting sites.",
    kind: "glsl",
    themeReactive: true,
    mouseReactive: false,
    fragment: VORONOI_DRIFT,
    params: [
      speedParam(),
      { key: "scale", label: "Scale", min: 0.3, max: 3, step: 0.05, value: 1 },
    ],
  },
  {
    id: "starfield",
    name: "Starfield",
    blurb: "Three parallax layers that lean away from the pointer.",
    kind: "glsl",
    themeReactive: true,
    mouseReactive: true,
    fragment: STARFIELD,
    params: [
      speedParam(),
      { key: "density", label: "Brightness", min: 0.2, max: 2, step: 0.05, value: 1 },
    ],
  },
]
