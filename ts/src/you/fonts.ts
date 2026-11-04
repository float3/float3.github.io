/**
 * Installed fonts, found by measuring text rather than asking: a family the
 * system has changes the width of a string drawn in it, and one it lacks falls
 * back to the generic face and does not. No permission is involved.
 *
 * Only fonts that say something about the person are probed. Each group pairs
 * the fonts with what having them suggests.
 */

interface FontGroup {
  hint: string
  fonts: string[]
  /** Systems that ship these fonts themselves, where they say nothing. */
  shippedWith?: string[]
}

const GROUPS: FontGroup[] = [
  {
    hint: "you write code",
    fonts: [
      "Fira Code",
      "JetBrains Mono",
      "Source Code Pro",
      "Hack",
      "Iosevka",
      "Monaspace Neon",
      "Victor Mono",
      "Inconsolata",
      "Roboto Mono",
      "IBM Plex Mono",
      "Berkeley Mono",
      "Geist Mono",
      "Commit Mono",
      "Maple Mono",
      "Hasklig",
    ],
  },
  {
    hint: "you write LaTeX",
    fonts: ["Latin Modern Roman", "CMU Serif", "TeX Gyre Termes", "Linux Libertine O"],
  },
  {
    hint: "you have Adobe apps",
    fonts: ["Myriad Pro", "Minion Pro", "Adobe Garamond Pro", "Adobe Caslon Pro", "Acumin Pro"],
  },
  {
    hint: "you design things",
    fonts: ["Inter", "SF Pro Display", "Proxima Nova", "Futura PT", "Gilroy"],
  },
  {
    hint: "you have Microsoft Office",
    fonts: ["Calibri", "Cambria", "Candara"],
    shippedWith: ["Windows"],
  },
  {
    hint: "you have Microsoft Office",
    fonts: ["Aptos"],
  },
]

const GENERIC = ["monospace", "sans-serif", "serif"]

/** Wide and narrow glyphs together, so a substitute rarely matches by luck. */
const PROBE = "mmmmmmmmmmlli1WQ@#"

const PROBE_SIZE = "72px"

export interface FontReading {
  hints: string[]
  found: string[]
}

/** Families the page itself loaded, which would otherwise read as installed. */
function pageFamilies(): Set<string> {
  if (!document.fonts) return new Set()

  return new Set(Array.from(document.fonts, (face) => face.family.replace(/["']/g, "")))
}

function isInstalled(context: CanvasRenderingContext2D, family: string): boolean {
  return GENERIC.some((generic) => {
    context.font = `${PROBE_SIZE} ${generic}`
    const fallback = context.measureText(PROBE).width

    context.font = `${PROBE_SIZE} "${family}", ${generic}`
    return context.measureText(PROBE).width !== fallback
  })
}

export function readFonts(platform: string): FontReading {
  const context = document.createElement("canvas").getContext("2d")
  if (!context) return { hints: [], found: [] }

  const loaded = pageFamilies()
  const hints = new Set<string>()
  const found: string[] = []

  for (const group of GROUPS) {
    if (group.shippedWith?.includes(platform)) continue

    const present = group.fonts.filter(
      (family) => !loaded.has(family) && isInstalled(context, family),
    )
    if (!present.length) continue

    hints.add(group.hint)
    found.push(...present)
  }

  return { hints: [...hints], found }
}
