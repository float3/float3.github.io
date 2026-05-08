use crate::{Result, Site, SiteError};
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::{ImageReader, Rgb, RgbImage, RgbaImage};
use ort::{ep, session::Session, value::Tensor};
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
    model: PathBuf,
    device: PhotoClassifierDevice,
    confidence: f32,
    nightshade_input: PathBuf,
    nightshade_output: PathBuf,
    quality: u8,
    dry_run: bool,
}

struct SetupNightshadeOptions {
    install_dir: PathBuf,
    download_dir: PathBuf,
    force: bool,
    print_url: bool,
    dry_run: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PhotoClassifierDevice {
    Auto,
    Cpu,
    Cuda,
    DirectMl,
}

#[derive(Serialize)]
struct GalleryEntry {
    src: String,
    title: String,
    meta: String,
    tags: Vec<String>,
    width: u32,
    height: u32,
}

struct PhotoClassification {
    title: String,
    tags: Vec<String>,
}

#[derive(Serialize)]
struct NightshadeStagingEntry {
    source: String,
    file: String,
    tag: String,
    title: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NightshadeArchiveKind {
    Zip,
    Dmg,
}

struct NightshadePackage {
    version: &'static str,
    url: &'static str,
    file_name: &'static str,
    archive_kind: NightshadeArchiveKind,
}

#[derive(Clone, Debug)]
struct DetectedObject {
    label: String,
    score: f32,
}

#[derive(Clone, Copy)]
enum DetectionLayout {
    ChannelsFirst { channels: usize, candidates: usize },
    CandidatesFirst { channels: usize, candidates: usize },
}

struct PhotoDetector {
    session: Session,
    confidence: f32,
}

const PHOTO_MODEL_INPUT_SIZE: u32 = 640;
const MAX_PHOTO_TAGS: usize = 4;
const NIGHTSHADE_VERSION: &str = "1.1";
const NIGHTSHADE_WINDOWS_URL: &str =
    "https://webvault.cs.uchicago.edu/sandlab/fawkes/files/nightshade/Nightshade-1.1-Windows.zip";
const NIGHTSHADE_MACOS_APPLE_SILICON_URL: &str =
    "https://webvault.cs.uchicago.edu/sandlab/fawkes/files/nightshade/Nightshade-1.1-m1.dmg";
const COCO_LABELS: [&str; 80] = [
    "person",
    "bicycle",
    "car",
    "motorcycle",
    "airplane",
    "bus",
    "train",
    "truck",
    "boat",
    "traffic light",
    "fire hydrant",
    "stop sign",
    "parking meter",
    "bench",
    "bird",
    "cat",
    "dog",
    "horse",
    "sheep",
    "cow",
    "elephant",
    "bear",
    "zebra",
    "giraffe",
    "backpack",
    "umbrella",
    "handbag",
    "tie",
    "suitcase",
    "frisbee",
    "skis",
    "snowboard",
    "sports ball",
    "kite",
    "baseball bat",
    "baseball glove",
    "skateboard",
    "surfboard",
    "tennis racket",
    "bottle",
    "wine glass",
    "cup",
    "fork",
    "knife",
    "spoon",
    "bowl",
    "banana",
    "apple",
    "sandwich",
    "orange",
    "broccoli",
    "carrot",
    "hot dog",
    "pizza",
    "donut",
    "cake",
    "chair",
    "couch",
    "potted plant",
    "bed",
    "dining table",
    "toilet",
    "tv",
    "laptop",
    "mouse",
    "remote",
    "keyboard",
    "cell phone",
    "microwave",
    "oven",
    "toaster",
    "sink",
    "refrigerator",
    "book",
    "clock",
    "vase",
    "scissors",
    "teddy bear",
    "hair drier",
    "toothbrush",
];

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

    let mut detector = if options.dry_run {
        None
    } else {
        Some(PhotoDetector::new(
            &options.model,
            options.device,
            options.confidence,
        )?)
    };
    let mut entries = Vec::new();
    let mut names = HashMap::<String, usize>::new();
    let mut staging_entries = Vec::new();
    let mut missing_nightshade_outputs = Vec::new();

