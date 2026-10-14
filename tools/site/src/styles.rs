//! The one layout rule a stylesheet can be held to without a browser.
//!
//! Below Quartz's `$mobile` breakpoint the left sidebar stops being a column
//! beside the page and becomes a row above it, and the explorer plugin makes it
//! `position: sticky` while `custom.scss` leaves it transparent and gives it
//! `z-index: 200`. So a page that pins something at `top: 0` parks it under the
//! site's own header, and the two are legible through each other. Nothing about
//! that is visible in the page's own stylesheet, which is why it shipped.
//!
//! Whether two boxes actually overlap is a question for a layout engine, and
//! there is no browser in this repository. What is checkable from the source is
//! the convention that avoids the question: anything stuck to the top of the
//! *window* goes through `--sticky-top`, which `custom.scss` defines and
//! `ts/src/shared/sticky.ts` keeps in step with the header. A sticky pinned
//! inside a scrolling box of the page's own — a table head, a group label in a
//! list — is a different thing and has no business knowing about the header, so
//! those are named in `INSIDE_A_SCROLLER` one at a time rather than waved
//! through by a pattern.

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    /// Selectors that stick inside a scrolling box of their own page rather
    /// than to the window, and so are measured from that box's top edge, not
    /// the header's. Each names the scroller it lives in.
    const INSIDE_A_SCROLLER: &[(&str, &str)] = &[
        (".rt-group", ".rt-results, the scale picker's own dropdown"),
        (".rt .rt-matrix thead th", ".rt-matrix-scroll"),
        (".tp-group", ".tp-list"),
    ];

    /// `quartz/styles/variables.scss` is where the number lives; if the fork
    /// ever moves it, every assertion below should be re-read rather than
    /// quietly keep passing against a stale one.
    const MOBILE: &str = "800px";

    fn stylesheets(root: &Path) -> Vec<PathBuf> {
        let mut found = Vec::new();
        let mut stack = vec![root.join("content")];

        while let Some(dir) = stack.pop() {
            let Ok(entries) = fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    // `content/js` is a build output and holds nothing authored.
                    if path.file_name().is_some_and(|name| name != "js") {
                        stack.push(path);
                    }
                } else if path.extension().is_some_and(|ext| ext == "css") {
                    found.push(path);
                }
            }
        }

        found.sort();
        found
    }

    /// A comment sits between two rules, so leaving it in glues it onto the
    /// front of the selector that follows and the allowlist stops matching.
    fn without_comments(text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        let mut rest = text;

        while let Some(open) = rest.find("/*") {
            out.push_str(&rest[..open]);
            let after = &rest[open + 2..];
            match after.find("*/") {
                Some(close) => rest = &after[close + 2..],
                None => return out,
            }
        }

        out.push_str(rest);
        out
    }

    /// Every rule in a stylesheet as `(selector, body)`, which is all the
    /// parsing this needs: the files are hand-written and flat, and a rule
    /// nested in a media query still comes out whole.
    fn rules(text: &str) -> Vec<(String, String)> {
        let stripped = without_comments(text);
        let mut found = Vec::new();
        let mut rest = stripped.as_str();

        while let Some(open) = rest.find('{') {
            let selector = rest[..open].trim();
            let selector = selector
                .rsplit(['{', '}'])
                .next()
                .unwrap_or(selector)
                .trim()
                .to_string();
            let after = &rest[open + 1..];
            let Some(close) = after.find('}') else {
                break;
            };
            // A media query's body opens another brace before it closes; its
            // own rules are picked up by the loop that follows.
            if !after[..close].contains('{') {
                found.push((selector, after[..close].to_string()));
            }
            rest = &rest[open + 1..];
        }

        found
    }

    fn declaration<'a>(body: &'a str, property: &str) -> Option<&'a str> {
        body.split(';')
            .map(str::trim)
            .find_map(|line| line.strip_prefix(property)?.strip_prefix(':'))
            .map(str::trim)
    }

    /// The check that the AI incident timeline failed: it stuck its panel to
    /// the window at `top: 0`, under a header that is sticky at `top: 0` too.
    #[test]
    fn nothing_pins_itself_to_the_top_of_the_window_by_hand() {
        let root = crate::find_repo_root().expect("tests run inside the repository");
        let mut wrong = Vec::new();

        for path in stylesheets(&root) {
            let text = fs::read_to_string(&path).unwrap();
            let name = path.strip_prefix(&root).unwrap_or(&path).display();

            for (selector, body) in rules(&text) {
                let position = declaration(&body, "position");
                if !matches!(position, Some("sticky" | "fixed")) {
                    continue;
                }
                let Some(top) = declaration(&body, "top") else {
                    // No `top` at all: it sticks sideways, or to wherever it
                    // already is, and clears nothing.
                    continue;
                };
                if top.contains("var(--sticky-top)")
                    || INSIDE_A_SCROLLER
                        .iter()
                        .any(|(named, _)| *named == selector)
                {
                    continue;
                }

                wrong.push(format!("{name}: `{selector}` has `top: {top}`"));
            }
        }

        assert!(
            wrong.is_empty(),
            "a page may not choose its own offset from the top of the window; use \
             `top: var(--sticky-top)`, or name the scrolling box it sticks inside in \
             INSIDE_A_SCROLLER in this file:\n  {}",
            wrong.join("\n  ")
        );
    }

    /// Every selector in the allowlist still exists. A rule that is renamed or
    /// deleted leaves an entry here excusing nothing, and the next sticky to
    /// take that name inherits the excuse.
    #[test]
    fn the_allowlist_names_rules_that_are_still_there() {
        let root = crate::find_repo_root().expect("tests run inside the repository");
        let text: String = stylesheets(&root)
            .iter()
            .filter_map(|path| fs::read_to_string(path).ok())
            .collect();
        let found: Vec<String> = rules(&text)
            .into_iter()
            .map(|(selector, _)| selector)
            .collect();

        let stale: Vec<&str> = INSIDE_A_SCROLLER
            .iter()
            .map(|(selector, _)| *selector)
            .filter(|selector| !found.iter().any(|rule| rule == selector))
            .collect();

        assert!(
            stale.is_empty(),
            "INSIDE_A_SCROLLER excuses selectors no stylesheet has: {stale:?}"
        );
    }

    /// `--sticky-top` is only worth using if it is defined, and only correct if
    /// it changes at the breakpoint that changes the header. Both halves live
    /// in files this repository owns, so both can be asserted.
    #[test]
    fn the_offset_is_defined_at_the_breakpoint_that_makes_the_header_sticky() {
        let root = crate::find_repo_root().expect("tests run inside the repository");
        let custom = fs::read_to_string(root.join("quartz/styles/custom.scss")).unwrap();
        let variables = fs::read_to_string(root.join("quartz/styles/variables.scss")).unwrap();
        let base = fs::read_to_string(root.join("quartz/styles/base.scss")).unwrap();

        assert!(
            variables.contains(&format!("mobile: {MOBILE}")),
            "$mobile is no longer {MOBILE}; the offset in custom.scss and the media query in \
             ts/src/shared/sticky.ts both name it"
        );
        assert!(
            custom.contains("--sticky-top: 0px"),
            "custom.scss no longer defines --sticky-top for the wide layout"
        );

        let narrow = custom
            .split("@media all and ($mobile)")
            .skip(1)
            .any(|block| block[..block.len().min(120)].contains("--sticky-top:"));
        assert!(
            narrow,
            "custom.scss defines --sticky-top but never changes it under $mobile, which is \
             the only width where the header is in the way"
        );

        assert!(
            base.contains("position: sticky;"),
            "the sidebar in base.scss is no longer sticky; if the header stopped standing at \
             the top of the window, --sticky-top can go back to nothing everywhere"
        );
    }

    /// The measuring script and the stylesheet have to mean the same width and
    /// the same property, and neither says so to the other.
    #[test]
    fn the_script_and_the_stylesheet_agree() {
        let root = crate::find_repo_root().expect("tests run inside the repository");
        let script = fs::read_to_string(root.join("ts/src/shared/sticky.ts")).unwrap();

        assert!(
            script.contains(&format!("(max-width: {MOBILE})")),
            "ts/src/shared/sticky.ts measures at a different width than $mobile"
        );
        assert!(
            script.contains("\"--sticky-top\""),
            "ts/src/shared/sticky.ts no longer writes --sticky-top"
        );
    }
}
