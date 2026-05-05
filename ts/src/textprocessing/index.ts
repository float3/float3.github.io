import * as wasm from "wasm"

type Transform = (text: string) => string

interface TransformDefinition {
  id: string
  group: string
  leftLabel: string
  rightLabel: string
  leftExample: string
  rightExample?: string
  leftToRight: Transform
  rightToLeft?: Transform
  keywords?: string[]
}

interface RenderedTransform {
  definition: TransformDefinition
  card: HTMLElement
}

type GraphNodeKind = "source" | "transform" | "output"
type GraphDirection = "forward" | "reverse"

interface GraphTransformOption {
  id: string
  definition: TransformDefinition
  direction: GraphDirection
  label: string
  inputLabel: string
  outputLabel: string
  transform: Transform
}

interface GraphNode {
  id: string
  kind: GraphNodeKind
  x: number
  y: number
  optionId?: string
  element?: HTMLElement
  input?: HTMLTextAreaElement
  output?: HTMLTextAreaElement
  error?: HTMLElement
  select?: HTMLSelectElement
}

interface GraphConnection {
  from: string
  to: string
}

interface StoredGraphNode {
  id: string
  kind: GraphNodeKind
  x: number
  y: number
  optionId?: string
  value?: string
}

interface StoredGraph {
  version: 1
  nextNodeId: number
  zoom?: number
  nodes: StoredGraphNode[]
  connections: GraphConnection[]
}

const graphStorageKey = "textprocessing-node-graph-v1"

wasm.main()

const wasmTransform =
  (index: number, leftToRight: boolean): Transform =>
  (text) =>
    wasm.transform_text(index, leftToRight, text)

const transforms: TransformDefinition[] = [
  {
    id: "pinyin-tones",
    group: "Chinese",
    leftLabel: "Pinyin tone marks",
    rightLabel: "Pinyin tone numbers",
    leftExample: "wèi shén me",
    leftToRight: wasmTransform(20, true),
    rightToLeft: wasmTransform(20, false),
    keywords: ["mandarin", "romanization"],
  },
  {
    id: "pinyin-zhuyin",
    group: "Chinese",
    leftLabel: "Pinyin",
    rightLabel: "Zhuyin",
    leftExample: "wèi shén me",
    leftToRight: wasmTransform(0, true),
    rightToLeft: wasmTransform(0, false),
    keywords: ["bopomofo", "mandarin"],
  },
  {
    id: "han-trad-simp",
    group: "Chinese",
    leftLabel: "Traditional",
    rightLabel: "Simplified",
    leftExample: "為什麼",
    leftToRight: wasmTransform(1, true),
    rightToLeft: wasmTransform(1, false),
    keywords: ["hanzi"],
  },
  {
    id: "hanzi-pinyin",
    group: "Chinese",
    leftLabel: "Hanzi",
    rightLabel: "Pinyin",
    leftExample: "漢字",
    leftToRight: wasmTransform(4, true),
    keywords: ["mandarin", "romanization"],
  },
  {
    id: "hanzi-zhuyin",
    group: "Chinese",
    leftLabel: "Hanzi",
    rightLabel: "Zhuyin",
    leftExample: "漢字",
    leftToRight: wasmTransform(8, true),
    keywords: ["bopomofo"],
  },
  {
    id: "hanzi-pinyin-readings",
    group: "Chinese",
    leftLabel: "Hanzi",
    rightLabel: "Pinyin readings",
    leftExample: "行",
    leftToRight: wasmTransform(5, true),
    keywords: ["polyphone", "readings"],
  },
  {
    id: "hanzi-zhuyin-readings",
    group: "Chinese",
    leftLabel: "Hanzi",
    rightLabel: "Zhuyin readings",
    leftExample: "行",
    leftToRight: wasmTransform(9, true),
    keywords: ["polyphone", "readings", "bopomofo"],
  },
  {
    id: "hanzi-tokenize",
    group: "Chinese",
    leftLabel: "Chinese text",
    rightLabel: "Tokens",
    leftExample: "我愛自然語言處理",
    leftToRight: wasmTransform(22, true),
    keywords: ["segmentation"],
  },
  {
    id: "kana",
    group: "Japanese",
    leftLabel: "Hiragana",
    rightLabel: "Katakana",
    leftExample: "ひらがな",
    leftToRight: wasmTransform(2, true),
    rightToLeft: wasmTransform(2, false),
    keywords: ["kana"],
  },
  {
    id: "kana-romaji",
    group: "Japanese",
    leftLabel: "Kana",
    rightLabel: "Romaji",
    leftExample: "ひらがな カタカナ きょう",
    leftToRight: wasmTransform(33, true),
    keywords: ["hepburn", "romanization", "hiragana", "katakana"],
  },
  {
    id: "hanja-hangeul",
    group: "Korean",
    leftLabel: "Hanja",
    rightLabel: "Hangeul",
    leftExample: "在元韓國",
    leftToRight: wasmTransform(3, true),
    rightToLeft: wasmTransform(3, false),
    keywords: ["hangul"],
  },
  {
    id: "hangeul-rr",
    group: "Korean",
    leftLabel: "Hangeul",
    rightLabel: "Revised Romanization",
    leftExample: "재원한국",
    leftToRight: wasmTransform(19, true),
    rightToLeft: wasmTransform(19, false),
    keywords: ["hangul", "romanization"],
  },
  {
    id: "hangeul-mr",
    group: "Korean",
    leftLabel: "Hangeul",
    rightLabel: "McCune-Reischauer",
    leftExample: "재원한국",
    leftToRight: wasmTransform(23, true),
    rightToLeft: wasmTransform(23, false),
    keywords: ["hangul", "romanization"],
  },
  {
    id: "korean-rr-mr",
    group: "Korean",
    leftLabel: "Revised Romanization",
    rightLabel: "McCune-Reischauer",
    leftExample: "jaewonhanguk",
    leftToRight: wasmTransform(24, true),
    rightToLeft: wasmTransform(24, false),
    keywords: ["hangul", "romanization"],
  },
  {
    id: "roman-numerals",
    group: "Numbers",
    leftLabel: "Arabic",
    rightLabel: "Roman",
    leftExample: "3339",
    leftToRight: wasmTransform(7, true),
    rightToLeft: wasmTransform(7, false),
  },
  {
    id: "japanese-number",
    group: "Numbers",
    leftLabel: "Arabic",
    rightLabel: "Japanese",
    leftExample: "1234567890",
    leftToRight: wasmTransform(18, true),
    keywords: ["kanji"],
  },
  {
    id: "chinese-number-lower",
    group: "Numbers",
    leftLabel: "Arabic",
    rightLabel: "Chinese lowercase",
    leftExample: "1234567890",
    leftToRight: wasmTransform(15, true),
    keywords: ["hanzi"],
  },
  {
    id: "chinese-number-financial",
    group: "Numbers",
    leftLabel: "Arabic",
    rightLabel: "Chinese financial",
    leftExample: "1234567890",
    leftToRight: wasmTransform(11, true),
    keywords: ["hanzi", "uppercase"],
  },
  {
    id: "utf8-hex",
    group: "Encoding",
    leftLabel: "Text",
    rightLabel: "UTF-8 hex bytes",
    leftExample: "hello 世界",
    leftToRight: wasmTransform(25, true),
    rightToLeft: wasmTransform(25, false),
    keywords: ["bytes"],
  },
  {
    id: "utf8-binary",
    group: "Encoding",
    leftLabel: "Text",
    rightLabel: "UTF-8 binary bytes",
    leftExample: "Hi",
    leftToRight: wasmTransform(26, true),
    rightToLeft: wasmTransform(26, false),
    keywords: ["bytes"],
  },
  {
    id: "base64",
    group: "Encoding",
    leftLabel: "Text",
    rightLabel: "Base64",
    leftExample: "hello 世界",
    leftToRight: wasmTransform(27, true),
    rightToLeft: wasmTransform(27, false),
  },
  {
    id: "url",
    group: "Encoding",
    leftLabel: "Text",
    rightLabel: "URL encoded",
    leftExample: "hello world? a=1&b=世界",
    leftToRight: encodeURIComponent,
    rightToLeft: decodeURIComponent,
    keywords: ["percent"],
  },
  {
    id: "html-entities",
    group: "Encoding",
    leftLabel: "Text",
    rightLabel: "HTML entities",
    leftExample: '<span title="hill">& text</span>',
    leftToRight: wasmTransform(28, true),
    rightToLeft: wasmTransform(28, false),
  },
  {
    id: "unicode-codepoints",
    group: "Encoding",
    leftLabel: "Text",
    rightLabel: "Unicode code points",
    leftExample: "漢字🙂",
    leftToRight: wasmTransform(29, true),
    rightToLeft: wasmTransform(29, false),
    keywords: ["unicode"],
  },
  {
    id: "big-endian",
    group: "Binary",
    leftLabel: "Unsigned integer",
    rightLabel: "Big endian bytes",
    leftExample: "305419896",
    leftToRight: wasmTransform(30, true),
    rightToLeft: wasmTransform(30, false),
    keywords: ["network order", "hex"],
  },
  {
    id: "little-endian",
    group: "Binary",
    leftLabel: "Unsigned integer",
    rightLabel: "Little endian bytes",
    leftExample: "305419896",
    leftToRight: wasmTransform(31, true),
    rightToLeft: wasmTransform(31, false),
    keywords: ["small endian", "hex"],
  },
  {
    id: "byte-order",
    group: "Binary",
    leftLabel: "Big endian bytes",
    rightLabel: "Little endian bytes",
    leftExample: "12 34 56 78",
    leftToRight: wasmTransform(32, true),
    rightToLeft: wasmTransform(32, false),
    keywords: ["endianness", "hex"],
  },
  {
    id: "cyrillic",
    group: "Scripts",
    leftLabel: "Cyrillic",
    rightLabel: "Latin",
    leftExample: "Привет, мир",
    leftToRight: wasmTransform(34, true),
    keywords: ["romanization", "russian"],
  },
  {
    id: "greek",
    group: "Scripts",
    leftLabel: "Greek",
    rightLabel: "Latin",
    leftExample: "Καλημέρα κόσμε",
    leftToRight: wasmTransform(35, true),
    keywords: ["romanization"],
  },
]

