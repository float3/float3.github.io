#[derive(Debug)]
enum Choseong {
    G,
    Kk,
    N,
    D,
    Tt,
    R,
    M,
    B,
    Pp,
    S,
    Ss,
    Ieung,
    J,
    Jj,
    Ch,
    K,
    T,
    P,
    H,
}

impl Choseong {
    /// Attempts to match a token to a Choseong variant and returns its index.
    fn from_str(s: &str) -> Option<(Self, usize)> {
        match s {
            "g" => Some((Choseong::G, 0)),
            "kk" => Some((Choseong::Kk, 1)),
            "n" => Some((Choseong::N, 2)),
            "d" => Some((Choseong::D, 3)),
            "tt" => Some((Choseong::Tt, 4)),
            "r" => Some((Choseong::R, 5)),
            "m" => Some((Choseong::M, 6)),
            "b" => Some((Choseong::B, 7)),
            "pp" => Some((Choseong::Pp, 8)),
            "s" => Some((Choseong::S, 9)),
            "ss" => Some((Choseong::Ss, 10)),
            "ieung" => Some((Choseong::Ieung, 11)),
            "j" => Some((Choseong::J, 12)),
            "jj" => Some((Choseong::Jj, 13)),
            "ch" => Some((Choseong::Ch, 14)),
            "k" => Some((Choseong::K, 15)),
            "t" => Some((Choseong::T, 16)),
            "p" => Some((Choseong::P, 17)),
            "h" => Some((Choseong::H, 18)),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Jungseong {
    A,
    Ae,
    Ya,
    Yae,
    Eo,
    E,
    O,
    Yo,
    U,
    I,
}

impl Jungseong {
    fn from_str(s: &str) -> Option<(Self, usize)> {
        match s {
            "yae" => Some((Jungseong::Yae, 3)),
            "ya" => Some((Jungseong::Ya, 2)),
            "ae" => Some((Jungseong::Ae, 1)),
            "a" => Some((Jungseong::A, 0)),
            "eo" => Some((Jungseong::Eo, 4)),
            "e" => Some((Jungseong::E, 5)),
            "o" => Some((Jungseong::O, 6)),
            "yo" => Some((Jungseong::Yo, 7)),
            "u" => Some((Jungseong::U, 8)),
            "i" => Some((Jungseong::I, 9)),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Jongseong {
    Ng,
    Kk,
    G,
    N,
    D,
    R,
    M,
    B,
}

impl Jongseong {
    /// Returns (variant, index). Index 0 means no final.
    fn from_str(s: &str) -> Option<(Self, usize)> {
        match s {
            "ng" => Some((Jongseong::Ng, 1)),
            "kk" => Some((Jongseong::Kk, 2)),
            "g" => Some((Jongseong::G, 3)),
            "n" => Some((Jongseong::N, 4)),
            "d" => Some((Jongseong::D, 5)),
            "r" => Some((Jongseong::R, 6)),
            "m" => Some((Jongseong::M, 7)),
            "b" => Some((Jongseong::B, 8)),
            _ => None,
        }
    }
}

pub fn roman_to_hangeul(input: &str) -> Option<String> {
    let mut output = String::new();
    let mut s = input;

    while !s.is_empty() {
        // --- Parse Initial (Choseong) ---
        let mut found_initial = None;
        // Try tokens in order of descending length for greedy matching.
        for token in [
            "kk", "tt", "pp", "ss", "jj", "ch", "ieung", "g", "n", "d", "r", "m", "b", "s", "j",
            "k", "t", "p", "h",
        ]
        .iter()
        {
            if s.starts_with(token) {
                if let Some((_, idx)) = Choseong::from_str(token) {
                    s = &s[token.len()..];
                    found_initial = Some(idx);
                    break;
                }
            }
        }
        let ci = found_initial?;

        // --- Parse Medial (Jungseong) ---
        let mut found_medial = None;
        for token in ["yae", "ya", "ae", "eo", "e", "o", "yo", "u", "i", "a"].iter() {
            if s.starts_with(token) {
                if let Some((_, idx)) = Jungseong::from_str(token) {
                    s = &s[token.len()..];
                    found_medial = Some(idx);
                    break;
                }
            }
        }
        let vi = found_medial?;

        // --- Parse Optional Final (Jongseong) ---
        let mut fi = 0; // 0 means no final.
        for token in ["ng", "kk", "g", "n", "d", "r", "m", "b"].iter() {
            if s.starts_with(token) {
                if let Some((_, idx)) = Jongseong::from_str(token) {
                    s = &s[token.len()..];
                    fi = idx;
                    break;
                }
            }
        }

        // Calculate the Unicode code point.
        let syllable = 0xAC00 + ((ci * 21 + vi) * 28 + fi) as u32;
        output.push(std::char::from_u32(syllable)?);
    }
    Some(output)
}

/// These constant arrays define the roman tokens for each component.
/// (Their order must match the indices used above.)
const CHOSEONG_ROMAN: [&str; 19] = [
    "g", "kk", "n", "d", "tt", "r", "m", "b", "pp", "s", "ss", "ieung", "j", "jj", "ch", "k", "t",
    "p", "h",
];
const JUNGSEONG_ROMAN: [&str; 10] = ["a", "ae", "ya", "yae", "eo", "e", "o", "yo", "u", "i"];
/// For Jongseong, index 0 means “no final”.
const JONGSEONG_ROMAN: [&str; 9] = ["", "ng", "kk", "g", "n", "d", "r", "m", "b"];

/// Converts a Hangeul string back to its romanized representation.
///
/// The function decomposes each syllable (which must be in the U+AC00..U+D7A3 block)
/// and uses the inverse mapping. It returns `None` if any syllable contains a medial or final
/// value outside of the supported range (i.e. if it wasn’t generated by `roman_to_hangeul`).
pub fn hangeul_to_roman(input: &str) -> Option<String> {
    let mut output = String::new();

    for ch in input.chars() {
        let code = ch as u32;
        // Ensure the character is a complete Hangul syllable.
        if code < 0xAC00 || code > 0xD7A3 {
            return None;
        }
        let syllable_index = code - 0xAC00;
        let ci = (syllable_index / (21 * 28)) as usize;
        let vi = ((syllable_index % (21 * 28)) / 28) as usize;
        let fi = (syllable_index % 28) as usize;

        // Check that the initial and medial are within our mapping.
        if ci >= CHOSEONG_ROMAN.len() || vi >= JUNGSEONG_ROMAN.len() {
            return None;
        }
        // Our scheme only supports finals in the set {0, 1, ..., 8}.
        if fi != 0 && (fi < 1 || fi > 8) {
            return None;
        }

        output.push_str(CHOSEONG_ROMAN[ci]);
        output.push_str(JUNGSEONG_ROMAN[vi]);
        if fi != 0 {
            output.push_str(JONGSEONG_ROMAN[fi]);
        }
    }
    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Tests for roman_to_hangeul (same as your original tests) ---

    // 1. "ga" -> 가
    #[test]
    fn test_ga() {
        assert_eq!(roman_to_hangeul("ga"), Some("\u{AC00}".to_string()));
    }

    // 2. "gag" -> 0xAC00 + 3 = "\u{AC03}"
    #[test]
    fn test_gag() {
        assert_eq!(roman_to_hangeul("gag"), Some("\u{AC03}".to_string()));
    }

    // 3. "na" -> 나
    #[test]
    fn test_na() {
        assert_eq!(roman_to_hangeul("na"), Some("\u{B098}".to_string()));
    }

    // 4. "ttae" -> 때
    #[test]
    fn test_ttae() {
        assert_eq!(roman_to_hangeul("ttae"), Some("때".to_string()));
    }

    // 5. "ra" -> 라
    #[test]
    fn test_ra() {
        let syllable = char::from_u32(0xAC00 + ((5 * 21) * 28)).unwrap();
        assert_eq!(roman_to_hangeul("ra"), Some(syllable.to_string()));
    }

    // 6. "mab" -> mab
    #[test]
    fn test_mab() {
        let syllable = char::from_u32(0xAC00 + ((6 * 21) * 28 + 8)).unwrap();
        assert_eq!(roman_to_hangeul("mab"), Some(syllable.to_string()));
    }

    // 7. "ssae" -> 싸에
    #[test]
    fn test_ssae() {
        let syllable = char::from_u32(0xAC00 + ((10 * 21 + 1) * 28)).unwrap();
        assert_eq!(roman_to_hangeul("ssae"), Some(syllable.to_string()));
    }

    // 8. "ieungyo" -> 의요 (initial ieung, vowel yo)
    #[test]
    fn test_ieungyo() {
        let syllable = char::from_u32(0xAC00 + ((11 * 21 + 7) * 28)).unwrap();
        assert_eq!(roman_to_hangeul("ieungyo"), Some(syllable.to_string()));
    }

    // 9. "jju" -> 주 (with initial jj and vowel u)
    #[test]
    fn test_jju() {
        let syllable = char::from_u32(0xAC00 + ((13 * 21 + 8) * 28)).unwrap();
        assert_eq!(roman_to_hangeul("jju"), Some(syllable.to_string()));
    }

    // 10. "chang" -> 창
    #[test]
    fn test_chang() {
        let syllable = char::from_u32(0xAC00 + ((14 * 21) * 28 + 1)).unwrap();
        assert_eq!(roman_to_hangeul("chang"), Some(syllable.to_string()));
    }

    // 11. "kt" -> invalid (no valid vowel after k)
    #[test]
    fn test_invalid_kt() {
        assert_eq!(roman_to_hangeul("kt"), None);
    }

    // 12. "h" -> invalid (no vowel)
    #[test]
    fn test_invalid_h() {
        assert_eq!(roman_to_hangeul("h"), None);
    }

    // 13. "gya" -> g:0, ya:2, no final
    #[test]
    fn test_gya() {
        let syllable = char::from_u32(0xAC00 + (2 * 28)).unwrap();
        assert_eq!(roman_to_hangeul("gya"), Some(syllable.to_string()));
    }

    // 14. "kkyae" -> kk:1, yae:3, no final
    #[test]
    fn test_kkyae() {
        let syllable = char::from_u32(0xAC00 + ((1 * 21 + 3) * 28)).unwrap();
        assert_eq!(roman_to_hangeul("kkyae"), Some(syllable.to_string()));
    }

    // 15. "dyo" -> d:3, yo:7, no final
    #[test]
    fn test_dyo() {
        let syllable = char::from_u32(0xAC00 + ((3 * 21 + 7) * 28)).unwrap();
        assert_eq!(roman_to_hangeul("dyo"), Some(syllable.to_string()));
    }

    // 16. "tpi" -> invalid vowel ("p" is not a vowel)
    #[test]
    fn test_invalid_tpi() {
        assert_eq!(roman_to_hangeul("tpi"), None);
    }

    // 17. "pue" -> invalid final ("e" is not a valid final)
    #[test]
    fn test_invalid_pue() {
        assert_eq!(roman_to_hangeul("pue"), None);
    }

    // 18. "hye" -> invalid vowel ("ye" not defined)
    #[test]
    fn test_invalid_hye() {
        assert_eq!(roman_to_hangeul("hye"), None);
    }

    // 19. "ka" -> k:15, a:0, no final
    #[test]
    fn test_ka() {
        let syllable = char::from_u32(0xAC00 + ((15 * 21) * 28)).unwrap();
        assert_eq!(roman_to_hangeul("ka"), Some(syllable.to_string()));
    }

    // 20. "ta" -> t:16, a:0, no final
    #[test]
    fn test_ta() {
        let syllable = char::from_u32(0xAC00 + ((16 * 21) * 28)).unwrap();
        assert_eq!(roman_to_hangeul("ta"), Some(syllable.to_string()));
    }

    // 21. "ppa" -> pp:8, a:0, no final
    #[test]
    fn test_ppa() {
        let syllable = char::from_u32(0xAC00 + ((8 * 21) * 28)).unwrap();
        assert_eq!(roman_to_hangeul("ppa"), Some(syllable.to_string()));
    }

    // 22. "sa" -> s:9, a:0, no final
    #[test]
    fn test_sa() {
        let syllable = char::from_u32(0xAC00 + ((9 * 21) * 28)).unwrap();
        assert_eq!(roman_to_hangeul("sa"), Some(syllable.to_string()));
    }

    // 23. "jjae" -> jj:13, ae:1, no final
    #[test]
    fn test_jjae() {
        let syllable = char::from_u32(0xAC00 + ((13 * 21 + 1) * 28)).unwrap();
        assert_eq!(roman_to_hangeul("jjae"), Some(syllable.to_string()));
    }

    // 24. "chae" -> ch:14, ae:1, no final
    #[test]
    fn test_chae() {
        let syllable = char::from_u32(0xAC00 + ((14 * 21 + 1) * 28)).unwrap();
        assert_eq!(roman_to_hangeul("chae"), Some(syllable.to_string()));
    }

    // 25. "gaja" -> two syllables: "ga" then "ja"
    #[test]
    fn test_gaja() {
        let syllable1 = char::from_u32(0xAC00).unwrap(); // "ga"
        let syllable2 = char::from_u32(0xAC00 + ((12 * 21) * 28)).unwrap(); // "ja"
        let expected = format!("{}{}", syllable1, syllable2);
        assert_eq!(roman_to_hangeul("gaja"), Some(expected));
    }

    // --- Tests for hangeul_to_roman (the reverse conversion) ---

    #[test]
    fn test_hangeul_to_roman_ga() {
        // "ga" -> 가
        let hangul = "\u{AC00}";
        assert_eq!(hangeul_to_roman(hangul), Some("ga".to_string()));
    }

    #[test]
    fn test_hangeul_to_roman_gag() {
        // "gag" -> 가 with final g (index 3)
        let hangul = std::char::from_u32(0xAC00 + 3).unwrap();
        assert_eq!(
            hangeul_to_roman(&hangul.to_string()),
            Some("gag".to_string())
        );
    }

    #[test]
    fn test_hangeul_to_roman_na() {
        let hangul = "\u{B098}"; // 나
        assert_eq!(hangeul_to_roman(hangul), Some("na".to_string()));
    }

    #[test]
    fn test_hangeul_to_roman_ttae() {
        let hangul = roman_to_hangeul("ttae").unwrap();
        assert_eq!(hangeul_to_roman(&hangul), Some("ttae".to_string()));
    }

    #[test]
    fn test_hangeul_to_roman_mab() {
        let hangul = roman_to_hangeul("mab").unwrap();
        assert_eq!(hangeul_to_roman(&hangul), Some("mab".to_string()));
    }

    #[test]
    fn test_hangeul_to_roman_chang() {
        let hangul = roman_to_hangeul("chang").unwrap();
        assert_eq!(hangeul_to_roman(&hangul), Some("chang".to_string()));
    }

    #[test]
    fn test_hangeul_to_roman_gaja() {
        let hangul = roman_to_hangeul("gaja").unwrap();
        assert_eq!(hangeul_to_roman(&hangul), Some("gaja".to_string()));
    }

    // Test an invalid Hangul syllable (one with a medial index outside our supported range)
    #[test]
    fn test_hangeul_to_roman_invalid_medial() {
        // Construct a syllable with initial 0 ("g"), medial index 10 (unsupported), no final.
        let syllable = std::char::from_u32(0xAC00 + (0 * 21 + 10) * 28).unwrap();
        assert_eq!(hangeul_to_roman(&syllable.to_string()), None);
    }

    // Test an invalid Hangul syllable (unsupported final)
    #[test]
    fn test_hangeul_to_roman_invalid_final() {
        // Construct a syllable with initial 0 ("g"), medial 0 ("a"), and final index 9 (unsupported)
        let syllable = std::char::from_u32(0xAC00 + (0 * 21 + 0) * 28 + 9).unwrap();
        assert_eq!(hangeul_to_roman(&syllable.to_string()), None);
    }

    // Test non-Hangeul input.
    #[test]
    fn test_hangeul_to_roman_non_hangeul() {
        assert_eq!(hangeul_to_roman("A"), None);
    }
}
