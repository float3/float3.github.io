/**
 * A gallery over any directory of media under `content/misc`.
 *
 * Nothing here knows what the pictures are of. The page names a collection and
 * how to word its captions; `site indices` writes the manifest of filenames;
 * this reads one and renders the other. The trolley problems and "guess we
 * doing" are two directories and two pages, sharing all of this.
 *
 * The manifest is why: the old version of this probed sixty-four numbered paths
 * with a HEAD request each, guessing `.jpg` and falling back to `.mp4`, because
 * it had no way to know what was in the directory. One fetch replaces all of
 * that, and adding a collection stops meaning editing a hardcoded count.
 */

import { gallery_media_kind, gallery_media_label, gallery_media_src } from "wasm-gallery"
import { renderMediaGallery, type GalleryItem } from "./media-gallery.js"
import { renderSubmitButton } from "./gallery/submit.js"

/** Written next to the media by `site indices`. */
const MANIFEST = "index.json"

interface GalleryConfig {
  /** Directory under `content/misc`, e.g. `trolley`. */
  collection: string
  /** What one item is called, for the count line: "trolley problem", "entry". */
  noun: string
  /** Plural of `noun`, for when adding an "s" is wrong. */
  plural: string
  /**
   * What a caption calls an item, before its number.
   *
   * Separate from `noun` because the two want different words as often as not:
   * a collection can be "12 entries" and still have every one of them captioned
   * "guess we doing 04".
   */
  caption: string
  /**
   * `owner/repo` the submit button opens an issue against, or nothing.
   *
   * Its presence is what puts the button on the page. Whether the submission is
   * then accepted is the repository's own answer, given by `Site::SUBMITTABLE`
   * in `tools/site/src/content.rs`; a test there refuses a page that offers a
   * button the workflow would only refuse.
   */
  submitRepo?: string
}

function readConfig(gallery: HTMLElement): GalleryConfig | undefined {
  const collection = gallery.dataset.collection?.trim()
  if (!collection) return undefined

  const noun = gallery.dataset.noun?.trim() || "item"
  return {
    collection,
    noun,
    plural: gallery.dataset.plural?.trim() || `${noun}s`,
    caption: gallery.dataset.caption?.trim() || noun,
    submitRepo: gallery.dataset.submitRepo?.trim() || undefined,
  }
}

const basePath = (collection: string) => `/misc/${collection}`

/**
 * The filenames in the collection, or an empty list.
 *
 * A missing or malformed manifest is not worth an error on the page: the
 * gallery renders as empty, which is what a reader would conclude anyway, and
 * the console says why for whoever is building the thing.
 */
async function readManifest(collection: string): Promise<string[]> {
  const url = `${basePath(collection)}/${MANIFEST}`
  try {
    const response = await fetch(url, { cache: "no-cache" })
    if (!response.ok) throw new Error(`${response.status}`)
    const parsed: unknown = await response.json()
    if (!Array.isArray(parsed)) throw new Error("not an array")
    return parsed.filter((name): name is string => typeof name === "string")
  } catch (error) {
    console.error(`gallery: could not read ${url}`, error)
    return []
  }
}

function toItem(config: GalleryConfig, name: string): GalleryItem {
  const kind = gallery_media_kind(name) === "video" ? "video" : "image"
  return {
    src: gallery_media_src(basePath(config.collection), name),
    title: `${config.caption} ${gallery_media_label(name)}`,
    meta: kind,
    kind,
  }
}

async function initialise(gallery: HTMLElement): Promise<void> {
  if (gallery.dataset.galleryInitialised === "true") return
  const config = readConfig(gallery)
  if (config === undefined) return

  gallery.dataset.galleryInitialised = "true"

  // Scoped to the section rather than fetched by id, so a page could hold two
  // galleries without their counts and lightboxes colliding.
  const section = gallery.closest(".photo-gallery-section") ?? document
  const count = section.querySelector<HTMLElement>(".gallery-count")
  const dialog = gallery
    .closest(".photo-page")
    ?.querySelector<HTMLDialogElement>("dialog.photo-lightbox")

  // Before the manifest is fetched, and not conditional on it: a gallery with
  // nothing in it yet is the one most worth being able to add to.
  if (config.submitRepo !== undefined) {
    renderSubmitButton(section, {
      repo: config.submitRepo,
      collection: config.collection,
      noun: config.noun,
      label: config.caption,
    })
  }

  const names = await readManifest(config.collection)

  // A collection listed before its first file lands is the normal way one
  // starts. Saying so beats a heading with nothing under it, which reads as a
  // page that failed rather than one that is waiting.
  if (names.length === 0) {
    const empty = document.createElement("p")
    empty.className = "gallery-empty"
    empty.textContent = "nothing here yet"
    gallery.replaceChildren(empty)
    if (count !== null) count.textContent = ""
    return
  }

  renderMediaGallery({
    items: names.map((name) => toItem(config, name)),
    gallery,
    count,
    dialog: dialog ?? null,
    countLabel: (total) => `${total} ${total === 1 ? config.noun : config.plural}`,
    caption: (item) => item.title,
  })
}

function initialiseAll(): void {
  for (const gallery of document.querySelectorAll<HTMLElement>("[data-collection]")) {
    void initialise(gallery)
  }
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", initialiseAll, { once: true })
} else {
  initialiseAll()
}

document.addEventListener("nav", initialiseAll)
