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
import type { CommentRecord, CommentRevision, CommentTarget } from "./types"
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
  // An email author has no profile to link to, and the address itself is not
  // printed — only the part before the `@`, so the page is not a place to
  // harvest it from. The full address stays in the file, where an edit can be
  // checked against it.
  if (author.profile === undefined) {
    return h(
      "span",
      {
        class: "comment-author",
        title: author.email !== undefined ? "sent by email" : undefined,
      },
      author.name,
    )
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
      // It starts on its own once scrolled to, so this reads as the way to stop
      // it and start it again rather than as the way to begin.
      "run it",
    ),
    h("span", { class: "comment-hint" }, "runs sandboxed"),
    h("div", { class: "comment-stage" }),
  ])
}

/**
 * When the comment was written, and every time it has been revised since.
 *
 * The list is the timeline only — each revision's date and the issue it arrived
 * as. The text of an old revision is not duplicated into the file, because git
 * already holds every one of them; the "all revisions" link goes there.
 */
function history(comment: CommentRecord, repo: string, locale: string): ComponentChildren {
  if (comment.edited === undefined) return null

  const when = (revision: CommentRevision) => formatDate(new Date(revision.date), locale as never)

  const entry = (revision: CommentRevision, index: number) => {
    const label = `${index === 0 ? "written" : "edited"} ${when(revision)}`
    return h(
      "li",
      { key: revision.date },
      revision.issue !== undefined
        ? h(
            "a",
            {
              href: `https://github.com/${repo}/issues/${revision.issue}`,
              target: "_blank",
              rel: "noopener noreferrer",
            },
            label,
          )
        : label,
    )
  }

  return h("details", { class: "comment-history" }, [
    h(
      "summary",
      {},
      `edited ${when(comment.history[comment.history.length - 1])} · ${comment.history.length} versions`,
    ),
    h("ol", {}, comment.history.map(entry)),
    h(
      "a",
      {
        class: "comment-history-diffs",
        href: `https://github.com/${repo}/commits/HEAD/${encodePath(comment.file)}`,
        target: "_blank",
        rel: "noopener noreferrer",
      },
      "all revisions, with their text",
    ),
  ])
}

