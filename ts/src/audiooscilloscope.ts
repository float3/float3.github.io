type AudioContextConstructor = new () => AudioContext

type OscilloscopeState = {
  analyser: AnalyserNode
  context: AudioContext
  data: Uint8Array<ArrayBuffer>
}

type OscilloscopeWindow = Window & {
  __audioOscilloscopeStates?: WeakMap<HTMLAudioElement, OscilloscopeState>
  addCleanup?: (fn: () => void) => void
}

const AudioContextClass: AudioContextConstructor | undefined =
  window.AudioContext ??
  (window as Window & { webkitAudioContext?: AudioContextConstructor }).webkitAudioContext

const browserWindow = window as OscilloscopeWindow
const states = (browserWindow.__audioOscilloscopeStates ??= new WeakMap<
  HTMLAudioElement,
  OscilloscopeState
>())

document.querySelectorAll<HTMLElement>("[data-oscilloscope]").forEach((figure) => {
  const audio = figure.querySelector("audio")
  if (!(audio instanceof HTMLAudioElement)) {
    return
  }

  if (figure.dataset.oscilloscopeEnhanced === "true") {
    return
  }

  if (!AudioContextClass) {
    audio.controls = true
    return
  }

  removeExistingStages(figure)
  figure.dataset.oscilloscopeEnhanced = "true"

  audio.controls = false
  audio.preload = audio.preload || "metadata"
  audio.classList.add("oscilloscope-audio")

  const stage = document.createElement("div")
  stage.className = "audio-oscilloscope-stage"

  const canvas = document.createElement("canvas")
  canvas.className = "oscilloscope-canvas"
  canvas.setAttribute("aria-hidden", "true")

  const playButton = document.createElement("button")
  playButton.className = "audio-oscilloscope-button"
  playButton.type = "button"
  playButton.dataset.playing = "false"
  playButton.setAttribute("aria-label", "Play audio")

  stage.append(canvas, playButton)
  audio.insertAdjacentElement("beforebegin", stage)

  const context2d = canvas.getContext("2d")
  if (!context2d) {
    stage.remove()
    audio.controls = true
    audio.classList.remove("oscilloscope-audio")
    return
  }

  let animationFrame: number | null = null

  const resize = () => resizeCanvas(canvas, context2d)
  const stopDrawing = () => {
    if (animationFrame !== null) {
      cancelAnimationFrame(animationFrame)
      animationFrame = null
    }
  }
  const updatePlaybackButton = () => {
    const isPlaying = !audio.paused && !audio.ended
    playButton.dataset.playing = String(isPlaying)
    playButton.setAttribute("aria-label", isPlaying ? "Pause audio" : "Play audio")
  }
  const draw = () => {
    const state = states.get(audio)
    if (!state) {
      return
    }

    resize()
    state.analyser.getByteTimeDomainData(state.data)
    drawPhaseScope(canvas, context2d, state.data)

    if (!audio.paused && !audio.ended) {
      animationFrame = requestAnimationFrame(draw)
    }
  }

  resize()
  drawIdle(canvas, context2d)
  updatePlaybackButton()

  playButton.addEventListener("click", () => {
    if (audio.paused || audio.ended) {
      if (audio.ended) {
        audio.currentTime = 0
      }
      audio.play().catch(() => undefined)
    } else {
      audio.pause()
    }
  })

  audio.addEventListener("play", () => {
    try {
      const state = getState(audio)
      state.context.resume().catch(() => undefined)
      updatePlaybackButton()
      stopDrawing()
      draw()
    } catch {
      stage.remove()
      audio.controls = true
      audio.classList.remove("oscilloscope-audio")
    }
  })

  audio.addEventListener("pause", () => {
    updatePlaybackButton()
    stopDrawing()
  })
  audio.addEventListener("ended", () => {
    updatePlaybackButton()
    stopDrawing()
    drawIdle(canvas, context2d)
  })

  const handleResize = () => {
    resize()
    const state = states.get(audio)
    if (state && !audio.paused && !audio.ended) {
      drawPhaseScope(canvas, context2d, state.data)
    } else {
      drawIdle(canvas, context2d)
    }
  }
  window.addEventListener("resize", handleResize)
  browserWindow.addCleanup?.(() => {
    stopDrawing()
    window.removeEventListener("resize", handleResize)
  })
})

