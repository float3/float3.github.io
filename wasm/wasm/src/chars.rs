use std::sync::{LazyLock, Mutex};
use wasm_bindgen::prelude::wasm_bindgen;

static KANJI: LazyLock<Mutex<Vec<char>>> = LazyLock::new(|| {
    const KANJI_START: char = '\u{4e00}';
    const KANJI_END: char = '\u{9faf}';

    const CFK_EXTENSION_A_START: char = '\u{3400}';
    const CFK_EXTENSION_A_END: char = '\u{4dbf}';

    let mut kanji: Vec<char> = (KANJI_START..=KANJI_END).collect();
    kanji.extend(CFK_EXTENSION_A_START..=CFK_EXTENSION_A_END);

    Mutex::new(kanji)
});

fn random_u32(max: u32) -> u32 {
    let num = getrandom::u32().expect("failed to get random");
    num % max
}

fn random_kanji(remove: bool) -> char {
    let mut v = KANJI.lock().expect("failed to lock kanji");
    let idx = random_u32(v.len() as u32) as usize;
    if remove { v.remove(idx) } else { v[idx] }
}

#[wasm_bindgen]
pub fn random_weighted_char(remove: bool) -> char {
    let kanji_len = KANJI.lock().expect("failed to lock kanji").len();
    assert!(kanji_len > 0, "All LUTs are empty");

    random_kanji(remove)
}
