//! Turning a gallery submission issue into files in a gallery.
//!
//! The submit button on a gallery page opens a prefilled GitHub issue and asks
//! for pictures to be dropped into it. GitHub uploads whatever is dropped and
//! writes a link to it in the body; this reads those links back, fetches the
//! files, puts them through `normalize-gallery`, and leaves the working tree
//! ready for the workflow to commit in the issue opener's name, open a pull
//! request, and close the issue. Merging that pull request is what publishes
//! the pictures — moderation is the merge button and nothing else, exactly as
//! it is for comments.
//!
//! One issue carries as many files as somebody cares to drop into it; they are
//! numbered in the order the body mentions them and land in one pull request.
//!
//! What may be submitted is what `normalize-gallery` can take, decided by the
//! bytes rather than by any name: [`gallery::sniff`] answers with an extension
//! that command accepts, or refuses the file. Anything that arrives is
//! re-encoded or remuxed on the way in, which is where its metadata goes.
//!
//! Everything read here was written by a stranger, so nothing is trusted
//! further than it has been checked:
//!
//! - the collection has to be one of `Site::SUBMITTABLE`, and the pages are
//!   held to that same list by a test in this file;
//! - a URL is fetched only if it is on one of GitHub's own attachment hosts,
//!   because this runs with a token that can push and must never fetch a URL
//!   of somebody else's choosing;
//! - the file's kind comes from its first bytes, never from the URL;
//! - the name it lands under is built here, never taken from the payload;
//! - a submission that would renumber files already published is refused
//!   rather than applied, since that changes the address of every one of them;
//! - and video is refused outright when ffmpeg is missing, because a video
//!   nobody has looked at is exactly the file whose metadata matters.

use crate::comments::{Rejected, parse_marked_issue, reject, write_outputs};
use crate::gallery;
use crate::{Result, Site, SiteError, fail, remove_file_if_exists};
use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The marker the submit button writes into an issue body.
///
/// Keyed off this rather than off a label, for the same reason the comment
/// workflow is: a label set through `?labels=` in a prefilled issue URL is
/// silently dropped for anyone without triage permission on the repository,
/// which is everyone this feature exists for.
pub(crate) const ISSUE_MARKER: &str = "hilll.dev:gallery";

/// The workflow that runs this, relative to the repository root.
///
/// One file for comments and galleries both: two workflows on the same event
/// meant every submission also produced a skipped run of the other one.
#[cfg(test)]
pub(crate) const WORKFLOW: &str = ".github/workflows/submission.yaml";

/// How many files one issue may carry.
///
/// Generous, because several at once is the point of it, and finite, because
/// every one of them is a download, a decode, and a comparison against every
/// other file in the gallery.
const MAX_FILES: usize = 20;

/// The most one file may weigh. GitHub's own ceiling on an issue attachment is
/// lower than this for every type it accepts, so this only ever catches
/// something unexpected.
const MAX_BYTES: u64 = 32 * 1024 * 1024;

/// Where GitHub puts a file somebody dropped into an issue.
///
/// An allowlist of prefixes rather than a pattern, and the single most
/// important line in this file: the workflow that calls it can push to the
/// repository, so the one thing it must never do is fetch a URL of a
/// stranger's choosing. Both of these are GitHub's own attachment hosts, and
/// neither needs a credential for a public repository — nothing here sends one.
const ATTACHMENT_HOSTS: [&str; 2] = [
    "https://github.com/user-attachments/assets/",
    "https://user-images.githubusercontent.com/",
];

/// What a downloaded file is called until `normalize-gallery` numbers it.
///
/// Numbering follows the sorted order of the directory and every name already
/// in a gallery starts with a digit, so a name starting with a letter sorts
/// after all of them and lands at the end of the run. It is also unmistakable
/// in a listing, which is what a half-finished run leaves behind.
const STAGED_PREFIX: &str = "submission-";

fn staged_name(index: usize, extension: &str) -> String {
    format!("{STAGED_PREFIX}{index:02}.{extension}")
}

fn is_staged(name: &str) -> bool {
    name.starts_with(STAGED_PREFIX)
}

/// The files staged in a gallery directory, removed if anything goes wrong.
///
/// A refusal must not leave a stranger's downloads sitting in `content`. On the
/// way out they have either been renamed into the gallery by the normalizer, in
/// which case there is nothing here to remove, or they have not, in which case
/// there is.
struct Staged {
    dir: PathBuf,
    names: Vec<String>,
}

