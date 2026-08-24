/**
 * The client half of the comment system.
 *
 * The thread itself is static HTML built at compile time; this adopts it,
 * re-finds every quoted passage in the article so the prose and the thread can
 * point at each other, and turns the compose box into a pull request.
 *
 * It runs on every page and does nothing at all on pages without a thread,
 * which is most of them.
 */

import {
  buildPatch,
  buildSubmission,
  MAX_URL_LENGTH,
  newCommentId,
  type CommentDraft,
  type Submission,
  type CommentTarget,
  type Route,
} from "./file.js"
import { captureSelection, linkQuotes, unlinkQuotes } from "./quotes.js"
import { wireRunner } from "./runner.js"

const ARTICLE = "article.popover-hint"

interface DraftState {
  id: string
  /** Fixed on the first keystroke, so the preview stops moving as you type. */
  date?: string
  quote?: string
  quoteHeading?: string
  replyTo?: string
  /** id of the comment being rewritten, when the reader pressed edit. */
  editing?: string
  /** Login the edited comment belongs to, for the warning if it is not theirs. */
  editingAuthor?: string
}

class CommentUi {
  private readonly target: CommentTarget
  private readonly article: Element | null
  private readonly state: DraftState
  private refreshHandle = 0
  private readonly cleanup: (() => void)[] = []

  private readonly text: HTMLTextAreaElement
  private readonly preview: HTMLElement
  private readonly previewLabel: HTMLElement
  private readonly error: HTMLElement
  private readonly submit: HTMLAnchorElement
  private readonly toolbar: HTMLElement
  private readonly post: HTMLElement
  private readonly postToggle: HTMLElement
  private readonly postMenu: HTMLElement
  private route: Route = "issue"

  constructor(private readonly section: HTMLElement) {
    this.target = JSON.parse(section.dataset.commentTarget ?? "{}") as CommentTarget
    this.article = document.querySelector(ARTICLE)

    this.text = this.require<HTMLTextAreaElement>(".comment-text")
    this.preview = this.require(".comment-preview-body")
    this.previewLabel = this.require(".comment-preview-label")
    this.error = this.require(".comment-error")
    this.submit = this.require<HTMLAnchorElement>(".comment-submit")
    this.toolbar = this.require(".comment-selection-toolbar")
    this.post = this.require(".comment-post")
    this.postToggle = this.require(".comment-post-toggle")
    this.postMenu = this.require(".comment-post-menu")

    this.state = { id: newCommentId(this.existingIds()) }

    this.markQuotes()
    this.wireRunners()
    this.wireThread()
    this.wireForm()
    this.wirePostMenu()
    this.wireSelection()
    this.render()
  }

  /** The section is emitted whole, so a missing element means a broken build. */
  private require<T extends HTMLElement = HTMLElement>(selector: string): T {
    const element = this.section.querySelector<T>(selector)
    if (element === null) throw new Error(`comments: no ${selector}`)
    return element
  }

  private existingIds(): Set<string> {
    return new Set(
      [...this.section.querySelectorAll<HTMLElement>(".comment")].map(
        (comment) => comment.dataset.commentId ?? "",
      ),
    )
  }

  destroy(): void {
    for (const undo of this.cleanup) undo()
    if (this.article !== null) unlinkQuotes(this.article)
  }

  private listen(element: EventTarget, type: string, handler: (event: Event) => void): void {
    element.addEventListener(type, handler)
    this.cleanup.push(() => element.removeEventListener(type, handler))
  }

  // -------------------------------------------------------------------------
  // Quotes

  private markQuotes(): void {
    if (this.article === null) return
    const quoting = [...this.section.querySelectorAll<HTMLElement>(".comment[data-quote]")]
    const located = linkQuotes(this.article, quoting)

    // Only the links that lead somewhere are turned on. A quote whose passage
    // has since been rewritten stays in the thread, and stops pretending the
    // sentence is still there to jump to.
    for (const comment of quoting) {
      const id = comment.dataset.commentId
      if (id === undefined || !located.has(id)) continue
      const jump = comment.querySelector<HTMLElement>(".comment-quote-jump")
      if (jump !== null) delete jump.dataset.inactive
    }
  }

  /**
   * Comments carrying code get a run button. Nothing starts on load — the
   * reader presses it — and the teardown below stops anything still running
   * when the page is navigated away from.
   */
  private wireRunners(): void {
    for (const button of this.section.querySelectorAll<HTMLElement>(".comment-run")) {
      this.cleanup.push(
        wireRunner(button, () => {
          // A frame appearing or vanishing moves everything under it, and the
          // selection toolbar is positioned in viewport coordinates.
          this.toolbar.hidden = true
        }),
      )
    }
  }

