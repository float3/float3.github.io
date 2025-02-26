use std::sync::{LazyLock, Mutex};

use wasm_bindgen::prelude::wasm_bindgen;

static HIRAGANA: LazyLock<Mutex<Vec<char>>> = LazyLock::new(|| {
    let mut v: Vec<char> = ('\u{3041}'..='\u{3096}').collect();
    v.push('\u{309f}');
    Mutex::new(v)
});

static KATAKANA: LazyLock<Mutex<Vec<char>>> = LazyLock::new(|| {
    let mut v: Vec<char> = ('\u{30a1}'..='\u{30fa}').collect();
    v.push('\u{30ff}');
    Mutex::new(v)
});

// static KANA: LazyLock<Mutex<Vec<char>>> = LazyLock::new(|| {
//     let mut v: Vec<char> = ('\u{31F0}'..='\u{31FF}').collect();
//     v.push('\u{30ff}');
//     Mutex::new(v)
// });

static KANJI: LazyLock<Mutex<Vec<char>>> = LazyLock::new(|| {
    const KANJI_START: char = '\u{4e00}';
    const KANJI_END: char = '\u{9faf}';

    const CFK_EXTENSION_A_START: char = '\u{3400}';
    const CFK_EXTENSION_A_END: char = '\u{4dbf}';

    const CFK_EXTENSION_D_START: char = '\u{2b740}';
    const CFK_EXTENSION_D_END: char = '\u{2b81d}';

    const CFK_EXTENSION_I_START: char = '\u{31640}';
    const CFK_EXTENSION_I_END: char = '\u{318f2}';

    /*
    const CFK_EXTENSION_B_START: char = '\u{20000}';
    const CFK_EXTENSION_B_END: char = '\u{2a6d6}';

    const CFK_EXTENSION_C_START: char = '\u{2a700}';
    const CFK_EXTENSION_C_END: char = '\u{2b734}';

    const CFK_EXTENSION_E_START: char = '\u{2b820}';
    const CFK_EXTENSION_E_END: char = '\u{2cea1}';

    const CFK_EXTENSION_F_START: char = '\u{2ceb0}';
    const CFK_EXTENSION_F_END: char = '\u{2ebe0}';

    const CFK_EXTENSION_G_START: char = '\u{30000}';
    const CFK_EXTENSION_G_END: char = '\u{3134a}';

    const CFK_EXTENSION_H_START: char = '\u{31350}';
    const CFK_EXTENSION_H_END: char = '\u{3163f}';
    */

    let mut kanji: Vec<char> = (KANJI_START..=KANJI_END).collect();
    kanji.extend(CFK_EXTENSION_A_START..=CFK_EXTENSION_A_END);
    kanji.extend(CFK_EXTENSION_D_START..=CFK_EXTENSION_D_END);
    kanji.extend(CFK_EXTENSION_I_START..=CFK_EXTENSION_I_END);

    Mutex::new(kanji)
});

static BOPOMOFO: LazyLock<Mutex<Vec<char>>> = LazyLock::new(|| {
    const BOPOMOFO_START: char = '\u{3105}';
    const BOPOMOFO_END: char = '\u{312f}';

    // const BOPOMOFO_EXT_END: char = '\u{31bf}';
    // const BOPOMOFO_EXT_START: char = '\u{31A0}';

    let mut bopomofo: Vec<char> = (BOPOMOFO_START..=BOPOMOFO_END).collect();
    // bopomofo.extend(BOPOMOFO_EXT_START..=BOPOMOFO_EXT_END);
    Mutex::new(bopomofo)
});

static HANGUL: LazyLock<Mutex<Vec<char>>> = LazyLock::new(|| {
    const HANGUL_JAMO_START: char = '\u{1100}';
    const HANGUL_JAMO_END: char = '\u{11FF}';

    const HANGUL_SYLLABLE_START: char = '\u{ac00}';
    const HANGUL_SYLLABLE_END: char = '\u{d7a3}';

    let mut hangul: Vec<char> = (HANGUL_JAMO_START..=HANGUL_JAMO_END).collect();

    hangul.extend(HANGUL_SYLLABLE_START..=HANGUL_SYLLABLE_END);

    Mutex::new(hangul)
});

fn random_u32(max: u32) -> u32 {
    let num = getrandom::u32().expect("failed to get random");
    num % max
}

fn random_hiragana() -> char {
    let idx = random_u32(HIRAGANA.lock().unwrap().len() as u32) as usize;
    HIRAGANA.lock().unwrap().remove(idx)
}

fn random_katakana() -> char {
    let idx = random_u32(KATAKANA.lock().unwrap().len() as u32) as usize;
    KATAKANA.lock().unwrap().remove(idx)
}

fn random_kanji() -> char {
    let idx = random_u32(KANJI.lock().unwrap().len() as u32) as usize;
    KANJI.lock().unwrap().remove(idx)
}

fn random_bopomofo() -> char {
    let idx = random_u32(BOPOMOFO.lock().unwrap().len() as u32) as usize;
    BOPOMOFO.lock().unwrap().remove(idx)
}

fn random_hangul() -> char {
    let idx = random_u32(HANGUL.lock().unwrap().len() as u32) as usize;
    HANGUL.lock().unwrap().remove(idx)
}

#[wasm_bindgen]
struct Char;

#[wasm_bindgen]
impl Char {
    pub fn new() -> Self {
        Self
    }

    pub fn next_char(&self) -> char {
        match random_u32(5) {
            0 => random_hiragana(),
            1 => random_katakana(),
            2 => random_kanji(),
            3 => random_bopomofo(),
            4 => random_hangul(),
            _ => unreachable!(),
        }
    }
}

impl Iterator for Char {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_char())
    }
}
