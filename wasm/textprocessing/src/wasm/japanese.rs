use japanese::converter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert_hiragana_to_katakana(text: String) -> String {
    converter::convert_hiragana_to_katakana_string(&text)
}

#[wasm_bindgen]
pub fn convert_katakana_to_hiragana(text: String) -> String {
    converter::convert_katakana_to_hiragana_string(&text)
}

pub fn kana_to_romaji(text: String) -> String {
    let kana = text
        .chars()
        .map(|char| {
            let code = char as u32;
            if (0x30A1..=0x30F6).contains(&code) {
                char::from_u32(code - 0x60).unwrap_or(char)
            } else {
                char
            }
        })
        .collect::<Vec<_>>();

    let mut output = String::new();
    let mut index = 0;
    while index < kana.len() {
        let char = kana[index];
        if char == '\u{3063}' {
            let next = kana
                .get(index + 1)
                .and_then(|next| kana_single(*next))
                .or_else(|| {
                    kana.get(index + 1).and_then(|first| {
                        kana.get(index + 2)
                            .and_then(|second| kana_digraph(*first, *second))
                    })
                });
            if let Some(next) = next {
                if let Some(first) = next.chars().next().filter(|char| is_ascii_consonant(*char)) {
                    output.push(first);
                }
            }
            index += 1;
            continue;
        }

        if char == '\u{30FC}' {
            if let Some(vowel) = output
                .chars()
                .last()
                .filter(|char| matches!(char, 'a' | 'e' | 'i' | 'o' | 'u'))
            {
                output.push(vowel);
            } else {
                output.push('-');
            }
            index += 1;
            continue;
        }

        if let (Some(first), Some(second)) = (kana.get(index), kana.get(index + 1)) {
            if let Some(digraph) = kana_digraph(*first, *second) {
                output.push_str(digraph);
                index += 2;
                continue;
            }
        }

        if let Some(single) = kana_single(char) {
            output.push_str(single);
        } else {
            output.push(char);
        }
        index += 1;
    }

    output
}

fn is_ascii_consonant(char: char) -> bool {
    matches!(
        char,
        'b' | 'c'
            | 'd'
            | 'f'
            | 'g'
            | 'h'
            | 'j'
            | 'k'
            | 'l'
            | 'm'
            | 'n'
            | 'p'
            | 'q'
            | 'r'
            | 's'
            | 't'
            | 'v'
            | 'w'
            | 'x'
            | 'y'
            | 'z'
    )
}

fn kana_digraph(first: char, second: char) -> Option<&'static str> {
    match (first, second) {
        ('\u{304D}', '\u{3083}') => Some("kya"),
        ('\u{304D}', '\u{3085}') => Some("kyu"),
        ('\u{304D}', '\u{3087}') => Some("kyo"),
        ('\u{3057}', '\u{3083}') => Some("sha"),
        ('\u{3057}', '\u{3085}') => Some("shu"),
        ('\u{3057}', '\u{3087}') => Some("sho"),
        ('\u{3061}', '\u{3083}') => Some("cha"),
        ('\u{3061}', '\u{3085}') => Some("chu"),
        ('\u{3061}', '\u{3087}') => Some("cho"),
        ('\u{306B}', '\u{3083}') => Some("nya"),
        ('\u{306B}', '\u{3085}') => Some("nyu"),
        ('\u{306B}', '\u{3087}') => Some("nyo"),
        ('\u{3072}', '\u{3083}') => Some("hya"),
        ('\u{3072}', '\u{3085}') => Some("hyu"),
        ('\u{3072}', '\u{3087}') => Some("hyo"),
        ('\u{307F}', '\u{3083}') => Some("mya"),
        ('\u{307F}', '\u{3085}') => Some("myu"),
        ('\u{307F}', '\u{3087}') => Some("myo"),
        ('\u{308A}', '\u{3083}') => Some("rya"),
        ('\u{308A}', '\u{3085}') => Some("ryu"),
        ('\u{308A}', '\u{3087}') => Some("ryo"),
        ('\u{304E}', '\u{3083}') => Some("gya"),
        ('\u{304E}', '\u{3085}') => Some("gyu"),
        ('\u{304E}', '\u{3087}') => Some("gyo"),
        ('\u{3058}', '\u{3083}') => Some("ja"),
        ('\u{3058}', '\u{3085}') => Some("ju"),
        ('\u{3058}', '\u{3087}') => Some("jo"),
        ('\u{3073}', '\u{3083}') => Some("bya"),
        ('\u{3073}', '\u{3085}') => Some("byu"),
        ('\u{3073}', '\u{3087}') => Some("byo"),
        ('\u{3074}', '\u{3083}') => Some("pya"),
        ('\u{3074}', '\u{3085}') => Some("pyu"),
        ('\u{3074}', '\u{3087}') => Some("pyo"),
        _ => None,
    }
}

