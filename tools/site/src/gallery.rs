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
//! A gif is the one thing that gets asked what it is rather than told: one
//! frame is a still like any other, and more than one is an animation, which
//! keeps its extension and its frames the way video keeps its packets.
//!
//! It also drops the copies. A folder of saved images holds the same picture
//! more than once, under two names and often at two sizes; `duplicates` decides
//! which of those are one picture, and the largest of each set is what stays.
//!
//! # Nothing but the picture
//!
//! Every file published from a gallery carries the picture and nothing else,
//! and the three kinds reach that state three ways.
//!
//! A **still** is decoded to pixels and written again as a JPEG, which leaves
//! nothing of the original file's structure to carry an EXIF block, and then
//! has its own encoder's `APPn` header taken off and anything past the
//! end-of-image marker dropped.
//!
//! An **animation** cannot be re-encoded here without losing every frame but
//! one, so its blocks are walked instead and the ones that are not frames —
//! the comment, the plain-text extension, an application extension that is not
//! the loop count, and whatever was appended past the trailer — are left out.
//!
//! A **video** cannot be re-encoded here at all, which would mean an encoder
//! rather than a muxer, so ffmpeg remuxes it without the tags and the data
//! tracks written around the packets. That one is optional: a gallery without
//! video in it never needs it, and its absence is a warning here rather than a
//! refusal. `gallery-from-issue` does refuse, a stranger's video being exactly
//! the file nobody has looked inside.
//!
//! All three ask the same question — would rewriting this change it — and a
//! file the answer is no for is moved rather than rewritten. That is what makes
//! rerunning the command free, and what stops a rename putting a picture
//! through the encoder for a second time.

use crate::content::is_gallery_item;
use crate::duplicates::{self, Fingerprint, Removal};
use crate::{Result, Site, SiteError, fail, remove_dir_if_exists};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Command;

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
const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp", "gif"];

/// The one extension that can hold either of the two things a gallery has.
///
/// A gif of one frame is a still like any other and becomes a JPEG; a gif of
/// several is an animation, and re-encoding it would mean keeping the first
/// frame and throwing the rest away, so it is carried across as it is. Which
/// of the two a file is cannot be read off its name, so [`is_still`] opens it.
const GIF: &str = "gif";

