/**
 * The comment thread and the box that composes a new one.
 *
 * Everything here is static HTML. `js/comments.js` adopts it, wires the form
 * up, and re-finds each quote in the page body so the thread and the prose can
 * link to each other — but with the script blocked the thread still reads, and
 * the fallback note still explains how to open the pull request by hand.
 */

import { h } from "preact"
import type { ComponentChildren } from "preact"
import type {
  QuartzComponent,
  QuartzComponentConstructor,
  QuartzComponentProps,
} from "../../quartz/components/types"
import { htmlToJsx } from "../../quartz/util/jsx"
import { classNames, formatDate } from "@quartz-community/utils"
import type { FilePath } from "../../quartz/util/path"
import type { CommentRecord } from "./types"
import { styles } from "./styles"

function avatar(comment: CommentRecord): ComponentChildren {
  const author = comment.author
  if (author?.avatar !== undefined) {
    return h("img", {
      class: "comment-avatar",
      src: author.avatar,
      alt: "",
      width: 32,
      height: 32,
      loading: "lazy",
      decoding: "async",
      // The picture is hotlinked from GitHub, so at least do not tell them
      // which page of mine the reader was on when it loaded.
      referrerpolicy: "no-referrer",
    })
  }
  // A letter beats a generic silhouette: it tells the regulars in a thread
  // apart at a glance without loading anything.
  const initial = author?.name.trim()?.[0]?.toUpperCase()
  return h(
    "span",
    { class: "comment-avatar comment-avatar-empty", "aria-hidden": "true" },
    initial ?? "·",
  )
}

function name(comment: CommentRecord): ComponentChildren {
  const author = comment.author
  // No author means the file is not committed yet, which locally is the whole
  // time one is being written. Saying so beats inventing a name for it.
  if (author === undefined) {
    return h("span", { class: "comment-author is-pending" }, "uncommitted")
  }
  if (author.profile === undefined) {
    return h("span", { class: "comment-author" }, author.name)
  }
  return h(
    "a",
    {
      class: "comment-author",
      href: author.profile,
      target: "_blank",
      rel: "noopener noreferrer",
    },
    author.name,
  )
}

function commentHeader(comment: CommentRecord, locale: string): ComponentChildren {
  return h("div", { class: "comment-head" }, [
    avatar(comment),
    name(comment),
    h(
      "time",
      { class: "comment-date", datetime: comment.date },
      formatDate(new Date(comment.date), locale as never),
    ),
    h(
      "a",
      { class: "comment-permalink", href: "#comment-" + comment.id, title: "link to this comment" },
      "#",
    ),
  ])
}

/**
 * The run button, and the empty space its frame will occupy.
 *
 * The document rides in a `data-` attribute rather than in a script holder so
 * that it is inert by construction: an attribute is a string to the parser, and
 * no arrangement of characters inside one starts executing. The client copies
 * it into a sandboxed frame's `srcdoc` when the reader asks for it.
 */
function runner(comment: CommentRecord): ComponentChildren {
  if (comment.runnable === undefined) return null
  return h("div", { class: "comment-runner" }, [
    h(
      "button",
      { type: "button", class: "comment-button comment-run", "data-run": comment.runnable },
      "run this",
    ),
    h("span", { class: "comment-hint" }, "sandboxed — it cannot reach this page"),
    h("div", { class: "comment-stage" }),
  ])
}

function quoteBlock(comment: CommentRecord): ComponentChildren {
  if (comment.quote === undefined) return null
  // The jump link is inert until the script has found the passage and given it
  // an id, so it ships marked inactive and is enabled from the client.
  return h("blockquote", { class: "comment-quote", "data-quote-for": comment.id }, [
    h("span", { class: "comment-quote-text" }, comment.quote),
    h(
      "a",
      {
        class: "comment-quote-jump",
        href: "#quote-" + comment.id,
        "data-inactive": "true",
        title: "jump to this passage in the page",
      },
      "in context",
    ),
  ])
}

function renderComment(
  comment: CommentRecord,
  replies: Map<string, CommentRecord[]>,
  filePath: FilePath,
  locale: string,
  depth: number,
): ComponentChildren {
  const children: ComponentChildren[] = [
    commentHeader(comment, locale),
    quoteBlock(comment),
    h("div", { class: "comment-body" }, htmlToJsx(filePath, comment.body)),
    runner(comment),
    h(
      "button",
      { type: "button", class: "comment-button comment-reply", "data-reply-to": comment.id },
      "reply",
    ),
  ]

  // Threading stops a few levels down: past that the indentation costs more
  // than the structure buys on a phone, and the reply still names its parent.
  const answers = replies.get(comment.id)
  if (answers !== undefined && depth < 3) {
    children.push(
      h(
        "ol",
        { class: "comment-list comment-replies" },
        answers.map((reply) => renderComment(reply, replies, filePath, locale, depth + 1)),
      ),
    )
  }

  return h(
    "li",
    {
      key: comment.id,
      id: "comment-" + comment.id,
      class: "comment",
      "data-comment-id": comment.id,
      "data-quote": comment.quote,
      "data-quote-heading": comment.quoteHeading,
    },
    children,
  )
}

