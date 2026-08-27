//! Checking that the site's own links point at something.
//!
//! Every `href` and `src` in the built site that stays on this site is
//! resolved against `public/` and looked for on disk. What this catches is the
//! link that is off by a directory: markdown says `](thing.mp4)`, the file is
//! in `misc/media`, and the page ends up asking for `/thing.mp4`. Nothing
//! complains -- Quartz resolves the link happily, the build succeeds, and the
//! reader gets the 404 page, which their browser reports as a video in an
//! unsupported format rather than a video that is not there. Five links on
//! this site were wrong that way, one of them for a year.
//!
//! Names are compared as text rather than handed to the filesystem, because
//! the filesystem this is usually run on does not care about case and the one
//! that serves the site does. `Build/build.data.gz` and `build/build.data.gz`
//! are the same file here and two different URLs on GitHub Pages, which is its
//! own bug from this week.
//!
//! Only local links are followed. An external URL is somebody else's to keep
//! working, and a fragment is a question about a page rather than about
//! whether a page is there.

use crate::{Result, Site, SiteError};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

/// Where the links that point at nothing were found: the target, and every
/// page asking for it.
type Broken = BTreeMap<String, BTreeSet<String>>;

impl Site {
    pub(crate) fn check_links(&self, args: &[String]) -> Result<()> {
        let root = match args.first() {
            Some(directory) => self.root.join(directory),
            None => self.root.join("public"),
        };

        if !root.is_dir() {
            return Err(Box::new(SiteError(format!(
                "{} is not a directory; build the site first",
                root.display()
            ))));
        }

        let mut pages = Vec::new();
        collect_html(&root, &mut pages)?;

        let mut directories = Directories::default();
        let mut broken: Broken = BTreeMap::new();

        for page in &pages {
            let html = fs::read_to_string(page)?;
            let from = relative(&root, page);

            for link in links(&html) {
                let Some(target) = local_target(&link) else {
                    continue;
                };

                let path = resolve(&root, page, &target);
                if !directories.holds(&root, &path) {
                    broken.entry(link).or_default().insert(from.clone());
                }
            }
        }

        if broken.is_empty() {
            println!("{} pages checked, every local link resolves", pages.len());
            return Ok(());
        }

        for (target, from) in &broken {
            println!("{target}");
            for page in from {
                println!("    in {page}");
            }
        }

        Err(Box::new(SiteError(if broken.len() == 1 {
            "one link points at nothing".to_string()
        } else {
            format!("{} links point at nothing", broken.len())
        })))
    }
}

fn collect_html(directory: &Path, found: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_html(&path, found)?;
        } else if path
            .extension()
            .is_some_and(|extension| extension == "html")
        {
            found.push(path);
        }
    }

    Ok(())
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// Every `href`, `src` and `poster` in the document, in the order they appear.
///
/// A scan rather than a parse: the question is only which URLs the page asks
/// for, and the answer is the same whether or not the tree around them is
/// well-formed. It does have to know where a tag begins and ends, though --
/// `href="/prose"` typed in the middle of a sentence is a sentence, and a
/// `>` inside a quoted attribute is not the end of the tag it sits in.
fn links(html: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut rest = html;

    while let Some(open) = rest.find('<') {
        let tag = &rest[open + 1..];
        let end = tag_end(tag);
        found.extend(attribute_links(&tag[..end]));
        rest = &tag[end..];
    }

    found
}

/// Where the tag's `>` is, ignoring the ones inside quoted values.
fn tag_end(tag: &str) -> usize {
    let mut quote = None;

    for (index, character) in tag.char_indices() {
        match (quote, character) {
            (None, '"' | '\'') => quote = Some(character),
            (Some(open), _) if character == open => quote = None,
            (None, '>') => return index,
            _ => {}
        }
    }

    tag.len()
}

/// The URLs one tag asks for.
fn attribute_links(tag: &str) -> Vec<String> {
    const ATTRIBUTES: [&str; 3] = ["href", "src", "poster"];

    let bytes = tag.as_bytes();
    let mut found = Vec::new();

    for (index, _) in tag.match_indices('=') {
        let named = ATTRIBUTES.iter().any(|name| {
            let Some(start) = index.checked_sub(name.len()) else {
                return false;
            };

            tag.get(start..index) == Some(*name)
                // `href=` and not `data-href=` or `xlink:href=`: what comes
                // before has to be the whitespace that ends the last word.
                && start
                    .checked_sub(1)
                    .is_none_or(|before| bytes[before].is_ascii_whitespace())
        });

        if !named {
            continue;
        }

        let value = &tag[index + 1..];
        let Some(quote) = value.chars().next() else {
            continue;
        };
        if quote != '"' && quote != '\'' {
            continue;
        }

        if let Some(end) = value[1..].find(quote) {
            found.push(value[1..=end].to_string());
        }
    }

    found
}

