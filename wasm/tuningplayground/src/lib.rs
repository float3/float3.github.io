use std::str::FromStr;
use std::sync::LazyLock;
use std::sync::Mutex;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub mod midi;

use music21_rs::Pitch;
use music21_rs::chord::Chord;
use music21_rs::tuningsystem::{ALL_TUNING_SYSTEMS, TuningSystem};

#[cfg(feature = "wasm")]
static TUNING_SYSTEM: LazyLock<Mutex<TuningSystem>> =
    LazyLock::new(|| Mutex::new(TuningSystem::EqualTemperament { octave_size: 12 }));
#[cfg(feature = "wasm")]
static KEYMAP: Mutex<KeyMap> = Mutex::new(KeyMap::Us);
static CHORD_NAME: Mutex<String> = Mutex::new(String::new());

#[cfg(feature = "wasm")]
#[derive(Clone, Copy)]
enum KeyMap {
    Us,
    UsExtended,
    Qwertz,
    German,
    Azerty,
    Linear,
}

#[cfg(feature = "wasm")]
impl KeyMap {
    fn from_str(keymap: &str) -> Option<Self> {
        match keymap.to_lowercase().as_str() {
            "us" | "qwerty" => Some(Self::Us),
            "us-extended" | "extended" | "qwerty-extended" => Some(Self::UsExtended),
            "qwertz" => Some(Self::Qwertz),
            "de" | "german" => Some(Self::German),
            "azerty" | "fr" | "french" => Some(Self::Azerty),
            "linear" | "chromatic" => Some(Self::Linear),
            _ => None,
        }
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    fn debug(s: &str);
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    fn info(s: &str);

    fn createTone(
        index: usize,
        frequency: f64,
        cents: f64,
        name: String,
        tuning_system: JsValue,
    ) -> JsValue;
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn get_tone(index: usize) -> JsValue {
    // Get the current tuning system and calculate frequency using it
    let tuning_system = TUNING_SYSTEM.lock().expect("couldn't lock");
    let frequency = tuning_system.frequency_at(index as f64);
    let octave_size = tuning_system.octave_size() as f64;

    // Calculate cents relative to the tuning system's octave
    let cents =
        ((index as f64 - 69.0) * 100.0 * 12.0 / octave_size) % (1200.0 * 12.0 / octave_size);

    let note_names = music21_rs::tuningsystem::TWELVE_TONE_NAMES_SHARP;
    let pitch_class = note_names[index % 12];

    // Calculate octave: A4 (MIDI 69) is in octave 4, so octave changes at C.
    // C is MIDI 0 (octave -1), C1 is MIDI 12, C4 is MIDI 60, etc.
    // Octave = (MIDI note - 12) / 12, then round down (floor)
    let octave = (index as i32 - 12) / 12;
    let note_name = format!("{}N{}", pitch_class, octave);

    createTone(index, frequency, cents, note_name, JsValue::NULL)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn get_tuning_size() -> usize {
    TUNING_SYSTEM.lock().expect("couldn't lock").octave_size() as usize
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn from_keymap(key: &str) -> i32 {
    use keymapping::{
        AZERTY_KEYMAP, GERMAN_KEYMAP, LINEAR_KEYMAP, QWERTZ_KEYMAP, US_EXTENDED_KEYMAP, US_KEYMAP,
    };

    match *KEYMAP.lock().expect("couldn't lock") {
        KeyMap::Us => *US_KEYMAP.get(key).unwrap_or(&-1),
        KeyMap::UsExtended => *US_EXTENDED_KEYMAP.get(key).unwrap_or(&-1),
        KeyMap::Qwertz => *QWERTZ_KEYMAP.get(key).unwrap_or(&-1),
        KeyMap::German => *GERMAN_KEYMAP.get(key).unwrap_or(&-1),
        KeyMap::Azerty => *AZERTY_KEYMAP.get(key).unwrap_or(&-1),
        KeyMap::Linear => *LINEAR_KEYMAP.get(key).unwrap_or(&-1),
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn set_keymap(keymap: &str) {
    match KeyMap::from_str(keymap) {
        Some(keymap) => {
            *KEYMAP.lock().expect("couldn't lock") = keymap;
        }
        None => {
            #[cfg(debug_assertions)]
            error("Invalid keymap");
        }
    }
}

fn normalize_tuning_system_name(name: &str) -> String {
    name.trim()
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect()
}

fn parse_tuning_system(tuning_system: &str, octave_size: usize) -> Option<TuningSystem> {
    let normalized = normalize_tuning_system_name(tuning_system);
    if normalized.is_empty() {
        return None;
    }

    if normalized == "12tet" {
        return Some(TuningSystem::EqualTemperament {
            octave_size: octave_size.max(1) as _,
        });
    }

    for tuning in ALL_TUNING_SYSTEMS {
        let id = normalize_tuning_system_name(tuning.id());
        let display = normalize_tuning_system_name(tuning.display_name());
        if normalized == id || normalized == display {
            return Some(match tuning {
                TuningSystem::EqualTemperament { .. } => TuningSystem::EqualTemperament {
                    octave_size: octave_size.max(1) as _,
                },
                other => other,
            });
        }
    }

    TuningSystem::from_str(tuning_system)
        .ok()
        .map(|tuning| match tuning {
            TuningSystem::EqualTemperament { .. } => TuningSystem::EqualTemperament {
                octave_size: octave_size.max(1) as _,
            },
            other => other,
        })
}

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

fn positive_integer_core(value: &str, fallback: usize) -> usize {
    value
        .parse::<usize>()
        .ok()
        .filter(|value| *value > 0)
        .unwrap_or(fallback)
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

fn tuning_hash_or_fallback_core(keys: &str, fallback_hash: &str) -> String {
    let hash = tuning_marked_hash_core(keys);
    if hash.is_empty() {
        fallback_hash.trim_start_matches('#').to_string()
    } else {
        hash
    }
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
/// same pitch. It used to build an ABC token instead, for abcjs to parse again
/// in the browser; that round trip through a second notation is gone.
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

/// The chord being held down, engraved as one labelled bar of SVG.
///
/// This used to return ABC for abcjs to lay out in the browser, which is 1.3 MB
/// of JavaScript fetched to draw a single bar of a single chord. The staff is
/// drawn here instead, in the wasm the page has already loaded.
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
    fn parses_positive_integers_with_fallbacks() {
        assert_eq!(positive_integer_core("24", 12), 24);
        assert_eq!(positive_integer_core("0", 12), 12);
        assert_eq!(positive_integer_core("nope", 12), 12);
    }

    #[test]
    fn canonicalizes_marked_key_hashes() {
        assert_eq!(tuning_marked_hash_core("5,3,5,-1"), "-1,3,5");
        assert_eq!(tuning_hash_or_fallback_core("", "#12,14"), "12,14");
    }
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
pub fn tuning_positive_integer(value: &str, fallback: usize) -> usize {
    positive_integer_core(value, fallback)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn tuning_marked_hash(keys: &str) -> String {
    tuning_marked_hash_core(keys)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn tuning_hash_or_fallback(keys: &str, fallback_hash: &str) -> String {
    tuning_hash_or_fallback_core(keys, fallback_hash)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn set_tuning_system(tuning_system: &str, octave_size: usize, _step_size: usize) {
    let new_system =
        parse_tuning_system(tuning_system, octave_size).unwrap_or(TuningSystem::EqualTemperament {
            octave_size: octave_size.max(12) as _,
        });

    let mut ts = TUNING_SYSTEM.lock().expect("couldn't lock");
    *ts = new_system;
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn available_tuning_systems() -> js_sys::Array {
    let arr = js_sys::Array::new();

    for tuning in ALL_TUNING_SYSTEMS {
        let obj = js_sys::Object::new();
        let id = JsValue::from_str(tuning.id());
        let display = JsValue::from_str(tuning.display_name());
        js_sys::Reflect::set(&obj, &JsValue::from_str("id"), &id).unwrap();
        js_sys::Reflect::set(&obj, &JsValue::from_str("display"), &display).unwrap();
        arr.push(&obj);
    }

    arr
}
