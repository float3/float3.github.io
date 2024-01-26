use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let first = format!("({}-", args[1]);
    let first = first.as_str();

    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let file_path = current_dir.join("..").join("..").join("content").join("movies.md");

    
    let file = File::open(file_path).unwrap();
    let mut buf_reader = BufReader::new(file.try_clone().unwrap());
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let mut contents = contents
        .split('\n')
        .map(|mut x| {
            if x.contains("- [x]") && x.contains(first) {
                remove_after_first_open_paren_str(x)
            } else {
                x = "";
                x
            }
        })
        .collect::<Vec<&str>>();

    contents.sort();
    contents.dedup();
    println!("{}", contents.join("\n"));
}

fn remove_after_first_open_paren_str(s: &str) -> &str {
    if let Some(pos) = s.rfind('(') {
        &s[..pos]
    } else {
        s
    }
}
