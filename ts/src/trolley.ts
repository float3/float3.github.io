const NUM = 63
import { trolley_media_src } from "wasm"
import { renderMediaGallery, type GalleryItem } from "./media-gallery.js"

const MAX_INDEX = 63
const trolleyPath = "/misc/trolley"

async function trolleyProblem(index: number): Promise<GalleryItem> {
  const jpgSrc = trolley_media_src(trolleyPath, index, "jpg")
  const mp4Src = trolley_media_src(trolleyPath, index, "mp4")
  const label = String(index).padStart(2, "0")

  try {
    const response = await fetch(jpgSrc, { method: "HEAD", cache: "no-cache" })
    if (response.ok) {
      return {
        src: jpgSrc,
        title: `trolley problem ${label}`,
        meta: "image",
        kind: "image",
      }
    }
  } catch {}

  return {
    src: mp4Src,
    title: `trolley problem ${label}`,
    meta: "video",
    kind: "video",
  }
}

async function trolleyProblems(): Promise<GalleryItem[]> {
  return Promise.all(Array.from({ length: MAX_INDEX + 1 }, (_, index) => trolleyProblem(index)))
}

async function initializeTrolleyGallery(): Promise<void> {
  const gallery = document.getElementById("trolley-gallery")
  if (!gallery || gallery.dataset.trolleyInitialized === "true") {
    return
  }

  gallery.dataset.trolleyInitialized = "true"
  const count = document.getElementById("trolley-gallery-count")
  const dialog = document.getElementById("trolley-lightbox") as HTMLDialogElement | null
  const problems = await trolleyProblems()

  renderMediaGallery({
    items: problems,
    gallery,
    count,
    dialog,
    countLabel: (total) => `${total} trolley problem${total === 1 ? "" : "s"}`,
    caption: (item) => item.title,
  })
}

function scheduleTrolleyGallery(): void {
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", () => void initializeTrolleyGallery(), {
      once: true,
    })
  } else {
    void initializeTrolleyGallery()
  }
}

scheduleTrolleyGallery()
document.addEventListener("nav", () => void initializeTrolleyGallery())
