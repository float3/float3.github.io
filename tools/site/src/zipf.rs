use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use crate::{Result, Site, html};

pub(crate) fn write(site: &Site, public: &Path) -> Result<()> {
    let content_dir = site.root.join("content");
    let mut md_paths = Vec::new();
    collect_md_files(&content_dir, &mut md_paths)?;

    let mut counts: HashMap<String, u64> = HashMap::new();

    for path in md_paths {
        if let Ok(text) = fs::read_to_string(&path)
            && !is_list_page(&text)
        {
            let stripped = strip_entities(&strip_html(&strip_links(&text)));
            for word in tokenize_words(&stripped) {
                *counts.entry(word).or_insert(0) += 1;
            }
        }
    }

    let mut items: Vec<(String, u64)> = counts.into_iter().collect();
    items.sort();
    items.sort_by_key(|b| std::cmp::Reverse(b.1));

    let data_js = format!("const ZIPF_DATA = {};", serde_json::to_string(&items)?);
    let top_list_html = build_top_list_html(&items);

    let word_count: u64 = items.iter().map(|(_, count)| count).sum();
    let unique = items.len();

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Zipf Distribution</title>
  <style>
    body {{ font-family: system-ui, -apple-system, 'Segoe UI', Roboto, 'Helvetica Neue', Arial; margin: 0; padding: 20px; color: #222; background: #fff; }}
    .container {{ display: flex; gap: 20px; flex-wrap: wrap; align-items: flex-start; }}
    .plot {{ flex: 1 1 480px; min-width: 300px; }}
    .list {{ flex: 0 1 320px; max-height: 80vh; overflow: auto; }}
    svg {{ width: 100%; height: auto; border: 1px solid #ddd; background: #fff; }}
    .axis {{ stroke: #666; stroke-width: 1; }}
    .fit {{ stroke: #c33; stroke-width: 1; stroke-dasharray: 4 4; }}
    .point {{ fill: steelblue; opacity: 0.8; }}
    .point:hover {{ fill: #c33; opacity: 1; }}
    .label {{ font-size: 11px; fill: #333; }}
    .word {{ font-size: 11px; fill: #333; pointer-events: none; }}
  </style>
</head>
<body>
  <h1>Zipf distribution of the words in /content</h1>
  <p>{word_count} words, {unique} distinct. Rank against frequency, both logarithmic. The dashed line is what Zipf's law predicts from the most common word alone. The most common words are labelled; hover any point for its word, or find it in the list.</p>
  <div class="container">
    <div class="plot">
      <svg id="plot" viewBox="0 0 800 600" role="img" aria-label="rank against frequency"></svg>
    </div>
    <div class="list">
      <h2>All words</h2>
      {top_list_html}
    </div>
  </div>

  <script>
{data_js}

const LABELLED = 25;
const POINTS = 5000;

function draw() {{
  const svg = document.getElementById('plot');
  const width = 800, height = 600, pad = 60;
  const data = ZIPF_DATA.slice(0, POINTS).map((d, i) => ({{ rank: i + 1, count: d[1], word: d[0] }}));
  if (data.length === 0) return;

  const log10 = Math.log10;
  const lxmax = Math.max(log10(ZIPF_DATA.length), 1);
  const lymax = Math.max(log10(data[0].count), 1);
  const sx = x => pad + (log10(x) / lxmax) * (width - pad * 2);
  const sy = y => height - pad - (log10(y) / lymax) * (height - pad * 2);

  const ns = 'http://www.w3.org/2000/svg';
  function el(name, attrs, parent) {{
    const e = document.createElementNS(ns, name);
    for (const k in attrs) e.setAttribute(k, attrs[k]);
    (parent || svg).appendChild(e);
    return e;
  }}

  el('line', {{ x1: pad, y1: height - pad, x2: width - pad, y2: height - pad, class: 'axis' }});
  el('line', {{ x1: pad, y1: pad, x2: pad, y2: height - pad, class: 'axis' }});

  for (let e = 0; e <= Math.floor(lxmax); e++) {{
    const x = sx(Math.pow(10, e));
    el('line', {{ x1: x, y1: height - pad, x2: x, y2: height - pad + 6, class: 'axis' }});
    el('text', {{ x, y: height - pad + 20, 'text-anchor': 'middle', class: 'label' }}).textContent = Math.pow(10, e).toLocaleString();
  }}
  el('text', {{ x: width / 2, y: height - pad + 40, 'text-anchor': 'middle', class: 'label' }}).textContent = 'rank';

  for (let e = 0; e <= Math.floor(lymax); e++) {{
    const y = sy(Math.pow(10, e));
    el('line', {{ x1: pad - 6, y1: y, x2: pad, y2: y, class: 'axis' }});
    el('text', {{ x: pad - 10, y: y + 4, 'text-anchor': 'end', class: 'label' }}).textContent = Math.pow(10, e).toLocaleString();
  }}
  el('text', {{ x: 16, y: height / 2, 'text-anchor': 'middle', class: 'label', transform: `rotate(-90 16 ${{height / 2}})` }}).textContent = 'occurrences';

  const top = data[0].count;
  const end = Math.min(ZIPF_DATA.length, top);
  el('line', {{ x1: sx(1), y1: sy(top), x2: sx(end), y2: sy(top / end), class: 'fit' }});

  for (const d of data) {{
    const point = el('circle', {{ cx: sx(d.rank), cy: sy(d.count), r: 2, class: 'point' }});
    el('title', {{}}, point).textContent = `${{d.rank}}. ${{d.word}} (${{d.count}})`;
  }}

  let lastY = -Infinity;
  for (const d of data.slice(0, LABELLED)) {{
    const y = sy(d.count);
    if (Math.abs(y - lastY) < 12) continue;
    lastY = y;
    el('text', {{ x: sx(d.rank) + 6, y: y + 4, class: 'word' }}).textContent = d.word;
  }}
}}

draw();
  </script>
</body>
</html>
"#
    );

    fs::write(public.join("zipf.html"), html)?;
    Ok(())
}

/// Whether a page is an index rather than prose: anything tagged `list`, which
/// covers the post listing, the gallery indices, and the 3 MB Udon exposure
/// dump that would otherwise be half of every count on the page.
fn is_list_page(text: &str) -> bool {
    let Some(body) = text.strip_prefix("---") else {
        return false;
    };
    let Some(end) = body.find("\n---") else {
        return false;
    };

    body[..end].lines().any(|line| {
        let line = line.trim();
        line == "- list"
            || line
                .strip_prefix("tags:")
                .is_some_and(|tags| tags.split(['[', ']', ',']).any(|tag| tag.trim() == "list"))
    })
}

/// Drops `&lt;`, `&#39;` and the rest: an entity renders as punctuation, and
/// left in it counts as a word called `lt`.
fn strip_entities(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;

    while let Some(start) = rest.find('&') {
        out.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        let entity = after.find(';').filter(|&end| {
            end <= 10
                && after[..end]
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '#')
        });

        match entity {
            Some(end) => rest = &after[end + 1..],
            None => {
                out.push('&');
                rest = after;
            }
        }
    }

    out.push_str(rest);
    out
}

fn collect_md_files(dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            collect_md_files(&path, out)?;
            continue;
        }

        let is_md = path
            .extension()
            .and_then(OsStr::to_str)
            .is_some_and(|ext| ext.eq_ignore_ascii_case("md"));

        if is_md {
            out.push(path);
        }
    }

    Ok(())
}

fn strip_links(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = String::with_capacity(text.len());

    let mut i = 0;
    let mut last_keep = 0;

    while i < bytes.len() {
        // Markdown image: ![alt](url)
        if bytes[i] == b'!'
            && i + 1 < bytes.len()
            && bytes[i + 1] == b'['
            && let Some(end) = skip_markdown_link(text, i + 1)
        {
            out.push_str(&text[last_keep..i]);
            last_keep = end;
            i = end;
            continue;
        }

        // Markdown link: [text](url) or [text][ref]
        if bytes[i] == b'['
            && let Some(end) = skip_markdown_link(text, i)
        {
            out.push_str(&text[last_keep..i]);
            last_keep = end;
            i = end;
            continue;
        }

        // Autolink: <https://example.com>
        if bytes[i] == b'<'
            && let Some(close_rel) = text[i + 1..].find('>')
        {
            let inner = &text[i + 1..i + 1 + close_rel];
            if looks_like_url(inner) {
                out.push_str(&text[last_keep..i]);
                last_keep = i + close_rel + 2;
                i = last_keep;
                continue;
            }
        }

        // Raw URL: http://... or https://...
        if bytes[i..].starts_with(b"http://") || bytes[i..].starts_with(b"https://") {
            out.push_str(&text[last_keep..i]);
            let mut j = i;
            while j < bytes.len() && !bytes[j].is_ascii_whitespace() {
                j += 1;
            }
            last_keep = j;
            i = j;
            continue;
        }

        // Advance by one UTF-8 character, not one byte
        let ch_len = text[i..].chars().next().unwrap().len_utf8();
        i += ch_len;
    }

    out.push_str(&text[last_keep..]);
    out
}

fn skip_markdown_link(text: &str, start: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    if bytes.get(start) != Some(&b'[') {
        return None;
    }

    let close_bracket = text[start + 1..].find(']')? + start + 1;
    let after = &text[close_bracket + 1..];

    // [text](url)
    if after.starts_with('(') {
        let close_paren = after.find(')')?;
        return Some(close_bracket + 2 + close_paren);
    }

    // [text][ref]
    if after.starts_with('[') {
        let close_ref = after.find(']')?;
        return Some(close_bracket + 2 + close_ref);
    }

    None
}

fn looks_like_url(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://") || s.starts_with("www.")
}

fn strip_html(text: &str) -> String {
    let mut out = String::with_capacity(text.len());

    let mut in_tag = false;

    for ch in text.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }

    out
}

fn tokenize_words(text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut buf = String::new();

    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        let apostrophe = matches!(ch, '\'' | '’');
        let joins =
            apostrophe && !buf.is_empty() && chars.peek().is_some_and(|next| next.is_alphabetic());

        if apostrophe && joins {
            buf.push('\'');
        } else if ch.is_alphabetic() {
            buf.extend(ch.to_lowercase());
        } else if !buf.is_empty() {
            words.push(std::mem::take(&mut buf));
        }
    }

    if !buf.is_empty() {
        words.push(buf);
    }

    words
}

fn build_top_list_html(items: &[(String, u64)]) -> String {
    let mut out = String::new();
    out.push_str("<ol>");
    for (word, count) in items {
        out.push_str(&format!(
            "<li><strong>{}</strong>: {}</li>",
            html::escape(word),
            count
        ));
    }
    out.push_str("</ol>");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaves_indices_out_of_the_count() {
        assert!(is_list_page(
            "---\ntitle: posts\ntags:\n  - list\n---\n\nbody"
        ));
        assert!(is_list_page("---\ntags: [unity, list]\n---\n"));
        assert!(!is_list_page(
            "---\ntitle: a list of things\n---\n\n- list\n"
        ));
        assert!(!is_list_page("no frontmatter\n---\n  - list\n"));
    }

    #[test]
    fn an_entity_is_not_a_word() {
        assert_eq!(strip_entities("a &lt;b&gt; &amp; c&#39;s"), "a b  cs");
        assert_eq!(strip_entities("tom & jerry"), "tom & jerry");
        assert_eq!(
            strip_entities("&notanentity because it is too long;"),
            "&notanentity because it is too long;"
        );
    }

    #[test]
    fn counts_the_words_of_the_prose_and_nothing_else() {
        let text = "See [the docs](https://example.com/x) and <em>this</em> &mdash; https://a.b/c.";
        let words = tokenize_words(&strip_entities(&strip_html(&strip_links(text))));
        assert_eq!(words, ["see", "and", "this"]);
    }

    #[test]
    fn keeps_a_contraction_together_and_drops_the_quotes_around_a_word() {
        assert_eq!(
            tokenize_words("Don't ‘stop’, it’s 'fine' École"),
            ["don't", "stop", "it's", "fine", "école"]
        );
    }
}
