use crate::{Result, Site, SiteError};
use image::codecs::jpeg::JpegEncoder;
use image::{ImageReader, Rgb, RgbImage, RgbaImage};
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

struct ProcessPhotosOptions {
    input: PathBuf,
    output: PathBuf,
    manifest: PathBuf,
    describer: PhotoDescriberOptions,
    // nightshade_input: PathBuf,
    // nightshade_output: PathBuf,
    quality: u8,
    dry_run: bool,
}

// struct SetupNightshadeOptions {
//     install_dir: PathBuf,
//     download_dir: PathBuf,
//     archive: Option<PathBuf>,
//     url: Option<String>,
//     force: bool,
//     print_url: bool,
//     dry_run: bool,
// }

#[derive(Clone, Debug)]
struct PhotoDescriberOptions {
    ollama_model: String,
    prompt: String,
}

#[derive(Serialize)]
struct GalleryEntry {
    src: String,
    title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    description: String,
    meta: String,
    tags: Vec<String>,
    width: u32,
    height: u32,
}

struct PhotoClassification {
    title: String,
    description: String,
    tags: Vec<String>,
}

// #[derive(Serialize)]
// struct NightshadeStagingEntry {
//     source: String,
//     file: String,
//     tag: String,
//     title: String,
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// enum NightshadeArchiveKind {
//     Zip,
//     Dmg,
// }

// struct NightshadePackage {
//     version: &'static str,
//     urls: &'static [&'static str],
//     file_name: &'static str,
//     archive_kind: NightshadeArchiveKind,
// }

struct OllamaPhotoDescriber {
    model: String,
    prompt: String,
}

const MAX_GENERATED_PHOTO_TAGS: usize = 8;
const DEFAULT_OLLAMA_PHOTO_MODEL: &str = "gemma3";
const DEFAULT_PHOTO_DESCRIPTION_PROMPT: &str = "\
Describe this image for a personal photography gallery.
Return only these three labeled fields:
Title: 3 to 7 words
Description: one concise sentence about the visible scene
Tags: 3 to 8 lowercase tags, separated by commas
Do not include markdown, filenames, camera settings, or guesses about private identity.";
// const NIGHTSHADE_VERSION: &str = "1.1";
// const NIGHTSHADE_DOWNLOADS_PAGE: &str = "https://nightshade.cs.uchicago.edu/downloads.html";
// const NIGHTSHADE_WINDOWS_URLS: &[&str] = &[
//     "https://webvault.cs.uchicago.edu/sandlab/fawkes/files/nightshade/Nightshade-1.1-Windows.zip",
//     "https://mirror.cs.uchicago.edu/fawkes/files/nightshade/Nightshade-1.1-Windows.zip",
// ];
// const NIGHTSHADE_MACOS_APPLE_SILICON_URLS: &[&str] = &[
//     "https://webvault.cs.uchicago.edu/sandlab/fawkes/files/nightshade/Nightshade-1.1-m1.dmg",
//     "https://mirror.cs.uchicago.edu/fawkes/files/nightshade/Nightshade-1.1-m1.dmg",
// ];

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

    let describer = if options.dry_run {
        None
    } else {
        Some(OllamaPhotoDescriber::new(
            site,
            &options.describer.ollama_model,
            &options.describer.prompt,
        )?)
    };
    let mut entries = Vec::new();
    let mut names = HashMap::<String, usize>::new();
    // let mut staging_entries = Vec::new();
    // let mut missing_nightshade_outputs = Vec::new();

    // if !options.dry_run {
    //     fs::create_dir_all(&options.nightshade_input)?;
    //     fs::create_dir_all(&options.nightshade_output)?;
    // }

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
        // let staged_file_name = staged_photo_file_name(&base, *count, &photo);
        // let staged_photo = options.nightshade_input.join(&staged_file_name);
        // let nightshade_photo =
        //     find_nightshade_output(&options.nightshade_output, &staged_file_name);

        println!(
            "process {} -> {}",
            relative.display(),
            target.strip_prefix(&site.root).unwrap_or(&target).display()
        );

        let (width, height, classification) = if options.dry_run {
            (0, 0, PhotoClassification::unclassified())
        } else {
            process_photo_file(
                site,
                &photo,
                &target,
                options.quality,
                describer
                    .as_ref()
                    .ok_or_else(|| SiteError::new("photo describer was not initialized"))?,
            )?
        };
        // let nightshade_tag = nightshade_tag(&classification.tags);

        // if !options.dry_run {
        //     stage_nightshade_input(&photo, &staged_photo)?;
        //     staging_entries.push(NightshadeStagingEntry {
        //         source: relative.to_string_lossy().replace('\\', "/"),
        //         file: staged_file_name.clone(),
        //         tag: nightshade_tag.clone(),
        //         title: classification.title.clone(),
        //     });

        //     if nightshade_photo.is_none() {
        //         missing_nightshade_outputs.push(staged_file_name);
        //     }
        // }

        println!(
            "  title: {}; tags: {}",
            classification.title,
            if classification.tags.is_empty() {
                "unclassified".to_string()
            } else {
                classification.tags.join(", ")
            }
        );

        entries.push(GalleryEntry {
            src: format!("/photography/gallery/{file_name}"),
            title: classification.title,
            description: classification.description,
            meta: relative.to_string_lossy().replace('\\', "/"),
            tags: classification.tags,
            width,
            height,
        });
    }

    // if !options.dry_run {
    //     write_nightshade_staging_manifest(&options.nightshade_input, &staging_entries)?;
    // }

    // if !missing_nightshade_outputs.is_empty() {
    //     return Err(Box::new(SiteError::new(format!(
    //         "staged {} photo(s) for Nightshade in {}; run Nightshade on that folder with the tags in nightshade-tags.csv, write results to {}, then rerun process-photos",
    //         missing_nightshade_outputs.len(),
    //         options.nightshade_input.display(),
    //         options.nightshade_output.display()
    //     ))));
    // }

    if !options.dry_run {
        write_gallery_manifest(&options.manifest, &entries)?;
    }

    Ok(())
}

