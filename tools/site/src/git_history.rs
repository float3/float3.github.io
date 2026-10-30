//! Each page's version count and dates, from git, for the Quartz git-history
//! plugin.
//!
//! `site git-history` reads the whole history in one `git log`, threads every
//! file back through its renames, and writes [`INDEX_PATH`]: for each markdown
//! file under `content/`, how many commits count as revisions of it, when it was
//! created, and when it was last really changed. Bot commits never count, and
//! neither do the hand-made sweeps in [`EXCLUDED`].

use crate::comments::git;
use crate::{Result, Site};
use serde_json::{Map, Value, json};
use std::collections::{HashMap, HashSet};
use std::fs;

/// Where the index is written, relative to the repository root. Gitignored,
/// like the rest of `.quartz`, and rewritten by every `site build`.
pub(crate) const INDEX_PATH: &str = ".quartz/git-history.json";

/// A commit that should not count as an edit to the pages it touched.
///
/// A sweep that touches thirty files to fix one typo, rename a directory or
/// restyle frontmatter is one act of maintenance, not thirty revisions. Left
/// in, these dominate the version counts and drag every page's "updated" date
/// to whichever day a find-and-replace last ran.
///
/// An exclusion never changes a page's created date: a page added by a
/// migration was still created then, whatever else the commit touched. And
/// this is a list of specific commits rather than a heuristic on file count or
/// diff size, because "broad and mechanical" is a judgement about intent that
/// a threshold gets wrong in both directions.
struct Exclusion {
    /// Full 40-character SHA. Abbreviations are not matched.
    sha: &'static str,
    /// The same patch under a second SHA. Most of this history exists twice: a
    /// rebased lineage was merged back, so 1546 of 1858 commits have an
    /// identical-patch twin, and excluding one of a pair leaves the other doing
    /// all the damage by itself.
    twin: Option<&'static str>,
    /// What the sweep did, so a later reader can re-judge it, and for one with
    /// `keep` paths, what it did to those.
    reason: &'static str,
    /// Paths the commit genuinely changed, which go on counting, for a sweep
    /// that also did one piece of real work in passing. These are the file's
    /// path today: history is threaded through renames before this is read.
    keep: &'static [&'static str],
}

const fn sweep(sha: &'static str, twin: Option<&'static str>, reason: &'static str) -> Exclusion {
    Exclusion {
        sha,
        twin,
        reason,
        keep: &[],
    }
}

const EXCLUDED: &[Exclusion] = &[
    sweep(
        "2ab131a745f52253618676431127601540806e18",
        None,
        "drop date frontmatter now that dates come from git history — 31 files, 2 lines each",
    ),
    sweep(
        "576e2bf88528fb7dce8f2941aadabba36bdc9e3b",
        Some("58f7acff7d67b276ee6efedf4461da7c6539eef4"),
        "set every `updated:` to the same day — 14 files, one line each",
    ),
    Exclusion {
        keep: &["content/notes/blogs.md"],
        ..sweep(
            "152fd7f9fa2dac8f19f6453085b5a1047390883b",
            Some("43d0c5f73a9c27aa276471b0ec18fe3d8d708e21"),
            "add `tags:` frontmatter — 18 files; it also added a Misc section and a link to \
             blogs.md while it was in there",
        )
    },
    sweep(
        "6d3a7f8604f6c2fa717eeee84e06dc72f7c26d7c",
        Some("39b09b2cd4852b5bf6aebc03f9e923f78f7f5778"),
        "add a `date:` line to the thoughts pages — 4 files, one line each",
    ),
    sweep(
        "f6d7220c04032beba213d6a0dd537ed16879cbb1",
        Some("9804200369b47f68ea40fbb236e461a18ba75d00"),
        "strip date frontmatter and reflow some URLs — 4 files",
    ),
    sweep(
        "c967dea69a3ed04611fc690721b362bda51ac256",
        None,
        "fix the wasm credit typo on the tool pages — 11 files, the same one-line fix",
    ),
    sweep(
        "15de5575ab179efdcbbe1dcbe112c21c028f29d3",
        None,
        "reword the wasm credit line on the tool pages — 11 files, the same replacement",
    ),
    sweep(
        "1f85f57fe8066ba88395f947d665d0e5cafa680f",
        None,
        "reformat the tag lists — 9 files, no prose changed",
    ),
    sweep(
        "36c5660c0cb707d66417d5f3193d8ff915f91719",
        Some("b04cfee4e8a80229e37793d17370cacb0a9687a6"),
        "whitespace reformatting — 19 files, every line removed and re-added identically",
    ),
    sweep(
        "f26dc1310cb6dd2d715b4ff913a79824f895b60d",
        Some("62db4fc16f117821e4601107412d2c25dc2d6b20"),
        "move blog/ to posts/ and notes/ to thoughts/ — 12 renames plus 3 link fixes",
    ),
    sweep(
        "6df46de3ffe2d0f98bc7beea2a5224ccb5fef73e",
        Some("4d8288f6ea53fde3285089e7e1f44ad63d7591d2"),
        "move posts/ back to blog/ — 26 files, almost all pure renames",
    ),
    sweep(
        "12e615cf57885be875487cfb90b2468cb8ac8ac1",
        Some("4119e2a933ef27b40822c464b1083afb9f48c5d4"),
        "rewrite tuningplayground/ links to piano/ — 5 files",
    ),
    sweep(
        "22632b42b5b924554c928863678c9d69f5ac85a7",
        Some("d0e08a89924715bc7bac6e2634e5715793db17d7"),
        "swap the tool `<script src>` paths after a module rename — 4 files",
    ),
    Exclusion {
        keep: &["content/notes/agi.md"],
        ..sweep(
            "2624fd57b50c2a3975a8548527dc2c75939fbdb9",
            Some("bd4ab6a470cf0f8d9f98a487a809e7f64f85a67c"),
            "convert to quartz: TOML to YAML frontmatter across everything that existed then; \
             agi.md was written in this commit rather than converted by it, and is kept under \
             the name it has now, notes/, not the thoughts/ it had then",
        )
    },
];

