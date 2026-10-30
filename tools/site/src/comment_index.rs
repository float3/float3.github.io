//! Every comment's metadata, for the Quartz comments plugin to render.
//!
//! `site comments-index` walks the content directory for comment files, reads
//! each one's frontmatter with the same reader the CI commands use, settles who
//! wrote it, and writes the lot to [`INDEX_PATH`], grouped by the page each
//! comment is on. The plugin is left with the part only it can do: turning each
//! body into sanitised HTML.
//!
//! Authorship has two sources that agree by construction. The workflow that
//! turns an issue into a pull request writes the opener's login into the file
//! *and* makes the commit in their name, so the frontmatter is what renders and
//! the commit is the corroboration. A file that names nobody, because it was
//! written by hand or before the workflow existed, falls back to the commit that
//! added it. The frontmatter cannot make an unreviewed claim true: a pull request
//! opened by hand can put any login in the file, and the merge is what checks it.

use crate::comments::{frontmatter_field, git, is_comment_file, parse_comment};
use crate::{Result, Site};
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

/// Where the index is written, relative to the repository root. Gitignored,
/// like the rest of `.quartz`, and rewritten by every `site build`.
pub(crate) const INDEX_PATH: &str = ".quartz/comments.json";

/// Who wrote a comment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Author {
    /// GitHub login, for a comment that came through GitHub.
    login: Option<String>,
    /// The address of a comment that arrived as mail. Kept so an edit can be
    /// checked against it, and never rendered: a page is what gets crawled, and
    /// printing an address into one is how it ends up on a list.
    email: Option<String>,
    /// The login, the part of the address before the `@`, or what git recorded.
    name: String,
    avatar: Option<String>,
    profile: Option<String>,
}

impl Author {
    fn github(login: &str, id: Option<u64>) -> Self {
        // The id-based picture survives a rename; the login-based one is all
        // there is without an id.
        let avatar = match id {
            Some(id) => format!("https://avatars.githubusercontent.com/u/{id}?s=64&v=4"),
            None => format!("https://github.com/{login}.png?size=64"),
        };
        Self {
            login: Some(login.to_string()),
            email: None,
            name: login.to_string(),
            avatar: Some(avatar),
            profile: Some(format!("https://github.com/{login}")),
        }
    }

    /// Whatever identity a file records: a GitHub login, or an email address
    /// for a comment added by hand from mail. A login cannot contain `@`.
    fn from_identity(identity: &str, id: Option<u64>) -> Self {
        let Some((local, _)) = identity.split_once('@') else {
            return Self::github(identity, id);
        };
        let local = local.trim();
        Self {
            login: None,
            email: Some(identity.to_string()),
            name: if local.is_empty() {
                "by email".to_string()
            } else {
                local.to_string()
            },
            avatar: None,
            profile: None,
        }
    }

    /// The author of a commit. GitHub's private address, in either of the
    /// shapes it has had, gives a login; a real address gives only the name.
    fn from_commit(name: &str, email: &str) -> Self {
        match noreply_login(email) {
            Some((id, login)) => Self::github(login, id),
            None => Self {
                login: None,
                email: None,
                name: name.to_string(),
                avatar: None,
                profile: None,
            },
        }
    }
}

impl Author {
    fn to_json(&self) -> Value {
        object([
            ("login", self.login.clone().map(Value::from)),
            ("email", self.email.clone().map(Value::from)),
            ("name", Some(Value::from(self.name.clone()))),
            ("avatar", self.avatar.clone().map(Value::from)),
            ("profile", self.profile.clone().map(Value::from)),
        ])
    }
}

/// A JSON object of the fields that are present, which is how the plugin's
/// optional properties read an absent one.
fn object<const N: usize>(fields: [(&str, Option<Value>); N]) -> Value {
    Value::Object(
        fields
            .into_iter()
            .filter_map(|(key, value)| value.map(|value| (key.to_string(), value)))
            .collect::<Map<_, _>>(),
    )
}

