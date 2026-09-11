//! Recursive tuning: one scale places the roots, another tunes what sounds
//! above each of them.
//!
//! A pair is a *global* scale and a *local* one. Row `r` of a pair is the
//! local scale started on the global scale's degree `r`, so the note in row
//! `r`, column `k` sounds at
//!
//! ```text
//! root_hz · global[r] · local[k]
//! ```
//!
//! Recursive just intonation is five-limit in both places. Any scale the
//! playground can realise can stand in either, and the two need not agree on
//! how many degrees they have or where they repeat: there is a row for every
//! degree of the global scale, and every row is as long as the local scale.

use serde::Serialize;

use crate::scale::{self, ADAPTIVE_ID, Scale};
use music21_rs::tuningsystem::TWELVE_TONE_NAMES;

/// Rows and columns past this many are left out of the matrix the page draws,
/// though every one of them can still be played. The Scala archive has scales
/// of several hundred notes, and a table of their squares is no use to anyone.
pub const MATRIX_LIMIT: usize = 72;

/// A major triad as equal-tempered cents over its root. Each scale plays the
/// degrees nearest these.
pub const MAJOR: &[f64] = &[0.0, 400.0, 700.0];

/// A dominant seventh chord, the same way.
pub const DOMINANT: &[f64] = &[0.0, 400.0, 700.0, 1000.0];

/// The progression the recursive JI post plays: a name, a root in
/// equal-tempered semitones above the global root, and a shape.
const PROGRESSION: [(&str, i64, &[f64]); 12] = [
    ("C", 0, MAJOR),
    ("E", 4, MAJOR),
    ("Ab", 8, MAJOR),
    ("C", 0, MAJOR),
    ("F", 5, MAJOR),
    ("A", 9, MAJOR),
    ("D", 2, MAJOR),
    ("G7", 7, DOMINANT),
    ("C", 0, MAJOR),
    ("E", 4, MAJOR),
    ("F", 5, MAJOR),
    ("C", 0, MAJOR),
];

/// Two scales, the first placing roots and the second tuning above them.
#[derive(Clone, Debug)]
pub struct Pair {
    pub global: Scale,
    pub local: Scale,
}

