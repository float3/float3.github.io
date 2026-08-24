//! The two halves of comment moderation that run in CI.
//!
//! `comment-from-issue` turns an issue the compose box built into a comment
//! file, ready for the workflow to commit and open a pull request for.
//! `check-comment-changes` refuses a pull request that touches somebody else's
//! comment. They are the same rule applied at the two doors a comment can
//! arrive by, and both live here because both read input written by strangers
//! and neither should be trusting more of it than it has checked.
//!
//! Everything either one needs arrives in the environment, so the commands take
//! no arguments; the workflows in `.github/workflows/comment*.yaml` are the only
//! callers.

use crate::{Result, Site, SiteError};
use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The marker the compose box writes into an issue body.
///
/// The workflow keys off this rather than off a label, because a label set
/// through `?labels=` in a prefilled issue URL is silently dropped for anyone
/// without triage permission on the repository — which is everyone this feature
/// exists for.
const ISSUE_MARKER: &str = "hilll.dev:comment";

/// Long enough for any comment worth reading, short enough to bound the damage.
const MAX_BODY: usize = 64 * 1024;

/// A refusal, phrased for whoever will read it on the issue or pull request.
///
/// Separate from the ordinary error type so the workflow can tell "this input
/// was not acceptable", which it reports back and closes, from "this command
/// broke", which is mine to fix.
#[derive(Debug)]
pub(crate) struct Rejected(pub String);

impl std::fmt::Display for Rejected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Rejected {}

fn reject<T>(message: impl Into<String>) -> Result<T> {
    Err(Box::new(Rejected(message.into())))
}

// ---------------------------------------------------------------------------
// Shared shapes

fn is_comment_file(name: &str) -> bool {
    let Some((stem, rest)) = name.split_once(".comment.") else {
        return false;
    };
    let Some(id) = rest.strip_suffix(".md") else {
        return false;
    };
    !stem.is_empty() && is_id(id)
}

fn is_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 32
        && id
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
}

/// A page path a comment may attach to: no `..`, nothing absolute, ending `.md`.
fn is_page_path(path: &str) -> bool {
    !path.is_empty()
        && path.ends_with(".md")
        && !path.split('/').any(|segment| segment == "..")
        && !path.starts_with('/')
        && !path.contains(':')
        && path
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '/' | '-'))
}

/// Whichever of a frontmatter's scalars is wanted, unquoted.
///
/// Deliberately not a YAML parse. This reads single fields out of frontmatter
/// that may have been written by the person being checked, and the less of
/// their input that gets interpreted, the smaller the surface.
fn frontmatter_field(source: &str, field: &str) -> Option<String> {
    let body = source
        .strip_prefix("---\n")
        .or(source.strip_prefix("---\r\n"))?;
    let end = body.find("\n---")?;
    let prefix = format!("{field}:");

    for line in body[..end].lines() {
        let Some(value) = line.strip_prefix(&prefix) else {
            continue;
        };
        let value = value.trim();
        if value.is_empty() {
            return None;
        }
        // Quoted values are JSON strings, which is what the compose box and
        // this module both write; a hand-written file might use a bare scalar.
        return match serde_json::from_str::<String>(value) {
            Ok(text) if !text.is_empty() => Some(text),
            Ok(_) => None,
            Err(_) => Some(value.to_string()),
        };
    }

    None
}

fn scalar(value: &str) -> String {
    Value::String(value.to_string()).to_string()
}

// ---------------------------------------------------------------------------
// comment-from-issue

/// One submission against a comment: the first, and then each edit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Revision {
    pub date: String,
    pub issue: Option<u64>,
    pub edited: bool,
}

/// A comment file, as this module reads and writes one.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Comment {
    pub parent: String,
    pub date: String,
    pub author: Option<String>,
    pub author_id: Option<u64>,
    pub reply_to: Option<String>,
    pub quote: Option<String>,
    pub quote_heading: Option<String>,
    pub history: Vec<Revision>,
    pub body: String,
}

impl Comment {
    pub(crate) fn render(&self) -> String {
        let mut out = String::from("---\n");
        out.push_str(&format!("parent: {}\n", scalar(&self.parent)));
        out.push_str(&format!("date: {}\n", scalar(&self.date)));

        if let Some(author) = &self.author {
            out.push_str(&format!("author: {}\n", scalar(author)));
        }
        if let Some(id) = self.author_id {
            out.push_str(&format!("authorId: {id}\n"));
        }
        for (field, value) in [
            ("replyTo", &self.reply_to),
            ("quote", &self.quote),
            ("quoteHeading", &self.quote_heading),
        ] {
            if let Some(value) = value {
                out.push_str(&format!("{field}: {}\n", scalar(value)));
            }
        }

        out.push_str("history:\n");
        for revision in &self.history {
            out.push_str(&format!("  - date: {}\n", scalar(&revision.date)));
            if let Some(issue) = revision.issue {
                out.push_str(&format!("    issue: {issue}\n"));
            }
            if revision.edited {
                out.push_str("    edited: true\n");
            }
        }

        out.push_str("---\n\n");
        out.push_str(self.body.trim());
        out.push('\n');
        out
    }
}

