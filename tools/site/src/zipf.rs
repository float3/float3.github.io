use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use crate::{Result, Site};

pub(crate) fn write(site: &Site, public: &Path) -> Result<()> {
    let content_dir = site.root.join("content");
    let mut md_paths = Vec::new();
    collect_md_files(&content_dir, &mut md_paths)?;

    let mut counts: HashMap<String, u64> = HashMap::new();

    for path in md_paths {
        if let Ok(text) = fs::read_to_string(&path) {
            let stripped = strip_links(&text);
            let stripped = strip_html(&stripped);
            for word in tokenize_words(&stripped) {
                *counts.entry(word).or_insert(0) += 1;
            }
        }
    }

    let mut items: Vec<(String, u64)> = counts.into_iter().collect();
    items.sort();
    items.sort_by(|a, b| b.1.cmp(&a.1));

    let mut data_entries = Vec::new();
    for (i, (word, count)) in items.iter().enumerate() {
        let rank = i + 1;
        let escaped = escape_js_string(word);
        data_entries.push(format!("[{}, {}, \"{}\"]", rank, count, escaped));
    }

    let data_js = format!("const ZIPF_DATA = [{}];", data_entries.join(",\n"));
    let top_list_html = build_top_list_html(&items, 2000000);

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <title>Zipf Distribution</title>
  <style>
    body {{ font-family: system-ui, -apple-system, 'Segoe UI', Roboto, 'Helvetica Neue', Arial; padding: 20px; }}
    .container {{ display: flex; gap: 20px; flex-wrap: wrap; }}
    .plot {{ flex: 1 1 480px; min-width: 300px; }}
    .list {{ width: 360px; max-height: 80vh; overflow: auto; }}
    svg {{ width: 100%; height: 600px; border: 1px solid #ddd; background: #fff; }}
    .axis {{ stroke: #666; stroke-width: 1; }}
    .point {{ fill: steelblue; opacity: 0.9; }}
    .label {{ font-size: 12px; fill: #333; }}
  </style>
</head>
<body>
  <h1>Zipf Distribution of Words in /content</h1>
  <p>Scatter plot of rank vs frequency (log-log). Right column lists the top words.</p>
  <div class="container">
    <div class="plot">
      <svg id="plot" viewBox="0 0 800 600" preserveAspectRatio="none"></svg>
    </div>
    <div class="list">
      <h2>Top words</h2>
      {top_list_html}
    </div>
  </div>

  <script>
{data_js}

function log10(x) {{ return Math.log(x) / Math.LN10; }}

function draw() {{
  const svg = document.getElementById('plot');
  const width = 800, height = 600, pad = 60;
  svg.setAttribute('viewBox', `0 0 ${{width}} ${{height}}`);
  const data = ZIPF_DATA.map(d => ({{ rank: d[0], count: d[1], word: d[2] }}));
  if (data.length === 0) return;

  const ranks = data.map(d => d.rank);
  const counts = data.map(d => d.count);
  const xmin = Math.max(1, Math.min(...ranks));
  const xmax = Math.max(...ranks);
  const ymin = Math.max(1, Math.min(...counts));
  const ymax = Math.max(...counts);

  const lxmin = log10(xmin), lxmax = log10(xmax);
  const lymin = log10(ymin), lymax = log10(ymax);

  function sx(x) {{ return pad + ((log10(x) - lxmin) / (lxmax - lxmin)) * (width - pad * 2); }}
  function sy(y) {{ return height - pad - ((log10(y) - lymin) / (lymax - lymin)) * (height - pad * 2); }}

  svg.innerHTML = '';
  const ns = 'http://www.w3.org/2000/svg';
  function el(name, attrs) {{
    const e = document.createElementNS(ns, name);
    for (const k in attrs) e.setAttribute(k, attrs[k]);
    return e;
  }}

  svg.appendChild(el('rect', {{ x:0, y:0, width:width, height:height, fill:'white' }}));
  svg.appendChild(el('line', {{ x1: pad, y1: height-pad, x2: width-pad, y2: height-pad, class:'axis' }}));
  svg.appendChild(el('line', {{ x1: pad, y1: pad, x2: pad, y2: height-pad, class:'axis' }}));

  for (let e = Math.ceil(lxmin); e <= Math.floor(lxmax); e++) {{
    const x = Math.pow(10, e);
    const xPos = sx(x);
    svg.appendChild(el('line', {{ x1:xPos, y1:height-pad, x2:xPos, y2:height-pad+6, stroke:'#666' }}));
    svg.appendChild(el('text', {{ x:xPos, y:height-pad+20, 'text-anchor':'middle', class:'label' }})).textContent = '10^' + e;
  }}

  for (let e = Math.ceil(lymin); e <= Math.floor(lymax); e++) {{
    const y = Math.pow(10, e);
    const yPos = sy(y);
    svg.appendChild(el('line', {{ x1:pad-6, y1:yPos, x2:pad, y2:yPos, stroke:'#666' }}));
    svg.appendChild(el('text', {{ x:pad-10, y:yPos+4, 'text-anchor':'end', class:'label' }})).textContent = '10^' + e;
  }}

  const limit = Math.min(data.length, 5000);
  for (let i = 0; i < limit; i++) {{
    const d = data[i];
    svg.appendChild(el('circle', {{ cx: sx(d.rank), cy: sy(d.count), r: 2, class: 'point' }}));
  }}

  for (let i = 0; i < Math.min(2000000, data.length); i++) {{
    const d = data[i];
    const t = el('text', {{ x: sx(d.rank) + 6, y: sy(d.count) + 4, class:'label' }});
    t.textContent = `${{i+1}}: ${{d.word}} (${{d.count}})`;
    svg.appendChild(t);
  }}
}}

window.addEventListener('load', draw);
window.addEventListener('resize', draw);
  </script>
</body>
</html>
"#,
        data_js = data_js,
        top_list_html = top_list_html
    );

    fs::write(public.join("zipf.html"), html)?;
    Ok(())
}

const FILE_BLACKLIST: &[&str] = &["content/misc/posts/index.md"];

fn is_blacklisted(path: &Path) -> bool {
    FILE_BLACKLIST
        .iter()
        .any(|blocked| path.ends_with(Path::new(blocked)))
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

        if is_md && !is_blacklisted(&path) {
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
        if bytes[i] == b'!' && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
            if let Some(end) = skip_markdown_link(text, i + 1) {
                out.push_str(&text[last_keep..i]);
                last_keep = end;
                i = end;
                continue;
            }
        }

        // Markdown link: [text](url) or [text][ref]
        if bytes[i] == b'[' {
            if let Some(end) = skip_markdown_link(text, i) {
                out.push_str(&text[last_keep..i]);
                last_keep = end;
                i = end;
                continue;
            }
        }

        // Autolink: <https://example.com>
        if bytes[i] == b'<' {
            if let Some(close_rel) = text[i + 1..].find('>') {
                let inner = &text[i + 1..i + 1 + close_rel];
                if looks_like_url(inner) {
                    out.push_str(&text[last_keep..i]);
                    last_keep = i + close_rel + 2;
                    i = last_keep;
                    continue;
                }
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

    for ch in text.chars() {
        if ch.is_alphabetic() {
            buf.push(ch.to_ascii_lowercase());
        } else if !buf.is_empty() {
            words.push(std::mem::take(&mut buf));
        }
    }

    if !buf.is_empty() {
        words.push(buf);
    }

    words
}

fn escape_js_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\"', "\\\"")
}

fn build_top_list_html(items: &[(String, u64)], limit: usize) -> String {
    let mut out = String::new();
    out.push_str("<ol>");
    for (word, count) in items.iter().take(limit) {
        out.push_str(&format!(
            "<li><strong>{}</strong>: {}</li>",
            escape_html(word),
            count
        ));
    }
    out.push_str("</ol>");
    out
}

fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}