/// What the page says about either scale of a pair.
#[derive(Serialize, Clone, Debug)]
pub struct Side {
    pub id: String,
    pub name: String,
    pub family: String,
    pub description: String,
    pub count: usize,
    pub period_cents: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct Column {
    pub step: i64,
    pub ratio_label: String,
    pub cents: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct Cell {
    pub step: i64,
    pub frequency: f64,
    /// Above the global root.
    pub cents: f64,
    /// Against the pitch the global scale plays for the same note on its own.
    pub from_fixed: f64,
    /// Which of the twelve-tone names the note takes its colour from.
    pub note: usize,
    pub name: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct Row {
    pub root: i64,
    pub label: String,
    pub ratio_label: String,
    pub frequency: f64,
    pub cells: Vec<Cell>,
}

/// The pair written out as frequencies, a row per root.
#[derive(Serialize, Clone, Debug)]
pub struct Matrix {
    pub global: Side,
    pub local: Side,
    pub root_hz: f64,
    /// Both scales have the same number of degrees to the same period, so a
    /// column is the same interval, and a cell the same note name, in every row.
    pub aligned: bool,
    pub twelve_tone: bool,
    /// The farthest any drawn note sits from the global scale's own pitch for
    /// it, in cents. Nought means recursing changed nothing.
    pub largest_shift: f64,
    /// Whether rows or columns past [`MATRIX_LIMIT`] were left out.
    pub truncated: bool,
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
}

/// One note of a chord in the progression, as each rendering plays it.
#[derive(Serialize, Clone, Debug)]
pub struct Voice {
    pub step: i64,
    pub name: String,
    pub recursive: f64,
    pub fixed: f64,
    pub equal: f64,
    pub from_fixed: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct Chord {
    pub name: String,
    pub root: i64,
    pub voices: Vec<Voice>,
}

impl Pair {
    /// The pair two share ids name, both over the same root frequency.
    pub fn realize(global_id: &str, local_id: &str, root_hz: f64) -> Result<Self, String> {
        let side = |id: &str| {
            if id == ADAPTIVE_ID {
                return Err(
                    "recursive just intonation is already a pair: pick five-limit for both scales"
                        .to_string(),
                );
            }
            scale::realize(id, root_hz)
        };
        Ok(Self {
            global: side(global_id)?,
            local: side(local_id)?,
        })
    }

    pub fn aligned(&self) -> bool {
        self.global.count == self.local.count
            && (self.global.period_ratio - self.local.period_ratio).abs() < 1e-9
    }

    fn twelve_tone(&self) -> bool {
        self.global.twelve_tone && self.local.twelve_tone
    }

    /// The note `step` degrees of the local scale above the global scale's
    /// degree `root`, in hertz. Either may run past a period, up or down.
    pub fn frequency(&self, root: i64, step: i64) -> f64 {
        self.global.root_hz * ratio_at(&self.global, root) * ratio_at(&self.local, step)
    }

    /// What the global scale plays for that note on its own: its degree
    /// `root + step` when the scales are aligned, and otherwise its degree
    /// nearest the note.
    pub fn fixed_frequency(&self, root: i64, step: i64) -> f64 {
        let global = &self.global;
        if self.aligned() {
            return global.root_hz * ratio_at(global, root + step);
        }
        let cents = cents_of(ratio_at(global, root) * ratio_at(&self.local, step));
        global.root_hz * ratio_at(global, nearest_step(global, cents))
    }

    fn from_fixed(&self, root: i64, step: i64) -> f64 {
        cents_of(self.frequency(root, step) / self.fixed_frequency(root, step))
    }

    /// The twelve-tone name a note is coloured by: its own when both scales are
    /// twelve-tone, and otherwise that of the equal-tempered semitone nearest it.
    pub fn note(&self, root: i64, step: i64) -> usize {
        if self.twelve_tone() {
            return (root + step).rem_euclid(12) as usize;
        }
        let cents = cents_of(ratio_at(&self.global, root) * ratio_at(&self.local, step));
        ((cents / 100.0).round() as i64).rem_euclid(12) as usize
    }

    pub fn note_name(&self, root: i64, step: i64) -> String {
        let name = TWELVE_TONE_NAMES[self.note(root, step)];
        if self.twelve_tone() {
            name.to_string()
        } else {
            format!("≈{name}")
        }
    }

    /// The local steps nearest a chord shape, lowest first and each once, with
    /// the equal-tempered cents each one stands for.
    pub fn chord(&self, shape: &[f64]) -> Vec<(i64, f64)> {
        let mut steps: Vec<(i64, f64)> = shape
            .iter()
            .map(|cents| (nearest_step(&self.local, *cents), *cents))
            .collect();
        steps.sort_by_key(|(step, _)| *step);
        steps.dedup_by_key(|(step, _)| *step);
        steps
    }

    pub fn matrix(&self) -> Matrix {
        let rows_drawn = self.global.count.min(MATRIX_LIMIT) as i64;
        let columns_drawn = self.local.count.min(MATRIX_LIMIT) as i64;

        let columns = (0..=columns_drawn)
            .map(|step| {
                let degree = &self.local.degrees[step as usize];
                Column {
                    step,
                    ratio_label: degree.ratio_label.clone(),
                    cents: degree.cents,
                }
            })
            .collect();

        let mut largest_shift = 0.0_f64;
        let rows = (0..rows_drawn)
            .map(|root| {
                let cells = (0..=columns_drawn)
                    .map(|step| {
                        let frequency = self.frequency(root, step);
                        let from_fixed = self.from_fixed(root, step);
                        largest_shift = largest_shift.max(from_fixed.abs());
                        Cell {
                            step,
                            frequency,
                            cents: cents_of(frequency / self.global.root_hz),
                            from_fixed,
                            note: self.note(root, step),
                            name: self.note_name(root, step),
                        }
                    })
                    .collect();
                Row {
                    root,
                    label: if self.global.twelve_tone {
                        TWELVE_TONE_NAMES[root as usize % 12].to_string()
                    } else {
                        (root + 1).to_string()
                    },
                    ratio_label: self.global.degrees[root as usize].ratio_label.clone(),
                    frequency: self.frequency(root, 0),
                    cells,
                }
            })
            .collect();

        Matrix {
            global: side(&self.global),
            local: side(&self.local),
            root_hz: self.global.root_hz,
            aligned: self.aligned(),
            twelve_tone: self.twelve_tone(),
            largest_shift,
            truncated: self.global.count > MATRIX_LIMIT || self.local.count > MATRIX_LIMIT,
            columns,
            rows,
        }
    }

    /// The recursive JI post's progression, with every root and every chord
    /// taken as the degrees of the pair nearest the equal-tempered ones.
    pub fn progression(&self) -> Vec<Chord> {
        let root_hz = self.global.root_hz;
        PROGRESSION
            .iter()
            .map(|(name, semitones, shape)| {
                let root_cents = *semitones as f64 * 100.0;
                let root = nearest_step(&self.global, root_cents);
                let voices = self
                    .chord(shape)
                    .into_iter()
                    .map(|(step, cents)| {
                        let recursive = self.frequency(root, step);
                        let fixed = self.fixed_frequency(root, step);
                        Voice {
                            step,
                            name: self.note_name(root, step),
                            recursive,
                            fixed,
                            equal: root_hz * 2f64.powf((root_cents + cents) / 1200.0),
                            from_fixed: cents_of(recursive / fixed),
                        }
                    })
                    .collect();
                Chord {
                    name: name.to_string(),
                    root,
                    voices,
                }
            })
            .collect()
    }
}

/// The root a key picks: the digit row, `1` for the global scale's first degree.
pub fn root_key(code: &str) -> Option<i64> {
    crate::KEY_ROWS[3]
        .iter()
        .position(|key| *key == code)
        .map(|index| index as i64)
}

/// The local step a key plays: the three letter rows, bottom to top, one
/// degree after another.
pub fn note_key(code: &str) -> Option<i64> {
    crate::KEY_ROWS[..3]
        .iter()
        .flat_map(|row| row.iter())
        .position(|key| *key == code)
        .map(|index| index as i64)
}

fn side(scale: &Scale) -> Side {
    Side {
        id: scale.id.clone(),
        name: scale.name.clone(),
        family: scale.family.clone(),
        description: scale.description.clone(),
        count: scale.count,
        period_cents: scale.period_cents,
    }
}

fn ratio_at(scale: &Scale, step: i64) -> f64 {
    let (period, degree) = scale.split(step);
    scale.period_ratio.powi(period as i32) * scale.degrees[degree].ratio
}

fn cents_of(ratio: f64) -> f64 {
    1200.0 * ratio.log2()
}

/// The step of a scale nearest a pitch given in cents above its root, in
/// whichever period the pitch falls.
fn nearest_step(scale: &Scale, cents: f64) -> i64 {
    let periods = (cents / scale.period_cents).floor();
    let within = cents - periods * scale.period_cents;
    let degree = scale
        .degrees
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            (a.cents - within)
                .abs()
                .total_cmp(&(b.cents - within).abs())
        })
        .map_or(0, |(degree, _)| degree);
    periods as i64 * scale.count as i64 + degree as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use music21_rs::tuningsystem::adaptive::RECURSIVE_JI;
    use music21_rs::tuningsystem::{C4, CN1};

    fn pair(global: &str, local: &str) -> Pair {
        Pair::realize(global, local, C4).unwrap()
    }

    /// Five-limit in both places is recursive just intonation, and has to agree
    /// with music21-rs's own.
    #[test]
    fn five_limit_twice_is_recursive_just_intonation() {
        let just = pair("FiveLimit", "FiveLimit");
        for root in 0..12 {
            for step in 0..=12 {
                let expected = RECURSIVE_JI.frequency_at(root as f64, step as f64, None) / CN1 * C4;
                assert!(
                    (just.frequency(root, step) - expected).abs() < 1e-9,
                    "row {root}, column {step}"
                );
            }
        }
        assert!((just.from_fixed(4, 4) + 41.059).abs() < 1e-3);
        assert!((just.from_fixed(2, 7) - 21.506).abs() < 1e-3);
    }

    /// A step of an equal division over a step is the step of their sum, so
    /// recursing one moves nothing. Any other table moves.
    #[test]
    fn an_equal_division_is_its_own_recursion() {
        for id in ["EqualTemperament", "equal:19:2:1", "equal:13:3:1"] {
            let matrix = pair(id, id).matrix();
            assert!(matrix.aligned, "{id}");
            assert!(
                matrix.largest_shift < 1e-6,
                "{id}: {}",
                matrix.largest_shift
            );
        }
        assert!(pair("FiveLimit", "FiveLimit").matrix().largest_shift > 40.0);
    }

    /// The recursive JI post's hybrid: equal-tempered roots, just chords.
    #[test]
    fn roots_and_chords_can_come_from_different_scales() {
        let hybrid = pair("EqualTemperament", "FiveLimit");
        let e = hybrid.frequency(4, 0);
        assert!((e - C4 * 2f64.powf(4.0 / 12.0)).abs() < 1e-9);
        assert!((hybrid.frequency(4, 4) / e - 1.25).abs() < 1e-12);
    }

    /// Scales of different sizes still make a matrix, a row per global degree
    /// and a column per local one, and each repeats at its own period.
    #[test]
    fn the_scales_need_not_be_the_same_size() {
        let diatonic = pair("FiveLimit", "PtolemyIntenseDiatonic");
        let matrix = diatonic.matrix();
        assert!(!matrix.aligned);
        assert_eq!(matrix.rows.len(), 12);
        assert_eq!(matrix.columns.len(), 8);
        assert!(matrix.rows.iter().all(|row| row.cells.len() == 8));
        assert!((diatonic.frequency(4, 7) - 2.0 * diatonic.frequency(4, 0)).abs() < 1e-9);

        let tritave = pair("FiveLimit", "equal:13:3:1");
        assert!((tritave.frequency(7, 13) - 3.0 * tritave.frequency(7, 0)).abs() < 1e-9);
        assert!(
            tritave.frequency(7, -13) < tritave.frequency(7, 0),
            "a step below the root is below it"
        );
    }

    /// A chord is the degrees nearest the equal-tempered shape, so the harmonic
    /// series answers a dominant seventh with its own 7/4.
    #[test]
    fn chords_are_the_nearest_degrees() {
        let just = pair("FiveLimit", "FiveLimit");
        let steps: Vec<i64> = just.chord(DOMINANT).iter().map(|(step, _)| *step).collect();
        assert_eq!(steps, [0, 4, 7, 10]);

        let harmonic = pair("CarlosHarmonic", "CarlosHarmonic");
        let seventh = harmonic.chord(DOMINANT)[3].0;
        assert!((ratio_at(&harmonic.local, seventh) - 7.0 / 4.0).abs() < 1e-12);

        let progression = just.progression();
        assert_eq!(progression.len(), 12);
        let e_major = &progression[1];
        assert_eq!(e_major.name, "E");
        assert_eq!(e_major.voices[1].name, "G#/Ab");
        assert!((e_major.voices[1].from_fixed + 41.059).abs() < 1e-3);
        assert!((e_major.voices[1].equal / C4 - 2f64.powf(8.0 / 12.0)).abs() < 1e-12);
    }

    #[test]
    fn a_huge_scale_is_drawn_in_part() {
        let huge = scale::scala_search("", 0)
            .into_iter()
            .find(|entry| entry.count > MATRIX_LIMIT)
            .expect("the archive has a scale of more than 72 notes");
        let matrix = pair(&huge.id, "FiveLimit").matrix();
        assert!(matrix.truncated);
        assert_eq!(matrix.rows.len(), MATRIX_LIMIT);
    }

    #[test]
    fn the_adaptive_entry_is_not_a_scale_to_recurse() {
        assert!(Pair::realize(ADAPTIVE_ID, "FiveLimit", C4).is_err());
    }

    #[test]
    fn digits_pick_roots_and_letters_play_notes() {
        assert_eq!(root_key("Digit1"), Some(0));
        assert_eq!(root_key("Equal"), Some(11));
        assert_eq!(note_key("KeyZ"), Some(0));
        assert_eq!(note_key("KeyA"), Some(10));
        assert_eq!(note_key("KeyQ"), Some(21));
        assert_eq!(note_key("Digit1"), None);
        assert_eq!(root_key("KeyQ"), None);
    }
}
