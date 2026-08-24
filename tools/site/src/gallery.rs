//! Normalising a `content/misc` gallery into a numbered run of files.
//!
//! A gallery is a directory of media and a manifest of its filenames, and the
//! page captions each item with its stem — so `07.jpg` reads as "trolley
//! problem 07" while a saved-from-the-internet filename reads as itself. The
//! trolley directory was numbered by hand; this does the same to any of them,
//! and does the two things hand-numbering never did: it makes every still a
//! JPEG whatever the source claimed to be, and it drops the metadata, which on
//! a folder of saved images means camera serials, GPS fixes, and editing
//! history belonging to strangers.
//!
//! It also drops the copies. A folder of saved images holds the same picture
//! more than once, under two names and often at two sizes; `duplicates` decides
//! which of those are one picture, and the largest of each set is what stays.
//!
//! Re-encoding is what strips the metadata: decoding to pixels and writing a
//! fresh JPEG leaves nothing of the original file's structure to carry an EXIF
//! block. That also makes it lossy, so a file already in its final shape —
//! right name, JPEG, no metadata segments — is copied rather than run through
//! the encoder again, and rerunning the command costs nothing.

use crate::content::is_gallery_item;
use crate::duplicates::{self, Fingerprint, Removal};
use crate::{remove_dir_if_exists, Result, Site, SiteError};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

/// Matches `process-photos`, and close enough to whatever the trolley pictures
/// were encoded with that the two galleries look like one thing.
const DEFAULT_QUALITY: u8 = 92;

/// Where the rewritten files sit until every one of them has been written.
///
/// The new names collide with the old ones — `00.jpg` can be both a source and
/// a target — so nothing can move into place until all of it exists. The
/// leading dot keeps it out of the listing if the command dies partway.
const STAGING: &str = ".normalize";

/// What the `image` dependency is built to decode; anything else is refused by
/// name rather than misfiled as a still that will never open.
const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp"];

/// What the gallery renders in a `<video>`, kept as it is: re-encoding video
/// would mean depending on ffmpeg for the sake of six files.
const VIDEO_EXTENSIONS: &[&str] = &["mp4", "webm", "mov", "m4v", "ogv"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Action {
    /// Decode and write a fresh JPEG, which is also what drops the metadata.
    Transcode,
    /// Move the bytes unchanged: video, or a still already in its final shape.
    Keep,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Step {
    source: String,
    target: String,
    action: Action,
}

struct Options {
    collections: Vec<String>,
    quality: u8,
    force: bool,
    keep_duplicates: bool,
    dry_run: bool,
}

pub(crate) fn normalize(site: &Site, args: &[String]) -> Result<()> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        return Ok(());
    }

    let options = parse_options(args)?;

    for collection in &options.collections {
        normalize_collection(site, collection, &options)?;
    }

    Ok(())
}