/// What the compose box put in the issue: the payload, and the comment's text.
pub(crate) struct Submission {
    pub payload: Value,
    pub body: String,
}

/// Splits the machine-readable half of an issue body from the comment itself.
///
/// The shape is fixed by line rather than by searching for the closing `-->`:
///
/// ```text
/// <!--hilll.dev:comment
/// {"parent":"blog/page.md"}
/// -->
///
/// the comment
/// ```
///
/// The payload is whatever sits on the line after the marker, because
/// serialised JSON never contains a raw newline. Delimiting it by the first
/// `-->` instead looks equivalent and is not: a comment quoting a passage that
/// contains an arrow puts `-->` *inside the payload*, and the split lands in
/// the middle of the JSON. The body may still contain as many as it likes.
pub(crate) fn parse_issue(issue_body: &str) -> Result<Submission> {
    let text = issue_body.replace("\r\n", "\n");
    let opening = format!("<!--{ISSUE_MARKER}");

    let Some(start) = text.find(&opening) else {
        return reject(format!("this issue carries no {ISSUE_MARKER} marker"));
    };
    let after_marker = start + opening.len();

    // Past the remainder of the marker's own line, onto the payload's.
    let Some(payload_start) = text[after_marker..]
        .find('\n')
        .map(|at| after_marker + at + 1)
    else {
        return reject("the comment marker is never closed");
    };
    let payload_end = text[payload_start..]
        .find('\n')
        .map(|at| payload_start + at)
        .unwrap_or(text.len());

    let payload: Value = match serde_json::from_str(text[payload_start..payload_end].trim()) {
        Ok(value) => value,
        Err(_) => return reject("the comment marker does not contain valid JSON"),
    };
    if !payload.is_object() {
        return reject("the comment payload is not an object");
    }

    let Some(close) = text[payload_end..].find("-->").map(|at| payload_end + at) else {
        return reject("the comment marker is never closed");
    };

    let body = text[close + 3..].trim().to_string();
    if body.is_empty() {
        return reject("the comment has no text");
    }
    if body.len() > MAX_BODY {
        return reject(format!("the comment is longer than {MAX_BODY} bytes"));
    }

    Ok(Submission { payload, body })
}

fn payload_string(payload: &Value, field: &str) -> Option<String> {
    let value = payload.get(field)?.as_str()?.trim();
    (!value.is_empty()).then(|| value.to_string())
}

/// Where a comment file goes, and what the payload asked for.
pub(crate) struct Resolved {
    pub parent: String,
    pub parent_path: PathBuf,
    pub reply_to: Option<String>,
    pub editing: Option<String>,
    pub quote: Option<String>,
    pub quote_heading: Option<String>,
}

pub(crate) fn resolve(payload: &Value, content_dir: &Path) -> Result<Resolved> {
    let Some(parent) = payload_string(payload, "parent") else {
        return reject("the comment names no page");
    };
    if !is_page_path(&parent) {
        return reject(format!("`{parent}` is not a page path"));
    }

    let parent_path = content_dir.join(&parent);
    if !parent_path.is_file() {
        return reject(format!("there is no page at `{parent}`"));
    }

    // The shape check above already refuses `..`, and this refuses the thing it
    // cannot see: a symlink inside `content` pointing somewhere else. Comparing
    // the resolved paths is the only containment check that survives one, and
    // both sides have to be resolved or the comparison is between two different
    // spellings of the same directory.
    let (Ok(root), Ok(resolved)) = (content_dir.canonicalize(), parent_path.canonicalize()) else {
        return reject(format!("`{parent}` cannot be resolved"));
    };
    if !resolved.starts_with(&root) {
        return reject(format!("`{parent}` resolves outside the content directory"));
    }

    for field in ["replyTo", "editing"] {
        if let Some(value) = payload_string(payload, field) {
            if !is_id(&value) {
                return reject(format!("`{field}` is not a comment id"));
            }
        }
    }

    Ok(Resolved {
        parent,
        parent_path,
        reply_to: payload_string(payload, "replyTo"),
        editing: payload_string(payload, "editing"),
        quote: payload_string(payload, "quote"),
        quote_heading: payload_string(payload, "quoteHeading"),
    })
}

/// The last check before anything is written to disk.
///
/// Everything upstream of this already constrains the path — the parent must be
/// a `.md` file inside `content`, the id is generated here or matched against
/// `is_id`, and the filename is built rather than taken. This exists anyway,
/// because all of that is reasoning about code, and the thing worth being sure
/// of is the single fact that a stranger opening an issue can only ever cause
/// one shape of file to be written: a `.comment.<id>.md` beside a page.
///
/// If this ever fires it is a bug in the code above, not bad input, and the
/// right response is to write nothing at all.
fn assert_writable(file: &Path, content_dir: &Path) -> Result<()> {
    let name = file
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();

    if !is_comment_file(&name) {
        return reject(format!(
            "refusing to write `{name}`, which is not a comment"
        ));
    }

    // The parent directory is what can be resolved; the file itself may not
    // exist yet.
    let (Ok(root), Some(Ok(directory))) = (
        content_dir.canonicalize(),
        file.parent().map(Path::canonicalize),
    ) else {
        return reject("refusing to write to a path that cannot be resolved");
    };
    if !directory.starts_with(&root) {
        return reject("refusing to write outside the content directory");
    }

    Ok(())
}