fn kana_single(char: char) -> Option<&'static str> {
    match char {
        '\u{3042}' => Some("a"),
        '\u{3044}' => Some("i"),
        '\u{3046}' => Some("u"),
        '\u{3048}' => Some("e"),
        '\u{304A}' => Some("o"),
        '\u{304B}' => Some("ka"),
        '\u{304D}' => Some("ki"),
        '\u{304F}' => Some("ku"),
        '\u{3051}' => Some("ke"),
        '\u{3053}' => Some("ko"),
        '\u{3055}' => Some("sa"),
        '\u{3057}' => Some("shi"),
        '\u{3059}' => Some("su"),
        '\u{305B}' => Some("se"),
        '\u{305D}' => Some("so"),
        '\u{305F}' => Some("ta"),
        '\u{3061}' => Some("chi"),
        '\u{3064}' => Some("tsu"),
        '\u{3066}' => Some("te"),
        '\u{3068}' => Some("to"),
        '\u{306A}' => Some("na"),
        '\u{306B}' => Some("ni"),
        '\u{306C}' => Some("nu"),
        '\u{306D}' => Some("ne"),
        '\u{306E}' => Some("no"),
        '\u{306F}' => Some("ha"),
        '\u{3072}' => Some("hi"),
        '\u{3075}' => Some("fu"),
        '\u{3078}' => Some("he"),
        '\u{307B}' => Some("ho"),
        '\u{307E}' => Some("ma"),
        '\u{307F}' => Some("mi"),
        '\u{3080}' => Some("mu"),
        '\u{3081}' => Some("me"),
        '\u{3082}' => Some("mo"),
        '\u{3084}' => Some("ya"),
        '\u{3086}' => Some("yu"),
        '\u{3088}' => Some("yo"),
        '\u{3089}' => Some("ra"),
        '\u{308A}' => Some("ri"),
        '\u{308B}' => Some("ru"),
        '\u{308C}' => Some("re"),
        '\u{308D}' => Some("ro"),
        '\u{308F}' => Some("wa"),
        '\u{3092}' => Some("o"),
        '\u{3093}' => Some("n"),
        '\u{304C}' => Some("ga"),
        '\u{304E}' => Some("gi"),
        '\u{3050}' => Some("gu"),
        '\u{3052}' => Some("ge"),
        '\u{3054}' => Some("go"),
        '\u{3056}' => Some("za"),
        '\u{3058}' => Some("ji"),
        '\u{305A}' => Some("zu"),
        '\u{305C}' => Some("ze"),
        '\u{305E}' => Some("zo"),
        '\u{3060}' => Some("da"),
        '\u{3062}' => Some("ji"),
        '\u{3065}' => Some("zu"),
        '\u{3067}' => Some("de"),
        '\u{3069}' => Some("do"),
        '\u{3070}' => Some("ba"),
        '\u{3073}' => Some("bi"),
        '\u{3076}' => Some("bu"),
        '\u{3079}' => Some("be"),
        '\u{307C}' => Some("bo"),
        '\u{3071}' => Some("pa"),
        '\u{3074}' => Some("pi"),
        '\u{3077}' => Some("pu"),
        '\u{307A}' => Some("pe"),
        '\u{307D}' => Some("po"),
        '\u{3094}' => Some("vu"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn romanizes_kana() {
        assert_eq!(
            kana_to_romaji("ひらがな カタカナ きょう".to_string()),
            "hiragana katakana kyou"
        );
    }
}
