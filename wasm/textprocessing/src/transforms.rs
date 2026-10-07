//! The transform cards: what each is called, which transform it runs, and the
//! worked example it shows.
//!
//! `site generate` writes this table into the bundle, together with each
//! example already run through its transform and the package each transform
//! needs, so the page draws every card without loading any wasm and the
//! TypeScript holds no index or label of its own.

/// One card on the page, converting between its left and right forms.
pub struct Card {
    pub id: &'static str,
    pub group: &'static str,
    pub left_label: &'static str,
    pub right_label: &'static str,
    pub example: &'static str,
    /// The arm of [`crate::wasm::transform_text`] the card runs.
    pub index: u32,
    pub keywords: &'static [&'static str],
}

pub const CARDS: &[Card] = &[
    Card {
        id: "pinyin-tones",
        group: "Chinese",
        left_label: "Pinyin tone marks",
        right_label: "Pinyin tone numbers",
        example: "wèi shén me",
        index: 20,
        keywords: &["mandarin", "romanization"],
    },
    Card {
        id: "pinyin-zhuyin",
        group: "Chinese",
        left_label: "Pinyin",
        right_label: "Zhuyin",
        example: "wèi shén me",
        index: 0,
        keywords: &["bopomofo", "mandarin"],
    },
    Card {
        id: "han-trad-simp",
        group: "Chinese",
        left_label: "Traditional",
        right_label: "Simplified",
        example: "為什麼",
        index: 1,
        keywords: &["hanzi"],
    },
    Card {
        id: "hanzi-pinyin",
        group: "Chinese",
        left_label: "Hanzi",
        right_label: "Pinyin",
        example: "漢字",
        index: 4,
        keywords: &["mandarin", "romanization"],
    },
    Card {
        id: "hanzi-zhuyin",
        group: "Chinese",
        left_label: "Hanzi",
        right_label: "Zhuyin",
        example: "漢字",
        index: 8,
        keywords: &["bopomofo"],
    },
    Card {
        id: "hanzi-pinyin-readings",
        group: "Chinese",
        left_label: "Hanzi",
        right_label: "Pinyin readings",
        example: "行",
        index: 5,
        keywords: &["polyphone", "readings"],
    },
    Card {
        id: "hanzi-zhuyin-readings",
        group: "Chinese",
        left_label: "Hanzi",
        right_label: "Zhuyin readings",
        example: "行",
        index: 9,
        keywords: &["polyphone", "readings", "bopomofo"],
    },
    Card {
        id: "hanzi-tokenize",
        group: "Chinese",
        left_label: "Chinese text",
        right_label: "Tokens",
        example: "我愛自然語言處理",
        index: 22,
        keywords: &["segmentation"],
    },
    Card {
        id: "kana",
        group: "Japanese",
        left_label: "Hiragana",
        right_label: "Katakana",
        example: "ひらがな",
        index: 2,
        keywords: &["kana"],
    },
    Card {
        id: "kana-romaji",
        group: "Japanese",
        left_label: "Kana",
        right_label: "Romaji",
        example: "ひらがな カタカナ きょう",
        index: 33,
        keywords: &["hepburn", "romanization", "hiragana", "katakana"],
    },
    Card {
        id: "hanja-hangeul",
        group: "Korean",
        left_label: "Hanja",
        right_label: "Hangeul",
        example: "在元韓國",
        index: 3,
        keywords: &["hangul"],
    },
    Card {
        id: "hanja-hangeul-readings",
        group: "Korean",
        left_label: "Hanja",
        right_label: "Every Hangeul reading",
        example: "樂",
        index: 6,
        keywords: &["hangul", "readings", "polyphone"],
    },
    Card {
        id: "hangeul-rr",
        group: "Korean",
        left_label: "Hangeul",
        right_label: "Revised Romanization",
        example: "재원한국",
        index: 19,
        keywords: &["hangul", "romanization"],
    },
    Card {
        id: "hangeul-mr",
        group: "Korean",
        left_label: "Hangeul",
        right_label: "McCune-Reischauer",
        example: "재원한국",
        index: 23,
        keywords: &["hangul", "romanization"],
    },
    Card {
        id: "korean-rr-mr",
        group: "Korean",
        left_label: "Revised Romanization",
        right_label: "McCune-Reischauer",
        example: "jaewonhanguk",
        index: 24,
        keywords: &["hangul", "romanization"],
    },
    Card {
        id: "roman-numerals",
        group: "Numbers",
        left_label: "Arabic",
        right_label: "Roman",
        example: "3339",
        index: 7,
        keywords: &[],
    },
    Card {
        id: "japanese-number",
        group: "Numbers",
        left_label: "Arabic",
        right_label: "Japanese",
        example: "1234567890",
        index: 18,
        keywords: &["kanji"],
    },
    Card {
        id: "chinese-number-lower",
        group: "Numbers",
        left_label: "Arabic",
        right_label: "Chinese lowercase",
        example: "1234567890",
        index: 15,
        keywords: &["hanzi"],
    },
    Card {
        id: "chinese-number-financial",
        group: "Numbers",
        left_label: "Arabic",
        right_label: "Chinese financial",
        example: "1234567890",
        index: 11,
        keywords: &["hanzi", "uppercase"],
    },
    Card {
        id: "chinese-number-lower-xiashu",
        group: "Numbers",
        left_label: "Arabic",
        right_label: "Chinese lowercase, 下數 (each unit ten times the last)",
        example: "1234567890",
        index: 14,
        keywords: &["hanzi", "xiashu", "low counting"],
    },
    Card {
        id: "chinese-number-financial-xiashu",
        group: "Numbers",
        left_label: "Arabic",
        right_label: "Chinese financial, 下數 (each unit ten times the last)",
        example: "1234567890",
        index: 10,
        keywords: &["hanzi", "uppercase", "xiashu", "low counting"],
    },
    Card {
        id: "chinese-number-lower-zhongshu",
        group: "Numbers",
        left_label: "Arabic",
        right_label: "Chinese lowercase, 中數 (each unit from 億 up 10⁸ times the last)",
        example: "123456789012345678901234567890",
        index: 16,
        keywords: &["hanzi", "zhongshu", "middle counting", "large numbers"],
    },
    Card {
        id: "chinese-number-financial-zhongshu",
        group: "Numbers",
        left_label: "Arabic",
        right_label: "Chinese financial, 中數 (each unit from 億 up 10⁸ times the last)",
        example: "123456789012345678901234567890",
        index: 12,
        keywords: &[
            "hanzi",
            "uppercase",
            "zhongshu",
            "middle counting",
            "large numbers",
        ],
    },
    Card {
        id: "chinese-number-lower-shangshu",
        group: "Numbers",
        left_label: "Arabic",
        right_label: "Chinese lowercase, 上數 (each unit the square of the last)",
        example: "123456789012345678901234567890",
        index: 17,
        keywords: &["hanzi", "shangshu", "high counting", "large numbers"],
    },
    Card {
        id: "chinese-number-financial-shangshu",
        group: "Numbers",
        left_label: "Arabic",
        right_label: "Chinese financial, 上數 (each unit the square of the last)",
        example: "123456789012345678901234567890",
        index: 13,
        keywords: &[
            "hanzi",
            "uppercase",
            "shangshu",
            "high counting",
            "large numbers",
        ],
    },
    Card {
        id: "utf8-hex",
        group: "Encoding",
        left_label: "Text",
        right_label: "UTF-8 hex bytes",
        example: "hello 世界",
        index: 25,
        keywords: &["bytes"],
    },
    Card {
        id: "utf8-binary",
        group: "Encoding",
        left_label: "Text",
        right_label: "UTF-8 binary bytes",
        example: "Hi",
        index: 26,
        keywords: &["bytes"],
    },
    Card {
        id: "base64",
        group: "Encoding",
        left_label: "Text",
        right_label: "Base64",
        example: "hello 世界",
        index: 27,
        keywords: &[],
    },
    Card {
        id: "url",
        group: "Encoding",
        left_label: "Text",
        right_label: "URL encoded",
        example: "hello world? a=1&b=世界",
        index: 36,
        keywords: &["percent"],
    },
    Card {
        id: "html-entities",
        group: "Encoding",
        left_label: "Text",
        right_label: "HTML entities",
        example: "<span title=\"hill\">& text</span>",
        index: 28,
        keywords: &[],
    },
    Card {
        id: "unicode-codepoints",
        group: "Encoding",
        left_label: "Text",
        right_label: "Unicode code points",
        example: "漢字🙂",
        index: 29,
        keywords: &["unicode"],
    },
    Card {
        id: "big-endian",
        group: "Binary",
        left_label: "Unsigned integer",
        right_label: "Big endian bytes",
        example: "305419896",
        index: 30,
        keywords: &["network order", "hex"],
    },
    Card {
        id: "little-endian",
        group: "Binary",
        left_label: "Unsigned integer",
        right_label: "Little endian bytes",
        example: "305419896",
        index: 31,
        keywords: &["small endian", "hex"],
    },
    Card {
        id: "byte-order",
        group: "Binary",
        left_label: "Big endian bytes",
        right_label: "Little endian bytes",
        example: "12 34 56 78",
        index: 32,
        keywords: &["endianness", "hex"],
    },
    Card {
        id: "cyrillic",
        group: "Scripts",
        left_label: "Cyrillic",
        right_label: "Latin",
        example: "Привет, мир",
        index: 34,
        keywords: &["romanization", "russian"],
    },
    Card {
        id: "greek",
        group: "Scripts",
        left_label: "Greek",
        right_label: "Latin",
        example: "Καλημέρα κόσμε",
        index: 35,
        keywords: &["romanization"],
    },
];

