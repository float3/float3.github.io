use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn recursive_ji_chord_progression_abc() -> String {
    recursive_ji_core::chord_progression_abc().unwrap_or_else(|err| abc_error(&err.to_string()))
}

#[wasm_bindgen]
pub fn recursive_ji_note_splits_abc() -> String {
    recursive_ji_core::note_splits_abc().unwrap_or_else(|err| abc_error(&err.to_string()))
}

fn abc_error(message: &str) -> String {
    format!("X:1\nL:1/1\nK:C\n\"{}\"z |]\n", message.replace('"', "'"))
}
