//! The tuning playground's wasm: which scale is under the keys, what each key
//! sounds and is called, the chord being held engraved as a bar of notation,
//! and the reading of MIDI files.
//!
//! The scale itself is [`scale::Scale`], and everything here that answers a
//! question about a step asks it. State is three globals -- the scale, the
//! keyboard layout, and the name of the last chord engraved -- because the
//! page has one of each.

use std::sync::LazyLock;
use std::sync::Mutex;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub mod midi;
pub mod scale;

use music21_rs::Pitch;
use music21_rs::chord::Chord;
use scale::Scale;

static CURRENT: LazyLock<Mutex<Scale>> = LazyLock::new(|| Mutex::new(Scale::default_scale()));
static KEYMAP: Mutex<KeyMap> = Mutex::new(KeyMap::Us);
static CHORD_NAME: Mutex<String> = Mutex::new(String::new());

fn current() -> std::sync::MutexGuard<'static, Scale> {
    CURRENT.lock().expect("couldn't lock the scale")
}

/// How the computer keyboard is laid over the scale.
///
/// The piano layouts come from the `keymapping` crate and name physical keys
/// by where a US layout prints a letter, which is what `KeyboardEvent.code`
/// reports whatever the keyboard prints; the QWERTZ and AZERTY variants are for
/// whoever would rather press the key that has the letter on it. `Rows` walks
/// the four rows of the keyboard degree by degree, and `Periods` starts each
/// row a period higher, which is what makes sense of a scale that does not
/// have twelve notes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyMap {
    Us,
    UsExtended,
    Qwertz,
    German,
    Azerty,
    Rows,
    Periods,
}

impl KeyMap {
    pub fn parse(keymap: &str) -> Option<Self> {
        match keymap.to_lowercase().as_str() {
            "us" | "qwerty" | "piano" => Some(Self::Us),
            "us-extended" | "extended" | "qwerty-extended" => Some(Self::UsExtended),
            "qwertz" => Some(Self::Qwertz),
            "de" | "german" => Some(Self::German),
            "azerty" | "fr" | "french" => Some(Self::Azerty),
            "rows" | "linear" | "chromatic" => Some(Self::Rows),
            "periods" | "octaves" => Some(Self::Periods),
            _ => None,
        }
    }

    /// The step a physical key plays in a scale of `count` degrees, or `None`
    /// for a key the layout does not use.
    pub fn step(self, code: &str, count: usize) -> Option<i64> {
        use keymapping::{AZERTY_KEYMAP, GERMAN_KEYMAP, QWERTZ_KEYMAP, US_EXTENDED_KEYMAP, US_KEYMAP};
        let count = count.max(1) as i64;
        let root = scale::ROOT_PERIOD * count;
        let piano = |map: &std::collections::HashMap<&'static str, i32>| {
            // The crate's maps start two octaves below middle C; the keyboard
            // is played one octave lower than the root and up, which is where
            // a chord over the root falls under two hands.
            map.get(code).map(|note| i64::from(*note) + 24)
        };
        match self {
            Self::Us => piano(&US_KEYMAP),
            Self::UsExtended => piano(&US_EXTENDED_KEYMAP),
            Self::Qwertz => piano(&QWERTZ_KEYMAP),
            Self::German => piano(&GERMAN_KEYMAP),
            Self::Azerty => piano(&AZERTY_KEYMAP),
            Self::Rows => KEY_ROWS
                .iter()
                .flat_map(|row| row.iter())
                .position(|key| *key == code)
                .map(|index| root + index as i64),
            Self::Periods => KEY_ROWS.iter().enumerate().find_map(|(row, keys)| {
                let column = keys.iter().position(|key| *key == code)? as i64;
                (column <= count).then(|| root + row as i64 * count + column)
            }),
        }
    }
}

/// The four rows of a keyboard, bottom to top, by physical position.
pub const KEY_ROWS: [&[&str]; 4] = [
    &[
        "KeyZ", "KeyX", "KeyC", "KeyV", "KeyB", "KeyN", "KeyM", "Comma", "Period", "Slash",
    ],
    &[
        "KeyA",
        "KeyS",
        "KeyD",
        "KeyF",
        "KeyG",
        "KeyH",
        "KeyJ",
        "KeyK",
        "KeyL",
        "Semicolon",
        "Quote",
    ],
    &[
        "KeyQ",
        "KeyW",
        "KeyE",
        "KeyR",
        "KeyT",
        "KeyY",
        "KeyU",
        "KeyI",
        "KeyO",
        "KeyP",
        "BracketLeft",
        "BracketRight",
    ],
    &[
        "Digit1", "Digit2", "Digit3", "Digit4", "Digit5", "Digit6", "Digit7", "Digit8", "Digit9",
        "Digit0", "Minus", "Equal",
    ],
];

fn to_json<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value).expect("the playground's types serialise")
}

// ---------------------------------------------------------------------------
// Scale

