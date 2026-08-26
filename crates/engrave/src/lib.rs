//! Staff notation, drawn as SVG.
//!
//! Everything here used to be ABC strings handed to abcjs in the browser. The
//! recursive just intonation post stopped doing that first, because its two
//! figures never change and cost every reader about a megabyte of JavaScript to
//! draw; this crate is that engraver pulled out of the post so the tuning
//! playground can use it too. The playground's case is not a fixed picture —
//! it draws whatever chord is being held down — but it was asking a full ABC
//! parser and layout engine to render one bar of one chord, which it can now
//! do here instead, in the wasm it already loads.
//!
//! The glyph outlines are abcjs's own (see [`glyphs`]), so the notation keeps
//! the shapes it has always had. Everything is stroked and filled in
//! `currentColor`, so a figure follows the page into dark mode.

use std::error::Error;
use std::fmt::Write as _;

use music21_rs::Pitch;

pub mod glyphs;

use glyphs::{FLAT, Glyph, LINE_GAP, NOTEHEAD_WHOLE, SHARP, STEP, TREBLE_CLEF};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// The staff sits between y = 0 (top line) and y = [`STAFF_HEIGHT`].
pub const STAFF_HEIGHT: f64 = LINE_GAP * 4.0;

/// Room left above a staff for the labels that sit over it.
pub const LABEL_HEIGHT: f64 = 30.0;

/// Room below a staff before the next one starts.
pub const SYSTEM_GAP: f64 = 18.0;

/// Left margin, and the room the clef takes before the first bar.
pub const MARGIN: f64 = 6.0;
pub const CLEF_WIDTH: f64 = 26.0;

pub const STAFF_STROKE: f64 = 0.9;
pub const BARLINE_STROKE: f64 = 1.1;
pub const LEDGER_STROKE: f64 = 1.0;

/// How far a ledger line reaches past the notehead it carries. A ledger the
/// width of the head is a ledger you cannot see, which is what a fixed 7.5 gave
/// against a whole notehead 14.98 wide.
pub const LEDGER_OVERHANG: f64 = 3.2;

/// How much room one column of accidentals takes. The widest of them sets it,
/// so stacked accidentals line up in columns instead of stepping raggedly.
const ACCIDENTAL_COLUMN: f64 = SHARP.width + 2.0;

/// How far apart, in staff steps, two accidentals have to be to share a column.
/// A sharp is about three steps tall and is drawn centred on its notehead, so
/// three steps of separation is the point where two stop overlapping.
const ACCIDENTAL_CLEARANCE: i32 = 3;

/// A note as the staff sees it.
pub struct StaffNote {
    /// Steps above the bottom staff line, which in treble clef is E4. A line and
    /// the space above it are one step apart, so the top line is 8.
    pub position: i32,
    pub accidental: Option<&'static Glyph>,
}