    if !options.dry_run {
        fs::create_dir_all(&options.nightshade_input)?;
        fs::create_dir_all(&options.nightshade_output)?;
    }

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
        let staged_file_name = staged_photo_file_name(&base, *count, &photo);
        let staged_photo = options.nightshade_input.join(&staged_file_name);
        let nightshade_photo =
            find_nightshade_output(&options.nightshade_output, &staged_file_name);

        println!(
            "process {} -> {}",
            relative.display(),
            target.strip_prefix(&site.root).unwrap_or(&target).display()
        );

        let (width, height, classification) = if options.dry_run {
            (0, 0, PhotoClassification::unclassified())
        } else {
            process_photo_file(
                &photo,
                nightshade_photo.as_deref(),
                &target,
                options.quality,
                nightshade_photo.is_some(),
                detector
                    .as_mut()
                    .ok_or_else(|| SiteError::new("photo detector was not initialized"))?,
            )?
        };
        let nightshade_tag = nightshade_tag(&classification.tags);

        if !options.dry_run {
            stage_nightshade_input(&photo, &staged_photo)?;
            staging_entries.push(NightshadeStagingEntry {
                source: relative.to_string_lossy().replace('\\', "/"),
                file: staged_file_name.clone(),
                tag: nightshade_tag.clone(),
                title: classification.title.clone(),
            });

            if nightshade_photo.is_none() {
                missing_nightshade_outputs.push(staged_file_name);
            }
        }

        println!(
            "  title: {}; nightshade tag: {}; tags: {}",
            classification.title,
            nightshade_tag,
            if classification.tags.is_empty() {
                "unclassified".to_string()
            } else {
                classification.tags.join(", ")
            }
        );

        entries.push(GalleryEntry {
            src: format!("/photography/gallery/{file_name}"),
            title: classification.title,
            meta: relative.to_string_lossy().replace('\\', "/"),
            tags: classification.tags,
            width,
            height,
        });
    }

    if !options.dry_run {
        write_nightshade_staging_manifest(&options.nightshade_input, &staging_entries)?;
    }

    if !missing_nightshade_outputs.is_empty() {
        return Err(Box::new(SiteError::new(format!(
            "staged {} photo(s) for Nightshade in {}; run Nightshade on that folder with the tags in nightshade-tags.csv, write results to {}, then rerun process-photos",
            missing_nightshade_outputs.len(),
            options.nightshade_input.display(),
            options.nightshade_output.display()
        ))));
    }

    if !options.dry_run {
        write_gallery_manifest(&options.manifest, &entries)?;
    }

    Ok(())
}

pub(crate) fn setup_nightshade(site: &Site, args: &[String]) -> Result<()> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_setup_help();
        return Ok(());
    }

    let options = parse_setup_options(site, args)?;
    let package = nightshade_package()?;

    if options.print_url {
        println!("{}", package.url);
        return Ok(());
    }

    let archive = options.download_dir.join(package.file_name);
    println!("Nightshade {}", package.version);
    println!("download: {}", package.url);
    println!("archive: {}", archive.display());

    if options.dry_run {
        println!("dry run: would download Nightshade and unpack it into private/tools/nightshade");
        return Ok(());
    }

    fs::create_dir_all(&options.download_dir)?;
    fs::create_dir_all(&options.install_dir)?;

    if archive.is_file() && !options.force {
        println!("reusing existing {}", archive.display());
    } else {
        download_file(site, package.url, &archive)?;
    }

    match package.archive_kind {
        NightshadeArchiveKind::Zip => {
            let app_dir = options
                .install_dir
                .join(format!("Nightshade-{}", package.version));
            unpack_zip_archive(site, &archive, &app_dir)?;
            if let Some(executable) = find_nightshade_executable(&app_dir)? {
                println!("Nightshade executable: {}", executable.display());
            } else {
                println!("Nightshade unpacked to {}", app_dir.display());
            }
        }
        NightshadeArchiveKind::Dmg => {
            println!(
                "Nightshade DMG downloaded to {}; mount it and drag the app into {}",
                archive.display(),
                options.install_dir.display()
            );
        }
    }

    Ok(())
}

