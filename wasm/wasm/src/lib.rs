#[cfg(feature = "textprocessing")]
pub use textprocessing::*;
#[cfg(feature = "tuningplayground")]
pub use tuningplayground::*;

#[cfg(feature = "aoc")]
pub mod aoc;
#[cfg(feature = "bayes")]
pub mod bayes;
#[cfg(feature = "chars")]
pub mod chars;
#[cfg(feature = "glsl")]
pub mod glsl;
pub mod graph;
#[cfg(feature = "pokemon")]
pub mod krabby;
#[cfg(feature = "movies")]
pub mod movies;
#[cfg(feature = "photography")]
pub mod photography;
#[cfg(feature = "polyrhythm")]
pub mod polyrhythm;
pub mod random;
#[cfg(feature = "recursive_ji")]
pub mod recursive_ji;
#[cfg(feature = "trolley")]
pub mod trolley;

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
