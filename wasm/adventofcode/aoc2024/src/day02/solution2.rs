fn is_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let diffs: Vec<i32> = levels.windows(2).map(|pair| pair[1] - pair[0]).collect();

    let first_sign = diffs.iter().find(|&&d| d != 0).map_or(0, |&d| d.signum());

    diffs
        .iter()
        .all(|&d| d.signum() == first_sign && (1..=3).contains(&d.abs()))
}

pub fn solve(input: &str) -> String {
    let safe_count = input
        .trim()
        .lines()
        .map(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .filter_map(|token| token.parse::<i32>().ok())
                .collect();

            let levels: &[i32] = &levels;
            if is_safe(levels) {
                return true;
            }

            for i in 0..levels.len() {
                let mut candidate = Vec::with_capacity(levels.len() - 1);
                candidate.extend_from_slice(&levels[..i]);
                candidate.extend_from_slice(&levels[i + 1..]);

                if is_safe(&candidate) {
                    return true;
                }
            }

            false
        })
        .filter(|&is_safe| is_safe)
        .count();

    safe_count.to_string()
}
