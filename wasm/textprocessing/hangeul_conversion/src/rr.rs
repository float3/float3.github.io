use crate::{block::greedy_match, consonant::Consonant, vowel::Vowel};

use super::*;

// Revised Romanization INITIALS (added "n")
pub(crate) const INITIALS: &[(&str, Consonant)] = &[
    ("g", Consonant::Giyeok(true)),
    ("kk", Consonant::SsangGiyeok(true)),
    ("tt", Consonant::SsangDigeut(true)),
    ("d", Consonant::Digeut(true)),
    ("n", Consonant::Nieun(true)),
    ("r", Consonant::Rieul(true)),
    ("m", Consonant::Mieum(true)),
    ("pp", Consonant::SsangBieup(true)),
    ("b", Consonant::Bieup(true)),
    ("ss", Consonant::SsangSiot(true)),
    ("s", Consonant::Siot(true)),
    ("j", Consonant::Jieut(true)),
    ("jj", Consonant::SsangJieut(true)),
    ("ch", Consonant::Chieut(true)),
    ("k", Consonant::Kieuk(true)),
    ("t", Consonant::Tieut(true)),
    ("p", Consonant::Pieup(true)),
    ("h", Consonant::Hieut(true)),
];

pub(crate) const VOWELS: &[(&str, Vowel)] = &[
    ("a", Vowel::A),
    ("ae", Vowel::Ae),
    ("ya", Vowel::Ya),
    ("yae", Vowel::Yae),
    ("eo", Vowel::Eo),
    ("e", Vowel::E),
    ("yeo", Vowel::Yeo),
    ("ye", Vowel::Ye),
    ("o", Vowel::O),
    ("wa", Vowel::Wa),
    ("wae", Vowel::Wae),
    ("oe", Vowel::Oe),
    ("yo", Vowel::Yo),
    ("u", Vowel::U),
    ("wo", Vowel::Wo),
    ("we", Vowel::We),
    ("wi", Vowel::Wi),
    ("yu", Vowel::Yu),
    ("eu", Vowel::Eu),
    ("ui", Vowel::Ui),
    ("i", Vowel::I),
];

const FINALS: &[(&str, Consonant)] = &[
    ("n", Consonant::Nieun(false)),
    ("ng", Consonant::Ieung(false)),
    ("k", Consonant::Giyeok(false)),
    ("t", Consonant::Digeut(false)),
    ("l", Consonant::Rieul(false)),
    ("m", Consonant::Mieum(false)),
    ("p", Consonant::Bieup(false)),
];

fn parse_rr_recursive(input: &str) -> Option<Vec<Block>> {
    let input = input.trim_start();
    if input.is_empty() {
        return Some(vec![]);
    }
    let mut idx = 0;
    let rest = &input[idx..];
    let (initial, consumed_init) = if let Some((cons, len)) = greedy_match(rest, INITIALS) {
        (cons, len)
    } else {
        (Consonant::Ieung(true), 0)
    };
    idx += consumed_init;
    let rest = &input[idx..];
    let (medial, consumed_v) = greedy_match(rest, VOWELS)?;
    idx += consumed_v;
    let base_idx = idx;
    let remainder = &input[base_idx..];
    // Option 1: no final
    if let Some(mut following) = parse_rr_recursive(remainder) {
        let mut blocks = vec![Block {
            initial,
            medial,
            r#final: None,
        }];
        blocks.append(&mut following);
        return Some(blocks);
    }
    // Option 2: try finals (shorter tokens first)
    let mut candidates: Vec<(&str, Consonant)> = FINALS
        .iter()
        .filter(|(token, _)| remainder.starts_with(*token))
        .cloned()
        .collect();
    candidates.sort_by_key(|(token, _)| token.len());
    for (token, final_cons) in candidates {
        let new_idx = base_idx + token.len();
        let new_remainder = &input[new_idx..];
        if let Some(mut following) = parse_rr_recursive(new_remainder) {
            let mut blocks = vec![Block {
                initial,
                medial,
                r#final: Some(final_cons),
            }];
            blocks.append(&mut following);
            return Some(blocks);
        }
    }
    None
}

pub fn parse(input: &str) -> Option<Vec<Block>> {
    parse_rr_recursive(input)
}
