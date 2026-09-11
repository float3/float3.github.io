import("./recursivetuning/index.js").catch((error: unknown) => {
  const status = document.getElementById("recursiveTuningStatus")
  if (status) {
    status.dataset.state = "error"
    status.textContent = `Could not load the recursive tuning explorer: ${error instanceof Error ? error.message : String(error)}`
  }
  window.setTimeout(() => {
    throw error
  }, 0)
})
