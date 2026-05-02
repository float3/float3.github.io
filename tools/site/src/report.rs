use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::{os_args, Result, Site};

const REPORT_START: &str = "<!-- REPORT START -->";
const REPORT_END: &str = "<!-- REPORT END -->";

#[derive(Default)]
struct TreeSummary {
    directories: usize,
    files: usize,
}

struct TreeNode {
    name: String,
    rel_path: PathBuf,
    size: u64,
    is_dir: bool,
    children: Vec<TreeNode>,
}

pub(crate) fn write(site: &Site, public: &Path, build_time: u64) -> Result<()> {
    let time_str = command_text(public, "date", &["+%x (%A) %X %z"]);
    let os_version = os_version();
    let nix_version = command_text(public, "nix", &["--version"]);
    let quartz_version = command_text(&site.root, "npx", &["quartz", "--version"])
        .lines()
        .last()
        .unwrap_or("not installed")
        .to_string();
    let python_version = command_text(public, "python", &["--version"]);
    let corepack_version = command_text(public, "corepack", &["--version"]);
    let node_version = command_text(public, "node", &["--version"]);
    let npm_version = command_text(public, "npm", &["--version"]);
    let pnpm_version = command_text(public, "pnpm", &["--version"]);
    let cargo_version = command_text(public, "cargo", &["--version"]);
    let rustc_version = command_text(public, "rustc", &["--version"]);
    let rustup_version = command_text(public, "rustup", &["--version"])
        .lines()
        .next()
        .unwrap_or("not installed")
        .to_string();
    let wasm_pack_version = command_text(public, "wasm-pack", &["--version"]);
    let tsc_version = command_text(public, "pnpm", &["exec", "tsc", "--version"]);
    let git_commit = site
        .output_optional(&site.root, "git", &os_args(&["rev-parse", "HEAD"]))?
        .unwrap_or_else(|| "unknown".to_string());
    let webpack_version = webpack_version(&site.root.join("ts"));

    let (tree, summary) = scan_public(public)?;
    let previous_report = fetch_previous_report();

    let mut current_body = Vec::new();
    current_body.push("<h1>Build Report</h1>".to_string());
    current_body.push(format!(
        "<p><strong>Report generated at:</strong> {}</p>",
        escape_html(&time_str)
    ));
    if build_time > 0 {
        current_body.push(format!(
            "<p><strong>Build Time:</strong> {build_time} seconds</p>"
        ));
    }
    current_body.push(format!(
        "<p><strong>Operating System:</strong> {}</p>",
        escape_html(&os_version)
    ));
    current_body.push(version_line("Nix", &nix_version));
    current_body.push(version_line("Quartz", &quartz_version));
    current_body.push(version_line("Python", &python_version));
    current_body.push(version_line("corepack", &corepack_version));
    current_body.push(version_line("Node.js", &node_version));
    current_body.push(version_line("npm", &npm_version));
    current_body.push(version_line("pnpm", &pnpm_version));
    current_body.push(version_line("cargo", &cargo_version));
    current_body.push(version_line("rustc", &rustc_version));
    current_body.push(version_line("rustup", &rustup_version));
    current_body.push(version_line("wasm-pack", &wasm_pack_version));
    current_body.push(version_line("tsc", &tsc_version));
    current_body.push(version_line("webpack", &webpack_version));
    current_body.push(version_line("Git Commit", &git_commit));
    current_body.push(format!(
        "<h2>Tree Summary</h2><p>Total Directories: {}, Total Files: {}</p>",
        summary.directories, summary.files
    ));
    current_body.push("<h2>Folder Contents</h2>".to_string());
    current_body.push(build_tree_html(&tree));

    let current_report_body = current_body.join("\n");
    let current_report = format!("{REPORT_START}\n{current_report_body}\n{REPORT_END}");

    let previous_column = previous_report
        .map(|report| format!("<div class='report-column'><h2>Previous Report</h2>{report}</div>"))
        .unwrap_or_default();

    let combined_html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8"/>
  <title>Build Report Comparison</title>
  <style>
    .container {{
      display: flex;
      flex-wrap: wrap;
    }}
    .report-column {{
      flex: 1;
      padding: 10px;
      box-sizing: border-box;
      min-width: 300px;
      border: 1px solid #ccc;
      margin: 5px;
    }}
  </style>
</head>
<body>
  <h1>Build Report Comparison</h1>
  <div class="container">
    <div class="report-column"><h2>Current Report</h2>{current_report}</div>
    {previous_column}
  </div>
</body>
</html>
"#
    );

    fs::write(public.join("report.html"), combined_html)?;
    Ok(())
}