fn exclusion(sha: &str) -> Option<&'static Exclusion> {
    EXCLUDED
        .iter()
        .find(|exclusion| exclusion.sha == sha || exclusion.twin == Some(sha))
}

/// Whether `sha` should be ignored when counting and dating `file`.
fn is_excluded(sha: &str, file: &str) -> bool {
    exclusion(sha).is_some_and(|exclusion| !exclusion.keep.contains(&file))
}

/// CI pushes as `github-actions[bot]` and Dependabot as `dependabot[bot]`;
/// neither is an edit to a page. A person's own noreply address is on the same
/// domain without the `[bot]`, so it still counts.
fn is_bot(email: &str) -> bool {
    email.ends_with("@users.noreply.github.com") && email.contains("[bot]")
}

/// One commit as it touched one file.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Touch {
    sha: String,
    /// The author date, ISO 8601 as git writes it.
    date: String,
    human: bool,
    /// This commit created the file, under this name or one it was renamed from.
    added: bool,
}

/// The output of `git log --format=%x00%H%x1f%aI%x1f%ae --name-status -M`,
/// newest first, as each file's commits under the name it has now.
///
/// The log is newest first, so every rename is seen before the older commits
/// that still use the previous name, which is what threads a file back through
/// its old names without a per-file `--follow`.
fn touches(log: &str) -> (HashMap<String, Vec<Touch>>, HashSet<String>) {
    let mut history: HashMap<String, Vec<Touch>> = HashMap::new();
    let mut renamed_to: HashMap<String, String> = HashMap::new();
    let mut seen = HashSet::new();

    let current_name = |renamed_to: &HashMap<String, String>, file: &str| {
        let mut file = file.to_string();
        let mut visited = HashSet::new();
        while let Some(next) = renamed_to.get(&file) {
            if !visited.insert(file.clone()) {
                break;
            }
            file = next.clone();
        }
        file
    };

    for block in log.split('\0') {
        let mut lines = block.lines();
        let Some(header) = lines.next() else {
            continue;
        };
        let mut fields = header.split('\x1f');
        let (Some(sha), Some(date), Some(email)) = (fields.next(), fields.next(), fields.next())
        else {
            continue;
        };
        seen.insert(sha.to_string());

        for entry in lines {
            let fields: Vec<&str> = entry.split('\t').collect();
            if fields.len() < 2 {
                continue;
            }
            let file = if fields[0].starts_with('R') && fields.len() >= 3 {
                let file = current_name(&renamed_to, fields[2]);
                renamed_to.insert(fields[1].to_string(), file.clone());
                file
            } else {
                current_name(&renamed_to, fields[1])
            };
            // Per file rather than per commit: one commit adds some paths and
            // merely touches others, and only the add means authorship.
            history.entry(file).or_default().push(Touch {
                sha: sha.to_string(),
                date: date.to_string(),
                human: !is_bot(email),
                added: fields[0].starts_with('A'),
            });
        }
    }
    (history, seen)
}

/// What a page's history comes to.
#[derive(Debug, PartialEq, Eq)]
struct PageHistory {
    versions: usize,
    created: String,
    modified: String,
}

