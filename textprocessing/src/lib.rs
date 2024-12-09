use pinyin_zhuyin::{
    decode_pinyin, decode_zhuyin, encode_pinyin, encode_zhuyin, pinyin_to_zhuyin, zhuyin_to_pinyin,
};

use character_converter::{simplified_to_traditional, tokenize, traditional_to_simplified};

use chinese_number::{ChineseCase, ChineseCountMethod, ChineseVariant, NumberToChinese};

use japanese::converter;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[cfg(feature = "mini-alloc")]
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

#[cfg(feature = "console_error_panic_hook")]
pub(crate) fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub(crate) fn main() {
    #[cfg(debug_assertions)]
    log("main");
    #[cfg(feature = "console_error_panic_hook")]
    set_panic_hook();
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
    use super::*;

    #[wasm_bindgen]
    pub fn pinyin_to_zhuyin_wasm_extended(pinyin: String) -> String {
        pinyin_to_zhuyin(&pinyin)
            .or_else(|| encode_pinyin(&pinyin).and_then(|encoded| pinyin_to_zhuyin(&encoded)))
            .or_else(|| encode_zhuyin(&pinyin))
            .unwrap_or_default()
    }

    #[wasm_bindgen]
    pub fn zhuyin_to_pinyin_wasm_extended(zhuyin: String) -> String {
        zhuyin_to_pinyin(&zhuyin)
            .or_else(|| decode_zhuyin(&zhuyin))
            .unwrap_or_default()
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
    pub fn number_to_chinese_f128(number: String, uppercase: bool, countmethod: i64) -> String {
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
    pub fn convert_japanese_to_kana(text: String) -> String {
        converter::convert_hiragana_to_katakana_string(&text)
    }

    #[wasm_bindgen]
    pub fn convert_japanese_to_kanji(text: String) -> String {
        converter::convert_katakana_to_hiragana_string(&text)
    }
}