// pub(crate) fn setup_nightshade(site: &Site, args: &[String]) -> Result<()> {
//     if args.iter().any(|arg| arg == "--help" || arg == "-h") {
//         print_setup_help();
//         return Ok(());
//     }

//     let options = parse_setup_options(site, args)?;
//     let package = nightshade_package()?;
//     let download_urls = setup_download_urls(&options, &package);

//     if options.print_url {
//         for url in &download_urls {
//             println!("{url}");
//         }
//         return Ok(());
//     }

//     let archive = options
//         .archive
//         .clone()
//         .unwrap_or_else(|| options.download_dir.join(package.file_name));
//     println!("Nightshade {}", package.version);
//     if options.archive.is_some() {
//         println!("archive source: local file");
//     } else {
//         for (index, url) in download_urls.iter().enumerate() {
//             if index == 0 {
//                 println!("download: {url}");
//             } else {
//                 println!("fallback download: {url}");
//             }
//         }
//     }
//     println!("archive: {}", archive.display());

//     if options.dry_run {
//         if options.archive.is_some() {
//             println!("dry run: would unpack Nightshade into private/tools/nightshade");
//         } else {
//             println!(
//                 "dry run: would download Nightshade and unpack it into private/tools/nightshade"
//             );
//         }
//         return Ok(());
//     }

//     fs::create_dir_all(&options.download_dir)?;
//     fs::create_dir_all(&options.install_dir)?;

//     if options.archive.is_some() {
//         if !archive.is_file() {
//             return Err(Box::new(SiteError::new(format!(
//                 "Nightshade archive does not exist: {}",
//                 archive.display()
//             ))));
//         }
//         println!("using existing {}", archive.display());
//     } else if archive.is_file() && !options.force {
//         println!("reusing existing {}", archive.display());
//     } else {
//         download_nightshade_archive(site, &download_urls, &archive)?;
//     }

//     match package.archive_kind {
//         NightshadeArchiveKind::Zip => {
//             let app_dir = options
//                 .install_dir
//                 .join(format!("Nightshade-{}", package.version));
//             unpack_zip_archive(&archive, &app_dir)?;
//             if let Some(executable) = find_nightshade_executable(&app_dir)? {
//                 println!("Nightshade executable: {}", executable.display());
//             } else {
//                 println!("Nightshade unpacked to {}", app_dir.display());
//             }
//         }
//         NightshadeArchiveKind::Dmg => {
//             println!(
//                 "Nightshade DMG downloaded to {}; mount it and drag the app into {}",
//                 archive.display(),
//                 options.install_dir.display()
//             );
//         }
//     }

//     Ok(())
// }

