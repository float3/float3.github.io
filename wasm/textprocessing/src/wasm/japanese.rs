use japanese::converter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert_hiragana_to_katakana(text: String) -> String {
    converter::convert_hiragana_to_katakana_string(&text)
}

#[wasm_bindgen]
pub fn convert_katakana_to_hiragana(text: String) -> String {
    converter::convert_katakana_to_hiragana_string(&text)
}
