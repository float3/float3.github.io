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
mod aoc2025;
pub(crate) mod shared;

/// The palette the highlighted solutions refer to by class, written by
/// `build.rs` alongside them. Sixteen colours in two themes, rather than a
/// second coloured copy of every solution.
#[cfg(target_arch = "wasm32")]
pub const HIGHLIGHT_CSS: &str = include_str!("shared/highlight.css");

/// The first year Advent of Code ran, and the last one this crate has.
pub const FIRST_YEAR: u32 = 2015;
pub const LAST_YEAR: u32 = 2025;

/// Days in a year's calendar: twenty-five, until 2025 cut it to twelve.
pub fn day_count(year: u32) -> u32 {
    if year >= 2025 { 12 } else { 25 }
}

/// Parts a day's puzzle has: two, except the last day of a calendar and every
/// day of 2025, which have one.
pub fn problem_count(year: u32, day: u32) -> u8 {
    if year >= 2025 || day == 25 { 1 } else { 2 }
}

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
        2025 => aoc2025::retrieve_problem(day, problem),
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
        2025 => aoc2025::retrieve_code(day, problem),
        _ => panic!("Year not found: {}", year),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn retrieve_html(year: u32, day: u32, problem: u8) -> String {
    match year {
        2015 => aoc2015::retrieve_html(day, problem),
        2016 => aoc2016::retrieve_html(day, problem),
        2017 => aoc2017::retrieve_html(day, problem),
        2018 => aoc2018::retrieve_html(day, problem),
        2019 => aoc2019::retrieve_html(day, problem),
        2020 => aoc2020::retrieve_html(day, problem),
        2021 => aoc2021::retrieve_html(day, problem),
        2022 => aoc2022::retrieve_html(day, problem),
        2023 => aoc2023::retrieve_html(day, problem),
        2024 => aoc2024::retrieve_html(day, problem),
        2025 => aoc2025::retrieve_html(day, problem),
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
        2025 => aoc2025::input(day),
        _ => panic!("Year not found: {}", year),
    }
}

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
        2025 => aoc2025::solve(input, day, problem),
        _ => panic!("Year not found: {}", year),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn solve_all() {
    println!("Year 2025:");
    aoc2025::solve_all();
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

#[cfg(test)]
mod calendar_tests {
    use super::*;

    #[test]
    fn every_problem_in_the_calendar_has_a_solution_file() {
        for year in FIRST_YEAR..=LAST_YEAR {
            for day in 1..=day_count(year) {
                for problem in 1..=problem_count(year, day) {
                    assert!(
                        !retrieve_code(year, day, problem).is_empty(),
                        "{year} day {day} part {problem}"
                    );
                }
            }
        }
    }
}
