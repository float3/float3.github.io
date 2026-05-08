use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn photo_count_label(count: usize) -> String {
    let suffix = if count == 1 { "" } else { "s" };
    format!("{count} photo{suffix}")
}

#[wasm_bindgen]
pub fn photo_caption(title: &str, meta: &str) -> String {
    let title = title.trim();
    let meta = meta.trim();

    match (title.is_empty(), meta.is_empty()) {
        (true, true) => String::new(),
        (false, true) => title.to_string(),
        (true, false) => meta.to_string(),
        (false, false) => format!("{title} - {meta}"),
    }
}

#[wasm_bindgen]
pub fn photo_manifest_entry_is_valid(src: &str, _title: &str) -> bool {
    !src.trim().is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pluralizes_photo_count() {
        assert_eq!(photo_count_label(1), "1 photo");
        assert_eq!(photo_count_label(2), "2 photos");
    }

    #[test]
    fn joins_caption_parts() {
        assert_eq!(photo_caption("frame", ""), "frame");
        assert_eq!(photo_caption("frame", "35mm"), "frame - 35mm");
    }

    #[test]
    fn validates_manifest_entry_shape() {
        assert!(photo_manifest_entry_is_valid("/photo.jpg", "photo"));
        assert!(!photo_manifest_entry_is_valid("", "photo"));
        assert!(photo_manifest_entry_is_valid("/photo.jpg", ""));
    }
}
