//! Finding the same picture twice in a gallery.
//!
//! A folder of saved images collects the same thing more than once: the same
//! meme downloaded from two places, the full-size version and the thumbnail
//! somebody reposted, one video pasted three times. None of that is visible in
//! the filenames or the file sizes, and only some of it is visible in the
//! bytes — a picture saved once as PNG and once as JPEG is the same picture and
//! shares not one byte with itself.
//!
//! So stills are compared as pictures: each one is flattened, squashed to a
//! thumbnail, and matched against the others by how far apart their pixels are
//! on average. The threshold is the whole question, and the answer is not
//! obvious from first principles, so it was measured — see [`SAME_PICTURE`].
//! Video is compared byte for byte, because decoding it would mean depending on
//! ffmpeg and because the duplicates that turn up in practice are literal
//! copies.

use crate::Result;
use std::cmp::Reverse;
use std::path::Path;

/// The side of the thumbnail every still is squashed to before comparison.
///
/// Small enough that comparing every pair of a large gallery is still cheap,
/// and large enough to keep the differences that matter: at this size the five
/// versions of one meme template in "guess we doing" — same drawing, different
/// caption — stay comfortably apart.
#[cfg(feature = "photos")]
const THUMBNAIL: u32 = 32;

/// How far apart two thumbnails may be, per colour byte, and still be one
/// picture.
///
/// Measured rather than guessed, against the gallery this was written for. The
/// two genuine duplicates in it — the same meme at 620x1200 and 1058x2048, and
/// another at 996x1274 and 938x1200 — sit at 0.18. The closest pair that is
/// *not* a duplicate, two captions over one template, sits at 2.27. Every other
/// pair is further out than that.
///
/// 1.0 splits those with an order of magnitude of room on either side. Erring
/// low is deliberate: a missed duplicate leaves a file in a gallery, and a false
/// one deletes a picture that is not stored anywhere else.
const SAME_PICTURE: f64 = 1.0;

/// How different two aspect ratios may be before the pictures are held to be
/// different pictures whatever their thumbnails say.
///
/// Squashing to a square throws the shape away, so two unrelated images of very
/// different proportions get more alike than they are. Real duplicates differ in
/// resolution, not in shape — both pairs above match to three decimal places.
const SAME_SHAPE: f64 = 0.02;

/// What one file is compared by.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(not(feature = "photos"), allow(dead_code))]
pub(crate) enum Fingerprint {
    /// A still: its size, and a thumbnail of it as RGB.
    Picture {
        width: u32,
        height: u32,
        thumbnail: Vec<u8>,
    },
    /// Anything not decoded, which today means video: its bytes.
    Bytes(Vec<u8>),
}

impl Fingerprint {
    /// Whether these are two copies of one thing.
    fn matches(&self, other: &Fingerprint) -> bool {
        match (self, other) {
            (
                Fingerprint::Picture {
                    width: aw,
                    height: ah,
                    thumbnail: a,
                },
                Fingerprint::Picture {
                    width: bw,
                    height: bh,
                    thumbnail: b,
                },
            ) => same_shape(*aw, *ah, *bw, *bh) && mean_difference(a, b) < SAME_PICTURE,
            (Fingerprint::Bytes(a), Fingerprint::Bytes(b)) => a == b,
            // A still and a video are not each other whatever else is true.
            _ => false,
        }
    }

    /// What makes one copy the better one to keep: pixels.
    ///
    /// The duplicates worth finding are usually the same picture at two sizes,
    /// and the bigger one is the one the gallery should show. Copies that carry
    /// no resolution — video — all weigh the same, so the first by name wins.
    fn weight(&self) -> u64 {
        match self {
            Fingerprint::Picture { width, height, .. } => u64::from(*width) * u64::from(*height),
            Fingerprint::Bytes(_) => 0,
        }
    }
}

/// One file that is a copy of another, and the one it is a copy of.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Removal {
    pub(crate) dropped: String,
    pub(crate) kept: String,
}

/// Which files to delete, given every file's fingerprint in name order.
///
/// Each file joins the first group it matches, which assumes that two copies of
/// one picture both match any third — true of near-identical images, and the
/// reason the threshold is set where it is rather than loosely. The largest of
/// a group survives; ties go to the first name, so the result does not depend on
/// the order the filesystem happened to hand them over in.
pub(crate) fn removals(files: &[(String, Fingerprint)]) -> Vec<Removal> {
    let mut groups: Vec<Vec<usize>> = Vec::new();

    for (index, (_, fingerprint)) in files.iter().enumerate() {
        match groups
            .iter_mut()
            .find(|group| files[group[0]].1.matches(fingerprint))
        {
            Some(group) => group.push(index),
            None => groups.push(vec![index]),
        }
    }

    let mut removals = Vec::new();
    for group in groups {
        if group.len() < 2 {
            continue;
        }

        // Largest first, and the earlier name on a tie. Spelled as a minimum
        // over a reversed weight because `max_by_key` keeps the *last* of equal
        // keys, which would hand ties to whichever name sorts last.
        let kept = *group
            .iter()
            .min_by_key(|index| (Reverse(files[**index].1.weight()), **index))
            .expect("a group holds at least one file");

        for index in group {
            if index != kept {
                removals.push(Removal {
                    dropped: files[index].0.clone(),
                    kept: files[kept].0.clone(),
                });
            }
        }
    }

    removals
}

/// The average distance between two thumbnails, per byte.
///
/// Thumbnails of different lengths cannot be compared, and saying they are
/// infinitely far apart keeps that from ever reading as a match.
fn mean_difference(a: &[u8], b: &[u8]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return f64::INFINITY;
    }

    let total: u64 = a
        .iter()
        .zip(b)
        .map(|(a, b)| u64::from(a.abs_diff(*b)))
        .sum();

    total as f64 / a.len() as f64
}

