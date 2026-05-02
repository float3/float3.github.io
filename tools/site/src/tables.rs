use std::fs;

use crate::{Result, SiteError};

pub(crate) fn align(args: &[String]) -> Result<()> {
    if args.len() != 3 || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        if args.len() == 3 {
            return Ok(());
        }
        return Err(Box::new(SiteError::new(
            "align-tables requires LEFT RIGHT SEP",
        )));
    }

    let left_lines = read_lines(&args[0])?;
    let right_lines = read_lines(&args[1])?;

    if left_lines.len() != right_lines.len() {
        return Err(Box::new(SiteError::new("number of lines must be the same")));
    }

    for (left, right) in left_lines.iter().zip(right_lines.iter()) {
        println!("{} {} {}", left.trim(), args[2], right.trim());
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
