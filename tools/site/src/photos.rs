use crate::{Result, Site, SiteError, fail};
use image::codecs::jpeg::JpegEncoder;
use image::{ImageReader, Rgb, RgbImage, RgbaImage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

struct ProcessPhotosOptions {
    input: PathBuf,
    output: PathBuf,
    manifest: PathBuf,
    quality: u8,
    dry_run: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GalleryEntry {
    src: String,
    meta: String,
    tags: Vec<String>,
    width: u32,
    height: u32,
}

pub(crate) fn process(site: &Site, args: &[String]) -> Result<()> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_process_help();
        return Ok(());
    }

    let options = parse_process_options(site, args)?;
    site.ensure_photo_input_is_private(&options.input)?;
    fs::create_dir_all(&options.input)?;
    fs::create_dir_all(&options.output)?;

    let mut photos = Vec::new();
    collect_photo_files(&options.input, &mut photos)?;
    photos.sort();

    if photos.is_empty() {
        site.warn(&format!(
            "no source photos found in {}; drop originals there and rerun process-photos",
            options.input.display()
        ));
        return Ok(());
    }

    let mut entries = Vec::new();
    let mut names = HashMap::<String, usize>::new();
    let existing_entries = load_existing_gallery_manifest(&options.manifest)?;

    for photo in photos {
        let relative = photo.strip_prefix(&options.input).unwrap_or(&photo);
        let stem = photo
            .file_stem()
            .and_then(OsStr::to_str)
            .map_or("photo", |value| value);
        let base = sanitize_file_stem(stem);
        let count = names.entry(base.clone()).or_insert(0);
        *count += 1;
        let file_name = if *count == 1 {
            format!("{base}.jpg")
        } else {
            format!("{base}-{count}.jpg")
        };
        let target = options.output.join(&file_name);

        println!(
            "process {} -> {}",
            relative.display(),
            target.strip_prefix(&site.root).unwrap_or(&target).display()
        );

        let (width, height) = if options.dry_run {
            (0, 0)
        } else {
            process_photo_file(&photo, &target, options.quality)?
        };

        let meta = relative.to_string_lossy().replace('\\', "/");
        let tags = existing_entries
            .get(&meta)
            .map(|entry| entry.tags.clone())
            .unwrap_or_default();

        println!(
            "  tags: {}",
            if tags.is_empty() {
                "[manual later]".to_string()
            } else {
                tags.join(", ")
            }
        );

        entries.push(GalleryEntry {
            src: format!("/photography/gallery/{file_name}"),
            meta,
            tags,
            width,
            height,
        });
    }

    if !options.dry_run {
        write_gallery_manifest(&options.manifest, &entries)?;
    }

    Ok(())
}

fn parse_process_options(site: &Site, args: &[String]) -> Result<ProcessPhotosOptions> {
    let mut positionals = Vec::new();
    let mut quality = 92_u8;
    let mut manifest = site.root.join("content/photography/gallery.json");
    let mut dry_run = false;
    let mut rest = args.iter();

    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--quality" => {
                let value = rest
                    .next()
                    .ok_or_else(|| SiteError::new("--quality requires a value"))?;
                quality = value.parse::<u8>().map_err(|source| {
                    SiteError::new(format!("invalid quality {value:?}: {source}"))
                })?;
            }
            "--manifest" => {
                let value = rest
                    .next()
                    .ok_or_else(|| SiteError::new("--manifest requires a path"))?;
                manifest = site.resolve_path(value);
            }
            "--dry-run" => dry_run = true,
            value if value.starts_with('-') => {
                return fail(format!("unknown process-photos option: {value}"));
            }
            value => positionals.push(value.to_string()),
        }
    }

    if !(60..=100).contains(&quality) {
        return fail("quality must be between 60 and 100");
    }
    if positionals.len() > 2 {
        return fail("process-photos accepts at most INPUT and OUTPUT paths");
    }

    Ok(ProcessPhotosOptions {
        input: positionals.first().map_or_else(
            || site.root.join("private/photography/originals"),
            |path| site.resolve_path(path),
        ),
        output: positionals.get(1).map_or_else(
            || site.root.join("content/photography/gallery"),
            |path| site.resolve_path(path),
        ),
        manifest,
        quality,
        dry_run,
    })
}

fn print_process_help() {
    println!(
        "\
process-photos

Usage:
  cargo run --manifest-path tools/site/Cargo.toml -- process-photos [INPUT] [OUTPUT] [options]

Defaults:
  INPUT   private/photography/originals
  OUTPUT  content/photography/gallery

Options:
  --quality N        JPEG quality, 60-100, default 92
  --manifest PATH    gallery JSON path, default content/photography/gallery.json
  --dry-run          print planned work without writing images
"
    );
}

