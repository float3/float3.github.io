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
            // For the silent initial ㅇ, some systems use an apostrophe or similar.
            // Here we choose a token "ieung" (or you can adjust as needed):
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
    None,
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