fn comment_file(parent_path: &Path, id: &str) -> PathBuf {
    let name = parent_path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    let stem = name.strip_suffix(".md").unwrap_or(&name);
    parent_path.with_file_name(format!("{stem}.comment.{id}.md"))
}

/// An id no comment on this page is already using.
///
/// Generated here rather than accepted from the payload, so a submission cannot
/// choose which file it lands on.
fn new_id(parent_path: &Path) -> Result<String> {
    for _ in 0..100 {
        let id = random_id();
        if !comment_file(parent_path, &id).exists() {
            return Ok(id);
        }
    }
    reject("could not find a free comment id")
}

/// Four bytes of hex from the OS.
///
/// Only has to be unique within one page's thread — the filename already
/// carries the page — and nothing about a comment id is a secret.
fn random_id() -> String {
    let mut bytes = [0u8; 4];
    getrandom(&mut bytes);
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn getrandom(bytes: &mut [u8; 4]) {
    // No `rand` dependency for four bytes: the clock and the process id are
    // mixed only to keep two runs in the same second apart, and collisions are
    // checked against the directory anyway.
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_nanos())
        .unwrap_or(0);
    let seed = (now as u64) ^ ((std::process::id() as u64) << 32);
    // SplitMix64, which is short enough to read and good enough for a filename.
    let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    bytes.copy_from_slice(&(z as u32).to_be_bytes());
}

/// Reads back a comment this module wrote, for the edit path.
fn read_comment(file: &Path) -> Comment {
    let source = fs::read_to_string(file).unwrap_or_default();
    let mut comment = Comment {
        parent: frontmatter_field(&source, "parent").unwrap_or_default(),
        date: frontmatter_field(&source, "date").unwrap_or_default(),
        author: frontmatter_field(&source, "author"),
        author_id: frontmatter_field(&source, "authorId").and_then(|id| id.parse().ok()),
        reply_to: frontmatter_field(&source, "replyTo"),
        quote: frontmatter_field(&source, "quote"),
        quote_heading: frontmatter_field(&source, "quoteHeading"),
        history: Vec::new(),
        body: String::new(),
    };

    // The history is a list, so it needs its own small walk rather than the
    // single-field reader above.
    let mut in_history = false;
    for line in source.lines() {
        if let Some(rest) = line.strip_prefix("  - date:") {
            if in_history {
                if let Ok(date) = serde_json::from_str::<String>(rest.trim()) {
                    comment.history.push(Revision {
                        date,
                        issue: None,
                        edited: false,
                    });
                }
            }
            continue;
        }
        if let Some(rest) = line.strip_prefix("    issue:") {
            if let Some(last) = comment.history.last_mut() {
                last.issue = rest.trim().parse().ok();
            }
            continue;
        }
        if line.starts_with("    edited:") {
            if let Some(last) = comment.history.last_mut() {
                last.edited = true;
            }
            continue;
        }
        in_history = line == "history:";
        if line == "---" && !comment.history.is_empty() {
            break;
        }
    }

    comment
}

/// What the command did, for the workflow's outputs.
pub(crate) struct Applied {
    pub action: &'static str,
    pub id: String,
    pub file: PathBuf,
    pub login: String,
    pub parent: String,
}

/// The whole job: validate, write, and report.
///
/// `issue` is the shape GitHub sends. The author is read from `issue.user` and
/// nowhere else, so what the payload claims about authorship is never consulted.
pub(crate) fn apply(issue: &Value, content_dir: &Path, now: &str) -> Result<Applied> {
    let body = issue
        .get("body")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let submission = parse_issue(body)?;
    let resolved = resolve(&submission.payload, content_dir)?;

    let Some(login) = issue
        .get("user")
        .and_then(|user| user.get("login"))
        .and_then(Value::as_str)
        .filter(|login| !login.is_empty())
    else {
        return reject("the issue has no author");
    };
    let issue_number = issue.get("number").and_then(Value::as_u64);
    let author_id = issue
        .get("user")
        .and_then(|user| user.get("id"))
        .and_then(Value::as_u64);

    if let Some(editing) = &resolved.editing {
        let file = comment_file(&resolved.parent_path, editing);
        if !file.is_file() {
            return reject(format!("there is no comment `{editing}` on that page"));
        }

        let existing = read_comment(&file);
        // The one rule that makes an edit button safe to offer to everyone: a
        // comment can only be rewritten by the account that wrote it.
        match &existing.author {
            None => {
                return reject("that comment records no author, so it cannot be claimed now");
            }
            Some(author) if author != login => {
                return reject(format!("that comment belongs to @{author}"));
            }
            Some(_) => {}
        }

        let mut history = existing.history;
        history.push(Revision {
            date: now.to_string(),
            issue: issue_number,
            edited: true,
        });

        let rewritten = Comment {
            parent: resolved.parent.clone(),
            // Unchanged: an edit revises the text, not when the comment was made.
            date: if existing.date.is_empty() {
                now.to_string()
            } else {
                existing.date
            },
            author: Some(login.to_string()),
            author_id: existing.author_id.or(author_id),
            reply_to: existing.reply_to,
            quote: existing.quote,
            quote_heading: existing.quote_heading,
            history,
            body: submission.body,
        };
        assert_writable(&file, content_dir)?;
        fs::write(&file, rewritten.render())?;

        return Ok(Applied {
            action: "edited",
            id: editing.clone(),
            file,
            login: login.to_string(),
            parent: resolved.parent,
        });
    }

    let id = new_id(&resolved.parent_path)?;
    let file = comment_file(&resolved.parent_path, &id);
    let comment = Comment {
        parent: resolved.parent.clone(),
        date: now.to_string(),
        author: Some(login.to_string()),
        author_id,
        reply_to: resolved.reply_to,
        quote: resolved.quote,
        quote_heading: resolved.quote_heading,
        history: vec![Revision {
            date: now.to_string(),
            issue: issue_number,
            edited: false,
        }],
        body: submission.body,
    };
    assert_writable(&file, content_dir)?;
    fs::write(&file, comment.render())?;

    Ok(Applied {
        action: "added",
        id,
        file,
        login: login.to_string(),
        parent: resolved.parent,
    })
}