function start() {
  const app = document.getElementById("textprocessing-app")
  if (!app) {
    return
  }

  app.textContent = ""

  const graphLauncher = document.createElement("div")
  graphLauncher.className = "graph-launcher"

  const graphLaunchButton = document.createElement("button")
  graphLaunchButton.className = "graph-launch-button"
  graphLaunchButton.type = "button"
  graphLaunchButton.textContent = "Launch node graph"
  graphLaunchButton.setAttribute("aria-expanded", "false")
  graphLauncher.append(graphLaunchButton)
  app.append(graphLauncher)

  let graphPanel: HTMLElement | undefined
  graphLaunchButton.addEventListener("click", () => {
    if (!graphPanel) {
      graphPanel = renderGraphWorkspace()
      graphLauncher.after(graphPanel)
    } else {
      graphPanel.hidden = !graphPanel.hidden
    }

    const graphIsOpen = !graphPanel.hidden
    graphLaunchButton.textContent = graphIsOpen ? "Hide node graph" : "Launch node graph"
    graphLaunchButton.setAttribute("aria-expanded", String(graphIsOpen))
    if (graphIsOpen) {
      graphPanel.scrollIntoView({ block: "nearest" })
    }
  })

  const toolbar = document.createElement("div")
  toolbar.className = "textprocessing-toolbar"

  const filterInput = document.createElement("input")
  filterInput.className = "textprocessing-filter"
  filterInput.type = "search"
  filterInput.placeholder = "filter transforms"
  filterInput.autocomplete = "off"
  toolbar.append(filterInput)
  app.append(toolbar)

  const empty = document.createElement("div")
  empty.className = "textprocessing-empty"
  empty.textContent = "No matching transforms."
  empty.hidden = true

  const groups = new Map<string, { element: HTMLElement; cards: RenderedTransform[] }>()
  for (const definition of transforms) {
    let group = groups.get(definition.group)
    if (!group) {
      const section = document.createElement("section")
      section.className = "transform-group"

      const heading = document.createElement("h2")
      heading.textContent = definition.group
      section.append(heading)

      const grid = document.createElement("div")
      grid.className = "transform-grid"
      section.append(grid)

      app.append(section)
      group = { element: section, cards: [] }
      groups.set(definition.group, group)
    }

    const grid = group.element.querySelector(".transform-grid")
    if (!grid) {
      continue
    }

    const rendered = renderTransform(definition)
    group.cards.push(rendered)
    grid.append(rendered.card)
  }

  app.append(empty)

  filterInput.addEventListener("input", () => {
    const query = filterInput.value.trim().toLowerCase()
    let visibleCount = 0

    for (const group of groups.values()) {
      let groupVisible = false
      for (const rendered of group.cards) {
        const visible = transformMatches(rendered.definition, query)
        rendered.card.hidden = !visible
        groupVisible ||= visible
        if (visible) {
          visibleCount += 1
        }
      }
      group.element.hidden = !groupVisible
    }

    empty.hidden = visibleCount !== 0
  })
}