fn parse_process_options(site: &Site, args: &[String]) -> Result<ProcessPhotosOptions> {
    let mut positionals = Vec::new();
    let mut quality = 92_u8;
    let mut manifest = site.root.join("content/photography/gallery.json");
    let mut ollama_model = DEFAULT_OLLAMA_PHOTO_MODEL.to_string();
    let mut description_prompt = DEFAULT_PHOTO_DESCRIPTION_PROMPT.to_string();
    // let mut nightshade_input = site.root.join("private/photography/nightshade/input");
    // let mut nightshade_output = site.root.join("private/photography/nightshade/output");
    let mut dry_run = false;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--quality" => {
                index += 1;
                let value = args.get(index).ok_or_else(|| {
                    Box::new(SiteError::new("--quality requires a value")) as Box<dyn Error>
                })?;
                quality = value.parse::<u8>().map_err(|source| {
                    SiteError::new(format!("invalid quality {value:?}: {source}"))
                })?;
            }
            "--manifest" => {
                index += 1;
                let value = args.get(index).ok_or_else(|| {
                    Box::new(SiteError::new("--manifest requires a path")) as Box<dyn Error>
                })?;
                manifest = site.resolve_path(value);
            }
            "--describer" => {
                index += 1;
                let value = args.get(index).ok_or_else(|| {
                    Box::new(SiteError::new("--describer requires a value")) as Box<dyn Error>
                })?;
                validate_photo_describer(value)?;
            }
            "--ollama-model" => {
                index += 1;
                let value = args.get(index).ok_or_else(|| {
                    Box::new(SiteError::new("--ollama-model requires a model name"))
                        as Box<dyn Error>
                })?;
                ollama_model = value.to_string();
            }
            "--description-prompt" => {
                index += 1;
                let value = args.get(index).ok_or_else(|| {
                    Box::new(SiteError::new("--description-prompt requires text")) as Box<dyn Error>
                })?;
                description_prompt = value.to_string();
            }
            // "--nightshade-input" => {
            //     index += 1;
            //     let value = args.get(index).ok_or_else(|| {
            //         Box::new(SiteError::new("--nightshade-input requires a path")) as Box<dyn Error>
            //     })?;
            //     nightshade_input = site.resolve_path(value);
            // }
            // "--nightshade-output" => {
            //     index += 1;
            //     let value = args.get(index).ok_or_else(|| {
            //         Box::new(SiteError::new("--nightshade-output requires a path"))
            //             as Box<dyn Error>
            //     })?;
            //     nightshade_output = site.resolve_path(value);
            // }
            "--dry-run" => dry_run = true,
            "--help" | "-h" => {
                print_process_help();
                return Err(Box::new(SiteError::new("help requested")));
            }
            value if value.starts_with('-') => {
                return Err(Box::new(SiteError::new(format!(
                    "unknown process-photos option: {value}"
                ))));
            }
            value => positionals.push(value.to_string()),
        }
        index += 1;
    }

    if !(60..=100).contains(&quality) {
        return Err(Box::new(SiteError::new(
            "quality must be between 60 and 100",
        )));
    }
    if positionals.len() > 2 {
        return Err(Box::new(SiteError::new(
            "process-photos accepts at most INPUT and OUTPUT paths",
        )));
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
        describer: PhotoDescriberOptions {
            ollama_model,
            prompt: description_prompt,
        },
        // nightshade_input,
        // nightshade_output,
        quality,
        dry_run,
    })
}

// fn parse_setup_options(site: &Site, args: &[String]) -> Result<SetupNightshadeOptions> {
//     let mut install_dir = site.root.join("private/tools/nightshade");
//     let mut download_dir = None;
//     let mut archive = None;
//     let mut url = None;
//     let mut force = false;
//     let mut print_url = false;
//     let mut dry_run = false;
//     let mut index = 0;