/// ISO 8601 in UTC, to the millisecond, which is the shape the compose box and
/// every date already in a comment file use.
fn now_iso8601() -> String {
    let since = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let millis = since.as_millis() as u64;
    let (secs, sub) = (millis / 1000, millis % 1000);

    let days = secs / 86_400;
    let time = secs % 86_400;
    let (year, month, day) = civil_from_days(days as i64);

    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{sub:03}Z",
        time / 3600,
        (time % 3600) / 60,
        time % 60,
    )
}

/// Howard Hinnant's days-to-civil, which is the whole of the calendar maths a
/// timestamp needs and shorter than taking on a date library for it.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

pub(crate) fn from_issue(site: &Site) -> Result<()> {
    let issue: Value = serde_json::from_str(&env::var("ISSUE_JSON").unwrap_or_default())
        .map_err(|error| SiteError::new(format!("ISSUE_JSON is not valid JSON: {error}")))?;
    let content_dir = site
        .root
        .join(env::var("CONTENT_DIR").unwrap_or_else(|_| "content".into()));

    // A refusal is reported back on the issue by the workflow, so it has to
    // reach it as an output before this exits.
    let applied = match apply(&issue, &content_dir, &now_iso8601()) {
        Ok(applied) => applied,
        Err(error) => {
            if error.downcast_ref::<Rejected>().is_some() {
                write_output(&format!("rejected={error}"))?;
            }
            return Err(error);
        }
    };

    let relative = applied
        .file
        .strip_prefix(&site.root)
        .unwrap_or(&applied.file)
        .to_string_lossy()
        .replace('\\', "/");

    let outputs = [
        format!("action={}", applied.action),
        format!("id={}", applied.id),
        format!("file={relative}"),
        format!("login={}", applied.login),
        format!("parent={}", applied.parent),
    ]
    .join("\n");

    println!("{outputs}");
    write_output(&outputs)
}

fn write_output(outputs: &str) -> Result<()> {
    if let Ok(path) = env::var("GITHUB_OUTPUT") {
        use std::io::Write;
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;
        writeln!(file, "{outputs}")?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// check-comment-changes

/// Branches the comment workflow pushes, which arrive already checked.
const WORKFLOW_ACTOR: &str = "github-actions[bot]";
/// Associations GitHub gives to people who can push here.
const TRUSTED: [&str; 3] = ["OWNER", "MEMBER", "COLLABORATOR"];

fn is_workflow_branch(head_ref: &str) -> bool {
    head_ref
        .strip_prefix("comment/")
        .is_some_and(|number| !number.is_empty() && number.chars().all(|ch| ch.is_ascii_digit()))
}

/// One changed path, and what happened to it.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Change {
    pub status: char,
    pub path: String,
}

/// `git diff --name-status base...head`, keeping only the comment files.
pub(crate) fn changed_comments(diff: &str) -> Vec<Change> {
    let mut changes = Vec::new();

    for line in diff.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() < 2 {
            continue;
        }

        let status = fields[0].chars().next().unwrap_or('?');
        let keep = |path: &str| is_comment_file(path.rsplit('/').next().unwrap_or(path));

        // A rename shows as "R100\told\tnew", and counts as touching both ends.
        if status == 'R' && fields.len() >= 3 {
            if keep(fields[1]) {
                changes.push(Change {
                    status: 'D',
                    path: fields[1].to_string(),
                });
            }
            if keep(fields[2]) {
                changes.push(Change {
                    status: 'A',
                    path: fields[2].to_string(),
                });
            }
            continue;
        }

        if keep(fields[1]) {
            changes.push(Change {
                status,
                path: fields[1].to_string(),
            });
        }
    }

    changes
}

/// What a pull request is, for the purposes of being allowed to touch comments.
pub(crate) struct PullRequest<'a> {
    pub actor: &'a str,
    pub association: &'a str,
    pub head_ref: &'a str,
}

