//! Draws the post's two notation figures as SVG.
//!
//! They used to be ABC strings handed to abcjs in the browser, which cost every
//! reader of the post about a megabyte of JavaScript to draw two pictures that
//! never change. Both come out of fixed data through fixed rules, so they are
//! drawn here instead, once, at build time.
//!
//! The glyph outlines are abcjs's own (see [`crate::glyphs`]), so the notation
//! keeps the shapes it had. Everything is stroked and filled in `currentColor`,
//! which the old rendering did not do — the figures now follow the page into
//! dark mode instead of staying black.

use std::fmt::Write as _;

use music21_rs::Pitch;

use crate::glyphs::{Glyph, FLAT, LINE_GAP, NOTEHEAD_QUARTER, NOTEHEAD_WHOLE, SHARP, STEP};
use crate::{
    cents_between, chord_context_label, format_signed_cents, notated_pitches, notation_chord_label,
    note_frequency, progression, split_pairs, Result, FIXED_C_JUST, RECURSIVE_JUST,
};

/// The staff sits between y = 0 (top line) and y = `STAFF_HEIGHT` (bottom line).
const STAFF_HEIGHT: f64 = LINE_GAP * 4.0;

/// Room left above a staff for the labels that sit over it.
const LABEL_HEIGHT: f64 = 30.0;

/// Room below a staff before the next one starts.
const SYSTEM_GAP: f64 = 18.0;

/// Left margin, and the room the clef takes before the first bar.
const MARGIN: f64 = 6.0;
const CLEF_WIDTH: f64 = 26.0;

const STAFF_STROKE: f64 = 0.9;
const BARLINE_STROKE: f64 = 1.1;
const LEDGER_STROKE: f64 = 1.0;

/// How far a ledger line reaches past the notehead it carries. A ledger the
/// width of the head is a ledger you cannot see, which is what a fixed 7.5 gave
/// against a whole notehead 14.98 wide.
const LEDGER_OVERHANG: f64 = 3.2;

/// A note as the staff sees it.
struct StaffNote {
    /// Steps above the bottom staff line, which in treble clef is E4. A line and
    /// the space above it are one step apart, so the top line is 8.
    position: i32,
    accidental: Option<&'static Glyph>,
}

/// Where a pitch sits on a treble staff, and what accidental it carries.
///
/// `Pitch::name` is the letter and any modifier (`"G#"`, `"A-"`), and the octave
/// comes separately, which together are all a staff needs to place a note.
fn staff_note(pitch: &Pitch) -> Result<StaffNote> {
    let name = pitch.name();
    let mut characters = name.chars();
    let letter = characters
        .next()
        .ok_or_else(|| format!("cannot engrave the empty pitch name {name:?}"))?;

    let degree = match letter {
        'C' => 0,
        'D' => 1,
        'E' => 2,
        'F' => 3,
        'G' => 4,
        'A' => 5,
        'B' => 6,
        other => return Err(format!("cannot engrave pitch step {other:?}").into()),
    };

    let accidental = match characters.next() {
        None => None,
        Some('#') => Some(&SHARP),
        Some('-') => Some(&FLAT),
        Some(other) => return Err(format!("cannot engrave accidental {other:?}").into()),
    };

    // The bottom line of a treble staff is E4, whose degree-plus-octave is 30.
    let octave = pitch.octave().unwrap_or(4);
    Ok(StaffNote {
        position: degree + 7 * octave - 30,
        accidental,
    })
}

/// An SVG document under construction.
struct Canvas {
    body: String,
    /// Glyphs drawn so far, in the order first drawn. Each is written into
    /// `<defs>` once and pointed at thereafter — the G clef alone is 2.8 KB of
    /// path, and it appears on every system.
    used: Vec<&'static Glyph>,
    /// Ids are prefixed per figure, because both figures end up inline in the
    /// same HTML document and ids there have to be unique across all of it.
    prefix: &'static str,
}

impl Canvas {
    fn new(prefix: &'static str) -> Self {
        Self {
            body: String::new(),
            used: Vec::new(),
            prefix,
        }
    }

    fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, width: f64) {
        let _ = write!(
            self.body,
            r#"<line x1="{x1:.2}" y1="{y1:.2}" x2="{x2:.2}" y2="{y2:.2}" stroke-width="{width}"/>"#
        );
    }

    fn glyph(&mut self, glyph: &'static Glyph, x: f64, y: f64) {
        if !self.used.iter().any(|seen| seen.name == glyph.name) {
            self.used.push(glyph);
        }
        let _ = write!(
            self.body,
            r##"<use href="#{}-{}" x="{x:.2}" y="{y:.2}"/>"##,
            self.prefix, glyph.name
        );
    }

    fn text(&mut self, text: &str, x: f64, y: f64, size: f64, class: &str) {
        let _ = write!(
            self.body,
            r#"<text x="{x:.2}" y="{y:.2}" font-size="{size}" class="{class}">{}</text>"#,
            escape(text)
        );
    }

    /// The five staff lines of one system.
    fn staff(&mut self, x: f64, y: f64, width: f64) {
        for index in 0..5 {
            let line_y = y + LINE_GAP * f64::from(index);
            self.line(x, line_y, x + width, line_y, STAFF_STROKE);
        }
    }

    /// A note, its accidental, and any ledger lines it needs to be reachable.
    fn note(&mut self, note: &StaffNote, head: &'static Glyph, x: f64, staff_y: f64) {
        let y = staff_y + STAFF_HEIGHT - f64::from(note.position) * STEP;

        // Ledger lines march outwards from the staff, on lines only, which are
        // the even positions.
        let mut ledger = if note.position < 0 { -2 } else { 10 };
        while (note.position < 0 && ledger >= note.position)
            || (note.position > 8 && ledger <= note.position)
        {
            let ledger_y = staff_y + STAFF_HEIGHT - f64::from(ledger) * STEP;
            let reach = head.width / 2.0 + LEDGER_OVERHANG;
            self.line(x - reach, ledger_y, x + reach, ledger_y, LEDGER_STROKE);
            ledger += if note.position < 0 { -2 } else { 2 };
        }

        if let Some(accidental) = note.accidental {
            self.glyph(accidental, x - head.width / 2.0 - accidental.width - 1.5, y);
        }

        self.glyph(head, x - head.width / 2.0, y);
    }

    /// The barline that ends a piece: a thin line, then a thick one.
    fn final_barline(&mut self, x: f64, staff_y: f64) {
        self.line(
            x - 5.0,
            staff_y,
            x - 5.0,
            staff_y + STAFF_HEIGHT,
            BARLINE_STROKE,
        );
        self.line(x - 1.6, staff_y, x - 1.6, staff_y + STAFF_HEIGHT, 3.2);
    }

    /// A stem, which hangs off whichever side of the notehead keeps it inside
    /// the staff: up on the right below the middle line, down on the left at or
    /// above it. Getting this wrong is both bad engraving and, here, a stem
    /// straight through the label above the note.
    fn stem(&mut self, note: &StaffNote, head: &'static Glyph, x: f64, staff_y: f64) {
        const STEM_LENGTH: f64 = STEP * 7.0;

        let y = staff_y + STAFF_HEIGHT - f64::from(note.position) * STEP;
        let down = note.position >= 4;
        let side = head.width / 2.0 - 0.5;
        let stem_x = if down { x - side } else { x + side };
        let tip = if down {
            y + STEM_LENGTH
        } else {
            y - STEM_LENGTH
        };

        self.line(stem_x, y, stem_x, tip, BARLINE_STROKE);
    }

    fn finish(self, width: f64, height: f64, title: &str) -> String {
        let mut defs = String::from("<defs>");
        for glyph in &self.used {
            let _ = write!(
                defs,
                r#"<path id="{}-{}" d="{}"/>"#,
                self.prefix, glyph.name, glyph.path
            );
        }
        defs.push_str("</defs>");

        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width:.0} {height:.0}\" \
             class=\"engraved-notation\" role=\"img\" aria-label=\"{}\">\
             <title>{}</title>{}\
             <g fill=\"currentColor\" stroke=\"currentColor\" stroke-linecap=\"square\">{}</g>\
             </svg>",
            escape(title),
            escape(title),
            defs,
            self.body
        )
    }
}

/// Text going into an SVG document, with the five characters XML minds escaped.
fn escape(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            other => escaped.push(other),
        }
    }
    escaped
}

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
        // The G clef is drawn about the line it names, the second from the bottom.
        canvas.glyph(
            &crate::glyphs::TREBLE_CLEF,
            staff_x + 4.0,
            staff_y + STAFF_HEIGHT - 2.0 * STEP,
        );
        canvas.line(
            staff_x,
            staff_y,
            staff_x,
            staff_y + STAFF_HEIGHT,
            BARLINE_STROKE,
        );

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
                canvas.line(
                    barline_x,
                    staff_y,
                    barline_x,
                    staff_y + STAFF_HEIGHT,
                    BARLINE_STROKE,
                );
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
        canvas.glyph(
            &crate::glyphs::TREBLE_CLEF,
            staff_x + 4.0,
            staff_y + STAFF_HEIGHT - 2.0 * STEP,
        );
        canvas.line(
            staff_x,
            staff_y,
            staff_x,
            staff_y + STAFF_HEIGHT,
            BARLINE_STROKE,
        );

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
    fn places_notes_against_the_treble_staff() {
        let cases = [("E4", 0), ("F4", 1), ("G4", 2), ("F5", 8), ("C4", -2)];
        for (name, expected) in cases {
            let pitch = Pitch::from_name(name).unwrap();
            assert_eq!(staff_note(&pitch).unwrap().position, expected, "{name}");
        }
    }

    #[test]
    fn reads_accidentals_off_the_pitch_name() {
        let sharp = staff_note(&Pitch::from_name("G#4").unwrap()).unwrap();
        assert!(sharp.accidental.is_some());
        assert_eq!(sharp.position, 2, "a sharp does not move the notehead");

        let natural = staff_note(&Pitch::from_name("G4").unwrap()).unwrap();
        assert!(natural.accidental.is_none());
    }

    #[test]
    fn both_figures_come_out_as_svg() {
        for svg in [chord_progression_svg().unwrap(), note_splits_svg().unwrap()] {
            assert!(svg.starts_with("<svg "), "{}", &svg[..40.min(svg.len())]);
            assert!(svg.ends_with("</svg>"));
            assert!(svg.contains("currentColor"), "should follow the page theme");
            assert!(!svg.contains("NaN"));
        }
    }

    /// The outlines go in `<defs>` once and are pointed at thereafter. Built and
    /// then not written into the document, which is a mistake that compiles, the
    /// figure comes out as bare staff lines with no clef and no notes on them.
    #[test]
    fn every_outline_is_written_once_and_referenced() {
        let svg = chord_progression_svg().unwrap();

        assert!(svg.contains("<defs>"), "the outlines are never defined");
        assert_eq!(
            svg.matches(crate::glyphs::TREBLE_CLEF.path).count(),
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