/// A card as the page receives it, with everything the transforms decide filled in.
#[cfg(all(feature = "chinese", feature = "korean"))]
pub struct Rendered {
    pub card: &'static Card,
    pub package: crate::wasm::Package,
    pub reversible: bool,
    pub right_example: String,
}

/// Every card with its example run forwards, the package that answered, and
/// whether there is an arm running it backwards. Needs every language feature
/// on, since a dispatcher compiled out would look like a missing arm.
#[cfg(all(feature = "chinese", feature = "korean"))]
pub fn rendered() -> Result<Vec<Rendered>, String> {
    use crate::wasm::dispatch;

    CARDS
        .iter()
        .map(|card| {
            let (package, right_example) = dispatch(card.index, true, card.example.to_string())
                .map_err(|_| format!("{}: no transform has index {}", card.id, card.index))?;
            if right_example.is_empty() || right_example == card.example {
                return Err(format!(
                    "{}: the example {:?} came back as {right_example:?}",
                    card.id, card.example
                ));
            }
            let reversible = match dispatch(card.index, false, right_example.clone()) {
                Ok((reverse, _)) if reverse != package => {
                    return Err(format!(
                        "{}: the two directions live in different packages",
                        card.id
                    ));
                }
                Ok(_) => true,
                Err(_) => false,
            };
            Ok(Rendered {
                card,
                package,
                reversible,
                right_example,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn card_ids_are_unique() {
        let ids: BTreeSet<_> = CARDS.iter().map(|card| card.id).collect();
        assert_eq!(ids.len(), CARDS.len());
    }

    #[cfg(all(feature = "chinese", feature = "korean"))]
    #[test]
    fn every_card_renders() {
        let rendered = rendered().unwrap();
        let url = rendered
            .iter()
            .find(|rendered| rendered.card.id == "url")
            .unwrap();
        assert!(url.reversible);
        assert_eq!(url.package, crate::wasm::Package::Base);
    }
}
