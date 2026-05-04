use std::sync::Mutex;
use std::sync::OnceLock;
#[cfg(feature = "wasm")]
use tuning_systems::TuningSystem;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
static OCTAVE_SIZE: Mutex<usize> = Mutex::new(12);
#[cfg(feature = "wasm")]
static STEP_SIZE: Mutex<usize> = Mutex::new(7);
#[cfg(feature = "wasm")]
static TUNING_SYSTEM: Mutex<TuningSystem> =
    Mutex::new(TuningSystem::EqualTemperament { octave_size: 12 });
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
    use tuning_systems::Tone;

    let tun_sys: TuningSystem = *TUNING_SYSTEM.lock().expect("couldn't lock");

    let tone: Tone = Tone::new(tun_sys, index);

    createTone(
        index,
        tone.frequency(),
        tone.cents(),
        tone.name,
        JsValue::NULL,
    )
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn get_tuning_size() -> usize {
    *OCTAVE_SIZE.lock().expect("couldn't lock")
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
    pitch_class: usize,
}

type LUTType = Vec<String>;

static CHORD_LUT: OnceLock<LUTType> = OnceLock::new();

fn static_data() -> &'static LUTType {
    CHORD_LUT.get_or_init(|| {
        include_str!("../../../content/misc/plaintext/chords.txt")
            .split(';')
            .map(|s| s.to_string())
            .collect::<LUTType>()
    })
}

fn split_chord_input(input: &str) -> Vec<&str> {
    input
        .split(|c: char| c.is_whitespace() || c == ',' || c == ';')
        .filter(|token| !token.trim().is_empty())
        .collect()
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

fn pitch_class(full_name: &str) -> Result<usize, String> {
    match full_name {
        "B#" | "C" | "Dbb" => Ok(0),
        "B##" | "C#" | "Db" => Ok(1),
        "C##" | "D" | "Ebb" => Ok(2),
        "D#" | "Eb" | "Fbb" => Ok(3),
        "D##" | "E" | "Fb" => Ok(4),
        "E#" | "F" | "Gbb" => Ok(5),
        "E##" | "F#" | "Gb" => Ok(6),
        "F##" | "G" | "Abb" => Ok(7),
        "G#" | "Ab" => Ok(8),
        "G##" | "A" | "Bbb" => Ok(9),
        "A#" | "Bb" | "Cbb" => Ok(10),
        "A##" | "B" | "Cb" => Ok(11),
        _ => Err(format!("Invalid note: {full_name}")),
    }
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

    let full_name = format!("{name}{accidental}");
    let pitch_class = pitch_class(&full_name)?;

    let octave = parse_octave(note);
    let abc_octave = octave.unwrap_or(4) - 4;
    let octave_str = if abc_octave < 0 {
        ",".repeat(abc_octave.unsigned_abs() as usize)
    } else {
        "'".repeat(abc_octave as usize)
    };

    let abc_accidental = accidental.replace('#', "^").replace('b', "_");

    Ok(ParsedNote {
        abc: format!("{abc_accidental}{name}{octave_str}"),
        pitch_class,
    })
}

fn chord_bitmask(input: &str) -> Result<usize, String> {
    let notes = split_chord_input(input)
        .into_iter()
        .map(parse_note_token)
        .collect::<Result<Vec<_>, _>>()?;

    if notes.is_empty() {
        return Err("Enter at least one note".to_string());
    }

    Ok(notes.into_iter().fold(0usize, |bitmask, note| {
        bitmask | (1usize << note.pitch_class)
    }))
}

pub fn chordname_core(input: &str) -> Result<String, String> {
    let bitmask = chord_bitmask(input)?;
    chordname_from_bitmask(bitmask).ok_or_else(|| format!("Unknown chord bitmask: {bitmask}"))
}

pub fn chord_details_core(input: &str) -> Result<String, String> {
    let bitmask = chord_bitmask(input)?;
    let chord = chordname_from_bitmask(bitmask)
        .ok_or_else(|| format!("Unknown chord bitmask: {bitmask}"))?;
    let pitch_classes = (0..12)
        .filter(|pitch_class| bitmask & (1usize << pitch_class) != 0)
        .map(|pitch_class| pitch_class.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    Ok(format!(
        "Name: {chord} | Pitch classes: {pitch_classes} | Bitmask: {bitmask}"
    ))
}

fn set_chord_name(chord: &str) {
    let mut chord_name = CHORD_NAME.lock().expect("couldn't lock");
    chord_name.clear();
    chord_name.push_str(chord);
}

fn abc_label(label: &str) -> String {
    label.replace('"', "'")
}

fn chordname_from_bitmask(bitmask: usize) -> Option<String> {
    bitmask
        .checked_sub(1)
        .and_then(|index| static_data().get(index))
        .filter(|name| !name.is_empty())
        .cloned()
}

pub fn convert_notes_core(input: Vec<String>) -> String {
    let mut notes = Vec::new();
    let mut bitmask = 0usize;

    for note_str in input.into_iter() {
        match parse_note_token(&note_str) {
            Ok(note) => {
                notes.push(note.abc);
                bitmask |= 1usize << note.pitch_class;
            }
            Err(err) => {
                set_chord_name(&err);
                return format!("X: 1\nL: 1/1\n|\"{}\"[]|", abc_label(&err));
            }
        }
    }

    let chord = chordname_from_bitmask(bitmask).unwrap_or_else(|| "Unknown chord".to_string());
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
    fn generated_lookup_uses_bitmask_order() {
        assert_eq!(chordname_from_bitmask(1).unwrap(), "C");
        assert_eq!(chordname_from_bitmask(4095).unwrap(), "C-aggregate");
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
pub fn set_tuning_system(tuning_system: &str, octave_size: usize, step_size: usize) {
    let tuning_system: Option<TuningSystem> = match tuning_system.to_lowercase().as_str() {
        "stepmethod" => Some(TuningSystem::StepMethod {
            octave_size,
            step_size,
        }),
        "equaltemperament" => Some(TuningSystem::EqualTemperament { octave_size }),
        "justintonation" => Some(TuningSystem::JustIntonation),
        "justintonation24" => Some(TuningSystem::JustIntonation24),
        "pythagoreantuning" => Some(TuningSystem::PythagoreanTuning),
        "fivelimit" => Some(TuningSystem::FiveLimit),
        "elevenlimit" => Some(TuningSystem::ElevenLimit),
        "fortythreetone" => Some(TuningSystem::FortyThreeTone),
        "indian" => Some(TuningSystem::Indian),
        "indianalt" => Some(TuningSystem::IndianAlt),
        "indianfull" => Some(TuningSystem::Indian22),
        // "thai" => Some(TuningSystem::Thai),
        // "javanese" => Some(TuningSystem::Javanese),
        "wholetone" => Some(TuningSystem::WholeTone),
        "quartertone" => Some(TuningSystem::QuarterTone),
        _ => None,
    };
    match tuning_system {
        Some(tuning_system) => {
            *TUNING_SYSTEM.lock().expect("couldn't lock") = tuning_system;
            *OCTAVE_SIZE.lock().expect("couldn't lock") = octave_size;
            *STEP_SIZE.lock().expect("couldn't lock") = step_size;
        }
        None => {
            #[cfg(debug_assertions)]
            error("Invalid tuning system");
        }
    }
}
