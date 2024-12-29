pub mod solution1;
pub mod solution2;

pub fn retrieve_problem(problem: u8) -> String {
    match problem {
        1 => include_str!("problem1.txt").to_string(),
        2 => include_str!("problem2.txt").to_string(),
        _ => panic!("Problem not found"),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn retrieve_code(problem: u8) -> String {
    match problem {
        1 => include_str!("solution1.rs").to_string(),
        2 => include_str!("solution2.rs").to_string(),
        _ => panic!("Solution not found"),
    }
}

pub fn retrieve_html(problem: u8) -> String {
    match problem {
        1 => include_str!("solution1.html").to_string(),
        2 => include_str!("solution2.html").to_string(),
        _ => panic!("Solution not found"),
    }
}

pub fn solve(input: &str, problem: u8) -> String {
    match problem {
        1 => solution1::solve(input),
        2 => solution2::solve(input),
        _ => panic!("Solution not found"),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn input() -> String {
    include_str!("input.txt").to_string()
}
