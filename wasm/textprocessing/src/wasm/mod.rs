pub mod chinese;
pub mod encoding;
pub mod japanese;
pub mod korean;
pub mod numbers;
pub mod scripts;

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
        (20, true) => chinese::encode_pinyin_wasm(text),
        (20, false) => chinese::decode_pinyin_wasm(text),
        (21, true) => chinese::encode_zhuyin_wasm(text),
        (21, false) => chinese::decode_zhuyin_wasm(text),
        (22, true) => chinese::tokenize_wasm(text),
        (23, true) => korean::hangeul_to_mccune_reischauer_romanization(&text),
        (23, false) => korean::mccune_reischauer_romanization_to_hangeul(&text),
        (24, true) => korean::rr_to_mr(&text),
        (24, false) => korean::mr_to_rr(&text),
        (25, true) => encoding::text_to_hex_bytes(text),
        (25, false) => encoding::hex_bytes_to_text(text),
        (26, true) => encoding::text_to_binary_bytes(text),
        (26, false) => encoding::binary_bytes_to_text(text),
        (27, true) => encoding::text_to_base64(text),
        (27, false) => encoding::base64_to_text(text),
        (28, true) => encoding::escape_html(text),
        (28, false) => encoding::unescape_html(text),
        (29, true) => encoding::text_to_code_points(text),
        (29, false) => encoding::code_points_to_text(text),
        (30, true) => encoding::integer_to_bytes(text, false),
        (30, false) => encoding::bytes_to_integer(text, false),
        (31, true) => encoding::integer_to_bytes(text, true),
        (31, false) => encoding::bytes_to_integer(text, true),
        (32, true) | (32, false) => encoding::reverse_byte_order(text),
        (33, true) => japanese::kana_to_romaji(text),
        (34, true) => scripts::transliterate_cyrillic(text),
        (35, true) => scripts::transliterate_greek(text),
        _ => text,
    }
}
