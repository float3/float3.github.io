use std::fs;
use std::path::Path;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
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
            let code = match fs::read_to_string(path) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let syntax = ss.find_syntax_by_extension("rs").unwrap();
            let html = highlighted_html_for_string(&code, &ss, syntax, theme);
            let out = path.with_file_name(format!(
                "{}.html",
                path.file_stem().unwrap().to_string_lossy()
            ));
            fs::write(out, html).unwrap();
        }
    }
}
