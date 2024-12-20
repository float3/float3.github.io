#!/usr/bin/env bash

function create_folders {
    for i in $(seq 1 25); do
        mkdir "day$i"
        touch "day$i/solution1.rs"
        touch "day$i/solution2.rs"
        touch "day$i/problem1.txt"
        touch "day$i/problem2.txt"

        cat << 'EOF' > "day$i/mod.rs"
pub mod solution1;
pub mod solution2;

fn problem1() -> String {
    include_str!("problem1.txt");
}

fn problem2() -> String {
    include_str!("problem2.txt")
}

fn solution1() -> String {
    include_str!("solution1.rs")
}

fn solution2() -> String {
    include_str!("solution2.rs")
}
EOF
    done
}

create_folders