/// `12345+login@users.noreply.github.com` or `login@users.noreply.github.com`.
fn noreply_login(email: &str) -> Option<(Option<u64>, &str)> {
    let user = email.strip_suffix("@users.noreply.github.com")?;
    let (id, login) = match user.split_once('+') {
        Some((id, login)) => (Some(id.parse().ok()?), login),
        None => (None, user),
    };
    let valid = !login.is_empty()
        && login
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-');
    valid.then_some((id, login))
}

/// The commit that added a comment file.
#[derive(Clone, Debug)]
struct Commit {
    author: Author,
    date: String,
}

/// One `git log` for every comment file in the repository, keyed by the path
/// each was added at.
///
/// Only the commit that *added* a file counts: later commits are edits by
/// whoever tidied it, and crediting them would be worse than showing nothing.
/// The log is newest first, so the first commit seen for a path stands, which
/// is also what resolves a deleted and re-added file to its current author.
fn added_by(site: &Site) -> HashMap<String, Commit> {
    let mut commits = HashMap::new();
    // No git, or a shallow checkout, leaves every comment to its frontmatter.
    let Some(log) = git(
        site,
        &[
            "log",
            "--format=%x00%aI%x1f%an%x1f%ae",
            "--name-only",
            "--diff-filter=A",
            "--no-show-signature",
            "--",
            "*.comment.*.md",
        ],
    ) else {
        return commits;
    };

    for block in log.split('\0') {
        let mut lines = block.lines();
        let Some(header) = lines.next() else {
            continue;
        };
        let mut fields = header.split('\x1f');
        let (Some(date), Some(name), Some(email)) = (fields.next(), fields.next(), fields.next())
        else {
            continue;
        };
        let commit = Commit {
            author: Author::from_commit(name, email),
            date: date.to_string(),
        };
        for file in lines.filter(|file| !file.is_empty()) {
            commits
                .entry(file.to_string())
                .or_insert_with(|| commit.clone());
        }
    }
    commits
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Revision {
    date: String,
    issue: Option<u64>,
    edited: bool,
}

/// A comment as the plugin receives it, everything but its rendered body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Record {
    /// The `<id>` from the filename; unique per page, not globally.
    id: String,
    /// Repo-relative path of the comment's own file.
    file: String,
    /// Content-relative path of the page it is on.
    parent: String,
    /// When it was first submitted.
    date: String,
    /// When it was last edited, if it ever was.
    edited: Option<String>,
    author: Option<Author>,
    /// Every submission, oldest first.
    history: Vec<Revision>,
    reply_to: Option<String>,
    quote: Option<String>,
    quote_heading: Option<String>,
    /// The body as written, for the plugin to render and the edit button to
    /// prefill.
    source: String,
}

impl Record {
    fn to_json(&self) -> Value {
        let history = self
            .history
            .iter()
            .map(|revision| {
                object([
                    ("date", Some(Value::from(revision.date.clone()))),
                    ("issue", revision.issue.map(Value::from)),
                    ("edited", Some(Value::from(revision.edited))),
                ])
            })
            .collect::<Vec<_>>();
        object([
            ("id", Some(Value::from(self.id.clone()))),
            ("file", Some(Value::from(self.file.clone()))),
            ("parent", Some(Value::from(self.parent.clone()))),
            ("date", Some(Value::from(self.date.clone()))),
            ("edited", self.edited.clone().map(Value::from)),
            ("author", self.author.as_ref().map(Author::to_json)),
            ("history", Some(json!(history))),
            ("replyTo", self.reply_to.clone().map(Value::from)),
            ("quote", self.quote.clone().map(Value::from)),
            ("quoteHeading", self.quote_heading.clone().map(Value::from)),
            ("source", Some(Value::from(self.source.clone()))),
        ])
    }
}

/// An ISO 8601 date or timestamp, as far as needs checking: a file whose
/// date is anything else is skipped, the way it always has been.
fn is_iso_date(date: &str) -> bool {
    let bytes = date.as_bytes();
    bytes.len() >= 10
        && bytes[..10]
            .iter()
            .enumerate()
            .all(|(index, byte)| match index {
                4 | 7 => *byte == b'-',
                _ => byte.is_ascii_digit(),
            })
}

