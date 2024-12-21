#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[cfg(feature = "wasm")]
#[cfg(feature = "mini-alloc")]
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub(crate) fn main() {
    #[cfg(debug_assertions)]
    log("main");
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    fn debug(s: &str);
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    fn info(s: &str);
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn retrieve_problem(year: u32, day: u32, problem: u8) -> String {
    match year {
        2015 => aoc2015::retrieve_problem(day, problem),
        2016 => aoc2016::retrieve_problem(day, problem),
        2017 => aoc2017::retrieve_problem(day, problem),
        2018 => aoc2018::retrieve_problem(day, problem),
        2019 => aoc2019::retrieve_problem(day, problem),
        2020 => aoc2020::retrieve_problem(day, problem),
        2021 => aoc2021::retrieve_problem(day, problem),
        2022 => aoc2022::retrieve_problem(day, problem),
        2023 => aoc2023::retrieve_problem(day, problem),
        2024 => aoc2024::retrieve_problem(day, problem),
        _ => panic!("Year not found: {}", year),
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn retrieve_solution(year: u32, day: u32, problem: u8) -> String {
    match year {
        2015 => aoc2015::retrieve_solution(day, problem),
        2016 => aoc2016::retrieve_solution(day, problem),
        2017 => aoc2017::retrieve_solution(day, problem),
        2018 => aoc2018::retrieve_solution(day, problem),
        2019 => aoc2019::retrieve_solution(day, problem),
        2020 => aoc2020::retrieve_solution(day, problem),
        2021 => aoc2021::retrieve_solution(day, problem),
        2022 => aoc2022::retrieve_solution(day, problem),
        2023 => aoc2023::retrieve_solution(day, problem),
        2024 => aoc2024::retrieve_solution(day, problem),
        _ => panic!("Year not found: {}", year),
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn solve(input: &str, year: u32, day: u32, problem: u8) -> String {
    match year {
        2015 => aoc2015::solve(input, day, problem),
        2016 => aoc2016::solve(input, day, problem),
        2017 => aoc2017::solve(input, day, problem),
        2018 => aoc2018::solve(input, day, problem),
        2019 => aoc2019::solve(input, day, problem),
        2020 => aoc2020::solve(input, day, problem),
        2021 => aoc2021::solve(input, day, problem),
        2022 => aoc2022::solve(input, day, problem),
        2023 => aoc2023::solve(input, day, problem),
        2024 => aoc2024::solve(input, day, problem),
        _ => panic!("Year not found: {}", year),
    }
}
