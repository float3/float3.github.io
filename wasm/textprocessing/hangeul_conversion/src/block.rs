use itertools::Itertools;

use crate::{Index, Print, consonant::Consonant, vowel::Vowel};

#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    pub initial: Consonant,
    pub medial: Vowel,
    pub r#final: Option<Consonant>,
}

impl Print for Block {
    fn revised_romanization(&self) -> String {
        format!(
            "{}{}{}",
            self.initial.revised_romanization(),
            self.medial.revised_romanization(),
            self.r#final
                .as_ref()
                .map_or(String::new(), |c| c.revised_romanization())
        )
    }
    fn mccune_reischauer_romanization(&self) -> String {
        format!(
            "{}{}{}",
            self.initial.mccune_reischauer_romanization(),
            self.medial.mccune_reischauer_romanization(),
            self.r#final
                .as_ref()
                .map_or(String::new(), |c| c.mccune_reischauer_romanization())
        )
    }
    fn hangeul(&self) -> String {
        // Compose a syllable from indices using Unicode algorithm.
        let init = self.initial.index();
        let med = self.medial.index();
        let fin = if let Some(c) = self.r#final {
            c.index()
        } else {
            0
        };
        let code = 0xAC00 + (init * 588) + (med * 28) + fin;
        char::from_u32(code as u32)
            .map(|c| c.to_string())
            .unwrap_or_default()
    }
}

pub(crate) fn greedy_match<T: Clone>(
    input: &str,
    tokens: &[(&'static str, T)],
) -> Option<(T, usize)> {
    tokens
        .iter()
        .sorted_by_key(|(k, _)| -(k.len() as isize))
        .find_map(|(k, token)| {
            if input.starts_with(k) {
                Some((token.clone(), k.len()))
            } else {
                None
            }
        })
}
