use std::str::FromStr;
use std::sync::LazyLock;
use std::sync::Mutex;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

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
    // For now, return a simple tone based on equal temperament
    // TODO: use music21-rs for tuning system calculations
    let frequency = 440.0 * 2.0_f64.powf((index as f64 - 69.0) / 12.0);
    let cents = ((index as f64 - 69.0) * 100.0) % 1200.0;
    let note_names = vec![
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
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

#[derive(Debug)]
struct ParsedNote {
    abc: String,
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

fn parse_note_token(note: &str) -> Result<ParsedNote, String> {
    let note = note.trim();
    let mut chars = note.chars().peekable();
    let name = chars
        .next()
        .ok_or_else(|| "Expected a note name".to_string())?
        .to_ascii_uppercase();

    if !('A'..='G').contains(&name) {
        return Err(format!("Invalid note: {note}"));
    }

    let mut accidental = String::new();
    while let Some(ch) = chars.peek() {
        match ch {
            '#' => {
                accidental.push('#');
                chars.next();
            }
            'b' | 'B' | '-' => {
                accidental.push('b');
                chars.next();
            }
            _ => break,
        }
    }

    let octave = parse_octave(note);
    let abc_octave = octave.unwrap_or(4) - 4;
    let octave_str = if abc_octave < 0 {
        ",".repeat(abc_octave.unsigned_abs() as usize)
    } else {
        "'".repeat(abc_octave as usize)
    };

    let abc_accidental = accidental.replace('#', "^").replace('b', "_");

    // ABC notation uses case to indicate octave ranges. Use lowercase
    // letters for notes at or above the reference octave (abc_octave >= 0),
    // and uppercase for notes below it. This lets ABC render ledger lines
    // and notes outside the single visible octave correctly.
    let note_letter = if abc_octave >= 0 {
        name.to_ascii_lowercase().to_string()
    } else {
        name.to_string()
    };

    Ok(ParsedNote {
        abc: format!("{abc_accidental}{note_letter}{octave_str}"),
    })
}

fn set_chord_name(chord: &str) {
    let mut chord_name = CHORD_NAME.lock().expect("couldn't lock");
    chord_name.clear();
    chord_name.push_str(chord);
}

fn abc_label(label: &str) -> String {
    label.replace('"', "'")
}

pub fn convert_notes_core(input: Vec<String>) -> String {
    let mut notes = Vec::new();
    let mut note_names = Vec::new();

    for note_str in input.into_iter() {
        match parse_note_token(&note_str) {
            Ok(note) => {
                notes.push(note.abc);
                // Normalize the note string for chord recognition:
                // some callers use an internal 'N' separator for octave
                // (e.g. "C#N4"). The chord parser expects formats like
                // "C#4", so remove a single 'N' if present.
                let normalized = note_str.replace("N", "");
                note_names.push(normalized);
            }
            Err(err) => {
                set_chord_name(&err);
                return format!("X: 1\nL: 1/1\n|\"{}\"[]|", abc_label(&err));
            }
        }
    }

    let chord = Chord::new(note_names)
        .map(|chord| chord.pitched_common_name())
        .unwrap_or_else(|_| "Unknown chord".to_string());
    set_chord_name(&chord);

    format!(
        "X: 1\nL: 1/1\n|\"{}\"[{}]|",
        abc_label(&chord),
        notes.join(" ")
    )
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
