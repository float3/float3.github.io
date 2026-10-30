import { renderMediaGallery, type GalleryItem } from "./shared/media-gallery.js"

interface Photo extends GalleryItem {
  tags?: string[]
}

const fallbackPhotos: Photo[] = []

const gallery = document.getElementById("photo-gallery")
const count = document.getElementById("photo-gallery-count")
const dialog = document.getElementById("photo-lightbox") as HTMLDialogElement | null

let photos = fallbackPhotos

function nonEmptyString(value: unknown): string | undefined {
  if (typeof value !== "string") {
    return undefined
  }

  const trimmed = value.trim()
  return trimmed.length > 0 ? trimmed : undefined
}

function photoTags(value: unknown): string[] | undefined {
  if (!Array.isArray(value)) {
    return undefined
  }

  const tags = value.flatMap((tag) => {
    const trimmed = nonEmptyString(tag)
    return trimmed ? [trimmed] : []
  })

  return tags.length > 0 ? tags : undefined
}

/**
 * One entry of `gallery.json`, as `site process-photos` writes it: the path,
 * and the original filename the caption shows.
 */
function toPhoto(value: unknown): Photo | null {
  if (!value || typeof value !== "object") {
    return null
  }

  const candidate = value as Record<string, unknown>
  const src = nonEmptyString(candidate.src)
  if (!src) {
    return null
  }

  const meta = nonEmptyString(candidate.meta) ?? ""
  const tags = photoTags(candidate.tags)

  return {
    src,
    kind: "image",
    title: meta,
    meta,
    ...(tags ? { tags } : {}),
  }
}

async function loadPhotos(): Promise<void> {
  try {
    const response = await fetch("/photography/gallery.json", { cache: "no-cache" })
    if (response.ok) {
      const loaded: unknown = await response.json()
      if (Array.isArray(loaded)) {
        const loadedPhotos = loaded.map(toPhoto).filter((photo): photo is Photo => photo !== null)

        if (loadedPhotos.length > 0) {
          photos = loadedPhotos
        }
      }
    }
  } catch {
    photos = fallbackPhotos
  }

  renderGallery()
}

function renderGallery(): void {
  renderMediaGallery({
    items: photos,
    gallery,
    count,
    dialog,
    countLabel: (total) => `${total} ${total === 1 ? "photo" : "photos"}`,
    caption: (photo) => photo.title,
  })
}

void loadPhotos()
