/**
 * The AudioLink demo, decompressed in the browser.
 *
 * Unity built this player with gzip compression and no decompression fallback,
 * which means the three big files are gzip on disk and the loader expects the
 * server to say so. It wants `Content-Encoding: gzip`, so that the browser has
 * already unpacked the bytes by the time they reach it; given anything else it
 * refuses with "Unable to parse build.framework.js.gz".
 *
 * GitHub Pages will not say it. It serves `build.wasm.gz` as
 * `application/gzip` with no encoding header at all, and it has no
 * configuration to change that. The usual answer is to rebuild in Unity with
 * the fallback turned on, which needs Unity; the other is to store the files
 * unpacked, which doubles what a reader downloads -- 19 MB to 38 MB -- because
 * Pages does not compress `application/wasm` or `application/octet-stream` on
 * the way out either.
 *
 * So the page unpacks them itself. `DecompressionStream` is the same gzip the
 * browser would have used for the header, and it takes 85 ms over the 22 MB of
 * wasm; the loader is handed blob URLs and never learns the difference.
 */

const BUILD = "/audiolink/build"

/**
 * What is wrong with AudioLink's own WebGL bridge, and what to do about it.
 *
 * The audio reactivity in this build goes through four functions compiled into
 * the framework from AudioLink's jslib: `_SetupAnalyserSpace`, `_LinkAnalyser`,
 * `_FetchAnalyserLeft`, `_FetchAnalyserRight`. Linking is what attaches a pair
 * of WebAudio `AnalyserNode`s to the scene's audio, and everything after it
 * reads samples out of those with `getFloatTimeDomainData`. If the link is
 * never made, `_FetchAnalyser*` returns 1 for ever and the scene has nothing to
 * react to.
 *
 * `_LinkAnalyser` looks for the right audio source by walking Unity's instances
 * from the newest backwards:
 *
 *     for (var index = WAInstKeys.length - 1; index >= 0; i--)
 *
 * `i` is not a variable in that function or any around it. The framework runs
 * without strict mode, so this does not throw: it makes a global `i`,
 * decrements `undefined` to `NaN`, and leaves `index` exactly where it started.
 * The loop looks at the newest instance and only the newest, over and over. If
 * that one happens to be the source it wants it breaks out and everything
 * works; if it is not -- a clip rather than a playing channel, another sound
 * started later -- the loop never ends and never finds anything.
 *
 * The repair is `index--`, which is plainly what was meant. It is applied to
 * the framework as it passes through this page, because the page is holding
 * the text anyway; the real fix is one character in AudioLink's jslib and a
 * fresh export from Unity, and this can go the day that lands.
 */
const REPAIRS = [
  {
    why: "the search for the audio source never advances past the newest instance",
    from: "for(var index=WAInstKeys.length-1;index>=0;i--)",
    to: "for(var index=WAInstKeys.length-1;index>=0;index--)",
  },
]

const FRAMEWORK = "build.framework.js.gz"

/**
 * The framework, with the repairs above made.
 *
 * A pattern that is not there any more means the build has been re-exported --
 * possibly with the bug fixed, possibly moved somewhere else -- and that is
 * worth saying out loud rather than patching silently or refusing to run.
 */
function repair(framework: string): string {
  let mended = framework

  for (const { why, from, to } of REPAIRS) {
    if (!mended.includes(from)) {
      console.warn(`[audiolink] this build no longer matches the repair for: ${why}`)
      continue
    }

    mended = mended.replace(from, to)
  }

  return mended
}

/** The parts, with the type each one has to be handed back as. */
const PARTS = [
  { key: "dataUrl", file: "build.data.gz", type: "application/octet-stream" },
  { key: "frameworkUrl", file: FRAMEWORK, type: "application/javascript" },
  // Anything else and the framework's streaming compile falls back to a slower
  // path and says so in the console.
  { key: "codeUrl", file: "build.wasm.gz", type: "application/wasm" },
] as const

interface UnityConfig {
  dataUrl: string
  frameworkUrl: string
  codeUrl: string
  streamingAssetsUrl: string
  companyName: string
  productName: string
  productVersion: string
  /** Unity's own IndexedDB cache, keyed by URL. See below for why it is off. */
  cacheControl: (url: string) => string
}

declare global {
  interface Window {
    createUnityInstance?: (
      canvas: HTMLCanvasElement,
      config: UnityConfig,
      onProgress?: (progress: number) => void,
    ) => Promise<unknown>
  }
}

