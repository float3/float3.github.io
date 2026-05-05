use crate::random::random_index_core;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn trolley_random_index(count: u32) -> u32 {
    if count == 0 {
        return 0;
    }

    random_index_core(count) + 1
}

#[wasm_bindgen]
pub fn trolley_media_src(base_path: &str, index: u32, extension: &str) -> String {
    let separator = if base_path.ends_with('/') { "" } else { "/" };
    let extension = extension.trim_start_matches('.');
    format!("{base_path}{separator}{index:02}.{extension}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_zero_count_random_index_inert() {
        assert_eq!(trolley_random_index(0), 0);
    }

    #[test]
    fn builds_padded_trolley_media_paths() {
        assert_eq!(
            trolley_media_src("/misc/trolley", 7, "jpg"),
            "/misc/trolley/07.jpg"
        );
        assert_eq!(
            trolley_media_src("/misc/trolley/", 12, ".mp4"),
            "/misc/trolley/12.mp4"
        );
    }
}
