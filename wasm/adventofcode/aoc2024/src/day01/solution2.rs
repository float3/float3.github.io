use super::solution1::parse_input;

pub fn solve(input: &str) -> String {
    let (left, right) = parse_input(input);
    left.into_iter()
        .map(|x| x * (right.iter().filter(|&&y| y == x).count() as i32))
        .sum::<i32>()
        .to_string()
}