  private wireSelection(): void {
    const hide = () => {
      this.toolbar.hidden = true
    }

    this.listen(document, "selectionchange", () => {
      if (this.article === null) return
      const captured = captureSelection(this.article)
      if (captured === undefined) {
        hide()
        return
      }

      const selection = window.getSelection()
      const rect = selection?.getRangeAt(0).getBoundingClientRect()
      if (rect === undefined || (rect.width === 0 && rect.height === 0)) {
        hide()
        return
      }

      // Above the selection where there is room, below it at the top of the
      // viewport — the button must not cover the words being quoted.
      const above = rect.top > 48
      this.toolbar.style.left = `${Math.max(8, rect.left)}px`
      this.toolbar.style.top = `${above ? rect.top - 40 : rect.bottom + 8}px`
      this.toolbar.hidden = false
    })

    this.listen(this.require(".comment-quote-button"), "click", () => {
      if (this.article === null) return
      const captured = captureSelection(this.article)
      hide()
      if (captured === undefined) return

      this.state.quote = captured.text
      this.state.quoteHeading = captured.heading
      this.showQuote()
      this.text.focus()
      this.refresh()
    })

    this.listen(this.require(".comment-drop-quote"), "click", () => {
      this.state.quote = undefined
      this.state.quoteHeading = undefined
      this.showQuote()
      this.refresh()
    })
  }

  private showQuote(): void {
    this.require(".comment-quoting").hidden = this.state.quote === undefined
    this.require(".comment-quote-draft").textContent = this.state.quote ?? ""
  }

  // -------------------------------------------------------------------------
  // Thread

  private wireThread(): void {
    for (const button of this.section.querySelectorAll<HTMLElement>(".comment-reply")) {
      this.listen(button, "click", () => {
        this.state.replyTo = button.dataset.replyTo
        this.showReply()
        this.text.focus()
        this.text.scrollIntoView({ block: "center", behavior: "smooth" })
        this.refresh()
      })
    }

    this.listen(this.require(".comment-drop-reply"), "click", () => {
      this.state.replyTo = undefined
      this.showReply()
      this.refresh()
    })

    for (const button of this.section.querySelectorAll<HTMLElement>(".comment-edit")) {
      this.listen(button, "click", () => {
        // An edit is an ordinary submission carrying the old text and the id it
        // replaces. The workflow rewrites that file rather than adding one, and
        // refuses unless the account opening the issue owns the comment — so
        // this button being pressable by anyone costs nothing.
        this.state.editing = button.dataset.editing
        this.state.editingAuthor = button.dataset.author
        this.state.replyTo = button.dataset.replyTo
        this.state.quote = button.dataset.quote
        this.state.quoteHeading = button.dataset.quoteHeading
        this.text.value = button.dataset.source ?? ""

        // Only the issue route can say which comment it replaces.
        const issue = this.postMenu.querySelector<HTMLElement>('[data-route="issue"]')
        if (issue !== null) this.selectRoute(issue)

        this.showEdit()
        this.showReply()
        this.showQuote()
        this.text.focus()
        this.text.scrollIntoView({ block: "center", behavior: "smooth" })
        this.refresh()
      })
    }

    this.listen(this.require(".comment-drop-edit"), "click", () => this.clearEdit())
  }

  private clearEdit(): void {
    this.state.editing = undefined
    this.state.editingAuthor = undefined
    this.state.quote = undefined
    this.state.quoteHeading = undefined
    this.state.replyTo = undefined
    this.text.value = ""
    this.showEdit()
    this.showReply()
    this.showQuote()
    this.refresh()
  }

  private showEdit(): void {
    const block = this.require(".comment-editing")
    block.hidden = this.state.editing === undefined
    if (this.state.editing === undefined) return
    // Said plainly rather than enforced here: the browser has no idea who is
    // signed in to GitHub, and finding out would cost a request to them on
    // every page load. The workflow is where the rule actually lives.
    this.require(".comment-editing-note").textContent =
      `this comment, which belongs to @${this.state.editingAuthor ?? "someone"} — the edit only lands if you open the issue as them`
  }

  private showReply(): void {
    const block = this.require(".comment-replying")
    block.hidden = this.state.replyTo === undefined
    if (this.state.replyTo === undefined) return

    const parent = this.section.querySelector<HTMLElement>(
      `.comment[data-comment-id="${CSS.escape(this.state.replyTo)}"] .comment-author`,
    )
    this.require(".comment-replying-to").textContent = parent?.textContent ?? this.state.replyTo
  }

  // -------------------------------------------------------------------------
  // Composing

  private wireForm(): void {
    this.listen(this.text, "input", () => {
      // The timestamp is pinned to when writing started rather than to the last
      // keystroke, so the preview below stops rewriting itself as you type. It
      // is only a fallback anyway: the published date comes from the commit.
      this.state.date ??= new Date().toISOString()
      this.refresh()
    })

    this.listen(this.require(".comment-copy"), "click", () => {
      if (!this.written()) return
      // Whatever the preview is showing, which is whatever the button would
      // send. Copying the file while the screen shows an issue would be the
      // same mismatch the old label had.
      const { preview } = this.compose()
      navigator.clipboard
        .writeText(preview)
        .then(() => this.say("Copied.", "note"))
        .catch(() => this.say("The clipboard refused; select the preview and copy it by hand."))
    })

    this.listen(this.require(".comment-patch"), "click", () => {
      if (!this.written()) return
      const { content, path } = this.compose()
      const name = path.split("/").pop() ?? "comment"
      download(new Blob([buildPatch(path, content)], { type: "text/x-patch" }), `${name}.patch`)
      this.say(
        `Saved ${name}.patch. Apply it with \`git apply\`, then open a pull request.`,
        "note",
      )
    })
  }

