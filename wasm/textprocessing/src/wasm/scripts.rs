pub fn transliterate_cyrillic(text: String) -> String {
    let mut output = String::new();
    for char in text.chars() {
        match transliterate_cyrillic_char(char) {
            Some(transliteration) => output.push_str(transliteration),
            None => output.push(char),
        }
    }
    output
}

pub fn transliterate_greek(text: String) -> String {
    let mut output = String::new();
    for char in text.chars() {
        if is_combining_mark(char) {
            continue;
        }
        match transliterate_greek_char(char) {
            Some(transliteration) => output.push_str(transliteration),
            None => output.push(char),
        }
    }
    output
}

fn transliterate_cyrillic_char(char: char) -> Option<&'static str> {
    match char {
        '\u{0410}' => Some("A"),
        '\u{0430}' => Some("a"),
        '\u{0411}' => Some("B"),
        '\u{0431}' => Some("b"),
        '\u{0412}' => Some("V"),
        '\u{0432}' => Some("v"),
        '\u{0413}' => Some("G"),
        '\u{0433}' => Some("g"),
        '\u{0414}' => Some("D"),
        '\u{0434}' => Some("d"),
        '\u{0415}' => Some("E"),
        '\u{0435}' => Some("e"),
        '\u{0401}' => Some("Yo"),
        '\u{0451}' => Some("yo"),
        '\u{0416}' => Some("Zh"),
        '\u{0436}' => Some("zh"),
        '\u{0417}' => Some("Z"),
        '\u{0437}' => Some("z"),
        '\u{0418}' => Some("I"),
        '\u{0438}' => Some("i"),
        '\u{0419}' => Some("Y"),
        '\u{0439}' => Some("y"),
        '\u{041A}' => Some("K"),
        '\u{043A}' => Some("k"),
        '\u{041B}' => Some("L"),
        '\u{043B}' => Some("l"),
        '\u{041C}' => Some("M"),
        '\u{043C}' => Some("m"),
        '\u{041D}' => Some("N"),
        '\u{043D}' => Some("n"),
        '\u{041E}' => Some("O"),
        '\u{043E}' => Some("o"),
        '\u{041F}' => Some("P"),
        '\u{043F}' => Some("p"),
        '\u{0420}' => Some("R"),
        '\u{0440}' => Some("r"),
        '\u{0421}' => Some("S"),
        '\u{0441}' => Some("s"),
        '\u{0422}' => Some("T"),
        '\u{0442}' => Some("t"),
        '\u{0423}' => Some("U"),
        '\u{0443}' => Some("u"),
        '\u{0424}' => Some("F"),
        '\u{0444}' => Some("f"),
        '\u{0425}' => Some("Kh"),
        '\u{0445}' => Some("kh"),
        '\u{0426}' => Some("Ts"),
        '\u{0446}' => Some("ts"),
        '\u{0427}' => Some("Ch"),
        '\u{0447}' => Some("ch"),
        '\u{0428}' => Some("Sh"),
        '\u{0448}' => Some("sh"),
        '\u{0429}' => Some("Shch"),
        '\u{0449}' => Some("shch"),
        '\u{042B}' => Some("Y"),
        '\u{044B}' => Some("y"),
        '\u{042D}' => Some("E"),
        '\u{044D}' => Some("e"),
        '\u{042E}' => Some("Yu"),
        '\u{044E}' => Some("yu"),
        '\u{042F}' => Some("Ya"),
        '\u{044F}' => Some("ya"),
        '\u{042A}' | '\u{044A}' | '\u{042C}' | '\u{044C}' => Some(""),
        _ => None,
    }
}

fn transliterate_greek_char(char: char) -> Option<&'static str> {
    match char {
        '\u{0391}' | '\u{0386}' => Some("A"),
        '\u{03B1}' | '\u{03AC}' => Some("a"),
        '\u{0392}' => Some("V"),
        '\u{03B2}' => Some("v"),
        '\u{0393}' => Some("G"),
        '\u{03B3}' => Some("g"),
        '\u{0394}' => Some("D"),
        '\u{03B4}' => Some("d"),
        '\u{0395}' | '\u{0388}' => Some("E"),
        '\u{03B5}' | '\u{03AD}' => Some("e"),
        '\u{0396}' => Some("Z"),
        '\u{03B6}' => Some("z"),
        '\u{0397}' | '\u{0389}' => Some("I"),
        '\u{03B7}' | '\u{03AE}' => Some("i"),
        '\u{0398}' => Some("Th"),
        '\u{03B8}' => Some("th"),
        '\u{0399}' | '\u{038A}' | '\u{03AA}' => Some("I"),
        '\u{03B9}' | '\u{03AF}' | '\u{03CA}' | '\u{0390}' => Some("i"),
        '\u{039A}' => Some("K"),
        '\u{03BA}' => Some("k"),
        '\u{039B}' => Some("L"),
        '\u{03BB}' => Some("l"),
        '\u{039C}' => Some("M"),
        '\u{03BC}' => Some("m"),
        '\u{039D}' => Some("N"),
        '\u{03BD}' => Some("n"),
        '\u{039E}' => Some("X"),
        '\u{03BE}' => Some("x"),
        '\u{039F}' | '\u{038C}' => Some("O"),
        '\u{03BF}' | '\u{03CC}' => Some("o"),
        '\u{03A0}' => Some("P"),
        '\u{03C0}' => Some("p"),
        '\u{03A1}' => Some("R"),
        '\u{03C1}' => Some("r"),
        '\u{03A3}' => Some("S"),
        '\u{03C3}' | '\u{03C2}' => Some("s"),
        '\u{03A4}' => Some("T"),
        '\u{03C4}' => Some("t"),
        '\u{03A5}' | '\u{038E}' | '\u{03AB}' => Some("Y"),
        '\u{03C5}' | '\u{03CD}' | '\u{03CB}' | '\u{03B0}' => Some("y"),
        '\u{03A6}' => Some("F"),
        '\u{03C6}' => Some("f"),
        '\u{03A7}' => Some("Ch"),
        '\u{03C7}' => Some("ch"),
        '\u{03A8}' => Some("Ps"),
        '\u{03C8}' => Some("ps"),
        '\u{03A9}' | '\u{038F}' => Some("O"),
        '\u{03C9}' | '\u{03CE}' => Some("o"),
        _ => None,
    }
}

fn is_combining_mark(char: char) -> bool {
    matches!(char as u32, 0x0300..=0x036F)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transliterates_cyrillic() {
        assert_eq!(
            transliterate_cyrillic("Привет, мир".to_string()),
            "Privet, mir"
        );
    }

    #[test]
    fn transliterates_greek() {
        assert_eq!(
            transliterate_greek("Καλημέρα κόσμε".to_string()),
            "Kalimera kosme"
        );
    }
}
