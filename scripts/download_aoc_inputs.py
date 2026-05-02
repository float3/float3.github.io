#!/usr/bin/env python3
"""Download Advent of Code puzzle inputs into wasm/adventofcode.

Puzzle inputs are user-specific and require a logged-in Advent of Code session
cookie. Provide it via AOC_SESSION; the script never prints it. By default this
targets every scaffolded AoC year under wasm/adventofcode/src.
"""

from __future__ import annotations

import argparse
import os
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path


DEFAULT_ROOT = Path("wasm/adventofcode/src")
DEFAULT_UA = "hilll.dev Advent of Code input downloader (contact: hill@hilll.dev)"
LOCKED_DAY_MARKER = "Please don't repeatedly request this endpoint before it unlocks"


class LockedDayError(RuntimeError):
    pass


def discover_years(root: Path) -> list[int]:
    return sorted(
        int(path.name.removeprefix("aoc"))
        for path in root.glob("aoc20*")
        if path.is_dir() and path.name.removeprefix("aoc").isdigit()
    )


def parse_args() -> argparse.Namespace:
    existing_years = discover_years(DEFAULT_ROOT)
    default_start = min(existing_years, default=2015)
    default_end = max(existing_years, default=2025)

    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=DEFAULT_ROOT)
    parser.add_argument("--start-year", type=int, default=default_start)
    parser.add_argument("--end-year", type=int, default=default_end)
    parser.add_argument("--days", default="1-25", help="Day range/list, e.g. 1-25 or 1,2,5")
    parser.add_argument("--overwrite", action="store_true", help="Rewrite non-empty input files")
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


def fetch_input(year: int, day: int, session: str, user_agent: str, timeout: float) -> str:
    request = urllib.request.Request(
        f"https://adventofcode.com/{year}/day/{day}/input",
        headers={
            "Cookie": f"session={session}",
            "User-Agent": user_agent,
        },
    )

    try:
        with urllib.request.urlopen(request, timeout=timeout) as response:
            return response.read().decode("utf-8")
    except urllib.error.HTTPError as error:
        body = error.read().decode("utf-8", errors="replace")
        message = f"HTTP {error.code} for {year} day {day} input: {body[:180].strip()}"
        if error.code == 404 and LOCKED_DAY_MARKER in body:
            raise LockedDayError(message) from error
        raise RuntimeError(message) from error


def target_path(root: Path, year: int, day: int) -> Path:
    return root / f"aoc{year}" / f"day{day:02d}" / "input.txt"


def main() -> int:
    args = parse_args()
    days = parse_days(args.days)
    session = os.environ.get(args.session_env)

    if args.start_year > args.end_year:
        print("error: --start-year must be <= --end-year", file=sys.stderr)
        return 2

    if not args.dry_run and not session:
        print(
            f"error: puzzle inputs require an Advent of Code session cookie. "
            f"Set {args.session_env}=...",
            file=sys.stderr,
        )
        return 2

    wrote = skipped = unscaffolded = locked = failed = 0
    first_request = True

    for year in range(args.start_year, args.end_year + 1):
        for day in days:
            path = target_path(args.root, year, day)
            if not path.parent.exists():
                unscaffolded += 1
                continue

            if not args.overwrite and path.exists() and path.stat().st_size > 0:
                skipped += 1
                continue

            if args.dry_run:
                print(f"would fetch {year} day {day} input -> {path}")
                continue

            if not first_request and args.delay > 0:
                time.sleep(args.delay)
            first_request = False

            try:
                input_text = fetch_input(year, day, session or "", args.user_agent, args.timeout)
            except LockedDayError as error:
                print(f"locked: {year} day {day}: {error}", file=sys.stderr)
                locked += len([future_day for future_day in days if future_day >= day])
                break
            except Exception as error:  # noqa: BLE001 - report and continue all requested days.
                print(f"failed: {year} day {day}: {error}", file=sys.stderr)
                failed += 1
                continue

            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(input_text, encoding="utf-8")
            wrote += 1
            print(f"wrote {path}")

    print(
        f"summary: wrote={wrote} skipped={skipped} "
        f"unscaffolded={unscaffolded} locked={locked} failed={failed}"
    )
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
