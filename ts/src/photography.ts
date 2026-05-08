import { photo_count_label } from "wasm"

interface Photo {
  src: string
  title: string
  description?: string
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

function nonEmptyString(value: unknown): string | undefined {
  if (typeof value !== "string") {
    return undefined
  }

  const trimmed = value.trim()
  return trimmed.length > 0 ? trimmed : undefined
}

function titleFromPath(path: string): string | undefined {
  const filename = path.split(/[\\/]/).pop()?.split(/[?#]/)[0] ?? path
  const stem = filename.replace(/\.[^.]*$/, "")
  const title = stem.replace(/[_-]+/g, " ").trim()
  return title.length > 0 ? title : undefined
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
  const title =
    nonEmptyString(candidate.title) ?? titleFromPath(meta ?? src) ?? description ?? "Untitled photo"
  const tags = photoTags(candidate.tags)

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
    text.append(title)

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
  dialogImage.alt = photo.description || photo.title
  renderCaption(photo)
  dialog.showModal()
}

function renderCaption(photo: Photo): void {
  if (!dialogCaption) {
    return
  }

  const title = document.createElement("strong")
  title.textContent = photo.title
  dialogCaption.replaceChildren(title)

  const description = photo.description?.trim()
  if (description) {
    const body = document.createElement("span")
    body.textContent = description
    dialogCaption.append(body)
  }
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
