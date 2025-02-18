use crate::{Index, Print};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Vowel {
    A,
    Ae,
    Ya,
    Yae,
    Eo,
    E,
    Yeo,
    Ye,
    O,
    Wa,
    Wae,
    Oe,
    Yo,
    U,
    Wo,
    We,
    Wi,
    Yu,
    Eu,
    Ui,
    I,
}

impl Index for Vowel {
    fn index(&self) -> usize {
        use Vowel::*;
        match *self {
            A => 0,
            Ae => 1,
            Ya => 2,
            Yae => 3,
            Eo => 4,
            E => 5,
            Yeo => 6,
            Ye => 7,
            O => 8,
            Wa => 9,
            Wae => 10,
            Oe => 11,
            Yo => 12,
            U => 13,
            Wo => 14,
            We => 15,
            Wi => 16,
            Yu => 17,
            Eu => 18,
            Ui => 19,
            I => 20,
        }
    }
}

impl Print for Vowel {
    fn revised_romanization(&self) -> String {
        use Vowel::*;
        let s = match self {
            A => "a",
            Ae => "ae",
            Ya => "ya",
            Yae => "yae",
            Eo => "eo",
            E => "e",
            Yeo => "yeo",
            Ye => "ye",
            O => "o",
            Wa => "wa",
            Wae => "wae",
            Oe => "oe",
            Yo => "yo",
            U => "u",
            Wo => "wo",
            We => "we",
            Wi => "wi",
            Yu => "yu",
            Eu => "eu",
            Ui => "ui",
            I => "i",
        };
        s.to_string()
    }

    fn mccune_reischauer_romanization(&self) -> String {
        // MR vowels: use breves for ㅓ, ㅕ, ㅜ, ㅡ etc.
        use Vowel::*;
        let s = match self {
            A => "a",
            Ae => "ae",
            Ya => "ya",
            Yae => "yae",
            Eo => "ŏ",
            E => "e",
            Yeo => "yŏ",
            Ye => "ye",
            O => "o",
            Wa => "wa",
            Wae => "wae",
            Oe => "oe",
            Yo => "yo",
            U => "u",
            Wo => "wŏ",
            We => "we",
            Wi => "wi",
            Yu => "yu",
            Eu => "ŭ",
            Ui => "ŭi",
            I => "i",
        };
        s.to_string()
    }

    fn hangeul(&self) -> String {
        // Return the individual vowel jamo.
        use Vowel::*;
        let s = match self {
            A => "ㅏ",
            Ae => "ㅐ",
            Ya => "ㅑ",
            Yae => "ㅒ",
            Eo => "ㅓ",
            E => "ㅔ",
            Yeo => "ㅕ",
            Ye => "ㅖ",
            O => "ㅗ",
            Wa => "ㅘ",
            Wae => "ㅙ",
            Oe => "ㅚ",
            Yo => "ㅛ",
            U => "ㅜ",
            Wo => "ㅝ",
            We => "ㅞ",
            Wi => "ㅟ",
            Yu => "ㅠ",
            Eu => "ㅡ",
            Ui => "ㅢ",
            I => "ㅣ",
        };
        s.to_string()
    }
}
