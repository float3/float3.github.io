use glsl2hlsl::{get_files, get_image_files, make_shader, ShaderType};
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[cfg(feature = "wasm")]
#[cfg(feature = "mini-alloc")]
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

#[cfg(feature = "wasm")]
#[cfg(feature = "console_error_panic_hook")]
pub(crate) fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
extern "C" {
    fn download_file(name: &str, contents: &str);
    fn download_image(name: &str, contents: &str);
    fn reset();
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn transpile(input: String, extract_props: bool, raymarch: bool) -> String {
    glsl2hlsl::transpile(
        input,
        extract_props,
        raymarch,
        ShaderType::MainImage("main".to_string(), None, vec![]),
    )
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn download(json: String, extract_props: bool, raymarch: bool) {
    let shader = make_shader(&json).unwrap();
    let files = get_files(&shader, extract_props, raymarch);
    let images = get_image_files(&shader);
    reset();
    for f in files.iter() {
        download_file(&f.name, &f.contents);
    }
    for f in images.iter() {
        download_image(&f.name, &f.contents);
    }
}