fn normalize_collection(site: &Site, collection: &str, options: &Options) -> Result<()> {
    let title = Site::index_title(collection).ok_or_else(|| {
        SiteError::new(format!(
            "{collection} is not a gallery; the listed ones are {}",
            Site::index_names().join(", ")
        ))
    })?;

    let dir = site.root.join("content/misc").join(collection);
    if !dir.is_dir() {
        return Err(Box::new(SiteError::new(format!(
            "no such gallery directory: {}",
            dir.display()
        ))));
    }

    let staging = dir.join(STAGING);
    remove_dir_if_exists(&staging)?;

    let mut names = Vec::new();
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        if entry.path().is_dir() {
            continue;
        }

        let name = entry.file_name().to_string_lossy().into_owned();
        if is_gallery_item(&name) {
            names.push(name);
        }
    }
    names.sort();

    if names.is_empty() {
        site.warn(&format!("{collection} is empty; nothing to normalize"));
        return Ok(());
    }

    // Before the numbering rather than after: dropping a file renumbers
    // everything below it, and doing it the other way round would rename the
    // whole gallery twice.
    if !options.keep_duplicates {
        let removals = duplicates(&dir, &names)?;

        for removal in &removals {
            println!(
                "{collection}: {} is a copy of {}, removing it",
                removal.dropped, removal.kept
            );
        }

        if !options.dry_run {
            for removal in &removals {
                fs::remove_file(dir.join(&removal.dropped))?;
            }
        }

        names.retain(|name| !removals.iter().any(|removal| &removal.dropped == name));
    }

    let plan = settle(plan(&names)?, &dir, options.force)?;

    for step in &plan {
        if step.source == step.target && step.action == Action::Keep {
            continue;
        }

        let note = match step.action {
            Action::Transcode => "",
            Action::Keep => " (unchanged)",
        };
        println!("{collection}: {} -> {}{note}", step.source, step.target);
    }

    if options.dry_run {
        println!("dry run: {} file(s) left as they are", plan.len());
        return Ok(());
    }

    fs::create_dir_all(&staging)?;
    for step in &plan {
        let source = dir.join(&step.source);
        let staged = staging.join(&step.target);

        match step.action {
            Action::Transcode => transcode(&source, &staged, options.quality)?,
            Action::Keep => {
                fs::copy(&source, &staged)?;
            }
        }
    }

    for step in &plan {
        fs::remove_file(dir.join(&step.source))?;
    }
    for step in &plan {
        fs::rename(staging.join(&step.target), dir.join(&step.target))?;
    }
    fs::remove_dir(&staging)?;

    let count = site.generate_index(collection, title)?;
    println!("{collection}: {count} file(s), index regenerated");
    Ok(())
}

/// Assigns every file its number, in the order the gallery already lists them.
///
/// Video keeps its extension and its place in the run — the trolley problems
/// have `35.mp4` sitting between two jpgs — because the manifest carries names
/// rather than a range, and the page picks its element from the extension.
fn plan(names: &[String]) -> Result<Vec<Step>> {
    let width = number_width(names.len());
    let mut steps = Vec::with_capacity(names.len());

    for (index, name) in names.iter().enumerate() {
        let extension = extension_of(name);
        let extension = extension.as_str();

        let (target, action) = if IMAGE_EXTENSIONS.contains(&extension) {
            (format!("{index:0width$}.jpg"), Action::Transcode)
        } else if VIDEO_EXTENSIONS.contains(&extension) {
            (format!("{index:0width$}.{extension}"), Action::Keep)
        } else {
            return Err(Box::new(SiteError::new(format!(
                "{name}: not gallery media; stills may be {} and video may be {}",
                IMAGE_EXTENSIONS.join(", "),
                VIDEO_EXTENSIONS.join(", ")
            ))));
        };

        steps.push(Step {
            source: name.clone(),
            target,
            action,
        });
    }

    Ok(steps)
}

/// Downgrades a re-encode to a copy where there is nothing left to do.
///
/// A still that is already a JPEG carrying no metadata segments would come out
/// of the encoder a little worse and no cleaner, so it is moved as the bytes it
/// already is. The decision is about the contents and not the name: a gallery
/// renumbers itself whenever anything joins or leaves the front of it, and a
/// rename is no reason to put a picture through the encoder again. `--force`
/// skips this and re-encodes the lot.
fn settle(plan: Vec<Step>, dir: &Path, force: bool) -> Result<Vec<Step>> {
    if force {
        return Ok(plan);
    }

    plan.into_iter()
        .map(|step| {
            if step.action != Action::Transcode {
                return Ok(step);
            }

            let bytes = fs::read(dir.join(&step.source))?;
            Ok(if jpeg_carries_metadata(&bytes) {
                step
            } else {
                Step {
                    action: Action::Keep,
                    ..step
                }
            })
        })
        .collect()
}

/// Which files in the gallery are copies of other files in it.
///
/// Stills are fingerprinted as pictures, which costs a decode each; everything
/// else is fingerprinted as bytes. Both are read here and then again by the
/// re-encode, which is a maintenance command reading a folder twice rather than
/// holding every picture in a gallery in memory at once.
fn duplicates(dir: &Path, names: &[String]) -> Result<Vec<Removal>> {
    let mut files: Vec<(String, Fingerprint)> = Vec::with_capacity(names.len());

    for name in names {
        let path = dir.join(name);
        let fingerprint = if IMAGE_EXTENSIONS.contains(&extension_of(name).as_str()) {
            duplicates::picture(&path)?
        } else {
            duplicates::bytes(&path)?
        };

        files.push((name.clone(), fingerprint));
    }

    Ok(duplicates::removals(&files))
}

