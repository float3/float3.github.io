pub mod chinese;
pub mod japanese;
pub mod korean;
pub mod numbers;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn transform_text(index: u32, left_to_right: bool, text: String) -> String {
    match (index, left_to_right) {
        (0, true) => chinese::pinyin_to_zhuyin_wasm_extended(text),
        (0, false) => chinese::zhuyin_to_pinyin_wasm_extended(text),
        (1, true) => chinese::traditional_to_simplified_wasm(text),
        (1, false) => chinese::simplified_to_traditional_wasm(text),
        (2, true) => japanese::convert_hiragana_to_katakana(text),
        (2, false) => japanese::convert_katakana_to_hiragana(text),
        (3, true) => korean::hanja_to_hangeul(&text),
        (3, false) => korean::hangeul_to_hanja(&text),
        (4, true) => chinese::to_pinyin_wasm(text),
        (5, true) => chinese::to_pinyin_multi_wasm(text),
        (6, true) => korean::hanja_to_hangeul_all_variants(&text),
        (7, true) => numbers::arabic_to_roman(text),
        (7, false) => numbers::roman_to_arabic(text),
        (8, true) => chinese::to_zhuyin_wasm(text),
        (9, true) => chinese::to_zhuyin_multi_wasm(text),
        (10, true) => numbers::number_to_chinese_f128(text, true, 0),
        (11, true) => numbers::number_to_chinese_f128(text, true, 1),
        (12, true) => numbers::number_to_chinese_f128(text, true, 2),
        (13, true) => numbers::number_to_chinese_f128(text, true, 3),
        (14, true) => numbers::number_to_chinese_f128(text, false, 0),
        (15, true) => numbers::number_to_chinese_f128(text, false, 1),
        (16, true) => numbers::number_to_chinese_f128(text, false, 2),
        (17, true) => numbers::number_to_chinese_f128(text, false, 3),
        (18, true) => numbers::number_to_japanese(text),
        (19, true) => korean::romanize_hangeul(&text),
        (19, false) => korean::roman_to_hangeul(&text),
        _ => text,
    }
}
