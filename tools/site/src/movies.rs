//! The movie lists, read out of `content/notes/movies.md` for its page script.
//!
//! `site movies` (part of `site build` and `site generate`) writes
//! [`OUTPUT`]: for every list on the page, each of its task-list lines in
//! order, with the title and year read off it, whether it has been watched,
//! and search links on IMDb and Letterboxd. The page puts the links beside each
//! line and spins its wheel over the ones not yet watched. Search links rather
//! than film pages, because a line carries a title and a year and nothing that
//! names one film.

use crate::{Result, Site};
use serde_json::{Value, json};
use std::fs;

pub(crate) const SOURCE: &str = "content/notes/movies.md";
/// Served beside the page as `/notes/movies.json`. Checked in, like the
/// gallery manifests, because Quartz copies no gitignored file but `js/`; the
/// build rewrites it anyway, so a deploy never serves a stale one.
pub(crate) const OUTPUT: &str = "content/notes/movies.json";

/// The list of films given up on, which the wheel never picks from.
const DROPPED: &str = "Dropped Movies";

/// One task-list line.
#[derive(Debug, PartialEq, Eq)]
struct Movie {
    /// `None` for a line that names no film: a `BREAK` week, or `UNKNOWN`.
    title: Option<String>,
    year: Option<String>,
    /// Ticked, or dated with the day it was watched or dropped.
    watched: bool,
}

impl Movie {
    fn label(&self) -> Option<String> {
        let title = self.title.as_deref()?;
        Some(match &self.year {
            Some(year) => format!("{title} ({year})"),
            None => title.to_string(),
        })
    }
}

/// `text` without a trailing ` (<inner>)` for which `inner` holds, and that
/// inner text.
fn strip_parenthesised(text: &str, inner: impl Fn(&str) -> bool) -> Option<(&str, &str)> {
    let rest = text.strip_suffix(')')?;
    let open = rest.rfind('(')?;
    let value = &rest[open + 1..];
    inner(value).then(|| (rest[..open].trim_end(), value))
}

fn is_date(text: &str) -> bool {
    let bytes = text.as_bytes();
    bytes.len() == 10
        && bytes.iter().enumerate().all(|(index, byte)| match index {
            4 | 7 => *byte == b'-',
            _ => byte.is_ascii_digit(),
        })
}

/// `- [ ] Title (2000)`, `- [x] Title (2000) (2022-09-11)`, and the variations
/// the lists actually hold: no year, `(TBA)` or `()` for one, and `(UNKNOWN)`
/// for the date.
fn parse_line(line: &str) -> Option<Movie> {
    let rest = line.trim_start().strip_prefix("- [")?;
    let (mark, text) = rest.split_once("] ")?;
    let ticked = match mark {
        " " => false,
        "x" | "X" => true,
        _ => return None,
    };

    let mut text = text.trim();
    let mut dated = false;
    if let Some((before, _)) =
        strip_parenthesised(text, |inner| is_date(inner) || inner == "UNKNOWN")
    {
        text = before;
        dated = true;
    }
    let mut year = None;
    if let Some((before, inner)) = strip_parenthesised(text, |inner| {
        inner.is_empty()
            || inner == "TBA"
            || (inner.len() == 4 && inner.bytes().all(|b| b.is_ascii_digit()))
    }) {
        text = before;
        year = (inner.len() == 4).then(|| inner.to_string());
    }

    let title = Some(text.trim())
        .filter(|title| !title.is_empty() && *title != "BREAK" && *title != "UNKNOWN")
        .map(str::to_string);
    Some(Movie {
        title,
        year,
        watched: ticked || dated,
    })
}