fn page_history(file: &str, commits: &[Touch]) -> Option<PageHistory> {
    let mine: Vec<&Touch> = commits.iter().filter(|commit| commit.human).collect();
    // A sweep goes on feeding the created date, but is not a revision of the
    // page and does not move its updated date. A commit that created the page
    // is authorship, never a sweep, however much else it swept on the way past.
    let edits: Vec<&Touch> = mine
        .iter()
        .copied()
        .filter(|commit| commit.added || !is_excluded(&commit.sha, file))
        .collect();
    // A page only automation has touched still gets real dates, and one whose
    // every edit was a sweep falls back rather than reporting nothing.
    let dated: Vec<&Touch> = if mine.is_empty() {
        commits.iter().collect()
    } else {
        mine
    };
    let touched = if edits.is_empty() { &dated } else { &edits };
    Some(PageHistory {
        versions: edits.len(),
        created: dated.last()?.date.clone(),
        modified: touched.first()?.date.clone(),
    })
}

/// A browsable `https://host/owner/repo` out of `git@host:owner/repo.git` or
/// `https://host/owner/repo.git`.
fn web_url(remote: &str) -> Option<String> {
    let remote = remote.trim();
    let trim = |path: &str| {
        let path = path.trim_end_matches('/');
        path.strip_suffix(".git").unwrap_or(path).to_string()
    };
    for scheme in ["https://", "http://", "ssh://", "git://"] {
        if let Some(rest) = remote.strip_prefix(scheme) {
            let rest = match rest.split_once('@') {
                Some((user, host)) if !user.contains('/') => host,
                _ => rest,
            };
            let rest = trim(rest);
            return (!rest.is_empty()).then(|| format!("https://{rest}"));
        }
    }
    let rest = remote.split_once('@').map_or(remote, |(_, rest)| rest);
    let (host, path) = rest.split_once(':')?;
    (!host.contains('/') && !path.is_empty()).then(|| format!("https://{host}/{}", trim(path)))
}

/// The remote's default branch, falling back to `master` for a CI checkout
/// that never wrote `origin/HEAD`.
fn default_branch(site: &Site) -> String {
    git(
        site,
        &["symbolic-ref", "--short", "refs/remotes/origin/HEAD"],
    )
    .map(|head| head.trim().trim_start_matches("origin/").to_string())
    .filter(|branch| !branch.is_empty())
    .unwrap_or_else(|| "master".to_string())
}