fn parse_process_options(site: &Site, args: &[String]) -> Result<ProcessPhotosOptions> {
    let mut positionals = Vec::new();
    let mut quality = 92_u8;
    let mut manifest = site.root.join("content/photography/gallery.json");
    let mut model = site.root.join("private/photography/models/yolo11n.onnx");
    let mut device = PhotoClassifierDevice::Auto;
    let mut confidence = 0.25_f32;
    let mut nightshade_input = site.root.join("private/photography/nightshade/input");
    let mut nightshade_output = site.root.join("private/photography/nightshade/output");
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
            "--model" => {
                index += 1;
                let value = args.get(index).ok_or_else(|| {
                    Box::new(SiteError::new("--model requires a path")) as Box<dyn Error>
                })?;
                model = site.resolve_path(value);
            }
            "--device" => {
                index += 1;
                let value = args.get(index).ok_or_else(|| {
                    Box::new(SiteError::new("--device requires a value")) as Box<dyn Error>
                })?;
                device = parse_photo_classifier_device(value)?;
            }
            "--confidence" => {
                index += 1;
                let value = args.get(index).ok_or_else(|| {
                    Box::new(SiteError::new("--confidence requires a value")) as Box<dyn Error>
                })?;
                confidence = value.parse::<f32>().map_err(|source| {
                    SiteError::new(format!("invalid confidence {value:?}: {source}"))
                })?;
            }
            "--nightshade-input" => {
                index += 1;
                let value = args.get(index).ok_or_else(|| {
                    Box::new(SiteError::new("--nightshade-input requires a path")) as Box<dyn Error>
                })?;
                nightshade_input = site.resolve_path(value);
            }
            "--nightshade-output" => {
                index += 1;
                let value = args.get(index).ok_or_else(|| {
                    Box::new(SiteError::new("--nightshade-output requires a path"))
                        as Box<dyn Error>
                })?;
                nightshade_output = site.resolve_path(value);
            }
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
    if !(0.0..=1.0).contains(&confidence) {
        return Err(Box::new(SiteError::new(
            "confidence must be between 0.0 and 1.0",
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
        model,
        device,
        confidence,
        nightshade_input,
        nightshade_output,
        quality,
        dry_run,
    })
}

fn parse_setup_options(site: &Site, args: &[String]) -> Result<SetupNightshadeOptions> {
    let mut install_dir = site.root.join("private/tools/nightshade");
    let mut download_dir = None;
    let mut force = false;
    let mut print_url = false;
    let mut dry_run = false;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--install-dir" => {
                index += 1;
                let value = args.get(index).ok_or_else(|| {
                    Box::new(SiteError::new("--install-dir requires a path")) as Box<dyn Error>
                })?;
                install_dir = site.resolve_path(value);
            }
            "--download-dir" => {
                index += 1;
                let value = args.get(index).ok_or_else(|| {
                    Box::new(SiteError::new("--download-dir requires a path")) as Box<dyn Error>
                })?;
                download_dir = Some(site.resolve_path(value));
            }
            "--force" => force = true,
            "--print-url" => print_url = true,
            "--dry-run" => dry_run = true,
            "--help" | "-h" => {
                print_setup_help();
                return Err(Box::new(SiteError::new("help requested")));
            }
            value => {
                return Err(Box::new(SiteError::new(format!(
                    "unknown setup-nightshade option: {value}"
                ))));
            }
        }
        index += 1;
    }

    let download_dir = download_dir.unwrap_or_else(|| install_dir.join("downloads"));

    Ok(SetupNightshadeOptions {
        install_dir,
        download_dir,
        force,
        print_url,
        dry_run,
    })
}

fn download_file(site: &Site, url: &str, output: &Path) -> Result<()> {
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }

    let program = if cfg!(target_os = "windows") {
        "curl.exe"
    } else {
        "curl"
    };
    let args = vec![
        OsString::from("--fail"),
        OsString::from("--location"),
        OsString::from("--show-error"),
        OsString::from("--output"),
        output.as_os_str().to_os_string(),
        OsString::from(url),
    ];
    site.run(&site.root, program, &args)
}

