use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn movie_candidate_is_visible(line: &str) -> bool {
    !line.contains("BREAK") && !has_drop_date(line)
}

fn has_drop_date(line: &str) -> bool {
    let suffix: Vec<char> = line.chars().rev().take(12).collect();
    if suffix.len() != 12 {
        return false;
    }

    suffix[0] == ')'
        && suffix[1].is_ascii_digit()
        && suffix[2].is_ascii_digit()
        && suffix[3] == '-'
        && suffix[4].is_ascii_digit()
        && suffix[5].is_ascii_digit()
        && suffix[6] == '-'
        && suffix[7].is_ascii_digit()
        && suffix[8].is_ascii_digit()
        && suffix[9].is_ascii_digit()
        && suffix[10].is_ascii_digit()
        && suffix[11] == '('
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_break_markers_and_dated_drops() {
        assert!(movie_candidate_is_visible("Tampopo"));
        assert!(movie_candidate_is_visible("Movie (1985)"));
        assert!(!movie_candidate_is_visible("BREAK"));
        assert!(!movie_candidate_is_visible("Tampopo (2026-05-04)"));
    }
}
