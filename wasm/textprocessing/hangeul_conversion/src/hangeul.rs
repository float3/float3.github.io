/// This module handles the decomposition and composition of Hangeul syllables.
/// Hangeul syllables range from U+AC00 (가) to U+D7A3 (힣).

const HANGEUL_BASE: u32 = 0xAC00;
const CHOSEONG_COUNT: u32 = 19;
const JUNGSEONG_COUNT: u32 = 21;
const JONGSEONG_COUNT: u32 = 28;

pub(crate) enum Choseong {}
pub(crate) enum Jungseong {}
pub(crate) enum Jongseong {}

// Lists of Choseong, Jungseong, Jongseong in order, as per the standard Korean Unicode block.
const CHOSEONG_LIST: [char; 19] = [
    'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ',
    'ㅌ', 'ㅍ', 'ㅎ',
];
const JUNGSEONG_LIST: [char; 21] = [
    'ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ', 'ㅞ',
    'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ',
];
const JONGSEONG_LIST: [char; 29] = [
    '\0', 'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ', 'ㄿ', 'ㅀ',
    'ㅁ', 'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
];

const CHOSEONG_LIST_RR: [&str; 19] = [
    "g", "kk", "n", "d", "tt", "r", "m", "b", "pp", "s", "ss", "", "j", "jj", "ch", "k", "t", "p",
    "h",
];
const JUNGSEONG_LIST_RR: [&str; 21] = [
    "a", "ae", "ya", "yae", "eo", "e", "yeo", "ye", "o", "wa", "wae", "oe", "yo", "u", "wo", "we",
    "wi", "yu", "eu", "ui", "i",
];
const JONGSEONG_LIST_RR: [&str; 28] = [
    "\0", "k", "kk", "ks", "n", "nj", "nh", "t", "l", "lk", "lm", "lb", "ls", "lt", "lp", "lh",
    "m", "b", "bs", "s", "ss", "ng", "j", "ch", "k", "t", "p", "h",
];

const CHOSEONG_LIST_MR: [&str; 19] = [
    "k", "kk", "n", "d", "tt", "r", "m", "b", "pp", "s", "ss", "", "ch", "tch", "ch'", "k'", "t'",
    "p'", "h",
];
const JUNGSEONG_LIST_MR: [&str; 21] = [
    "a", "ae", "ya", "yae", "ŏ", "e", "yŏ", "ye", "o", "wa", "wae", "oe", "yo", "u", "wŏ", "we",
    "wi", "yu", "ŭ", "ŭi", "i",
];
const JONGSEONG_LIST_MR: [&str; 28] = [
    "\0", "k", "kk", "ks", "n", "nj", "nh", "t", "l", "lk'", "lm", "lb", "ls", "lt", "lp", "lh",
    "m", "b", "ps", "t", "t", "ng", "t", "ch", "k", "t", "p", "h",
];

/// Decompose a single Hangeul syllable into (choseong, jungseong, jongseong)
pub fn hangeul_to_components(ch: char) -> Option<(char, char, Option<char>)> {
    let code = ch as u32;
    if !(HANGEUL_BASE..=0xD7A3).contains(&code) {
        return None;
    }

    let syllable_index = code - HANGEUL_BASE;
    let jongseong_index = syllable_index % JONGSEONG_COUNT;
    let jungseong_index = ((syllable_index - jongseong_index) / JONGSEONG_COUNT) % JUNGSEONG_COUNT;
    let choseong_index = (((syllable_index - jongseong_index) / JONGSEONG_COUNT) - jungseong_index)
        / JUNGSEONG_COUNT;

    let choseong = CHOSEONG_LIST[choseong_index as usize];
    let jungseong = JUNGSEONG_LIST[jungseong_index as usize];
    let jongseong = if jongseong_index == 0 {
        None
    } else {
        Some(JONGSEONG_LIST[jongseong_index as usize])
    };

    Some((choseong, jungseong, jongseong))
}

/// Compose a syllable from (choseong, jungseong, jongseong)
pub fn components_to_hangeul(
    choseong: char,
    jungseong: char,
    jongseong: Option<char>,
) -> Option<char> {
    let c_index = CHOSEONG_LIST.iter().position(|&c| c == choseong)? as u32;
    let j_index = JUNGSEONG_LIST.iter().position(|&j| j == jungseong)? as u32;
    let jong_index = match jongseong {
        Some(j) => JONGSEONG_LIST.iter().position(|&x| x == j)? as u32,
        None => 0,
    };

    let code = HANGEUL_BASE
        + (c_index * JUNGSEONG_COUNT * JONGSEONG_COUNT)
        + (j_index * JONGSEONG_COUNT)
        + jong_index;
    char::from_u32(code)
}

/// Convert a full Hangeul string into a vector of components for each syllable
pub fn from_hangeul(input: &str) -> Vec<(char, char, Option<char>)> {
    input.chars().filter_map(hangeul_to_components).collect()
}

/// Convert a vector of components into a Hangeul string
pub fn to_hangeul(components: &[(char, char, Option<char>)]) -> String {
    components
        .iter()
        .filter_map(|&(c, j, o)| components_to_hangeul(c, j, o))
        .collect()
}