fn unpack_zip_archive(site: &Site, archive: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination)?;

    if cfg!(target_os = "windows") {
        let args = vec![
            OsString::from("-NoProfile"),
            OsString::from("-ExecutionPolicy"),
            OsString::from("Bypass"),
            OsString::from("-Command"),
            OsString::from("Expand-Archive -LiteralPath $args[0] -DestinationPath $args[1] -Force"),
            archive.as_os_str().to_os_string(),
            destination.as_os_str().to_os_string(),
        ];
        site.run(&site.root, "powershell.exe", &args)
    } else {
        let args = vec![
            OsString::from("-o"),
            archive.as_os_str().to_os_string(),
            OsString::from("-d"),
            destination.as_os_str().to_os_string(),
        ];
        site.run(&site.root, "unzip", &args)
    }
}

fn print_setup_help() {
    println!(
        "\
setup-nightshade

Usage:
  cargo run --manifest-path tools/site/Cargo.toml -- setup-nightshade [options]

Options:
  --install-dir PATH   default private/tools/nightshade
  --download-dir PATH  default INSTALL_DIR/downloads
  --force              redownload the archive if it already exists
  --print-url          print the pinned official download URL and exit
  --dry-run            print planned work without downloading

Nightshade is not part of the normal build and is ignored by git.
"
    );
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
  --model PATH       local YOLO ONNX model, default private/photography/models/yolo11n.onnx
  --device DEVICE    auto, cpu, cuda, or directml, default auto
  --confidence N     object confidence threshold, 0.0-1.0, default 0.25
  --nightshade-input PATH
                     private staging folder for source images and tags
  --nightshade-output PATH
                     folder where Nightshade writes protected images
  --dry-run          print planned work without writing images

Model:
  Download or export Ultralytics YOLO11n detection as ONNX and place it at
  private/photography/models/yolo11n.onnx.
  Example: yolo export model=yolo11n.pt format=onnx imgsz=640
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

fn nightshade_package() -> Result<NightshadePackage> {
    if cfg!(target_os = "windows") {
        return Ok(NightshadePackage {
            version: NIGHTSHADE_VERSION,
            url: NIGHTSHADE_WINDOWS_URL,
            file_name: "Nightshade-1.1-Windows.zip",
            archive_kind: NightshadeArchiveKind::Zip,
        });
    }

    if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
        return Ok(NightshadePackage {
            version: NIGHTSHADE_VERSION,
            url: NIGHTSHADE_MACOS_APPLE_SILICON_URL,
            file_name: "Nightshade-1.1-m1.dmg",
            archive_kind: NightshadeArchiveKind::Dmg,
        });
    }

    Err(Box::new(SiteError::new(
        "Nightshade setup is only pinned for Windows and Apple Silicon macOS",
    )))
}

fn find_nightshade_executable(dir: &Path) -> Result<Option<PathBuf>> {
    if !dir.is_dir() {
        return Ok(None);
    }

    let mut pending = vec![dir.to_path_buf()];
    while let Some(current) = pending.pop() {
        for entry in fs::read_dir(current)? {
            let path = entry?.path();
            if is_nightshade_executable(&path) {
                return Ok(Some(path));
            }
            if path.is_dir() {
                pending.push(path);
            }
        }
    }

    Ok(None)
}

fn is_nightshade_executable(path: &Path) -> bool {
    let Some(stem) = path.file_stem().and_then(OsStr::to_str) else {
        return false;
    };
    if !stem.to_ascii_lowercase().contains("nightshade") {
        return false;
    }

    if cfg!(target_os = "windows") {
        path.extension()
            .and_then(OsStr::to_str)
            .is_some_and(|extension| extension.eq_ignore_ascii_case("exe"))
    } else if cfg!(target_os = "macos") {
        path.extension()
            .and_then(OsStr::to_str)
            .is_some_and(|extension| extension.eq_ignore_ascii_case("app"))
    } else {
        false
    }
}

