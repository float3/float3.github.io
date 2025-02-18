use crate::{block::Block, consonant::Consonant, vowel::Vowel};

pub fn parse(input: &str) -> Option<Vec<Block>> {
    let mut blocks = Vec::new();
    for ch in input.chars() {
        if ch.is_whitespace() {
            continue;
        }
        let code = ch as u32;
        if !(0xAC00..=0xD7A3).contains(&code) {
            return None;
        }
        let syllable_index = code - 0xAC00;
        let init = (syllable_index / 588) as usize;
        let med = ((syllable_index % 588) / 28) as usize;
        let fin = (syllable_index % 28) as usize;
        // Map indices to our enums using standard order:
        let initial = match init {
            0 => Consonant::Giyeok(true),
            1 => Consonant::SsangGiyeok(true),
            2 => Consonant::Nieun(true),
            3 => Consonant::Digeut(true),
            4 => Consonant::SsangDigeut(true),
            5 => Consonant::Rieul(true),
            6 => Consonant::Mieum(true),
            7 => Consonant::Bieup(true),
            8 => Consonant::SsangBieup(true),
            9 => Consonant::Siot(true),
            10 => Consonant::SsangSiot(true),
            11 => Consonant::Ieung(true),
            12 => Consonant::Jieut(true),
            13 => Consonant::SsangJieut(true),
            14 => Consonant::Chieut(true),
            15 => Consonant::Kieuk(true),
            16 => Consonant::Tieut(true),
            17 => Consonant::Pieup(true),
            18 => Consonant::Hieut(true),
            _ => return None,
        };
        let medial = match med {
            0 => Vowel::A,
            1 => Vowel::Ae,
            2 => Vowel::Ya,
            3 => Vowel::Yae,
            4 => Vowel::Eo,
            5 => Vowel::E,
            6 => Vowel::Yeo,
            7 => Vowel::Ye,
            8 => Vowel::O,
            9 => Vowel::Wa,
            10 => Vowel::Wae,
            11 => Vowel::Oe,
            12 => Vowel::Yo,
            13 => Vowel::U,
            14 => Vowel::Wo,
            15 => Vowel::We,
            16 => Vowel::Wi,
            17 => Vowel::Yu,
            18 => Vowel::Eu,
            19 => Vowel::Ui,
            20 => Vowel::I,
            _ => return None,
        };
        let final_cons = if fin == 0 {
            None
        } else {
            let cons = match fin {
                1 => Consonant::Giyeok(false),
                2 => Consonant::SsangGiyeok(false),
                4 => Consonant::Nieun(false),
                7 => Consonant::Digeut(false),
                8 => Consonant::Rieul(false),
                16 => Consonant::Mieum(false),
                17 => Consonant::Bieup(false),
                19 => Consonant::Siot(false),
                20 => Consonant::SsangSiot(false),
                21 => Consonant::Ieung(false),
                22 => Consonant::Jieut(false),
                23 => Consonant::Chieut(false),
                24 => Consonant::Kieuk(false),
                25 => Consonant::Tieut(false),
                26 => Consonant::Pieup(false),
                27 => Consonant::Hieut(false),
                _ => return None,
            };
            Some(cons)
        };
        blocks.push(Block {
            initial,
            medial,
            r#final: final_cons,
        });
    }
    Some(blocks)
}