//     while index < args.len() {
//         match args[index].as_str() {
//             "--install-dir" => {
//                 index += 1;
//                 let value = args.get(index).ok_or_else(|| {
//                     Box::new(SiteError::new("--install-dir requires a path")) as Box<dyn Error>
//                 })?;
//                 install_dir = site.resolve_path(value);
//             }
//             "--download-dir" => {
//                 index += 1;
//                 let value = args.get(index).ok_or_else(|| {
//                     Box::new(SiteError::new("--download-dir requires a path")) as Box<dyn Error>
//                 })?;
//                 download_dir = Some(site.resolve_path(value));
//             }
//             "--archive" => {
//                 index += 1;
//                 let value = args.get(index).ok_or_else(|| {
//                     Box::new(SiteError::new("--archive requires a path")) as Box<dyn Error>
//                 })?;
//                 archive = Some(site.resolve_path(value));
//             }
//             "--url" => {
//                 index += 1;
//                 let value = args.get(index).ok_or_else(|| {
//                     Box::new(SiteError::new("--url requires a URL")) as Box<dyn Error>
//                 })?;
//                 url = Some(value.to_string());
//             }
//             "--force" => force = true,
//             "--print-url" => print_url = true,
//             "--dry-run" => dry_run = true,
//             "--help" | "-h" => {
//                 print_setup_help();
//                 return Err(Box::new(SiteError::new("help requested")));
//             }
//             value => {
//                 return Err(Box::new(SiteError::new(format!(
//                     "unknown setup-nightshade option: {value}"
//                 ))));
//             }
//         }
//         index += 1;
//     }

//     let download_dir = download_dir.unwrap_or_else(|| install_dir.join("downloads"));

//     Ok(SetupNightshadeOptions {
//         install_dir,
//         download_dir,
//         archive,
//         url,
//         force,
//         print_url,
//         dry_run,
//     })
// }

// fn setup_download_urls(
//     options: &SetupNightshadeOptions,
//     package: &NightshadePackage,
// ) -> Vec<String> {
//     if let Some(url) = &options.url {
//         return vec![url.to_string()];
//     }

//     package.urls.iter().map(|url| url.to_string()).collect()
// }

// fn download_nightshade_archive(site: &Site, urls: &[String], output: &Path) -> Result<()> {
//     let mut errors = Vec::new();

//     for url in urls {
//         match download_file(site, url, output) {
//             Ok(()) => return Ok(()),
//             Err(error) => {
//                 errors.push(format!("{url}: {error}"));
//                 site.warn(&format!("Nightshade download failed from {url}"));
//             }
//         }
//     }

//     Err(Box::new(SiteError::new(format!(
//         "failed to download Nightshade archive from pinned URL(s): {}\nDownload it manually from {NIGHTSHADE_DOWNLOADS_PAGE}, then rerun setup-nightshade with --archive PATH.",
//         errors.join("; ")
//     ))))
// }

// fn unpack_zip_archive(archive: &Path, destination: &Path) -> Result<()> {
//     fs::create_dir_all(destination)?;

//     println!(
//         "extracting {} to {}",
//         archive.display(),
//         destination.display()
//     );
//     let archive_file = File::open(archive)?;
//     let mut archive = zip::ZipArchive::new(archive_file)?;
//     archive.extract(destination)?;

//     Ok(())
// }

// fn print_setup_help() {
//     println!(
//         "\
// setup-nightshade

// Usage:
//   cargo run --manifest-path tools/site/Cargo.toml -- setup-nightshade [options]

// Options:
//   --install-dir PATH   default private/tools/nightshade
//   --download-dir PATH  default INSTALL_DIR/downloads
//   --archive PATH       unpack an already downloaded Nightshade archive
//   --url URL            override the pinned download URL
//   --force              redownload the archive if it already exists
//   --print-url          print the pinned official download URL(s) and exit
//   --dry-run            print planned work without downloading

// Nightshade is not part of the normal build and is ignored by git.
// "
//     );
// }

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
  --describer NAME   ollama, default ollama
  --ollama-model M   local Ollama vision model, default gemma3
  --description-prompt TEXT
                     prompt for Ollama labeled descriptions
  --dry-run          print planned work without writing images

Model:
  Install Ollama, pull a local vision model, then run process-photos.
  The --describer option is kept for explicitness and currently accepts ollama.
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

// fn nightshade_package() -> Result<NightshadePackage> {
//     if cfg!(target_os = "windows") {
//         return Ok(NightshadePackage {
//             version: NIGHTSHADE_VERSION,
//             urls: NIGHTSHADE_WINDOWS_URLS,
//             file_name: "Nightshade-1.1-Windows.zip",
//             archive_kind: NightshadeArchiveKind::Zip,
//         });
//     }

