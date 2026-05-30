use std::fs;
use std::path::{Path, PathBuf};

use crate::{Result, Site, SiteError};
use music21_rs::tuningsystem::TWELVE_TONE_NAMES;
use recursive_ji_core::{generated_audio_files, generated_media_text_files, generated_text_files};
use std::collections::HashMap;

const DEFAULT_AUDIO_OUTPUT_DIR: &str = "content/misc/media";
const DEFAULT_TEXT_OUTPUT_DIR: &str = "content/misc/plaintext";

pub(crate) fn generate(site: &Site, args: &[String]) -> Result<()> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        return Ok(());
    }

    if args.len() > 1 {
        return Err(Box::new(SiteError::new(
            "recursive-ji-music accepts at most one output directory",
        )));
    }

    let override_dir = args.first().map(|path| site.root.join(path));
    let audio_output_dir = override_dir
        .clone()
        .unwrap_or_else(|| site.root.join(DEFAULT_AUDIO_OUTPUT_DIR));
    let text_output_dir = override_dir.unwrap_or_else(|| site.root.join(DEFAULT_TEXT_OUTPUT_DIR));

    fs::create_dir_all(&audio_output_dir)?;
    fs::create_dir_all(&text_output_dir)?;

    for file in generated_audio_files()? {
        let path = audio_output_dir.join(file.name);
        fs::write(&path, file.bytes)?;
        println!("wrote {}", relative_to_root(site, &path).display());
    }

    for file in generated_media_text_files() {
        let path = audio_output_dir.join(file.name);
        fs::write(&path, file.text)?;
        println!("wrote {}", relative_to_root(site, &path).display());
    }

    for file in generated_text_files() {
        let path = text_output_dir.join(file.name);
        fs::write(&path, file.text)?;
        println!("wrote {}", relative_to_root(site, &path).display());
    }

    // Update the blog post table in-place so site `generate` keeps the HTML in
    // the post in sync with the generated CSV.
    let csv_path = text_output_dir.join("recursive-ji-frequencies.csv");
    let post_path = site.root.join("content/blog/recursive-just-intonation.md");
    if csv_path.exists() && post_path.exists() {
        if let Err(e) = update_recursive_ji_table(&post_path, &csv_path) {
            eprintln!("warning: failed to update recursive JI table: {e}");
        }
    }

    Ok(())
}

fn relative_to_root(site: &Site, path: &Path) -> PathBuf {
    path.strip_prefix(&site.root).unwrap_or(path).to_path_buf()
}

fn print_help() {
    println!(
        "\
recursive-ji-music

Usage:
  cargo run --manifest-path tools/site/Cargo.toml -- recursive-ji-music [OUTPUT]

Defaults:
  WAV output   content/misc/media
  Text output  content/misc/plaintext

With OUTPUT, writes all generated files under that directory.

Audio files:
  twelve-tet-progression.wav
  twelve-tet-sine-progression.wav
  twelve-tet-c-drone-progression.wav
  fixed-c-ji-progression.wav
  fixed-c-ji-sine-progression.wav
  fixed-c-ji-c-drone-progression.wav
  recursive-ji-progression.wav
  recursive-ji-sine-progression.wav
  recursive-ji-c-drone-progression.wav
  recursive-ji-note-splits.wav
  twelve-tet-rooted-ji-progression.wav
  recursive-just-intonation-composition.wav
  mozart-dies-irae-recursive-just-intonation-piano.wav

Media text files:
  recursive-just-intonation-composition.musicxml

Text files:
  recursive-ji-frequencies.csv
"
    );
}

fn update_recursive_ji_table(post: &PathBuf, csv: &PathBuf) -> Result<()> {
    let csv_text = fs::read_to_string(csv)?;

    // Parse CSV: tuning,chord,note,frequency_hz,cents_vs_12_tet,cents_vs_fixed_c_ji
    let mut map: HashMap<(String, String), (String, String)> = HashMap::new();
    for (i, line) in csv_text.lines().enumerate() {
        if i == 0 || line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 6 {
            continue;
        }
        let tuning = parts[0].to_string();
        let chord = parts[1].to_string();
        let note = parts[2].to_string();
        let freq = parts[3].to_string();
        let cents_vs_tet = parts[4].to_string();
        map.insert(
            (tuning, format!("{}|{}", chord, note)),
            (freq, cents_vs_tet),
        );
    }

    let source = fs::read_to_string(post)?;
    let lines: Vec<&str> = source.lines().collect();

    // Find the start of the table (line that contains "| local root |")
    let start_idx = lines.iter().position(|l| l.contains("| local root |"));
    let start_idx = match start_idx {
        Some(i) => i,
        None => return Ok(()),
    };

    // Find the end of the table: first line after start that does NOT start with '|'
    let mut end_idx = start_idx + 1;
    while end_idx < lines.len() && lines[end_idx].trim_start().starts_with('|') {
        end_idx += 1;
    }

    // Build new table: columns = TWELVE_TONE_NAMES, rows = TWELVE_TONE_NAMES
    let mut table = String::new();
    // header
    table.push_str("| local root |");
    for name in TWELVE_TONE_NAMES.iter() {
        table.push_str(&format!(" {} |", format!("`{}`", name)));
    }
    table.push('\n');
    // separator
    table.push_str("| ---------- |");
    for _ in TWELVE_TONE_NAMES.iter() {
        table.push_str(" -------------------------------------------------------------------------------------------------------------------------------------------: |");
    }
    table.push('\n');

    for root in TWELVE_TONE_NAMES.iter() {
        table.push_str(&format!("| {root} |", root = root));
        for col in TWELVE_TONE_NAMES.iter() {
            let key = (
                "Recursive just intonation".to_string(),
                format!("{}|{}", root, col),
            );
            let (freq, cents) = map
                .get(&key)
                .cloned()
                .unwrap_or(("".to_string(), "".to_string()));
            let class = note_to_class(col);
            let data_note = html_escape(col);
            let cell = if freq.is_empty() {
                " ".to_string()
            } else {
                format!("<span class=\"recursive-note-cell {class}\" data-note=\"{data_note}\"><code>{freq} Hz</code><small class=\"tet-cents\">{cents} cents</small></span>", class=class, data_note=data_note, freq=freq, cents=cents)
            };
            table.push_str(&format!(" {cell} |", cell = cell));
        }
        table.push('\n');
    }

    // Replace lines[start_idx..end_idx] with table
    let prefix = lines[..start_idx].join("\n");
    let suffix = if end_idx < lines.len() {
        lines[end_idx..].join("\n")
    } else {
        "".to_string()
    };
    let final_text = if suffix.is_empty() {
        format!("{}\n{}", prefix, table)
    } else {
        format!("{}\n{}\n{}", prefix, table, suffix)
    };
    fs::write(post, final_text)?;
    Ok(())
}

fn note_to_class(name: &str) -> &'static str {
    match name {
        "C" => "note-c",
        "C#/Db" => "note-c-sharp",
        "D" => "note-d",
        "D#/Eb" => "note-d-sharp",
        "E" => "note-e",
        "F" => "note-f",
        "F#/Gb" => "note-f-sharp",
        "G" => "note-g",
        "G#/Ab" => "note-g-sharp",
        "A" => "note-a",
        "A#/Bb" => "note-a-sharp",
        "B" => "note-b",
        _ => "note-c",
    }
}

fn html_escape(s: &str) -> String {
    s.replace('"', "'")
}
