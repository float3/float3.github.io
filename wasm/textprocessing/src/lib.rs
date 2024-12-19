use pinyin_zhuyin::{
    decode_pinyin, decode_zhuyin, encode_pinyin, encode_zhuyin, pinyin_to_zhuyin, zhuyin_to_pinyin,
};

use character_converter::{simplified_to_traditional, tokenize, traditional_to_simplified};

use chinese_number::{ChineseCase, ChineseCountMethod, ChineseVariant, NumberToChinese};

use japanese::converter;

use roman::{from, to};

use pinyin::ToPinyin;
use pinyin::ToPinyinMulti;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[cfg(feature = "wasm")]
#[cfg(feature = "mini-alloc")]
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub(crate) fn main() {
    #[cfg(debug_assertions)]
    log("main");
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
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

}

#[cfg(feature = "wasm")]
mod wasm_functions {
    use std::collections::HashSet;

    use super::*;

    #[wasm_bindgen]
    pub fn pinyin_to_zhuyin_wasm_extended(pinyin: String) -> String {
        pinyin
            .split(' ')
            .map(|pinyin| {
                pinyin_to_zhuyin(&pinyin)
                    .or_else(|| {
                        encode_pinyin(&pinyin).and_then(|encoded| pinyin_to_zhuyin(&encoded))
                    })
                    .or_else(|| encode_zhuyin(&pinyin))
                    .unwrap_or_default()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    #[wasm_bindgen]
    pub fn zhuyin_to_pinyin_wasm_extended(zhuyin: String) -> String {
        pinyin
            .split(' ')
            .map(|zhuyin| {
                zhuyin_to_pinyin(&zhuyin)
                    .or_else(|| decode_zhuyin(&zhuyin))
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
    pub fn convert_hiragana_to_katakana(text: String) -> String {
        converter::convert_hiragana_to_katakana_string(&text)
    }

    #[wasm_bindgen]
    pub fn convert_katakana_to_hiragana(text: String) -> String {
        converter::convert_katakana_to_hiragana_string(&text)
    }

    #[wasm_bindgen]
    pub fn hangeul_to_hanja(input: &str) -> String {
        input
            .chars()
            .map(|c| {
                // For simplicity, we take the first available hanja variant.
                // In practice, you might want to handle errors or multiple variants.
                hanja::get(c).expect("No Hanja found for given Hangeul character")[0].0
            })
            .collect()
    }

    #[wasm_bindgen]
    pub fn hanja_to_hangeul(input: &str) -> String {
        input
            .chars()
            .map(|c| {
                // We must invert the lookup. Since the hanja::get function
                // works from Hangeul to Hanja, we need to search all Hangeul
                // characters that produce this Hanja. This is inefficient:
                // a real implementation would likely use a precomputed map.

                // For demonstration, assume we have a known range of Hangeul
                // and attempt to find the corresponding character:
                for hangeul_candidate in '\u{AC00}'..='\u{D7A3}' {
                    if let Some(results) = hanja::get(hangeul_candidate) {
                        if results.iter().any(|&(h, _)| h == c) {
                            return hangeul_candidate;
                        }
                    }
                }
                panic!("No Hangeul found for given Hanja character");
            })
            .collect()
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
        pinyin_to_zhuyin_wasm_extended(to_pinyin_wasm(hanzi))
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
        pinyin_to_zhuyin_wasm_extended(to_pinyin_multi_wasm(text))
    }

    #[wasm_bindgen]
    pub fn hanja_to_hangeul_all_variants(input: &str) -> String {
        let mut readings: Vec<String> = Vec::new();

        for hanja_char in input.chars() {
            let mut hangeul_candidates = HashSet::new();

            // Iterate over the known range of Hangeul syllables
            for hangeul_candidate in '\u{AC00}'..='\u{D7A3}' {
                if let Some(results) = hanja::get(hangeul_candidate) {
                    if results.iter().any(|&(h, _)| h == hanja_char) {
                        hangeul_candidates.insert(hangeul_candidate);
                    }
                }
            }

            if hangeul_candidates.is_empty() {
                panic!(
                    "No Hangeul readings found for Hanja character {}",
                    hanja_char
                );
            }

            // Convert the HashSet into a sorted String
            let mut candidate_list: Vec<char> = hangeul_candidates.into_iter().collect();
            candidate_list.sort(); // Optional: Sort to ensure consistent order

            readings.push(candidate_list.into_iter().collect());
        }

        readings.join(" ")
    }

    #[wasm_bindgen]
    pub fn number_to_chinese_f128(number: String, uppercase: bool, countmethod: i32) -> String {
        let variant: ChineseVariant = ChineseVariant::Traditional;

        let case: ChineseCase = match uppercase {
            true => ChineseCase::Upper,
            false => ChineseCase::Lower,
        };

        let method: ChineseCountMethod = match countmethod {
            0 => ChineseCountMethod::Low,
            1 => ChineseCountMethod::TenThousand,
            2 => ChineseCountMethod::Middle,
            _ => ChineseCountMethod::High,
        };

        let parsed_number = number
            .parse::<i128>()
            .map(|num| num.to_chinese(variant, case, method))
            .or_else(|_| {
                number
                    .parse::<u128>()
                    .map(|num| num.to_chinese(variant, case, method))
            })
            .or_else(|_| {
                number
                    .parse::<f64>()
                    .map(|num| num.to_chinese(variant, case, method))
            });

        match parsed_number {
            Ok(Ok(chinese)) => chinese,
            _ => String::from(""),
        }
    }

    #[wasm_bindgen]
    pub fn arabic_to_roman(number: String) -> String {
        roman::to(number.parse::<u32>().unwrap())
    }

    #[wasm_bindgen]
    pub fn roman_to_arabic(roman: String) -> String {
        roman::from(&roman).to_string()
    }
}