impl Drop for Staged {
    fn drop(&mut self) {
        for name in &self.names {
            let _ = remove_file_if_exists(&self.dir.join(name));
        }
    }
}

/// One file on disk, under the name it will be staged as, and where it came
/// from.
struct Fetched {
    staged: String,
    path: PathBuf,
    source: String,
}

/// One file the gallery gained, and where it came from — which is what the
/// pull request says, so that whoever presses merge can see the original.
pub(crate) struct Added {
    /// Repo-relative path.
    pub path: String,
    pub source: String,
}

/// What the command did, for the workflow's outputs and the pull request.
pub(crate) struct Applied {
    pub collection: String,
    pub login: String,
    pub number: u64,
    /// The files added, in the order they were numbered.
    pub files: Vec<Added>,
    /// The submissions that were copies of something, and what of.
    pub duplicates: Vec<String>,
}

/// Which gallery the payload names, if it is one that takes submissions.
fn collection_of(payload: &Value) -> Result<String> {
    let Some(collection) = payload
        .get("collection")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|collection| !collection.is_empty())
    else {
        return reject("this submission names no gallery");
    };

    if !Site::is_submittable(collection) {
        return reject(format!(
            "`{collection}` does not take submissions; the ones that do are {}",
            Site::submittable_names().join(", ")
        ));
    }

    Ok(collection.to_string())
}

/// The attachment links in an issue body, in the order they appear.
///
/// GitHub writes one of these for every file dropped into the compose box,
/// either as markdown or as an `img` tag, and [`crate::content::extract_urls`]
/// reads both. Everything else in the body — a link somebody typed, a
/// screenshot hosted elsewhere, the guidance the button wrote — is not an
/// attachment and is not fetched.
fn attachments(body: &str) -> Vec<String> {
    let mut urls = Vec::new();

    for url in crate::content::extract_urls(body) {
        let Some(url) = allowed_attachment(&url) else {
            continue;
        };
        if !urls.contains(&url) {
            urls.push(url);
        }
    }

    urls
}

/// The URL to fetch, if the allowlist admits it.
///
/// The prefix test has to be applied to the URL curl will actually ask for, not
/// to the one that was written down, and those are not the same string: curl
/// resolves `..` in a path before it sends anything, so
/// `https://github.com/user-attachments/assets/../../owner/repo/raw/main/x`
/// passes a `starts_with` against the allowlist and then asks for something
/// else entirely. The host cannot be moved this way and no credential is ever
/// sent, so the worst of it was fetching a public file from somewhere else on
/// github.com — but the check and the request disagreeing is the bug, whatever
/// today's blast radius is. So the path is resolved here, the resolved form is
/// what gets tested, and the resolved form is what gets fetched.
fn allowed_attachment(url: &str) -> Option<String> {
    // A percent-encoded dot or slash exists only to make this function and the
    // server read one path two ways: curl passes both through untouched, so
    // whatever the far end does with them, it is not what was resolved here. No
    // attachment URL has ever contained either.
    let lowered = url.to_ascii_lowercase();
    if lowered.contains("%2e") || lowered.contains("%2f") {
        return None;
    }

    let normalized = normalize_path(url)?;
    ATTACHMENT_HOSTS
        .iter()
        .any(|host| normalized.starts_with(host))
        .then_some(normalized)
}

/// RFC 3986's dot-segment removal, over the path and nothing else.
///
/// The authority is left exactly as it was: `..` cannot climb past a host, and
/// the query and fragment are not paths and are not touched.
fn normalize_path(url: &str) -> Option<String> {
    let (scheme, rest) = url.split_once("://")?;
    let (authority, rest) = match rest.find('/') {
        Some(at) => (&rest[..at], &rest[at..]),
        None => (rest, ""),
    };
    let (path, suffix) = match rest.find(['?', '#']) {
        Some(at) => (&rest[..at], &rest[at..]),
        None => (rest, ""),
    };

    let mut segments: Vec<&str> = Vec::new();
    for segment in path.split('/') {
        match segment {
            "." => {}
            // The first segment of an absolute path is the empty string before
            // the leading slash, and it is the root: `..` stops there.
            ".." => {
                if segments.len() > 1 {
                    segments.pop();
                }
            }
            other => segments.push(other),
        }
    }

    Some(format!(
        "{scheme}://{authority}{}{suffix}",
        segments.join("/")
    ))
}

