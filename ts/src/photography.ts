import { photo_caption, photo_count_label, photo_manifest_entry_is_valid } from "wasm"
import { renderMediaGallery, type GalleryItem } from "./media-gallery.js"

const fallbackPhotos: GalleryItem[] = []

const gallery = document.getElementById("photo-gallery")
const count = document.getElementById("photo-gallery-count")
const dialog = document.getElementById("photo-lightbox") as HTMLDialogElement | null

let photos = fallbackPhotos

function isPhoto(value: unknown): value is GalleryItem {
  if (!value || typeof value !== "object") {
    return false
  }

  const candidate = value as Partial<GalleryItem>
  return (
    typeof candidate.src === "string" &&
    typeof candidate.title === "string" &&
    photo_manifest_entry_is_valid(candidate.src, candidate.title)
  )
}

async function loadPhotos(): Promise<void> {
  try {
    const response = await fetch("/photography/gallery.json", { cache: "no-cache" })
    if (response.ok) {
      const loaded: unknown = await response.json()
      if (Array.isArray(loaded) && loaded.length > 0 && loaded.every(isPhoto)) {
        photos = loaded
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
    caption: (photo) => photo_caption(photo.title, photo.meta ?? ""),
  })
}

void loadPhotos()