  private written(): boolean {
    if (this.text.value.trim() !== "") return true
    this.say("Write something first.")
    return false
  }

  private draft(): CommentDraft {
    return {
      id: this.state.id,
      date: this.state.date ?? new Date().toISOString(),
      body: this.text.value,
      replyTo: this.state.replyTo,
      quote: this.state.quote,
      quoteHeading: this.state.quoteHeading,
      editing: this.state.editing,
    }
  }

  // -------------------------------------------------------------------------
  // The split button

  private menuIsOpen(): boolean {
    return this.post.dataset.open === "true"
  }

  private openMenu(open: boolean): void {
    this.post.dataset.open = String(open)
    this.postMenu.hidden = !open
    this.postToggle.setAttribute("aria-expanded", String(open))
  }

  private selectRoute(option: HTMLElement): void {
    const route = option.dataset.route as Route | undefined
    if (route === undefined) return

    this.route = route
    for (const other of this.postMenu.querySelectorAll<HTMLElement>(".comment-post-option")) {
      other.setAttribute("aria-checked", String(other === option))
    }

    // The label comes off the option rather than from a table in here: the
    // component owns the wording, this owns what happens when it is clicked.
    const title = option.querySelector(".comment-post-title")?.textContent
    if (title) this.submit.textContent = title

    // Only the issue route can say which comment it replaces, so leaving it
    // turns the draft back into a new comment rather than dropping the link
    // to the comment being replaced without saying so.
    if (route !== "issue" && this.state.editing !== undefined) {
      this.state.editing = undefined
      this.state.editingAuthor = undefined
      this.showEdit()
    }

    this.openMenu(false)
    this.render()
  }

  private wirePostMenu(): void {
    this.listen(this.postToggle, "click", () => {
      this.openMenu(!this.menuIsOpen())
    })

    for (const option of this.postMenu.querySelectorAll<HTMLElement>(".comment-post-option")) {
      this.listen(option, "click", () => this.selectRoute(option))
    }

    // A menu that stays open after the page has moved on from it is a menu
    // covering something the reader is trying to read.
    this.listen(document, "pointerdown", (event) => {
      const target = event.target
      if (target instanceof Node && this.post.contains(target)) return
      this.openMenu(false)
    })

    this.listen(document, "keydown", (event) => {
      if (event instanceof KeyboardEvent && event.key === "Escape" && this.menuIsOpen()) {
        this.openMenu(false)
        this.postToggle.focus()
      }
    })
  }

  /** The file, and the link that submits it by whichever route is selected. */
  private compose(): Submission {
    return buildSubmission(this.route, this.target, this.draft())
  }

  private refresh(): void {
    window.clearTimeout(this.refreshHandle)
    this.refreshHandle = window.setTimeout(() => this.render(), 200)
  }

  private render(): void {
    const { preview, previewLabel, url } = this.compose()

    this.previewLabel.textContent = previewLabel

    if (this.text.value.trim() === "") {
      this.preview.textContent = ""
      this.disable("Write something first.")
      return
    }

    this.preview.textContent = preview

    if (url.length > MAX_URL_LENGTH) {
      // Every route's fallback is to send the same thing by hand, and the copy
      // button hands over exactly what is on screen for whichever route it is.
      this.disable("Too long for the one-click route — copy it and send it by hand.")
      return
    }

    this.submit.href = url
    this.submit.removeAttribute("aria-disabled")
    this.error.hidden = true
  }

  private disable(reason: string): void {
    this.submit.href = "#"
    this.submit.setAttribute("aria-disabled", "true")
    this.say(reason)
  }

  /** One line for both "here is why it will not go" and "here is what went". */
  private say(message: string, tone: "problem" | "note" = "problem"): void {
    this.error.hidden = false
    this.error.dataset.tone = tone
    this.error.textContent = message
  }
}

function download(blob: Blob, filename: string): void {
  const url = URL.createObjectURL(blob)
  const link = document.createElement("a")
  link.href = url
  link.download = filename
  link.click()
  URL.revokeObjectURL(url)
}

let current: CommentUi | null = null

function mount(): void {
  current?.destroy()
  current = null

  const section = document.querySelector<HTMLElement>("section.comments")
  if (section === null) return

  try {
    current = new CommentUi(section)
  } catch (error) {
    // A thread that fails to wire up still reads; it just cannot be added to.
    console.error("comments failed to mount", error)
  }
}

// Quartz's SPA router replaces the page body, taking the whole section with it,
// so there is nothing to preserve across a navigation — only to rebuild.
document.addEventListener("nav", mount)

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", mount, { once: true })
} else {
  mount()
}
