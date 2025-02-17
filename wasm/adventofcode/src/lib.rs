mod aoc2015;
mod aoc2016;
mod aoc2017;
mod aoc2018;
mod aoc2019;
mod aoc2020;
mod aoc2021;
mod aoc2022;
mod aoc2023;
mod aoc2024;
pub mod shared;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

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

#[cfg(not(target_arch = "wasm32"))]
pub fn retrieve_code(year: u32, day: u32, problem: u8) -> String {
    match year {
        2015 => aoc2015::retrieve_code(day, problem),
        2016 => aoc2016::retrieve_code(day, problem),
        2017 => aoc2017::retrieve_code(day, problem),
        2018 => aoc2018::retrieve_code(day, problem),
        2019 => aoc2019::retrieve_code(day, problem),
        2020 => aoc2020::retrieve_code(day, problem),
        2021 => aoc2021::retrieve_code(day, problem),
        2022 => aoc2022::retrieve_code(day, problem),
        2023 => aoc2023::retrieve_code(day, problem),
        2024 => aoc2024::retrieve_code(day, problem),
        _ => panic!("Year not found: {}", year),
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn retrieve_html(year: u32, day: u32, problem: u8, dark: bool) -> String {
    match year {
        2015 => aoc2015::retrieve_html(day, problem, dark),
        2016 => aoc2016::retrieve_html(day, problem, dark),
        2017 => aoc2017::retrieve_html(day, problem, dark),
        2018 => aoc2018::retrieve_html(day, problem, dark),
        2019 => aoc2019::retrieve_html(day, problem, dark),
        2020 => aoc2020::retrieve_html(day, problem, dark),
        2021 => aoc2021::retrieve_html(day, problem, dark),
        2022 => aoc2022::retrieve_html(day, problem, dark),
        2023 => aoc2023::retrieve_html(day, problem, dark),
        2024 => aoc2024::retrieve_html(day, problem, dark),
        _ => panic!("Year not found: {}", year),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn input(year: u32, day: u32) -> String {
    match year {
        2015 => aoc2015::input(day),
        2016 => aoc2016::input(day),
        2017 => aoc2017::input(day),
        2018 => aoc2018::input(day),
        2019 => aoc2019::input(day),
        2020 => aoc2020::input(day),
        2021 => aoc2021::input(day),
        2022 => aoc2022::input(day),
        2023 => aoc2023::input(day),
        2024 => aoc2024::input(day),
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

#[cfg(not(target_arch = "wasm32"))]
pub fn solve_all() {
    println!("Year 2024:");
    aoc2024::solve_all();
    println!("Year 2023:");
    aoc2023::solve_all();
    println!("Year 2022:");
    aoc2022::solve_all();
    println!("Year 2021:");
    aoc2021::solve_all();
    println!("Year 2020:");
    aoc2020::solve_all();
    println!("Year 2019:");
    aoc2019::solve_all();
    println!("Year 2018:");
    aoc2018::solve_all();
    println!("Year 2017:");
    aoc2017::solve_all();
    println!("Year 2016:");
    aoc2016::solve_all();
    println!("Year 2015:");
    aoc2015::solve_all();
}
