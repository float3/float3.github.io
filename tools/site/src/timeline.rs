//! The AI incident timeline's markup is its data.
//!
//! Every date, kind, headline number and running-total contribution on
//! `content/notes/ai-incidents-2026.md` is an attribute on an entry, and
//! `ts/src/aiincidents.ts` and `ts/src/aiincidents/charts.ts` read them back
//! out. Nothing in either direction fails loudly: a tally key with a typo in
//! it contributes nothing to a total that still adds up, an entry missing its
//! short label drops silently out of a chart, and a date out of order puts a
//! tick in the wrong place on a ruler nobody checks by hand.
//!
//! So the agreement is asserted here instead, against the TypeScript rather
//! than against a copy of it, the way `shaders.rs` compiles the real shader
//! sources rather than a transcription.

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::fs;
    use std::path::{Path, PathBuf};

    const PAGE: &str = "content/notes/ai-incidents-2026.md";
    const SCRIPT: &str = "ts/src/aiincidents.ts";
    const CHARTS: &str = "ts/src/aiincidents/charts.ts";

    /// The chips an entry may wear. The first two are the only ones that carry
    /// colour; the rest are the page's own ink.
    const KINDS: &[&str] = &[
        "agents",
        "accomplices",
        "courts",
        "states",
        "products",
        "people",
    ];

    fn root() -> PathBuf {
        crate::find_repo_root().expect("tests run inside the repository")
    }

    fn read(root: &Path, relative: &str) -> String {
        fs::read_to_string(root.join(relative))
            .unwrap_or_else(|error| panic!("{relative}: {error}"))
    }

    /// One `<li class="ait-entry" ...>` and everything up to the next one.
    fn entries(page: &str) -> Vec<&str> {
        let mut found = Vec::new();
        let mut rest = page;

        while let Some(start) = rest.find("<li class=\"ait-entry\"") {
            rest = &rest[start..];
            let end = rest[1..]
                .find("<li class=\"ait-entry\"")
                .map_or(rest.len(), |next| next + 1);
            found.push(&rest[..end]);
            rest = &rest[end..];
        }

        found
    }

    /// Every `attribute="value"` of one name, in order.
    fn attribute<'a>(text: &'a str, name: &str) -> Vec<&'a str> {
        let open = format!("{name}=\"");
        let mut found = Vec::new();
        let mut rest = text;

        while let Some(start) = rest.find(&open) {
            let after = &rest[start + open.len()..];
            let Some(end) = after.find('"') else { break };
            found.push(&after[..end]);
            rest = &after[end..];
        }

        found
    }

    /// Everything between `open` and the next `close`, for each occurrence.
    fn between<'a>(text: &'a str, open: &str, close: char) -> Vec<&'a str> {
        let mut found = Vec::new();
        let mut rest = text;

        while let Some(start) = rest.find(open) {
            let after = &rest[start + open.len()..];
            let Some(end) = after.find(close) else { break };
            found.push(&after[..end]);
            rest = &after[end..];
        }

        found
    }

    /// The `{ key: "..." }` names in the page script's `TALLIES`, which is the
    /// list of running totals it knows how to add up.
    fn tally_keys(script: &str) -> BTreeSet<String> {
        let named = script
            .find("const TALLIES")
            .expect("the page script has no TALLIES");
        // `readonly Tally[]` closes a bracket of its own before the array does.
        let start = named
            + script[named..]
                .find("= [")
                .expect("TALLIES is not an array");
        let end = start
            + script[start..]
                .find(
                    "
]",
                )
                .expect("TALLIES is never closed");

        let keys: BTreeSet<String> = between(&script[start..end], "key: \"", '"')
            .iter()
            .map(|key| (*key).to_string())
            .collect();

        assert!(
            !keys.is_empty(),
            "TALLIES parsed as empty, so this test proves nothing"
        );
        keys
    }

    /// The entry's heading, which is what a failure message has to name.
    fn heading(entry: &str) -> String {
        between(entry, "<h3>", '<')
            .first()
            .map_or_else(|| "?".to_string(), |title| (*title).to_string())
    }

    #[test]
    fn every_entry_is_dated_and_the_dates_are_in_order() {
        let page = read(&root(), PAGE);
        let found = entries(&page);
        assert!(found.len() > 20, "the timeline lost most of its entries");

        let mut dates = Vec::new();
        for entry in &found {
            let at = attribute(entry, "class=\"ait-at\" datetime");
            assert_eq!(
                at.len(),
                1,
                "an entry has {} dates where it should have one:\n{}",
                at.len(),
                &entry[..entry.len().min(200)]
            );
            assert!(
                at[0].starts_with("2026-") && at[0].len() == 10,
                "`{}` is not a 2026 date",
                at[0]
            );
            dates.push(at[0]);
        }

        let mut sorted = dates.clone();
        sorted.sort_unstable();
        assert_eq!(
            dates, sorted,
            "the entries are not in date order, so the ruler would draw them out of place"
        );
    }

    /// A key the script does not sum adds nothing, silently, to a total that
    /// still looks right.
    #[test]
    fn every_tally_in_the_markup_is_one_the_script_adds_up() {
        let root = root();
        let page = read(&root, PAGE);
        let known = tally_keys(&read(&root, SCRIPT));

        let used: BTreeSet<String> = entries(&page)
            .iter()
            .flat_map(|entry| between(entry, "data-tally-", '='))
            .map(str::to_string)
            .collect();

        assert!(!used.is_empty(), "no entry contributes to any total");
        let unknown: Vec<&String> = used.difference(&known).collect();
        assert!(
            unknown.is_empty(),
            "the markup contributes to totals the page script never sums: {unknown:?}\n\
             known: {known:?}"
        );
    }

    /// `charts.ts` paints `is-agents` and `is-accomplices` and treats every
    /// other chip as unremarkable ink, so a renamed chip loses its colour
    /// without anything going wrong anywhere.
    #[test]
    fn every_chip_is_a_kind_the_charts_know() {
        let root = root();
        let page = read(&root, PAGE);
        let charts = read(&root, CHARTS);

        let used: BTreeSet<String> = entries(&page)
            .iter()
            .flat_map(|entry| between(entry, "class=\"ait-kind\">", '<'))
            .map(str::to_string)
            .collect();

        let unknown: Vec<&String> = used
            .iter()
            .filter(|kind| !KINDS.contains(&kind.as_str()))
            .collect();
        assert!(
            unknown.is_empty(),
            "entries wear chips nothing knows: {unknown:?}"
        );

        for coloured in ["agents", "accomplices"] {
            assert!(
                used.contains(coloured),
                "no entry is `{coloured}` any more, so a whole series is empty"
            );
            assert!(
                charts.contains(&format!("kind === \"{coloured}\"")),
                "charts.ts no longer gives `{coloured}` a colour of its own"
            );
        }
    }

    /// The silence chart draws one labelled row per entry that was kept quiet,
    /// and skips anything missing either half. Both halves are easy to forget.
    #[test]
    fn an_entry_that_was_kept_quiet_carries_a_label_for_the_chart() {
        let page = read(&root(), PAGE);
        let mut missing = Vec::new();
        let mut spare = Vec::new();

        for entry in entries(&page) {
            let quiet = entry.contains("class=\"ait-disclosed\"");
            let labelled = entry.contains("data-short=\"");
            if quiet && !labelled {
                missing.push(heading(entry));
            }
            if labelled && !quiet {
                spare.push(heading(entry));
            }
        }

        assert!(
            missing.is_empty(),
            "these were disclosed late but carry no data-short, so they are left out of the \
             silence chart without a word: {missing:?}"
        );
        assert!(
            spare.is_empty(),
            "these carry a data-short the silence chart will never draw: {spare:?}"
        );
    }
}
