use crate::{os_args, Result, Site};
use std::fs;

impl Site {
    pub(crate) fn generate(&self) -> Result<()> {
        crate::recursive_ji::generate(self, &[])?;
        self.links()?;
        self.indices()?;
        self.generate_chords()
    }

    pub(crate) fn links(&self) -> Result<()> {
        let output_dir = self.root.join("content/misc/plaintext");
        fs::create_dir_all(&output_dir)?;

        self.collect_links(
            "content/notes/talks.md",
            "content/misc/plaintext/talks.txt",
            false,
        )?;
        self.collect_links(
            "content/notes/blogs.md",
            "content/misc/plaintext/blogs.txt",
            false,
        )?;
        self.collect_links(
            "content/notes/graphics-resources.md",
            "content/misc/plaintext/graphics-resources.txt",
            true,
        )
    }

    fn collect_links(&self, input: &str, output: &str, unique: bool) -> Result<()> {
        let source = fs::read_to_string(self.root.join(input))?;
        let mut links = extract_urls(&source);

        if unique {
            links.sort();
            links.dedup();
        }

        let mut body = String::new();
        for link in links {
            body.push_str(&link);
            body.push('\n');
        }

        fs::write(self.root.join(output), body)?;
        Ok(())
    }

    /// The directories under `content/misc` that get a generated listing.
    ///
    /// A gallery is nothing but a directory in this list plus a page that names
    /// it, which is the point of it being a list: another one is two lines and
    /// a folder of pictures.
    const INDICES: &'static [(&'static str, &'static str)] = &[
        ("media", "media"),
        ("blobs", "blobs"),
        ("plaintext", "plaintext"),
        ("trolley", "trolley"),
        ("guesswedoing", "guess we doing"),
    ];

    pub(crate) fn indices(&self) -> Result<()> {
        for (dir, title) in Self::INDICES {
            self.generate_index(dir, title)?;
        }
        Ok(())
    }

    /// The heading a listed directory gets, or `None` for a directory that is
    /// not one of the galleries.
    pub(crate) fn index_title(dir: &str) -> Option<&'static str> {
        Self::INDICES
            .iter()
            .find_map(|(name, title)| (*name == dir).then_some(*title))
    }

    /// The listed directories, for an error message that names them rather than
    /// leaving the reader to find this list.
    pub(crate) fn index_names() -> Vec<&'static str> {
        Self::INDICES.iter().map(|(name, _)| *name).collect()
    }

    pub(crate) fn generate_index(&self, dir: &str, title: &str) -> Result<usize> {
        let base = self.root.join("content/misc").join(dir);
        // A gallery gets listed here before its first file lands as often as
        // after, and an empty one is a page that says so rather than an error.
        if !base.is_dir() {
            fs::create_dir_all(&base)?;
        }

        let mut entries = Vec::new();

        for entry in fs::read_dir(&base)? {
            let entry = entry?;
            if entry.path().is_dir() {
                continue;
            }

            let name = entry.file_name().to_string_lossy().into_owned();
            if is_gallery_item(&name) {
                entries.push(name);
            }
        }

        entries.sort();

        let mut body = format!("---\ntitle: {title}\ntags:\n  - list\n---\n\n");
        for (index, entry) in entries.iter().enumerate() {
            let continuation = if index + 1 == entries.len() {
                ""
            } else {
                " \\"
            };
            body.push_str(&format!("[{entry}](/misc/{dir}/{entry}){continuation}\n"));
        }

        fs::write(base.join("index.md"), body)?;
        fs::write(base.join(MANIFEST), manifest(&entries))?;
        Ok(entries.len())
    }

    fn generate_chords(&self) -> Result<()> {
        let dir = self.root.join("wasm/tuningplayground");
        self.run(&dir, "cargo", &os_args(&["run", "-p", "chord_generator"]))
    }
}

/// The machine-readable half of a listing.
///
/// The galleries read this rather than the markdown beside it: they want the
/// filenames, and recovering those from rendered links would mean parsing a
/// format built for people to read in order to get at something never meant to
/// be hidden in the first place.
const MANIFEST: &str = "index.json";

