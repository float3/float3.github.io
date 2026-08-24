# comments

Comments as files in the repository, contributed as pull requests.

There is no server, no database and no third-party widget. A reader writes a
comment in the page's compose box and picks one of three ways to send it.
Whichever they pick, the comment arrives as a pull request, and merging that
pull request is what publishes it — moderation is the merge button and nothing
else.

| route               | what happens                                                                                                                                                               |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **issue** (default) | Opens a prefilled GitHub issue. `.github/workflows/comment.yaml` writes the file, commits it **in the issue opener's name**, opens the pull request, and closes the issue. |
| **pull request**    | Opens GitHub's "create a new file" editor with the path and body filled in, forking the repository if the reader cannot push. They commit and propose it themselves.       |
| **email**           | A `mailto:` link. No GitHub account at all; the comment gets added by hand.                                                                                                |

The issue route is the default because it is the only one that ends with the
comment attributed to whoever wrote it without them having to do anything about
it. The other two are there because the first can fail for reasons the reader
cannot fix.

The first two need a GitHub account. In exchange they need nothing else: no name
to type, no picture to upload, no key to keep — the account is the identity.

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
author: "octocat"
authorId: 12345
replyTo: "aaaa1111"
quote: "Neurons fire; muscles move."
quoteHeading: "where-do-our-choices-come-from"
history:
  - date: "2026-01-02T10:00:00.000Z"
    issue: 148
  - date: "2026-01-09T14:20:00.000Z"
    issue: 151
    edited: true
---