/// Where a pitch sits on a treble staff, and what accidental it carries.
///
/// `Pitch::name` is the letter and any modifier (`"G#"`, `"A-"`), and the octave
/// comes separately, which together are all a staff needs to place a note.
pub fn staff_note(pitch: &Pitch) -> Result<StaffNote> {
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
pub struct Canvas {
    body: String,
    /// Glyphs drawn so far, in the order first drawn. Each is written into
    /// `<defs>` once and pointed at thereafter — the G clef alone is 2.8 KB of
    /// path, and it appears on every system.
    used: Vec<&'static Glyph>,
    /// Ids are prefixed per figure, because figures end up inline in the same
    /// HTML document and ids there have to be unique across all of it.
    prefix: &'static str,
}

impl Canvas {
    pub fn new(prefix: &'static str) -> Self {
        Self {
            body: String::new(),
            used: Vec::new(),
            prefix,
        }
    }

    pub fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, width: f64) {
        let _ = write!(
            self.body,
            r#"<line x1="{x1:.2}" y1="{y1:.2}" x2="{x2:.2}" y2="{y2:.2}" stroke-width="{width}"/>"#
        );
    }

    pub fn glyph(&mut self, glyph: &'static Glyph, x: f64, y: f64) {
        if !self.used.iter().any(|seen| seen.name == glyph.name) {
            self.used.push(glyph);
        }
        let _ = write!(
            self.body,
            r##"<use href="#{}-{}" x="{x:.2}" y="{y:.2}"/>"##,
            self.prefix, glyph.name
        );
    }

    pub fn text(&mut self, text: &str, x: f64, y: f64, size: f64, class: &str) {
        let _ = write!(
            self.body,
            r#"<text x="{x:.2}" y="{y:.2}" font-size="{size}" class="{class}">{}</text>"#,
            escape(text)
        );
    }

    /// The five staff lines of one system.
    pub fn staff(&mut self, x: f64, y: f64, width: f64) {
        for index in 0..5 {
            let line_y = y + LINE_GAP * f64::from(index);
            self.line(x, line_y, x + width, line_y, STAFF_STROKE);
        }
    }

    /// The G clef, drawn about the line it names, the second from the bottom.
    pub fn treble_clef(&mut self, staff_x: f64, staff_y: f64) {
        self.glyph(
            &TREBLE_CLEF,
            staff_x + 4.0,
            staff_y + STAFF_HEIGHT - 2.0 * STEP,
        );
    }

    /// A note, its accidental, and any ledger lines it needs to be reachable.
    pub fn note(&mut self, note: &StaffNote, head: &'static Glyph, x: f64, staff_y: f64) {
        let y = self.note_y(note, staff_y);

        self.ledgers(note.position, head, x, x, staff_y);

        if let Some(accidental) = note.accidental {
            self.glyph(accidental, x - head.width / 2.0 - accidental.width - 1.5, y);
        }

        self.glyph(head, x - head.width / 2.0, y);
    }

    /// Where a notehead's baseline falls.
    fn note_y(&self, note: &StaffNote, staff_y: f64) -> f64 {
        staff_y + STAFF_HEIGHT - f64::from(note.position) * STEP
    }

    /// The ledger lines that carry a note back to the staff, marching outwards
    /// on lines only, which are the even positions. `left` and `right` are the
    /// centres of the leftmost and rightmost heads the ledger has to reach.
    fn ledgers(
        &mut self,
        position: i32,
        head: &'static Glyph,
        left: f64,
        right: f64,
        staff_y: f64,
    ) {
        let mut ledger = if position < 0 { -2 } else { 10 };
        while (position < 0 && ledger >= position) || (position > 8 && ledger <= position) {
            let ledger_y = staff_y + STAFF_HEIGHT - f64::from(ledger) * STEP;
            let reach = head.width / 2.0 + LEDGER_OVERHANG;
            self.line(
                left - reach,
                ledger_y,
                right + reach,
                ledger_y,
                LEDGER_STROKE,
            );
            ledger += if position < 0 { -2 } else { 2 };
        }
    }

    /// A chord: several noteheads at one point in time.
    ///
    /// Two notes a second apart cannot share a column, so the upper one steps
    /// to the right of the head below it; accidentals stack leftwards into
    /// columns so that they clear each other too. Both of those are what an
    /// engraver does by hand and what abcjs was being loaded to do.
    ///
    /// `notes` must be sorted by position, lowest first.
    pub fn chord(&mut self, notes: &[StaffNote], head: &'static Glyph, x: f64, staff_y: f64) {
        let offsets = second_offsets(notes);
        let columns = accidental_columns(notes);
        let rightmost = if offsets.iter().any(|&offset| offset) {
            x + head.width
        } else {
            x
        };

        // Ledgers first, so noteheads sit on top of them rather than under.
        for note in notes {
            self.ledgers(note.position, head, x, rightmost, staff_y);
        }

        for (index, note) in notes.iter().enumerate() {
            let y = self.note_y(note, staff_y);
            let head_x = if offsets[index] { x + head.width } else { x };

            if let Some(accidental) = note.accidental {
                let column = f64::from(columns[index] + 1);
                self.glyph(
                    accidental,
                    x - head.width / 2.0 - column * ACCIDENTAL_COLUMN,
                    y,
                );
            }

            self.glyph(head, head_x - head.width / 2.0, y);
        }
    }

    /// The barline that ends a piece: a thin line, then a thick one.
    pub fn final_barline(&mut self, x: f64, staff_y: f64) {
        self.line(
            x - 5.0,
            staff_y,
            x - 5.0,
            staff_y + STAFF_HEIGHT,
            BARLINE_STROKE,
        );
        self.line(x - 1.6, staff_y, x - 1.6, staff_y + STAFF_HEIGHT, 3.2);
    }

    /// A plain barline, which is also what opens a staff.
    pub fn barline(&mut self, x: f64, staff_y: f64) {
        self.line(x, staff_y, x, staff_y + STAFF_HEIGHT, BARLINE_STROKE);
    }

    /// A stem, which hangs off whichever side of the notehead keeps it inside
    /// the staff: up on the right below the middle line, down on the left at or
    /// above it. Getting this wrong is both bad engraving and, here, a stem
    /// straight through the label above the note.
    pub fn stem(&mut self, note: &StaffNote, head: &'static Glyph, x: f64, staff_y: f64) {
        const STEM_LENGTH: f64 = STEP * 7.0;

        let y = self.note_y(note, staff_y);
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

    pub fn finish(self, width: f64, height: f64, title: &str) -> String {
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

/// Which notes of a chord have to step right of the column. A second cannot be
/// written head-on-head, and two seconds in a row alternate rather than walking
/// off to the right: C-D-E puts D out and brings E back.
fn second_offsets(notes: &[StaffNote]) -> Vec<bool> {
    let mut offsets = vec![false; notes.len()];
    for index in 1..notes.len() {
        offsets[index] =
            notes[index].position - notes[index - 1].position <= 1 && !offsets[index - 1];
    }
    offsets
}

/// Which accidental column each note's accidental goes in, counting leftwards
/// from the noteheads. Notes with no accidental get column 0 and never use it.
fn accidental_columns(notes: &[StaffNote]) -> Vec<i32> {
    // The lowest position an accidental has reached in each column so far.
    let mut lowest: Vec<i32> = Vec::new();
    let mut columns = vec![0; notes.len()];

    // Top down, which is the order accidentals are conventionally placed in.
    for index in (0..notes.len()).rev() {
        if notes[index].accidental.is_none() {
            continue;
        }
        let position = notes[index].position;

        let column = lowest
            .iter()
            .position(|bottom| bottom - position >= ACCIDENTAL_CLEARANCE);
        let column = match column {
            Some(column) => column,
            None => {
                lowest.push(position);
                lowest.len() - 1
            }
        };

        lowest[column] = position;
        columns[index] = i32::try_from(column).unwrap_or(0);
    }

    columns
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

/// One bar holding one chord, labelled with whatever the chord is called.
///
/// This is the whole of what the tuning playground ever asked abcjs for. The
/// staff grows to fit its ledger lines, so a chord well off the top or bottom
/// of the staff is drawn in full rather than clipped.
pub fn chord_svg(label: &str, pitches: &[Pitch]) -> Result<String> {
    const LABEL_SIZE: f64 = 11.0;
    /// Room to the right of the heads before the closing barline.
    const TAIL: f64 = 16.0;

    let mut notes: Vec<StaffNote> = pitches.iter().map(staff_note).collect::<Result<_>>()?;
    notes.sort_by_key(|note| note.position);

    let head = &NOTEHEAD_WHOLE;
    let columns = accidental_columns(&notes).into_iter().max().unwrap_or(0);
    let accidental_space = if notes.iter().any(|note| note.accidental.is_some()) {
        f64::from(columns + 1) * ACCIDENTAL_COLUMN
    } else {
        0.0
    };

    // The staff is five lines between positions 0 and 8; anything outside that
    // needs room of its own, plus its ledger lines.
    let highest = notes.last().map_or(8, |note| note.position.max(8));
    let lowest = notes.first().map_or(0, |note| note.position.min(0));
    let above = f64::from(highest - 8) * STEP + LABEL_HEIGHT;
    let below = f64::from(-lowest) * STEP + SYSTEM_GAP;

    let staff_x = MARGIN;
    let staff_y = above;
    let note_x = staff_x + CLEF_WIDTH + accidental_space + head.width / 2.0 + 4.0;
    // An SVG root clips what leaves its viewBox, so a bar narrower than its own
    // label loses the ends of the chord name. Half the font size per character
    // is a generous average for the body font at this size.
    let label_width = label.chars().count() as f64 * LABEL_SIZE * 0.55;
    let staff_width = (note_x + head.width * 1.5 + TAIL - staff_x).max(label_width);
    let width = staff_x + staff_width + MARGIN;
    let height = above + STAFF_HEIGHT + below;

    let mut canvas = Canvas::new("chord");
    canvas.staff(staff_x, staff_y, staff_width);
    canvas.treble_clef(staff_x, staff_y);
    canvas.barline(staff_x, staff_y);

    if !label.is_empty() {
        canvas.text(
            label,
            staff_x + staff_width / 2.0,
            staff_y - 10.0,
            LABEL_SIZE,
            "notation-chord",
        );
    }

    canvas.chord(&notes, head, note_x, staff_y);
    canvas.final_barline(staff_x + staff_width, staff_y);

    let title = if label.is_empty() {
        "An empty bar".to_string()
    } else {
        format!("{label}, written on a treble staff")
    };
    Ok(canvas.finish(width, height, &title))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pitches(names: &[&str]) -> Vec<Pitch> {
        names
            .iter()
            .map(|name| Pitch::from_name(*name).unwrap())
            .collect()
    }

    fn notes(names: &[&str]) -> Vec<StaffNote> {
        let mut notes: Vec<StaffNote> = pitches(names)
            .iter()
            .map(staff_note)
            .collect::<Result<_>>()
            .unwrap();
        notes.sort_by_key(|note| note.position);
        notes
    }

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

    /// A third is written head above head; a second cannot be, and a run of
    /// seconds alternates rather than staircasing away from the stem.
    #[test]
    fn only_seconds_step_out_of_the_column() {
        assert_eq!(
            second_offsets(&notes(&["C4", "E4", "G4"])),
            [false, false, false]
        );
        assert_eq!(second_offsets(&notes(&["C4", "D4"])), [false, true]);
        assert_eq!(
            second_offsets(&notes(&["C4", "D4", "E4"])),
            [false, true, false]
        );
    }

    /// Accidentals share a column when they are far enough apart to clear each
    /// other, and take a new one when they are not.
    #[test]
    fn accidentals_stack_into_columns_only_when_they_collide() {
        assert_eq!(accidental_columns(&notes(&["C#4", "E4", "G#4"])), [0, 0, 0]);
        assert_eq!(accidental_columns(&notes(&["C#4", "D#4"])), [1, 0]);
        assert_eq!(
            accidental_columns(&notes(&["C4", "E-4", "G4"])),
            [0, 0, 0],
            "a note with no accidental never claims a column"
        );
    }

    #[test]
    fn draws_a_chord_as_svg() {
        let svg = chord_svg("C-major triad", &pitches(&["C4", "E4", "G4"])).unwrap();

        assert!(svg.starts_with("<svg "));
        assert!(svg.ends_with("</svg>"));
        assert!(svg.contains("currentColor"), "should follow the page theme");
        assert!(svg.contains(">C-major triad<"), "the label is written");
        assert!(!svg.contains("NaN"));
        assert_eq!(
            svg.matches(r##"href="#chord-whole""##).count(),
            3,
            "one notehead per note"
        );
    }

    /// A chord name can easily be longer than the one bar carrying it, and an
    /// SVG root clips rather than overflows, so the bar has to make room.
    #[test]
    fn the_bar_widens_to_hold_a_long_label() {
        let width = |svg: &str| {
            svg.split("viewBox=\"0 0 ")
                .nth(1)
                .and_then(|rest| rest.split(' ').next().map(str::to_string))
                .and_then(|width| width.parse::<f64>().ok())
                .unwrap()
        };

        let short = chord_svg("C", &pitches(&["C4", "E4", "G4"])).unwrap();
        let long = chord_svg(
            "C-dominant seventh chord in second inversion",
            &pitches(&["C4", "E4", "G4"]),
        )
        .unwrap();

        assert!(width(&long) > width(&short));
    }

    /// Everything the bar draws has to land inside the bar, because an SVG root
    /// clips and a notehead outside the viewBox is a notehead that is simply
    /// not there. Checked over the shapes a chord can actually take -- clusters,
    /// stacked accidentals, notes far above and below the staff -- since each
    /// of those is a different way for the drawing to reach past its own edge.
    #[test]
    fn nothing_is_drawn_outside_the_picture() {
        let cases: [&[&str]; 8] = [
            &["C4", "E4", "G4"],
            &["C4", "D4", "E4"],
            &["C#4", "D#4"],
            &["C-4", "E-4", "G-4", "B-4"],
            &["C3", "E3", "G3", "B-3"],
            &["G5", "B5", "D6"],
            &["A0", "C8"],
            &["C2"],
        ];

        for notes in cases {
            let svg = chord_svg("a chord", &pitches(notes)).unwrap();
            let (width, height) = view_box(&svg);

            for (x, y) in drawn_points(&svg) {
                assert!(
                    (0.0..=width).contains(&x) && (0.0..=height).contains(&y),
                    "{notes:?} draws at ({x}, {y}), outside {width}x{height}"
                );
            }
        }
    }

    fn view_box(svg: &str) -> (f64, f64) {
        let box_ = svg
            .split("viewBox=\"0 0 ")
            .nth(1)
            .and_then(|rest| rest.split('"').next())
            .unwrap();
        let mut parts = box_.split(' ').map(|n| n.parse::<f64>().unwrap());
        (parts.next().unwrap(), parts.next().unwrap())
    }

    /// Every point the body places something at. Glyphs are drawn about their
    /// own origin, so a glyph's own extent is not counted here -- what this
    /// catches is a placement that has left the picture altogether.
    fn drawn_points(svg: &str) -> Vec<(f64, f64)> {
        let number = |fragment: &str, key: &str| {
            fragment
                .split(&format!("{key}=\""))
                .nth(1)
                .and_then(|rest| rest.split('"').next())
                .and_then(|value| value.parse::<f64>().ok())
        };

        let mut points = Vec::new();
        for element in svg.split('<').skip(1) {
            if element.starts_with("use ") || element.starts_with("text ") {
                if let (Some(x), Some(y)) = (number(element, "x"), number(element, "y")) {
                    points.push((x, y));
                }
            } else if element.starts_with("line ") {
                for (x, y) in [("x1", "y1"), ("x2", "y2")] {
                    if let (Some(x), Some(y)) = (number(element, x), number(element, y)) {
                        points.push((x, y));
                    }
                }
            }
        }

        assert!(!points.is_empty(), "found nothing drawn at all");
        points
    }

    #[test]
    fn draws_an_empty_bar_when_nothing_is_held() {
        let svg = chord_svg("", &[]).unwrap();

        assert!(svg.starts_with("<svg "));
        assert!(!svg.contains("<text"), "there is nothing to label");
        assert!(svg.contains("chord-clef"), "the staff is still a staff");
    }

    /// The viewBox has to grow with the ledger lines, or a chord below the
    /// staff is drawn outside the picture and simply does not appear.
    #[test]
    fn the_picture_grows_to_fit_notes_off_the_staff() {
        let inside = chord_svg("", &pitches(&["E4"])).unwrap();
        let below = chord_svg("", &pitches(&["C3"])).unwrap();

        let height = |svg: &str| {
            svg.split("viewBox=\"0 0 ")
                .nth(1)
                .and_then(|rest| rest.split('"').next())
                .and_then(|box_| box_.split(' ').nth(1).map(str::to_string))
                .and_then(|height| height.parse::<f64>().ok())
                .unwrap()
        };

        assert!(
            height(&below) > height(&inside),
            "a note two octaves down needs a taller picture"
        );
        assert!(below.contains("chord-whole"), "the note is drawn");
    }
}