const encodePath = (file: string) => file.split("/").map(encodeURIComponent).join("/")

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
  repo: string,
  locale: string,
  depth: number,
): ComponentChildren {
  const children: ComponentChildren[] = [
    commentHeader(comment, locale),
    quoteBlock(comment),
    h("div", { class: "comment-body" }, htmlToJsx(filePath, comment.body)),
    runner(comment),
    history(comment, repo, locale),
    h("div", { class: "comment-tools" }, [
      h(
        "button",
        { type: "button", class: "comment-button comment-reply", "data-reply-to": comment.id },
        "reply",
      ),
      // The source rides along so the compose box can prefill it. Only the
      // account named here can actually land the edit — the workflow checks the
      // file's author against whoever opened the issue — so this being a plain
      // button that anyone can press costs nothing.
      comment.author?.login !== undefined
        ? h(
            "button",
            {
              type: "button",
              class: "comment-button comment-edit",
              "data-editing": comment.id,
              "data-author": comment.author.login,
              "data-source": comment.source,
              "data-quote": comment.quote,
              "data-quote-heading": comment.quoteHeading,
              "data-reply-to": comment.replyTo,
            },
            "edit",
          )
        : null,
    ]),
  ]

  // Threading stops a few levels down: past that the indentation costs more
  // than the structure buys on a phone, and the reply still names its parent.
  const answers = replies.get(comment.id)
  if (answers !== undefined && depth < 3) {
    children.push(
      h(
        "ol",
        { class: "comment-list comment-replies" },
        answers.map((reply) => renderComment(reply, replies, filePath, repo, locale, depth + 1)),
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
                renderComment(comment, replies, filePath as FilePath, target.repo, cfg.locale, 0),
              ),
            ),
        composer(target),
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
function composer(target: CommentTarget): ComponentChildren {
  const repo = target.repo
  const button = (className: string, text: string) =>
    h("button", { type: "button", class: "comment-button " + className }, text)

  // Issue first, and the default: it is the only route that ends with the
  // comment attributed to whoever wrote it without them having to do anything
  // about it. The blurbs are here rather than in the client because they are
  // words, and the client's job is behaviour — it reads the title back off
  // whichever option was picked.
  const routes: { route: string; label: string; blurb: string }[] = [
    {
      route: "issue",
      label: "post using github issues",
      blurb: "A workflow turns it into a pull request in your name, then closes the issue.",
    },
    {
      route: "pull-request",
      label: "post using github pull request",
      blurb: `Opens GitHub's file editor, forking ${repo}.`,
    },
  ]
  if (target.email !== undefined) {
    routes.push({
      route: "email",
      label: "post by email",
      blurb:
        "Opens your mail client. No GitHub account needed; it gets added by hand. If you want to post it anonymously, put ANON at the end of the subject line.",
    })
  }

  return h("div", { class: "comment-composer" }, [
    h("h3", { class: "comment-composer-heading" }, "Add a comment"),
    h(
      "p",
      { class: "comment-composer-note" },
      "Comments are files in the repository. Writing one here opens a pull request against " +
        repo +
        ", and it appears on the page once that is merged",
    ),

    h("div", { class: "comment-editing", hidden: true }, [
      h("span", { class: "comment-label" }, "Editing"),
      h("span", { class: "comment-editing-note" }),
      button("comment-drop-edit", "start a new comment instead"),
    ]),

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

    // The two buttons in here act on the file shown above them, and anyone who
    // wants either already knows what a patch is — so they live behind the
    // same fold as the file itself rather than beside the one button that
    // everybody uses.
    h("details", { class: "comment-preview" }, [
      // Named by the client, because what the button sends is a different
      // thing on each route. The default matches the default route, so it reads
      // correctly before any script has run.
      h("summary", {}, h("span", { class: "comment-preview-label" }, "the issue this will open")),
      h("pre", { class: "comment-preview-body" }),
      h("div", { class: "comment-preview-actions" }, [
        button("comment-copy", "copy the file"),
        button("comment-patch", "download a patch"),
      ]),
    ]),

    h("p", { class: "comment-error", hidden: true, role: "status" }),

    // Only for the pull-request route, and directly above the button it is
    // about. GitHub loses the prefilled file when it has to fork the repository
    // on the way through, so the fork has to exist first — and by the time the
    // reader finds that out on their own, their comment is gone.
    h("div", { class: "comment-fork-warning", hidden: true, role: "note" }, [
      h("p", {}, [
        h("strong", {}, "Fork it first. "),
        "Automatically forking the Repo on edit does not currently work (known issue according to support) and produces 'unknown error'.",
      ]),
      h("ol", {}, [
        h("li", {}, [
          h(
            "a",
            {
              href: "https://github.com/" + repo + "/fork",
              target: "_blank",
              rel: "noopener noreferrer",
            },
            "Fork " + repo,
          ),
          ", in another tab.",
        ]),
        h("li", {}, "Come back here and press the button below."),
        h("li", {}, "On GitHub's editor, commit the file and open the pull request."),
      ]),
    ]),

    // A split button: the action on the left, the choice of action behind the
    // caret. The default is the one almost everybody wants, so the alternatives
    // cost a click to find and nothing to ignore — which is why there is no
    // label sitting beside it explaining what the menu is for.
    h("div", { class: "comment-actions" }, [
      h("div", { class: "comment-post", "data-open": "false" }, [
        h(
          "a",
          {
            class: "comment-button comment-submit",
            href: "#",
            target: "_blank",
            rel: "noopener noreferrer",
            "aria-disabled": "true",
          },
          routes[0].label,
        ),
        h(
          "button",
          {
            type: "button",
            class: "comment-button comment-post-toggle",
            "aria-haspopup": "menu",
            "aria-expanded": "false",
            "aria-label": "choose how to post this",
          },
          // A caret, drawn rather than typed: the glyph fonts have for this
          // sits differently in each of them.
          h(
            "svg",
            { viewBox: "0 0 16 16", width: "12", height: "12", "aria-hidden": "true" },
            h("path", {
              d: "M4 6l4 4 4-4",
              fill: "none",
              stroke: "currentColor",
              "stroke-width": "2",
            }),
          ),
        ),
        h(
          "div",
          { class: "comment-post-menu", role: "menu", hidden: true },
          routes.map((route, index) =>
            h(
              "button",
              {
                key: route.route,
                type: "button",
                role: "menuitemradio",
                "aria-checked": index === 0 ? "true" : "false",
                class: "comment-post-option",
                "data-route": route.route,
              },
              [
                h("span", { class: "comment-post-check", "aria-hidden": "true" }, "✓"),
                h("span", { class: "comment-post-text" }, [
                  h("span", { class: "comment-post-title" }, route.label),
                  h("span", { class: "comment-post-blurb" }, route.blurb),
                ]),
              ],
            ),
          ),
        ),
      ]),
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
