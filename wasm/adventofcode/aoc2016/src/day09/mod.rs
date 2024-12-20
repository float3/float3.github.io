pub mod solution1;
pub mod solution2;

pub fn retrieve_problem(problem: u8) -> String {
    match problem {
        1 => include_str!("problem1.txt").to_string(),
        2 => include_str!("problem2.txt").to_string(),
        _ => panic!("Problem not found"),
    }
}

pub fn retrieve_solution(problem: u8) -> String {
    match problem {
        1 => include_str!("solution1.rs").to_string(),
        2 => include_str!("solution2.rs").to_string(),
        _ => panic!("Solution not found"),
    }
}

pub fn solve(input: &str, problem: u8) -> Option<String> {
    match problem {
        1 => solution1::solve(input),
        2 => solution2::solve(input),
        _ => panic!("Solution not found"),
    }
}

