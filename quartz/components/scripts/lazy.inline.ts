import lozad from "lozad"

document.addEventListener("nav", (evt) => {
  const url = evt.detail.url
  if (url === "notes/craft") {
    const observer = lozad(".lazy")
    observer.observe()
  }
})