fn version_line(label: &str, value: &str) -> String {
    format!(
        "<p><strong>{}:</strong> {}</p>",
        escape_html(label),
        escape_html(value)
    )
}

fn command_text(cwd: &Path, program: &str, args: &[&str]) -> String {
    let output = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(output) => {
            let mut text = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if text.is_empty() {
                text = String::from_utf8_lossy(&output.stderr).trim().to_string();
            }
            if text.is_empty() {
                "not installed".to_string()
            } else {
                text
            }
        }
        Err(_) => "not installed".to_string(),
    }
}

fn webpack_version(ts_dir: &Path) -> String {
    let full = command_text(ts_dir, "pnpm", &["exec", "webpack", "--version"]);
    if let Some((_, packages)) = full.split_once("Packages:") {
        packages.trim().replace('\n', "<br>")
    } else {
        full
    }
}

fn os_version() -> String {
    let Ok(source) = fs::read_to_string("/etc/os-release") else {
        return "Unknown OS".to_string();
    };

    source
        .lines()
        .find_map(|line| {
            line.strip_prefix("PRETTY_NAME=")
                .map(|value| value.trim_matches('"').to_string())
        })
        .unwrap_or_else(|| "Unknown OS".to_string())
}

fn scan_public(public: &Path) -> Result<(TreeNode, TreeSummary)> {
    let mut summary = TreeSummary::default();
    let mut root = scan_node(public, PathBuf::from("."), &mut summary)?;
    sort_tree(&mut root);
    Ok((root, summary))
}

fn scan_node(path: &Path, rel_path: PathBuf, summary: &mut TreeSummary) -> Result<TreeNode> {
    let name = rel_path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| ".".to_string());

    if path.is_dir() {
        let mut children = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let child_path = entry.path();
            let child_name = entry.file_name();
            let child_rel = if rel_path.as_os_str() == "." {
                PathBuf::from(child_name)
            } else {
                rel_path.join(child_name)
            };

            if child_path.is_dir() {
                summary.directories += 1;
            } else {
                summary.files += 1;
            }
            children.push(scan_node(&child_path, child_rel, summary)?);
        }

        let size = children.iter().map(|child| child.size).sum();
        Ok(TreeNode {
            name,
            rel_path,
            size,
            is_dir: true,
            children,
        })
    } else {
        Ok(TreeNode {
            name,
            rel_path,
            size: fs::metadata(path)
                .map(|metadata| metadata.len())
                .unwrap_or(0),
            is_dir: false,
            children: Vec::new(),
        })
    }
}

fn sort_tree(node: &mut TreeNode) {
    node.children.sort_by(|left, right| {
        right
            .size
            .cmp(&left.size)
            .then_with(|| left.name.cmp(&right.name))
    });
    for child in &mut node.children {
        sort_tree(child);
    }
}

fn build_tree_html(node: &TreeNode) -> String {
    let name = escape_html(&node.name);
    let size = format!("{:.2} MB", node.size as f64 / 1024.0 / 1024.0);

    if node.is_dir {
        let mut html = format!("<details><summary>{name} ({size})</summary>\n");
        for child in &node.children {
            html.push_str(&build_tree_html(child));
        }
        html.push_str("</details>\n");
        html
    } else {
        let rel_path = escape_html(&path_for_html(&node.rel_path));
        format!(r#"<div><a href="{rel_path}">{name}</a> ({size})</div>"#) + "\n"
    }
}

fn path_for_html(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn fetch_previous_report() -> Option<String> {
    let output = Command::new("curl")
        .args(["-fsSL", "--max-time", "10", "https://hilll.dev/report"])
        .stderr(Stdio::null())
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let remote_html = String::from_utf8_lossy(&output.stdout);
    let reports = extract_marked_reports(&remote_html);
    if reports.len() >= 2 {
        reports.get(reports.len() - 2).cloned()
    } else if let Some(report) = reports.last() {
        Some(report.clone())
    } else {
        extract_body(&remote_html).filter(|body| !body.trim().is_empty())
    }
}

fn extract_marked_reports(html: &str) -> Vec<String> {
    let mut reports = Vec::new();
    let mut offset = 0;

    while let Some(start) = html[offset..].find(REPORT_START) {
        let body_start = offset + start + REPORT_START.len();
        let Some(end) = html[body_start..].find(REPORT_END) else {
            break;
        };
        let body_end = body_start + end;
        reports.push(html[body_start..body_end].to_string());
        offset = body_end + REPORT_END.len();
    }

    reports
}

fn extract_body(html: &str) -> Option<String> {
    let start = html.find("<body>")?;
    let end = html.find("</body>")?;
    Some(html[start + "<body>".len()..end].to_string())
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