function getGraphTransformOptions(): GraphTransformOption[] {
  const options: GraphTransformOption[] = []

  for (const definition of transforms) {
    options.push({
      id: `${definition.id}:forward`,
      definition,
      direction: "forward",
      label: `${definition.leftLabel} -> ${definition.rightLabel}`,
      inputLabel: definition.leftLabel,
      outputLabel: definition.rightLabel,
      transform: definition.leftToRight,
    })

    if (definition.rightToLeft) {
      options.push({
        id: `${definition.id}:reverse`,
        definition,
        direction: "reverse",
        label: `${definition.rightLabel} -> ${definition.leftLabel}`,
        inputLabel: definition.rightLabel,
        outputLabel: definition.leftLabel,
        transform: definition.rightToLeft,
      })
    }
  }

  return options
}

function renderGraphWorkspace(): HTMLElement {
  const options = getGraphTransformOptions()
  const optionsById = new Map(options.map((option) => [option.id, option]))
  const state = {
    nodes: new Map<string, GraphNode>(),
    connections: [] as GraphConnection[],
    connectingFrom: "",
    dragConnection: null as { from: string; x: number; y: number } | null,
    zoom: 1,
  }
  let nextNodeId = 1
  let statusTimeout: number | undefined
  let suppressSocketClick = false
  let suppressConnectionClick = false
  const minBoardWidth = 1200
  const minBoardHeight = 560
  const minZoom = 0.5
  const maxZoom = 2
  const zoomStep = 0.1

  const panel = document.createElement("section")
  panel.className = "graph-panel"

  const header = document.createElement("div")
  header.className = "graph-header"

  const title = document.createElement("h2")
  title.textContent = "node graph"

  const controls = document.createElement("div")
  controls.className = "graph-controls"

  const addSelect = document.createElement("select")
  addSelect.className = "graph-add-select"
  addSelect.title = "Transform"

  const groupedOptions = new Map<string, GraphTransformOption[]>()
  for (const option of options) {
    const group = groupedOptions.get(option.definition.group) ?? []
    group.push(option)
    groupedOptions.set(option.definition.group, group)
  }

  const appendGraphOptions = (select: HTMLSelectElement, selectedId = options[0]?.id ?? "") => {
    select.replaceChildren()
    for (const [groupName, groupOptions] of groupedOptions) {
      const optgroup = document.createElement("optgroup")
      optgroup.label = groupName
      for (const option of groupOptions) {
        const item = document.createElement("option")
        item.value = option.id
        item.textContent = option.label
        item.selected = option.id === selectedId
        optgroup.append(item)
      }
      select.append(optgroup)
    }
  }
  appendGraphOptions(addSelect)

  const addButton = document.createElement("button")
  addButton.type = "button"
  addButton.textContent = "Add"

  const addSourceButton = document.createElement("button")
  addSourceButton.type = "button"
  addSourceButton.textContent = "Source"
  addSourceButton.title = "Add source"

  const addOutputButton = document.createElement("button")
  addOutputButton.type = "button"
  addOutputButton.textContent = "Output"
  addOutputButton.title = "Add output"

  const arrangeButton = document.createElement("button")
  arrangeButton.type = "button"
  arrangeButton.textContent = "Arrange"

  const resetButton = document.createElement("button")
  resetButton.type = "button"
  resetButton.textContent = "Reset"

  const zoomControls = document.createElement("div")
  zoomControls.className = "graph-zoom-controls"
  zoomControls.setAttribute("aria-label", "Zoom")

  const zoomOutButton = document.createElement("button")
  zoomOutButton.type = "button"
  zoomOutButton.textContent = "-"
  zoomOutButton.title = "Zoom out"

  const zoomLabel = document.createElement("span")
  zoomLabel.className = "graph-zoom-label"
  zoomLabel.textContent = "100%"

  const zoomInButton = document.createElement("button")
  zoomInButton.type = "button"
  zoomInButton.textContent = "+"
  zoomInButton.title = "Zoom in"

  zoomControls.append(zoomOutButton, zoomLabel, zoomInButton)

  const status = document.createElement("span")
  status.className = "graph-status"
  status.setAttribute("role", "status")

  controls.append(
    addSelect,
    addButton,
    addSourceButton,
    addOutputButton,
    arrangeButton,
    resetButton,
    status,
  )
  header.append(title, controls)

  const board = document.createElement("div")
  board.className = "graph-board"

  const boardFrame = document.createElement("div")
  boardFrame.className = "graph-board-frame"

  const graphCanvas = document.createElement("div")
  graphCanvas.className = "graph-canvas"

  const graphWorld = document.createElement("div")
  graphWorld.className = "graph-world"

  const links = document.createElementNS("http://www.w3.org/2000/svg", "svg")
  links.classList.add("graph-links")
  links.setAttribute("aria-hidden", "true")
  graphWorld.append(links)
  graphCanvas.append(graphWorld)
  board.append(graphCanvas)
  boardFrame.append(board, zoomControls)

  panel.append(header, boardFrame)

  const setStatus = (message: string) => {
    status.textContent = message
    if (statusTimeout) {
      window.clearTimeout(statusTimeout)
    }
    if (message) {
      statusTimeout = window.setTimeout(() => {
        status.textContent = ""
      }, 2200)
    }
  }

  const createNode = (node: Omit<GraphNode, "element">) => {
    const graphNode: GraphNode = { ...node }
    state.nodes.set(graphNode.id, graphNode)
    renderGraphNode(graphNode)
    return graphNode
  }

  const positionNode = (node: GraphNode) => {
    if (!node.element) {
      return
    }
    node.element.style.left = `${node.x}px`
    node.element.style.top = `${node.y}px`
  }

  function getNodesByKind(kind: GraphNodeKind): GraphNode[] {
    return [...state.nodes.values()].filter((node) => node.kind === kind)
  }

  function getDefaultNode(kind: "source" | "output"): GraphNode | undefined {
    return state.nodes.get(kind) ?? getNodesByKind(kind)[0]
  }

  function createUniqueNodeId(kind: GraphNodeKind): string {
    let id = `${kind}-${nextNodeId}`
    nextNodeId += 1

    while (state.nodes.has(id)) {
      id = `${kind}-${nextNodeId}`
      nextNodeId += 1
    }

    return id
  }

  function getSpawnPoint(kind: GraphNodeKind, index: number): { x: number; y: number } {
    const viewportWidth = board.clientWidth || 760
    const viewportX =
      kind === "source"
        ? 28
        : kind === "output"
          ? Math.max(28, viewportWidth - 280)
          : Math.max(28, viewportWidth / 2 - 130)

    return {
      x: Math.max(20, (board.scrollLeft + viewportX) / state.zoom + (index % 3) * 24),
      y: Math.max(20, (board.scrollTop + 88) / state.zoom + Math.floor(index / 3) * 180),
    }
  }

  function getNodeTitle(node: GraphNode): string {
    const base = node.kind === "source" ? "Source" : node.kind === "output" ? "Output" : "Transform"
    const suffix = new RegExp(`^${node.kind}-(\\d+)$`).exec(node.id)?.[1]
    return suffix ? `${base} ${suffix}` : base
  }

  function wheelDeltaPixels(event: WheelEvent, delta: number, pagePixels: number): number {
    return wasm.graph_wheel_delta_pixels(event.deltaMode, delta, pagePixels)
  }

  function clampZoom(value: number): number {
    return wasm.graph_clamp_zoom(value, minZoom, maxZoom)
  }

  function syncZoomControls() {
    const percent = Math.round(state.zoom * 100)
    zoomLabel.textContent = `${percent}%`
  }

  function setZoom(
    nextZoom: number,
    anchor?: { worldX: number; worldY: number; viewportX: number; viewportY: number },
    persist = true,
  ) {
    const zoom = clampZoom(nextZoom)
    if (zoom === state.zoom) {
      return
    }

    const fallbackAnchor = {
      worldX: (board.scrollLeft + board.clientWidth / 2) / state.zoom,
      worldY: (board.scrollTop + board.clientHeight / 2) / state.zoom,
      viewportX: board.clientWidth / 2,
      viewportY: board.clientHeight / 2,
    }
    const zoomAnchor = anchor ?? fallbackAnchor

    state.zoom = zoom
    syncZoomControls()
    updateBoardExtent()
    board.scrollLeft = Math.max(0, zoomAnchor.worldX * state.zoom - zoomAnchor.viewportX)
    board.scrollTop = Math.max(0, zoomAnchor.worldY * state.zoom - zoomAnchor.viewportY)
    drawConnections()
    if (persist) {
      saveGraph()
    }
  }

  function updateBoardExtent() {
    let width = Math.max(minBoardWidth, Math.ceil(board.clientWidth / state.zoom))
    let height = Math.max(minBoardHeight, Math.ceil(board.clientHeight / state.zoom))

    for (const node of state.nodes.values()) {
      const nodeWidth = node.element?.offsetWidth ?? (node.kind === "transform" ? 238 : 220)
      const nodeHeight = node.element?.offsetHeight ?? 220
      width = Math.max(width, Math.ceil(node.x + nodeWidth + 160))
      height = Math.max(height, Math.ceil(node.y + nodeHeight + 96))
    }

    graphCanvas.style.width = `${Math.ceil(width * state.zoom)}px`
    graphCanvas.style.height = `${Math.ceil(height * state.zoom)}px`
    graphWorld.style.width = `${width}px`
    graphWorld.style.height = `${height}px`
    graphWorld.style.transform = `scale(${state.zoom})`
    links.style.width = `${width}px`
    links.style.height = `${height}px`
    links.setAttribute("width", String(width))
    links.setAttribute("height", String(height))
    links.setAttribute("viewBox", `0 0 ${width} ${height}`)
  }

  const drawConnections = () => {
    links.replaceChildren()
    updateBoardExtent()

    for (const [index, connection] of state.connections.entries()) {
      const fromSocket = getSocket(connection.from, "output")
      const toSocket = getSocket(connection.to, "input")
      if (!fromSocket || !toSocket) {
        continue
      }

      const fromPoint = getSocketPoint(fromSocket)
      const toPoint = getSocketPoint(toSocket)
      const d = getLinkPath(fromPoint.x, fromPoint.y, toPoint.x, toPoint.y)

      const path = createLinkPath(d, "graph-link")
      const hitPath = createLinkPath(d, "graph-link-hit")
      hitPath.addEventListener("pointerdown", (event) =>
        beginConnectionReconnectDrag(event, connection),
      )
      hitPath.addEventListener("click", (event) => {
        event.stopPropagation()
        if (suppressConnectionClick) {
          suppressConnectionClick = false
          return
        }
        state.connections.splice(index, 1)
        setStatus("Connection removed.")
        updateGraph()
      })
      links.append(path, hitPath)
    }

    if (state.dragConnection) {
      const fromSocket = getSocket(state.dragConnection.from, "output")
      if (fromSocket) {
        const fromPoint = getSocketPoint(fromSocket)
        const d = getLinkPath(
          fromPoint.x,
          fromPoint.y,
          state.dragConnection.x,
          state.dragConnection.y,
        )
        links.append(createLinkPath(d, "graph-link-preview"))
      }
    }
  }

  function getSocketPoint(socket: HTMLElement) {
    const rect = socket.getBoundingClientRect()
    const worldRect = graphWorld.getBoundingClientRect()
    return {
      x: (rect.left + rect.width / 2 - worldRect.left) / state.zoom,
      y: (rect.top + rect.height / 2 - worldRect.top) / state.zoom,
    }
  }

  function getBoardPoint(event: PointerEvent) {
    const worldRect = graphWorld.getBoundingClientRect()
    return {
      x: (event.clientX - worldRect.left) / state.zoom,
      y: (event.clientY - worldRect.top) / state.zoom,
    }
  }

  function getLinkPath(fromX: number, fromY: number, toX: number, toY: number) {
    return wasm.graph_link_path(fromX, fromY, toX, toY)
  }

  function createLinkPath(d: string, className: string) {
    const path = document.createElementNS("http://www.w3.org/2000/svg", "path")
    path.classList.add(className)
    path.setAttribute("d", d)
    return path
  }

  function saveGraph() {
    const nodes = [...state.nodes.values()].map((node): StoredGraphNode => {
      const stored: StoredGraphNode = {
        id: node.id,
        kind: node.kind,
        x: node.x,
        y: node.y,
      }

      if (node.optionId) {
        stored.optionId = node.optionId
      }
      if (node.kind === "source") {
        stored.value = node.input?.value ?? ""
      }

      return stored
    })

    try {
      localStorage.setItem(
        graphStorageKey,
        JSON.stringify({
          version: 1,
          nextNodeId,
          zoom: state.zoom,
          nodes,
          connections: state.connections,
        } satisfies StoredGraph),
      )
    } catch {
      // Ignore storage failures in private browsing or quota-limited contexts.
    }
  }

  function clearGraph() {
    for (const node of state.nodes.values()) {
      node.element?.remove()
    }
    state.nodes.clear()
    state.connections = []
    state.connectingFrom = ""
    state.dragConnection = null
    panel.classList.remove("is-connecting")
    links.replaceChildren()
  }

  function restoreGraph(): boolean {
    let stored: StoredGraph
    try {
      const raw = localStorage.getItem(graphStorageKey)
      if (!raw) {
        return false
      }
      stored = JSON.parse(raw) as StoredGraph
    } catch {
      return false
    }

    if (
      stored.version !== 1 ||
      !Array.isArray(stored.nodes) ||
      !stored.nodes.some((node) => node.kind === "source") ||
      !stored.nodes.some((node) => node.kind === "output")
    ) {
      return false
    }

    const maxNodeId = Math.max(
      0,
      ...stored.nodes.map((node) => {
        const match = /^(?:source|transform|output)-(\d+)$/.exec(node.id)
        return match ? Number.parseInt(match[1], 10) : 0
      }),
    )

    clearGraph()
    nextNodeId = Math.max(stored.nextNodeId || 1, maxNodeId + 1)
    state.zoom = clampZoom(stored.zoom ?? 1)
    syncZoomControls()

    for (const storedNode of stored.nodes) {
      if (
        !["source", "transform", "output"].includes(storedNode.kind) ||
        typeof storedNode.id !== "string" ||
        state.nodes.has(storedNode.id)
      ) {
        continue
      }

      const optionId =
        storedNode.kind === "transform" && optionsById.has(storedNode.optionId ?? "")
          ? storedNode.optionId
          : options[0]?.id
      const node = createNode({
        id: storedNode.id,
        kind: storedNode.kind,
        x: Number.isFinite(storedNode.x) ? storedNode.x : 20,
        y: Number.isFinite(storedNode.y) ? storedNode.y : 86,
        ...(storedNode.kind === "transform" && optionId ? { optionId } : {}),
      })

      if (storedNode.kind === "source" && node.input) {
        node.input.value = storedNode.value ?? node.input.value
      }
    }

    state.connections = (Array.isArray(stored.connections) ? stored.connections : []).filter(
      (connection) =>
        state.nodes.has(connection.from) &&
        state.nodes.has(connection.to) &&
        state.nodes.get(connection.from)?.kind !== "output" &&
        state.nodes.get(connection.to)?.kind !== "source",
    )

    updateGraph()
    return true
  }

  const updateGraph = () => {
    const results = new Map<string, { value: string; error: string }>()

    const evaluate = (nodeId: string, stack: string[] = []): { value: string; error: string } => {
      const cached = results.get(nodeId)
      if (cached) {
        return cached
      }

      const node = state.nodes.get(nodeId)
      if (!node) {
        return { value: "", error: "Missing node." }
      }

      if (stack.includes(nodeId)) {
        return { value: "", error: "Cycle detected." }
      }

      if (node.kind === "source") {
        const result = { value: node.input?.value ?? "", error: "" }
        results.set(nodeId, result)
        return result
      }

      const incoming = state.connections.find((connection) => connection.to === nodeId)
      if (!incoming) {
        const result = { value: "", error: node.kind === "output" ? "" : "Input not connected." }
        results.set(nodeId, result)
        return result
      }

      const input = evaluate(incoming.from, [...stack, nodeId])
      if (node.kind === "output") {
        results.set(nodeId, input)
        return input
      }

      if (node.input) {
        node.input.value = input.value
      }

      if (input.error) {
        const result = { value: "", error: input.error }
        results.set(nodeId, result)
        return result
      }

      const option = node.optionId ? optionsById.get(node.optionId) : undefined
      if (!option) {
        const result = { value: "", error: "Missing transform." }
        results.set(nodeId, result)
        return result
      }

      try {
        const result = { value: option.transform(input.value), error: "" }
        results.set(nodeId, result)
        return result
      } catch (caught) {
        const result = {
          value: "",
          error: caught instanceof Error ? caught.message : String(caught),
        }
        results.set(nodeId, result)
        return result
      }
    }

    for (const node of state.nodes.values()) {
      if (node.kind === "transform") {
        const result = evaluate(node.id)
        if (node.output) {
          node.output.value = result.value
        }
        if (node.error) {
          node.error.textContent = result.error
        }
        node.element?.classList.toggle("has-error", Boolean(result.error))
      }
    }

    for (const node of state.nodes.values()) {
      if (node.kind === "output") {
        const result = evaluate(node.id)
        if (node.output) {
          node.output.value = result.value
        }
        if (node.error) {
          node.error.textContent = result.error
        }
        node.element?.classList.toggle("has-error", Boolean(result.error))
      }
    }

    window.requestAnimationFrame(drawConnections)
    saveGraph()
  }

  function getSocket(nodeId: string, kind: "input" | "output") {
    return board.querySelector<HTMLElement>(`.graph-socket-${kind}[data-node-id="${nodeId}"]`)
  }

  function createSocket(nodeId: string, kind: "input" | "output") {
    const socket = document.createElement("button")
    socket.type = "button"
    socket.className = `graph-socket graph-socket-${kind}`
    socket.dataset.nodeId = nodeId
    socket.title = kind === "input" ? "Input" : "Output"
    if (kind === "output") {
      socket.addEventListener("pointerdown", (event) => beginConnectionDrag(event, nodeId))
    }
    socket.addEventListener("click", () => {
      if (suppressSocketClick) {
        suppressSocketClick = false
        return
      }

      if (kind === "output") {
        state.connectingFrom = nodeId
        panel.classList.add("is-connecting")
        setStatus("Pick an input.")
        return
      }

      if (!state.connectingFrom) {
        setStatus("Pick an output first.")
        return
      }

      connectNodes(state.connectingFrom, nodeId)
      state.connectingFrom = ""
      panel.classList.remove("is-connecting")
    })
    return socket
  }

  function beginConnectionDrag(event: PointerEvent, nodeId: string) {
    if (event.button !== 0) {
      return
    }

    event.preventDefault()
    event.stopPropagation()
    const startClientX = event.clientX
    const startClientY = event.clientY
    let moved = false
    state.connectingFrom = nodeId
    state.dragConnection = { from: nodeId, ...getBoardPoint(event) }
    panel.classList.add("is-connecting")
    drawConnections()

    const onMove = (moveEvent: PointerEvent) => {
      moved ||= Math.hypot(moveEvent.clientX - startClientX, moveEvent.clientY - startClientY) > 4
      state.dragConnection = { from: nodeId, ...getBoardPoint(moveEvent) }
      drawConnections()
    }

    const onUp = (upEvent: PointerEvent) => {
      window.removeEventListener("pointermove", onMove)
      window.removeEventListener("pointerup", onUp)

      const target = document
        .elementFromPoint(upEvent.clientX, upEvent.clientY)
        ?.closest<HTMLElement>(".graph-socket-input")
      const targetNodeId = target?.dataset.nodeId

      state.dragConnection = null
      state.connectingFrom = ""
      panel.classList.remove("is-connecting")

      if (!moved && !targetNodeId) {
        drawConnections()
        return
      }

      suppressSocketClick = true
      window.setTimeout(() => {
        suppressSocketClick = false
      })

      if (targetNodeId) {
        connectNodes(nodeId, targetNodeId)
      } else {
        setStatus("Connection cancelled.")
        drawConnections()
      }
    }

    window.addEventListener("pointermove", onMove)
    window.addEventListener("pointerup", onUp, { once: true })
  }

  function restoreConnection(connection: GraphConnection) {
    if (
      state.nodes.has(connection.from) &&
      state.nodes.has(connection.to) &&
      !state.connections.some(
        (current) => current.from === connection.from && current.to === connection.to,
      )
    ) {
      state.connections.push(connection)
    }
  }

  function beginConnectionReconnectDrag(event: PointerEvent, connection: GraphConnection) {
    if (event.button !== 0) {
      return
    }

    event.preventDefault()
    event.stopPropagation()

    const originalConnection = { ...connection }
    const startClientX = event.clientX
    const startClientY = event.clientY
    let moved = false

    const startPreview = (moveEvent: PointerEvent) => {
      moved = true
      suppressConnectionClick = true
      state.connections = state.connections.filter((current) => current !== connection)
      state.connectingFrom = originalConnection.from
      state.dragConnection = { from: originalConnection.from, ...getBoardPoint(moveEvent) }
      panel.classList.add("is-connecting")
      drawConnections()
    }

    const onMove = (moveEvent: PointerEvent) => {
      if (!moved) {
        if (Math.hypot(moveEvent.clientX - startClientX, moveEvent.clientY - startClientY) <= 4) {
          return
        }
        startPreview(moveEvent)
        return
      }

      state.dragConnection = { from: originalConnection.from, ...getBoardPoint(moveEvent) }
      drawConnections()
    }

    const onUp = (upEvent: PointerEvent) => {
      window.removeEventListener("pointermove", onMove)
      window.removeEventListener("pointerup", onUp)

      if (!moved) {
        return
      }

      const target = document
        .elementFromPoint(upEvent.clientX, upEvent.clientY)
        ?.closest<HTMLElement>(".graph-socket-input")
      const targetNodeId = target?.dataset.nodeId

      state.dragConnection = null
      state.connectingFrom = ""
      panel.classList.remove("is-connecting")

      if (targetNodeId) {
        connectNodes(originalConnection.from, targetNodeId)
      } else {
        restoreConnection(originalConnection)
        setStatus("Connection restored.")
        updateGraph()
      }

      window.setTimeout(() => {
        suppressConnectionClick = false
      })
    }

    window.addEventListener("pointermove", onMove)
    window.addEventListener("pointerup", onUp, { once: true })
  }

  function renderGraphNode(node: GraphNode) {
    const element = document.createElement("article")
    element.className = `graph-node graph-node-${node.kind}`
    element.dataset.nodeId = node.id

    const top = document.createElement("div")
    top.className = "graph-node-header"

    const label = document.createElement("span")
    label.className = "graph-node-title"
    label.textContent = getNodeTitle(node)
    top.append(label)

    const remove = document.createElement("button")
    remove.type = "button"
    remove.className = "graph-node-remove"
    remove.textContent = "x"
    remove.title = `Remove ${getNodeTitle(node).toLowerCase()}`
    remove.addEventListener("click", () => removeNode(node.id))
    top.append(remove)

    element.append(top)

    if (node.kind === "source") {
      const textarea = document.createElement("textarea")
      textarea.className = "graph-textarea"
      textarea.spellcheck = false
      textarea.value =
        transforms.find((transform) => transform.id === "utf8-hex")?.leftExample ?? "hello"
      textarea.addEventListener("input", updateGraph)
      node.input = textarea

      const sockets = document.createElement("div")
      sockets.className = "graph-node-sockets graph-node-sockets-end"
      const outputSocket = createSocket(node.id, "output")
      sockets.append(outputSocket)
      element.append(textarea, sockets)
    }

    if (node.kind === "transform") {
      const option = node.optionId ? optionsById.get(node.optionId) : undefined

      const sockets = document.createElement("div")
      sockets.className = "graph-node-sockets"
      const inputSocket = createSocket(node.id, "input")
      const outputSocket = createSocket(node.id, "output")

      const socketLabel = document.createElement("span")
      socketLabel.className = "graph-node-socket-label"
      socketLabel.textContent = option
        ? `${option.inputLabel} -> ${option.outputLabel}`
        : "Transform"
      sockets.append(inputSocket, socketLabel, outputSocket)

      const select = document.createElement("select")
      select.className = "graph-node-select"
      appendGraphOptions(select, node.optionId)
      select.addEventListener("change", () => {
        node.optionId = select.value
        const nextOption = optionsById.get(select.value)
        socketLabel.textContent = nextOption
          ? `${nextOption.inputLabel} -> ${nextOption.outputLabel}`
          : "Transform"
        updateGraph()
      })
      node.select = select

      const preview = document.createElement("div")
      preview.className = "graph-node-preview"

      const input = document.createElement("textarea")
      input.className = "graph-textarea graph-textarea-compact"
      input.readOnly = true
      input.spellcheck = false
      node.input = input

      const output = document.createElement("textarea")
      output.className = "graph-textarea graph-textarea-compact"
      output.readOnly = true
      output.spellcheck = false
      node.output = output

      preview.append(input, output)

      const error = document.createElement("div")
      error.className = "graph-node-error"
      error.setAttribute("role", "status")
      node.error = error

      element.append(sockets, select, preview, error)
    }

    if (node.kind === "output") {
      const sockets = document.createElement("div")
      sockets.className = "graph-node-sockets"
      sockets.append(createSocket(node.id, "input"))

      const output = document.createElement("textarea")
      output.className = "graph-textarea"
      output.readOnly = true
      output.spellcheck = false
      node.output = output

      const footer = document.createElement("div")
      footer.className = "graph-node-footer"

      const copy = document.createElement("button")
      copy.type = "button"
      copy.textContent = "Copy"
      copy.addEventListener("click", () => {
        copyToClipboard(output.value)
          .then(() => setStatus("Copied."))
          .catch((caught) => setStatus(caught instanceof Error ? caught.message : String(caught)))
      })

      const error = document.createElement("div")
      error.className = "graph-node-error"
      error.setAttribute("role", "status")
      node.error = error

      footer.append(copy)
      element.append(sockets, output, footer, error)
    }

    top.addEventListener("pointerdown", (event) => startDrag(event, node))
    node.element = element
    graphWorld.append(element)
    positionNode(node)
  }

  function startDrag(event: PointerEvent, node: GraphNode) {
    if (
      event.button !== 0 ||
      (event.target as HTMLElement).closest("button, select, textarea, .graph-socket")
    ) {
      return
    }

    event.preventDefault()
    const startX = event.clientX
    const startY = event.clientY
    const originX = node.x
    const originY = node.y

    const onMove = (moveEvent: PointerEvent) => {
      node.x = Math.max(8, originX + (moveEvent.clientX - startX) / state.zoom)
      node.y = Math.max(8, originY + (moveEvent.clientY - startY) / state.zoom)
      positionNode(node)
      drawConnections()
    }

    const onUp = () => {
      window.removeEventListener("pointermove", onMove)
      window.removeEventListener("pointerup", onUp)
      saveGraph()
    }

    window.addEventListener("pointermove", onMove)
    window.addEventListener("pointerup", onUp, { once: true })
  }

  function removeNode(nodeId: string) {
    const node = state.nodes.get(nodeId)
    if (!node) {
      return
    }

    if (node.kind === "source" && getNodesByKind("source").length <= 1) {
      setStatus("Keep at least one source.")
      return
    }

    if (node.kind === "output" && getNodesByKind("output").length <= 1) {
      setStatus("Keep at least one output.")
      return
    }

    node.element?.remove()
    state.nodes.delete(nodeId)
    state.connections = state.connections.filter(
      (connection) => connection.from !== nodeId && connection.to !== nodeId,
    )
    updateGraph()
  }

  function connectNodes(from: string, to: string) {
    const fromNode = state.nodes.get(from)
    const toNode = state.nodes.get(to)
    if (!fromNode || !toNode || fromNode.kind === "output" || toNode.kind === "source") {
      setStatus("That connection does not fit.")
      return
    }

    if (from === to || hasPath(to, from)) {
      setStatus("Cycle blocked.")
      return
    }

    state.connections = state.connections.filter((connection) => connection.to !== to)
    if (!state.connections.some((connection) => connection.from === from && connection.to === to)) {
      state.connections.push({ from, to })
    }
    setStatus("")
    updateGraph()
  }

  function hasPath(from: string, target: string, visited = new Set<string>()): boolean {
    if (from === target) {
      return true
    }
    if (visited.has(from)) {
      return false
    }
    visited.add(from)

    return state.connections
      .filter((connection) => connection.from === from)
      .some((connection) => hasPath(connection.to, target, visited))
  }

  function addTransformNode(optionId: string) {
    const option = optionsById.get(optionId)
    if (!option) {
      return
    }

    const transformCount = getNodesByKind("transform").length
    const spawnPoint = getSpawnPoint("transform", transformCount)
    const node = createNode({
      id: createUniqueNodeId("transform"),
      kind: "transform",
      x: spawnPoint.x,
      y: spawnPoint.y,
      optionId: option.id,
    })

    const outputNode = getDefaultNode("output")
    const sourceNode = getDefaultNode("source")
    const outputInput = outputNode
      ? state.connections.find((connection) => connection.to === outputNode.id)
      : undefined
    const previous = outputInput?.from ?? sourceNode?.id
    if (outputNode && previous) {
      state.connections = state.connections.filter((connection) => connection.to !== outputNode.id)
      state.connections.push({ from: previous, to: node.id }, { from: node.id, to: outputNode.id })
      layoutOutputChains()
    }
    updateGraph()
  }

  function addSourceNode() {
    const sourceCount = getNodesByKind("source").length
    const spawnPoint = getSpawnPoint("source", sourceCount)
    createNode({
      id: createUniqueNodeId("source"),
      kind: "source",
      x: spawnPoint.x,
      y: spawnPoint.y,
    })
    updateGraph()
    setStatus("Source added.")
  }

  function addOutputNode() {
    const outputCount = getNodesByKind("output").length
    const spawnPoint = getSpawnPoint("output", outputCount)
    createNode({
      id: createUniqueNodeId("output"),
      kind: "output",
      x: spawnPoint.x,
      y: spawnPoint.y,
    })
    updateGraph()
    setStatus("Output added.")
  }

  function getOutputChain(outputId: string): string[] {
    const chain: string[] = []
    const visited = new Set<string>()
    let current = outputId

    while (!visited.has(current)) {
      chain.unshift(current)
      visited.add(current)
      const incoming = state.connections.find((connection) => connection.to === current)
      if (!incoming) {
        break
      }
      current = incoming.from
    }

    return chain
  }

  function layoutOutputChains() {
    const nodeStep = 292
    const rowStep = 250
    const positioned = new Set<string>()

    getNodesByKind("output").forEach((outputNode, rowIndex) => {
      getOutputChain(outputNode.id).forEach((nodeId, index) => {
        if (positioned.has(nodeId)) {
          return
        }

        const node = state.nodes.get(nodeId)
        if (!node) {
          return
        }

        node.x = 20 + index * nodeStep
        node.y = 86 + rowIndex * rowStep
        positionNode(node)
        positioned.add(nodeId)
      })
    })

    let looseIndex = 0
    for (const node of state.nodes.values()) {
      if (positioned.has(node.id)) {
        continue
      }

      node.x = 20 + (looseIndex % 3) * nodeStep
      node.y = 86 + getNodesByKind("output").length * rowStep + Math.floor(looseIndex / 3) * rowStep
      positionNode(node)
      looseIndex += 1
    }

    drawConnections()
  }

  function resetGraph() {
    clearGraph()
    nextNodeId = 1
    state.zoom = 1
    syncZoomControls()

    createNode({ id: "source", kind: "source", x: 20, y: 86 })
    const defaultOption = optionsById.get("utf8-hex:forward") ?? options[0]
    if (defaultOption) {
      const node = createNode({
        id: createUniqueNodeId("transform"),
        kind: "transform",
        x: 274,
        y: 86,
        optionId: defaultOption.id,
      })
      state.connections.push({ from: "source", to: node.id }, { from: node.id, to: "output" })
    } else {
      state.connections.push({ from: "source", to: "output" })
    }
    createNode({ id: "output", kind: "output", x: 528, y: 86 })
    layoutOutputChains()
    updateGraph()
  }

  addButton.addEventListener("click", () => addTransformNode(addSelect.value))
  addSourceButton.addEventListener("click", addSourceNode)
  addOutputButton.addEventListener("click", addOutputNode)
  arrangeButton.addEventListener("click", () => {
    layoutOutputChains()
    saveGraph()
    setStatus("Arranged.")
  })
  resetButton.addEventListener("click", resetGraph)
  zoomOutButton.addEventListener("click", () => setZoom(state.zoom - zoomStep))
  zoomInButton.addEventListener("click", () => setZoom(state.zoom + zoomStep))
  board.addEventListener(
    "wheel",
    (event) => {
      if (!event.ctrlKey && !event.metaKey) {
        const horizontalDelta = event.deltaX || (event.shiftKey ? event.deltaY : 0)
        if (!horizontalDelta) {
          return
        }

        event.preventDefault()
        board.scrollLeft += wheelDeltaPixels(event, horizontalDelta, board.clientWidth)
        if (event.deltaX && event.deltaY && !event.shiftKey) {
          board.scrollTop += wheelDeltaPixels(event, event.deltaY, board.clientHeight)
        }
        drawConnections()
        return
      }

      event.preventDefault()
      const boardRect = board.getBoundingClientRect()
      const viewportX = event.clientX - boardRect.left
      const viewportY = event.clientY - boardRect.top
      const anchor = {
        worldX: (board.scrollLeft + viewportX) / state.zoom,
        worldY: (board.scrollTop + viewportY) / state.zoom,
        viewportX,
        viewportY,
      }
      const direction = event.deltaY < 0 ? 1 : -1
      setZoom(state.zoom + direction * zoomStep, anchor)
    },
    { passive: false },
  )
  board.addEventListener("scroll", drawConnections)
  window.addEventListener("resize", drawConnections)
  syncZoomControls()
  if (!restoreGraph()) {
    resetGraph()
  }

  return panel
}

