pub fn solve(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|token| token.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .filter(|levels| is_safe(levels))
        .count()
        .to_string()
}

pub fn is_safe(levels: &[i32]) -> bool {
    let diffs: Vec<_> = levels.windows(2).map(|w| w[1] - w[0]).collect();
    if diffs.is_empty() {
        true
    } else {
        let sign = diffs.iter().find(|&&d| d != 0).map_or(0, |&d| d.signum());
        diffs
            .iter()
            .all(|&d| d.signum() == sign && (1..=3).contains(&d.abs()))
    }
}
