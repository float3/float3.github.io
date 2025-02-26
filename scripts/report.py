#!/usr/bin/env python3
import html
import json
import os
import re
import subprocess
import sys
import urllib.request


def compute_sizes(node, parent_path):
    total_size = 0
    if "contents" in node:
        for child in node["contents"]:
            child_size = compute_sizes(child, os.path.join(parent_path, node["name"]))
            total_size += child_size
        node["size"] = total_size
    else:
        file_path = os.path.join(parent_path, node["name"])
        total_size = os.path.getsize(file_path) if os.path.isfile(file_path) else 0
        node["size"] = total_size
    return total_size


def sort_by_size(node):
    if "contents" in node:
        node["contents"].sort(key=lambda x: x["size"], reverse=True)
        for child in node["contents"]:
            sort_by_size(child)


def build_html(node, parent_path):
    name = html.escape(node["name"])
    if "contents" in node:
        size_mb = f"{(node['size']/1024/1024):.2f} MB"
        s = f"<details><summary>{name} ({size_mb})</summary>\n"
        for child in node["contents"]:
            s += build_html(child, os.path.join(parent_path, node["name"]))
        s += "</details>\n"
        return s
    else:
        size_mb = f"{(node['size']/1024/1024):.2f} MB"
        rel_path = html.escape(os.path.join(parent_path, node["name"]))
        return f'<div><a href="{rel_path}">{name}</a> ({size_mb})</div>\n'


def get_os_version():
    try:
        with open("/etc/os-release", "r", encoding="utf-8") as f:
            for line in f:
                if line.startswith("PRETTY_NAME="):
                    return line.split("=", 1)[1].strip().strip('"')
        return "Unknown OS"
    except FileNotFoundError:
        return "Unknown OS"


def get_version(command, args=["--version"]):
    try:
        result = subprocess.run(
            [command] + args,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
        )
        return result.stdout.strip()
    except FileNotFoundError:
        return "not installed"


def extract_body(html_text):
    start = html_text.find("<body>")
    end = html_text.find("</body>")
    if start != -1 and end != -1:
        return html_text[start + len("<body>") : end]
    return html_text


def fetch_previous_report():
    url = "https://hilll.dev/report"
    try:
        with urllib.request.urlopen(url) as response:
            remote_html = response.read().decode("utf-8")
    except Exception:
        return None

    pattern = re.compile(r"<!-- REPORT START -->(.*?)<!-- REPORT END -->", re.DOTALL)
    reports = pattern.findall(remote_html)
    if not reports:
        body = extract_body(remote_html)
        return body if body.strip() else None
    return reports[-2] if len(reports) >= 2 else reports[-1]


def main():
    # ---- Capture build time from CLI args (if provided) ----
    build_time = 0
    if len(sys.argv) > 1:
        try:
            build_time = int(sys.argv[1])
        except ValueError:
            print("Invalid build time argument. Ignoring.")
    else:
        print("No build time provided")

    # ---- Gather version info ----
    time_str = get_version("date", ["+%x (%A) %X %z"])
    os_version = get_os_version()
    nix_version = get_version("nix")
    quartz_version = get_version("npx", ["quartz", "--version"]).split("\n").pop()
    python_version = get_version("python")
    core_pack = get_version("corepack")
    node_version = get_version("node")
    npm_version = get_version("npm")
    pnpm_version = get_version("pnpm")
    cargo_version = get_version("cargo")
    rustc_version = get_version("rustc")
    rustup_version = get_version("rustup").split("\n").pop(0)
    wasm_pack_version = get_version("wasm-pack")
    tsc_version = get_version("pnpm", ["exec", "tsc", "--version"])
    # Retrieve git commit hash
    git_commit = get_version("git", ["rev-parse", "HEAD"])
    os.chdir("../ts")
    webpack_full = get_version("pnpm", ["exec", "webpack", "--version"])
    webpack_version = (
        webpack_full.split("Packages:")[1].replace("\n", "<br>")
        if "Packages:" in webpack_full
        else webpack_full
    )
    os.chdir("../public")

    # ---- Run 'tree' command to gather folder structure info ----
    folder_to_scan = "."
    result = subprocess.run(
        ["tree", "-J", folder_to_scan], stdout=subprocess.PIPE, text=True
    )
    data = json.loads(result.stdout)
    report_info = None

    # ---- Compute sizes and sort ----
    for top_node in data:
        if top_node["type"] == "report":
            report_info = top_node
        else:
            compute_sizes(top_node, "")
            sort_by_size(top_node)

    # ---- Build current report HTML with markers ----
    current_body = []
    current_body.append("<h1>Build Report</h1>")
    current_body.append(f"<p><strong>Report generated at:</strong> {time_str}</p>")
    if build_time > 0:
        current_body.append(f"<p><strong>Build Time:</strong> {build_time} seconds</p>")
    current_body.append(f"<p><strong>Operating System:</strong> {os_version}</p>")
    current_body.append(f"<p><strong>Nix:</strong> {nix_version}</p>")
    current_body.append(f"<p><strong>Quartz:</strong> {quartz_version}</p>")
    current_body.append(f"<p><strong>Python:</strong> {python_version}</p>")
    current_body.append(f"<p><strong>corepack:</strong> {core_pack}</p>")
    current_body.append(f"<p><strong>Node.js:</strong> {node_version}</p>")
    current_body.append(f"<p><strong>npm:</strong> {npm_version}</p>")
    current_body.append(f"<p><strong>pnpm:</strong> {pnpm_version}</p>")
    current_body.append(f"<p><strong>cargo:</strong> {cargo_version}</p>")
    current_body.append(f"<p><strong>rustc:</strong> {rustc_version}</p>")
    current_body.append(f"<p><strong>rustup:</strong> {rustup_version}</p>")
    current_body.append(f"<p><strong>wasm-pack:</strong> {wasm_pack_version}</p>")
    current_body.append(f"<p><strong>tsc:</strong> {tsc_version}</p>")
    current_body.append(f"<p><strong>webpack:</strong> {webpack_version}</p>")
    # Add git commit hash
    current_body.append(f"<p><strong>Git Commit:</strong> {git_commit}</p>")
    if report_info is not None:
        directories = report_info.get("directories", 0)
        files = report_info.get("files", 0)
        current_body.append(
            f"<h2>Tree Summary</h2><p>Total Directories: {directories}, Total Files: {files}</p>"
        )
    current_body.append("<h2>Folder Contents</h2>")
    for top_node in data:
        if top_node["type"] != "report":
            current_body.append(build_html(top_node, ""))
    current_report_body = "\n".join(current_body)
    current_report = (
        f"<!-- REPORT START -->\n{current_report_body}\n<!-- REPORT END -->"
    )

    # ---- Fetch previous report (if available) ----
    previous_report = fetch_previous_report()

    # ---- Build combined HTML with side-by-side columns ----
    combined_html = f"""<!DOCTYPE html>
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
    {"<div class='report-column'><h2>Previous Report</h2>" + previous_report + "</div>" if previous_report else ""}
  </div>
</body>
</html>
"""

    # ---- Write the combined report to 'report.html' ----
    with open("report.html", "w", encoding="utf-8") as f:
        f.write(combined_html)


if __name__ == "__main__":
    main()
