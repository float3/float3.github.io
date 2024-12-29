use std::fs;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_file;
use syntect::parsing::SyntaxSet;
use walkdir::WalkDir;

fn main() {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];

    for entry in WalkDir::new("./") {
        let entry = if let Ok(e) = entry { e } else { continue };

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        let filename = match path.file_name() {
            Some(name) => name.to_string_lossy(),
            None => continue,
        };
        if filename == "solution1.rs" || filename == "solution2.rs" {
            let out = path.with_file_name(format!(
                "{}.html",
                path.file_stem().unwrap().to_string_lossy()
            ));

            let html = match highlighted_html_for_file(path, &ss, theme) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error highlighting file {}: {}", path.display(), e);
                    continue;
                }
            };

            fs::write(out, html).unwrap();
        }
    }
}