/// Checks every comment file a pull request touches.
///
/// Returns all the refusals rather than stopping at the first, so a pull
/// request that gets several things wrong is told about all of them at once.
pub(crate) fn check_changes(
    pull_request: &PullRequest,
    changes: &[Change],
    // `at(ref, path)` is the file's contents in that tree, or None if absent.
    at: impl Fn(&str, &str) -> Option<String>,
    base: &str,
    head: &str,
) -> Vec<String> {
    // Moderating means being able to edit and delete comments that are not
    // yours, and adding one on someone's behalf from an email means writing
    // their address into the author field.
    if TRUSTED.contains(&pull_request.association.to_ascii_uppercase().as_str()) {
        return Vec::new();
    }
    // The issue that produced this was checked when it was opened, and nothing
    // but this repository's workflow can push such a branch under the bot's
    // name. Both halves are required, so neither alone gets past it.
    if pull_request.actor == WORKFLOW_ACTOR && is_workflow_branch(pull_request.head_ref) {
        return Vec::new();
    }

    let mut refusals = Vec::new();

    for change in changes {
        let path = &change.path;

        if change.status == 'A' {
            // A new comment may say nothing about who wrote it — the commit
            // answers that — but it may not say somebody else.
            let claimed = at(head, path).and_then(|source| frontmatter_field(&source, "author"));
            if let Some(claimed) = claimed {
                if claimed != pull_request.actor {
                    refusals.push(format!(
                        "`{path}` is new but claims to be by `{claimed}`. A comment you add can only be your own."
                    ));
                }
            }
            continue;
        }

        let Some(owner) = at(base, path).and_then(|source| frontmatter_field(&source, "author"))
        else {
            refusals.push(format!(
                "`{path}` records no author, so there is no way to establish that it is yours. Only someone with write access can change it."
            ));
            continue;
        };

        if owner != pull_request.actor {
            refusals.push(format!("`{path}` belongs to `{owner}`, not to you."));
            continue;
        }

        // Editing your own comment is fine; signing it over to someone else
        // while you are in there is not.
        if change.status != 'D' {
            let claimed = at(head, path).and_then(|source| frontmatter_field(&source, "author"));
            if claimed.as_deref() != Some(owner.as_str()) {
                refusals.push(format!(
                    "`{path}` changes its author from `{owner}` to `{}`.",
                    claimed.unwrap_or_else(|| "nothing".into())
                ));
            }
        }
    }

    refusals
}

fn git(site: &Site, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(&site.root)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
}

