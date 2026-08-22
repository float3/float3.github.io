/**
 * The background picker.
 *
 * Built with plain DOM rather than Preact because this file loads on every
 * page and must not pull a framework in for a panel most visitors never open.
 * The panel's contents are built lazily the first time it is shown.
 */

import { BackgroundDef, BackgroundSettings, ShaderParam } from "./types.js"

export interface MenuCallbacks {
  onSelect(id: string): void
  onParamChange(backgroundId: string, key: string, value: number): void
  onGlobalChange(patch: Partial<BackgroundSettings>): void
  onAddCustom(name: string, source: string): string | null
  onDeleteCustom(id: string): void
  /** Returns a compile error for the given source, or null when it is valid. */
  validate(source: string): string | null
}

export class BackgroundMenu {
  private root: HTMLElement
  private panel!: HTMLDivElement
  private list!: HTMLDivElement
  private paramHost!: HTMLDivElement
  private noticeHost!: HTMLParagraphElement
  private notice: string | null = null
  private built = false
  private open = false

  constructor(
    private backgrounds: () => BackgroundDef[],
    private settings: () => BackgroundSettings,
    private callbacks: MenuCallbacks,
  ) {
    this.root = adoptHost()
    this.mount()

    // Escape closes, and a click outside dismisses, matching the site's other
    // popovers.
    document.addEventListener("keydown", (event) => {
      if (event.key === "Escape" && this.open) this.toggle(false)
    })
    document.addEventListener("click", (event) => {
      // Ask the path rather than the tree: it is captured when the event is
      // dispatched, so it still names this menu when the handler that ran
      // first detached the clicked node — which is what picking a background,
      // resetting parameters or compiling a shader all do.
      const path = event.composedPath()
      const inside =
        path.length > 0 ? path.includes(this.root) : this.root.contains(event.target as Node)
      if (this.open && !inside) this.toggle(false)
    })
  }

  /**
   * Builds the toggle and the empty panel into the host element.
   *
   * Called again after every SPA navigation: the router morphs `<body>`
   * against the incoming page, which empties the host element, so the menu has
   * to be put back afterwards.
   */
  mount(host?: HTMLElement | null): void {
    if (host && host !== this.root) this.root = host
    this.built = false
    this.open = false
    this.root.setAttribute("data-open", "false")

    const toggle = document.createElement("button")
    toggle.type = "button"
    toggle.className = "bg-menu-toggle"
    toggle.setAttribute("aria-label", "Background settings")
    toggle.setAttribute("aria-expanded", "false")
    toggle.innerHTML = TOGGLE_ICON
    toggle.addEventListener("click", () => this.toggle())

    this.panel = document.createElement("div")
    this.panel.className = "bg-menu-panel"
    this.panel.setAttribute("role", "dialog")
    this.panel.setAttribute("aria-label", "Background settings")

    this.list = document.createElement("div")
    this.list.className = "bg-menu-list"
    this.paramHost = document.createElement("div")
    this.paramHost.className = "bg-menu-params"
    this.noticeHost = document.createElement("p")
    this.noticeHost.className = "bg-menu-notice"
    this.noticeHost.setAttribute("role", "status")

    this.root.replaceChildren(toggle, this.panel)
  }

  toggle(force?: boolean): void {
    this.open = force ?? !this.open
    if (this.open && !this.built) this.build()
    this.root.setAttribute("data-open", String(this.open))
    this.root.querySelector(".bg-menu-toggle")?.setAttribute("aria-expanded", String(this.open))
    if (this.open) this.refresh()
  }

  private build(): void {
    this.built = true
    const settings = this.settings()

    const header = document.createElement("div")
    header.className = "bg-menu-header"
    header.textContent = "Background"

    const globals = document.createElement("div")
    globals.className = "bg-menu-globals"
    globals.append(
      this.checkbox("Animate", settings.enabled, (value) =>
        this.callbacks.onGlobalChange({ enabled: value }),
      ),
      this.slider("Opacity", 0, 1, 0.01, settings.opacity, (value) =>
        this.callbacks.onGlobalChange({ opacity: value }),
      ),
      this.slider("Global speed", 0, 2, 0.05, settings.speed, (value) =>
        this.callbacks.onGlobalChange({ speed: value }),
      ),
    )

    this.panel.append(
      header,
      this.noticeHost,
      this.list,
      this.paramHost,
      globals,
      this.buildCustomSection(),
    )
    this.renderNotice()
  }

