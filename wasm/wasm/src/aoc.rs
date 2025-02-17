use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn retrieve_problem(year: u32, day: u32, problem: u8) -> String {
    adventofcode::retrieve_problem(year, day, problem)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn retrieve_html(year: u32, day: u32, problem: u8, dark: bool) -> String {
    adventofcode::retrieve_html(year, day, problem, dark)
}

#[wasm_bindgen]
pub fn solve(input: &str, year: u32, day: u32, problem: u8) -> String {
    adventofcode::solve(input, year, day, problem)
}
