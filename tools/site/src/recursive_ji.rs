use std::fs;
use std::path::{Path, PathBuf};

use crate::{Result, Site, SiteError};
use recursive_ji_core::{generated_audio_files, generated_text_files};

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

    for file in generated_text_files() {
        let path = text_output_dir.join(file.name);
        fs::write(&path, file.text)?;
        println!("wrote {}", relative_to_root(site, &path).display());
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

Text files:
  recursive-ji-frequencies.csv
"
    );
}