/// Everything the picker lists, as JSON.
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn library_json() -> String {
    to_json(&scale::library())
}

/// Scala scales matching `query`, as JSON; every one of them for an empty query.
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn scala_search_json(query: &str, limit: usize) -> String {
    to_json(&scale::scala_search(query, limit))
}

/// Puts the scale a share id names under the keys, and describes it as JSON.
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn select_scale(id: &str, root_hz: f64) -> Result<String, JsError> {
    let scale = scale::realize(id, root_hz).map_err(|message| JsError::new(&message))?;
    let json = to_json(&scale);
    *current() = scale;
    Ok(json)
}

/// Puts a scale from the text of a `.scl` file under the keys.
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn select_scl(name: &str, contents: &str, root_hz: f64) -> Result<String, JsError> {
    let scale =
        scale::realize_scl(name, contents, root_hz).map_err(|message| JsError::new(&message))?;
    let json = to_json(&scale);
    *current() = scale;
    Ok(json)
}

/// The scale under the keys, as JSON.
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn scale_json() -> String {
    to_json(&*current())
}

/// The keys the page should draw for the scale under them, as JSON.
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn keyboard_json() -> String {
    to_json(&current().keyboard())
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn scale_count() -> usize {
    current().count
}

/// The pitch of a step in hertz, given the lowest step held, or a negative
/// `context` when nothing else is.
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn step_frequency(step: i32, context: i32) -> f64 {
    let scale = current();
    if context < 0 {
        scale.frequency(i64::from(step))
    } else {
        scale.frequency_from(i64::from(context), i64::from(step))
    }
}

/// A step's name as music21 spells it, which is what the staff and the chord
/// namer read.
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn step_name(step: i32) -> String {
    current().note_name(i64::from(step))
}

// ---------------------------------------------------------------------------
// Keyboard layout

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn from_keymap(key: &str) -> i32 {
    let count = current().count;
    KEYMAP
        .lock()
        .expect("couldn't lock")
        .step(key, count)
        .map_or(-1, |step| step as i32)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn set_keymap(keymap: &str) {
    if let Some(keymap) = KeyMap::parse(keymap) {
        *KEYMAP.lock().expect("couldn't lock") = keymap;
    }
}

// ---------------------------------------------------------------------------
// Chords and the staff

pub fn chordname_core(input: &str) -> Result<String, String> {
    Chord::new(input)
        .map(|chord| chord.pitched_common_name())
        .map_err(|err| err.to_string())
}

pub fn chord_details_core(input: &str) -> Result<String, String> {
    let chord = Chord::new(input).map_err(|err| err.to_string())?;
    let chord_name = chord.pitched_common_name();
    let chord_symbol = chord
        .chord_symbol()
        .unwrap_or_else(|| "unknown".to_string());
    let pitch_classes = chord
        .pitch_classes()
        .into_iter()
        .map(|pitch_class| pitch_class.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let forte_class = chord.forte_class().unwrap_or_else(|| "unknown".to_string());

    Ok(format!(
        "Name: {chord_name} | Symbol: {chord_symbol} | Pitch classes: {pitch_classes} | Forte class: {forte_class}"
    ))
}

fn tuning_marked_hash_core(keys: &str) -> String {
    let mut keys = keys
        .split(',')
        .filter_map(|key| key.trim().parse::<i32>().ok())
        .collect::<Vec<_>>();

    keys.sort_unstable();
    keys.dedup();
    keys.into_iter()
        .map(|key| key.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn parse_octave(note: &str) -> Option<i32> {
    let note = note.trim();
    if let Some(index) = note.rfind('N') {
        let suffix = &note[index + 1..];
        if !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit()) {
            return suffix.parse().ok();
        }
    }

    let mut start = note.len();
    for (index, ch) in note.char_indices().rev() {
        if ch.is_ascii_digit() {
            start = index;
        } else {
            break;
        }
    }

    (start < note.len())
        .then(|| note[start..].parse().ok())
        .flatten()
}

/// One of the playground's note spellings as a pitch: a letter, any
/// accidentals, and an octave.
///
/// The name this builds is music21's, where a flat is `-`, because music21 is
/// what names the chord and the engraver reads the staff position back off the
/// same pitch.
fn parse_note_token(note: &str) -> Result<Pitch, String> {
    let note = note.trim();
    let mut chars = note.chars().peekable();
    let letter = chars
        .next()
        .ok_or_else(|| "Expected a note name".to_string())?
        .to_ascii_uppercase();

    if !('A'..='G').contains(&letter) {
        return Err(format!("Invalid note: {note}"));
    }

    let mut accidentals = String::new();
    while let Some(ch) = chars.peek() {
        match ch {
            '#' => {
                accidentals.push('#');
                chars.next();
            }
            'b' | 'B' | '-' => {
                accidentals.push('-');
                chars.next();
            }
            _ => break,
        }
    }

    let octave = parse_octave(note).unwrap_or(4);
    let name = format!("{letter}{accidentals}{octave}");
    Pitch::from_name(&name).map_err(|err| format!("Invalid note {note}: {err}"))
}

fn set_chord_name(chord: &str) {
    let mut chord_name = CHORD_NAME.lock().expect("couldn't lock");
    chord_name.clear();
    chord_name.push_str(chord);
}

/// The chord being held down, engraved as one labelled bar of SVG, in the wasm
/// the page has already loaded.
pub fn convert_notes_core(input: Vec<String>) -> String {
    let mut pitches = Vec::new();
    let mut names = Vec::new();

    for token in input {
        match parse_note_token(&token) {
            Ok(pitch) => {
                names.push(pitch.name_with_octave());
                pitches.push(pitch);
            }
            Err(err) => return failed_bar(&err),
        }
    }

    let chord = Chord::new(names)
        .map(|chord| chord.pitched_common_name())
        .unwrap_or_else(|_| "Unknown chord".to_string());
    set_chord_name(&chord);

    match engrave::chord_svg(&chord, &pitches) {
        Ok(svg) => svg,
        Err(err) => failed_bar(&err.to_string()),
    }
}

/// An empty staff, for when nothing is being played.
pub fn empty_staff_core() -> String {
    engrave::chord_svg("", &[]).unwrap_or_default()
}

/// An empty bar carrying the reason there is nothing on it. The message goes to
/// the chord name too, which is where the page's log reads it from.
fn failed_bar(message: &str) -> String {
    set_chord_name(message);
    engrave::chord_svg(message, &[]).unwrap_or_default()
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn get_chord_name() -> String {
    CHORD_NAME.lock().expect("couldn't lock").clone()
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn chordname(notes: &str) -> String {
    chordname_core(notes).unwrap_or_else(|err| err)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn chord_details(notes: &str) -> String {
    chord_details_core(notes).unwrap_or_else(|err| err)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn convert_notes(notes: Vec<String>) -> String {
    convert_notes_core(notes)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn empty_staff() -> String {
    empty_staff_core()
}

/// The notes of a MIDI file, flat: key, velocity, start and end for each, four
/// numbers at a time.
///
/// Flat because that crosses into JavaScript as one `Float64Array` sharing the
/// wasm's own memory, where a list of note objects would be an allocation and a
/// property bag each.
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn parse_midi(bytes: &[u8]) -> Result<Vec<f64>, JsError> {
    let notes = midi::parse(bytes).map_err(|err| JsError::new(&err))?;
    let mut flat = Vec::with_capacity(notes.len() * 4);

    for note in notes {
        flat.push(f64::from(note.key));
        flat.push(f64::from(note.velocity));
        flat.push(note.start);
        flat.push(note.end);
    }

    Ok(flat)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn tuning_marked_hash(keys: &str) -> String {
    tuning_marked_hash_core(keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_common_chords_from_generated_lookup() {
        assert_eq!(chordname_core("C E G").unwrap(), "C-major triad");
        assert_eq!(
            chordname_core("C Eb G Bb").unwrap(),
            "C-minor seventh chord"
        );
    }

    #[test]
    fn chord_details_core_reports_primary_name_and_pitch_classes() {
        let details = chord_details_core("C E G").unwrap();
        assert!(details.contains("Name: C-major triad"));
        assert!(details.contains("Pitch classes: 0, 4, 7"));
        assert!(details.contains("Forte class:"));
    }

    #[test]
    fn canonicalizes_marked_key_hashes() {
        assert_eq!(tuning_marked_hash_core("5,3,5,-1"), "-1,3,5");
        assert_eq!(tuning_marked_hash_core(""), "");
    }

    #[test]
    fn the_staff_reads_the_names_the_scale_writes() {
        let scale = Scale::default_scale();
        let names: Vec<String> = [60, 64, 67].iter().map(|s| scale.note_name(*s)).collect();
        assert_eq!(names, ["CN4", "EN4", "GN4"]);
        let svg = convert_notes_core(names);
        assert!(svg.contains("<svg"));
        assert_eq!(get_chord_name_core(), "C-major triad");
    }

    fn get_chord_name_core() -> String {
        CHORD_NAME.lock().unwrap().clone()
    }

    #[test]
    fn keymaps_cover_the_root() {
        assert_eq!(KeyMap::Us.step("KeyZ", 12), Some(48));
        assert_eq!(KeyMap::Us.step("KeyQ", 12), Some(60));
        assert_eq!(KeyMap::Rows.step("KeyZ", 7), Some(35));
        assert_eq!(KeyMap::Rows.step("KeyA", 7), Some(45));
        assert_eq!(KeyMap::Periods.step("KeyA", 7), Some(42));
        assert_eq!(KeyMap::Periods.step("Quote", 7), None);
        assert_eq!(KeyMap::Periods.step("Nothing", 7), None);
        assert_eq!(KeyMap::parse("piano"), Some(KeyMap::Us));
        assert_eq!(KeyMap::parse("nope"), None);
    }
}