fn encode_path(file: &str) -> String {
    file.split('/')
        .map(|segment| {
            segment
                .bytes()
                .map(|byte| {
                    if byte.is_ascii_alphanumeric() || b"-_.!~*'()".contains(&byte) {
                        char::from(byte).to_string()
                    } else {
                        format!("%{byte:02X}")
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("/")
}

impl Site {
    pub(crate) fn git_history(&self) -> Result<()> {
        let index = match git(
            self,
            &[
                "log",
                "--format=%x00%H%x1f%aI%x1f%ae",
                "--name-status",
                "-M",
                "--no-show-signature",
            ],
        ) {
            Some(log) => self.history_index(&log),
            None => {
                self.warn("no git history available, so pages fall back to frontmatter dates");
                json!({ "pages": null })
            }
        };
        let path = self.root.join(INDEX_PATH);
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        fs::write(path, index.to_string())?;
        Ok(())
    }

    fn history_index(&self, log: &str) -> Value {
        let (history, seen) = touches(log);

        // A listed SHA or kept path that history no longer has excludes
        // nothing, silently: after a rebase, or from a typo, the list has rotted.
        let stray_paths: Vec<&str> = EXCLUDED
            .iter()
            .flat_map(|exclusion| exclusion.keep.iter().copied())
            .filter(|file| !history.contains_key(*file))
            .collect();
        if !stray_paths.is_empty() {
            self.warn(&format!(
                "git_history.rs keeps {} path(s) no file has: {}",
                stray_paths.len(),
                stray_paths.join(", ")
            ));
        }
        let missing: Vec<String> = EXCLUDED
            .iter()
            .flat_map(|exclusion| {
                std::iter::once(exclusion.sha)
                    .chain(exclusion.twin)
                    .map(move |sha| (sha, exclusion.reason))
            })
            .filter(|(sha, _)| !seen.contains(*sha))
            .map(|(sha, reason)| format!("{} ({reason})", &sha[..8]))
            .collect();
        if !missing.is_empty() {
            self.warn(&format!(
                "git_history.rs lists {} commit(s) not in this history: {}",
                missing.len(),
                missing.join(", ")
            ));
        }

        let base = git(self, &["remote", "get-url", "origin"]).and_then(|remote| web_url(&remote));
        let branch = default_branch(self);

        let mut pages = Map::new();
        for (file, commits) in &history {
            if !file.starts_with("content/") || !file.ends_with(".md") {
                continue;
            }
            let Some(page) = page_history(file, commits) else {
                continue;
            };
            let mut entry = json!({
                "versions": page.versions,
                "created": page.created,
                "modified": page.modified,
            });
            if let Some(base) = &base {
                entry["historyUrl"] =
                    Value::from(format!("{base}/commits/{branch}/{}", encode_path(file)));
            }
            pages.insert(file.clone(), entry);
        }
        json!({ "pages": pages })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SWEEP: &str = "2ab131a745f52253618676431127601540806e18";

    fn log(commits: &[(&str, &str, &str, &[&str])]) -> String {
        commits
            .iter()
            .map(|(sha, date, email, entries)| {
                format!("\0{sha}\x1f{date}\x1f{email}\n\n{}\n", entries.join("\n"))
            })
            .collect()
    }

    #[test]
    fn threads_files_through_renames() {
        let (history, _) = touches(&log(&[
            (
                "c",
                "2026-03-01T00:00:00Z",
                "me@example.com",
                &["M\tcontent/blog/new.md"],
            ),
            (
                "b",
                "2026-02-01T00:00:00Z",
                "me@example.com",
                &["R100\tcontent/posts/old.md\tcontent/blog/new.md"],
            ),
            (
                "a",
                "2026-01-01T00:00:00Z",
                "me@example.com",
                &["A\tcontent/posts/old.md"],
            ),
        ]));
        let commits = &history["content/blog/new.md"];
        assert_eq!(
            commits
                .iter()
                .map(|commit| commit.sha.as_str())
                .collect::<Vec<_>>(),
            ["c", "b", "a"]
        );
        assert!(commits[2].added);
        assert!(!history.contains_key("content/posts/old.md"));
    }

    #[test]
    fn bots_and_sweeps_do_not_count() {
        let file = "content/blog/post.md";
        let (history, _) = touches(&log(&[
            (
                "bot",
                "2026-04-01T00:00:00Z",
                "41898282+github-actions[bot]@users.noreply.github.com",
                &["M\tcontent/blog/post.md"],
            ),
            (
                SWEEP,
                "2026-03-01T00:00:00Z",
                "me@example.com",
                &["M\tcontent/blog/post.md"],
            ),
            (
                "edit",
                "2026-02-01T00:00:00Z",
                "1+me@users.noreply.github.com",
                &["M\tcontent/blog/post.md"],
            ),
            (
                "add",
                "2026-01-01T00:00:00Z",
                "me@example.com",
                &["A\tcontent/blog/post.md"],
            ),
        ]));
        let page = page_history(file, &history[file]).unwrap();
        assert_eq!(page.versions, 2);
        assert_eq!(page.created, "2026-01-01T00:00:00Z");
        assert_eq!(page.modified, "2026-02-01T00:00:00Z");
    }

    #[test]
    fn a_page_only_bots_touched_keeps_its_dates() {
        let file = "content/misc/index.md";
        let (history, _) = touches(&log(&[(
            "bot",
            "2026-04-01T00:00:00Z",
            "github-actions[bot]@users.noreply.github.com",
            &["A\tcontent/misc/index.md"],
        )]));
        let page = page_history(file, &history[file]).unwrap();
        assert_eq!(page.versions, 0);
        assert_eq!(page.created, "2026-04-01T00:00:00Z");
        assert_eq!(page.modified, "2026-04-01T00:00:00Z");
    }

    #[test]
    fn kept_paths_still_count() {
        assert!(is_excluded(SWEEP, "content/blog/post.md"));
        let keeps = "152fd7f9fa2dac8f19f6453085b5a1047390883b";
        assert!(!is_excluded(keeps, "content/notes/blogs.md"));
        assert!(is_excluded(
            "43d0c5f73a9c27aa276471b0ec18fe3d8d708e21",
            "content/x.md"
        ));
    }

    #[test]
    fn remotes_become_web_urls() {
        for remote in [
            "git@github.com:float3/float3.github.io.git",
            "https://github.com/float3/float3.github.io.git",
            "https://github.com/float3/float3.github.io",
            "ssh://git@github.com/float3/float3.github.io.git",
        ] {
            assert_eq!(
                web_url(remote).as_deref(),
                Some("https://github.com/float3/float3.github.io"),
                "{remote}"
            );
        }
        assert_eq!(encode_path("content/a b.md"), "content/a%20b.md");
    }
}
