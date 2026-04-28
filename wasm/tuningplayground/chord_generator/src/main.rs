use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use music21_rs::chord::Chord;

fn main() {
    let chords = generate_chords();

    let text = chords
        .values()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(";")
        + ";";
    let json = json_encode(&chords);

    let output_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..")
        .join("content")
        .join("misc")
        .join("plaintext");
    fs::write(output_dir.join("chords.txt"), text).expect("failed to write chords.txt");
    fs::write(output_dir.join("chords.json"), json).expect("failed to write chords.json");
}

fn generate_chords() -> BTreeMap<u16, String> {
    let mut chords = BTreeMap::new();

    for size in 1..=12 {
        let mut combination = Vec::with_capacity(size);
        collect_combinations(size, 0, &mut combination, &mut chords);
    }

    chords
}

fn collect_combinations(
    size: usize,
    start: i32,
    combination: &mut Vec<i32>,
    chords: &mut BTreeMap<u16, String>,
) {
    if combination.len() == size {
        let bitmask = combination
            .iter()
            .fold(0_u16, |mask, note| mask | (1_u16 << *note as u16));
        let chord = Chord::new(Some(combination.as_slice())).expect("failed to construct chord");
        chords.insert(bitmask, chord.pitched_common_name());
        return;
    }

    for note in start..12 {
        combination.push(note);
        collect_combinations(size, note + 1, combination, chords);
        combination.pop();
    }
}

fn json_encode(chords: &BTreeMap<u16, String>) -> String {
    let entries = chords
        .iter()
        .map(|(key, value)| format!("\"{}\":\"{}\"", key, escape_json_string(value)))
        .collect::<Vec<_>>()
        .join(",");

    format!("{{{entries}}}")
}

fn escape_json_string(value: &str) -> String {
    value
        .chars()
        .flat_map(|ch| match ch {
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            '\\' => "\\\\".chars().collect::<Vec<_>>(),
            '\n' => "\\n".chars().collect::<Vec<_>>(),
            '\r' => "\\r".chars().collect::<Vec<_>>(),
            '\t' => "\\t".chars().collect::<Vec<_>>(),
            '\u{08}' => "\\b".chars().collect::<Vec<_>>(),
            '\u{0c}' => "\\f".chars().collect::<Vec<_>>(),
            ch if ch.is_control() => format!("\\u{:04x}", ch as u32).chars().collect::<Vec<_>>(),
            ch => vec![ch],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_every_non_empty_pitch_class_set_in_bitmask_order() {
        let chords = generate_chords();
        let keys = chords.keys().copied().collect::<Vec<_>>();

        assert_eq!(chords.len(), 4095);
        assert_eq!(keys.first(), Some(&1));
        assert_eq!(keys.last(), Some(&4095));
        assert_eq!(chords.get(&1).map(String::as_str), Some("C"));
        assert_eq!(chords.get(&145).map(String::as_str), Some("C-major triad"));
    }

    #[test]
    fn json_encoder_keeps_compact_object_encoding() {
        let chords = BTreeMap::from([(1, "C".to_string()), (3, "interval \"x\"".to_string())]);

        assert_eq!(
            json_encode(&chords),
            "{\"1\":\"C\",\"3\":\"interval \\\"x\\\"\"}"
        );
    }
}
