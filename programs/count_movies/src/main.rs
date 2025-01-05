use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let search_str = env::args()
        .nth(1)
        .map(|arg| format!("({}-", arg))
        .expect("Usage: <program> <search_str>");

    let file_path = env::current_dir()
        .expect("Failed to get current directory")
        .join("..")
        .join("..")
        .join("content")
        .join("notes")
        .join("movies.md");

    let contents = File::open(&file_path)
        .and_then(|file| {
            let mut buffer = String::new();
            BufReader::new(file).read_to_string(&mut buffer)?;
            Ok(buffer)
        })
        .expect("Failed to read file");

    let lines = contents
        .lines()
        .filter(|line| line.contains("- [x]") && line.contains(&search_str))
        .map(remove_after_first_open_paren_str)
        .map(ToOwned::to_owned)
        .collect::<BTreeSet<_>>();

    println!(
        "{}",
        lines.into_iter().collect::<Vec<_>>().join("\n").trim()
    );
}

fn remove_after_first_open_paren_str(s: &str) -> &str {
    s.find('(').map(|pos| &s[..pos]).unwrap_or(s)
}
