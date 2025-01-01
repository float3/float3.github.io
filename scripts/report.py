#!/usr/bin/env python3

import html
import json
import os
import subprocess


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


def main():
    folder_to_scan = "."
    result = subprocess.run(
        ["tree", "-J", folder_to_scan], stdout=subprocess.PIPE, text=True
    )
    data = json.loads(result.stdout)
    report_info = None

    for top_node in data:
        if top_node["type"] == "report":
            report_info = top_node
        else:
            compute_sizes(top_node, "")
            sort_by_size(top_node)

    html_output = """<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8"/>
<title>Folder Structure Report</title>
<style>
details { margin-left: 20px; }
div { margin-left: 40px; }
</style>
</head>
<body>
<h1>Folder Structure Report</h1>
"""

    if report_info is not None:
        directories = report_info.get("directories", 0)
        files = report_info.get("files", 0)
        html_output += (
            f"<p>Total Directories: {directories}, Total Files: {files}</p>\n"
        )

    for top_node in data:
        if top_node["type"] != "report":
            html_output += build_html(top_node, "")

    html_output += "</body></html>"

    with open("report.html", "w", encoding="utf-8") as f:
        f.write(html_output)


if __name__ == "__main__":
    main()
