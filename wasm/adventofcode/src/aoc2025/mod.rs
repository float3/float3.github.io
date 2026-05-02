pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;

pub(super) fn retrieve_problem(day: u32, problem: u8) -> String {
    match day {
        1 => day01::retrieve_problem(problem),
        2 => day02::retrieve_problem(problem),
        3 => day03::retrieve_problem(problem),
        4 => day04::retrieve_problem(problem),
        5 => day05::retrieve_problem(problem),
        6 => day06::retrieve_problem(problem),
        7 => day07::retrieve_problem(problem),
        8 => day08::retrieve_problem(problem),
        9 => day09::retrieve_problem(problem),
        10 => day10::retrieve_problem(problem),
        11 => day11::retrieve_problem(problem),
        12 => day12::retrieve_problem(problem),
        _ => panic!("Day not found"),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn problem_count(day: u32) -> u8 {
    match day {
        1 => day01::problem_count(),
        2 => day02::problem_count(),
        3 => day03::problem_count(),
        4 => day04::problem_count(),
        5 => day05::problem_count(),
        6 => day06::problem_count(),
        7 => day07::problem_count(),
        8 => day08::problem_count(),
        9 => day09::problem_count(),
        10 => day10::problem_count(),
        11 => day11::problem_count(),
        12 => day12::problem_count(),
        _ => panic!("Day not found"),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn retrieve_code(day: u32, problem: u8) -> String {
    match day {
        1 => day01::retrieve_code(problem),
        2 => day02::retrieve_code(problem),
        3 => day03::retrieve_code(problem),
        4 => day04::retrieve_code(problem),
        5 => day05::retrieve_code(problem),
        6 => day06::retrieve_code(problem),
        7 => day07::retrieve_code(problem),
        8 => day08::retrieve_code(problem),
        9 => day09::retrieve_code(problem),
        10 => day10::retrieve_code(problem),
        11 => day11::retrieve_code(problem),
        12 => day12::retrieve_code(problem),
        _ => panic!("Day not found"),
    }
}

#[cfg(target_arch = "wasm32")]
pub(super) fn retrieve_html(day: u32, problem: u8, dark: bool) -> String {
    match day {
        1 => day01::retrieve_html(problem, dark),
        2 => day02::retrieve_html(problem, dark),
        3 => day03::retrieve_html(problem, dark),
        4 => day04::retrieve_html(problem, dark),
        5 => day05::retrieve_html(problem, dark),
        6 => day06::retrieve_html(problem, dark),
        7 => day07::retrieve_html(problem, dark),
        8 => day08::retrieve_html(problem, dark),
        9 => day09::retrieve_html(problem, dark),
        10 => day10::retrieve_html(problem, dark),
        11 => day11::retrieve_html(problem, dark),
        12 => day12::retrieve_html(problem, dark),
        _ => panic!("Day not found"),
    }
}

pub(super) fn solve(input: &str, day: u32, problem: u8) -> String {
    match day {
        1 => day01::solve(input, problem),
        2 => day02::solve(input, problem),
        3 => day03::solve(input, problem),
        4 => day04::solve(input, problem),
        5 => day05::solve(input, problem),
        6 => day06::solve(input, problem),
        7 => day07::solve(input, problem),
        8 => day08::solve(input, problem),
        9 => day09::solve(input, problem),
        10 => day10::solve(input, problem),
        11 => day11::solve(input, problem),
        12 => day12::solve(input, problem),
        _ => panic!("Day not found"),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn input(day: u32) -> String {
    match day {
        1 => day01::input(),
        2 => day02::input(),
        3 => day03::input(),
        4 => day04::input(),
        5 => day05::input(),
        6 => day06::input(),
        7 => day07::input(),
        8 => day08::input(),
        9 => day09::input(),
        10 => day10::input(),
        11 => day11::input(),
        12 => day12::input(),
        _ => panic!("Day not found"),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn solve_all() {
    (1..=12).for_each(|day| {
        println!("  Day {}", day);
        (1..=problem_count(day)).for_each(|problem| {
            println!("    Problem {}: {}", problem, solve(&input(day), day, problem));
        });
    });
}
