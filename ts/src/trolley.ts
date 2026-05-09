import { trolley_media_src, trolley_random_index } from "wasm"
import { renderMediaGallery, type GalleryItem } from "./media-gallery.js"

const NUM = 63

async function pickTrolleyProblem(): Promise<GalleryItem> {
  const trolleyPath = "/misc/trolley"
  const randomNumber = trolley_random_index(NUM)
  const mp4Src = trolley_media_src(trolleyPath, randomNumber, "mp4")
  const jpgSrc = trolley_media_src(trolleyPath, randomNumber, "jpg")
  const label = String(randomNumber).padStart(2, "0")

  const response = await fetch(jpgSrc)
  if (response.ok) {
    return {
      src: jpgSrc,
      title: `trolley problem ${label}`,
      meta: "image",
      kind: "image",
    }
  }

  return {
    src: mp4Src,
    title: `trolley problem ${label}`,
    meta: "video",
    kind: "video",
  }
}

document.addEventListener("DOMContentLoaded", async () => {
  const gallery = document.getElementById("trolley-gallery")
  const count = document.getElementById("trolley-gallery-count")
  const dialog = document.getElementById("trolley-lightbox") as HTMLDialogElement | null
  const problem = await pickTrolleyProblem()

  renderMediaGallery({
    items: [problem],
    gallery,
    count,
    dialog,
    countLabel: (total) => `${total} trolley problem${total === 1 ? "" : "s"}`,
    caption: (item) => item.title,
  })
})