function say(message: string): void {
  const status = document.getElementById("unity-status")
  if (status !== null) status.textContent = message
}

/**
 * One part, unpacked into a blob URL.
 *
 * The progress is the compressed size, because that is the part that goes over
 * the network and the only one a `Content-Length` is given for.
 */
async function unpack(
  file: string,
  type: string,
  onBytes: (bytes: number) => void,
): Promise<string> {
  const response = await fetch(`${BUILD}/${file}`)
  if (!response.ok || response.body === null) {
    throw new Error(`${file}: ${response.status}`)
  }

  // Read by hand rather than through a counting transform: this is where the
  // waiting happens -- 12 MB of it for the data file -- and the reader should
  // be told how it is going.
  const reader = response.body.getReader()
  const packed: BlobPart[] = []
  for (;;) {
    const { done, value } = await reader.read()
    if (done) break

    // A chunk is typed as possibly backed by a SharedArrayBuffer, which a Blob
    // will not take. A fetch body's chunks are not.
    packed.push(value as BlobPart)
    onBytes(value.byteLength)
  }

  const unpacked = new Blob(packed).stream().pipeThrough(new DecompressionStream("gzip"))
  const bytes = await new Response(unpacked).arrayBuffer()

  if (file === FRAMEWORK) {
    return URL.createObjectURL(new Blob([repair(new TextDecoder().decode(bytes))], { type }))
  }

  return URL.createObjectURL(new Blob([bytes], { type }))
}

/** The loader defines `createUnityInstance` as a global and is not a module. */
function loader(): Promise<void> {
  return new Promise((resolve, reject) => {
    const script = document.createElement("script")
    script.src = `${BUILD}/build.loader.js`
    script.onload = () => {
      resolve()
    }
    script.onerror = () => {
      reject(new Error("build.loader.js did not load"))
    }
    document.head.append(script)
  })
}

async function start(canvas: HTMLCanvasElement): Promise<void> {
  if (typeof DecompressionStream === "undefined") {
    say("This browser cannot unpack the player: it has no DecompressionStream.")
    return
  }

  // What the three files weigh compressed, so that the reader sees a bar that
  // means something before Unity has anything of its own to report.
  const total = 18_700_000
  let read = 0

  let parts: (readonly [string, string])[] = []
  try {
    say("downloading the player…")

    parts = await Promise.all(
      PARTS.map(async (part) => {
        const url = await unpack(part.file, part.type, (bytes) => {
          read += bytes
          say(`downloading the player… ${Math.min(99, Math.round((read / total) * 100))}%`)
        })
        return [part.key, url] as const
      }),
    )

    say("starting…")
    await loader()

    const createUnityInstance = window.createUnityInstance
    if (createUnityInstance === undefined) throw new Error("build.loader.js defined nothing")

    await createUnityInstance(
      canvas,
      {
        ...(Object.fromEntries(parts) as Pick<UnityConfig, "dataUrl" | "frameworkUrl" | "codeUrl">),
        streamingAssetsUrl: `${BUILD}/StreamingAssets`,
        companyName: "com.llealloo.audiolink",
        productName: "AudioLinkWebProject",
        productVersion: "1.4.0",
        // Unity keeps downloads in IndexedDB against their URL. A blob URL is
        // new every time this page is opened, so that cache could only ever
        // grow: the same 38 MB again under a name that will never be asked for
        // twice. The browser's own cache still holds the gzip files, which is
        // the copy worth keeping.
        cacheControl: () => "no-store",
      },
      (progress) => {
        say(`starting… ${Math.round(progress * 100)}%`)
      },
    )

    say("")

    // The framework has been evaluated and the wasm compiled, so those two are
    // finished with -- 22 MB of the 38 handed back. The data file is Unity's
    // filesystem image, and whether it reads that again is its business, so it
    // is left alone until the page is.
    for (const [key, url] of parts) {
      if (key !== "dataUrl") URL.revokeObjectURL(url)
    }
  } catch (error: unknown) {
    for (const [, url] of parts) URL.revokeObjectURL(url)
    say(`The player did not start: ${error instanceof Error ? error.message : String(error)}`)
    throw error
  }
}

function initialise(): void {
  const canvas = document.querySelector<HTMLCanvasElement>("#unity-canvas")
  if (canvas === null || canvas.dataset.started === "true") return

  canvas.dataset.started = "true"
  void start(canvas).catch((error: unknown) => {
    console.error("[audiolink]", error)
  })
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", initialise, { once: true })
} else {
  initialise()
}

document.addEventListener("nav", initialise)
