import("./tuningplayground/index.js").catch((error: unknown) => {
  const status = document.getElementById("tuningPlaygroundStatus")
  if (status) {
    status.dataset.state = "error"
    status.textContent = `Could not load tuning playground: ${formatStartupError(error)}`
  }
  window.setTimeout(() => {
    throw error
  }, 0)
})

function formatStartupError(error: unknown): string {
  if (error instanceof Error) {
    return error.message
  }
  return String(error)
}