/// The path a link asks for, or `None` when it is not this site's to answer.
fn local_target(link: &str) -> Option<String> {
    let link = link.trim();

    if link.is_empty() || link.starts_with('#') || link.starts_with("//") {
        return None;
    }

    // Any scheme at all: http, mailto, data, javascript, tel. A Windows drive
    // letter cannot appear here, so a colon before the first slash is a scheme.
    if let Some(colon) = link.find(':')
        && link[..colon]
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '-')
        && !link[..colon].contains('/')
    {
        return None;
    }

    let path = link
        .split('#')
        .next()
        .unwrap_or_default()
        .split('?')
        .next()
        .unwrap_or_default();

    if path.is_empty() {
        return None;
    }

    Some(percent_decoded(path))
}

/// `%20` and friends, since what is on disk is the decoded name.
fn percent_decoded(path: &str) -> String {
    let bytes = path.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] == b'%'
            && let Some(hex) = path.get(index + 1..index + 3)
            && let Ok(byte) = u8::from_str_radix(hex, 16)
        {
            out.push(byte);
            index += 3;
            continue;
        }

        out.push(bytes[index]);
        index += 1;
    }

    String::from_utf8_lossy(&out).into_owned()
}

/// Where a link points, as a path under the site root.
///
/// A link that starts with a slash is from the root; anything else is from the
/// directory the page is in. `..` is resolved here rather than by the
/// filesystem so that it cannot climb out of the site.
fn resolve(root: &Path, page: &Path, target: &str) -> PathBuf {
    let mut path = if target.starts_with('/') {
        root.to_path_buf()
    } else {
        page.parent().unwrap_or(root).to_path_buf()
    };

    for segment in target.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                if path != root {
                    path.pop();
                }
            }
            segment => path.push(segment),
        }
    }

    path
}

/// The directory listings seen so far, so that a page of fifty links into the
/// same folder reads it once.
#[derive(Default)]
struct Directories {
    entries: HashMap<PathBuf, HashSet<String>>,
}

impl Directories {
    /// Whether the site actually holds this path, by the name it is asked for.
    ///
    /// A URL can name a file, a page whose `.html` the server adds, or a
    /// directory the server answers with its `index.html`.
    fn holds(&mut self, root: &Path, path: &Path) -> bool {
        self.named(root, path)
            || self.named(root, &with_suffix(path, ".html"))
            || self.named(root, &path.join("index.html"))
    }

    /// Whether every segment of the path is in its parent's listing, spelled
    /// the way the link spells it.
    ///
    /// `path.exists()` would answer this on Linux and lie on Windows and
    /// macOS, where the filesystem does not care about case and the web server
    /// does.
    fn named(&mut self, root: &Path, path: &Path) -> bool {
        let Ok(relative) = path.strip_prefix(root) else {
            return false;
        };

        let mut at = root.to_path_buf();
        for segment in relative.components() {
            let name = segment.as_os_str().to_string_lossy().into_owned();
            if !self.listing(&at).contains(&name) {
                return false;
            }
            at.push(name);
        }

        true
    }

    fn listing(&mut self, directory: &Path) -> &HashSet<String> {
        self.entries
            .entry(directory.to_path_buf())
            .or_insert_with(|| {
                fs::read_dir(directory)
                    .map(|entries| {
                        entries
                            .filter_map(|entry| {
                                Some(entry.ok()?.file_name().to_string_lossy().into_owned())
                            })
                            .collect()
                    })
                    .unwrap_or_default()
            })
    }
}