#[cfg(feature = "photos")]
fn transcode(source: &Path, target: &Path, quality: u8) -> Result<()> {
    crate::photos::process_photo_file(source, target, quality)?;

    // The encoder writes a JFIF header of its own, which is a segment like any
    // other and the one thing standing between this and the trolley pictures,
    // whose files start at the quantisation table. Dropping it is also what
    // makes rerunning the command free rather than another round of
    // compression: a file with no segments left is one this can leave alone.
    let encoded = fs::read(target)?;
    if let Some(stripped) = without_metadata_segments(&encoded) {
        fs::write(target, stripped)?;
    }

    Ok(())
}

#[cfg(not(feature = "photos"))]
fn transcode(_: &Path, _: &Path, _: u8) -> Result<()> {
    Err(Box::new(SiteError::new(
        "normalize-gallery re-encodes images and needs the `photos` feature; \
         rebuild without --no-default-features, or pass --dry-run",
    )))
}

/// Two digits, as the trolley problems are numbered, widening only if a gallery
/// outgrows what two digits can name.
fn number_width(count: usize) -> usize {
    let mut width = 1;
    let mut last = count.saturating_sub(1);

    while last >= 10 {
        last /= 10;
        width += 1;
    }

    width.max(2)
}

fn extension_of(name: &str) -> String {
    Path::new(name)
        .extension()
        .and_then(OsStr::to_str)
        .map(str::to_ascii_lowercase)
        .unwrap_or_default()
}

/// Whether a JPEG's header holds anything but the picture.
///
/// EXIF, JFIF, Photoshop resource blocks, and XMP all ride in an `APPn` marker,
/// and a comment rides in `COM`; a file with neither has nothing to strip. The
/// scan stops at the scan data, which is where the header ends, and anything it
/// cannot parse counts as metadata — being wrong in that direction costs one
/// re-encode, and being wrong in the other publishes somebody's GPS fix.
fn jpeg_carries_metadata(bytes: &[u8]) -> bool {
    if bytes.len() < 4 || bytes[0] != 0xFF || bytes[1] != 0xD8 {
        return true;
    }

    let mut offset = 2;
    while offset + 1 < bytes.len() {
        if bytes[offset] != 0xFF {
            return true;
        }

        match bytes[offset + 1] {
            // Fill bytes are allowed to pad any marker.
            0xFF => offset += 1,
            // Start of scan: the header ended without any metadata in it.
            0xDA => return false,
            // APPn and COM are the segments metadata travels in.
            0xE0..=0xEF | 0xFE => return true,
            // Standalone markers, which carry no length to skip past.
            0x01 | 0xD0..=0xD7 => offset += 2,
            _ => {
                let Some(length) = bytes
                    .get(offset + 2..offset + 4)
                    .map(|length| usize::from(u16::from_be_bytes([length[0], length[1]])))
                else {
                    return true;
                };

                if length < 2 {
                    return true;
                }
                offset += 2 + length;
            }
        }
    }

    true
}

/// The same JPEG with every `APPn` and `COM` segment removed, or `None` if it
/// is not a JPEG this can walk.
///
/// Everything from the start of the scan is copied across untouched: that is
/// the picture, and a trailer after it belongs to whoever wrote it.
// Only the re-encode calls this, and only the `photos` feature can re-encode;
// the tests below still run either way, which is the point of it living out
// here rather than inside the gate.
#[cfg_attr(not(feature = "photos"), allow(dead_code))]
fn without_metadata_segments(bytes: &[u8]) -> Option<Vec<u8>> {
    if bytes.len() < 4 || bytes[0] != 0xFF || bytes[1] != 0xD8 {
        return None;
    }

    let mut kept = Vec::with_capacity(bytes.len());
    kept.extend_from_slice(&bytes[..2]);
    let mut offset = 2;

    loop {
        if offset + 1 >= bytes.len() || bytes[offset] != 0xFF {
            return None;
        }

        let marker = bytes[offset + 1];
        match marker {
            // Padding before a marker, which the segments below reintroduce as
            // their own leading 0xFF.
            0xFF => offset += 1,
            0xDA => {
                kept.extend_from_slice(&bytes[offset..]);
                return Some(kept);
            }
            0x01 | 0xD0..=0xD7 => {
                kept.extend_from_slice(&bytes[offset..offset + 2]);
                offset += 2;
            }
            _ => {
                let length = bytes
                    .get(offset + 2..offset + 4)
                    .map(|length| usize::from(u16::from_be_bytes([length[0], length[1]])))?;
                let end = offset.checked_add(2)?.checked_add(length)?;
                if length < 2 || end > bytes.len() {
                    return None;
                }

                if !matches!(marker, 0xE0..=0xEF | 0xFE) {
                    kept.extend_from_slice(&bytes[offset..end]);
                }
                offset = end;
            }
        }
    }
}

