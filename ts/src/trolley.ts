import { trolley_media_src, trolley_random_index } from "wasm"

const NUM = 63
document.addEventListener("DOMContentLoaded", async () => {
  const trolleyPath = "/misc/trolley"
  const randomNumber = trolley_random_index(NUM)
  const mp4Src = trolley_media_src(trolleyPath, randomNumber, "mp4")
  const jpgSrc = trolley_media_src(trolleyPath, randomNumber, "jpg")

  const response = await fetch(jpgSrc)
  const selectedSrc = response.ok ? jpgSrc : mp4Src
  window.location.href = selectedSrc
})