pub(crate) fn check_pull_request(site: &Site) -> Result<()> {
    let base = env::var("BASE_SHA").unwrap_or_else(|_| "origin/master".into());
    let head = env::var("HEAD_SHA").unwrap_or_else(|_| "HEAD".into());
    let actor = env::var("ACTOR").unwrap_or_default();
    let association = env::var("ASSOCIATION").unwrap_or_default();
    let head_ref = env::var("HEAD_REF").unwrap_or_default();

    let diff = git(
        site,
        &["diff", "--name-status", "-M", &format!("{base}...{head}")],
    )
    .ok_or_else(|| SiteError::new(format!("could not diff {base}...{head}")))?;

    let refusals = check_changes(
        &PullRequest {
            actor: &actor,
            association: &association,
            head_ref: &head_ref,
        },
        &changed_comments(&diff),
        |reference, path| git(site, &["show", &format!("{reference}:{path}")]),
        &base,
        &head,
    );

    if refusals.is_empty() {
        println!("comment changes are the author's own");
        return Ok(());
    }

    let message = refusals
        .iter()
        .map(|line| format!("- {line}"))
        .collect::<Vec<_>>()
        .join("\n");
    eprintln!("{message}");
    // Multi-line outputs need a delimiter GitHub can find. The refusals are
    // built from paths and logins, never from free text.
    write_output(&format!("refusals<<REFUSALS\n{message}\nREFUSALS"))?;

    Err(Box::new(Rejected(message)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn issue_body(payload: &str, body: &str) -> String {
        format!("<!--{ISSUE_MARKER}\n{payload}\n-->\n\n{body}\n")
    }

    fn issue(payload: &str, body: &str, login: &str, id: u64) -> Value {
        serde_json::json!({
            "number": 7,
            "user": { "login": login, "id": id },
            "body": issue_body(payload, body),
        })
    }

    struct Fixture {
        dir: PathBuf,
    }

    impl Fixture {
        fn new(name: &str) -> Self {
            let dir = env::temp_dir().join(format!("site-comments-{name}-{}", std::process::id()));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(dir.join("blog")).unwrap();
            fs::write(dir.join("blog/page.md"), "---\ntitle: p\n---\n\nbody\n").unwrap();
            Self { dir }
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }

    const NOW: &str = "2026-01-02T10:00:00.000Z";

    // -- parsing ------------------------------------------------------------

    #[test]
    fn splits_the_payload_from_the_text() {
        let parsed = parse_issue(&issue_body(r#"{"parent":"blog/page.md"}"#, "hello")).unwrap();
        assert_eq!(parsed.payload["parent"], "blog/page.md");
        assert_eq!(parsed.body, "hello");
    }

    #[test]
    fn a_body_containing_a_comment_close_does_not_truncate_the_text() {
        let parsed = parse_issue(&issue_body(r#"{"parent":"blog/page.md"}"#, "a --> b")).unwrap();
        assert_eq!(parsed.body, "a --> b");
    }

    #[test]
    fn a_payload_containing_an_arrow_is_not_cut_in_half() {
        // Quoting a passage with `-->` in it is the case that searching for the
        // first close gets wrong, because the arrow lands inside the JSON.
        let parsed = parse_issue(&issue_body(
            r#"{"parent":"blog/page.md","quote":"a --> b"}"#,
            "commenting on an arrow",
        ))
        .unwrap();

        assert_eq!(parsed.payload["quote"], "a --> b");
        assert_eq!(parsed.body, "commenting on an arrow");
    }

    #[test]
    fn refuses_bodies_that_are_not_submissions() {
        assert!(parse_issue("just a normal issue").is_err());
        assert!(parse_issue(&format!("<!--{ISSUE_MARKER}\nnot json\n-->\n\nhi")).is_err());
        assert!(parse_issue(&issue_body(r#"{"parent":"blog/page.md"}"#, "   ")).is_err());
    }

    #[test]
    fn reads_one_field_out_of_frontmatter() {
        let source = "---\nparent: \"a.md\"\nauthor: \"alice\"\n---\n\nbody\n";
        assert_eq!(
            frontmatter_field(source, "author").as_deref(),
            Some("alice")
        );
        assert_eq!(frontmatter_field(source, "quote"), None);
        // Bare scalars are legal YAML, and a hand-written file might use one.
        assert_eq!(
            frontmatter_field("---\nauthor: alice\n---\n\nbody", "author").as_deref(),
            Some("alice")
        );
        // Never out of the comment's own text.
        assert_eq!(
            frontmatter_field("---\ndate: \"x\"\n---\n\nauthor: alice\n", "author"),
            None
        );
    }

    // -- adding -------------------------------------------------------------

    #[test]
    fn writes_a_file_authored_by_the_issue_opener() {
        let fixture = Fixture::new("adds");
        let applied = apply(
            &issue(r#"{"parent":"blog/page.md"}"#, "first!", "alice", 1),
            &fixture.dir,
            NOW,
        )
        .unwrap();

        assert_eq!(applied.action, "added");
        let written = fs::read_to_string(&applied.file).unwrap();
        assert!(written.contains("author: \"alice\""));
        assert!(written.contains("authorId: 1"));
        assert!(written.contains("issue: 7"));
        assert!(written.trim_end().ends_with("first!"));
    }

    #[test]
    fn takes_the_author_from_the_issue_never_the_payload() {
        let fixture = Fixture::new("spoof");
        let applied = apply(
            &issue(
                r#"{"parent":"blog/page.md","author":"mallory","authorId":99}"#,
                "hi",
                "alice",
                1,
            ),
            &fixture.dir,
            NOW,
        )
        .unwrap();

        let written = fs::read_to_string(&applied.file).unwrap();
        assert!(written.contains("author: \"alice\""));
        assert!(!written.contains("mallory"));
    }

    #[test]
    fn refuses_paths_that_are_not_pages_here() {
        let fixture = Fixture::new("paths");
        for parent in [
            "../../etc/passwd.md",
            "/etc/passwd.md",
            "blog/nope.md",
            "blog/page.txt",
        ] {
            let payload = format!(r#"{{"parent":"{parent}"}}"#);
            assert!(
                apply(&issue(&payload, "hi", "alice", 1), &fixture.dir, NOW).is_err(),
                "accepted {parent}"
            );
        }
    }

    #[test]
    fn refuses_a_reply_id_that_is_not_an_id() {
        let fixture = Fixture::new("replyid");
        assert!(apply(
            &issue(
                r#"{"parent":"blog/page.md","replyTo":"../../x"}"#,
                "hi",
                "alice",
                1
            ),
            &fixture.dir,
            NOW
        )
        .is_err());
    }

    // -- editing ------------------------------------------------------------

    fn add(fixture: &Fixture, login: &str) -> Applied {
        apply(
            &issue(r#"{"parent":"blog/page.md"}"#, "original", login, 1),
            &fixture.dir,
            NOW,
        )
        .unwrap()
    }

    #[test]
    fn the_author_may_rewrite_their_own_comment() {
        let fixture = Fixture::new("edit");
        let first = add(&fixture, "alice");

        let payload = format!(r#"{{"parent":"blog/page.md","editing":"{}"}}"#, first.id);
        let edited = apply(
            &issue(&payload, "revised", "alice", 1),
            &fixture.dir,
            "2026-02-02T10:00:00.000Z",
        )
        .unwrap();

        assert_eq!(edited.action, "edited");
        assert_eq!(edited.file, first.file);

        let written = fs::read_to_string(&first.file).unwrap();
        assert!(written.contains("revised"));
        assert!(!written.contains("original"));
        assert_eq!(written.matches("  - date:").count(), 2);
        assert!(written.contains("edited: true"));
        // An edit revises the text, not when the comment was made.
        assert!(written.contains(&format!("date: \"{NOW}\"")));
    }

    #[test]
    fn nobody_else_may_rewrite_it() {
        let fixture = Fixture::new("hijack");
        let first = add(&fixture, "alice");

        let payload = format!(r#"{{"parent":"blog/page.md","editing":"{}"}}"#, first.id);
        assert!(apply(
            &issue(&payload, "hijacked", "mallory", 2),
            &fixture.dir,
            NOW
        )
        .is_err());
        assert!(fs::read_to_string(&first.file)
            .unwrap()
            .contains("original"));
    }

    #[test]
    fn a_comment_with_no_author_cannot_be_claimed() {
        let fixture = Fixture::new("unowned");
        let file = fixture.dir.join("blog/page.comment.deadbeef.md");
        fs::write(
            &file,
            "---\nparent: \"blog/page.md\"\ndate: \"2026-01-01T00:00:00.000Z\"\n---\n\nold\n",
        )
        .unwrap();

        assert!(apply(
            &issue(
                r#"{"parent":"blog/page.md","editing":"deadbeef"}"#,
                "mine now",
                "m",
                3
            ),
            &fixture.dir,
            NOW
        )
        .is_err());
    }

    #[test]
    fn refuses_to_edit_a_comment_that_is_not_there() {
        let fixture = Fixture::new("missing");
        assert!(apply(
            &issue(
                r#"{"parent":"blog/page.md","editing":"aaaaaaaa"}"#,
                "hi",
                "alice",
                1
            ),
            &fixture.dir,
            NOW
        )
        .is_err());
    }

    // -- the pull-request guard --------------------------------------------

    fn tree(entries: &[(&str, &str, &str)]) -> impl Fn(&str, &str) -> Option<String> {
        let mut map: HashMap<(String, String), String> = HashMap::new();
        for (reference, path, author) in entries {
            let source = if author.is_empty() {
                "---\nparent: \"_index.md\"\n---\n\nhi\n".to_string()
            } else {
                format!("---\nparent: \"_index.md\"\nauthor: \"{author}\"\n---\n\nhi\n")
            };
            map.insert(((*reference).to_string(), (*path).to_string()), source);
        }
        move |reference: &str, path: &str| map.get(&(reference.into(), path.into())).cloned()
    }

    const ALICES: &str = "content/_index.comment.aaaa1111.md";

    fn stranger() -> PullRequest<'static> {
        PullRequest {
            actor: "mallory",
            association: "NONE",
            head_ref: "patch-1",
        }
    }

    #[test]
    fn picks_the_comment_files_out_of_a_diff() {
        let diff = "M\tcontent/_index.md\nA\tcontent/a.comment.bb11.md\nR100\tcontent/a.comment.cc22.md\tcontent/a.comment.dd33.md\n";
        let changes = changed_comments(diff);
        assert_eq!(changes.len(), 3);
        assert_eq!(changes[0].status, 'A');
        // A rename is a deletion of one and an addition of the other.
        assert_eq!(changes[1].status, 'D');
        assert_eq!(changes[2].status, 'A');
    }

    #[test]
    fn a_stranger_may_add_their_own_comment() {
        let changes = vec![Change {
            status: 'A',
            path: "content/_index.comment.cccc.md".into(),
        }];
        let at = tree(&[("head", "content/_index.comment.cccc.md", "mallory")]);
        assert!(check_changes(&stranger(), &changes, at, "base", "head").is_empty());
    }

    #[test]
    fn a_stranger_may_not_add_one_in_somebody_elses_name() {
        let changes = vec![Change {
            status: 'A',
            path: "content/_index.comment.cccc.md".into(),
        }];
        let at = tree(&[("head", "content/_index.comment.cccc.md", "alice")]);
        let refusals = check_changes(&stranger(), &changes, at, "base", "head");
        assert_eq!(refusals.len(), 1);
        assert!(refusals[0].contains("claims to be by `alice`"));
    }

    #[test]
    fn a_stranger_may_not_edit_or_delete_somebody_elses() {
        for status in ['M', 'D'] {
            let changes = vec![Change {
                status,
                path: ALICES.into(),
            }];
            let at = tree(&[("base", ALICES, "alice"), ("head", ALICES, "alice")]);
            let refusals = check_changes(&stranger(), &changes, at, "base", "head");
            assert_eq!(refusals.len(), 1, "status {status}");
            assert!(refusals[0].contains("belongs to `alice`"));
        }
    }

    #[test]
    fn a_comment_with_no_recorded_author_is_not_a_strangers_to_touch() {
        let changes = vec![Change {
            status: 'M',
            path: ALICES.into(),
        }];
        let at = tree(&[("base", ALICES, ""), ("head", ALICES, "")]);
        let refusals = check_changes(&stranger(), &changes, at, "base", "head");
        assert!(refusals[0].contains("records no author"));
    }

    #[test]
    fn the_author_may_edit_and_delete_their_own() {
        let mine = PullRequest {
            actor: "alice",
            association: "NONE",
            head_ref: "patch-1",
        };
        for status in ['M', 'D'] {
            let changes = vec![Change {
                status,
                path: ALICES.into(),
            }];
            let at = tree(&[("base", ALICES, "alice"), ("head", ALICES, "alice")]);
            assert!(check_changes(&mine, &changes, at, "base", "head").is_empty());
        }
    }

    #[test]
    fn the_author_may_not_sign_it_over_on_the_way_past() {
        let mine = PullRequest {
            actor: "alice",
            association: "NONE",
            head_ref: "patch-1",
        };
        let changes = vec![Change {
            status: 'M',
            path: ALICES.into(),
        }];
        let at = tree(&[("base", ALICES, "alice"), ("head", ALICES, "mallory")]);
        let refusals = check_changes(&mine, &changes, at, "base", "head");
        assert!(refusals[0].contains("changes its author"));
    }

    #[test]
    fn write_access_passes_unchecked() {
        let changes = vec![Change {
            status: 'M',
            path: ALICES.into(),
        }];
        for association in ["OWNER", "collaborator", "Member"] {
            let owner = PullRequest {
                actor: "hill",
                association,
                head_ref: "patch-1",
            };
            let at = tree(&[("base", ALICES, "alice"), ("head", ALICES, "alice")]);
            assert!(check_changes(&owner, &changes, at, "base", "head").is_empty());
        }
    }

    #[test]
    fn the_workflows_own_branch_is_taken_as_already_checked() {
        let changes = vec![Change {
            status: 'A',
            path: "content/_index.comment.cccc.md".into(),
        }];
        let at = tree(&[("head", "content/_index.comment.cccc.md", "someone-else")]);
        let bot = PullRequest {
            actor: WORKFLOW_ACTOR,
            association: "NONE",
            head_ref: "comment/42",
        };
        assert!(check_changes(&bot, &changes, at, "base", "head").is_empty());
    }

    #[test]
    fn but_only_that_actor_on_that_branch() {
        let changes = vec![Change {
            status: 'A',
            path: "content/_index.comment.cccc.md".into(),
        }];

        // The bot, but somewhere else.
        let at = tree(&[("head", "content/_index.comment.cccc.md", "someone-else")]);
        let elsewhere = PullRequest {
            actor: WORKFLOW_ACTOR,
            association: "NONE",
            head_ref: "patch-1",
        };
        assert_eq!(
            check_changes(&elsewhere, &changes, at, "base", "head").len(),
            1
        );

        // Somebody else, on the bot's branch.
        let at = tree(&[("head", "content/_index.comment.cccc.md", "someone-else")]);
        let pretender = PullRequest {
            actor: "mallory",
            association: "NONE",
            head_ref: "comment/42",
        };
        assert_eq!(
            check_changes(&pretender, &changes, at, "base", "head").len(),
            1
        );
    }

    // -- odds and ends ------------------------------------------------------

    #[test]
    fn only_ever_writes_a_comment_beside_a_page() {
        let fixture = Fixture::new("writable");
        let page = fixture.dir.join("blog/page.md");

        assert!(assert_writable(&comment_file(&page, "aaaa1111"), &fixture.dir).is_ok());

        // The page itself, and anything else that is not a comment.
        assert!(assert_writable(&page, &fixture.dir).is_err());
        assert!(assert_writable(&fixture.dir.join("blog/notes.md"), &fixture.dir).is_err());
        assert!(assert_writable(&fixture.dir.join("blog/x.comment..md"), &fixture.dir).is_err());

        // Anywhere outside the content directory, however it is spelled.
        let outside = fixture.dir.join("../elsewhere.comment.aaaa1111.md");
        assert!(assert_writable(&outside, &fixture.dir).is_err());
    }

    #[test]
    fn refuses_a_page_reached_through_a_symlink_out_of_content() {
        // The shape check cannot see a link, only the resolved path can.
        let fixture = Fixture::new("symlink");
        let outside = env::temp_dir().join(format!("site-outside-{}", std::process::id()));
        fs::create_dir_all(&outside).unwrap();
        fs::write(
            outside.join("secret.md"),
            "---
title: s
---

x
",
        )
        .unwrap();

        let link = fixture.dir.join("escape");
        #[cfg(unix)]
        let linked = std::os::unix::fs::symlink(&outside, &link).is_ok();
        #[cfg(windows)]
        let linked = std::os::windows::fs::symlink_dir(&outside, &link).is_ok();

        // Making a link needs a privilege that CI may not have; when it cannot
        // be made there is nothing here to test.
        if linked {
            let payload = serde_json::json!({ "parent": "escape/secret.md" });
            assert!(resolve(&payload, &fixture.dir).is_err());
        }

        let _ = fs::remove_dir_all(&outside);
    }

    #[test]
    fn recognises_comment_filenames() {
        assert!(is_comment_file("page.comment.aaaa1111.md"));
        assert!(!is_comment_file("page.md"));
        assert!(!is_comment_file("page.comment..md"));
        assert!(!is_comment_file(".comment.aa.md"));
        assert!(!is_comment_file("page.comment.aa.txt"));
    }

    #[test]
    fn formats_a_timestamp_the_way_the_files_do() {
        let stamp = now_iso8601();
        assert_eq!(stamp.len(), 24, "{stamp}");
        assert!(stamp.ends_with('Z'));
        assert_eq!(&stamp[4..5], "-");
        assert_eq!(&stamp[10..11], "T");
    }

    #[test]
    fn converts_days_to_a_calendar_date() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(19_000), (2022, 1, 8));
        // A leap day, which is where naive arithmetic goes wrong.
        assert_eq!(civil_from_days(19_782), (2024, 2, 29));
    }
}
