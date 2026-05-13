use crate::{os_args, Result, Site};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

impl Site {
    pub(crate) fn generate(&self) -> Result<()> {
        self.links()?;
        self.indices()?;
        self.generate_chords()?;
        self.dates("content")
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

    pub(crate) fn indices(&self) -> Result<()> {
        self.generate_index("media", "media")?;
        self.generate_index("blobs", "blobs")?;
        self.generate_index("plaintext", "plaintext")?;
        self.generate_index("trolley", "trolley")?;
        Ok(())
    }

    fn generate_index(&self, dir: &str, title: &str) -> Result<usize> {
        let base = self.root.join("content/misc").join(dir);
        let mut entries = Vec::new();

        for entry in fs::read_dir(&base)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().into_owned();
            if name != "index.md" {
                entries.push(name);
            }
        }

        entries.sort();

        let existing_dates = existing_date_metadata(&base.join("index.md"))?;
        let mut body = format!("---\ntitle: {title}\n");
        for line in existing_dates {
            body.push_str(&line);
            body.push('\n');
        }
        body.push_str("tags:\n  - list\n---\n\n");
        for (index, entry) in entries.iter().enumerate() {
            let continuation = if index + 1 == entries.len() {
                ""
            } else {
                " \\"
            };
            body.push_str(&format!("[{entry}](/misc/{dir}/{entry}){continuation}\n"));
        }

        fs::write(base.join("index.md"), body)?;
        Ok(entries.len())
    }

    fn generate_chords(&self) -> Result<()> {
        let dir = self.root.join("wasm/tuningplayground");
        self.run(&dir, "cargo", &os_args(&["run", "-p", "chord_generator"]))
    }
    pub(crate) fn dates(&self, target: &str) -> Result<()> {
        if self.ci {
            let shallow = self.output_optional(
                &self.root,
                "git",
                &os_args(&["rev-parse", "--is-shallow-repository"]),
            )?;
            if shallow.as_deref() == Some("true") {
                self.run(&self.root, "git", &os_args(&["fetch", "--unshallow"]))?;
            }
        }

        let mut files = Vec::new();
        collect_markdown_files(&self.root.join(target), &mut files)?;
        files.sort();

        for file in files {
            self.update_dates_for_file(&file)?;
        }

        Ok(())
    }

    fn update_dates_for_file(&self, file: &Path) -> Result<()> {
        let relative = self.relative_git_path(file)?;
        let created = self.git_iso_date(&os_args(&[
            "log",
            "--diff-filter=A",
            "--follow",
            "--format=%aI",
            "-1",
            "--",
            relative.as_str(),
        ]))?;

        let Some(created) = created else {
            self.warn(&format!("skipping untracked markdown file: {relative}"));
            return Ok(());
        };

        let updated = self
            .git_iso_date(&os_args(&[
                "log",
                "--invert-grep",
                "--grep=generate",
                "-1",
                "--format=%aI",
                "--",
                relative.as_str(),
            ]))?
            .unwrap_or_else(|| created.clone());

        let source = fs::read_to_string(file)?;
        let mut body = String::new();
        let mut inserted = false;

        for line in source.lines() {
            if line.starts_with("date:") || line.starts_with("updated:") {
                continue;
            }

            body.push_str(line);
            body.push('\n');

            if !inserted && line.starts_with("title:") {
                body.push_str(&format!("date: {created}\nupdated: {updated}\n"));
                inserted = true;
            }
        }

        if !inserted {
            body = format!("date: {created}\nupdated: {updated}\n\n{body}");
        }

        fs::write(file, body)?;
        Ok(())
    }
}

fn existing_date_metadata(path: &Path) -> Result<Vec<String>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let source = fs::read_to_string(path)?;
    Ok(source
        .lines()
        .filter(|line| line.starts_with("date:") || line.starts_with("updated:"))
        .map(str::to_string)
        .collect())
}

fn collect_markdown_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_markdown_files(&path, files)?;
        } else if path
            .extension()
            .is_some_and(|extension| extension == OsStr::new("md"))
        {
            files.push(path);
        }
    }

    Ok(())
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