fn parse_options(args: &[String]) -> Result<Options> {
    let mut collections = Vec::new();
    let mut quality = DEFAULT_QUALITY;
    let mut force = false;
    let mut keep_duplicates = false;
    let mut dry_run = false;
    let mut rest = args.iter();

    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--force" => force = true,
            "--keep-duplicates" => keep_duplicates = true,
            "--dry-run" => dry_run = true,
            "--quality" => {
                let value = rest
                    .next()
                    .ok_or_else(|| SiteError::new("--quality needs a value between 1 and 100"))?;
                quality = value
                    .parse::<u8>()
                    .ok()
                    .filter(|quality| (1..=100).contains(quality))
                    .ok_or_else(|| {
                        SiteError::new(format!("invalid --quality {value:?}: expected 1 to 100"))
                    })?;
            }
            other if other.starts_with('-') => {
                return Err(Box::new(SiteError::new(format!(
                    "unknown normalize-gallery option: {other}"
                ))))
            }
            other => collections.push(other.to_string()),
        }
    }

    if collections.is_empty() {
        return Err(Box::new(SiteError::new(format!(
            "normalize-gallery needs a gallery to normalize; the listed ones are {}",
            Site::index_names().join(", ")
        ))));
    }

    Ok(Options {
        collections,
        quality,
        force,
        keep_duplicates,
        dry_run,
    })
}