  /**
   * Reports why a background could not be used, or clears the last such
   * report. Without this a GPU that cannot run any of them looks exactly like
   * a menu whose items do nothing.
   */
  setNotice(text: string | null): void {
    this.notice = text
    if (this.built) this.renderNotice()
  }

  private renderNotice(): void {
    this.noticeHost.textContent = this.notice ?? ""
    this.noticeHost.hidden = this.notice === null
  }

  /** Rebuilds the list and parameter rows from current state. */
  refresh(): void {
    if (!this.built) return
    const settings = this.settings()
    this.list.replaceChildren()

    for (const background of this.backgrounds()) {
      const item = document.createElement("button")
      item.type = "button"
      item.className = "bg-menu-item"
      item.setAttribute("aria-pressed", String(background.id === settings.selected))

      const name = document.createElement("span")
      name.className = "bg-menu-item-name"
      name.textContent = background.name

      const blurb = document.createElement("span")
      blurb.className = "bg-menu-item-blurb"
      blurb.textContent = background.blurb

      const tags = document.createElement("span")
      tags.className = "bg-menu-item-tags"
      if (background.mouseReactive) tags.appendChild(tag("pointer"))
      if (background.themeReactive) tags.appendChild(tag("theme"))
      if (background.custom) tags.appendChild(tag("custom"))

      item.append(name, blurb, tags)
      item.addEventListener("click", () => {
        this.callbacks.onSelect(background.id)
        this.refresh()
      })

      if (background.custom) {
        const remove = document.createElement("button")
        remove.type = "button"
        remove.className = "bg-menu-delete"
        remove.setAttribute("aria-label", `Delete ${background.name}`)
        remove.textContent = "×"
        remove.addEventListener("click", (event) => {
          event.stopPropagation()
          this.callbacks.onDeleteCustom(background.id)
          this.refresh()
        })
        item.appendChild(remove)
      }

      this.list.appendChild(item)
    }

    this.renderParams()
  }

  private renderParams(): void {
    const settings = this.settings()
    const active = this.backgrounds().find((background) => background.id === settings.selected)
    this.paramHost.replaceChildren()
    if (!active || active.params.length === 0) return

    const heading = document.createElement("div")
    heading.className = "bg-menu-subheader"
    heading.textContent = `${active.name} settings`
    this.paramHost.appendChild(heading)

    for (const param of active.params) {
      const current = settings.params[active.id]?.[param.key] ?? param.value
      this.paramHost.appendChild(
        this.slider(param.label, param.min, param.max, param.step, current, (value) =>
          this.callbacks.onParamChange(active.id, param.key, value),
        ),
      )
    }

    const reset = document.createElement("button")
    reset.type = "button"
    reset.className = "bg-menu-reset"
    reset.textContent = "Reset to defaults"
    reset.addEventListener("click", () => {
      for (const param of active.params) {
        this.callbacks.onParamChange(active.id, param.key, param.value)
      }
      this.renderParams()
    })
    this.paramHost.appendChild(reset)
  }

