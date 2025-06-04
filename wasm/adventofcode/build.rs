use minify_html::{minify, Cfg};
use std::fs;
use std::ops::ControlFlow;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_file;
use syntect::parsing::SyntaxSet;
use walkdir::WalkDir;

fn main() {
    generate_html_from_sources();
}

fn generate_html_from_sources() {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let darktheme = &ts.themes["Solarized (dark)"];
    let lightheme = &ts.themes["Solarized (light)"];

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
            if let ControlFlow::Break(_) = generate_file(&ss, darktheme, path, "dark") {
                continue;
            }

            if let ControlFlow::Break(_) = generate_file(&ss, lightheme, path, "light") {
                continue;
            }
        }
    }
}

fn generate_file(
    ss: &SyntaxSet,
    darktheme: &syntect::highlighting::Theme,
    path: &std::path::Path,
    theme: &str,
) -> ControlFlow<()> {
    let path_buf = path.with_file_name(format!(
        "{}-{}.html",
        path.file_stem().unwrap().to_string_lossy(),
        theme
    ));
    let html = match highlighted_html_for_file(path, ss, darktheme) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error highlighting file {}: {}", path.display(), e);
            return ControlFlow::Break(());
        }
    };

    let cfg = Cfg {
        minify_doctype: true,
        keep_closing_tags: false,
        keep_comments: false,
        minify_css: true,
        minify_js: true,
        remove_bangs: true,
        remove_processing_instructions: true,
        allow_noncompliant_unquoted_attribute_values: true,
        keep_html_and_head_opening_tags: false,
        // keep_spaces_between_attributes: false,
        keep_input_type_text_attr: false,
        keep_ssi_comments: false,
        preserve_brace_template_syntax: false,
        preserve_chevron_percent_template_syntax: false,
        allow_optimal_entities: true,
        allow_removing_spaces_between_attributes: true,
    };
    let minified = minify(html.as_bytes(), &cfg);
    let minified_html = String::from_utf8(minified).unwrap();

    fs::write(path_buf, minified_html).unwrap();

    ControlFlow::Continue(())
}