function renderTransform(definition: TransformDefinition): RenderedTransform {
  const card = document.createElement("article")
  card.className = "transform-card"
  card.dataset.transformId = definition.id

  const title = document.createElement("div")
  title.className = "transform-title"

  const titleText = document.createElement("span")
  titleText.textContent = `${definition.leftLabel} ${definition.rightToLeft ? "↔" : "→"} ${
    definition.rightLabel
  }`

  const direction = document.createElement("span")
  direction.className = "transform-direction"
  direction.textContent = definition.rightToLeft ? "two-way" : "one-way"

  title.append(titleText, direction)

  const fields = document.createElement("div")
  fields.className = "transform-fields"

  const left = createField(definition.leftLabel)
  const right = createField(definition.rightLabel)
  right.textarea.readOnly = !definition.rightToLeft

  const swap = document.createElement("button")
  swap.className = "swap-button"
  swap.type = "button"
  swap.textContent = "↔"
  swap.title = "Swap"
  swap.disabled = !definition.rightToLeft

  const error = document.createElement("div")
  error.className = "transform-error"
  error.setAttribute("role", "status")

  fields.append(left.element, swap, right.element)
  card.append(title, fields, error)

  const applyLeftToRight = () =>
    applyTransform(definition.leftToRight, left.textarea.value, right.textarea, error)
  const applyRightToLeft = () => {
    if (definition.rightToLeft) {
      applyTransform(definition.rightToLeft, right.textarea.value, left.textarea, error)
    }
  }

  left.textarea.addEventListener("input", applyLeftToRight)
  right.textarea.addEventListener("input", applyRightToLeft)
  swap.addEventListener("click", () => {
    if (!definition.rightToLeft) {
      return
    }
    const nextLeft = right.textarea.value
    right.textarea.value = left.textarea.value
    left.textarea.value = nextLeft
    applyLeftToRight()
  })

  left.textarea.value = definition.leftExample
  right.textarea.value =
    definition.rightExample ?? safeTransform(definition.leftToRight, left.textarea.value)

  return { definition, card }
}

