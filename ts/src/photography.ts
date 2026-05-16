import { photo_caption, photo_count_label, photo_manifest_entry_is_valid } from "wasm-photography"
import { renderMediaGallery, type GalleryItem } from "./media-gallery.js"

interface Photo extends GalleryItem {
  description?: string
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

function toPhoto(value: unknown): Photo | null {
  if (!value || typeof value !== "object") {
    return null
  }

  const candidate = value as Record<string, unknown>
  const src = nonEmptyString(candidate.src)
  if (!src) {
    return null
  }

  const description = nonEmptyString(candidate.description)
  const meta = nonEmptyString(candidate.meta)
  const title = nonEmptyString(candidate.title) ?? ""
  const tags = photoTags(candidate.tags)

  if (!photo_manifest_entry_is_valid(src, title)) {
    return null
  }

  return {
    src,
    title,
    ...(description ? { description } : {}),
    ...(meta ? { meta } : {}),
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
    countLabel: photo_count_label,
    caption: (photo) => {
      const description = "description" in photo ? (photo as Photo).description : undefined
      return photo_caption(photo.title, description ?? photo.meta ?? "")
    },
  })
}

void loadPhotos()