impl PhotoClassifierDevice {
    fn label(self) -> &'static str {
        match self {
            PhotoClassifierDevice::Auto => "auto",
            PhotoClassifierDevice::Cpu => "cpu",
            PhotoClassifierDevice::Cuda => "cuda",
            PhotoClassifierDevice::DirectMl => "directml",
        }
    }
}

impl PhotoDetector {
    fn new(model: &Path, device: PhotoClassifierDevice, confidence: f32) -> Result<Self> {
        if !model.is_file() {
            return Err(Box::new(SiteError::new(format!(
                "photo object model not found at {}; export Ultralytics YOLO11n with `yolo export model=yolo11n.pt format=onnx imgsz=640` or pass --model PATH",
                model.display()
            ))));
        }

        let providers = photo_detector_providers(device);
        let session = if device == PhotoClassifierDevice::Auto && !providers.is_empty() {
            match build_photo_detector_session(model, providers.as_slice()) {
                Ok(session) => session,
                Err(error) => {
                    eprintln!(
                        "warning: failed to initialize GPU object detection ({error}); falling back to CPU"
                    );
                    build_photo_detector_session(model, &[])?
                }
            }
        } else {
            build_photo_detector_session(model, providers.as_slice())?
        };

        println!(
            "object detection model: {} ({})",
            model.display(),
            device.label()
        );

        Ok(Self {
            session,
            confidence,
        })
    }

    fn classify(&mut self, source: &RgbaImage) -> Result<PhotoClassification> {
        let input = photo_model_input(source);
        let tensor = Tensor::from_array((
            [
                1_usize,
                3,
                PHOTO_MODEL_INPUT_SIZE as usize,
                PHOTO_MODEL_INPUT_SIZE as usize,
            ],
            input.into_boxed_slice(),
        ))?;
        let outputs = self.session.run(ort::inputs![tensor])?;

        if outputs.len() == 0 {
            return Err(Box::new(SiteError::new(
                "object detection model returned no outputs",
            )));
        }

        let (shape, data) = outputs[0].try_extract_tensor::<f32>()?;
        let detections = detections_from_output(shape, data, self.confidence)?;
        Ok(PhotoClassification::from_detections(&detections))
    }
}

impl PhotoClassification {
    fn unclassified() -> Self {
        Self {
            title: "Unclassified photo".to_string(),
            tags: Vec::new(),
        }
    }

    fn from_detections(detections: &[DetectedObject]) -> Self {
        let tags = detections
            .iter()
            .take(MAX_PHOTO_TAGS)
            .map(|detection| detection.label.clone())
            .collect::<Vec<_>>();

        Self {
            title: title_from_tags(&tags),
            tags,
        }
    }
}

fn parse_photo_classifier_device(value: &str) -> Result<PhotoClassifierDevice> {
    match value.to_ascii_lowercase().as_str() {
        "auto" => Ok(PhotoClassifierDevice::Auto),
        "cpu" => Ok(PhotoClassifierDevice::Cpu),
        "cuda" => Ok(PhotoClassifierDevice::Cuda),
        "directml" | "dml" => Ok(PhotoClassifierDevice::DirectMl),
        _ => Err(Box::new(SiteError::new(format!(
            "unknown photo classifier device {value:?}; use auto, cpu, cuda, or directml"
        )))),
    }
}

fn photo_detector_providers(device: PhotoClassifierDevice) -> Vec<ep::ExecutionProviderDispatch> {
    let mut providers = Vec::new();

    match device {
        PhotoClassifierDevice::Auto => {
            providers.push(ep::CUDA::default().build());
            if cfg!(target_os = "windows") {
                providers.push(ep::DirectML::default().build());
            }
        }
        PhotoClassifierDevice::Cuda => providers.push(ep::CUDA::default().build()),
        PhotoClassifierDevice::DirectMl => providers.push(ep::DirectML::default().build()),
        PhotoClassifierDevice::Cpu => {}
    }

    providers
}

fn build_photo_detector_session(
    model: &Path,
    providers: &[ep::ExecutionProviderDispatch],
) -> std::result::Result<Session, ort::Error> {
    let mut builder = Session::builder()?;
    if !providers.is_empty() {
        builder = builder.with_execution_providers(providers)?;
    }
    builder.commit_from_file(model)
}