function removeExistingStages(figure: HTMLElement) {
  Array.from(figure.children).forEach((child) => {
    if (child.classList.contains("audio-oscilloscope-stage")) {
      child.remove()
    }
  })
}

function getState(audio: HTMLAudioElement): OscilloscopeState {
  const existing = states.get(audio)
  if (existing) {
    return existing
  }

  if (!AudioContextClass) {
    throw new Error("AudioContext is unavailable")
  }

  const context = new AudioContextClass()
  const source = context.createMediaElementSource(audio)
  const analyser = context.createAnalyser()
  analyser.fftSize = 2048
  source.connect(analyser)
  analyser.connect(context.destination)

  const state = {
    analyser,
    context,
    data: new Uint8Array(analyser.fftSize),
  }
  states.set(audio, state)
  return state
}

function resizeCanvas(canvas: HTMLCanvasElement, context: CanvasRenderingContext2D) {
  const ratio = window.devicePixelRatio || 1
  const { size: cssSize } = canvasDisplaySize(canvas)
  const width = Math.max(1, Math.floor(cssSize * ratio))
  const height = width

  if (canvas.width !== width || canvas.height !== height) {
    canvas.width = width
    canvas.height = height
  }

  context.setTransform(ratio, 0, 0, ratio, 0, 0)
}

function drawIdle(canvas: HTMLCanvasElement, context: CanvasRenderingContext2D) {
  const { width, height } = canvasDisplaySize(canvas)
  drawBackground(context, width, height)
  context.strokeStyle = "rgba(160, 210, 190, 0.38)"
  context.lineWidth = 1
  context.beginPath()
  context.arc(width / 2, height / 2, width * 0.18, 0, Math.PI * 2)
  context.stroke()
}

function drawPhaseScope(
  canvas: HTMLCanvasElement,
  context: CanvasRenderingContext2D,
  data: Uint8Array<ArrayBuffer>,
) {
  const { width, height, size } = canvasDisplaySize(canvas)
  const centerX = width / 2
  const centerY = height / 2
  const radius = size * 0.42
  const delay = Math.floor(data.length / 5)

  drawBackground(context, width, height)

  context.strokeStyle = "rgba(170, 230, 205, 0.78)"
  context.lineWidth = 1.4
  context.beginPath()

  data.forEach((sample, index) => {
    const delayed = data[(index + delay) % data.length]
    const x = centerX + normalizedSample(sample) * radius
    const y = centerY + normalizedSample(delayed) * radius
    if (index === 0) {
      context.moveTo(x, y)
    } else {
      context.lineTo(x, y)
    }
  })

  context.stroke()
}

function canvasDisplaySize(canvas: HTMLCanvasElement) {
  const rect = canvas.getBoundingClientRect()
  const width = rect.width || canvas.clientWidth || 1
  const height = rect.height || rect.width || canvas.clientHeight || width
  const size = Math.max(1, Math.min(width, height))
  return { height, size, width }
}

function drawBackground(context: CanvasRenderingContext2D, width: number, height: number) {
  context.clearRect(0, 0, width, height)
  context.fillStyle = "#020403"
  context.fillRect(0, 0, width, height)
  context.strokeStyle = "rgba(170, 230, 205, 0.16)"
  context.lineWidth = 1

  const inset = width * 0.08
  context.strokeRect(inset, inset, width - inset * 2, height - inset * 2)

  context.beginPath()
  context.moveTo(width / 2, inset)
  context.lineTo(width / 2, height - inset)
  context.moveTo(inset, height / 2)
  context.lineTo(width - inset, height / 2)
  context.stroke()
}

function normalizedSample(sample: number) {
  return (sample - 128) / 128
}
