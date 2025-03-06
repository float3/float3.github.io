use krabby::{Generations, PokemonOptions, RandomOptions, random_pokemon};
use std::collections::HashSet;
use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn random_pokemon_wasm() -> String {
    let options = RandomOptions {
        generations: Generations::from_str("1").unwrap(),
        no_mega: false,
        no_gmax: false,
        no_regional: false,
        no_variant: false,
        common: PokemonOptions {
            info: false,
            shiny: false,
            no_title: false,
            padding_left: 0,
        },
    };
    random_pokemon(&options).unwrap()
}

#[wasm_bindgen]
pub fn random_n_pokemon(n: u8, padding: usize) -> String {
    let options = RandomOptions {
        generations: Generations::from_str("1").unwrap(),
        no_mega: false,
        no_gmax: false,
        no_regional: false,
        no_variant: false,
        common: PokemonOptions {
            info: false,
            shiny: false,
            no_title: false,
            padding_left: 0,
        },
    };
    let mut seen_names = HashSet::new();
    let mut sprites: Vec<Vec<String>> = Vec::new();

    while sprites.len() < n as usize {
        let pokemon = random_pokemon(&options).unwrap();
        let mut lines = pokemon.lines();
        let name = lines.next().unwrap();
        if seen_names.insert(name.to_string()) {
            sprites.push(lines.map(|l| l.to_string()).collect());
        }
    }

    sprites.sort_by_key(|sprite| std::cmp::Reverse(sprite.len()));

    let max_height = sprites.first().unwrap().len();

    assert_eq!(max_height, sprites.iter().map(|s| s.len()).max().unwrap());

    // prepare output lines
    let mut output_lines = vec![String::new(); max_height];
    let padding = " ".repeat(padding);

    // vertically center and append each sprite
    for sprite in &sprites {
        let h = sprite.len();
        let diff = max_height - h;
        // top offset is half of the difference; no right-padding, so no width-based spacing
        let top_offset = diff / 2;
        for i in 0..max_height {
            if i < top_offset || i >= top_offset + h {
                // out of this sprite's range
                // just append nothing here
            } else {
                output_lines[i].push_str(&sprite[i - top_offset]);
                output_lines[i].push_str(&padding);
            }
        }
    }

    // join the lines into one string
    output_lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fun() {
        let x = random_n_pokemon(5, 3);
        println!("{}", x);
    }
}
