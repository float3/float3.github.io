use lazy_static::lazy_static;
use std::collections::HashMap;

fn piano_keymap(
    lower_white: [&'static str; 7],
    lower_black: [&'static str; 5],
    upper_white: [&'static str; 10],
    upper_black: [&'static str; 7],
) -> HashMap<&'static str, i32> {
    let mut m = HashMap::new();

    for (key, note) in lower_white.into_iter().zip([24, 26, 28, 29, 31, 33, 35]) {
        m.insert(key, note);
    }
    for (key, note) in lower_black.into_iter().zip([25, 27, 30, 32, 34]) {
        m.insert(key, note);
    }
    for (key, note) in upper_white
        .into_iter()
        .zip([36, 38, 40, 41, 43, 45, 47, 48, 50, 52])
    {
        m.insert(key, note);
    }
    for (key, note) in upper_black.into_iter().zip([37, 39, 42, 44, 46, 49, 51]) {
        m.insert(key, note);
    }

    m
}

fn us_keymap() -> HashMap<&'static str, i32> {
    piano_keymap(
        ["KeyZ", "KeyX", "KeyC", "KeyV", "KeyB", "KeyN", "KeyM"],
        ["KeyS", "KeyD", "KeyG", "KeyH", "KeyJ"],
        [
            "KeyQ", "KeyW", "KeyE", "KeyR", "KeyT", "KeyY", "KeyU", "KeyI", "KeyO", "KeyP",
        ],
        [
            "Digit2", "Digit3", "Digit5", "Digit6", "Digit7", "Digit9", "Digit0",
        ],
    )
}

fn extended_us_keymap() -> HashMap<&'static str, i32> {
    let mut m = us_keymap();

    m.insert("IntlBackslash", 22);
    m.insert("KeyA", 23);
    m.insert("Comma", 36);
    m.insert("KeyL", 37);
    m.insert("Period", 38);
    m.insert("Semicolon", 39);
    m.insert("Slash", 40);
    m.insert("Quote", 41);
    m.insert("Minus", 53);
    m.insert("BracketLeft", 54);
    m.insert("Equal", 55);
    m.insert("BracketRight", 56);
    m.insert("Backslash", 57);

    m
}

fn qwertz_keymap() -> HashMap<&'static str, i32> {
    piano_keymap(
        ["KeyY", "KeyX", "KeyC", "KeyV", "KeyB", "KeyN", "KeyM"],
        ["KeyS", "KeyD", "KeyG", "KeyH", "KeyJ"],
        [
            "KeyQ", "KeyW", "KeyE", "KeyR", "KeyT", "KeyZ", "KeyU", "KeyI", "KeyO", "KeyP",
        ],
        [
            "Digit2", "Digit3", "Digit5", "Digit6", "Digit7", "Digit9", "Digit0",
        ],
    )
}

fn azerty_keymap() -> HashMap<&'static str, i32> {
    piano_keymap(
        ["KeyW", "KeyX", "KeyC", "KeyV", "KeyB", "KeyN", "Semicolon"],
        ["KeyS", "KeyD", "KeyG", "KeyH", "KeyJ"],
        [
            "KeyA", "KeyZ", "KeyE", "KeyR", "KeyT", "KeyY", "KeyU", "KeyI", "KeyO", "KeyP",
        ],
        [
            "Digit2", "Digit3", "Digit5", "Digit6", "Digit7", "Digit9", "Digit0",
        ],
    )
}

fn linear_keymap() -> HashMap<&'static str, i32> {
    [
        ("KeyZ", 24),
        ("KeyX", 25),
        ("KeyC", 26),
        ("KeyV", 27),
        ("KeyB", 28),
        ("KeyN", 29),
        ("KeyM", 30),
        ("Comma", 31),
        ("Period", 32),
        ("Slash", 33),
        ("KeyA", 34),
        ("KeyS", 35),
        ("KeyD", 36),
        ("KeyF", 37),
        ("KeyG", 38),
        ("KeyH", 39),
        ("KeyJ", 40),
        ("KeyK", 41),
        ("KeyL", 42),
        ("Semicolon", 43),
        ("Quote", 44),
        ("KeyQ", 45),
        ("KeyW", 46),
        ("KeyE", 47),
        ("KeyR", 48),
        ("KeyT", 49),
        ("KeyY", 50),
        ("KeyU", 51),
        ("KeyI", 52),
        ("KeyO", 53),
        ("KeyP", 54),
        ("BracketLeft", 55),
        ("BracketRight", 56),
    ]
    .into_iter()
    .collect()
}

lazy_static! {
    pub static ref US_KEYMAP: HashMap<&'static str, i32> = us_keymap();
    pub static ref US_EXTENDED_KEYMAP: HashMap<&'static str, i32> = extended_us_keymap();
    pub static ref QWERTZ_KEYMAP: HashMap<&'static str, i32> = qwertz_keymap();
    pub static ref GERMAN_KEYMAP: HashMap<&'static str, i32> = qwertz_keymap();
    pub static ref AZERTY_KEYMAP: HashMap<&'static str, i32> = azerty_keymap();
    pub static ref LINEAR_KEYMAP: HashMap<&'static str, i32> = linear_keymap();
}
