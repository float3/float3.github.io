//! The worked example shown on each transform card.
//!
//! These used to be produced in the browser, which meant the page ran every
//! transform in the table before a visitor had typed anything — and so had to
//! have every language's dictionary in hand just to draw itself. They are fixed
//! strings run through fixed transforms, so they are computed here instead and
//! written into the bundle as text, and the page now loads no wasm at all until
//! somebody actually uses it.

use crate::wasm::transform_text;

pub struct Example {
    pub id: &'static str,
    pub index: u32,
    pub left: &'static str,
}

pub const EXAMPLES: &[Example] = &[
    Example {
        id: "pinyin-tones",
        index: 20,
        left: "wèi shén me",
    },
    Example {
        id: "pinyin-zhuyin",
        index: 0,
        left: "wèi shén me",
    },
    Example {
        id: "han-trad-simp",
        index: 1,
        left: "為什麼",
    },
    Example {
        id: "hanzi-pinyin",
        index: 4,
        left: "漢字",
    },
    Example {
        id: "hanzi-zhuyin",
        index: 8,
        left: "漢字",
    },
    Example {
        id: "hanzi-pinyin-readings",
        index: 5,
        left: "行",
    },
    Example {
        id: "hanzi-zhuyin-readings",
        index: 9,
        left: "行",
    },
    Example {
        id: "hanzi-tokenize",
        index: 22,
        left: "我愛自然語言處理",
    },
    Example {
        id: "kana",
        index: 2,
        left: "ひらがな",
    },
    Example {
        id: "kana-romaji",
        index: 33,
        left: "ひらがな カタカナ きょう",
    },
    Example {
        id: "hanja-hangeul",
        index: 3,
        left: "在元韓國",
    },
    Example {
        id: "hangeul-rr",
        index: 19,
        left: "재원한국",
    },
    Example {
        id: "hangeul-mr",
        index: 23,
        left: "재원한국",
    },
    Example {
        id: "korean-rr-mr",
        index: 24,
        left: "jaewonhanguk",
    },
    Example {
        id: "roman-numerals",
        index: 7,
        left: "3339",
    },
    Example {
        id: "japanese-number",
        index: 18,
        left: "1234567890",
    },
    Example {
        id: "chinese-number-lower",
        index: 15,
        left: "1234567890",
    },
    Example {
        id: "chinese-number-financial",
        index: 11,
        left: "1234567890",
    },
    Example {
        id: "utf8-hex",
        index: 25,
        left: "hello 世界",
    },
    Example {
        id: "utf8-binary",
        index: 26,
        left: "Hi",
    },
    Example {
        id: "base64",
        index: 27,
        left: "hello 世界",
    },
    Example {
        id: "html-entities",
        index: 28,
        left: "'<span title=\"hill\">& text</span>'",
    },
    Example {
        id: "unicode-codepoints",
        index: 29,
        left: "漢字🙂",
    },
    Example {
        id: "big-endian",
        index: 30,
        left: "305419896",
    },
    Example {
        id: "little-endian",
        index: 31,
        left: "305419896",
    },
    Example {
        id: "byte-order",
        index: 32,
        left: "12 34 56 78",
    },
    Example {
        id: "cyrillic",
        index: 34,
        left: "Привет, мир",
    },
    Example {
        id: "greek",
        index: 35,
        left: "Καλημέρα κόσμε",
    },
];

/// Each example paired with what the transform makes of it.
pub fn rendered() -> Vec<(&'static str, &'static str, String)> {
    EXAMPLES
        .iter()
        .map(|example| {
            (
                example.id,
                example.left,
                transform_text(example.index, true, example.left.to_string()),
            )
        })
        .collect()
}