//     if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
//         return Ok(NightshadePackage {
//             version: NIGHTSHADE_VERSION,
//             urls: NIGHTSHADE_MACOS_APPLE_SILICON_URLS,
//             file_name: "Nightshade-1.1-m1.dmg",
//             archive_kind: NightshadeArchiveKind::Dmg,
//         });
//     }

//     Err(Box::new(SiteError::new(
//         "Nightshade setup is only pinned for Windows and Apple Silicon macOS",
//     )))
// }

// fn find_nightshade_executable(dir: &Path) -> Result<Option<PathBuf>> {
//     if !dir.is_dir() {
//         return Ok(None);
//     }

//     let mut pending = vec![dir.to_path_buf()];
//     while let Some(current) = pending.pop() {
//         for entry in fs::read_dir(current)? {
//             let path = entry?.path();
//             if is_nightshade_executable(&path) {
//                 return Ok(Some(path));
//             }
//             if path.is_dir() {
//                 pending.push(path);
//             }
//         }
//     }

//     Ok(None)
// }

// fn is_nightshade_executable(path: &Path) -> bool {
//     let Some(stem) = path.file_stem().and_then(OsStr::to_str) else {
//         return false;
//     };
//     if !stem.to_ascii_lowercase().contains("nightshade") {
//         return false;
//     }

//     if cfg!(target_os = "windows") {
//         path.extension()
//             .and_then(OsStr::to_str)
//             .is_some_and(|extension| extension.eq_ignore_ascii_case("exe"))
//     } else if cfg!(target_os = "macos") {
//         path.extension()
//             .and_then(OsStr::to_str)
//             .is_some_and(|extension| extension.eq_ignore_ascii_case("app"))
//     } else {
//         false
//     }
// }

impl PhotoClassification {
    fn unclassified() -> Self {
        Self {
            title: "Unclassified photo".to_string(),
            description: String::new(),
            tags: Vec::new(),
        }
    }
}

impl OllamaPhotoDescriber {
    fn new(site: &Site, model: &str, prompt: &str) -> Result<Self> {
        let model = model.trim();
        if model.is_empty() {
            return Err(Box::new(SiteError::new(
                "--ollama-model must not be empty when --describer ollama is used",
            )));
        }

        let show_args = vec![OsString::from("show"), OsString::from(model)];
        if !site.status_success(&site.root, "ollama", &show_args)? {
            return Err(Box::new(SiteError::new(format!(
                "Ollama model {model:?} is not available locally; run `ollama pull {model}` first"
            ))));
        }

        println!("photo describer: ollama ({model})");

        Ok(Self {
            model: model.to_string(),
            prompt: prompt.to_string(),
        })
    }

    fn describe(&self, site: &Site, image: &Path) -> Result<PhotoClassification> {
        let args = vec![
            OsString::from("run"),
            OsString::from(&self.model),
            image.as_os_str().to_os_string(),
            OsString::from(&self.prompt),
        ];
        let output = site
            .output_optional(&site.root, "ollama", &args)?
            .ok_or_else(|| {
                SiteError::new(format!(
                    "Ollama failed to describe {}; check that {} is a local vision model",
                    image.display(),
                    self.model
                ))
            })?;

        parse_generated_photo_description(&output)
    }
}

fn validate_photo_describer(value: &str) -> Result<()> {
    match value.to_ascii_lowercase().as_str() {
        "ollama" => Ok(()),
        _ => Err(Box::new(SiteError::new(format!(
            "unknown photo describer {value:?}; only ollama is supported"
        )))),
    }
}

fn process_photo_file(
    site: &Site,
    input: &Path,
    output: &Path,
    quality: u8,
    describer: &OllamaPhotoDescriber,
) -> Result<(u32, u32, PhotoClassification)> {
    let image = ImageReader::open(input)?.with_guessed_format()?.decode()?;
    let source = image.to_rgba8();
    let classification = describer.describe(site, input)?;
    let (width, height) = publish_photo(&source, output, quality)?;

    Ok((width, height, classification))
}

fn publish_photo(source: &RgbaImage, output: &Path, quality: u8) -> Result<(u32, u32)> {
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }

    let (width, height) = source.dimensions();
    let rgb = rgba_to_rgb_on_white(source);
    let file = File::create(output)?;
    let writer = BufWriter::new(file);
    let mut encoder = JpegEncoder::new_with_quality(writer, quality);
    encoder.encode_image(&rgb)?;

    Ok((width, height))
}