/// What the gallery renders in a `<video>`: remuxed rather than re-encoded,
/// since the packets are not what carries the metadata.
const VIDEO_EXTENSIONS: &[&str] = &["mp4", "webm", "mov", "m4v", "ogv"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Action {
    /// Decode and write a fresh JPEG, which is also what drops the metadata.
    Transcode,
    /// Rewrite the container without the blocks that are not the picture.
    ///
    /// What an animation and a video get, neither being re-encodable here: a
    /// gif keeps every frame and loses its comment and its XMP packet, and a
    /// video keeps every packet and loses the tags a camera wrote around them.
    Strip,
    /// Move the bytes unchanged: nothing in the file left to take out.
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

/// One gallery, with the options a submission wants.
///
/// `gallery-from-issue` settles duplicates itself, before anything is put in
/// the directory, because the scan here deletes the smaller copy of a pair and
/// that is the wrong answer when one of the two is already published under a
/// number people have linked to.
pub(crate) fn normalize_submission(site: &Site, collection: &str) -> Result<()> {
    normalize_collection(
        site,
        collection,
        &Options {
            collections: vec![collection.to_string()],
            quality: DEFAULT_QUALITY,
            force: false,
            keep_duplicates: true,
            dry_run: false,
        },
    )
}

/// The name a file's own bytes say it should have, or `None` for anything a
/// gallery cannot take.
///
/// What a file was called where it came from says nothing, and a GitHub
/// attachment URL does not carry a name at all, so the answer comes from the
/// first few bytes. Every extension this returns is one [`plan`] accepts, which
/// is what makes "whatever `normalize-gallery` takes" a true description of
/// what may be submitted — there is a test below holding the two together.
pub(crate) fn sniff(bytes: &[u8]) -> Option<&'static str> {
    let starts = |prefix: &[u8]| bytes.starts_with(prefix);

    if starts(&[0xFF, 0xD8, 0xFF]) {
        return Some("jpg");
    }
    if starts(b"\x89PNG\r\n\x1a\n") {
        return Some("png");
    }
    if starts(b"GIF87a") || starts(b"GIF89a") {
        return Some(GIF);
    }
    if starts(b"RIFF") && bytes.get(8..12) == Some(b"WEBP") {
        return Some("webp");
    }
    if starts(b"OggS") {
        return Some("ogv");
    }
    // Matroska and webm are one format with two names, and only one of them is
    // a thing a browser plays, so the doctype is what decides rather than the
    // magic — an mkv renamed to webm is a file that does not open.
    if starts(&[0x1A, 0x45, 0xDF, 0xA3]) {
        return bytes
            .get(..64)
            .unwrap_or(bytes)
            .windows(4)
            .any(|window| window == b"webm")
            .then_some("webm");
    }
    if bytes.get(4..8) == Some(b"ftyp") {
        // QuickTime and mp4 share the container and differ in the brand.
        return Some(if bytes.get(8..12) == Some(b"qt  ") {
            "mov"
        } else {
            "mp4"
        });
    }

    None
}

/// Whether an extension names something the gallery renders in a `<video>`,
/// which is also the answer to whether ffmpeg has to be there to clean it.
pub(crate) fn is_video(extension: &str) -> bool {
    VIDEO_EXTENSIONS.contains(&extension)
}

/// What a page can say a gallery accepts, in the order the reader meets them.
pub(crate) fn accepted_extensions() -> String {
    let mut all = IMAGE_EXTENSIONS.to_vec();
    all.extend_from_slice(VIDEO_EXTENSIONS);
    all.join(", ")
}

/// Whether adding these files would move anything already in the gallery.
///
/// Numbering follows the sorted order of the directory, so a gallery crossing
/// a hundred files widens every name in it, and a gallery that was never
/// normalized renumbers on the first run. Either is fine to do by hand and
/// neither is fine to do on a stranger's behalf: it changes the address of
/// every file below the change, and the pages linking to them do not follow.
pub(crate) fn renumbering(existing: &[String], incoming: &[String]) -> Result<Vec<String>> {
    let mut names = existing.to_vec();
    names.extend_from_slice(incoming);
    names.sort();

    Ok(plan(&names)?
        .into_iter()
        .filter(|step| existing.contains(&step.source) && step.target != step.source)
        .map(|step| step.source)
        .collect())
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
        return fail(format!("no such gallery directory: {}", dir.display()));
    }

    let staging = dir.join(STAGING);
    remove_dir_if_exists(&staging)?;

    let mut names = names_in(&dir)?;

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

    // Created before the plan is settled rather than after, because settling it
    // is what rewrites the video, and that has to land somewhere.
    fs::create_dir_all(&staging)?;
    let plan = settle(site, plan(&names)?, &dir, &staging, options.force)?;

    for step in &plan {
        if step.source == step.target && step.action == Action::Keep {
            continue;
        }

        let note = match step.action {
            Action::Transcode => "",
            Action::Strip => " (metadata stripped)",
            Action::Keep => " (unchanged)",
        };
        println!("{collection}: {} -> {}{note}", step.source, step.target);
    }

    if options.dry_run {
        remove_dir_if_exists(&staging)?;
        println!("dry run: {} file(s) left as they are", plan.len());
        return Ok(());
    }

    for step in &plan {
        let source = dir.join(&step.source);
        let staged = staging.join(&step.target);

        match step.action {
            Action::Transcode => transcode(&source, &staged, options.quality)?,
            // A gif is rewritten here; a video already was, by `settle`, which
            // had to write it in order to find out whether it needed writing.
            Action::Strip if extension_of(&step.source) == GIF => {
                strip_gif(&source, &staged)?;
            }
            Action::Strip => {}
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

/// Everything in a gallery directory that the gallery publishes, sorted, which
/// is the order the numbering follows.
pub(crate) fn names_in(dir: &Path) -> Result<Vec<String>> {
    let mut names = Vec::new();

    for entry in fs::read_dir(dir)? {
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
    Ok(names)
}

/// Assigns every file its number, in the order the gallery already lists them.
///
/// Video keeps its extension and its place in the run — the trolley problems
/// have `35.mp4` sitting between two jpgs — because the manifest carries names
/// rather than a range, and the page picks its element from the extension. A
/// gif is planned as one too, and [`settle`] turns the ones holding a single
/// frame back into stills; that way nothing here has to open a file.
fn plan(names: &[String]) -> Result<Vec<Step>> {
    let width = number_width(names.len());
    let mut steps = Vec::with_capacity(names.len());

    for (index, name) in names.iter().enumerate() {
        let extension = extension_of(name);
        let extension = extension.as_str();

        let (target, action) = if extension == GIF {
            (format!("{index:0width$}.{GIF}"), Action::Keep)
        } else if IMAGE_EXTENSIONS.contains(&extension) {
            (format!("{index:0width$}.jpg"), Action::Transcode)
        } else if VIDEO_EXTENSIONS.contains(&extension) {
            (format!("{index:0width$}.{extension}"), Action::Keep)
        } else {
            return fail(format!(
                "{name}: not gallery media; stills may be {} and video may be {}",
                IMAGE_EXTENSIONS.join(", "),
                VIDEO_EXTENSIONS.join(", ")
            ));
        };

        steps.push(Step {
            source: name.clone(),
            target,
            action,
        });
    }

    Ok(steps)
}

/// Settles the two questions a filename cannot answer, by opening the file.
///
/// The first is whether a gif holds a picture or an animation: one frame is a
/// still and becomes a JPEG like any other, and more than one stays the gif it
/// is, since there is no re-encoding an animation into a still without losing
/// most of it.
///
/// The second is whether there is anything left to take out. Every file the
/// gallery publishes has to end up carrying the picture and nothing else, and
/// the three kinds get there by three different routes: a still is decoded and
/// written afresh, an animation and a video have their containers rewritten
/// without the blocks that are not frames. A file already in that state is
/// moved as the bytes it is — a still that went through the encoder again would
/// come out a little worse and no cleaner, and a gallery renumbers itself
/// whenever anything joins or leaves the front of it, which is no reason to
/// recompress the lot. `--force` does the work anyway, for when the question is
/// whether this is right rather than whether it is needed.
/// Every one of the three asks the same question — would rewriting this change
/// it — and only the video has to do the rewriting to find out. That one is
/// left in `staging` under its new name, which is where it was going anyway.
///
/// `--force` reaches only the stills, because the other two answers are exact
/// rather than a judgement about whether the work is worth doing.
fn settle(
    site: &Site,
    plan: Vec<Step>,
    dir: &Path,
    staging: &Path,
    force: bool,
) -> Result<Vec<Step>> {
    plan.into_iter()
        .map(|step| {
            let extension = extension_of(&step.source);
            let path = dir.join(&step.source);

            if extension == GIF {
                let bytes = fs::read(&path)?;
                if !gif_is_animated(&bytes) {
                    return Ok(Step {
                        target: with_jpg_extension(&step.target),
                        action: Action::Transcode,
                        ..step
                    });
                }

                return Ok(Step {
                    action: strip_or_keep(gif_carries_metadata(&bytes)),
                    ..step
                });
            }

            if VIDEO_EXTENSIONS.contains(&extension.as_str()) {
                let stripped = staging.join(&step.target);
                strip_video(site, &path, &stripped)?;

                // Which is also what makes this idempotent, and the reason the
                // question is asked this way round rather than by reading the
                // container: ffmpeg writes boxes of its own, so a file that
                // "has metadata boxes in it" is what a stripped file looks
                // like, and normalizing twice would rewrite it every time.
                if fs::read(&stripped)? == fs::read(&path)? {
                    fs::remove_file(&stripped)?;
                    return Ok(Step {
                        action: Action::Keep,
                        ..step
                    });
                }

                return Ok(Step {
                    action: Action::Strip,
                    ..step
                });
            }

            if force {
                return Ok(step);
            }

            let bytes = fs::read(&path)?;
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

fn strip_or_keep(carries_metadata: bool) -> Action {
    if carries_metadata {
        Action::Strip
    } else {
        Action::Keep
    }
}

/// Whether the gallery holds this file as a picture rather than as something it
/// can only carry across: a still, or a gif of one frame.
///
/// Every caller wants the same answer — the re-encode and the duplicate scan
/// have to agree about what a gif is, or a gif would be fingerprinted as bytes
/// and then transcoded, or compared as a picture and then kept whole.
fn is_still(dir: &Path, name: &str) -> Result<bool> {
    let extension = extension_of(name);
    if extension == GIF {
        return Ok(!gif_is_animated(&fs::read(dir.join(name))?));
    }

    Ok(IMAGE_EXTENSIONS.contains(&extension.as_str()))
}

/// The same numbered name as a JPEG, for a gif that turned out to be a still.
fn with_jpg_extension(target: &str) -> String {
    let stem = target.rsplit_once('.').map_or(target, |(stem, _)| stem);
    format!("{stem}.jpg")
}

/// Which files in the gallery are copies of other files in it.
///
/// Stills are fingerprinted as pictures, which costs a decode each; everything
/// else — video, and a gif that moves — is fingerprinted as bytes, so an
/// animation is never weighed against the first frame of itself. Both are read
/// here and then again by the re-encode, which is a maintenance command reading
/// a folder twice rather than holding every picture in a gallery in memory at
/// once.
pub(crate) fn duplicates(dir: &Path, names: &[String]) -> Result<Vec<Removal>> {
    let mut files: Vec<(String, Fingerprint)> = Vec::with_capacity(names.len());

    for name in names {
        let path = dir.join(name);
        let fingerprint = if is_still(dir, name)? {
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
    fail(
        "normalize-gallery re-encodes images and needs the `photos` feature; \
         rebuild without --no-default-features, or pass --dry-run",
    )
}

/// Carries an animation across without the blocks that are not its frames.
fn strip_gif(source: &Path, target: &Path) -> Result<()> {
    let bytes = fs::read(source)?;

    match gif_without_metadata(&bytes) {
        Some(stripped) => Ok(fs::write(target, stripped)?),
        // Nothing asks for a strip unless that walk already got through the
        // file once, so this is unreachable; copying is the harmless way to be
        // wrong about it.
        None => {
            fs::copy(source, target)?;
            Ok(())
        }
    }
}

/// Remuxes a video without the tags written around it.
///
/// `-c copy` and nothing else: re-encoding would mean an encoder as well as a
/// muxer, and the packets are not what carries the metadata. What goes is the
/// container's own tags, the chapters, and — through mapping only the streams
/// that are played — any data track alongside them, which on a phone is a
/// `mebx` track of where and how the thing was held. `+bitexact` stops ffmpeg
/// stamping its own version into the output, so running this twice writes the
/// same bytes twice and rerunning the command stays free.
///
/// ffmpeg is optional here, since it is only needed by a gallery that has video
/// in it with something to take out. Its absence is a warning rather than an
/// error, because refusing to normalize a gallery over it would be worse — but
/// `gallery-from-issue` does refuse, a stranger's video being exactly the case
/// where nobody has looked at what is in the file.
fn strip_video(site: &Site, source: &Path, target: &Path) -> Result<()> {
    let output = Command::new("ffmpeg")
        .args(["-hide_banner", "-loglevel", "error", "-nostdin", "-y", "-i"])
        .arg(source)
        .args([
            "-map",
            "0:v?",
            "-map",
            "0:a?",
            "-map",
            "0:s?",
            "-map_metadata",
            "-1",
            "-map_chapters",
            "-1",
            "-c",
            "copy",
            "-fflags",
            "+bitexact",
        ])
        .arg(target)
        .output();

    match output {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => fail(format!(
            "ffmpeg could not strip {}: {}",
            source.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        )),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            site.warn(&format!(
                "{} carries container metadata and ffmpeg is not installed, \
                 so it is being published as it is",
                source.display()
            ));
            fs::copy(source, target)?;
            Ok(())
        }
        Err(error) => Err(Box::new(error)),
    }
}

/// Whether ffmpeg is on the path, for a caller that would rather refuse than
/// publish a video it cannot clean.
pub(crate) fn ffmpeg_available() -> bool {
    Command::new("ffmpeg")
        .args(["-hide_banner", "-loglevel", "error", "-version"])
        .output()
        .is_ok_and(|output| output.status.success())
}

/// The identifiers of the two application extensions that are part of an
/// animation rather than something written alongside it.
///
/// Both say how many times the gif loops. Dropping one turns a looping gif into
/// a gif that plays once, which is why application extensions are not simply
/// thrown away as a class — XMP packets ride in one of these too.
const GIF_LOOP_EXTENSIONS: [&[u8; 11]; 2] = [b"NETSCAPE2.0", b"ANIMEXTS1.0"];

/// Whether a gif holds anything the animation does not need.
///
/// Unparseable counts as carrying nothing, which is the opposite of the answer
/// [`jpeg_carries_metadata`] gives — and for the same reason. A JPEG that
/// cannot be walked is re-encoded, which fixes it; a gif that cannot be walked
/// cannot be rewritten either, so claiming there is something to take out would
/// only mean rewriting it wrong.
fn gif_carries_metadata(bytes: &[u8]) -> bool {
    gif_without_metadata(bytes).is_some_and(|stripped| stripped != bytes)
}

/// The same gif with every block that is not part of the animation removed:
/// the comment, the plain-text extension, any application extension that is not
/// the loop count, and anything appended after the trailer.
///
/// `None` if this is not a gif this can walk, in which case it is carried
/// across as it is rather than guessed at.
fn gif_without_metadata(bytes: &[u8]) -> Option<Vec<u8>> {
    let start = gif_blocks_offset(bytes)?;
    let mut kept = Vec::with_capacity(bytes.len());
    kept.extend_from_slice(&bytes[..start]);
    let mut offset = start;

    loop {
        match *bytes.get(offset)? {
            // The trailer, and the end of everything worth keeping: whatever
            // somebody appended past it is not part of the picture.
            0x3B => {
                kept.push(0x3B);
                return Some(kept);
            }
            0x21 => {
                let label = *bytes.get(offset + 1)?;
                let end = gif_skip_sub_blocks(bytes, offset + 2)?;
                let keep = match label {
                    // Graphic control: this frame's delay and transparency.
                    0xF9 => true,
                    // Application: the loop count, or somebody's metadata.
                    0xFF => bytes.get(offset + 3..offset + 14).is_some_and(|id| {
                        GIF_LOOP_EXTENSIONS
                            .iter()
                            .any(|identifier| id == identifier.as_slice())
                    }),
                    // Comment, plain text, and anything else nobody draws.
                    _ => false,
                };

                if keep {
                    kept.extend_from_slice(bytes.get(offset..end)?);
                }
                offset = end;
            }
            // An image descriptor and the frame behind it, kept whole. Ten
            // bytes, the last of them the packed field saying whether a local
            // colour table follows, then the code size and the frame's data.
            0x2C => {
                let packed = *bytes.get(offset + 9)?;
                let data = offset + 10 + gif_colour_table_len(packed) + 1;
                let end = gif_skip_sub_blocks(bytes, data)?;
                kept.extend_from_slice(bytes.get(offset..end)?);
                offset = end;
            }
            _ => return None,
        }
    }
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

/// Whether a JPEG holds anything but the picture.
///
/// Phrased as "would stripping it change it", so that the question and the
/// answer cannot drift apart — one walk of the file decides both. EXIF, JFIF,
/// Photoshop resource blocks and XMP all ride in an `APPn` marker, a comment
/// rides in `COM`, and anything appended past the end-of-image marker rides in
/// no marker at all. A file this cannot walk counts as carrying metadata:
/// being wrong in that direction costs one re-encode, and being wrong in the
/// other publishes somebody's GPS fix.
fn jpeg_carries_metadata(bytes: &[u8]) -> bool {
    without_metadata_segments(bytes).is_none_or(|stripped| stripped != bytes)
}

/// The same JPEG with every `APPn` and `COM` segment removed and everything
/// past the end-of-image marker dropped, or `None` if it is not a JPEG this
/// can walk.
///
/// The trailer goes because a JPEG ends at `FFD9` and a file that carries on
/// afterwards is carrying something somebody else put there — a second image,
/// a zip, a thumbnail, an XMP packet no reader would show. The scan data
/// itself is copied through untouched; that is the picture.
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
            // End of image. A progressive JPEG reaches this after several
            // scans; either way, the file is over.
            0xD9 => {
                kept.extend_from_slice(&bytes[offset..offset + 2]);
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

                // A scan header is followed by entropy-coded data rather than
                // by the next marker, and that data is the only part of a JPEG
                // that has to be walked a byte at a time.
                if marker == 0xDA {
                    let scan_end = jpeg_end_of_scan(bytes, end)?;
                    kept.extend_from_slice(&bytes[end..scan_end]);
                    offset = scan_end;
                }
            }
        }
    }
}

/// Where a scan's entropy-coded data ends: at the first marker inside it that
/// is neither a stuffed `0xFF` nor a restart.
///
/// A literal `0xFF` byte in the compressed data is written as `FF 00`, and
/// `FFD0` through `FFD7` punctuate the scan itself, so neither ends it. Every
/// other `FF` does — in a well-formed file that is `FFD9`, and in a progressive
/// one it can be the next scan's tables.
fn jpeg_end_of_scan(bytes: &[u8], mut offset: usize) -> Option<usize> {
    loop {
        let at = offset + bytes[offset..].iter().position(|&byte| byte == 0xFF)?;

        match *bytes.get(at + 1)? {
            0x00 | 0xD0..=0xD7 => offset = at + 2,
            _ => return Some(at),
        }
    }
}

/// Whether a gif holds more than one frame.
///
/// Walking the blocks rather than decoding: the question is how many image
/// descriptors the file has, and stopping at the second means never inflating a
/// frame. It also keeps this compiling and testable without the `photos`
/// feature, which CI builds without.
///
/// Anything unparseable counts as animated, because that is the harmless
/// direction to be wrong in: an animation kept as a gif is a file that still
/// plays, and a still kept as a gif is a slightly larger file, while an
/// animation mistaken for a still loses every frame but the first.
fn gif_is_animated(bytes: &[u8]) -> bool {
    let Some(mut offset) = gif_blocks_offset(bytes) else {
        return true;
    };
    let mut frames = 0;

    loop {
        let Some(&block) = bytes.get(offset) else {
            return true;
        };
        offset += 1;

        match block {
            // Trailer: the file ended, having held at most the one frame.
            0x3B => return false,
            // An extension is its label and then a run of sub-blocks. Graphic
            // control, comment, application: none of them is a frame.
            0x21 => {
                let Some(next) = gif_skip_sub_blocks(bytes, offset + 1) else {
                    return true;
                };
                offset = next;
            }
            // An image descriptor is a frame: nine bytes, the last of them the
            // packed field saying whether a local colour table follows, then
            // the LZW code size and the frame's own sub-blocks.
            0x2C => {
                frames += 1;
                if frames > 1 {
                    return true;
                }

                let Some(&packed) = bytes.get(offset + 8) else {
                    return true;
                };
                let data = offset + 9 + gif_colour_table_len(packed) + 1;
                let Some(next) = gif_skip_sub_blocks(bytes, data) else {
                    return true;
                };
                offset = next;
            }
            _ => return true,
        }
    }
}

/// Where a gif's blocks start: past the header, the screen descriptor, and the
/// global colour table if it has one. `None` if this is not a gif.
fn gif_blocks_offset(bytes: &[u8]) -> Option<usize> {
    if !bytes.starts_with(b"GIF87a") && !bytes.starts_with(b"GIF89a") {
        return None;
    }

    // Six bytes of header, then width, height, and the packed field.
    let packed = *bytes.get(10)?;
    let offset = 13 + gif_colour_table_len(packed);
    (offset <= bytes.len()).then_some(offset)
}

/// The size of the colour table a packed field describes, in bytes: three per
/// entry, and two to the power of one more than the low three bits of entries.
fn gif_colour_table_len(packed: u8) -> usize {
    if packed & 0x80 == 0 {
        0
    } else {
        3 * (1 << ((packed & 0x07) + 1))
    }
}

/// Past a run of length-prefixed sub-blocks and the zero that ends it.
fn gif_skip_sub_blocks(bytes: &[u8], mut offset: usize) -> Option<usize> {
    loop {
        let length = usize::from(*bytes.get(offset)?);
        offset = offset.checked_add(1)?.checked_add(length)?;
        if offset > bytes.len() {
            return None;
        }
        if length == 0 {
            return Some(offset);
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
                return fail(format!("unknown normalize-gallery option: {other}"));
            }
            other => collections.push(other.to_string()),
        }
    }

    if collections.is_empty() {
        return fail(format!(
            "normalize-gallery needs a gallery to normalize; the listed ones are {}",
            Site::index_names().join(", ")
        ));
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

Renumber a content/misc gallery and leave every file in it carrying the picture
and nothing else: stills are re-encoded as JPEG, animations lose the blocks that
are not frames, and video is remuxed without its tags. Copies of another file
are deleted. Video, and any gif that moves, keeps its extension and its place in
the numbering.

Remuxing video needs ffmpeg on the path; without it a video that has tags in it
is published as it is, with a warning.

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
    use std::process;

    fn names(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    /// Only ever asked to print a warning here, so it needs no root.
    fn site() -> Site {
        Site {
            root: std::path::PathBuf::new(),
            ci: false,
        }
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
        assert!(plan(&names(&["nameless"])).is_err());
    }

    #[test]
    fn plans_a_gif_as_a_gif_until_something_has_opened_it() {
        let plan = plan(&names(&["a.jpg", "loop.GIF"])).unwrap();

        assert_eq!(plan[1].target, "01.gif");
        assert_eq!(plan[1].action, Action::Keep);
        // And the still it may turn out to be keeps the number it was given.
        assert_eq!(with_jpg_extension(&plan[1].target), "01.jpg");
        assert_eq!(with_jpg_extension("07"), "07.jpg");
    }

    /// A directory of files, cleaned up whatever the test does.
    struct Fixture {
        dir: std::path::PathBuf,
    }

    impl Fixture {
        fn new(name: &str, files: &[(&str, Vec<u8>)]) -> Self {
            let dir = std::env::temp_dir().join(format!("site-gallery-{name}-{}", process::id()));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            for (name, bytes) in files {
                fs::write(dir.join(name), bytes).unwrap();
            }
            Self { dir }
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }

    #[test]
    fn opens_a_gif_to_decide_which_of_the_two_it_is() {
        let fixture = Fixture::new("gifs", &[("still.gif", gif(1)), ("animated.gif", gif(3))]);
        let plan = settle(
            &site(),
            plan(&names(&["animated.gif", "still.gif"])).unwrap(),
            &fixture.dir,
            // Only a video is written while the plan is settled, and there is
            // none here, so this directory is never created.
            &fixture.dir.join(STAGING),
            false,
        )
        .unwrap();

        // The animation keeps its bytes and its extension, as video does.
        assert_eq!(plan[0].target, "00.gif");
        assert_eq!(plan[0].action, Action::Keep);
        // The one-frame gif is a still, and becomes a JPEG under its number.
        assert_eq!(plan[1].target, "01.jpg");
        assert_eq!(plan[1].action, Action::Transcode);

        assert!(is_still(&fixture.dir, "still.gif").unwrap());
        assert!(!is_still(&fixture.dir, "animated.gif").unwrap());
    }

    /// The animation has to survive `--force` too: that flag is about
    /// re-encoding a picture, and there is no re-encoding this into a still.
    #[test]
    fn keeps_an_animation_whole_even_when_forced() {
        let fixture = Fixture::new("forced", &[("00.gif", gif(2))]);
        let plan = settle(
            &site(),
            plan(&names(&["00.gif"])).unwrap(),
            &fixture.dir,
            &fixture.dir.join(STAGING),
            true,
        )
        .unwrap();

        assert_eq!(plan[0].target, "00.gif");
        assert_eq!(plan[0].action, Action::Keep);
    }

    #[cfg(feature = "photos")]
    #[test]
    fn re_encodes_a_one_frame_gif_into_a_clean_jpeg() {
        let fixture = Fixture::new("transcode", &[]);
        let source = fixture.dir.join("still.gif");
        let target = fixture.dir.join("00.jpg");
        image::RgbaImage::from_pixel(4, 4, image::Rgba([10, 200, 30, 255]))
            .save(&source)
            .unwrap();

        assert!(is_still(&fixture.dir, "still.gif").unwrap());
        transcode(&source, &target, DEFAULT_QUALITY).unwrap();

        let encoded = fs::read(&target).unwrap();
        assert_eq!(&encoded[..2], &[0xFF, 0xD8]);
        assert!(!jpeg_carries_metadata(&encoded));
    }

    /// What may be submitted is exactly what this command can take, so the two
    /// answers have to be the same answer. Every extension the sniffer returns
    /// is put through the planner here; a kind added to one and not the other
    /// shows up as a submission accepted and then refused, or refused and then
    /// silently misfiled.
    #[test]
    fn only_ever_names_a_kind_the_planner_accepts() {
        let files: [(&[u8], &str); 8] = [
            (&[0xFF, 0xD8, 0xFF, 0xE0], "jpg"),
            (b"\x89PNG\r\n\x1a\n", "png"),
            (b"GIF89a and so on", GIF),
            (b"RIFF\0\0\0\0WEBPVP8 ", "webp"),
            (b"OggS\0\x02\0\0", "ogv"),
            (b"\x1A\x45\xDF\xA3 doctype webm follows", "webm"),
            (b"\0\0\0\x20ftypisom", "mp4"),
            (b"\0\0\0\x14ftypqt  ", "mov"),
        ];

        for (bytes, extension) in files {
            assert_eq!(sniff(bytes), Some(extension));
            assert!(
                plan(&names(&[&format!("submission-00.{extension}")])).is_ok(),
                "{extension} is sniffed and then refused by the planner"
            );
        }

        // Matroska is the same container under another name, and not one the
        // page can play; the doctype is what tells them apart.
        assert_eq!(sniff(b"\x1A\x45\xDF\xA3 doctype matroska here"), None);
        // And the things people try to attach that are not pictures at all.
        assert_eq!(sniff(b"%PDF-1.7"), None);
        assert_eq!(sniff(b"<svg xmlns="), None);
        assert_eq!(sniff(b"PK\x03\x04"), None);
        assert_eq!(sniff(&[]), None);
    }

    #[test]
    fn refuses_to_renumber_what_is_already_published() {
        let existing = names(&["00.jpg", "01.mp4", "02.jpg"]);

        // The ordinary case: the new file lands at the end and nothing moves.
        assert!(
            renumbering(&existing, &names(&["submission-00.png"]))
                .unwrap()
                .is_empty()
        );

        // A gallery whose numbering has a hole in it renumbers on the way
        // through, which is a thing to do by hand rather than to a stranger.
        let holed = names(&["00.jpg", "02.jpg"]);
        assert_eq!(
            renumbering(&holed, &names(&["submission-00.png"])).unwrap(),
            vec!["02.jpg".to_string()]
        );

        // And so does the one that takes a full gallery past what two digits
        // can name, where every file in it widens at once.
        let full: Vec<String> = (0..100).map(|index| format!("{index:02}.jpg")).collect();
        assert!(renumbering(&full, &[]).unwrap().is_empty());
        assert_eq!(
            renumbering(&full, &names(&["submission-00.png"]))
                .unwrap()
                .len(),
            100
        );
    }

    #[test]
    fn counts_a_gif_of_one_frame_as_a_still() {
        assert!(!gif_is_animated(&gif(1)));
        assert!(gif_is_animated(&gif(2)));
        assert!(gif_is_animated(&gif(7)));
    }

    #[test]
    fn treats_a_gif_it_cannot_walk_as_an_animation() {
        assert!(gif_is_animated(&[]));
        assert!(gif_is_animated(&[0xFF, 0xD8, 0xFF, 0xE0]));
        // A header and nothing after it: no trailer to say the file ended.
        assert!(gif_is_animated(b"GIF89a     "));
        // A sub-block claiming more bytes than the file has.
        let mut truncated = gif(1);
        truncated.truncate(truncated.len() - 4);
        assert!(gif_is_animated(&truncated));
    }

    #[test]
    fn skips_the_colour_tables_on_its_way_through_a_gif() {
        // The global table is what the offsets below would run into if it were
        // not skipped, and a local one sits inside the frame the same way.
        assert_eq!(gif_colour_table_len(0x00), 0);
        assert_eq!(gif_colour_table_len(0x07), 0);
        assert_eq!(gif_colour_table_len(0x80), 6);
        assert_eq!(gif_colour_table_len(0x87), 768);

        assert!(!gif_is_animated(&gif_with_tables(1)));
        assert!(gif_is_animated(&gif_with_tables(2)));
    }

    /// A gif of `frames` frames: header, screen descriptor with no colour
    /// table, and a graphic control extension and an image descriptor each.
    fn gif(frames: usize) -> Vec<u8> {
        let mut bytes = b"GIF89a".to_vec();
        // Width, height, packed (no global table), background, aspect.
        bytes.extend_from_slice(&[0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00]);

        for _ in 0..frames {
            // Graphic control extension: label, one sub-block, terminator.
            bytes.extend_from_slice(&[0x21, 0xF9, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00]);
            // Image descriptor: left, top, width, height, packed.
            bytes.extend_from_slice(&[0x2C, 0, 0, 0, 0, 0x01, 0x00, 0x01, 0x00, 0x00]);
            // LZW code size, one sub-block of data, terminator.
            bytes.extend_from_slice(&[0x02, 0x02, 0x4C, 0x01, 0x00]);
        }

        bytes.push(0x3B);
        bytes
    }

    /// The same, with a global colour table and a local one per frame.
    fn gif_with_tables(frames: usize) -> Vec<u8> {
        let mut bytes = b"GIF89a".to_vec();
        bytes.extend_from_slice(&[0x01, 0x00, 0x01, 0x00, 0x80, 0x00, 0x00]);
        bytes.extend_from_slice(&[0xFF; 6]);

        for _ in 0..frames {
            bytes.extend_from_slice(&[0x2C, 0, 0, 0, 0, 0x01, 0x00, 0x01, 0x00, 0x80]);
            bytes.extend_from_slice(&[0xFF; 6]);
            bytes.extend_from_slice(&[0x02, 0x02, 0x4C, 0x01, 0x00]);
        }

        bytes.push(0x3B);
        bytes
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
        // SOI, a quantisation table, the scan, and the end: what the encoder
        // writes, and every byte of it the picture.
        let clean = [
            0xFF, 0xD8, 0xFF, 0xDB, 0x00, 0x04, 0x00, 0x00, 0xFF, 0xDA, 0x00, 0x02, 0x99, 0xFF,
            0xD9,
        ];
        assert!(!jpeg_carries_metadata(&clean));
    }

    #[test]
    fn treats_anything_it_cannot_parse_as_metadata() {
        assert!(jpeg_carries_metadata(&[]));
        assert!(jpeg_carries_metadata(&[0x89, b'P', b'N', b'G']));
        // A truncated segment length has nothing to skip past.
        assert!(jpeg_carries_metadata(&[0xFF, 0xD8, 0xFF, 0xDB, 0x00]));
        // A scan that never reaches the end of the image: the file was cut off
        // somewhere, and re-encoding is what makes it whole again.
        assert!(jpeg_carries_metadata(&[
            0xFF, 0xD8, 0xFF, 0xDA, 0x00, 0x02, 0x99
        ]));
    }

    #[test]
    fn strips_the_segments_and_leaves_the_picture() {
        // SOI, an empty JFIF header, an EXIF block, a quantisation table, the
        // scan, and the end. Every length counts its own two bytes.
        let source = [
            0xFF, 0xD8, // SOI
            0xFF, 0xE0, 0x00, 0x02, // APP0, empty
            0xFF, 0xE1, 0x00, 0x03, b'E', // APP1
            0xFF, 0xDB, 0x00, 0x04, 0x11, 0x22, // DQT
            0xFF, 0xDA, 0x00, 0x02, // SOS
            0x99, 0xFF, 0x00, 0xFF, 0xD0, 0x99, // scan data, a stuffed byte, a restart
            0xFF, 0xD9, // EOI
        ];
        let stripped = without_metadata_segments(&source).unwrap();

        assert_eq!(
            stripped,
            vec![
                0xFF, 0xD8, 0xFF, 0xDB, 0x00, 0x04, 0x11, 0x22, 0xFF, 0xDA, 0x00, 0x02, 0x99, 0xFF,
                0x00, 0xFF, 0xD0, 0x99, 0xFF, 0xD9
            ]
        );
        // Which is the point of it: what comes out has nothing left to strip,
        // so the next run can leave the file alone.
        assert!(!jpeg_carries_metadata(&stripped));
    }

    /// A JPEG ends at `FFD9`. Anything after that was appended by something
    /// other than the encoder — another image, an archive, an XMP packet — and
    /// no reader shows it, which is exactly why things get hidden there.
    #[test]
    fn drops_whatever_was_appended_after_the_end_of_the_image() {
        let mut source = vec![
            0xFF, 0xD8, 0xFF, 0xDB, 0x00, 0x04, 0x00, 0x00, 0xFF, 0xDA, 0x00, 0x02, 0x99, 0xFF,
            0xD9,
        ];
        let picture = source.clone();
        source.extend_from_slice(b"PK\x03\x04and a whole zip file");

        assert!(jpeg_carries_metadata(&source));
        assert_eq!(without_metadata_segments(&source).unwrap(), picture);
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
    fn takes_the_blocks_that_are_not_frames_out_of_a_gif() {
        let clean = gif(2);
        assert!(!gif_carries_metadata(&clean));
        assert_eq!(gif_without_metadata(&clean).unwrap(), clean);

        let commented = gif_with_extension(&[0x21, 0xFE, 0x03, b'h', b'i', b'!', 0x00]);
        assert!(gif_carries_metadata(&commented));
        assert_eq!(gif_without_metadata(&commented).unwrap(), gif(2));

        // XMP travels in an application extension, and so does the loop count;
        // only one of them is part of the animation.
        let xmp = gif_with_extension(&[
            0x21, 0xFF, 0x0B, b'X', b'M', b'P', b' ', b'D', b'a', b't', b'a', b'X', b'M', b'P',
            0x00,
        ]);
        assert!(gif_carries_metadata(&xmp));
        assert_eq!(gif_without_metadata(&xmp).unwrap(), gif(2));

        let looping = gif_with_extension(&[
            0x21, 0xFF, 0x0B, b'N', b'E', b'T', b'S', b'C', b'A', b'P', b'E', b'2', b'.', b'0',
            0x03, 0x01, 0x00, 0x00, 0x00,
        ]);
        assert!(!gif_carries_metadata(&looping));
    }

    #[test]
    fn drops_whatever_a_gif_carries_past_its_trailer() {
        let mut source = gif(1);
        source.extend_from_slice(b"appended");

        assert!(gif_carries_metadata(&source));
        assert_eq!(gif_without_metadata(&source).unwrap(), gif(1));
    }

    /// The same gif, with one more extension block in front of the frames.
    fn gif_with_extension(extension: &[u8]) -> Vec<u8> {
        let mut bytes = gif(2);
        // Past the six-byte header and the seven-byte screen descriptor.
        bytes.splice(13..13, extension.iter().copied());
        bytes
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
