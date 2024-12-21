use super::solution1::is_safe;

pub fn solve(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|t| t.parse().ok())
                .collect::<Vec<i32>>()
        })
        .filter(|levels| {
            is_safe(levels)
                || (0..levels.len()).any(|i| is_safe(&[&levels[..i], &levels[i + 1..]].concat()))
        })
        .count()
        .to_string()
}