The body, in markdown.
```

Only `parent` and a non-empty body are required. The body is markdown, and may
contain HTML and `<script>` — see below for where each of those ends up. Every value is written as a
double-quoted YAML scalar, which accepts exactly the JSON string escapes, so the
compose box can emit any of them with `JSON.stringify`.

| field          | meaning                                                                             |
| -------------- | ----------------------------------------------------------------------------------- |
| `parent`       | content-relative path of the page.                                                  |
| `replyTo`      | id of the comment being answered. Threading renders three levels deep.              |
| `quote`        | text quoted verbatim from the page.                                                 |
| `quoteHeading` | slug of the heading it sat under, used to disambiguate a phrase that appears twice. |
| `date`         | when the comment was first made. An edit never moves it.                            |
| `author`       | who wrote it: a GitHub login, or an email address for one that arrived as mail.     |
| `authorId`     | numeric account id, which is what the avatar URL is built from.                     |
| `history`      | one entry per submission, oldest first, each naming the issue it arrived as.        |

## Authorship

`author` is one of two things: a **GitHub login**, for anything that came
through the issue or pull-request routes, or an **email address**, for a comment
that arrived as mail and was added by hand. They are told apart by the `@`,
which a login cannot contain.

An email author renders as the part before the `@`, and links nowhere. The whole
address stays in the file — it is the only thing a later edit can be checked
against — but it is not printed into the page, because the file being public and
the address sitting on a crawled page are not the same exposure. Put one in only
when whoever sent it would be content to see it in the repository.

For the GitHub case there are two sources, agreeing by construction. The
workflow writes the issue opener's login into the file **and** makes the commit
in their name, using GitHub's private-email form
`12345+octocat@users.noreply.github.com`. The frontmatter is what renders; the
commit is the corroboration.

Neither is ever read from the issue payload: `site comment-from-issue` takes
the author from `github.event.issue.user` and nowhere else, so an issue
whose body claims to be from someone else is still attributed to whoever opened
it. The numeric id is what the avatar URL is built from — a login can be changed
or reused, an id cannot.

A file with no `author` — one submitted through the pull-request route, or
written before the workflow existed — falls back to the commit that added it,
via `git log --diff-filter=A`. A file with neither renders as _uncommitted_,
which locally is the whole time one is being tested.

### A comment belongs to its author, at both doors

`site check-comment-changes` runs on every pull request that touches a comment
file and refuses one that touches somebody else's. For each changed file
it asks the same question the issue route asks:

| what the pull request does    | what has to hold                                             |
| ----------------------------- | ------------------------------------------------------------ |
| adds a comment                | it claims no author, or claims the pull request's author     |
| edits, deletes or renames one | the **base** version's `author` is the pull request's author |
| edits one                     | the author does not change on the way past                   |

A comment with no recorded `author` cannot be touched by a stranger at all —
there is nothing to establish that it is theirs.

Two exemptions, both deliberate. **Write access passes unchecked**, because
moderating means being able to edit and delete comments that are not yours, and
adding one on someone's behalf from an email means writing their address into
the `author` field. And the comment workflow's own `comment/<n>` branches pass,
because the issue that produced them was checked when it was opened, and nothing
but this repository's workflow can push one under the bot's name — the check is
scoped to that branch pattern _and_ that actor, so neither half alone gets past
it.

Avatars are hotlinked from `avatars.githubusercontent.com` with
`referrerpolicy="no-referrer"`. That does mean a reader's IP reaches GitHub when
a thread has comments in it.

## Editing, and version history

Every comment with a recorded author gets an **edit** button. It fills the
compose box with that comment's own source and submits as an issue carrying
`editing: <id>`; the workflow rewrites that file in place instead of adding one,
appends a revision to `history`, and leaves `date` alone.

The rule that makes this safe to offer to everyone lives in the workflow, not
the page: **a comment can only be rewritten by the account that wrote it.** The
browser has no idea who is signed in to GitHub and does not try to guess — the
button says whose comment it is, and the workflow refuses anything else. A
comment with no recorded author cannot be claimed by anyone.

The page shows the revision count and dates, each linking to the issue it
arrived as, plus a link to the file's commit history for the text of each
version. Old revisions are not duplicated into the file, because git already
holds every one of them.

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

## The workflow

`.github/workflows/comment.yaml` fires on `issues: opened` and keys off the
`hilll.dev:comment` marker in the body rather than off a label — `?labels=` in a
prefilled issue URL is silently dropped for anyone without triage permission,
which is everyone this feature exists for.

It runs `site comment-from-issue`, which is the whole of the validation.
Everything that script reads was written by a stranger, so nothing is trusted
further than it has been checked:

- `parent` must name a page that exists, and resolve inside `content/`;
- the comment id is generated there, never accepted from the payload;
- `replyTo` and `editing` have to look like ids;
- the author comes from the issue, never the payload;
- an edit has to come from the account that owns the comment.

Anything it refuses is said back on the issue, which is then closed. Nothing is
written in that case.

`.github/workflows/comment-guard.yaml` is the other half, on `pull_request`. It
uses `pull_request` rather than `pull_request_target` deliberately: it checks out
a fork's branch, so it runs with the read-only token and no access to secrets,
which is all it needs. Refusals go to the job summary.

### What a stranger's issue can reach

No secret is involved in any of this. Both workflows authenticate with
`${{ github.token }}` — the `GITHUB_TOKEN` GitHub mints for a single run and
throws away after it — so there is no stored credential to steal, and no
personal access token should ever be added for them.

The comment workflow needs `contents: write` to push its branch, so it is worth
being able to say exactly what an issue can cause it to change. Three separate
things constrain that, and they are checked in three different ways:

1. it checks out the **default branch**, never anything from the issue, so the
   only code that runs is the code in the repository;
2. the issue body reaches the program as `ISSUE_JSON` in the environment and is
   parsed as JSON, never interpolated into a shell command;
3. the path it writes is built rather than accepted — `assert_writable` refuses
   anything that is not a `.comment.<id>.md` inside `content/`, with symlinks
   resolved so a link out of `content` cannot be followed — and then the
   workflow independently checks the working tree afterwards, refusing unless
   exactly one file changed and it is that comment.

The third one is deliberate belt and braces: the first two are arguments about
code, and the last is a fact about the tree, checked after the fact.

Two things worth setting on the repository itself, which no workflow can do for
you: **branch protection on the default branch**, so that even a compromised run
cannot push to it directly, and **pinning the actions to commit SHAs** rather
than tags, so a moved tag cannot change what runs.

## Testing it locally

Both checks are subcommands of the `site` CLI, in
`tools/site/src/comments.rs`, and neither needs GitHub:

```sh
cargo test --locked --manifest-path tools/site/Cargo.toml comments
```

Neither does the thing it drives — `apply()` takes an issue-shaped object and a
content directory, so a thread can be built up locally by calling it directly
(see the test file for the shape), then `bun run quartz build`. Delete the
generated `content/**/*.comment.*.md` afterwards.

A file written by hand renders as _uncommitted_ until it is committed, at which
point the author and date come from the commit.
