use chinese_number::{ChineseCase, ChineseCountMethod, ChineseVariant, NumberToChinese};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn number_to_japanese(number: String) -> String {
    number
        .parse::<usize>()
        .map(|number| {
            japanese_number_converter::JapaneseNumber::convert(number)
                .kanji()
                .to_string()
        })
        .unwrap_or_default()
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
    number
        .parse::<i32>()
        .ok()
        .and_then(roman::to)
        .unwrap_or_default()
}

#[wasm_bindgen]
pub fn roman_to_arabic(roman: String) -> String {
    match roman::from(&roman) {
        Some(num) => num.to_string(),
        None => String::from(""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_numbers_do_not_panic() {
        assert_eq!(number_to_japanese("nope".to_string()), "");
        assert_eq!(arabic_to_roman("nope".to_string()), "");
    }

    #[test]
    fn roman_numerals_round_trip() {
        assert_eq!(arabic_to_roman("3339".to_string()), "MMMCCCXXXIX");
        assert_eq!(roman_to_arabic("MMMCCCXXXIX".to_string()), "3339");
    }
}
