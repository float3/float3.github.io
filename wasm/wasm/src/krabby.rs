use krabby::{Generations, PokemonOptions, RandomOptions, random_pokemon};
use std::collections::HashSet;
use std::str::FromStr;
use unicode_width::UnicodeWidthStr;
use wasm_bindgen::prelude::wasm_bindgen;

struct Sprite {
    lines: Vec<String>,
    line_widths: Vec<usize>,
    width: usize,
}

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
pub fn random_n_pokemon(n: u8, padding: usize) -> Vec<String> {
    let options = RandomOptions {
        generations: Generations::from_str("1-4").unwrap(),
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
    let mut sprites: Vec<Sprite> = Vec::new();

    while sprites.len() < n as usize {
        let pokemon = random_pokemon(&options).unwrap();
        let mut lines = pokemon.lines();
        let name = lines.next().unwrap();
        if seen_names.insert(name.to_string()) {
            let lines = lines.map(|line| line.to_string()).collect::<Vec<_>>();
            let line_widths = lines
                .iter()
                .map(|line| visible_width(line))
                .collect::<Vec<_>>();
            let width = line_widths.iter().copied().max().unwrap_or(0);

            sprites.push(Sprite {
                lines,
                line_widths,
                width,
            });
        }
    }

    let max_height = sprites
        .iter()
        .map(|sprite| sprite.lines.len())
        .max()
        .unwrap_or(0);

    let mut output_lines = vec![String::new(); max_height];
    let padding = " ".repeat(padding);

    for sprite in &sprites {
        let h = sprite.lines.len();
        let diff = max_height - h;
        let top_offset = diff / 2;
        for i in 0..max_height {
            if i < top_offset || i >= top_offset + h {
                output_lines[i].push_str(&" ".repeat(sprite.width));
            } else {
                let line_index = i - top_offset;
                output_lines[i].push_str(&sprite.lines[line_index]);
                output_lines[i].push_str(
                    &" ".repeat(sprite.width.saturating_sub(sprite.line_widths[line_index])),
                );
            }
            output_lines[i].push_str(&padding);
        }
    }

    output_lines
}

fn visible_width(line: &str) -> usize {
    visible_text(line).width()
}

fn visible_text(line: &str) -> String {
    let mut visible = String::new();
    let mut i = 0;

    while i < line.len() {
        let rest = &line[i..];

        if rest.starts_with("<style") {
            if let Some(end) = rest.find("</style>") {
                i += end + "</style>".len();
                continue;
            }
        }

        if rest.starts_with('<') {
            if let Some(end) = rest.find('>') {
                i += end + 1;
                continue;
            }
        }

        if rest.starts_with("&nbsp;") {
            visible.push(' ');
            i += "&nbsp;".len();
            continue;
        } else if rest.starts_with("&lt;") {
            visible.push('<');
            i += "&lt;".len();
            continue;
        } else if rest.starts_with("&gt;") {
            visible.push('>');
            i += "&gt;".len();
            continue;
        } else if rest.starts_with("&amp;") {
            visible.push('&');
            i += "&amp;".len();
            continue;
        }

        let ch = rest.chars().next().unwrap();
        visible.push(ch);
        i += ch.len_utf8();
    }

    visible
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fun() {
        let x = random_n_pokemon(5, 3);
        println!("{:?}", x);
    }

    #[test]
    fn test_visible_width_ignores_html() {
        let line = "<style>body { white-space: pre; }</style> <span style='color:#fff'>▀▀</span>";
        assert_eq!(visible_width(line), 3);
    }
}