fn with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut name = path.as_os_str().to_os_string();
    name.push(suffix);
    PathBuf::from(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process;

    #[test]
    fn reads_the_urls_a_page_asks_for() {
        let html = r#"<a href="/one">x</a><img src="two.png"><video poster='three.jpg'>"#;
        assert_eq!(links(html), ["/one", "two.png", "three.jpg"]);
    }

    #[test]
    fn is_not_fooled_by_an_attribute_that_ends_in_href() {
        // `data-href` is not a link, and neither is a word inside a sentence.
        let html = r#"<a data-href="/no" href="/yes">the href="/prose" of it</a>"#;
        assert_eq!(links(html), ["/yes"]);
    }

    #[test]
    fn is_not_fooled_by_a_bracket_inside_an_attribute() {
        let html = r#"<img alt="a > b" src="/x.png"><a href="/y">z</a>"#;
        assert_eq!(links(html), ["/x.png", "/y"]);
    }

    #[test]
    fn reads_an_svg_anchor_by_its_href_and_not_its_xlink() {
        // The graph draws its nodes as SVG anchors, and the deprecated form
        // is not the one the browser resolves.
        let html = r#"<a class="node" xlink:href="/old" href="/new"></a>"#;
        assert_eq!(links(html), ["/new"]);
    }

    #[test]
    fn leaves_other_people_s_urls_alone() {
        for link in [
            "https://example.com/x",
            "http://example.com",
            "//example.com/x",
            "mailto:someone@example.com",
            "tel:+123",
            "data:image/png;base64,AAAA",
            "javascript:void(0)",
            "#heading",
            "",
        ] {
            assert_eq!(local_target(link), None, "{link}");
        }
    }

    #[test]
    fn takes_the_path_off_a_link_and_nothing_else() {
        assert_eq!(local_target("/a/b.png#frag"), Some("/a/b.png".to_string()));
        assert_eq!(local_target("/a/b.png?v=2"), Some("/a/b.png".to_string()));
        assert_eq!(
            local_target("/misc/media/a%20b.mp4"),
            Some("/misc/media/a b.mp4".to_string())
        );
    }

    #[test]
    fn resolves_a_link_from_the_root_or_from_the_page() {
        let root = Path::new("/site");
        let page = Path::new("/site/notes/one.html");

        assert_eq!(
            resolve(root, page, "/misc/x.mp4"),
            Path::new("/site/misc/x.mp4")
        );
        assert_eq!(resolve(root, page, "x.mp4"), Path::new("/site/notes/x.mp4"));
        assert_eq!(
            resolve(root, page, "./x.mp4"),
            Path::new("/site/notes/x.mp4")
        );
        assert_eq!(resolve(root, page, "../x.mp4"), Path::new("/site/x.mp4"));
        // The bug this was written for: one directory up from /notes/ is the
        // site root, and the file is in /misc/media.
        assert_eq!(
            resolve(root, page, "../volumetrics.mp4"),
            Path::new("/site/volumetrics.mp4")
        );
    }

    #[test]
    fn cannot_be_walked_out_of_the_site() {
        let root = Path::new("/site");
        let page = Path::new("/site/index.html");
        assert_eq!(
            resolve(root, page, "../../../etc/passwd"),
            Path::new("/site/etc/passwd")
        );
    }

    /// A directory of files, cleaned up whatever the test does.
    struct Fixture {
        dir: PathBuf,
    }

    impl Fixture {
        fn new(name: &str, files: &[(&str, &str)]) -> Self {
            let dir = std::env::temp_dir().join(format!("site-links-{name}-{}", process::id()));
            let _ = fs::remove_dir_all(&dir);

            for (path, contents) in files {
                let file = dir.join(path);
                fs::create_dir_all(file.parent().unwrap()).unwrap();
                fs::write(file, contents).unwrap();
            }

            Self { dir }
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }

    #[test]
    fn counts_a_page_a_file_and_a_directory_as_there() {
        let fixture = Fixture::new(
            "there",
            &[
                ("index.html", ""),
                ("notes/one.html", ""),
                ("blog/index.html", ""),
                ("misc/media/x.mp4", ""),
            ],
        );
        let root = &fixture.dir;
        let mut directories = Directories::default();

        // A file by its own name, a page without its extension, a directory
        // answered by the index inside it.
        assert!(directories.holds(root, &root.join("misc/media/x.mp4")));
        assert!(directories.holds(root, &root.join("notes/one")));
        assert!(directories.holds(root, &root.join("blog")));
        assert!(!directories.holds(root, &root.join("misc/x.mp4")));
        assert!(!directories.holds(root, &root.join("notes/two")));
    }

    #[test]
    fn spells_names_the_way_the_link_spells_them() {
        // The filesystem under this test does not care about case; the one
        // serving the site does, so neither does this.
        let fixture = Fixture::new("case", &[("misc/build/x.gz", "")]);
        let root = &fixture.dir;
        let mut directories = Directories::default();

        assert!(directories.holds(root, &root.join("misc/build/x.gz")));
        assert!(!directories.holds(root, &root.join("misc/Build/x.gz")));
    }

    #[test]
    fn finds_the_link_that_points_at_nothing() {
        let fixture = Fixture::new(
            "broken",
            &[
                (
                    "notes/one.html",
                    r#"<video src="../volumetrics.mp4"></video>"#,
                ),
                ("misc/media/volumetrics.mp4", ""),
            ],
        );
        let root = &fixture.dir;

        let mut pages = Vec::new();
        collect_html(root, &mut pages).unwrap();
        let mut directories = Directories::default();

        let html = fs::read_to_string(&pages[0]).unwrap();
        let target = local_target(&links(&html)[0]).unwrap();
        let path = resolve(root, &pages[0], &target);

        assert!(!directories.holds(root, &path));
        // And with the path it meant, it is there.
        assert!(directories.holds(
            root,
            &resolve(root, &pages[0], "/misc/media/volumetrics.mp4")
        ));
    }
}
