use crate::{Index, Print};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Consonant {
    Giyeok(bool),
    SsangGiyeok(bool),
    Nieun(bool),
    Digeut(bool),
    SsangDigeut(bool),
    Rieul(bool),
    Mieum(bool),
    Bieup(bool),
    SsangBieup(bool),
    Siot(bool),
    SsangSiot(bool),
    Ieung(bool),
    Jieut(bool),
    SsangJieut(bool),
    Chieut(bool),
    Kieuk(bool),
    Tieut(bool),
    Pieup(bool),
    Hieut(bool),
}

impl Index for Consonant {
    fn index(&self) -> usize {
        use Consonant::*;
        match *self {
            Giyeok(initial) if !initial => 1,
            SsangGiyeok(initial) if !initial => 2,
            Nieun(initial) if !initial => 4,
            Digeut(initial) if !initial => 7,
            Rieul(initial) if !initial => 8,
            Mieum(initial) if !initial => 16,
            Bieup(initial) if !initial => 17,
            Siot(initial) if !initial => 19,
            SsangSiot(initial) if !initial => 20,
            Ieung(initial) if !initial => 21,
            Jieut(initial) if !initial => 22,
            Chieut(initial) if !initial => 23,
            Kieuk(initial) if !initial => 24,
            Tieut(initial) if !initial => 25,
            Pieup(initial) if !initial => 26,
            Hieut(initial) if !initial => 27,
            Giyeok(initial) if initial => 0,
            SsangGiyeok(initial) if initial => 1,
            Nieun(initial) if initial => 2,
            Digeut(initial) if initial => 3,
            SsangDigeut(initial) if initial => 4,
            Rieul(initial) if initial => 5,
            Mieum(initial) if initial => 6,
            Bieup(initial) if initial => 7,
            SsangBieup(initial) if initial => 8,
            Siot(initial) if initial => 9,
            SsangSiot(initial) if initial => 10,
            Ieung(initial) if initial => 11,
            Jieut(initial) if initial => 12,
            SsangJieut(initial) if initial => 13,
            Chieut(initial) if initial => 14,
            Kieuk(initial) if initial => 15,
            Tieut(initial) if initial => 16,
            Pieup(initial) if initial => 17,
            Hieut(initial) if initial => 18,
            _ => unreachable!(),
        }
    }
}

impl Print for Consonant {
    fn revised_romanization(&self) -> String {
        use Consonant::*;
        // mapping depends on whether in initial (true) or final (false)
        match *self {
            Giyeok(true) => "g",
            Giyeok(false) => "k",
            SsangGiyeok(true) => "kk",
            SsangGiyeok(false) => "k", // finals collapse
            Nieun(_) => "n",
            Digeut(true) => "d",
            Digeut(false) => "t",
            SsangDigeut(true) => "tt",
            SsangDigeut(false) => unreachable!("tensed consonants not allowed in final"),
            Rieul(true) => "r",
            Rieul(false) => "l",
            Mieum(_) => "m",
            Bieup(true) => "b",
            Bieup(false) => "p",
            SsangBieup(true) => "pp",
            SsangBieup(false) => unreachable!("plosives not tensed in final"),
            Siot(true) => "s",
            Siot(false) => "t",
            SsangSiot(true) => "ss",
            SsangSiot(false) => "t",
            Ieung(true) => "", // silent as initial
            Ieung(false) => "ng",
            Jieut(true) => "j",
            Jieut(false) => "t",
            SsangJieut(true) => "jj",
            SsangJieut(false) => unreachable!("tensed consonants not allowed in final"),
            Chieut(true) => "ch",
            Chieut(false) => "t",
            Kieuk(_) => "k",
            Tieut(_) => "t",
            Pieup(_) => "p",
            Hieut(true) => "h",
            Hieut(false) => "t",
        }
        .to_string()
    }

    fn mccune_reischauer_romanization(&self) -> String {
        use Consonant::*;
        match *self {
            Giyeok(_) => "k",
            SsangGiyeok(true) => "kk",
            SsangGiyeok(false) => "k",
            Nieun(_) => "n",
            Digeut(_) => "t",
            SsangDigeut(true) => "tt",
            SsangDigeut(false) => unreachable!("tensed consonants not allowed in final"),
            Rieul(true) => "r",
            Rieul(false) => "l",
            Mieum(_) => "m",
            Bieup(_) => "p",
            SsangBieup(true) => "pp",
            SsangBieup(false) => unreachable!("plosives not tensed in final"),
            Siot(true) => "s",
            Siot(false) => "t",
            SsangSiot(true) => "ss",
            SsangSiot(false) => "t",
            Ieung(true) => "",
            Ieung(false) => "ng",
            Jieut(true) => "ch",
            Jieut(false) => "t",
            SsangJieut(true) => "tch",
            SsangJieut(false) => unreachable!("tensed consonants not allowed in final"),
            Chieut(true) => "ch'",
            Chieut(false) => "t",
            Kieuk(true) => "k'",
            Kieuk(false) => "k",
            Tieut(true) => "t'",
            Tieut(false) => "t",
            Pieup(true) => "p'",
            Pieup(false) => "p",
            Hieut(true) => "h",
            Hieut(false) => "",
        }
        .to_string()
    }
    fn hangeul(&self) -> String {
        // For individual jamos (not composed into a syllable)
        use Consonant::*;
        let s = match *self {
            Giyeok(_) => "ㄱ",
            SsangGiyeok(_) => "ㄲ",
            Nieun(_) => "ㄴ",
            Digeut(_) => "ㄷ",
            SsangDigeut(_) => "ㄸ",
            Rieul(_) => "ㄹ",
            Mieum(_) => "ㅁ",
            Bieup(_) => "ㅂ",
            SsangBieup(_) => "ㅃ",
            Siot(_) => "ㅅ",
            SsangSiot(_) => "ㅆ",
            Ieung(_) => "ㅇ",
            Jieut(_) => "ㅈ",
            SsangJieut(_) => "ㅉ",
            Chieut(_) => "ㅊ",
            Kieuk(_) => "ㅋ",
            Tieut(_) => "ㅌ",
            Pieup(_) => "ㅍ",
            Hieut(_) => "ㅎ",
        };
        s.to_string()
    }
}
