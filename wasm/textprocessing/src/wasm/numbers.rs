use chinese_number::{ChineseCase, ChineseCountMethod, ChineseVariant, NumberToChinese};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn number_to_japanese(number: String) -> String {
    japanese_number_converter::JapaneseNumber::convert(number.parse::<usize>().unwrap())
        .kanji()
        .to_string()
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
    roman::to(number.parse::<i32>().unwrap()).unwrap()
}

#[wasm_bindgen]
pub fn roman_to_arabic(roman: String) -> String {
    match roman::from(&roman) {
        Some(num) => num.to_string(),
        None => String::from(""),
    }
}