/// Fetches one attachment.
///
/// curl rather than a HTTP crate, as everything else here that speaks to the
/// network does. The flags are the interesting part: https only, on the first
/// request and on any redirect GitHub answers with, a ceiling on the size and
/// on the time, and no credential of any kind. `--` keeps a URL that begins
/// with a dash from being read as an option, which the allowlist already rules
/// out and which costs nothing to rule out twice.
fn download(url: &str, target: &Path) -> Result<Vec<u8>> {
    let output = Command::new("curl")
        .args([
            "-sS",
            "--location",
            "--fail",
            "--proto",
            "=https",
            "--proto-redir",
            "=https",
            "--max-redirs",
            "5",
            "--max-time",
            "120",
            "--max-filesize",
            &MAX_BYTES.to_string(),
            "-A",
            "hilll.dev gallery submissions",
            "-o",
        ])
        .arg(target)
        .arg("--")
        .arg(url)
        .output()?;

    if !output.status.success() {
        return reject(format!(
            "{url} could not be fetched: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    Ok(fs::read(target)?)
}

/// The whole job: validate, fetch, normalize, and report.
///
/// `issue` is the shape GitHub sends. The author is read from `issue.user` and
/// nowhere else, so what the body claims about who is submitting is never
/// consulted — the same rule the comment route follows, for the same reason.
pub(crate) fn apply(
    site: &Site,
    issue: &Value,
    content_dir: &Path,
    downloads: &Path,
) -> Result<Applied> {
    let body = issue
        .get("body")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let submission = parse_marked_issue(body, ISSUE_MARKER)?;
    let collection = collection_of(&submission.payload)?;

    let Some(login) = issue
        .get("user")
        .and_then(|user| user.get("login"))
        .and_then(Value::as_str)
        .filter(|login| !login.is_empty())
    else {
        return reject("the issue has no author");
    };
    let Some(number) = issue.get("number").and_then(Value::as_u64) else {
        return fail("the issue has no number");
    };

    let dir = content_dir.join("misc").join(&collection);
    if !dir.is_dir() {
        return reject(format!("there is no gallery directory for `{collection}`"));
    }

    let urls = attachments(&submission.body);
    if urls.is_empty() {
        return reject(
            "there are no files attached to this issue. Drop pictures into the issue body \
             — GitHub uploads them and writes the links — then open it again",
        );
    }
    if urls.len() > MAX_FILES {
        return reject(format!(
            "{} files is more than one issue takes; the limit is {MAX_FILES}, and there is \
             nothing stopping a second issue",
            urls.len()
        ));
    }

    // Fetched into a directory of their own first: nothing a stranger sent goes
    // anywhere near `content` before its bytes have said what it is.
    fs::create_dir_all(downloads)?;
    let mut fetched = Vec::new();

    for (index, url) in urls.iter().enumerate() {
        let path = downloads.join(format!("{index:02}"));
        let bytes = download(url, &path)?;

        let Some(extension) = gallery::sniff(&bytes) else {
            return reject(format!(
                "{url} is not something a gallery can show. It takes {}",
                gallery::accepted_extensions()
            ));
        };

        if gallery::is_video(extension) && !gallery::ffmpeg_available() {
            return reject(
                "video cannot be taken right now: the metadata around it has to come out \
                 before it is published, and ffmpeg is not installed on the runner",
            );
        }

        fetched.push(Fetched {
            staged: staged_name(index, extension),
            path,
            source: url.clone(),
        });
    }

    let installed = install(site, &collection, &dir, &fetched)?;

    Ok(Applied {
        collection,
        login: login.to_string(),
        number,
        files: installed.files,
        duplicates: installed.duplicates,
    })
}

struct Installed {
    files: Vec<Added>,
    duplicates: Vec<String>,
}

/// Puts the fetched files into the gallery and lets the normalizer name them.
///
/// Split from [`apply`] exactly where the network stops mattering: what arrives
/// here is a list of files on disk under the names they will be staged as,
/// which is what makes the numbering, the re-encoding and the duplicate rules
/// testable without GitHub in the room.
fn install(site: &Site, collection: &str, dir: &Path, fetched: &[Fetched]) -> Result<Installed> {
    let existing = gallery::names_in(dir)?;
    let incoming: Vec<String> = fetched.iter().map(|file| file.staged.clone()).collect();

    // Asked before anything is written, because the answer is a refusal rather
    // than a thing to do: a gallery crossing what two digits can name, or one
    // that was never normalized, renumbers on the way through, and every link
    // to every file below the change breaks. That is a fine afternoon's work by
    // hand and not something to do on somebody's behalf while they watch.
    let moved = gallery::renumbering(&existing, &incoming)?;
    if !moved.is_empty() {
        return reject(format!(
            "adding {} file(s) would renumber {} already in the gallery, which changes the \
             address of every one of them. This one needs doing by hand",
            incoming.len(),
            moved.len()
        ));
    }

    let staged = Staged {
        dir: dir.to_path_buf(),
        names: incoming.clone(),
    };
    for file in fetched {
        fs::copy(&file.path, dir.join(&file.staged))?;
    }

    let dropped = match drop_duplicates(dir, &existing, &incoming) {
        Ok(dropped) => dropped,
        // Every step of that decodes a file a stranger sent, so a failure is
        // far likelier to be a file that is not the picture its first bytes
        // claimed than a fault worth failing the run over. Said back on the
        // issue, where the person who sent it can do something about it.
        Err(error) if error.downcast_ref::<Rejected>().is_none() => {
            return reject(format!(
                "one of these files could not be read as the picture it claims to be: {error}"
            ));
        }
        Err(error) => return Err(error),
    };

    if dropped.len() == incoming.len() {
        return reject(
            "every picture in this issue is already in the gallery. Nothing has been changed",
        );
    }

    gallery::normalize_submission(site, collection)?;
    // Everything staged has been renamed by now, so the guard has nothing left
    // to clean up. Dropping it here rather than at the end of the function is
    // what says so.
    drop(staged);

    let after = gallery::names_in(dir)?;
    // The check above was an argument about what the normalizer would do; this
    // is the fact about what it did. Nothing already published may have moved.
    if let Some(vanished) = existing.iter().find(|name| !after.contains(name)) {
        return fail(format!(
            "{vanished} is no longer in {collection}; the gallery was renumbered after all, \
             and nothing here should be committed"
        ));
    }

    // The normalizer numbers in name order and the staged names sort in the
    // order they were fetched, so the new numbers and the surviving sources
    // line up one to one. That is an argument; the length check is the fact.
    let survivors: Vec<&Fetched> = fetched
        .iter()
        .filter(|file| !dropped.iter().any(|(goes, _)| *goes == file.staged))
        .collect();
    let names: Vec<&String> = after
        .iter()
        .filter(|name| !existing.contains(name))
        .collect();

    if names.is_empty() {
        return fail(format!("nothing was added to {collection}"));
    }
    if names.len() != survivors.len() {
        return fail(format!(
            "{} file(s) were added to {collection} for {} that survived the duplicate scan, \
             so which came from where cannot be said",
            names.len(),
            survivors.len()
        ));
    }

    let files = names
        .iter()
        .zip(survivors)
        .map(|(name, file)| Added {
            path: format!("content/misc/{collection}/{name}"),
            source: file.source.clone(),
        })
        .collect();
    let duplicates = dropped
        .into_iter()
        .map(|(goes, stays)| format!("{goes} is a copy of {stays}"))
        .collect();

    Ok(Installed { files, duplicates })
}

/// Deletes the submissions that are copies of something already in the gallery,
/// or of each other, and says which went and what for.
///
/// The scan itself is `normalize-gallery`'s, run over the whole directory at
/// once so that a submission is weighed against every published file. What is
/// different here is which of a pair goes: the scan keeps the larger copy, and
/// that is the wrong answer when the smaller one is already published under a
/// number people have linked to. So the file that goes is always the one that
/// arrived today, and two files that were both already here are left alone —
/// tidying the gallery is not a stranger's issue's business.
fn drop_duplicates(
    dir: &Path,
    existing: &[String],
    incoming: &[String],
) -> Result<Vec<(String, String)>> {
    let mut names = existing.to_vec();
    names.extend_from_slice(incoming);
    names.sort();

    let mut dropped = Vec::new();

    for removal in gallery::duplicates(dir, &names)? {
        let (goes, stays) = match (is_staged(&removal.dropped), is_staged(&removal.kept)) {
            (true, _) => (removal.dropped, removal.kept),
            (false, true) => (removal.kept, removal.dropped),
            (false, false) => continue,
        };

        remove_file_if_exists(&dir.join(&goes))?;
        dropped.push((goes, stays));
    }

    Ok(dropped)
}

/// The pull request's description, written here so that the workflow never
/// has to compose markdown in a shell.
///
/// It names every file with where it came from, so that reviewing the pull
/// request is comparing the picture with its original rather than taking the
/// diff's word for it, and says which submissions were dropped as copies.
fn pull_request_body(applied: &Applied) -> String {
    let mut body = format!(
        "From #{}, opened by @{}.\n\n",
        applied.number, applied.login
    );

    for file in &applied.files {
        body.push_str(&format!("- `{}`, from <{}>\n", file.path, file.source));
    }

    if !applied.duplicates.is_empty() {
        body.push('\n');
        for note in &applied.duplicates {
            body.push_str(&format!("{note}, and was not added.\n"));
        }
    }

    body.push_str("\nMerging publishes them. Closing this without merging discards them.\n");
    body
}

pub(crate) fn from_issue(site: &Site) -> Result<()> {
    let issue: Value = serde_json::from_str(&env::var("ISSUE_JSON").unwrap_or_default())
        .map_err(|error| SiteError::new(format!("ISSUE_JSON is not valid JSON: {error}")))?;
    let content_dir = site
        .root
        .join(env::var("CONTENT_DIR").unwrap_or_else(|_| "content".into()));
    let downloads = env::temp_dir().join(format!("site-submission-{}", std::process::id()));

    let applied = apply(site, &issue, &content_dir, &downloads);
    let _ = fs::remove_dir_all(&downloads);

    let applied = match applied {
        Ok(applied) => applied,
        Err(error) => {
            // A refusal is said back on the issue by the workflow, so it has to
            // reach it as an output before this exits.
            if error.downcast_ref::<Rejected>().is_some() {
                write_outputs(&[("rejected", error.to_string())])?;
            }
            return Err(error);
        }
    };

    for note in &applied.duplicates {
        println!("{}: {note}, and was not added", applied.collection);
    }

    let files = applied
        .files
        .iter()
        .map(|file| file.path.as_str())
        .collect::<Vec<_>>();
    let outputs = [
        ("collection", applied.collection.clone()),
        ("login", applied.login.clone()),
        ("count", files.len().to_string()),
        ("files", files.join(" ")),
        ("dir", format!("content/misc/{}", applied.collection)),
        ("body", pull_request_body(&applied)),
    ];

    for (key, value) in &outputs {
        println!("{key}={value}");
    }
    write_outputs(&outputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn issue_body(payload: &str, body: &str) -> String {
        format!("<!--{ISSUE_MARKER}\n{payload}\n-->\n\n{body}\n")
    }

    #[test]
    fn takes_the_gallery_from_the_payload_and_nowhere_else() {
        let submission = parse_marked_issue(
            &issue_body(r#"{"collection":"trolley"}"#, "![a](x)"),
            ISSUE_MARKER,
        )
        .unwrap();
        assert_eq!(collection_of(&submission.payload).unwrap(), "trolley");

        // A gallery, but not one that takes submissions.
        let media = parse_marked_issue(
            &issue_body(r#"{"collection":"media"}"#, "![a](x)"),
            ISSUE_MARKER,
        )
        .unwrap();
        assert!(collection_of(&media.payload).is_err());

        // A path, hopefully, rather than a name.
        let climbing = parse_marked_issue(
            &issue_body(r#"{"collection":"../../etc"}"#, "![a](x)"),
            ISSUE_MARKER,
        )
        .unwrap();
        assert!(collection_of(&climbing.payload).is_err());
    }

    #[test]
    fn fetches_only_what_github_itself_is_hosting() {
        let body = "\
![one](https://github.com/user-attachments/assets/1111-2222)
<img src=\"https://user-images.githubusercontent.com/1/two.png\" width=\"200\">
and a link somebody typed: https://example.com/trolley.jpg
and one dressed up as an attachment: https://github.com.evil.example/user-attachments/assets/3
and the same file twice: https://github.com/user-attachments/assets/1111-2222";

        assert_eq!(
            attachments(body),
            vec![
                "https://github.com/user-attachments/assets/1111-2222",
                "https://user-images.githubusercontent.com/1/two.png",
            ]
        );
    }

    /// The allowlist has to hold against the URL curl resolves, not the one
    /// that was written down.
    #[test]
    fn climbing_out_of_the_attachment_path_is_not_an_attachment() {
        for url in [
            "https://github.com/user-attachments/assets/../../float3/private/raw/main/x.jpg",
            "https://github.com/user-attachments/assets/..%2f..%2fx.jpg",
            "https://github.com/user-attachments/assets/%2e%2e/%2e%2e/x.jpg",
            "https://github.com/user-attachments/../user-attachments/assets/x.jpg/../../../x",
        ] {
            assert_eq!(allowed_attachment(url), None, "{url} should not be fetched");
        }

        // And an ordinary attachment still is one, unchanged.
        let plain = "https://github.com/user-attachments/assets/1111-2222";
        assert_eq!(allowed_attachment(plain), Some(plain.to_string()));

        // A `.` segment resolves away rather than being refused; it is not an
        // attempt at anything, and the resolved form is what gets fetched.
        assert_eq!(
            allowed_attachment("https://github.com/user-attachments/./assets/7?x=1").as_deref(),
            Some("https://github.com/user-attachments/assets/7?x=1")
        );

        // The authority is never a path: `..` cannot climb into it.
        assert_eq!(
            normalize_path("https://github.com/../../../evil").as_deref(),
            Some("https://github.com/evil")
        );
    }

    #[test]
    fn names_a_download_after_what_its_bytes_say_it_is() {
        assert_eq!(staged_name(0, "jpg"), "submission-00.jpg");
        assert_eq!(staged_name(11, "mp4"), "submission-11.mp4");
        assert!(is_staged("submission-00.jpg"));
        assert!(!is_staged("42.jpg"));

        // Which is what puts them at the end of the run rather than in the
        // middle of it: every published name starts with a digit.
        let mut names = ["42.jpg".to_string(), staged_name(0, "jpg")];
        names.sort();
        assert_eq!(names[1], "submission-00.jpg");
    }

    #[test]
    fn the_pull_request_says_where_every_file_came_from() {
        let applied = Applied {
            collection: "trolley".into(),
            login: "somebody".into(),
            number: 156,
            files: vec![
                Added {
                    path: "content/misc/trolley/69.jpg".into(),
                    source: "https://github.com/user-attachments/assets/7a4a".into(),
                },
                Added {
                    path: "content/misc/trolley/70.jpg".into(),
                    source: "https://user-images.githubusercontent.com/1/two.png".into(),
                },
            ],
            duplicates: vec!["submission-01.png is a copy of 00.jpg".into()],
        };

        assert_eq!(
            pull_request_body(&applied),
            "From #156, opened by @somebody.\n\
             \n\
             - `content/misc/trolley/69.jpg`, from <https://github.com/user-attachments/assets/7a4a>\n\
             - `content/misc/trolley/70.jpg`, from <https://user-images.githubusercontent.com/1/two.png>\n\
             \n\
             submission-01.png is a copy of 00.jpg, and was not added.\n\
             \n\
             Merging publishes them. Closing this without merging discards them.\n"
        );
    }

    /// The button writes the marker and this reads it, and they are in two
    /// languages that cannot see each other. If they ever disagree, every
    /// submission is quietly a normal issue that nobody answers, so the one
    /// string they share is checked rather than trusted.
    #[test]
    fn the_page_and_the_workflow_agree_on_the_marker() {
        let root = crate::find_repo_root().unwrap();
        let source = fs::read_to_string(root.join("ts/src/gallery/submit.ts")).unwrap();

        assert!(
            source.contains(&format!("ISSUE_MARKER = \"{ISSUE_MARKER}\"")),
            "ts/src/gallery/submit.ts does not write the {ISSUE_MARKER} marker"
        );
        // And the workflow routes on it, which is a third spelling of it again.
        let workflow = fs::read_to_string(root.join(WORKFLOW)).unwrap();
        assert!(
            workflow.contains(&format!("\"{ISSUE_MARKER}\"")),
            "{WORKFLOW} does not route on the {ISSUE_MARKER} marker"
        );
    }

    /// The submit button appears on a page because the page says so, and the
    /// gallery accepts submissions because this crate says so. A page offering
    /// what the command will refuse is a button that only ever wastes
    /// somebody's afternoon, so the two lists are held together here.
    #[test]
    fn every_page_offering_a_submit_button_names_a_gallery_that_takes_one() {
        let root = crate::find_repo_root().unwrap();
        let mut offered = Vec::new();

        for entry in walk(&root.join("content")) {
            let source = fs::read_to_string(&entry).unwrap_or_default();
            if !source.contains("data-submit-repo") {
                continue;
            }

            for line in source
                .lines()
                .filter(|line| line.contains("data-submit-repo"))
            {
                let collection = line
                    .split_once("data-collection=\"")
                    .and_then(|(_, rest)| rest.split_once('"'))
                    .map(|(collection, _)| collection.to_string())
                    .unwrap_or_default();

                assert!(
                    Site::is_submittable(&collection),
                    "{} offers submissions for {collection:?}, which Site::SUBMITTABLE does not list",
                    entry.display()
                );
                offered.push(collection);
            }
        }

        for collection in Site::submittable_names() {
            assert!(
                offered.iter().any(|offered| offered == collection),
                "{collection} takes submissions and no page offers them"
            );
        }
    }

    /// The whole of the disk half, over a gallery of its own: three files
    /// arrive, one of them a picture already in the gallery under another
    /// format, and what comes out is numbered, re-encoded, one shorter, and
    /// still knows where each file came from.
    #[cfg(feature = "photos")]
    #[test]
    fn numbers_re_encodes_and_deduplicates_what_arrives() {
        use image::{Rgb, RgbImage};

        let root = env::temp_dir().join(format!("site-install-{}", std::process::id()));
        let dir = root.join("content/misc/trolley");
        let downloads = root.join("downloads");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&dir).unwrap();
        fs::create_dir_all(&downloads).unwrap();

        let red = RgbImage::from_pixel(64, 48, Rgb([220, 30, 30]));
        let blue = RgbImage::from_pixel(64, 48, Rgb([30, 30, 220]));
        let green = RgbImage::from_pixel(64, 48, Rgb([30, 220, 30]));

        // What the gallery already holds, published under a number.
        red.save(dir.join("00.jpg")).unwrap();

        // What arrives: a PNG, the same red square again as a PNG, which
        // shares no byte with the JPEG of it above, and a third picture.
        blue.save(downloads.join("one.png")).unwrap();
        red.save(downloads.join("two.png")).unwrap();
        green.save(downloads.join("three.png")).unwrap();
        let fetched = vec![
            Fetched {
                staged: staged_name(0, "png"),
                path: downloads.join("one.png"),
                source: "https://github.com/user-attachments/assets/1".into(),
            },
            Fetched {
                staged: staged_name(1, "png"),
                path: downloads.join("two.png"),
                source: "https://github.com/user-attachments/assets/2".into(),
            },
            Fetched {
                staged: staged_name(2, "png"),
                path: downloads.join("three.png"),
                source: "https://github.com/user-attachments/assets/3".into(),
            },
        ];

        let site = Site { root, ci: false };
        let installed = install(&site, "trolley", &dir, &fetched).unwrap();

        // The new pictures are JPEGs under the next numbers, each still paired
        // with its origin, and the copy of one already here is gone rather
        // than published twice.
        let added: Vec<(&str, &str)> = installed
            .files
            .iter()
            .map(|file| (file.path.as_str(), file.source.as_str()))
            .collect();
        assert_eq!(
            added,
            vec![
                (
                    "content/misc/trolley/01.jpg",
                    "https://github.com/user-attachments/assets/1"
                ),
                (
                    "content/misc/trolley/02.jpg",
                    "https://github.com/user-attachments/assets/3"
                ),
            ]
        );
        assert_eq!(installed.duplicates.len(), 1);
        assert!(installed.duplicates[0].starts_with("submission-01.png is a copy of 00.jpg"));

        // Nothing staged is left in the gallery, and the manifest the page
        // fetches names exactly what is there.
        assert_eq!(
            gallery::names_in(&dir).unwrap(),
            vec!["00.jpg".to_string(), "01.jpg".into(), "02.jpg".into()]
        );
        let manifest = fs::read_to_string(dir.join("index.json")).unwrap();
        assert_eq!(manifest.trim(), r#"["00.jpg","01.jpg","02.jpg"]"#);

        let _ = fs::remove_dir_all(&site.root);
    }

    fn walk(dir: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();
        let Ok(entries) = fs::read_dir(dir) else {
            return files;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(walk(&path));
            } else if path.extension().is_some_and(|extension| extension == "md") {
                files.push(path);
            }
        }

        files
    }
}
