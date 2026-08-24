//! Media galleries over a directory of files.
//!
//! Nothing here knows what the pictures are of. A gallery is a directory under
//! `content/misc/`, a manifest of the filenames in it — written by `site
//! indices` — and a page that names the directory; the trolley problems and
//! "guess we doing" are two such directories and nothing distinguishes them but
//! their contents.
//!
//! The manifest carries names rather than an index range because the files are
//! not a contiguous run: `63.mp4` sits among sixty-odd jpgs, and probing for
//! each one's extension used to cost a request per item before the first
//! picture appeared.

use crate::random::random_index_core;
use wasm_bindgen::prelude::wasm_bindgen;

/// A one-based index into a gallery of `count` items, or zero when it is empty.
#[wasm_bindgen]
pub fn gallery_random_index(count: u32) -> u32 {
    if count == 0 {
        return 0;
    }

    random_index_core(count) + 1
}

/// Joins a gallery's base path to one of its filenames.
#[wasm_bindgen]
pub fn gallery_media_src(base_path: &str, name: &str) -> String {
    let separator = if base_path.ends_with('/') { "" } else { "/" };
    let name = name.trim_start_matches('/');
    format!("{base_path}{separator}{name}")
}

/// `"video"` or `"image"`, by extension — which is all the page needs to know
/// to pick between a `<video>` and an `<img>`.
#[wasm_bindgen]
pub fn gallery_media_kind(name: &str) -> String {
    let extension = name
        .rsplit_once('.')
        .map(|(_, extension)| extension.to_ascii_lowercase())
        .unwrap_or_default();

    match extension.as_str() {
        "mp4" | "webm" | "mov" | "m4v" | "ogv" => "video",
        _ => "image",
    }
    .to_string()
}

/// The filename without its extension, which is what a numbered gallery shows
/// as a caption. A name with no extension is its own label.
#[wasm_bindgen]
pub fn gallery_media_label(name: &str) -> String {
    match name.rsplit_once('.') {
        Some((stem, _)) if !stem.is_empty() => stem.to_string(),
        _ => name.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_zero_count_random_index_inert() {
        assert_eq!(gallery_random_index(0), 0);
    }

    #[test]
    fn joins_media_paths_without_doubling_the_separator() {
        assert_eq!(
            gallery_media_src("/misc/trolley", "07.jpg"),
            "/misc/trolley/07.jpg"
        );
        assert_eq!(
            gallery_media_src("/misc/trolley/", "12.mp4"),
            "/misc/trolley/12.mp4"
        );
        assert_eq!(
            gallery_media_src("/misc/guesswedoing", "/03.png"),
            "/misc/guesswedoing/03.png"
        );
    }

    #[test]
    fn tells_video_from_image_by_extension() {
        assert_eq!(gallery_media_kind("00.jpg"), "image");
        assert_eq!(gallery_media_kind("63.mp4"), "video");
        assert_eq!(gallery_media_kind("clip.MOV"), "video");
        // Anything unrecognised renders as an image, which fails visibly rather
        // than silently producing a video element that will never play.
        assert_eq!(gallery_media_kind("notes"), "image");
    }

    #[test]
    fn labels_media_by_its_stem() {
        assert_eq!(gallery_media_label("07.jpg"), "07");
        assert_eq!(gallery_media_label("a.b.png"), "a.b");
        assert_eq!(gallery_media_label("plain"), "plain");
        assert_eq!(gallery_media_label(".hidden"), ".hidden");
    }
}
