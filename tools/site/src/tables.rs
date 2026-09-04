use std::fs;

use crate::{Result, fail};

pub(crate) fn align(args: &[String]) -> Result<()> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        return Ok(());
    }

    let [left, right, separator] = args else {
        print_help();
        return fail("align-tables requires LEFT RIGHT SEP");
    };

    let left_lines = read_lines(left)?;
    let right_lines = read_lines(right)?;

    if left_lines.len() != right_lines.len() {
        return fail("number of lines must be the same");
    }

    for (left, right) in left_lines.iter().zip(&right_lines) {
        println!("{} {separator} {}", left.trim(), right.trim());
    }

    Ok(())
}

fn read_lines(path: &str) -> Result<Vec<String>> {
    Ok(fs::read_to_string(path)?
        .lines()
        .map(str::to_string)
        .collect())
}

fn print_help() {
    println!(
        "\
Usage:
  site align-tables LEFT RIGHT SEPARATOR

Merges files line-by-line as:
  <left line> <separator> <right line>
"
    );
}