  private buildCustomSection(): HTMLElement {
    const section = document.createElement("details")
    section.className = "bg-menu-custom"

    const summary = document.createElement("summary")
    summary.textContent = "Paste a shader"
    section.appendChild(summary)

    const help = document.createElement("p")
    help.className = "bg-menu-note"
    help.textContent =
      "GLSL ES 3.00. Write mainImage(out vec4 fragColor, in vec2 fragCoord) — Shadertoy shaders paste in unchanged. iResolution, iTime, iMouse and uTheme (0 light, 1 dark) are in scope."
    section.appendChild(help)

    const nameInput = document.createElement("input")
    nameInput.type = "text"
    nameInput.placeholder = "Name"
    nameInput.className = "bg-menu-input"

    const source = document.createElement("textarea")
    source.className = "bg-menu-textarea"
    source.rows = 10
    source.spellcheck = false
    source.placeholder =
      "void mainImage(out vec4 fragColor, in vec2 fragCoord) {\n  vec2 uv = fragCoord / iResolution.xy;\n  fragColor = vec4(uv, 0.5 + 0.5 * sin(iTime), 1.0);\n}"

    const status = document.createElement("pre")
    status.className = "bg-menu-status"

    const add = document.createElement("button")
    add.type = "button"
    add.className = "bg-menu-add"
    add.textContent = "Compile and use"
    add.addEventListener("click", () => {
      const text = source.value.trim()
      if (!text) {
        status.textContent = "Nothing to compile."
        status.dataset.state = "error"
        return
      }
      // Compile before storing, so a broken paste never becomes the persisted
      // background and leaves the site with no working backdrop on reload.
      const error = this.callbacks.validate(text)
      if (error) {
        status.textContent = error
        status.dataset.state = "error"
        return
      }
      const id = this.callbacks.onAddCustom(nameInput.value.trim() || "Custom shader", text)
      if (!id) {
        status.textContent = "Could not save the shader."
        status.dataset.state = "error"
        return
      }
      status.textContent = "Compiled. Now active."
      status.dataset.state = "ok"
      this.refresh()
    })

    section.append(nameInput, source, add, status)
    return section
  }

  private checkbox(label: string, value: boolean, onChange: (value: boolean) => void): HTMLElement {
    const row = document.createElement("label")
    row.className = "bg-menu-row bg-menu-check"

    // Label first, in the same column the sliders put theirs, so the box lines
    // up with the slider tracks rather than sitting out on its own margin.
    const text = document.createElement("span")
    text.className = "bg-menu-row-label"
    text.textContent = label

    const input = document.createElement("input")
    input.type = "checkbox"
    input.checked = value
    input.addEventListener("change", () => onChange(input.checked))

    row.append(text, input)
    return row
  }

  private slider(
    label: string,
    min: number,
    max: number,
    step: number,
    value: number,
    onChange: (value: number) => void,
  ): HTMLElement {
    const row = document.createElement("label")
    row.className = "bg-menu-row"

    const text = document.createElement("span")
    text.className = "bg-menu-row-label"
    text.textContent = label

    const readout = document.createElement("span")
    readout.className = "bg-menu-row-value"
    readout.textContent = format(value)

    const input = document.createElement("input")
    input.type = "range"
    input.min = String(min)
    input.max = String(max)
    input.step = String(step)
    input.value = String(value)
    input.addEventListener("input", () => {
      const next = Number(input.value)
      readout.textContent = format(next)
      onChange(next)
    })

    row.append(text, input, readout)
    return row
  }
}

/**
 * The picker's host lives in every page's markup.
 *
 * Injecting it here instead would add a `<body>` child the incoming page does
 * not have, and the SPA router's diff pairs body children by index — one extra
 * node shifts every sibling and the morph tears the page apart.
 */
function adoptHost(): HTMLElement {
  const existing = document.getElementById("bg-menu")
  if (existing) return existing
  const created = document.createElement("div")
  created.id = "bg-menu"
  document.body.appendChild(created)
  return created
}

function tag(text: string): HTMLElement {
  const element = document.createElement("em")
  element.textContent = text
  return element
}

function format(value: number): string {
  return Number.isInteger(value) ? String(value) : value.toFixed(2)
}

const TOGGLE_ICON = `<svg viewBox="0 0 24 24" width="18" height="18" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3.2"/><path d="M12 2.6v2.6M12 18.8v2.6M21.4 12h-2.6M5.2 12H2.6M18.6 5.4l-1.8 1.8M7.2 16.8l-1.8 1.8M18.6 18.6l-1.8-1.8M7.2 7.2 5.4 5.4"/></svg>`

export type { ShaderParam }