function createField(labelText: string) {
  const element = document.createElement("label")
  element.className = "transform-field"

  const label = document.createElement("span")
  label.className = "transform-label"

  const text = document.createElement("span")
  text.textContent = labelText

  const copy = document.createElement("button")
  copy.type = "button"
  copy.textContent = "Copy"

  const textarea = document.createElement("textarea")
  textarea.spellcheck = false

  copy.addEventListener("click", () => copyToClipboard(textarea.value))
  label.append(text, copy)
  element.append(label, textarea)

  return { element, textarea }
}

function applyTransform(
  transform: Transform,
  input: string,
  output: HTMLTextAreaElement,
  error: HTMLElement,
) {
  try {
    output.value = transform(input)
    error.textContent = ""
  } catch (caught) {
    error.textContent = caught instanceof Error ? caught.message : String(caught)
  }
}

function safeTransform(transform: Transform, input: string): string {
  try {
    return transform(input)
  } catch {
    return ""
  }
}

function transformMatches(definition: TransformDefinition, query: string): boolean {
  if (!query) {
    return true
  }

  return [
    definition.group,
    definition.leftLabel,
    definition.rightLabel,
    definition.leftExample,
    ...(definition.keywords ?? []),
  ]
    .join(" ")
    .toLowerCase()
    .includes(query)
}

export function transformLeftToRight(index: number) {
  legacyTransform(index, true)
}

export function transformRightToLeft(index: number) {
  legacyTransform(index, false)
}

function legacyTransform(index: number, leftToRight: boolean) {
  const source = document.getElementById(
    `${leftToRight ? "left" : "right"}${index}`,
  ) as HTMLInputElement | null
  const target = document.getElementById(
    `${leftToRight ? "right" : "left"}${index}`,
  ) as HTMLInputElement | null
  if (source && target) {
    target.value = wasm.transform_text(index, leftToRight, source.value)
  }
}

async function copyToClipboard(text: string): Promise<void> {
  if (navigator.clipboard) {
    await navigator.clipboard.writeText(text)
    return
  }

  const textarea = document.createElement("textarea")
  textarea.value = text
  textarea.style.position = "fixed"
  textarea.style.inset = "0 auto auto 0"
  document.body.append(textarea)
  textarea.focus()
  textarea.select()
  document.execCommand("copy")
  textarea.remove()
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", start, { once: true })
} else {
  start()
}