fn collect_photo_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    if !dir.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_photo_files(&path, files)?;
        } else if is_photo_file(&path) {
            files.push(path);
        }
    }

    Ok(())
}

fn is_photo_file(path: &Path) -> bool {
    let Some(extension) = path.extension().and_then(OsStr::to_str) else {
        return false;
    };

    matches!(
        extension.to_ascii_lowercase().as_str(),
        "jpg" | "jpeg" | "png" | "webp"
    )
}

/// Decodes a picture and writes it back out as a JPEG.
///
/// Also used by `normalize-gallery`, which wants the re-encode for what it
/// discards — an image rebuilt from its pixels keeps no EXIF.
pub(crate) fn process_photo_file(input: &Path, output: &Path, quality: u8) -> Result<(u32, u32)> {
    let image = ImageReader::open(input)?.with_guessed_format()?.decode()?;
    publish_photo(&image.to_rgba8(), output, quality)
}

fn publish_photo(source: &RgbaImage, output: &Path, quality: u8) -> Result<(u32, u32)> {
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }

    let (width, height) = source.dimensions();
    let rgb = rgba_to_rgb_on_white(source);
    let writer = BufWriter::new(File::create(output)?);
    JpegEncoder::new_with_quality(writer, quality).encode_image(&rgb)?;

    Ok((width, height))
}

/// Flattens transparency onto white, which is what JPEG needs and what the
/// duplicate scan compares, so a picture saved with an alpha channel and the
/// same picture saved without one fingerprint alike.
pub(crate) fn rgba_to_rgb_on_white(source: &RgbaImage) -> RgbImage {
    let (width, height) = source.dimensions();
    let mut output = RgbImage::new(width, height);

    for (x, y, pixel) in source.enumerate_pixels() {
        let alpha = f32::from(pixel[3]) / 255.0;
        let channel = |value: u8| {
            let blended = f32::from(value) * alpha + 255.0 * (1.0 - alpha);
            blended.clamp(0.0, 255.0).round() as u8
        };
        output.put_pixel(
            x,
            y,
            Rgb([channel(pixel[0]), channel(pixel[1]), channel(pixel[2])]),
        );
    }

    output
}

fn sanitize_file_stem(stem: &str) -> String {
    let mut result = String::new();
    for ch in stem.chars() {
        if ch.is_ascii_alphanumeric() {
            result.push(ch.to_ascii_lowercase());
        } else if !result.ends_with('-') {
            result.push('-');
        }
    }

    let result = result.trim_matches('-');
    if result.is_empty() {
        "photo".to_string()
    } else {
        result.to_string()
    }
}

fn write_gallery_manifest(path: &Path, entries: &[GalleryEntry]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut body = serde_json::to_string_pretty(entries)?;
    body.push('\n');
    fs::write(path, body)?;
    Ok(())
}

fn load_existing_gallery_manifest(path: &Path) -> Result<HashMap<String, GalleryEntry>> {
    if !path.is_file() {
        return Ok(HashMap::new());
    }

    let entries: Vec<GalleryEntry> = serde_json::from_str(&fs::read_to_string(path)?)?;

    Ok(entries
        .into_iter()
        .map(|entry| (entry.meta.clone(), entry))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_a_published_photo_after_its_source() {
        assert_eq!(sanitize_file_stem("P1073908"), "p1073908");
        assert_eq!(sanitize_file_stem("Trip to Ōsaka (2)"), "trip-to-saka-2");
        assert_eq!(sanitize_file_stem("---"), "photo");
    }

    #[test]
    fn flattens_transparency_onto_white() {
        let mut source = RgbaImage::new(2, 1);
        source.put_pixel(0, 0, image::Rgba([0, 0, 0, 0]));
        source.put_pixel(1, 0, image::Rgba([0, 0, 0, 128]));

        let flattened = rgba_to_rgb_on_white(&source);
        assert_eq!(flattened.get_pixel(0, 0).0, [255, 255, 255]);
        assert_eq!(flattened.get_pixel(1, 0).0, [127, 127, 127]);
    }

    #[test]
    fn parses_the_options_it_documents() {
        let site = Site {
            root: PathBuf::from("C:/repo"),
            ci: false,
        };
        let options = parse_process_options(
            &site,
            &[
                "--quality".to_string(),
                "80".to_string(),
                "--dry-run".to_string(),
            ],
        )
        .unwrap();

        assert_eq!(options.quality, 80);
        assert!(options.dry_run);
        assert!(parse_process_options(&site, &["--quality".to_string(), "5".to_string()]).is_err());
        assert!(parse_process_options(&site, &["--nope".to_string()]).is_err());
    }
}
