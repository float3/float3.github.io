use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn aoc_day_count_for_year(year: u32, default_days: u32) -> u32 {
    if year == 2025 { 12 } else { default_days }
}

#[wasm_bindgen]
pub fn aoc_problem_count_for_day(year: u32, day: u32, default_problems: u32) -> u32 {
    if year == 2025 || day == 25 {
        1
    } else {
        default_problems
    }
}

#[wasm_bindgen]
pub fn aoc_completion_percentage(complete_count: u32, total_problems: u32) -> u32 {
    if total_problems == 0 {
        0
    } else {
        complete_count.saturating_mul(100) / total_problems
    }
}

#[wasm_bindgen]
pub fn aoc_day_status(complete_count: u32, total_problems: u32) -> u8 {
    if complete_count == 0 {
        0
    } else if complete_count >= total_problems {
        2
    } else {
        1
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_partial_2025_calendar() {
        assert_eq!(aoc_day_count_for_year(2025, 25), 12);
        assert_eq!(aoc_day_count_for_year(2024, 25), 25);
        assert_eq!(aoc_problem_count_for_day(2025, 3, 2), 1);
        assert_eq!(aoc_problem_count_for_day(2024, 25, 2), 1);
    }

    #[test]
    fn computes_progress_status() {
        assert_eq!(aoc_completion_percentage(12, 49), 24);
        assert_eq!(aoc_day_status(0, 2), 0);
        assert_eq!(aoc_day_status(1, 2), 1);
        assert_eq!(aoc_day_status(2, 2), 2);
    }
}