/// The text after the frontmatter, or `None` if the file has none.
fn body(source: &str) -> Option<&str> {
    let rest = source
        .strip_prefix("---\n")
        .or_else(|| source.strip_prefix("---\r\n"))?;
    let end = rest.find("\n---")?;
    let after = &rest[end + "\n---".len()..];
    Some(after.trim_start_matches(['\r', '\n']))
}

fn posix(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

/// Reads one comment file, or `None` for one with nothing to attach to: no
/// page beside it, no frontmatter, no body, or no date anywhere.
fn read(
    file: &Path,
    content_dir: &Path,
    repo_root: &Path,
    commits: &HashMap<String, Commit>,
) -> Option<Record> {
    let name = file.file_name()?.to_str()?;
    let (stem, rest) = name.split_once(".comment.")?;
    let id = rest.strip_suffix(".md")?;

    // A comment on a page since deleted or renamed has nowhere to appear. The
    // file is still in the repository and in history; it just renders nowhere.
    let parent_path = file.with_file_name(format!("{stem}.md"));
    if !parent_path.is_file() {
        return None;
    }

    let source = fs::read_to_string(file).ok()?;
    let source = source.trim();
    let text = body(source)?.trim();
    if text.is_empty() {
        return None;
    }

    let relative = posix(file.strip_prefix(repo_root).ok()?);
    let commit = commits.get(&relative);

    // The file's own date stands once the workflow writes it, since an edit
    // moves the commit but not when the comment was made.
    let date = frontmatter_field(source, "date").or_else(|| commit.map(|c| c.date.clone()))?;
    if !is_iso_date(&date) {
        return None;
    }

    let comment = parse_comment(source);
    let mut history: Vec<Revision> = comment
        .history
        .iter()
        .filter(|revision| is_iso_date(&revision.date))
        .map(|revision| Revision {
            date: revision.date.clone(),
            issue: revision.issue,
            edited: revision.edited,
        })
        .collect();
    if let Some(first) = history.first_mut() {
        first.edited = false;
    } else {
        history.push(Revision {
            date: date.clone(),
            issue: None,
            edited: false,
        });
    }
    let edited = history
        .iter()
        .rev()
        .find(|revision| revision.edited)
        .map(|revision| revision.date.clone());

    // Only a file that claims no author at all is credited to its commit.
    let author = match &comment.author {
        Some(identity) => Some(Author::from_identity(identity, comment.author_id)),
        None => commit.map(|commit| commit.author.clone()),
    };

    Some(Record {
        id: id.to_string(),
        file: relative,
        parent: posix(parent_path.strip_prefix(content_dir).ok()?),
        date: history[0].date.clone(),
        edited,
        author,
        history,
        reply_to: comment.reply_to,
        quote: comment.quote,
        quote_heading: comment.quote_heading,
        source: text.to_string(),
    })
}

fn comment_files(dir: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let path = entry.path();
        if path.is_dir() {
            if name != "node_modules" && !name.starts_with('.') {
                comment_files(&path, found);
            }
        } else if is_comment_file(&name) {
            found.push(path);
        }
    }
}

/// Every comment under `content_dir`, keyed by the content-relative path of its
/// page, each thread oldest first.
fn threads(
    content_dir: &Path,
    repo_root: &Path,
    commits: &HashMap<String, Commit>,
) -> BTreeMap<String, Vec<Record>> {
    let mut files = Vec::new();
    comment_files(content_dir, &mut files);

    let mut threads: BTreeMap<String, Vec<Record>> = BTreeMap::new();
    for file in files {
        if let Some(record) = read(&file, content_dir, repo_root, commits) {
            threads
                .entry(record.parent.clone())
                .or_default()
                .push(record);
        }
    }
    // Every date here is UTC with a `Z`, which is what the compose box and the
    // workflow write, so the strings sort as the times do.
    for thread in threads.values_mut() {
        thread.sort_by(|a, b| a.date.cmp(&b.date).then_with(|| a.id.cmp(&b.id)));
    }
    threads
}

