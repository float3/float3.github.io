import { photo_caption, photo_count_label, photo_manifest_entry_is_valid } from "wasm"

interface Photo {
  src: string
  title: string
  meta?: string
  tags?: string[]
}

const fallbackPhotos: Photo[] = []

const gallery = document.getElementById("photo-gallery")
const count = document.getElementById("photo-gallery-count")
const dialog = document.getElementById("photo-lightbox") as HTMLDialogElement | null
const dialogImage = dialog?.querySelector<HTMLImageElement>("img") ?? null
const dialogCaption = dialog?.querySelector<HTMLElement>("figcaption") ?? null
const closeButton = dialog?.querySelector<HTMLButtonElement>(".photo-lightbox-close") ?? null
const prevButton = dialog?.querySelector<HTMLButtonElement>(".photo-lightbox-prev") ?? null
const nextButton = dialog?.querySelector<HTMLButtonElement>(".photo-lightbox-next") ?? null

let photos = fallbackPhotos
let currentIndex = 0

function isPhoto(value: unknown): value is Photo {
  if (!value || typeof value !== "object") {
    return false
  }

  const candidate = value as Partial<Photo>
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
  gallery?.replaceChildren()

  for (const [index, photo] of photos.entries()) {
    const card = document.createElement("button")
    card.className = "photo-card"
    card.type = "button"
    card.dataset.index = String(index)

    const image = document.createElement("img")
    image.src = photo.src
    image.alt = photo.title
    image.loading = "lazy"

    const text = document.createElement("span")
    const title = document.createElement("strong")
    title.textContent = photo.title
    const meta = document.createElement("small")
    meta.textContent = photo.meta ?? ""
    text.append(title, meta)

    card.append(image, text)
    card.addEventListener("click", () => showPhoto(index))
    gallery?.append(card)
  }

  if (count) {
    count.textContent = photo_count_label(photos.length)
  }
}

function showPhoto(index: number): void {
  if (!dialog || !dialogImage || !dialogCaption || photos.length === 0) {
    return
  }

  currentIndex = (index + photos.length) % photos.length
  const photo = photos[currentIndex]
  dialogImage.src = photo.src
  dialogImage.alt = photo.title
  dialogCaption.textContent = photo_caption(photo.title, photo.meta ?? "")
  dialog.showModal()
}

function stepPhoto(delta: number): void {
  showPhoto(currentIndex + delta)
}

closeButton?.addEventListener("click", () => dialog?.close())
prevButton?.addEventListener("click", () => stepPhoto(-1))
nextButton?.addEventListener("click", () => stepPhoto(1))
dialog?.addEventListener("click", (event) => {
  if (event.target === dialog) {
    dialog.close()
  }
})
window.addEventListener("keydown", (event) => {
  if (!dialog?.open) {
    return
  }
  if (event.key === "ArrowLeft") {
    stepPhoto(-1)
  } else if (event.key === "ArrowRight") {
    stepPhoto(1)
  } else if (event.key === "Escape") {
    dialog.close()
  }
})

void loadPhotos()
