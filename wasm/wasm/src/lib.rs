pub use textprocessing::*;
pub use tuningplayground::*;

pub mod aoc;
pub mod chars;
pub mod glsl;
pub mod polyrhythm;

use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[cfg(feature = "mini-alloc")]
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    pub fn debug(s: &str);
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    pub fn warn(s: &str);
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    pub fn info(s: &str);
}

#[wasm_bindgen(start)]
pub(crate) fn main() {
    #[cfg(debug_assertions)]
    log("main");
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
