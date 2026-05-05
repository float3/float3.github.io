type AudioContextConstructor = new () => AudioContext

type OscilloscopeState = {
  analyser: AnalyserNode
  context: AudioContext
  data: Uint8Array<ArrayBuffer>
}

const AudioContextClass: AudioContextConstructor | undefined =
  window.AudioContext ??
  (window as Window & { webkitAudioContext?: AudioContextConstructor }).webkitAudioContext

const states = new WeakMap<HTMLAudioElement, OscilloscopeState>()

document.querySelectorAll<HTMLElement>("[data-oscilloscope]").forEach((figure) => {
  const audio = figure.querySelector("audio")
  if (!(audio instanceof HTMLAudioElement) || !AudioContextClass) {
    return
  }

  const canvas = document.createElement("canvas")
  canvas.className = "oscilloscope-canvas"
  canvas.setAttribute("aria-hidden", "true")
  audio.insertAdjacentElement("afterend", canvas)

  const context2d = canvas.getContext("2d")
  if (!context2d) {
    canvas.remove()
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

  audio.addEventListener("play", () => {
    try {
      const state = getState(audio)
      state.context.resume().catch(() => undefined)
      stopDrawing()
      draw()
    } catch {
      canvas.remove()
    }
  })

  audio.addEventListener("pause", stopDrawing)
  audio.addEventListener("ended", stopDrawing)
  window.addEventListener("resize", () => {
    resize()
    const state = states.get(audio)
    if (state && !audio.paused && !audio.ended) {
      drawPhaseScope(canvas, context2d, state.data)
    } else {
      drawIdle(canvas, context2d)
    }
  })
})

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
  const rect = canvas.getBoundingClientRect()
  const cssSize = Math.min(rect.width, rect.height || rect.width)
  const width = Math.max(220, Math.floor(cssSize * ratio))
  const height = width

  if (canvas.width !== width || canvas.height !== height) {
    canvas.width = width
    canvas.height = height
    context.setTransform(ratio, 0, 0, ratio, 0, 0)
  }
}

function drawIdle(canvas: HTMLCanvasElement, context: CanvasRenderingContext2D) {
  const width = canvas.clientWidth
  const height = canvas.clientHeight
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
  const width = canvas.clientWidth
  const height = canvas.clientHeight
  const size = Math.min(width, height)
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
