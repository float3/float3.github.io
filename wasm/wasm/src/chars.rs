use std::sync::{LazyLock, Mutex};
use wasm_bindgen::prelude::wasm_bindgen;

static HIRAGANA: LazyLock<Mutex<Vec<char>>> = LazyLock::new(|| {
    let mut v: Vec<char> = ('\u{3041}'..='\u{3094}')
        .filter(|&c| {
            c != '\u{3041}'
                && c != '\u{3043}'
                && c != '\u{3045}'
                && c != '\u{3047}'
                && c != '\u{3049}'
        })
        .collect();
    v.push('\u{309f}');
    Mutex::new(v)
});

static KATAKANA: LazyLock<Mutex<Vec<char>>> = LazyLock::new(|| {
    let mut v: Vec<char> = ('\u{30a1}'..='\u{30f4}')
        .filter(|&c| {
            c != '\u{30a1}'
                && c != '\u{30a3}'
                && c != '\u{30a5}'
                && c != '\u{30a7}'
                && c != '\u{30a9}'
        })
        .collect();
    v.extend('\u{30f7}'..='\u{30fa}');
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

    /*
    const CFK_EXTENSION_D_START: char = '\u{2b740}';
    const CFK_EXTENSION_D_END: char = '\u{2b81d}';

    const CFK_EXTENSION_I_START: char = '\u{31640}';
    const CFK_EXTENSION_I_END: char = '\u{318f2}';

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
    // kanji.extend(CFK_EXTENSION_D_START..=CFK_EXTENSION_D_END);
    // kanji.extend(CFK_EXTENSION_I_START..=CFK_EXTENSION_I_END);

    Mutex::new(kanji)
});

static BOPOMOFO: LazyLock<Mutex<Vec<char>>> = LazyLock::new(|| {
    const BOPOMOFO_START: char = '\u{3105}';
    const BOPOMOFO_END: char = '\u{312f}';

    // const BOPOMOFO_EXT_END: char = '\u{31bf}';
    // const BOPOMOFO_EXT_START: char = '\u{31A0}';

    let bopomofo: Vec<char> = (BOPOMOFO_START..=BOPOMOFO_END).collect();
    // bopomofo.extend(BOPOMOFO_EXT_START..=BOPOMOFO_EXT_END);
    Mutex::new(bopomofo)
});

static HANGUL: LazyLock<Mutex<Vec<char>>> = LazyLock::new(|| {
    // const HANGUL_JAMO_START: char = '\u{1100}';
    // const HANGUL_JAMO_END: char = '\u{11FF}';

    const HANGUL_SYLLABLE_START: char = '\u{ac00}';
    const HANGUL_SYLLABLE_END: char = '\u{d7a3}';

    let hangul: Vec<char> = (HANGUL_SYLLABLE_START..=HANGUL_SYLLABLE_END).collect();
    // hangul.extend(HANGUL_SYLLABLE_START..=HANGUL_SYLLABLE_END);
    Mutex::new(hangul)
});

fn random_u32(max: u32) -> u32 {
    let num = getrandom::u32().expect("failed to get random");
    num % max
}

fn random_hiragana(remove: bool) -> char {
    let mut v = HIRAGANA.lock().expect("failed to lock hiragana");
    let idx = random_u32(v.len() as u32) as usize;
    if remove { v.remove(idx) } else { v[idx] }
}

fn random_katakana(remove: bool) -> char {
    let mut v = KATAKANA.lock().expect("failed to lock katakana");
    let idx = random_u32(v.len() as u32) as usize;
    if remove { v.remove(idx) } else { v[idx] }
}

fn random_kanji(remove: bool) -> char {
    let mut v = KANJI.lock().expect("failed to lock kanji");
    let idx = random_u32(v.len() as u32) as usize;
    if remove { v.remove(idx) } else { v[idx] }
}

fn random_bopomofo(remove: bool) -> char {
    let mut v = BOPOMOFO.lock().expect("failed to lock bopomofo");
    let idx = random_u32(v.len() as u32) as usize;
    if remove { v.remove(idx) } else { v[idx] }
}

fn random_hangul(remove: bool) -> char {
    let mut v = HANGUL.lock().expect("failed to lock hangul");
    let idx = random_u32(v.len() as u32) as usize;
    if remove { v.remove(idx) } else { v[idx] }
}

#[wasm_bindgen]
pub fn random_weighted_char(remove: bool) -> char {
    let hiragana_len = HIRAGANA.lock().expect("failed to lock hiragana").len();
    let katakana_len = KATAKANA.lock().expect("failed to lock katakana").len();
    let kanji_len = KANJI.lock().expect("failed to lock kanji").len();
    let bopomofo_len = BOPOMOFO.lock().expect("failed to lock bopomofo").len();
    let hangul_len = HANGUL.lock().expect("failed to lock hangul").len();

    let total = hiragana_len + katakana_len + kanji_len + bopomofo_len + hangul_len;
    assert!(total > 0, "All LUTs are empty");

    let rnd = random_u32(total as u32) as usize;
    let mut offset = rnd;
    if offset < hiragana_len {
        return random_hiragana(remove);
    }
    offset -= hiragana_len;
    if offset < katakana_len {
        return random_katakana(remove);
    }
    offset -= katakana_len;
    if offset < kanji_len {
        return random_kanji(remove);
    }
    offset -= kanji_len;
    if offset < bopomofo_len {
        return random_bopomofo(remove);
    }
    // Must fall within hangul range
    random_hangul(remove)
}

// #[wasm_bindgen]
// pub struct Char;

// #[wasm_bindgen]
// impl Char {
//     #[wasm_bindgen(constructor)]
//     pub fn new() -> Char {
//         Char
//     }

//     pub fn next_char(&self) -> char {
//         random_weighted_char(true)
//     }
// }

// impl Iterator for Char {
//     type Item = char;

//     fn next(&mut self) -> Option<Self::Item> {
//         Some(self.next_char())
//     }
// }
