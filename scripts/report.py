#!/usr/bin/env python3

import html
import json
import os
import subprocess
import sys


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
    """
    Attempt to extract OS version info from /etc/os-release (commonly found on many Linux distros).
    Fallback if the file doesn't exist or doesn't contain expected info.
    """
    try:
        with open("/etc/os-release", "r", encoding="utf-8") as f:
            for line in f:
                if line.startswith("PRETTY_NAME="):
                    # Example line: PRETTY_NAME="Ubuntu 20.04.2 LTS"
                    return line.split("=", 1)[1].strip().strip('"')
        return "Unknown OS"
    except FileNotFoundError:
        return "Unknown OS"


def get_command(command, args=["--version"]):
    """
    Returns the version output of a command.
    If the command is not found, returns 'not installed'.
    """
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


def main():
    # ---- Capture build time from CLI args (if provided) ----
    build_time = 0
    if len(sys.argv) > 1:
        try:
            build_time = int(sys.argv[1])
        except ValueError:
            print("Invalid build time argument. Ignoring.")
            pass
    else:
        print("No build time provided")

    # ---- Gather version info ----
    time = get_command("date", ["+%x (%A) %X %z"])
    os_version = get_os_version()
    quartz_version = get_command("npx", ["quartz", "--version"]).split("\n").pop()
    python_version = get_command("python")
    core_pack = get_command("corepack")
    node_version = get_command("node")
    npm_version = get_command("npm")
    pnpm_version = get_command("pnpm")
    cargo_version = get_command("cargo")
    rustc_version = get_command("rustc")
    rustup_version = get_command("rustup").split("\n").pop(0)
    wasm_pack_version = get_command("wasm-pack")
    tsc_version = get_command("pnpm", ["exec", "tsc", "--version"])
    os.chdir("../wasm/adventofcode/ts")
    webpack_version = (
        get_command("pnpm", ["exec", "webpack", "--version"])
        .split("Packages:")[1]
        .replace("\n", "<br>")
    )
    os.chdir("../../../public")

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

    # ---- Build HTML output ----
    html_output = """<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8"/>
<title>Build Report</title>
<style>
details { margin-left: 20px; }
div { margin-left: 40px; }
</style>
</head>
<body>
<h1>Build Report</h1>
"""

    # ---- Insert version info block ----
    html_output += f"<p><strong>Report generated at:</strong> {time}</p>\n"
    if build_time > 0:
        html_output += f"<p><strong>Build Time:</strong> {build_time} seconds</p>\n"
    html_output += f"<p><strong>Operating System:</strong> {os_version}</p>\n"
    html_output += f"<p><strong>Quartz:</strong> {quartz_version}</p>\n"
    html_output += f"<p><strong>Python:</strong> {python_version}</p>\n"
    html_output += f"<p><strong>corepack:</strong> {core_pack}</p>\n"
    html_output += f"<p><strong>Node.js:</strong> {node_version}</p>\n"
    html_output += f"<p><strong>npm:</strong> {npm_version}</p>\n"
    html_output += f"<p><strong>pnpm:</strong> {pnpm_version}</p>\n"
    html_output += f"<p><strong>cargo:</strong> {cargo_version}</p>\n"
    html_output += f"<p><strong>rustc:</strong> {rustc_version}</p>\n"
    html_output += f"<p><strong>rustup:</strong> {rustup_version}</p>\n"
    html_output += f"<p><strong>wasm-pack:</strong> {wasm_pack_version}</p>\n"
    html_output += f"<p><strong>tsc:</strong> {tsc_version}</p>\n"
    html_output += f"<p><strong>webpack:</strong> {webpack_version}</p>\n"

    # ---- If report_info is available, show directory/file counts ----
    if report_info is not None:
        directories = report_info.get("directories", 0)
        files = report_info.get("files", 0)
        html_output += f"<h2>Tree Summary</h2><p>Total Directories: {directories}, Total Files: {files}</p>\n"

    # ---- Build the tree structure HTML ----
    html_output += "<h2>Folder Contents</h2>\n"
    for top_node in data:
        if top_node["type"] != "report":
            html_output += build_html(top_node, "")

    html_output += "</body></html>"

    # ---- Write the report to 'report.html' ----
    with open("report.html", "w", encoding="utf-8") as f:
        f.write(html_output)


if __name__ == "__main__":
    main()