/// Whether a filename in a gallery directory is one of the gallery's items.
///
/// The two generated files are not, and neither is anything starting with a
/// dot: that is either the staging directory of a `normalize-gallery` that died
/// partway or something the operating system left behind, and listing it would
/// put a broken link on the page. `normalize-gallery` filters by this too, so
/// the numbering and the manifest cannot disagree about what is in a gallery.
pub(crate) fn is_gallery_item(name: &str) -> bool {
    !name.starts_with('.') && name != "index.md" && name != MANIFEST
}

/// A JSON array of filenames.
///
/// Written by hand rather than with a serialiser: quoting a list of strings is
/// the whole of the job, and the escape rules for a JSON string are shorter
/// than the argument for taking on a dependency to apply them.
fn manifest(entries: &[String]) -> String {
    let mut out = String::from("[");

    for (index, entry) in entries.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push('"');
        for ch in entry.chars() {
            match ch {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                // Control characters cannot appear raw inside a JSON string.
                c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                c => out.push(c),
            }
        }
        out.push('"');
    }

    out.push_str("]\n");
    out
}

fn extract_urls(source: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut offset = 0;

    while let Some(start) = find_next_url(&source[offset..]) {
        let absolute_start = offset + start;
        let mut absolute_end = absolute_start;

        for (index, ch) in source[absolute_start..].char_indices() {
            if is_url_delimiter(ch) {
                break;
            }
            absolute_end = absolute_start + index + ch.len_utf8();
        }

        if absolute_end > absolute_start {
            let link = source[absolute_start..absolute_end]
                .trim_end_matches(['.', ',', ';'])
                .to_string();
            links.push(link);
        }

        offset = absolute_end.max(absolute_start + 1);
    }

    links
}

fn find_next_url(source: &str) -> Option<usize> {
    match (source.find("http://"), source.find("https://")) {
        (Some(http), Some(https)) => Some(http.min(https)),
        (Some(http), None) => Some(http),
        (None, Some(https)) => Some(https),
        (None, None) => None,
    }
}

fn is_url_delimiter(ch: char) -> bool {
    ch.is_whitespace() || matches!(ch, '<' | '>' | '"' | '\'' | '`' | '[' | ']' | '(' | ')')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_media_and_nothing_the_build_or_the_os_dropped_in() {
        assert!(is_gallery_item("00.jpg"));
        assert!(is_gallery_item("63.mp4"));
        assert!(!is_gallery_item("index.md"));
        assert!(!is_gallery_item("index.json"));
        assert!(!is_gallery_item(".normalize"));
        assert!(!is_gallery_item(".DS_Store"));
    }

    #[test]
    fn finds_a_listed_gallery_by_its_directory() {
        assert_eq!(Site::index_title("guesswedoing"), Some("guess we doing"));
        assert_eq!(Site::index_title("trolley"), Some("trolley"));
        assert_eq!(Site::index_title("not-a-gallery"), None);
        assert!(Site::index_names().contains(&"trolley"));
    }

    #[test]
    fn writes_an_empty_manifest_for_an_empty_gallery() {
        assert_eq!(manifest(&[]), "[]\n");
    }

    #[test]
    fn quotes_filenames_into_the_manifest() {
        let entries = vec!["00.jpg".to_string(), "63.mp4".to_string()];
        assert_eq!(manifest(&entries), "[\"00.jpg\",\"63.mp4\"]\n");
    }

    #[test]
    fn escapes_filenames_that_would_break_the_json() {
        let entries = vec!["a\"b.jpg".to_string(), "c\\d.png".to_string()];
        assert_eq!(manifest(&entries), "[\"a\\\"b.jpg\",\"c\\\\d.png\"]\n");
    }

    #[test]
    fn extracts_markdown_urls_without_trailing_punctuation() {
        let links =
            extract_urls(r#"see [one](https://example.com/a?b=1), "http://example.org/x"."#);

        assert_eq!(
            links,
            vec![
                "https://example.com/a?b=1".to_string(),
                "http://example.org/x".to_string()
            ]
        );
    }
}
