//! Syntax-highlights every solution once, at build time.
//!
//! It used to highlight each of them twice, into `solutionN-dark.html` and
//! `solutionN-light.html` with the colours written inline on every span, and
//! `include_str!` both into the wasm. Every reader of the page downloaded two
//! coloured copies of all 205 solutions in order to read one of them in one
//! theme, and switching theme re-rendered all 205 from the wasm.
//!
//! The two copies only ever differed in their colours, so what is written now
//! is one copy of the markup, with each span carrying a short class instead of
//! a colour, and one small stylesheet giving those classes their colour in each
//! theme. Solarized uses sixteen colours, so the palette is sixteen rules long
//! however many solutions there are, and a theme switch is a CSS matter again.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use minify_html::{Cfg, minify};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Color, Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use walkdir::WalkDir;

/// Where the palette the generated markup refers to is written.
const STYLESHEET: &str = "src/shared/highlight.css";

/// The attribute Quartz puts the chosen theme in, and the value that means dark.
const DARK_SELECTOR: &str = "[saved-theme=\"dark\"]";

fn main() {
    let syntaxes = SyntaxSet::load_defaults_newlines();
    let themes = ThemeSet::load_defaults();
    let dark = &themes.themes["Solarized (dark)"];
    let light = &themes.themes["Solarized (light)"];

    let mut palette = Palette::new(dark, light);

    // Sorted, because the palette numbers colours in the order it first meets
    // them and the stylesheet has to agree with the markup. Walk order is not
    // promised to be stable across platforms; this makes the output so.
    let mut solutions: Vec<_> = WalkDir::new("./")
        .into_iter()
        .flatten()
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.into_path())
        .filter(|path| {
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            name == "solution1.rs" || name == "solution2.rs"
        })
        .collect();
    solutions.sort();

    for path in &solutions {
        if let Err(error) = highlight(&syntaxes, dark, light, &mut palette, path) {
            // A solution that will not highlight should not take the build with
            // it, but it should be loud: it is about to ship as a blank panel.
            println!(
                "cargo::warning=could not highlight {}: {error}",
                path.display()
            );
        }
    }

    if let Err(error) = fs::write(STYLESHEET, palette.stylesheet()) {
        panic!("could not write {STYLESHEET}: {error}");
    }
}

fn highlight(
    syntaxes: &SyntaxSet,
    dark: &Theme,
    light: &Theme,
    palette: &mut Palette,
    path: &Path,
) -> Result<(), String> {
    let source = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let syntax = syntaxes
        .find_syntax_by_extension("rs")
        .ok_or("no Rust syntax definition")?;

    let mut dark = HighlightLines::new(syntax, dark);
    let mut light = HighlightLines::new(syntax, light);
    let mut html = String::from("<pre class=\"aoc-code\">");

    // The highlighter hands back a span per parse step, so `-> String ` arrives
    // as three spans that all want the same colour. Runs are joined rather than
    // written out one at a time: the tag is longer than most of the text in it.
    let mut run: Option<(usize, String)> = None;

    for line in LinesWithEndings::from(&source) {
        // The two runs differ only in the colours they attach: the syntax
        // decides where one span ends and the next begins, and the syntax is
        // the same both times. If that ever stops holding, the zip below would
        // silently colour spans by their neighbours, so check rather than trust.
        let in_dark = dark
            .highlight_line(line, syntaxes)
            .map_err(|error| error.to_string())?;
        let in_light = light
            .highlight_line(line, syntaxes)
            .map_err(|error| error.to_string())?;

        if in_dark.len() != in_light.len() {
            return Err(format!(
                "the two themes split a line differently ({} spans against {})",
                in_dark.len(),
                in_light.len()
            ));
        }

        for ((dark_style, text), (light_style, _)) in in_dark.iter().zip(in_light.iter()) {
            let class = palette.intern(dark_style.foreground, light_style.foreground);

            match &mut run {
                Some((open, run_text)) if *open == class => run_text.push_str(text),
                Some(_) => {
                    flush(&mut html, run.take());
                    run = Some((class, text.to_string()));
                }
                None => run = Some((class, text.to_string())),
            }
        }
    }

    flush(&mut html, run.take());
    html.push_str("</pre>");

    let minified = minify(html.as_bytes(), &minify_config());
    let minified = String::from_utf8(minified).map_err(|error| error.to_string())?;

    let output = path.with_file_name(format!(
        "{}.html",
        path.file_stem().unwrap_or_default().to_string_lossy()
    ));
    fs::write(output, minified).map_err(|error| error.to_string())
}

