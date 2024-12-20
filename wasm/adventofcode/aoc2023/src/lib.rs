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
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub fn retrieve_problem(day: u32, problem: u8) -> String {
    match day {
        01 => day01::retrieve_problem(problem),
        02 => day02::retrieve_problem(problem),
        03 => day03::retrieve_problem(problem),
        04 => day04::retrieve_problem(problem),
        05 => day05::retrieve_problem(problem),
        06 => day06::retrieve_problem(problem),
        07 => day07::retrieve_problem(problem),
        08 => day08::retrieve_problem(problem),
        09 => day09::retrieve_problem(problem),
        10 => day10::retrieve_problem(problem),
        11 => day11::retrieve_problem(problem),
        12 => day12::retrieve_problem(problem),
        13 => day13::retrieve_problem(problem),
        14 => day14::retrieve_problem(problem),
        15 => day15::retrieve_problem(problem),
        16 => day16::retrieve_problem(problem),
        17 => day17::retrieve_problem(problem),
        18 => day18::retrieve_problem(problem),
        19 => day19::retrieve_problem(problem),
        20 => day20::retrieve_problem(problem),
        21 => day21::retrieve_problem(problem),
        22 => day22::retrieve_problem(problem),
        23 => day23::retrieve_problem(problem),
        24 => day24::retrieve_problem(problem),
        25 => day25::retrieve_problem(problem),
        _ => panic!("Day not found"),
    }
}

pub fn retrieve_solution(day: u32, solution: u8) -> String {
    match day {
        01 => day01::retrieve_solution(solution),
        02 => day02::retrieve_solution(solution),
        03 => day03::retrieve_solution(solution),
        04 => day04::retrieve_solution(solution),
        05 => day05::retrieve_solution(solution),
        06 => day06::retrieve_solution(solution),
        07 => day07::retrieve_solution(solution),
        08 => day08::retrieve_solution(solution),
        09 => day09::retrieve_solution(solution),
        10 => day10::retrieve_solution(solution),
        11 => day11::retrieve_solution(solution),
        12 => day12::retrieve_solution(solution),
        13 => day13::retrieve_solution(solution),
        14 => day14::retrieve_solution(solution),
        15 => day15::retrieve_solution(solution),
        16 => day16::retrieve_solution(solution),
        17 => day17::retrieve_solution(solution),
        18 => day18::retrieve_solution(solution),
        19 => day19::retrieve_solution(solution),
        20 => day20::retrieve_solution(solution),
        21 => day21::retrieve_solution(solution),
        22 => day22::retrieve_solution(solution),
        23 => day23::retrieve_solution(solution),
        24 => day24::retrieve_solution(solution),
        25 => day25::retrieve_solution(solution),
        _ => panic!("Day not found"),
    }
}

pub fn solve(input: &str, day: u32, problem: u8) -> Option<String> {
    match day {
        01 => day01::solve(input, problem),
        02 => day02::solve(input, problem),
        03 => day03::solve(input, problem),
        04 => day04::solve(input, problem),
        05 => day05::solve(input, problem),
        06 => day06::solve(input, problem),
        07 => day07::solve(input, problem),
        08 => day08::solve(input, problem),
        09 => day09::solve(input, problem),
        10 => day10::solve(input, problem),
        11 => day11::solve(input, problem),
        12 => day12::solve(input, problem),
        13 => day13::solve(input, problem),
        14 => day14::solve(input, problem),
        15 => day15::solve(input, problem),
        16 => day16::solve(input, problem),
        17 => day17::solve(input, problem),
        18 => day18::solve(input, problem),
        19 => day19::solve(input, problem),
        20 => day20::solve(input, problem),
        21 => day21::solve(input, problem),
        22 => day22::solve(input, problem),
        23 => day23::solve(input, problem),
        24 => day24::solve(input, problem),
        25 => day25::solve(input, problem),
        _ => panic!("Day not found"),
    }
}
