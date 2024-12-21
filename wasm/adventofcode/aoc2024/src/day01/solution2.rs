pub fn solve(input: &str) -> String {
    let (left, right): (Vec<i32>, Vec<i32>) = input
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once("   ").unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .unzip();

    left.into_iter()
        .map(|x| x * (right.iter().filter(|&&y| y == x).count() as i32))
        .sum::<i32>()
        .to_string()
}
