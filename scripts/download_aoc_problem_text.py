#!/usr/bin/env python3
"""Download Advent of Code problem statements into wasm/adventofcode.

The Advent of Code site exposes part 1 publicly, but part 2 requires a
logged-in session that has unlocked the second part for that day. Provide the
session cookie via AOC_SESSION; the script never prints it. By default this
targets only the 12 available 2025 problem statements.
"""

from __future__ import annotations

import argparse
import os
import re
import sys
import time
import urllib.error
import urllib.request
from html.parser import HTMLParser
from pathlib import Path


DEFAULT_ROOT = Path("wasm/adventofcode/src")
DEFAULT_DOWNLOAD_YEAR = 2025
DEFAULT_DAYS = "1-12"
DEFAULT_UA = "hilll.dev Advent of Code problem downloader (contact: hill@hilll.dev)"


class AoCArticleParser(HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.articles: list[str] = []
        self._in_article = False
        self._in_pre = False
        self._lines: list[str] = []
        self._current = ""

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        attrs_dict = dict(attrs)
        classes = set((attrs_dict.get("class") or "").split())

        if tag == "article" and "day-desc" in classes:
            self._in_article = True
            self._in_pre = False
            self._lines = []
            self._current = ""
            return

        if not self._in_article:
            return

        if tag in {"h2", "p", "pre"}:
            self._finish_line(blank_after=False)
            if tag == "pre":
                self._in_pre = True
        elif tag == "li":
            self._finish_line(blank_after=False)
            self._current = "    "
        elif tag == "br":
            self._finish_line(blank_after=False)

    def handle_endtag(self, tag: str) -> None:
        if not self._in_article:
            return

        if tag in {"h2", "p", "pre"}:
            self._finish_line(blank_after=True)
            if tag == "pre":
                self._in_pre = False
        elif tag == "li":
            self._finish_line(blank_after=False)
        elif tag in {"ul", "ol"}:
            self._finish_line(blank_after=True)
        elif tag == "article":
            self._finish_line(blank_after=False)
            text = "\n".join(self._trim_blank_edges(self._lines)).strip()
            self.articles.append(text + "\n")
            self._in_article = False
            self._in_pre = False
            self._lines = []
            self._current = ""

    def handle_data(self, data: str) -> None:
        if not self._in_article:
            return

        if self._in_pre:
            for idx, line in enumerate(data.splitlines()):
                if idx > 0:
                    self._finish_line(blank_after=False)
                self._current += line.rstrip()
            return

        text = re.sub(r"\s+", " ", data)
        if not text.strip():
            return

        if self._current and not self._current.endswith((" ", "\n")) and not text.startswith(" "):
            self._current += " "
        self._current += text

    def _finish_line(self, *, blank_after: bool) -> None:
        line = self._current.rstrip()
        if line:
            self._lines.append(line)
        self._current = ""
        if blank_after and (not self._lines or self._lines[-1] != ""):
            self._lines.append("")

    @staticmethod
    def _trim_blank_edges(lines: list[str]) -> list[str]:
        start = 0
        end = len(lines)
        while start < end and lines[start] == "":
            start += 1
        while end > start and lines[end - 1] == "":
            end -= 1
        return lines[start:end]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=DEFAULT_ROOT)
    parser.add_argument("--start-year", type=int, default=DEFAULT_DOWNLOAD_YEAR)
    parser.add_argument("--end-year", type=int, default=DEFAULT_DOWNLOAD_YEAR)
    parser.add_argument("--days", default=DEFAULT_DAYS, help="Day range/list, e.g. 1-12 or 1,2,5")
    parser.add_argument("--parts", choices=("all", "1", "2"), default="1")
    parser.add_argument("--overwrite", action="store_true", help="Rewrite non-empty problem files")
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--delay", type=float, default=1.0, help="Seconds between HTTP requests")
    parser.add_argument("--timeout", type=float, default=30.0)
    parser.add_argument("--session-env", default="AOC_SESSION")
    parser.add_argument("--user-agent", default=DEFAULT_UA)
    return parser.parse_args()


def parse_days(spec: str) -> list[int]:
    days: set[int] = set()
    for chunk in spec.split(","):
        chunk = chunk.strip()
        if not chunk:
            continue
        if "-" in chunk:
            start_s, end_s = chunk.split("-", 1)
            start, end = int(start_s), int(end_s)
            days.update(range(start, end + 1))
        else:
            days.add(int(chunk))

    invalid = sorted(day for day in days if day < 1 or day > 25)
    if invalid:
        raise ValueError(f"invalid AoC day(s): {invalid}")
    return sorted(days)


def selected_parts(parts: str) -> list[int]:
    if parts == "all":
        return [1, 2]
    return [int(parts)]


def fetch_page(year: int, day: int, session: str | None, user_agent: str, timeout: float) -> str:
    request = urllib.request.Request(
        f"https://adventofcode.com/{year}/day/{day}",
        headers={"User-Agent": user_agent},
    )
    if session:
        request.add_header("Cookie", f"session={session}")

    try:
        with urllib.request.urlopen(request, timeout=timeout) as response:
            return response.read().decode("utf-8")
    except urllib.error.HTTPError as error:
        body = error.read().decode("utf-8", errors="replace")
        raise RuntimeError(f"HTTP {error.code} for {year} day {day}: {body[:160]}") from error


def extract_articles(html: str) -> list[str]:
    parser = AoCArticleParser()
    parser.feed(html)
    return parser.articles


def target_path(root: Path, year: int, day: int, part: int) -> Path:
    return root / f"aoc{year}" / f"day{day:02d}" / f"problem{part}.txt"


def main() -> int:
    args = parse_args()
    days = parse_days(args.days)
    parts = selected_parts(args.parts)
    session = os.environ.get(args.session_env)

    if not args.dry_run and 2 in parts and not session:
        print(
            f"error: part 2 problem text requires an Advent of Code session cookie. "
            f"Set {args.session_env}=... or run with --parts 1.",
            file=sys.stderr,
        )
        return 2

    wrote = skipped = unscaffolded = missing = failed = 0
    first_request = True

    for year in range(args.start_year, args.end_year + 1):
        for day in days:
            targets = []
            for part in parts:
                path = target_path(args.root, year, day, part)
                if not path.exists():
                    unscaffolded += 1
                    continue
                targets.append((part, path))

            missing_targets = [
                path for _, path in targets if args.overwrite or path.stat().st_size == 0
            ]
            if not missing_targets:
                skipped += len(targets)
                continue

            if args.dry_run:
                print(f"would fetch {year} day {day} for {', '.join(path.name for path in missing_targets)}")
                continue

            if not first_request and args.delay > 0:
                time.sleep(args.delay)
            first_request = False

            try:
                articles = extract_articles(fetch_page(year, day, session, args.user_agent, args.timeout))
            except Exception as error:  # noqa: BLE001 - report and continue all requested days.
                print(f"failed: {year} day {day}: {error}", file=sys.stderr)
                failed += len(missing_targets)
                continue

            for part, path in targets:
                if not (args.overwrite or path.stat().st_size == 0):
                    skipped += 1
                    continue

                if len(articles) < part:
                    print(f"missing part {part}: {year} day {day}", file=sys.stderr)
                    missing += 1
                    continue

                path.write_text(articles[part - 1], encoding="utf-8")
                wrote += 1
                print(f"wrote {path}")

    print(
        f"summary: wrote={wrote} skipped={skipped} "
        f"unscaffolded={unscaffolded} missing={missing} failed={failed}"
    )
    return 1 if missing or failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