fn same_shape(a_width: u32, a_height: u32, b_width: u32, b_height: u32) -> bool {
    if a_height == 0 || b_height == 0 {
        return false;
    }

    let a = f64::from(a_width) / f64::from(a_height);
    let b = f64::from(b_width) / f64::from(b_height);
    (a - b).abs() / a.max(b) <= SAME_SHAPE
}

/// A still's fingerprint: the picture as the gallery would publish it, small.
///
/// Flattening onto white before shrinking is what the re-encode does too, so
/// two copies of one picture that differ only in whether they kept an alpha
/// channel still land on the same thumbnail.
#[cfg(feature = "photos")]
pub(crate) fn picture(path: &Path) -> Result<Fingerprint> {
    use image::ImageReader;
    use image::imageops::{FilterType, resize};

    let image = ImageReader::open(path)?.with_guessed_format()?.decode()?;
    let (width, height) = (image.width(), image.height());
    let flattened = crate::photos::rgba_to_rgb_on_white(&image.to_rgba8());
    let thumbnail = resize(&flattened, THUMBNAIL, THUMBNAIL, FilterType::Lanczos3);

    Ok(Fingerprint::Picture {
        width,
        height,
        thumbnail: thumbnail.into_raw(),
    })
}

#[cfg(not(feature = "photos"))]
pub(crate) fn picture(_: &Path) -> Result<Fingerprint> {
    Err(Box::new(crate::SiteError::new(
        "comparing pictures needs the `photos` feature; rebuild without --no-default-features",
    )))
}

/// Anything compared as bytes rather than as a picture.
pub(crate) fn bytes(path: &Path) -> Result<Fingerprint> {
    Ok(Fingerprint::Bytes(std::fs::read(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn picture_of(width: u32, height: u32, shade: u8) -> Fingerprint {
        Fingerprint::Picture {
            width,
            height,
            thumbnail: vec![shade; 12],
        }
    }

    fn files(items: &[(&str, Fingerprint)]) -> Vec<(String, Fingerprint)> {
        items
            .iter()
            .map(|(name, fingerprint)| (name.to_string(), fingerprint.clone()))
            .collect()
    }

    #[test]
    fn keeps_the_larger_copy_of_one_picture() {
        let removals = removals(&files(&[
            ("small.jpg", picture_of(620, 1200, 40)),
            ("big.jpg", picture_of(1058, 2048, 40)),
        ]));

        assert_eq!(
            removals,
            vec![Removal {
                dropped: "small.jpg".to_string(),
                kept: "big.jpg".to_string(),
            }]
        );
    }

    #[test]
    fn keeps_the_first_name_when_two_copies_are_the_same_size() {
        let removals = removals(&files(&[
            ("a.mp4", Fingerprint::Bytes(vec![1, 2, 3])),
            ("b.mp4", Fingerprint::Bytes(vec![1, 2, 3])),
            ("c.mp4", Fingerprint::Bytes(vec![1, 2, 3])),
        ]));

        assert_eq!(removals.len(), 2);
        assert!(removals.iter().all(|removal| removal.kept == "a.mp4"));
    }

    #[test]
    fn leaves_pictures_that_only_look_alike() {
        // Two captions over one template: close, but past the threshold.
        let a = Fingerprint::Picture {
            width: 498,
            height: 637,
            thumbnail: vec![100; 12],
        };
        let b = Fingerprint::Picture {
            width: 498,
            height: 637,
            thumbnail: vec![103; 12],
        };

        assert!(!a.matches(&b));
        assert!(removals(&files(&[("02.jpg", a), ("12.jpg", b)])).is_empty());
    }

    #[test]
    fn allows_the_drift_that_re_encoding_costs() {
        let a = picture_of(500, 500, 100);
        let b = picture_of(250, 250, 100);
        assert!(a.matches(&b));
    }

    #[test]
    fn refuses_to_match_across_shapes() {
        let wide = Fingerprint::Picture {
            width: 1000,
            height: 200,
            thumbnail: vec![100; 12],
        };
        let tall = Fingerprint::Picture {
            width: 200,
            height: 1000,
            thumbnail: vec![100; 12],
        };

        assert!(!wide.matches(&tall));
    }

    #[test]
    fn never_matches_a_still_against_a_video() {
        let still = picture_of(10, 10, 0);
        let video = Fingerprint::Bytes(vec![0; 12]);

        assert!(!still.matches(&video));
        assert!(!video.matches(&still));
    }

    #[test]
    fn compares_video_byte_for_byte() {
        let a = Fingerprint::Bytes(vec![1, 2, 3]);
        let b = Fingerprint::Bytes(vec![1, 2, 4]);

        assert!(a.matches(&a.clone()));
        assert!(!a.matches(&b));
    }

    #[test]
    fn reports_nothing_for_a_gallery_of_distinct_files() {
        let removals = removals(&files(&[
            ("a.jpg", picture_of(100, 100, 0)),
            ("b.jpg", picture_of(100, 100, 200)),
            ("c.mp4", Fingerprint::Bytes(vec![9])),
        ]));

        assert!(removals.is_empty());
    }

    #[test]
    fn measures_the_distance_between_thumbnails() {
        assert_eq!(mean_difference(&[10, 10], &[10, 10]), 0.0);
        assert_eq!(mean_difference(&[10, 20], &[12, 24]), 3.0);
        // Nothing to compare is not a match.
        assert_eq!(mean_difference(&[], &[]), f64::INFINITY);
        assert_eq!(mean_difference(&[1], &[1, 2]), f64::INFINITY);
    }
}
