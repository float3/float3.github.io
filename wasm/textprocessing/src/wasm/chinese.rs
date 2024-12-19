use character_converter::{simplified_to_traditional, tokenize, traditional_to_simplified};
use pinyin::{ToPinyin, ToPinyinMulti};
use pinyin_zhuyin::{
    decode_pinyin, decode_zhuyin, encode_pinyin, encode_zhuyin, pinyin_to_zhuyin, zhuyin_to_pinyin,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn pinyin_to_zhuyin_wasm_extended(pinyin: String) -> String {
    pinyin
        .split(' ')
        .map(|pinyin| {
            pinyin_to_zhuyin(pinyin)
                .or_else(|| encode_pinyin(pinyin).and_then(|encoded| pinyin_to_zhuyin(&encoded)))
                .or_else(|| encode_zhuyin(pinyin))
                .unwrap_or_default()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[wasm_bindgen]
pub fn zhuyin_to_pinyin_wasm_extended(zhuyin: String) -> String {
    zhuyin
        .split(' ')
        .map(|zhuyin| {
            zhuyin_to_pinyin(zhuyin)
                .or_else(|| decode_zhuyin(zhuyin))
                .unwrap_or_default()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[wasm_bindgen]
pub fn encode_pinyin_wasm(pinyin: String) -> String {
    encode_pinyin(&pinyin).unwrap_or_default()
}

#[wasm_bindgen]
pub fn decode_pinyin_wasm(pinyin: String) -> String {
    decode_pinyin(&pinyin).unwrap_or_default()
}

#[wasm_bindgen]
pub fn encode_zhuyin_wasm(zhuyin: String) -> String {
    encode_zhuyin(&zhuyin).unwrap_or_default()
}

#[wasm_bindgen]
pub fn decode_zhuyin_wasm(zhuyin: String) -> String {
    decode_zhuyin(&zhuyin).unwrap_or_default()
}

#[wasm_bindgen]
pub fn pinyin_to_zhuyin_wasm(pinyin: String) -> String {
    pinyin_to_zhuyin(&pinyin).unwrap_or_default()
}

#[wasm_bindgen]
pub fn zhuyin_to_pinyin_wasm(zhuyin: String) -> String {
    zhuyin_to_pinyin(&zhuyin).unwrap_or_default()
}

#[wasm_bindgen]
pub fn simplified_to_traditional_wasm(simplified: String) -> String {
    simplified_to_traditional(&simplified).to_string()
}

#[wasm_bindgen]
pub fn traditional_to_simplified_wasm(traditional: String) -> String {
    traditional_to_simplified(&traditional).to_string()
}

#[wasm_bindgen]
pub fn tokenize_wasm(text: String) -> String {
    tokenize(&text).join(" ")
}

#[wasm_bindgen]
pub fn to_pinyin_wasm(text: String) -> String {
    let mut aggregate_pinyin = String::new();
    for pinyin in text.as_str().to_pinyin().flatten() {
        aggregate_pinyin.push_str(pinyin.with_tone());
        aggregate_pinyin.push(' ');
    }
    aggregate_pinyin.pop();
    aggregate_pinyin
}

#[wasm_bindgen]
pub fn to_zhuyin_wasm(hanzi: String) -> String {
    super::chinese::pinyin_to_zhuyin_wasm_extended(to_pinyin_wasm(hanzi))
}

#[wasm_bindgen]
pub fn to_pinyin_multi_wasm(text: String) -> String {
    let mut aggregate_pinyin = String::new();
    for multi in text.as_str().to_pinyin_multi().flatten() {
        for pinyin in multi {
            aggregate_pinyin.push_str(pinyin.with_tone());
            aggregate_pinyin.push(' ');
        }
    }
    aggregate_pinyin.pop();
    aggregate_pinyin
}

#[wasm_bindgen]
pub fn to_zhuyin_multi_wasm(text: String) -> String {
    super::chinese::pinyin_to_zhuyin_wasm_extended(to_pinyin_multi_wasm(text))
}
