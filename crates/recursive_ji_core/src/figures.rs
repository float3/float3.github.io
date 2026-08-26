//! Draws the post's two notation figures as SVG.
//!
//! They used to be ABC strings handed to abcjs in the browser, which cost every
//! reader of the post about a megabyte of JavaScript to draw two pictures that
//! never change. Both come out of fixed data through fixed rules, so they are
//! drawn here instead, once, at build time.
//!
//! The staff, the glyphs and the layout live in the [`engrave`] crate, which
//! the tuning playground draws its chords with too. What is left here is what
//! is particular to the post: which chords, in what order, labelled how.

use music21_rs::Pitch;

use engrave::glyphs::{NOTEHEAD_QUARTER, NOTEHEAD_WHOLE};
use engrave::{CLEF_WIDTH, Canvas, LABEL_HEIGHT, MARGIN, STAFF_HEIGHT, SYSTEM_GAP, staff_note};

use crate::{
    FIXED_C_JUST, RECURSIVE_JUST, Result, cents_between, chord_context_label, format_signed_cents,
    notated_pitches, notation_chord_label, note_frequency, progression, split_pairs,
};

/// The twelve chords of the progression, four to a system.
pub fn chord_progression_svg() -> Result<String> {
    const BARS_PER_SYSTEM: usize = 4;
    const BAR_WIDTH: f64 = 86.0;
    const LABEL_SIZE: f64 = 11.0;

    let chords = progression();
    let systems = chords.len().div_ceil(BARS_PER_SYSTEM);
    let width = MARGIN * 2.0 + CLEF_WIDTH + BAR_WIDTH * BARS_PER_SYSTEM as f64;
    let system_height = LABEL_HEIGHT + STAFF_HEIGHT + SYSTEM_GAP;
    let height = system_height * systems as f64;

    let mut canvas = Canvas::new("rji-prog");

    for (system, bars) in chords.chunks(BARS_PER_SYSTEM).enumerate() {
        let staff_y = system as f64 * system_height + LABEL_HEIGHT;
        let staff_x = MARGIN;
        let staff_width = CLEF_WIDTH + BAR_WIDTH * bars.len() as f64;

        canvas.staff(staff_x, staff_y, staff_width);
        canvas.treble_clef(staff_x, staff_y);
        canvas.barline(staff_x, staff_y);

        for (index, chord) in bars.iter().enumerate() {
            let bar_x = staff_x + CLEF_WIDTH + BAR_WIDTH * index as f64;
            let centre = bar_x + BAR_WIDTH / 2.0;

            canvas.text(
                notation_chord_label(*chord),
                centre,
                staff_y - 10.0,
                LABEL_SIZE,
                "notation-chord",
            );

            for pitch in notated_pitches(*chord)? {
                let note = staff_note(&pitch)?;
                canvas.note(&note, &NOTEHEAD_WHOLE, centre, staff_y);
            }

            let barline_x = bar_x + BAR_WIDTH;
            if system == systems - 1 && index + 1 == bars.len() {
                canvas.final_barline(barline_x, staff_y);
            } else {
                canvas.barline(barline_x, staff_y);
            }
        }
    }

    Ok(canvas.finish(
        width,
        height,
        "The twelve-chord progression, written on a treble staff",
    ))
}

