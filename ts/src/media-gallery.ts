export interface GalleryItem {
  src: string
  title: string
  meta?: string
  kind?: "image" | "video"
}

interface GalleryOptions {
  items: GalleryItem[]
  gallery: HTMLElement | null
  count?: HTMLElement | null
  dialog?: HTMLDialogElement | null
  countLabel: (count: number) => string
  caption: (item: GalleryItem) => string
}

function mediaKind(item: GalleryItem): "image" | "video" {
  if (item.kind) {
    return item.kind
  }

  return /\.(mp4|webm|mov)$/i.test(item.src) ? "video" : "image"
}

function mediaLabel(item: GalleryItem): string {
  const title = item.title.trim()
  const meta = item.meta?.trim() ?? ""
  return title || meta || "photo"
}

function createMediaElement(
  item: GalleryItem,
  preview: boolean,
): HTMLImageElement | HTMLVideoElement {
  if (mediaKind(item) === "video") {
    const video = document.createElement("video")
    video.src = item.src
    video.preload = "metadata"
    video.playsInline = true
    video.muted = preview
    video.controls = !preview
    video.setAttribute("aria-label", mediaLabel(item))
    return video
  }

  const image = document.createElement("img")
  image.src = item.src
  image.alt = mediaLabel(item)
  image.loading = preview ? "lazy" : "eager"
  return image
}

export function renderMediaGallery(options: GalleryOptions): void {
  const { items, gallery, count, dialog, countLabel, caption } = options
  const figure = dialog?.querySelector("figure") ?? null
  const dialogCaption = figure?.querySelector<HTMLElement>("figcaption") ?? null
  const closeButton = dialog?.querySelector<HTMLButtonElement>(".photo-lightbox-close") ?? null
  const prevButton = dialog?.querySelector<HTMLButtonElement>(".photo-lightbox-prev") ?? null
  const nextButton = dialog?.querySelector<HTMLButtonElement>(".photo-lightbox-next") ?? null
  let currentIndex = 0

  function showItem(index: number): void {
    if (!dialog || !figure || !dialogCaption || items.length === 0) {
      return
    }

    currentIndex = (index + items.length) % items.length
    const item = items[currentIndex]
    const media = createMediaElement(item, false)
    figure.querySelector("img, video")?.remove()
    figure.insertBefore(media, dialogCaption)
    dialogCaption.textContent = caption(item)
    if (!dialog.open) {
      dialog.showModal()
    }
  }

  function stepItem(delta: number): void {
    showItem(currentIndex + delta)
  }

  gallery?.replaceChildren()

  for (const [index, item] of items.entries()) {
    const card = document.createElement("button")
    card.className = "photo-card"
    card.type = "button"
    card.dataset.index = String(index)

    const media = createMediaElement(item, true)
    const text = document.createElement("span")
    const title = document.createElement("strong")
    const meta = document.createElement("small")

    title.textContent = item.title
    title.hidden = item.title.trim().length === 0
    meta.textContent = item.meta ?? ""
    text.append(title, meta)
    card.append(media, text)
    card.addEventListener("click", () => showItem(index))
    gallery?.append(card)
  }

  if (count) {
    count.textContent = countLabel(items.length)
  }
  if (prevButton) {
    prevButton.hidden = items.length < 2
  }
  if (nextButton) {
    nextButton.hidden = items.length < 2
  }

  closeButton?.addEventListener("click", () => dialog?.close())
  prevButton?.addEventListener("click", () => stepItem(-1))
  nextButton?.addEventListener("click", () => stepItem(1))
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
      stepItem(-1)
    } else if (event.key === "ArrowRight") {
      stepItem(1)
    } else if (event.key === "Escape") {
      dialog.close()
    }
  })
}
