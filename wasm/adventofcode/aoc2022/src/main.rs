use aoc2022::{input, solve};

fn main() {
    (1..=25)
        .flat_map(|day| (1..=2).map(move |problem| (day, problem)))
        .for_each(|(day, problem)| {
            println!(
                "Day {} Problem {}: {}",
                day,
                problem,
                solve(&input(day), day, problem)
            );
        });
}