/// Everything `encodeURIComponent` leaves alone, kept; everything else
/// percent-encoded as UTF-8.
fn encode(text: &str) -> String {
    text.bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || b"-_.!~*'()".contains(&byte) {
                char::from(byte).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect()
}

fn movie_json(movie: &Movie) -> Value {
    let Some(label) = movie.label() else {
        return json!({ "watched": movie.watched });
    };
    let query = match &movie.year {
        Some(year) => format!("{} {year}", movie.title.as_deref().unwrap_or_default()),
        None => movie.title.clone().unwrap_or_default(),
    };
    json!({
        "label": label,
        "watched": movie.watched,
        "imdb": format!("https://www.imdb.com/find/?q={}&s=tt", encode(&query)),
        "letterboxd": format!("https://letterboxd.com/search/films/{}/", encode(&query)),
    })
}

/// Each `# ` heading's task-list lines, in page order.
fn lists(markdown: &str) -> Vec<(String, Vec<Movie>)> {
    let mut lists: Vec<(String, Vec<Movie>)> = Vec::new();
    for line in markdown.lines() {
        if let Some(heading) = line.strip_prefix("# ") {
            lists.push((heading.trim().to_string(), Vec::new()));
        } else if let (Some(movie), Some((_, movies))) = (parse_line(line), lists.last_mut()) {
            movies.push(movie);
        }
    }
    lists.retain(|(_, movies)| !movies.is_empty());
    lists
}

fn render(markdown: &str) -> Value {
    Value::from(
        lists(markdown)
            .iter()
            .map(|(heading, movies)| {
                let pickable = heading != DROPPED
                    && movies
                        .iter()
                        .any(|movie| movie.title.is_some() && !movie.watched);
                json!({
                    "heading": heading,
                    "pickable": pickable,
                    "movies": movies.iter().map(movie_json).collect::<Vec<_>>(),
                })
            })
            .collect::<Vec<_>>(),
    )
}

impl Site {
    pub(crate) fn movies(&self) -> Result<()> {
        let markdown = fs::read_to_string(self.root.join(SOURCE))?;
        fs::write(self.root.join(OUTPUT), render(&markdown).to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn movie(title: Option<&str>, year: Option<&str>, watched: bool) -> Movie {
        Movie {
            title: title.map(str::to_string),
            year: year.map(str::to_string),
            watched,
        }
    }

    #[test]
    fn reads_the_shapes_the_lists_hold() {
        let cases = [
            (
                "- [ ] Amadeus (1984)",
                movie(Some("Amadeus"), Some("1984"), false),
            ),
            (
                "- [x] Kill Bill: Volume 2 (2004) (2022-09-17)",
                movie(Some("Kill Bill: Volume 2"), Some("2004"), true),
            ),
            ("- [ ] 12 Monkeys", movie(Some("12 Monkeys"), None, false)),
            (
                "- [ ] Kill Bill: Volume 3 (TBA)",
                movie(Some("Kill Bill: Volume 3"), None, false),
            ),
            ("- [ ] BREAK (2022-10-02)", movie(None, None, true)),
            ("- [ ] UNKNOWN () (2021-04-16)", movie(None, None, true)),
            (
                "- [x] パプリカ (Paprika) (2006) (UNKNOWN)",
                movie(Some("パプリカ (Paprika)"), Some("2006"), true),
            ),
            (
                "- [x] 怪物 (Monster) (2025-01-05)",
                movie(Some("怪物 (Monster)"), None, true),
            ),
            (
                "- [ ] Little Shop of Horrors (1986) (2024-01-07)",
                movie(Some("Little Shop of Horrors"), Some("1986"), true),
            ),
        ];
        for (line, expected) in cases {
            assert_eq!(parse_line(line).as_ref(), Some(&expected), "{line}");
        }
        assert_eq!(parse_line("- [Red Mage:](#red-mage)"), None);
        assert_eq!(parse_line("Special thanks"), None);
    }

    #[test]
    fn links_search_by_title_and_year() {
        let json = movie_json(&movie(Some("Dude, Where's My Car?"), Some("2000"), false));
        assert_eq!(json["label"], "Dude, Where's My Car? (2000)");
        assert_eq!(
            json["imdb"],
            "https://www.imdb.com/find/?q=Dude%2C%20Where's%20My%20Car%3F%202000&s=tt"
        );
        assert_eq!(
            json["letterboxd"],
            "https://letterboxd.com/search/films/Dude%2C%20Where's%20My%20Car%3F%202000/"
        );
        assert_eq!(
            movie_json(&movie(None, None, true)),
            json!({ "watched": true })
        );
    }

    #[test]
    fn groups_lines_under_their_heading_and_never_picks_from_dropped() {
        let rendered = render(
            "intro\n\n- [Me](#me)\n\n# Me\n\n- [x] Pi (1998) (2022-09-25)\n- [ ] Amadeus (1984)\n\n\
             # Done\n\n- [x] Pi (1998)\n\n# Dropped Movies\n\n- [ ] Cats (2019)\n",
        );
        let lists = rendered.as_array().unwrap();
        assert_eq!(lists.len(), 3);
        assert_eq!(lists[0]["heading"], "Me");
        assert_eq!(lists[0]["pickable"], true);
        assert_eq!(lists[0]["movies"].as_array().unwrap().len(), 2);
        assert_eq!(lists[1]["pickable"], false);
        assert_eq!(lists[2]["pickable"], false);
    }

    #[test]
    fn reads_the_real_lists() {
        let root = crate::find_repo_root().unwrap();
        let markdown = fs::read_to_string(root.join(SOURCE)).unwrap();
        let task_lines = markdown
            .lines()
            .filter(|line| line.starts_with("- [ ] ") || line.starts_with("- [x] "))
            .count();
        let read: usize = lists(&markdown)
            .iter()
            .map(|(_, movies)| movies.len())
            .sum();
        assert_eq!(read, task_lines, "a task-list line was not read");
    }
}
