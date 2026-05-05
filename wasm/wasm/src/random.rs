use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn random_index(count: u32) -> u32 {
    random_index_core(count)
}

#[wasm_bindgen]
pub fn random_range(min: f64, max: f64) -> f64 {
    if !min.is_finite() || !max.is_finite() || max <= min {
        return min;
    }

    let unit = getrandom::u32().map_or(0.0, |value| value as f64 / (u32::MAX as f64 + 1.0));
    min + unit * (max - min)
}

pub(crate) fn random_index_core(count: u32) -> u32 {
    if count == 0 {
        return 0;
    }

    getrandom::u32().map_or(0, |value| value % count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_empty_random_index_inert() {
        assert_eq!(random_index(0), 0);
    }

    #[test]
    fn handles_invalid_random_ranges_without_entropy() {
        assert_eq!(random_range(4.0, 4.0), 4.0);
        assert_eq!(random_range(5.0, 4.0), 5.0);
        assert!(random_range(f64::NAN, 4.0).is_nan());
    }
}
