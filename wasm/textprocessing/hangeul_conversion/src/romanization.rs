/// This module defines the RomanizationSystem enum and provides a common interface.
///
/// Assume we have functions that map (choseong, jungseong, jongseong) to romanization strings.
/// For full correctness, these mappings should follow the defined romanization rules strictly.
use crate::hangeul::{from_hangeul, to_hangeul};

#[derive(Debug, Clone, Copy)]
pub enum RomanizationSystem {
    Revised,
    McCuneReischauer,
}

/// These functions return the romanized form of a single initial or final consonant.
fn revised_initial_consonant(c: char) -> str {
    match c {
        'ㄱ' => "g",
        'ㄲ' => "kk",
        'ㄴ' => "n",
        'ㄷ' => "d",
        'ㄸ' => "tt",
        'ㄹ' => "r",
        'ㅁ' => "m",
        'ㅂ' => "b",
        'ㅃ' => "pp",
        'ㅅ' => "s",
        'ㅆ' => "ss",
        'ㅇ' => "",
        'ㅈ' => "j",
        'ㅉ' => "jj",
        'ㅊ' => "ch",
        'ㅋ' => "k",
        'ㅌ' => "t",
        'ㅍ' => "p",
        'ㅎ' => "h",
        _ => panic!(),
    }
}

fn revised_final_consonant(c: char) -> str {
    match c {
        'ㄱ' => "k",
        'ㄲ' => "k",
        'ㄳ' => "ks",
        'ㄴ' => "n",
        'ㄵ' => "nch",
        'ㄶ' => "nh",
        'ㄷ' => "t",
        'ㄹ' => "l",
        'ㄺ' => "lk",
        'ㄻ' => "lm",
        'ㄼ' => "lp",
        'ㄽ' => "ls",
        'ㄾ' => "lt",
        'ㄿ' => "lp",
        'ㅀ' => "lh",
        'ㅁ' => "m",
        'ㅂ' => "p",
        'ㅄ' => "ps",
        'ㅅ' => "t",
        'ㅆ' => "t",
        'ㅇ' => "ng",
        'ㅈ' => "t",
        'ㅊ' => "t",
        'ㅋ' => "k",
        'ㅌ' => "t",
        'ㅍ' => "p",
        'ㅎ' => "h",
        _ => panic!(),
    }
}

fn revised_vowel(j: char) -> str {
    match j {
        'ㅏ' => "a",
        'ㅐ' => "ae",
        'ㅑ' => "ya",
        'ㅒ' => "yae",
        'ㅓ' => "eo",
        'ㅔ' => "e",
        'ㅕ' => "yeo",
        'ㅖ' => "ye",
        'ㅗ' => "o",
        'ㅘ' => "wa",
        'ㅙ' => "wae",
        'ㅚ' => "oe",
        'ㅛ' => "yo",
        'ㅜ' => "u",
        'ㅝ' => "wo",
        'ㅞ' => "we",
        'ㅟ' => "wi",
        'ㅠ' => "yu",
        'ㅡ' => "eu",
        'ㅢ' => "ui",
        'ㅣ' => "i",
        _ => panic!(),
    }
}

fn map_components_to_revised(c: char, j: char, o: Option<char>) -> String {
    let initial = revised_initial_consonant(c);
    let mid = revised_vowel(j);
    let final_roman = if let Some(final_c) = o {
        revised_final_consonant(final_c)
    } else {
        ""
    };

    format!("{}{}{}", initial, mid, final_roman)
}

fn mc_cune_reischauer_initial_consonant(c: char) -> str {
    match c {
        'ㄱ' => "k",
        'ㄲ' => "kk",
        'ㄴ' => "n",
        'ㄷ' => "t",
        'ㄸ' => "tt",
        'ㄹ' => "r",
        'ㅁ' => "m",
        'ㅂ' => "p",
        'ㅃ' => "pp",
        'ㅅ' => "s",
        'ㅆ' => "ss",
        'ㅇ' => "",
        'ㅈ' => "ch",
        'ㅉ' => "tch",
        'ㅊ' => "ch’",
        'ㅋ' => "k’",
        'ㅌ' => "t’",
        'ㅍ' => "p’",
        'ㅎ' => "h",
        _ => "?",
    }
}

fn mc_cune_reischauer_final_consonant(c: char) -> str {
    match c {
        'ㄱ' => "k",
        'ㄲ' => "k",
        'ㄳ' => "ks",
        'ㄴ' => "n",
        'ㄵ' => "nch",
        'ㄶ' => "nh",
        'ㄷ' => "t",
        'ㄹ' => "l",
        'ㄺ' => "lk",
        'ㄻ' => "lm",
        'ㄼ' => "lp",
        'ㄽ' => "ls",
        'ㄾ' => "lt",
        'ㄿ' => "lp",
        'ㅀ' => "lh",
        'ㅁ' => "m",
        'ㅂ' => "p",
        'ㅄ' => "ps",
        'ㅅ' => "t",
        'ㅆ' => "t",
        'ㅇ' => "ng",
        'ㅈ' => "t",
        'ㅊ' => "t",
        'ㅋ' => "k",
        'ㅌ' => "t",
        'ㅍ' => "p",
        'ㅎ' => "h",
        _ => "",
    }
}

fn mc_cune_reischauer_vowel(j: char) -> str {
    match j {
        'ㅏ' => "a",
        'ㅐ' => "ae",
        'ㅑ' => "ya",
        'ㅒ' => "yae",
        'ㅓ' => "ŏ",
        'ㅔ' => "e",
        'ㅕ' => "yŏ",
        'ㅖ' => "ye",
        'ㅗ' => "o",
        'ㅘ' => "wa",
        'ㅙ' => "wae",
        'ㅚ' => "oe",
        'ㅛ' => "yo",
        'ㅜ' => "u",
        'ㅝ' => "wŏ",
        'ㅞ' => "we",
        'ㅟ' => "wi",
        'ㅠ' => "yu",
        'ㅡ' => "ŭ",
        'ㅢ' => "ŭi",
        'ㅣ' => "i",
        _ => "?",
    }
}

fn map_components_to_mc_cune_reischauer(c: char, j: char, o: Option<char>) -> String {
    let initial = mc_cune_reischauer_initial_consonant(c);
    let mid = mc_cune_reischauer_vowel(j);
    let final_roman = if let Some(final_c) = o {
        mc_cune_reischauer_final_consonant(final_c)
    } else {
        ""
    };

    format!("{}{}{}", initial, mid, final_roman)
}

pub fn to_revised_romanization(hangeul_str: &str) -> String {
    let components = from_hangeul(hangeul_str);
    components
        .iter()
        .map(|&(c, j, o)| map_components_to_revised(c, j, o))
        .collect::<Vec<_>>()
        .join("")
}

pub fn to_mc_cune_reischauer_romanization(hangeul_str: &str) -> String {
    let components = from_hangeul(hangeul_str);
    components
        .iter()
        .map(|&(c, j, o)| map_components_to_mc_cune_reischauer(c, j, o))
        .collect::<Vec<_>>()
        .join("")
}

pub fn from_revised_romanization(rr_str: &str) -> String {
    // Implement a parser that consumes rr_str and matches it to hangeul blocks.
    // For now, a stub:
    // This would be non-trivial because you'd have to match substrings like "g", "ga", "geo", "gyeo" etc. to correct blocks.
    // A real implementation would use a trie or a set of rules.
}

pub fn from_mc_cune_reischauer_romanization(alt_str: &str) -> String {
    // Similar to above, but for alternate romanization system
    "가".to_string() // Placeholder
}
