/**
 * The AudioLink demo, decompressed in the browser.
 *
 * Unity built this player with gzip compression and no decompression fallback,
 * which means the three big files are gzip on disk and the loader expects the
 * server to say so. It wants `Content-Encoding: gzip`, so that the browser has
 * already unpacked the bytes by the time they reach it; given anything else it
 * refuses with "Unable to parse audiolink.framework.js.gz".
 *
 * GitHub Pages will not say it. It serves `audiolink.wasm.gz` as
 * `application/gzip` with no encoding header at all, and it has no
 * configuration to change that. The usual answer is to rebuild in Unity with
 * the fallback turned on, which needs Unity; the other is to store the files
 * unpacked, which is 14 MB becoming 32 MB for every reader, because Pages does
 * not compress `application/wasm` or `application/octet-stream` on the way out
 * either.
 *
 * So the page unpacks them itself. `DecompressionStream` is the same gzip the
 * browser would have used for the header, and it takes under a tenth of a
 * second over the 21 MB of wasm; the loader is handed blob URLs and never
 * learns the difference.
 */

const BUILD = "/audiolink/build"

/** The parts, with the type each one has to be handed back as. */
const PARTS = [
  { key: "dataUrl", file: "audiolink.data.gz", type: "application/octet-stream" },
  { key: "frameworkUrl", file: "audiolink.framework.js.gz", type: "application/javascript" },
  // Anything else and the framework's streaming compile falls back to a slower
  // path and says so in the console.
  { key: "codeUrl", file: "audiolink.wasm.gz", type: "application/wasm" },
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
  // waiting happens -- 7 MB of it for the data file -- and the reader should
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
  return URL.createObjectURL(new Blob([bytes], { type }))
}

/** The loader defines `createUnityInstance` as a global and is not a module. */
function loader(): Promise<void> {
  return new Promise((resolve, reject) => {
    const script = document.createElement("script")
    script.src = `${BUILD}/audiolink.loader.js`
    script.onload = () => {
      resolve()
    }
    script.onerror = () => {
      reject(new Error("audiolink.loader.js did not load"))
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
  const total = 13_700_000
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
    if (createUnityInstance === undefined) throw new Error("audiolink.loader.js defined nothing")

    await createUnityInstance(
      canvas,
      {
        ...(Object.fromEntries(parts) as Pick<UnityConfig, "dataUrl" | "frameworkUrl" | "codeUrl">),
        streamingAssetsUrl: `${BUILD}/StreamingAssets`,
        // What the export calls itself, which Unity only uses for the key its
        // IndexedDB cache would live under -- and that is turned off below.
        companyName: "DefaultCompany",
        productName: "AudioLink Controller",
        productVersion: "0.1",
        // Unity keeps downloads in IndexedDB against their URL. A blob URL is
        // new every time this page is opened, so that cache could only ever
        // grow: the same 32 MB again under a name that will never be asked for
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
    // finished with -- 22 MB of the 32 handed back. The data file is Unity's
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
