use std::collections::HashSet;
use wasm_bindgen::prelude::*;

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
pub fn roman_to_hangeul(input: &str) -> String {
    hangeul_conversion::roman_to_hangeul(input).unwrap_or("".to_string())
}

#[wasm_bindgen]
pub fn romanize_hangeul(input: &str) -> String {
    hangeul_conversion::hangeul_to_roman(input).unwrap_or("".to_string())
}