fn print_help() {
    println!(
        "\
site normalize-gallery

Renumber a content/misc gallery, re-encode its stills as JPEG, strip their
metadata, and delete the ones that are copies of another. Video keeps its
extension and its place in the numbering.

Usage:
  site normalize-gallery COLLECTION... [--quality N] [--force]
                                       [--keep-duplicates] [--dry-run]

Options:
  --quality N        JPEG quality, 1 to 100 (default {DEFAULT_QUALITY})
  --force            re-encode stills that are already clean
  --keep-duplicates  keep every copy instead of the largest of each
  --dry-run          print what would change without touching anything
"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn numbers_a_gallery_from_zero() {
        let plan = plan(&names(&["b.png", "a.jpg"])).unwrap();

        assert_eq!(plan[0].source, "b.png");
        assert_eq!(plan[0].target, "00.jpg");
        assert_eq!(plan[1].target, "01.jpg");
    }

    #[test]
    fn makes_every_still_a_jpg_whatever_it_claimed_to_be() {
        let plan = plan(&names(&["shot.jpg_large.webp", "meme.PNG"])).unwrap();

        assert_eq!(plan[0].target, "00.jpg");
        assert_eq!(plan[1].target, "01.jpg");
        assert!(plan.iter().all(|step| step.action == Action::Transcode));
    }

    #[test]
    fn keeps_video_in_place_with_its_extension() {
        let plan = plan(&names(&["a.jpg", "clip.MP4", "b.png"])).unwrap();

        assert_eq!(plan[1].target, "01.mp4");
        assert_eq!(plan[1].action, Action::Keep);
    }

    #[test]
    fn refuses_files_that_are_not_gallery_media() {
        assert!(plan(&names(&["notes.txt"])).is_err());
        assert!(plan(&names(&["loop.gif"])).is_err());
        assert!(plan(&names(&["nameless"])).is_err());
    }

    #[test]
    fn widens_the_numbering_only_when_two_digits_run_out() {
        assert_eq!(number_width(0), 2);
        assert_eq!(number_width(64), 2);
        assert_eq!(number_width(100), 2);
        assert_eq!(number_width(101), 3);
        assert_eq!(number_width(1001), 4);
    }

    #[test]
    fn finds_the_metadata_a_saved_image_arrives_with() {
        // SOI, then an APP1 holding EXIF.
        let exif = [0xFF, 0xD8, 0xFF, 0xE1, 0x00, 0x22, b'E', b'x', b'i', b'f'];
        assert!(jpeg_carries_metadata(&exif));

        // SOI, then a JFIF header, which is still a segment to drop.
        let jfif = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, b'J', b'F', b'I', b'F'];
        assert!(jpeg_carries_metadata(&jfif));
    }

    #[test]
    fn sees_nothing_to_strip_in_a_freshly_encoded_jpeg() {
        // SOI, a quantisation table, then the scan: what the encoder writes.
        let clean = [
            0xFF, 0xD8, 0xFF, 0xDB, 0x00, 0x04, 0x00, 0x00, 0xFF, 0xDA, 0x00, 0x02,
        ];
        assert!(!jpeg_carries_metadata(&clean));
    }

    #[test]
    fn treats_anything_it_cannot_parse_as_metadata() {
        assert!(jpeg_carries_metadata(&[]));
        assert!(jpeg_carries_metadata(&[0x89, b'P', b'N', b'G']));
        // A truncated segment length has nothing to skip past.
        assert!(jpeg_carries_metadata(&[0xFF, 0xD8, 0xFF, 0xDB, 0x00]));
    }

    #[test]
    fn strips_the_segments_and_leaves_the_picture() {
        // SOI, an empty JFIF header, an EXIF block, a quantisation table, and
        // the scan. Every length counts its own two bytes.
        let source = [
            0xFF, 0xD8, // SOI
            0xFF, 0xE0, 0x00, 0x02, // APP0, empty
            0xFF, 0xE1, 0x00, 0x03, b'E', // APP1
            0xFF, 0xDB, 0x00, 0x04, 0x11, 0x22, // DQT
            0xFF, 0xDA, 0x00, 0x02, // SOS
            0x99, // scan data
        ];
        let stripped = without_metadata_segments(&source).unwrap();

        assert_eq!(
            stripped,
            vec![0xFF, 0xD8, 0xFF, 0xDB, 0x00, 0x04, 0x11, 0x22, 0xFF, 0xDA, 0x00, 0x02, 0x99]
        );
        // Which is the point of it: what comes out has nothing left to strip,
        // so the next run can leave the file alone.
        assert!(!jpeg_carries_metadata(&stripped));
    }

    #[test]
    fn leaves_a_jpeg_it_cannot_walk_alone() {
        assert!(without_metadata_segments(&[0x89, b'P', b'N', b'G']).is_none());
        // A segment claiming more length than the file has.
        assert!(without_metadata_segments(&[0xFF, 0xD8, 0xFF, 0xE1, 0xFF, 0xFF]).is_none());
        // No start of scan anywhere in it.
        assert!(without_metadata_segments(&[0xFF, 0xD8, 0xFF, 0xDB, 0x00, 0x02]).is_none());
    }

    #[test]
    fn parses_the_options_it_documents() {
        let options = parse_options(&names(&[
            "guesswedoing",
            "--quality",
            "80",
            "--force",
            "--keep-duplicates",
            "--dry-run",
        ]))
        .unwrap();

        assert_eq!(options.collections, vec!["guesswedoing".to_string()]);
        assert_eq!(options.quality, 80);
        assert!(options.force);
        assert!(options.keep_duplicates);
        assert!(options.dry_run);

        // Removing the copies is what the command does unless told otherwise.
        let plain = parse_options(&names(&["guesswedoing"])).unwrap();
        assert!(!plain.keep_duplicates);
    }

    #[test]
    fn refuses_options_it_cannot_honour() {
        assert!(parse_options(&names(&[])).is_err());
        assert!(parse_options(&names(&["trolley", "--quality", "0"])).is_err());
        assert!(parse_options(&names(&["trolley", "--nope"])).is_err());
    }
}
