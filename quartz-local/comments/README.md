# comments

Comments as files in the repository, contributed as pull requests.

There is no server, no database and no third-party widget. A reader writes a
comment in the page's compose box; the browser assembles the markdown file it
would take to publish that comment and hands it to GitHub's "create a new file"
editor with the path and the body already filled in. GitHub forks the repository
for them if they cannot push to it, and its own commit form opens the pull
request. Merging the pull request is what publishes the comment, so moderation
is the merge button and nothing else.

Commenting therefore needs a GitHub account. In exchange it needs nothing else:
no name to type, no picture to upload, no key to keep — the account is the
identity.

## Where the files live

Next to the page they comment on, sharing its name:

```
content/blog/freewill.md
content/blog/freewill.comment.aaaa1111.md
content/blog/freewill.comment.bbbb2222.md
```

The id is 8 hex characters and only needs to be unique within one page — the
filename already carries the page. `quartz.config.yaml` lists
`**/*.comment.*.md` under `ignorePatterns`, so these never become pages of their
own, never enter search, the graph, or the feed. The transformer reads them off
disk directly instead.

## The file

```markdown
---
parent: "blog/freewill.md"
date: "2026-01-02T10:00:00.000Z"
replyTo: "aaaa1111"
quote: "Neurons fire; muscles move."
quoteHeading: "where-do-our-choices-come-from"
---

The body, in markdown.
```

Only `parent` and a non-empty body are required. The body is markdown, and may
contain HTML and `<script>` — see below for where each of those ends up. Every value is written as a
double-quoted YAML scalar, which accepts exactly the JSON string escapes, so the
compose box can emit any of them with `JSON.stringify`.

| field          | meaning                                                                                  |
| -------------- | ---------------------------------------------------------------------------------------- |
| `parent`       | content-relative path of the page.                                                       |
| `replyTo`      | id of the comment being answered. Threading renders three levels deep.                   |
| `quote`        | text quoted verbatim from the page.                                                      |
| `quoteHeading` | slug of the heading it sat under, used to disambiguate a phrase that appears twice.      |
| `date`         | a fallback, used only while the file is uncommitted. The published date is the commit's. |

## Authorship

Nothing in the file says who wrote it, deliberately. The frontmatter is composed
by a stranger's browser and could claim anything; what cannot be typed is the
commit. A comment only reaches the site by merging a pull request, and GitHub
attributes the commit that added the file to whoever opened it — so `authors.ts`
runs one `git log --diff-filter=A` over the comment files and reads the author
from there. Impersonation would require getting a lie merged.

The commit email is what identifies the account. GitHub's private-email form,
`12345+octocat@users.noreply.github.com`, carries the numeric account id, which
is what the avatar URL is built from — a login can be changed or reused, an id
cannot. Someone whose commits use a real address gives no login to look up, so
they get the name git recorded and a placeholder initial.

Avatars are hotlinked from `avatars.githubusercontent.com` with
`referrerpolicy="no-referrer"`. That does mean a reader's IP reaches GitHub when
a thread has comments in it.

A file that is not committed yet — which, locally, is the whole time one is being
tested — renders as _uncommitted_ with no picture.

## Quoting

The quote is stored as text, not as an offset or a generated anchor, because a
position-based reference would rot silently the first time the page is edited.
`ts/src/comments/quotes.ts` flattens the article's text nodes at read time,
collapses whitespace, finds the passage, and wraps it in `<mark id="quote-<id>">`
with a `¶` backlink to `#comment-<id>`. The comment's own blockquote links the
other way, to `#quote-<id>`.

A passage that has since been rewritten simply is not found: the quote still
shows in the thread and its "in context" link stays hidden, rather than
scrolling somewhere wrong.

## HTML, and running code

A comment may contain HTML, and may contain a script. Both are wanted — someone
building a small thing in a comment is the good case — but they are put in
different places.

**Markup renders inline**, with a wide allowlist: text formatting, lists,
tables, `details`/`summary`, figures, images, media, and a `style` attribute for
anything cosmetic. Because the body goes through a real HTML parser before it is
cleaned, a comment cannot escape its own box by writing unbalanced closing tags:
the parser drops them, and the tree the page renders is well-formed by
construction. Headings are demoted so a comment cannot compete with the page's
outline, `style` declarations that leave the box or fetch a URL are dropped, and
every link gets `rel="nofollow ugc noopener noreferrer"`.

**Code runs in a sandbox.** A comment containing a `<script>` or `<style>` tag
gets a _run this_ button, and pressing it loads the whole comment — markup and
all — into an iframe with `sandbox="allow-scripts allow-modals"` and no
`allow-same-origin`. A comment with fenced `html`, `css` and `js` blocks instead
gets them stitched into one page, which is the shape a pasted snippet arrives
in. Either way nothing executes until a reader asks for it, and pressing the
button again throws the frame away — as does navigating to another page.

Without `allow-same-origin` the frame is in an opaque origin. Measured from
inside one:

```
origin: null
parent title blocked: SecurityError
localStorage blocked: SecurityError
cookie blocked: SecurityError
```

That is the whole reason for the split, and it is not about distrusting the
commenter: every comment is read before it merges. It is that a merge is a
judgement made once, by eye, on code that may be minified or clever, while the
cost of getting it wrong is script running on this origin for every visitor
afterwards. The sandbox makes a mistaken merge cost a silly iframe instead.

If a particular comment ever deserves to run on the real origin, the honest way
to do it is to move its code into the page or into `ts/` — where it goes through
the same review as everything else here — rather than to widen the sandbox for
all comments at once. Adding `allow-same-origin` to the list in
`ts/src/comments/runner.ts` would do exactly that, and is the one line in this
plugin worth being suspicious of in a diff.

## Testing it locally

Write a file matching the shape above next to any page in `content/` and build.
It will render as _uncommitted_ until it is actually committed, at which point
the author, picture and date all appear from the commit.