fn rgba_to_rgb_on_white(source: &RgbaImage) -> RgbImage {
    let (width, height) = source.dimensions();
    let mut output = RgbImage::new(width, height);

    for (x, y, pixel) in source.enumerate_pixels() {
        let alpha = f32::from(pixel[3]) / 255.0;
        let red = f32::from(pixel[0]) * alpha + 255.0 * (1.0 - alpha);
        let green = f32::from(pixel[1]) * alpha + 255.0 * (1.0 - alpha);
        let blue = f32::from(pixel[2]) * alpha + 255.0 * (1.0 - alpha);
        output.put_pixel(
            x,
            y,
            Rgb([
                red.clamp(0.0, 255.0).round() as u8,
                green.clamp(0.0, 255.0).round() as u8,
                blue.clamp(0.0, 255.0).round() as u8,
            ]),
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

// fn staged_photo_file_name(base: &str, count: usize, source: &Path) -> String {
//     let extension = source
//         .extension()
//         .and_then(OsStr::to_str)
//         .map(|extension| extension.to_ascii_lowercase())
//         .filter(|extension| is_safe_extension(extension))
//         .unwrap_or_else(|| "jpg".to_string());

//     if count == 1 {
//         format!("{base}.{extension}")
//     } else {
//         format!("{base}-{count}.{extension}")
//     }
// }

// fn is_safe_extension(extension: &str) -> bool {
//     extension
//         .chars()
//         .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
// }

// fn find_nightshade_output(output_dir: &Path, staged_file_name: &str) -> Option<PathBuf> {
//     let direct = output_dir.join(staged_file_name);
//     if direct.is_file() {
//         return Some(direct);
//     }

//     let stem = Path::new(staged_file_name).file_stem()?.to_string_lossy();
//     for extension in ["jpg", "jpeg", "png", "webp"] {
//         let candidate = output_dir.join(format!("{stem}.{extension}"));
//         if candidate.is_file() {
//             return Some(candidate);
//         }
//     }

//     None
// }

// fn stage_nightshade_input(source: &Path, target: &Path) -> Result<()> {
//     if let Some(parent) = target.parent() {
//         fs::create_dir_all(parent)?;
//     }
//     fs::copy(source, target)?;
//     Ok(())
// }

// fn nightshade_tag(tags: &[String]) -> String {
//     tags.first()
//         .filter(|tag| !tag.trim().is_empty())
//         .cloned()
//         .unwrap_or_else(|| "photo".to_string())
// }

// fn write_nightshade_staging_manifest(
//     input_dir: &Path,
//     entries: &[NightshadeStagingEntry],
// ) -> Result<()> {
//     fs::create_dir_all(input_dir)?;

//     let mut json = serde_json::to_string_pretty(entries)?;
//     json.push('\n');
//     fs::write(input_dir.join("nightshade-tags.json"), json)?;

//     let mut csv = String::from("file,tag,title,source\n");
//     for entry in entries {
//         csv.push_str(&csv_field(&entry.file));
//         csv.push(',');
//         csv.push_str(&csv_field(&entry.tag));
//         csv.push(',');
//         csv.push_str(&csv_field(&entry.title));
//         csv.push(',');
//         csv.push_str(&csv_field(&entry.source));
//         csv.push('\n');
//     }
//     fs::write(input_dir.join("nightshade-tags.csv"), csv)?;

//     Ok(())
// }

// fn csv_field(value: &str) -> String {
//     if value
//         .chars()
//         .any(|ch| matches!(ch, '"' | ',' | '\n' | '\r'))
//     {
//         format!("\"{}\"", value.replace('"', "\"\""))
//     } else {
//         value.to_string()
//     }
// }

fn parse_generated_photo_description(output: &str) -> Result<PhotoClassification> {
    let (raw_title, raw_description, raw_tags) = parse_labeled_photo_description(output)
        .ok_or_else(|| {
            SiteError::new(format!(
                "photo describer did not return Title, Description, and Tags fields: {}",
                output.trim()
            ))
        })?;
    let description = clean_generated_text(&raw_description).unwrap_or_default();
    let title = clean_generated_text(&raw_title)
        .or_else(|| title_from_description(&description))
        .unwrap_or_else(|| "Untitled photo".to_string());
    let tags = raw_tags
        .split(',')
        .filter_map(|tag| clean_generated_tag(tag))
        .take(MAX_GENERATED_PHOTO_TAGS)
        .collect::<Vec<_>>();

    Ok(PhotoClassification {
        title,
        description,
        tags,
    })
}

fn parse_labeled_photo_description(output: &str) -> Option<(String, String, String)> {
    let mut title = String::new();
    let mut description = String::new();
    let mut tags = String::new();
    let mut current = None;

    for line in strip_code_fence(output).lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some((field, value)) = parse_labeled_line(line) {
            current = Some(field);
            append_generated_field(
                match field {
                    Field::Title => &mut title,
                    Field::Description => &mut description,
                    Field::Tags => &mut tags,
                },
                value,
            );
            continue;
        }

        if let Some(field) = current {
            append_generated_field(
                match field {
                    Field::Title => &mut title,
                    Field::Description => &mut description,
                    Field::Tags => &mut tags,
                },
                line,
            );
        }
    }

    (!title.trim().is_empty() || !description.trim().is_empty() || !tags.trim().is_empty())
        .then_some((title, description, tags))
}

fn parse_labeled_line(line: &str) -> Option<(Field, &str)> {
    let line = line
        .trim_start_matches(|ch: char| matches!(ch, '-' | '*'))
        .trim();
    let (label, value) = line.split_once(':')?;
    let field = match label.trim().to_ascii_lowercase().as_str() {
        "title" => Field::Title,
        "description" | "caption" => Field::Description,
        "tags" => Field::Tags,
        _ => return None,
    };

    Some((field, value.trim()))
}

#[derive(Clone, Copy)]
enum Field {
    Title,
    Description,
    Tags,
}

fn append_generated_field(target: &mut String, value: &str) {
    if value.trim().is_empty() {
        return;
    }
    if !target.is_empty() {
        target.push(' ');
    }
    target.push_str(value.trim());
}

fn strip_code_fence(output: &str) -> &str {
    let trimmed = output.trim();
    let without_start = trimmed
        .strip_prefix("```text")
        .or_else(|| trimmed.strip_prefix("```"))
        .unwrap_or(trimmed)
        .trim();

    without_start
        .strip_suffix("```")
        .unwrap_or(without_start)
        .trim()
}

fn clean_generated_text(value: &str) -> Option<String> {
    let cleaned = value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim_matches(|ch: char| matches!(ch, '"' | '\'' | '`'))
        .trim()
        .to_string();

    (!cleaned.is_empty()).then_some(cleaned)
}

fn clean_generated_tag(value: &str) -> Option<String> {
    let tag = value
        .trim()
        .trim_matches(|ch: char| matches!(ch, '"' | '\'' | '`' | '.' | ',' | ';' | ':'))
        .to_ascii_lowercase()
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || matches!(ch, ' ' | '-' | '_'))
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    (!tag.is_empty()).then_some(tag)
}

fn title_from_description(description: &str) -> Option<String> {
    let words = description
        .split_whitespace()
        .take(7)
        .map(|word| {
            word.trim_matches(|ch: char| !ch.is_alphanumeric())
                .to_string()
        })
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>();

    (!words.is_empty()).then(|| titleize_tag(&words.join(" ")))
}

fn titleize_tag(tag: &str) -> String {
    tag.split_whitespace()
        .map(|word| {
            if word.eq_ignore_ascii_case("tv") {
                return "TV".to_string();
            }

            let mut chars = word.chars();
            match chars.next() {
                Some(first) => {
                    let mut title = String::new();
                    title.push(first.to_ascii_uppercase());
                    title.push_str(chars.as_str());
                    title
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
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

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn prepares_nightshade_staging_names_and_tags() {
    //     assert_eq!(
    //         staged_photo_file_name("p1073908", 1, Path::new("P1073908.JPG")),
    //         "p1073908.jpg"
    //     );
    //     assert_eq!(
    //         staged_photo_file_name("p1073908", 2, Path::new("P1073908.PNG")),
    //         "p1073908-2.png"
    //     );
    //     assert_eq!(nightshade_tag(&[]), "photo");
    //     assert_eq!(nightshade_tag(&["cat".to_string()]), "cat");
    //     assert_eq!(csv_field("cat, portrait"), "\"cat, portrait\"");
    // }

    // #[test]
    // fn parses_setup_nightshade_archive_and_url_options() {
    //     let site = Site {
    //         root: PathBuf::from("C:/repo"),
    //         ci: false,
    //     };
    //     let options = parse_setup_options(
    //         &site,
    //         &[
    //             "--archive".to_string(),
    //             "downloads/Nightshade.zip".to_string(),
    //             "--url".to_string(),
    //             "https://example.invalid/Nightshade.zip".to_string(),
    //         ],
    //     )
    //     .unwrap();

    //     assert_eq!(
    //         options.archive.as_deref(),
    //         Some(Path::new("C:/repo/downloads/Nightshade.zip"))
    //     );
    //     assert_eq!(
    //         options.url.as_deref(),
    //         Some("https://example.invalid/Nightshade.zip")
    //     );
    // }

    #[test]
    fn parses_ollama_photo_describer_options() {
        let site = Site {
            root: PathBuf::from("C:/repo"),
            ci: false,
        };
        let options = parse_process_options(
            &site,
            &[
                "--describer".to_string(),
                "ollama".to_string(),
                "--ollama-model".to_string(),
                "moondream".to_string(),
                "--description-prompt".to_string(),
                "describe briefly".to_string(),
            ],
        )
        .unwrap();

        assert_eq!(options.describer.ollama_model, "moondream");
        assert_eq!(options.describer.prompt, "describe briefly");
    }

    #[test]
    fn defaults_to_ollama_photo_describer() {
        let site = Site {
            root: PathBuf::from("C:/repo"),
            ci: false,
        };
        let options = parse_process_options(&site, &[]).unwrap();

        assert_eq!(options.describer.ollama_model, DEFAULT_OLLAMA_PHOTO_MODEL);
        assert_eq!(options.describer.prompt, DEFAULT_PHOTO_DESCRIPTION_PROMPT);
    }

    #[test]
    fn parses_labeled_photo_description() {
        let classification = parse_generated_photo_description(
            r#"Title: Bicycle by Brick Wall
Description: A bicycle leans against a brick wall on a quiet street.
Tags: Bicycle, brick wall, street!,
```"#,
        )
        .unwrap();

        assert_eq!(classification.title, "Bicycle by Brick Wall");
        assert_eq!(
            classification.description,
            "A bicycle leans against a brick wall on a quiet street."
        );
        assert_eq!(
            classification.tags,
            vec![
                "bicycle".to_string(),
                "brick wall".to_string(),
                "street".to_string()
            ]
        );
    }

    #[test]
    fn parses_wrapped_ollama_description_output() {
        let classification = parse_generated_photo_description(
            r#"Title: Red Cat in Motion
Description: A blurred, warm-toned image of a ginger cat in a moment
of movement.
Tags: cat, motion blur, ginger, pet, animal, texture,
warm tones, photography"#,
        )
        .unwrap();

        assert_eq!(classification.title, "Red Cat in Motion");
        assert_eq!(
            classification.description,
            "A blurred, warm-toned image of a ginger cat in a moment of movement."
        );
        assert_eq!(
            classification.tags,
            vec![
                "cat".to_string(),
                "motion blur".to_string(),
                "ginger".to_string(),
                "pet".to_string(),
                "animal".to_string(),
                "texture".to_string(),
                "warm tones".to_string(),
                "photography".to_string()
            ]
        );
    }

    #[test]
    fn parses_bulleted_labeled_photo_description() {
        let classification = parse_generated_photo_description(
            r#"- Title: Warm Cat Blur
- Description: A ginger cat moves through warm light.
- Tags: cat, motion blur, warm tones"#,
        )
        .unwrap();

        assert_eq!(classification.title, "Warm Cat Blur");
        assert_eq!(
            classification.description,
            "A ginger cat moves through warm light."
        );
        assert_eq!(
            classification.tags,
            vec![
                "cat".to_string(),
                "motion blur".to_string(),
                "warm tones".to_string()
            ]
        );
    }

    #[test]
    fn derives_title_from_generated_description_when_missing() {
        let classification = parse_generated_photo_description(
            r#"Description: a foggy hill path curves into the trees.
Tags: fog, hill path, trees"#,
        )
        .unwrap();

        assert_eq!(classification.title, "A Foggy Hill Path Curves Into The");
        assert_eq!(
            classification.description,
            "a foggy hill path curves into the trees."
        );
    }

    #[test]
    fn rejects_unlabeled_photo_description() {
        assert!(
            parse_generated_photo_description("A foggy hill path curves into the trees.").is_err()
        );
    }
}