impl Site {
    pub(crate) fn comments_index(&self) -> Result<()> {
        let commits = added_by(self);
        let threads = threads(&self.root.join("content"), &self.root, &commits);
        let path = self.root.join(INDEX_PATH);
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        let json = threads
            .into_iter()
            .map(|(page, thread)| {
                (
                    page,
                    Value::from(thread.iter().map(Record::to_json).collect::<Vec<_>>()),
                )
            })
            .collect::<Map<_, _>>();
        fs::write(&path, Value::Object(json).to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identities_become_authors() {
        let login = Author::from_identity("octocat", Some(583231));
        assert_eq!(login.login.as_deref(), Some("octocat"));
        assert_eq!(
            login.avatar.as_deref(),
            Some("https://avatars.githubusercontent.com/u/583231?s=64&v=4")
        );
        assert_eq!(
            Author::from_identity("octocat", None).avatar.as_deref(),
            Some("https://github.com/octocat.png?size=64")
        );

        let mail = Author::from_identity("someone@example.com", None);
        assert_eq!(mail.name, "someone");
        assert_eq!(mail.login, None);
        assert_eq!(Author::from_identity("@example.com", None).name, "by email");
    }

    #[test]
    fn commits_by_noreply_address_name_the_login() {
        let modern = Author::from_commit("Octo Cat", "583231+octocat@users.noreply.github.com");
        assert_eq!(modern.login.as_deref(), Some("octocat"));
        assert!(modern.avatar.unwrap().contains("/u/583231"));

        let old = Author::from_commit("Octo Cat", "octocat@users.noreply.github.com");
        assert_eq!(old.login.as_deref(), Some("octocat"));

        let real = Author::from_commit("Octo Cat", "octo@example.com");
        assert_eq!(real.name, "Octo Cat");
        assert_eq!(real.login, None);
    }

    #[test]
    fn reads_a_comment_beside_its_page() {
        let root = std::env::temp_dir().join(format!("comment-index-{}", std::process::id()));
        let content = root.join("content");
        fs::create_dir_all(content.join("blog")).unwrap();
        fs::write(content.join("blog/post.md"), "# post\n").unwrap();
        fs::write(
            content.join("blog/post.comment.abc.md"),
            "---\nparent: \"blog/post.md\"\ndate: \"2026-01-02T03:04:05.000Z\"\nauthor: \"octocat\"\nauthorId: 1\nreplyTo: \"xyz\"\nhistory:\n  - date: \"2026-01-02T03:04:05.000Z\"\n    issue: 7\n  - date: \"2026-01-03T00:00:00.000Z\"\n    issue: 8\n    edited: true\n---\n\nhello\n",
        )
        .unwrap();
        fs::write(
            content.join("blog/post.comment.early.md"),
            "---\ndate: \"2025-12-31T00:00:00.000Z\"\n---\n\nfirst\n",
        )
        .unwrap();
        fs::write(
            content.join("blog/gone.comment.orphan.md"),
            "---\ndate: \"2026-01-01T00:00:00.000Z\"\n---\n\nnobody\n",
        )
        .unwrap();

        let threads = threads(&content, &root, &HashMap::new());
        fs::remove_dir_all(&root).unwrap();

        assert_eq!(threads.len(), 1);
        let thread = &threads["blog/post.md"];
        assert_eq!(
            thread
                .iter()
                .map(|record| record.id.as_str())
                .collect::<Vec<_>>(),
            ["early", "abc"]
        );

        let early = &thread[0];
        assert_eq!(early.author, None);
        assert_eq!(early.history.len(), 1);
        assert!(!early.history[0].edited);

        let abc = &thread[1];
        assert_eq!(abc.file, "content/blog/post.comment.abc.md");
        assert_eq!(abc.source, "hello");
        assert_eq!(abc.reply_to.as_deref(), Some("xyz"));
        assert_eq!(abc.history[0].issue, Some(7));
        assert_eq!(abc.edited.as_deref(), Some("2026-01-03T00:00:00.000Z"));
        assert_eq!(
            abc.author.as_ref().unwrap().login.as_deref(),
            Some("octocat")
        );
    }

    #[test]
    fn iso_dates() {
        assert!(is_iso_date("2026-01-02"));
        assert!(is_iso_date("2026-01-02T03:04:05.000Z"));
        assert!(!is_iso_date("yesterday"));
        assert!(!is_iso_date("2026/01/02"));
    }
}