/// Writes out one run of same-coloured text.
fn flush(html: &mut String, run: Option<(usize, String)>) {
    let Some((class, text)) = run else { return };
    let _ = write!(html, "<span class=\"c{class}\">{}</span>", escape(&text));
}

fn minify_config() -> Cfg {
    Cfg {
        minify_doctype: true,
        keep_closing_tags: false,
        keep_comments: false,
        minify_css: true,
        minify_js: true,
        remove_bangs: true,
        remove_processing_instructions: true,
        allow_noncompliant_unquoted_attribute_values: true,
        keep_html_and_head_opening_tags: false,
        keep_input_type_text_attr: false,
        keep_ssi_comments: false,
        preserve_brace_template_syntax: false,
        preserve_chevron_percent_template_syntax: false,
        allow_optimal_entities: true,
        allow_removing_spaces_between_attributes: true,
    }
}

/// The colours the markup refers to by number.
///
/// A pair, because a class has to name the same token in both themes at once:
/// `c3` is whatever Solarized dark paints a keyword and whatever Solarized
/// light paints it, and the stylesheet gives both.
struct Palette {
    indices: HashMap<(u32, u32), usize>,
    colours: Vec<(Color, Color)>,
    dark_background: Color,
    light_background: Color,
    dark_foreground: Color,
    light_foreground: Color,
}

impl Palette {
    fn new(dark: &Theme, light: &Theme) -> Self {
        let background = |theme: &Theme| theme.settings.background.unwrap_or(Color::WHITE);
        let foreground = |theme: &Theme| theme.settings.foreground.unwrap_or(Color::BLACK);

        Self {
            indices: HashMap::new(),
            colours: Vec::new(),
            dark_background: background(dark),
            light_background: background(light),
            dark_foreground: foreground(dark),
            light_foreground: foreground(light),
        }
    }

    fn intern(&mut self, dark: Color, light: Color) -> usize {
        let key = (key(dark), key(light));
        if let Some(index) = self.indices.get(&key) {
            return *index;
        }

        let index = self.colours.len();
        self.colours.push((dark, light));
        self.indices.insert(key, index);
        index
    }

    fn stylesheet(&self) -> String {
        let mut css = String::from(
            "/* Written by wasm/adventofcode/build.rs. The classes here are the\n\
             ones the highlighted solutions carry. */\n",
        );

        let _ = write!(
            css,
            ".aoc-code{{background:{};color:{}}}",
            hex(self.light_background),
            hex(self.light_foreground)
        );
        let _ = write!(
            css,
            "{DARK_SELECTOR} .aoc-code{{background:{};color:{}}}",
            hex(self.dark_background),
            hex(self.dark_foreground)
        );

        for (index, (_, light)) in self.colours.iter().enumerate() {
            let _ = write!(css, ".aoc-code .c{index}{{color:{}}}", hex(*light));
        }
        for (index, (dark, _)) in self.colours.iter().enumerate() {
            let _ = write!(
                css,
                "{DARK_SELECTOR} .aoc-code .c{index}{{color:{}}}",
                hex(*dark)
            );
        }

        css
    }
}

/// Alpha is not written out, so two colours that differ only there would
/// otherwise take two palette entries that render the same.
fn key(colour: Color) -> u32 {
    u32::from_be_bytes([0, colour.r, colour.g, colour.b])
}

fn hex(colour: Color) -> String {
    format!("#{:02x}{:02x}{:02x}", colour.r, colour.g, colour.b)
}

fn escape(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            other => escaped.push(other),
        }
    }
    escaped
}
