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
  buildCommentFile,
  buildPatch,
  commentPath,
  MAX_URL_LENGTH,
  newCommentId,
  newFileUrl,
  type CommentDraft,
  type CommentTarget,
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
}

class CommentUi {
  private readonly target: CommentTarget
  private readonly article: Element | null
  private readonly state: DraftState
  private refreshHandle = 0
  private readonly cleanup: (() => void)[] = []

  private readonly text: HTMLTextAreaElement
  private readonly preview: HTMLElement
  private readonly error: HTMLElement
  private readonly submit: HTMLAnchorElement
  private readonly toolbar: HTMLElement

  constructor(private readonly section: HTMLElement) {
    this.target = JSON.parse(section.dataset.commentTarget ?? "{}") as CommentTarget
    this.article = document.querySelector(ARTICLE)

    this.text = this.require<HTMLTextAreaElement>(".comment-text")
    this.preview = this.require(".comment-preview-body")
    this.error = this.require(".comment-error")
    this.submit = this.require<HTMLAnchorElement>(".comment-submit")
    this.toolbar = this.require(".comment-selection-toolbar")

    this.state = { id: newCommentId(this.existingIds()) }

    this.markQuotes()
    this.wireRunners()
    this.wireThread()
    this.wireForm()
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

  private listen(element: EventTarget, type: string, handler: () => void): void {
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
      const { content } = this.compose()
      navigator.clipboard
        .writeText(content)
        .then(() =>
          this.say("Copied. Add it at the path shown above and open a pull request.", "note"),
        )
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
    }
  }

  /** The file exactly as the pull request would add it. */
  private compose(): { content: string; path: string } {
    const draft = this.draft()
    return {
      content: buildCommentFile(this.target, draft),
      path: commentPath(this.target, draft.id),
    }
  }

  private refresh(): void {
    window.clearTimeout(this.refreshHandle)
    this.refreshHandle = window.setTimeout(() => this.render(), 200)
  }

  private render(): void {
    if (this.text.value.trim() === "") {
      this.preview.textContent = ""
      this.disable("Write something first.")
      return
    }

    const { content, path } = this.compose()
    this.preview.textContent = `${path}\n\n${content}`

    const url = newFileUrl(this.target, path, content)
    if (url.length > MAX_URL_LENGTH) {
      this.disable("Too long for the one-click route — copy the file and add it by hand.")
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
