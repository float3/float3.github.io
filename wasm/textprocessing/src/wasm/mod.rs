#[cfg(feature = "chinese")]
pub mod chinese;
pub mod encoding;
pub mod japanese;
#[cfg(feature = "korean")]
pub mod korean;
pub mod numbers;
pub mod scripts;

use wasm_bindgen::prelude::*;

/// The three wasm packages built from this crate: the base one, and one each
/// carrying the Chinese and the Korean dictionaries as well.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Package {
    Base,
    Chinese,
    Korean,
}

impl Package {
    pub fn name(self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Chinese => "chinese",
            Self::Korean => "korean",
        }
    }
}

/// One global index space shared by every build of this crate: a package built
/// without `chinese` has no Chinese dispatcher, so the numbering the TypeScript
/// side uses never shifts when a language is added or left out. Which package a
/// transform needs is whichever dispatcher answers for it, so there is no list
/// of indices per language to keep in step with the arms.
#[wasm_bindgen]
pub fn transform_text(index: u32, left_to_right: bool, text: String) -> String {
    dispatch(index, left_to_right, text).map_or_else(|text| text, |(_, output)| output)
}

/// Runs a transform and names the package it needs, or hands the text back when
/// no dispatcher in this build has an arm for it.
pub fn dispatch(
    index: u32,
    left_to_right: bool,
    text: String,
) -> Result<(Package, String), String> {
    #[cfg(feature = "chinese")]
    let text = match chinese_transform(index, left_to_right, text) {
        Ok(output) => return Ok((Package::Chinese, output)),
        Err(text) => text,
    };
    #[cfg(feature = "korean")]
    let text = match korean_transform(index, left_to_right, text) {
        Ok(output) => return Ok((Package::Korean, output)),
        Err(text) => text,
    };
    base_transform(index, left_to_right, text).map(|output| (Package::Base, output))
}

#[cfg(feature = "chinese")]
fn chinese_transform(index: u32, left_to_right: bool, text: String) -> Result<String, String> {
    match (index, left_to_right) {
        (0, true) => Ok(chinese::pinyin_to_zhuyin_wasm_extended(text)),
        (0, false) => Ok(chinese::zhuyin_to_pinyin_wasm_extended(text)),
        (1, true) => Ok(chinese::traditional_to_simplified_wasm(text)),
        (1, false) => Ok(chinese::simplified_to_traditional_wasm(text)),
        (4, true) => Ok(chinese::to_pinyin_wasm(text)),
        (5, true) => Ok(chinese::to_pinyin_multi_wasm(text)),
        (8, true) => Ok(chinese::to_zhuyin_wasm(text)),
        (9, true) => Ok(chinese::to_zhuyin_multi_wasm(text)),
        (10, true) => Ok(numbers::number_to_chinese_f128(text, true, 0)),
        (11, true) => Ok(numbers::number_to_chinese_f128(text, true, 1)),
        (12, true) => Ok(numbers::number_to_chinese_f128(text, true, 2)),
        (13, true) => Ok(numbers::number_to_chinese_f128(text, true, 3)),
        (14, true) => Ok(numbers::number_to_chinese_f128(text, false, 0)),
        (15, true) => Ok(numbers::number_to_chinese_f128(text, false, 1)),
        (16, true) => Ok(numbers::number_to_chinese_f128(text, false, 2)),
        (17, true) => Ok(numbers::number_to_chinese_f128(text, false, 3)),
        (20, true) => Ok(chinese::decode_pinyin_wasm(text)),
        (20, false) => Ok(chinese::encode_pinyin_wasm(text)),
        (21, true) => Ok(chinese::encode_zhuyin_wasm(text)),
        (21, false) => Ok(chinese::decode_zhuyin_wasm(text)),
        (22, true) => Ok(chinese::tokenize_wasm(text)),
        _ => Err(text),
    }
}

#[cfg(feature = "korean")]
fn korean_transform(index: u32, left_to_right: bool, text: String) -> Result<String, String> {
    match (index, left_to_right) {
        (3, true) => Ok(korean::hanja_to_hangeul(&text)),
        (3, false) => Ok(korean::hangeul_to_hanja(&text)),
        (6, true) => Ok(korean::hanja_to_hangeul_all_variants(&text)),
        (19, true) => Ok(korean::romanize_hangeul(&text)),
        (19, false) => Ok(korean::roman_to_hangeul(&text)),
        (23, true) => Ok(korean::hangeul_to_mccune_reischauer_romanization(&text)),
        (23, false) => Ok(korean::mccune_reischauer_romanization_to_hangeul(&text)),
        (24, true) => Ok(korean::rr_to_mr(&text)),
        (24, false) => Ok(korean::mr_to_rr(&text)),
        _ => Err(text),
    }
}

fn base_transform(index: u32, left_to_right: bool, text: String) -> Result<String, String> {
    match (index, left_to_right) {
        (2, true) => Ok(japanese::convert_hiragana_to_katakana(text)),
        (2, false) => Ok(japanese::convert_katakana_to_hiragana(text)),
        (7, true) => Ok(numbers::arabic_to_roman(text)),
        (7, false) => Ok(numbers::roman_to_arabic(text)),
        (18, true) => Ok(numbers::number_to_japanese(text)),
        (25, true) => Ok(encoding::text_to_hex_bytes(text)),
        (25, false) => Ok(encoding::hex_bytes_to_text(text)),
        (26, true) => Ok(encoding::text_to_binary_bytes(text)),
        (26, false) => Ok(encoding::binary_bytes_to_text(text)),
        (27, true) => Ok(encoding::text_to_base64(text)),
        (27, false) => Ok(encoding::base64_to_text(text)),
        (28, true) => Ok(encoding::escape_html(text)),
        (28, false) => Ok(encoding::unescape_html(text)),
        (29, true) => Ok(encoding::text_to_code_points(text)),
        (29, false) => Ok(encoding::code_points_to_text(text)),
        (30, true) => Ok(encoding::integer_to_bytes(text, false)),
        (30, false) => Ok(encoding::bytes_to_integer(text, false)),
        (31, true) => Ok(encoding::integer_to_bytes(text, true)),
        (31, false) => Ok(encoding::bytes_to_integer(text, true)),
        (32, true) | (32, false) => Ok(encoding::reverse_byte_order(text)),
        (33, true) => Ok(japanese::kana_to_romaji(text)),
        (34, true) => Ok(scripts::transliterate_cyrillic(text)),
        (35, true) => Ok(scripts::transliterate_greek(text)),
        (36, true) => Ok(encoding::url_encode(text)),
        (36, false) => Ok(encoding::url_decode(text)),
        _ => Err(text),
    }
}
