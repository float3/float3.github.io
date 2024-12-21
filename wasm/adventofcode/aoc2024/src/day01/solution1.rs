pub fn solve(input: &str) -> String {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(x, y)| (x - y).abs())
        .sum::<i32>()
        .to_string()
}

pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once("   ").unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .unzip()
}