/// The four pitch-name splits: each written pitch three times over, as the fixed
/// tuning has it, as the recursive tuning has it, and the two together.
pub fn note_splits_svg() -> Result<String> {
    const NOTE_WIDTH: f64 = 172.0;
    const NOTES_PER_BAR: usize = 3;
    const LABEL_SIZE: f64 = 9.5;
    /// Long labels need two lines to stay inside their column.
    const LABEL_LEADING: f64 = 12.0;
    /// Room above the staff for two lines of label and the stems below them.
    const SPLIT_LABEL_HEIGHT: f64 = 40.0;

    let pairs = split_pairs();
    let width = MARGIN * 2.0 + CLEF_WIDTH + NOTE_WIDTH * NOTES_PER_BAR as f64;
    let label_height = SPLIT_LABEL_HEIGHT + LABEL_LEADING;
    let system_height = label_height + STAFF_HEIGHT + SYSTEM_GAP;
    let height = system_height * pairs.len() as f64;

    let mut canvas = Canvas::new("rji-split");

    for (index, pair) in pairs.iter().enumerate() {
        let staff_y = index as f64 * system_height + label_height;
        let staff_x = MARGIN;
        let staff_width = CLEF_WIDTH + NOTE_WIDTH * NOTES_PER_BAR as f64;

        let fixed = note_frequency(FIXED_C_JUST, pair.chord, pair.offset);
        let recursive = note_frequency(RECURSIVE_JUST, pair.chord, pair.offset);
        let difference = format_signed_cents(cents_between(recursive, fixed));
        let context = chord_context_label(pair.chord);

        let columns = [
            (
                format!("{context} {}", pair.note),
                "fixed +0.000c".to_string(),
            ),
            ("recursive".to_string(), format!("{difference}c")),
            ("together".to_string(), format!("+0.000c / {difference}c")),
        ];

        canvas.staff(staff_x, staff_y, staff_width);
        canvas.treble_clef(staff_x, staff_y);
        canvas.barline(staff_x, staff_y);

        let pitch = Pitch::from_name(pair.abc_pitch)?;
        let note = staff_note(&pitch)?;

        for (column, (upper, lower)) in columns.iter().enumerate() {
            let centre = staff_x + CLEF_WIDTH + NOTE_WIDTH * column as f64 + NOTE_WIDTH / 2.0;

            // Clear of the stems, which reach an octave above a low notehead and
            // so come within a whisker of the staff's own ceiling.
            let lower_baseline = staff_y - 14.0;
            canvas.text(
                upper,
                centre,
                lower_baseline - LABEL_LEADING,
                LABEL_SIZE,
                "notation-annotation",
            );
            canvas.text(
                lower,
                centre,
                lower_baseline,
                LABEL_SIZE,
                "notation-annotation",
            );

            canvas.note(&note, &NOTEHEAD_QUARTER, centre, staff_y);
            canvas.stem(&note, &NOTEHEAD_QUARTER, centre, staff_y);
        }

        canvas.final_barline(staff_x + staff_width, staff_y);
    }

    Ok(canvas.finish(
        width,
        height,
        "Each split pitch written three times: fixed, recursive, and both together",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_figures_come_out_as_svg() {
        for svg in [chord_progression_svg().unwrap(), note_splits_svg().unwrap()] {
            assert!(svg.starts_with("<svg "), "{}", &svg[..40.min(svg.len())]);
            assert!(svg.ends_with("</svg>"));
            assert!(svg.contains("currentColor"), "should follow the page theme");
            assert!(!svg.contains("NaN"));
        }
    }

    /// What the two figures have to say, as opposed to how they are drawn. These
    /// assertions came off the ABC builders that used to feed abcjs; the SVG is
    /// what ships now, so they belong here.
    #[test]
    fn the_figures_are_labelled_the_way_the_post_reads() {
        let progression = chord_progression_svg().unwrap();
        assert!(
            progression.contains(">Ab<"),
            "the flat spelling is the one used"
        );
        assert!(
            !progression.contains("G#/Ab"),
            "the chord name is spelled one way on the staff"
        );

        let splits = note_splits_svg().unwrap();
        assert!(
            splits.contains("E major G#/Ab"),
            "the split is named in full"
        );
        // 25/16 against the five-limit scale's 8/5: the diesis. music21-rs owns
        // this number, and it moved when its tuning tables were corrected.
        assert!(splits.contains("-41.059c"), "the recursive offset is shown");
    }

    /// The outlines go in `<defs>` once and are pointed at thereafter. Built and
    /// then not written into the document, which is a mistake that compiles, the
    /// figure comes out as bare staff lines with no clef and no notes on them.
    #[test]
    fn every_outline_is_written_once_and_referenced() {
        let svg = chord_progression_svg().unwrap();

        assert!(svg.contains("<defs>"), "the outlines are never defined");
        assert_eq!(
            svg.matches(engrave::glyphs::TREBLE_CLEF.path).count(),
            1,
            "the clef outline should appear once, not once per system"
        );
        assert_eq!(
            svg.matches(r##"href="#rji-prog-clef""##).count(),
            3,
            "one clef reference per system"
        );
        for id in ["rji-prog-clef", "rji-prog-whole"] {
            assert!(
                svg.contains(&format!(r#"<path id="{id}""#)),
                "{id} is referenced but never defined"
            );
        }
    }
}