fn process_photo_file(
    input: &Path,
    nightshade_output: Option<&Path>,
    output: &Path,
    quality: u8,
    publish: bool,
    detector: &mut PhotoDetector,
) -> Result<(u32, u32, PhotoClassification)> {
    let image = ImageReader::open(input)?.with_guessed_format()?.decode()?;
    let source = image.to_rgba8();
    let classification = detector.classify(&source)?;
    let (width, height) = if publish {
        let nightshade_output = nightshade_output.ok_or_else(|| {
            SiteError::new("Nightshade output is required before publishing photo")
        })?;
        publish_nightshade_photo(nightshade_output, output, quality)?
    } else {
        source.dimensions()
    };

    Ok((width, height, classification))
}

fn publish_nightshade_photo(input: &Path, output: &Path, quality: u8) -> Result<(u32, u32)> {
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }

    let image = ImageReader::open(input)?.with_guessed_format()?.decode()?;
    let source = image.to_rgba8();
    let (width, height) = source.dimensions();
    let rgb = rgba_to_rgb_on_white(&source);
    let file = File::create(output)?;
    let writer = BufWriter::new(file);
    let mut encoder = JpegEncoder::new_with_quality(writer, quality);
    encoder.encode_image(&rgb)?;

    Ok((width, height))
}

