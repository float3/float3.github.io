use hangeul_conversion::Print;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hangeul_to_hanja(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            hanja::get(c)
                .and_then(|variants| variants.first().map(|(hanja, _)| *hanja))
                .unwrap_or(c)
        })
        .collect()
}

#[wasm_bindgen]
pub fn hanja_to_hangeul(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            for hangeul_candidate in '\u{AC00}'..='\u{D7A3}' {
                if let Some(results) = hanja::get(hangeul_candidate)
                    && results.iter().any(|&(h, _)| h == c)
                {
                    return hangeul_candidate;
                }
            }
            c
        })
        .collect()
}

#[wasm_bindgen]
/// Every reading of each hanja, `/` between one character's readings and a
/// space between characters: `樂` reads `낙/락/악/요`.
pub fn hanja_to_hangeul_all_variants(input: &str) -> String {
    let mut readings: Vec<String> = Vec::new();

    for hanja_char in input.chars() {
        let mut hangeul_candidates = HashSet::new();

        // Iterate over the known range of Hangeul syllables
        for hangeul_candidate in '\u{AC00}'..='\u{D7A3}' {
            if let Some(results) = hanja::get(hangeul_candidate)
                && results.iter().any(|&(h, _)| h == hanja_char)
            {
                hangeul_candidates.insert(hangeul_candidate);
            }
        }

        if hangeul_candidates.is_empty() {
            readings.push(hanja_char.to_string());
        } else {
            let mut candidate_list: Vec<char> = hangeul_candidates.into_iter().collect();
            candidate_list.sort();
            readings.push(
                candidate_list
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<_>>()
                    .join("/"),
            );
        }
    }

    readings.join(" ")
}

#[wasm_bindgen]
pub fn roman_to_hangeul(input: &str) -> String {
    hangeul_conversion::rr::parse(input)
        .map(|blocks| blocks.iter().map(|b| b.hangeul()).collect())
        .unwrap_or_else(|| input.to_string())
}

#[wasm_bindgen]
pub fn romanize_hangeul(input: &str) -> String {
    hangeul_conversion::hangeul::parse(input)
        .map(|blocks| blocks.iter().map(|b| b.revised_romanization()).collect())
        .unwrap_or_else(|| input.to_string())
}

#[wasm_bindgen]
pub fn mccune_reischauer_romanization_to_hangeul(input: &str) -> String {
    hangeul_conversion::mr::parse(input)
        .map(|blocks| blocks.iter().map(|b| b.hangeul()).collect())
        .unwrap_or_else(|| input.to_string())
}

#[wasm_bindgen]
pub fn hangeul_to_mccune_reischauer_romanization(input: &str) -> String {
    hangeul_conversion::hangeul::parse(input)
        .map(|blocks| {
            blocks
                .iter()
                .map(|b| b.mccune_reischauer_romanization())
                .collect()
        })
        .unwrap_or_else(|| input.to_string())
}

#[wasm_bindgen]
pub fn rr_to_mr(input: &str) -> String {
    hangeul_conversion::rr::parse(input)
        .map(|blocks| {
            blocks
                .iter()
                .map(|b| b.mccune_reischauer_romanization())
                .collect()
        })
        .unwrap_or_else(|| input.to_string())
}

#[wasm_bindgen]
pub fn mr_to_rr(input: &str) -> String {
    hangeul_conversion::mr::parse(input)
        .map(|blocks| blocks.iter().map(|b| b.revised_romanization()).collect())
        .unwrap_or_else(|| input.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_hanja_and_hangeul_are_preserved() {
        assert_eq!(hanja_to_hangeul("?"), "?");
        assert_eq!(hangeul_to_hanja("?"), "?");
        assert_eq!(hanja_to_hangeul_all_variants("?"), "?");
    }

    #[test]
    fn separates_one_characters_readings_from_the_next_character() {
        assert_eq!(hanja_to_hangeul_all_variants("樂"), "낙/락/악/요");
        assert_eq!(hanja_to_hangeul_all_variants("行金"), "항/행 금/김");
    }
}