export const Comments: QuartzComponentConstructor = () => {
  const Component: QuartzComponent = ({ cfg, fileData, displayClass }: QuartzComponentProps) => {
    // Only a page that came from a markdown file can take a comment: folder
    // listings, tag pages and generated indices have no file in the repository
    // for a pull request to drop a comment next to.
    const target = fileData.commentTarget
    const filePath = fileData.filePath
    if (target === undefined || filePath === undefined) return null

    const comments = fileData.comments ?? []

    const replies = new Map<string, CommentRecord[]>()
    for (const comment of comments) {
      if (comment.replyTo === undefined) continue
      const existing = replies.get(comment.replyTo)
      if (existing) existing.push(comment)
      else replies.set(comment.replyTo, [comment])
    }

    // A reply whose parent is missing would otherwise drop out of the thread.
    const known = new Set(comments.map((comment) => comment.id))
    const roots = comments.filter(
      (comment) => comment.replyTo === undefined || !known.has(comment.replyTo),
    )

    return h(
      "section",
      {
        class: classNames(displayClass, "comments"),
        id: "comments",
        "data-comment-target": JSON.stringify(target),
      },
      [
        h("h2", { class: "comments-heading" }, [
          "Comments",
          h("span", { class: "comment-count" }, String(comments.length)),
        ]),
        comments.length === 0
          ? h(
              "p",
              { class: "comments-empty" },
              "Nothing here yet. Select any part of the page to quote it, or just start writing.",
            )
          : h(
              "ol",
              { class: "comment-list" },
              roots.map((comment) =>
                renderComment(comment, replies, filePath as FilePath, cfg.locale, 0),
              ),
            ),
        composer(target.repo),
      ],
    )
  }

  Component.css = styles
  return Component
}

/**
 * The compose box, emitted inert.
 *
 * It is real markup rather than something the script assembles, so the SPA
 * router has a stable tree to morph against on every navigation.
 */
function composer(repo: string): ComponentChildren {
  const button = (className: string, text: string) =>
    h("button", { type: "button", class: "comment-button " + className }, text)

  return h("div", { class: "comment-composer" }, [
    h("h3", { class: "comment-composer-heading" }, "Add a comment"),
    h(
      "p",
      { class: "comment-composer-note" },
      "Comments are files in the repository. Writing one here opens a pull request against " +
        repo +
        ", and it appears on the page once that is merged",
    ),

    h("div", { class: "comment-replying", hidden: true }, [
      h("span", { class: "comment-label" }, "Replying to"),
      h("span", { class: "comment-replying-to" }),
      button("comment-drop-reply", "not a reply"),
    ]),

    h("div", { class: "comment-quoting", hidden: true }, [
      h("span", { class: "comment-label" }, "Quoting"),
      h("blockquote", { class: "comment-quote comment-quote-draft" }),
      button("comment-drop-quote", "remove quote"),
    ]),

    h("label", { class: "comment-field" }, [
      h("span", { class: "comment-label" }, "Comment"),
      h("textarea", {
        class: "comment-input comment-text",
        rows: 6,
        placeholder: "Markdown works here. So does html or <script>",
      }),
    ]),

    h("details", { class: "comment-preview" }, [
      h("summary", {}, "the file this will add"),
      h("pre", { class: "comment-preview-body" }),
    ]),

    h("p", { class: "comment-error", hidden: true, role: "status" }),

    h("div", { class: "comment-actions" }, [
      h(
        "a",
        {
          class: "comment-button comment-submit",
          href: "#",
          target: "_blank",
          rel: "noopener noreferrer",
          "aria-disabled": "true",
        },
        "open a pull request",
      ),
      button("comment-copy", "copy the file"),
      button("comment-patch", "download a patch"),
    ]),

    h(
      "noscript",
      {},
      h(
        "p",
        { class: "comment-hint" },
        "Commenting needs JavaScript, because the file is assembled in the browser. Without it, add a file named like the others next to this page in " +
          repo +
          " and open a pull request.",
      ),
    ),

    // Floats beside the selection. It lives inside the section so a navigation
    // carries it away with the rest of the page rather than stranding it.
    h("div", { class: "comment-selection-toolbar", hidden: true }, [
      h("button", { type: "button", class: "comment-button comment-quote-button" }, "quote this"),
    ]),
  ])
}

export default Comments
export function init(): void {}