fn photo_model_input(source: &RgbaImage) -> Vec<f32> {
    let rgb = rgba_to_rgb_on_white(source);
    let (width, height) = rgb.dimensions();
    let scale = (PHOTO_MODEL_INPUT_SIZE as f32 / width as f32)
        .min(PHOTO_MODEL_INPUT_SIZE as f32 / height as f32);
    let resized_width = ((width as f32 * scale).round() as u32).clamp(1, PHOTO_MODEL_INPUT_SIZE);
    let resized_height = ((height as f32 * scale).round() as u32).clamp(1, PHOTO_MODEL_INPUT_SIZE);
    let resized =
        image::imageops::resize(&rgb, resized_width, resized_height, FilterType::Triangle);
    let mut letterboxed = RgbImage::from_pixel(
        PHOTO_MODEL_INPUT_SIZE,
        PHOTO_MODEL_INPUT_SIZE,
        Rgb([114, 114, 114]),
    );
    let x_offset = (PHOTO_MODEL_INPUT_SIZE - resized_width) / 2;
    let y_offset = (PHOTO_MODEL_INPUT_SIZE - resized_height) / 2;

    for y in 0..resized_height {
        for x in 0..resized_width {
            let pixel = *resized.get_pixel(x, y);
            letterboxed.put_pixel(x + x_offset, y + y_offset, pixel);
        }
    }

    let capacity = (PHOTO_MODEL_INPUT_SIZE * PHOTO_MODEL_INPUT_SIZE * 3) as usize;
    let mut input = Vec::with_capacity(capacity);
    for channel in 0..3 {
        for y in 0..PHOTO_MODEL_INPUT_SIZE {
            for x in 0..PHOTO_MODEL_INPUT_SIZE {
                input.push(f32::from(letterboxed.get_pixel(x, y)[channel]) / 255.0);
            }
        }
    }

    input
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

fn detections_from_output(
    shape: &ort::value::Shape,
    data: &[f32],
    confidence: f32,
) -> Result<Vec<DetectedObject>> {
    let layout = detection_layout(shape, data.len()).ok_or_else(|| {
        SiteError::new(format!(
            "unsupported object detection output shape {:?} with {} values",
            shape.to_vec(),
            data.len()
        ))
    })?;
    let mut best_by_label = HashMap::<String, f32>::new();

    for candidate in 0..layout.candidates() {
        if layout.channels() == 6 {
            let score = detection_value(data, layout, candidate, 4);
            let class_index = detection_value(data, layout, candidate, 5).round();
            if score.is_finite() && score >= confidence && class_index >= 0.0 {
                if let Some(label) = COCO_LABELS.get(class_index as usize) {
                    update_best_detection(&mut best_by_label, label, score);
                }
            }
            continue;
        }

        let class_offset = if layout.channels() == COCO_LABELS.len() + 5 {
            5
        } else {
            4
        };
        let objectness = if class_offset == 5 {
            detection_value(data, layout, candidate, 4).clamp(0.0, 1.0)
        } else {
            1.0
        };
        let class_count = (layout.channels() - class_offset).min(COCO_LABELS.len());
        let mut best_class = None;
        let mut best_score = confidence;

        for class_index in 0..class_count {
            let class_score =
                detection_value(data, layout, candidate, class_offset + class_index) * objectness;
            if class_score.is_finite() && class_score >= best_score {
                best_score = class_score;
                best_class = Some(class_index);
            }
        }

        if let Some(class_index) = best_class {
            update_best_detection(&mut best_by_label, COCO_LABELS[class_index], best_score);
        }
    }

    let mut detections = best_by_label
        .into_iter()
        .map(|(label, score)| DetectedObject { label, score })
        .collect::<Vec<_>>();
    detections.sort_by(|a, b| {
        b.score
            .total_cmp(&a.score)
            .then_with(|| a.label.cmp(&b.label))
    });

    Ok(detections)
}

fn detection_layout(shape: &ort::value::Shape, data_len: usize) -> Option<DetectionLayout> {
    let dims = shape
        .iter()
        .copied()
        .map(usize::try_from)
        .collect::<std::result::Result<Vec<_>, _>>()
        .ok()?;

    match dims.as_slice() {
        [1, first, second] | [first, second] => {
            if first.checked_mul(*second)? != data_len {
                return None;
            }

            let first_is_channels = looks_like_detection_channels(*first);
            let second_is_channels = looks_like_detection_channels(*second);

            if first_is_channels && (!second_is_channels || second > first) {
                Some(DetectionLayout::ChannelsFirst {
                    channels: *first,
                    candidates: *second,
                })
            } else if second_is_channels {
                Some(DetectionLayout::CandidatesFirst {
                    channels: *second,
                    candidates: *first,
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

fn looks_like_detection_channels(value: usize) -> bool {
    (6..=256).contains(&value)
}

fn update_best_detection(best_by_label: &mut HashMap<String, f32>, label: &str, score: f32) {
    best_by_label
        .entry(label.to_string())
        .and_modify(|best| *best = (*best).max(score))
        .or_insert(score);
}

fn detection_value(data: &[f32], layout: DetectionLayout, candidate: usize, channel: usize) -> f32 {
    match layout {
        DetectionLayout::ChannelsFirst { candidates, .. } => data[channel * candidates + candidate],
        DetectionLayout::CandidatesFirst { channels, .. } => data[candidate * channels + channel],
    }
}

impl DetectionLayout {
    fn channels(self) -> usize {
        match self {
            DetectionLayout::ChannelsFirst { channels, .. }
            | DetectionLayout::CandidatesFirst { channels, .. } => channels,
        }
    }

    fn candidates(self) -> usize {
        match self {
            DetectionLayout::ChannelsFirst { candidates, .. }
            | DetectionLayout::CandidatesFirst { candidates, .. } => candidates,
        }
    }
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

fn staged_photo_file_name(base: &str, count: usize, source: &Path) -> String {
    let extension = source
        .extension()
        .and_then(OsStr::to_str)
        .map(|extension| extension.to_ascii_lowercase())
        .filter(|extension| is_safe_extension(extension))
        .unwrap_or_else(|| "jpg".to_string());

    if count == 1 {
        format!("{base}.{extension}")
    } else {
        format!("{base}-{count}.{extension}")
    }
}

fn is_safe_extension(extension: &str) -> bool {
    extension
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
}

fn find_nightshade_output(output_dir: &Path, staged_file_name: &str) -> Option<PathBuf> {
    let direct = output_dir.join(staged_file_name);
    if direct.is_file() {
        return Some(direct);
    }

    let stem = Path::new(staged_file_name).file_stem()?.to_string_lossy();
    for extension in ["jpg", "jpeg", "png", "webp"] {
        let candidate = output_dir.join(format!("{stem}.{extension}"));
        if candidate.is_file() {
            return Some(candidate);
        }
    }

    None
}

fn stage_nightshade_input(source: &Path, target: &Path) -> Result<()> {
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(source, target)?;
    Ok(())
}

fn nightshade_tag(tags: &[String]) -> String {
    tags.first()
        .filter(|tag| !tag.trim().is_empty())
        .cloned()
        .unwrap_or_else(|| "photo".to_string())
}

fn write_nightshade_staging_manifest(
    input_dir: &Path,
    entries: &[NightshadeStagingEntry],
) -> Result<()> {
    fs::create_dir_all(input_dir)?;

    let mut json = serde_json::to_string_pretty(entries)?;
    json.push('\n');
    fs::write(input_dir.join("nightshade-tags.json"), json)?;

    let mut csv = String::from("file,tag,title,source\n");
    for entry in entries {
        csv.push_str(&csv_field(&entry.file));
        csv.push(',');
        csv.push_str(&csv_field(&entry.tag));
        csv.push(',');
        csv.push_str(&csv_field(&entry.title));
        csv.push(',');
        csv.push_str(&csv_field(&entry.source));
        csv.push('\n');
    }
    fs::write(input_dir.join("nightshade-tags.csv"), csv)?;

    Ok(())
}

fn csv_field(value: &str) -> String {
    if value
        .chars()
        .any(|ch| matches!(ch, '"' | ',' | '\n' | '\r'))
    {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn title_from_tags(tags: &[String]) -> String {
    let parts = tags.iter().map(|tag| titleize_tag(tag)).collect::<Vec<_>>();

    match parts.as_slice() {
        [] => "Unclassified photo".to_string(),
        [only] => only.clone(),
        [first, second] => format!("{first} and {second}"),
        _ => {
            let last = parts.last().expect("parts is not empty");
            let leading = parts[..parts.len() - 1].join(", ");
            format!("{leading}, and {last}")
        }
    }
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

    #[test]
    fn titles_photos_from_detected_object_tags() {
        assert_eq!(
            title_from_tags(&["person".to_string(), "bicycle".to_string()]),
            "Person and Bicycle"
        );
        assert_eq!(
            title_from_tags(&[
                "person".to_string(),
                "traffic light".to_string(),
                "tv".to_string()
            ]),
            "Person, Traffic Light, and TV"
        );
        assert_eq!(title_from_tags(&[]), "Unclassified photo");
    }

    #[test]
    fn prepares_nightshade_staging_names_and_tags() {
        assert_eq!(
            staged_photo_file_name("p1073908", 1, Path::new("P1073908.JPG")),
            "p1073908.jpg"
        );
        assert_eq!(
            staged_photo_file_name("p1073908", 2, Path::new("P1073908.PNG")),
            "p1073908-2.png"
        );
        assert_eq!(nightshade_tag(&[]), "photo");
        assert_eq!(nightshade_tag(&["cat".to_string()]), "cat");
        assert_eq!(csv_field("cat, portrait"), "\"cat, portrait\"");
    }

    #[test]
    fn extracts_yolo_channel_first_object_tags() {
        let shape = ort::value::Shape::new([1_i64, 7, 2]);
        let data = vec![
            0.0, 0.0, // x
            0.0, 0.0, // y
            0.0, 0.0, // w
            0.0, 0.0, // h
            0.8, 0.2, // person
            0.1, 0.9, // bicycle
            0.3, 0.2, // car
        ];

        let detections = detections_from_output(&shape, &data, 0.5).unwrap();

        assert_eq!(detections[0].label, "bicycle");
        assert_eq!(detections[1].label, "person");
    }

    #[test]
    fn extracts_postprocessed_row_object_tags() {
        let shape = ort::value::Shape::new([1_i64, 2, 6]);
        let data = vec![
            0.0, 0.0, 0.0, 0.0, 0.7, 2.0, // car
            0.0, 0.0, 0.0, 0.0, 0.4, 0.0, // below threshold
        ];

        let detections = detections_from_output(&shape, &data, 0.5).unwrap();

        assert_eq!(detections.len(), 1);
        assert_eq!(detections[0].label, "car");
    }
}
