#!/usr/bin/env bash

function create_folders {
    for j in $(seq 2015 2024); do 
        for i in $(seq 1 25); do
            day=$(printf "%02d" $i)
            mkdir -p "aoc$j/src/day$day" && touch "aoc$j/src/day$day/input.txt"
        done
    done
}

create_folders
