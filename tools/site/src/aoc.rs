use std::collections::BTreeSet;
use std::env;
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use crate::{Result, Site, SiteError};

const DEFAULT_ROOT: &str = "wasm/adventofcode/src";
const DEFAULT_PROBLEM_YEAR: u16 = 2025;
const DEFAULT_PROBLEM_DAYS: &str = "1-12";
const DEFAULT_INPUT_DAYS: &str = "1-25";
const DEFAULT_PROBLEM_UA: &str =
    "hilll.dev Advent of Code problem downloader (contact: hill@hilll.dev)";
const DEFAULT_INPUT_UA: &str =
    "hilll.dev Advent of Code input downloader (contact: hill@hilll.dev)";
const LOCKED_DAY_MARKER: &str = "Please don't repeatedly request this endpoint before it unlocks";

pub(crate) fn download_problem_text(site: &Site, args: &[String]) -> Result<()> {
    let Some(options) = ProblemOptions::parse(site, args)? else {
        return Ok(());
    };

    let session = env::var(&options.session_env)
        .ok()
        .filter(|value| !value.is_empty());
    if !options.dry_run && options.parts.contains(&2) && session.is_none() {
        return Err(Box::new(SiteError::new(format!(
            "part 2 problem text requires an Advent of Code session cookie. Set {}=... or run with --parts 1.",
            options.session_env
        ))));
    }

    let mut wrote = 0usize;
    let mut skipped = 0usize;
    let mut unscaffolded = 0usize;
    let mut missing = 0usize;
    let mut failed = 0usize;
    let mut first_request = true;

    for year in options.start_year..=options.end_year {
        for &day in &options.days {
            let mut targets = Vec::new();
            for &part in &options.parts {
                let path = problem_path(&options.root, year, day, part);
                if !path.exists() {
                    unscaffolded += 1;
                    continue;
                }
                targets.push((part, path));
            }

            let missing_targets = targets
                .iter()
                .filter(|(_, path)| options.overwrite || !has_content(path))
                .collect::<Vec<_>>();

            if missing_targets.is_empty() {
                skipped += targets.len();
                continue;
            }

            if options.dry_run {
                let names = missing_targets
                    .iter()
                    .map(|(_, path)| path.file_name().unwrap().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("would fetch {year} day {day} for {names}");
                continue;
            }

            wait_between_requests(&mut first_request, options.delay);
            let page = match fetch_page(
                year,
                day,
                session.as_deref(),
                &options.user_agent,
                options.timeout,
            ) {
                Ok(page) => page,
                Err(error) => {
                    eprintln!("failed: {year} day {day}: {error}");
                    failed += missing_targets.len();
                    continue;
                }
            };
            let articles = extract_articles(&page);

            for (part, path) in targets {
                if !options.overwrite && has_content(&path) {
                    skipped += 1;
                    continue;
                }

                let Some(article) = articles.get(usize::from(part - 1)) else {
                    eprintln!("missing part {part}: {year} day {day}");
                    missing += 1;
                    continue;
                };

                fs::write(&path, article)?;
                wrote += 1;
                println!("wrote {}", display_path(site, &path));
            }
        }
    }

    println!(
        "summary: wrote={wrote} skipped={skipped} unscaffolded={unscaffolded} missing={missing} failed={failed}"
    );

    if missing > 0 || failed > 0 {
        Err(Box::new(SiteError::new(
            "one or more AoC problem downloads failed",
        )))
    } else {
        Ok(())
    }
}

pub(crate) fn download_inputs(site: &Site, args: &[String]) -> Result<()> {
    let Some(options) = InputOptions::parse(site, args)? else {
        return Ok(());
    };

    let session = env::var(&options.session_env)
        .ok()
        .filter(|value| !value.is_empty());
    if !options.dry_run && session.is_none() {
        return Err(Box::new(SiteError::new(format!(
            "puzzle inputs require an Advent of Code session cookie. Set {}=...",
            options.session_env
        ))));
    }

    let mut wrote = 0usize;
    let mut skipped = 0usize;
    let mut unscaffolded = 0usize;
    let mut locked = 0usize;
    let mut failed = 0usize;
    let mut first_request = true;

    for year in options.start_year..=options.end_year {
        for &day in &options.days {
            let path = input_path(&options.root, year, day);
            if !path.parent().is_some_and(Path::exists) {
                unscaffolded += 1;
                continue;
            }

            if !options.overwrite && has_content(&path) {
                skipped += 1;
                continue;
            }

            if options.dry_run {
                println!(
                    "would fetch {year} day {day} input -> {}",
                    display_path(site, &path)
                );
                continue;
            }

            wait_between_requests(&mut first_request, options.delay);
            let input_text = match fetch_input(
                year,
                day,
                session.as_deref().unwrap_or_default(),
                &options.user_agent,
                options.timeout,
            ) {
                Ok(input_text) => input_text,
                Err(error) if error.is_locked_day() => {
                    eprintln!("locked: {year} day {day}: {error}");
                    locked += options
                        .days
                        .iter()
                        .filter(|future_day| **future_day >= day)
                        .count();
                    break;
                }
                Err(error) => {
                    eprintln!("failed: {year} day {day}: {error}");
                    failed += 1;
                    continue;
                }
            };

            fs::write(&path, input_text)?;
            wrote += 1;
            println!("wrote {}", display_path(site, &path));
        }
    }

    println!(
        "summary: wrote={wrote} skipped={skipped} unscaffolded={unscaffolded} locked={locked} failed={failed}"
    );

    if failed > 0 {
        Err(Box::new(SiteError::new(
            "one or more AoC input downloads failed",
        )))
    } else {
        Ok(())
    }
}

struct ProblemOptions {
    root: PathBuf,
    start_year: u16,
    end_year: u16,
    days: Vec<u8>,
    parts: Vec<u8>,
    overwrite: bool,
    dry_run: bool,
    delay: Duration,
    timeout: f64,
    session_env: String,
    user_agent: String,
}

struct InputOptions {
    root: PathBuf,
    start_year: u16,
    end_year: u16,
    days: Vec<u8>,
    overwrite: bool,
    dry_run: bool,
    delay: Duration,
    timeout: f64,
    session_env: String,
    user_agent: String,
}

impl ProblemOptions {
    fn parse(site: &Site, args: &[String]) -> Result<Option<Self>> {
        let mut root = PathBuf::from(DEFAULT_ROOT);
        let mut start_year = DEFAULT_PROBLEM_YEAR;
        let mut end_year = DEFAULT_PROBLEM_YEAR;
        let mut days = DEFAULT_PROBLEM_DAYS.to_string();
        let mut parts = "1".to_string();
        let mut overwrite = false;
        let mut dry_run = false;
        let mut delay = 1.0;
        let mut timeout = 30.0;
        let mut session_env = "AOC_SESSION".to_string();
        let mut user_agent = DEFAULT_PROBLEM_UA.to_string();

        let mut index = 0usize;
        while index < args.len() {
            if help_requested(&args[index]) {
                print_problem_help();
                return Ok(None);
            } else if let Some(value) = option_value(args, &mut index, "--root")? {
                root = PathBuf::from(value);
            } else if let Some(value) = option_value(args, &mut index, "--start-year")? {
                start_year = parse_year(&value, "--start-year")?;
            } else if let Some(value) = option_value(args, &mut index, "--end-year")? {
                end_year = parse_year(&value, "--end-year")?;
            } else if let Some(value) = option_value(args, &mut index, "--days")? {
                days = value;
            } else if let Some(value) = option_value(args, &mut index, "--parts")? {
                parts = value;
            } else if let Some(value) = option_value(args, &mut index, "--delay")? {
                delay = parse_nonnegative_f64(&value, "--delay")?;
            } else if let Some(value) = option_value(args, &mut index, "--timeout")? {
                timeout = parse_nonnegative_f64(&value, "--timeout")?;
            } else if let Some(value) = option_value(args, &mut index, "--session-env")? {
                session_env = value;
            } else if let Some(value) = option_value(args, &mut index, "--user-agent")? {
                user_agent = value;
            } else {
                match args[index].as_str() {
                    "--overwrite" => overwrite = true,
                    "--dry-run" => dry_run = true,
                    other => {
                        return Err(Box::new(SiteError::new(format!(
                            "unknown aoc-problems option: {other}"
                        ))))
                    }
                }
            }
            index += 1;
        }

        validate_year_range(start_year, end_year)?;

        Ok(Some(Self {
            root: resolve_root(site, root),
            start_year,
            end_year,
            days: parse_days(&days)?,
            parts: parse_parts(&parts)?,
            overwrite,
            dry_run,
            delay: Duration::from_secs_f64(delay),
            timeout,
            session_env,
            user_agent,
        }))
    }
}

impl InputOptions {
    fn parse(site: &Site, args: &[String]) -> Result<Option<Self>> {
        let mut root = PathBuf::from(DEFAULT_ROOT);
        let mut start_year = None;
        let mut end_year = None;
        let mut days = DEFAULT_INPUT_DAYS.to_string();
        let mut overwrite = false;
        let mut dry_run = false;
        let mut delay = 1.0;
        let mut timeout = 30.0;
        let mut session_env = "AOC_SESSION".to_string();
        let mut user_agent = DEFAULT_INPUT_UA.to_string();

        let mut index = 0usize;
        while index < args.len() {
            if help_requested(&args[index]) {
                print_input_help();
                return Ok(None);
            } else if let Some(value) = option_value(args, &mut index, "--root")? {
                root = PathBuf::from(value);
            } else if let Some(value) = option_value(args, &mut index, "--start-year")? {
                start_year = Some(parse_year(&value, "--start-year")?);
            } else if let Some(value) = option_value(args, &mut index, "--end-year")? {
                end_year = Some(parse_year(&value, "--end-year")?);
            } else if let Some(value) = option_value(args, &mut index, "--days")? {
                days = value;
            } else if let Some(value) = option_value(args, &mut index, "--delay")? {
                delay = parse_nonnegative_f64(&value, "--delay")?;
            } else if let Some(value) = option_value(args, &mut index, "--timeout")? {
                timeout = parse_nonnegative_f64(&value, "--timeout")?;
            } else if let Some(value) = option_value(args, &mut index, "--session-env")? {
                session_env = value;
            } else if let Some(value) = option_value(args, &mut index, "--user-agent")? {
                user_agent = value;
            } else {
                match args[index].as_str() {
                    "--overwrite" => overwrite = true,
                    "--dry-run" => dry_run = true,
                    other => {
                        return Err(Box::new(SiteError::new(format!(
                            "unknown aoc-inputs option: {other}"
                        ))))
                    }
                }
            }
            index += 1;
        }

        let root = resolve_root(site, root);
        let years = discover_years(&root)?;
        let start_year = start_year.unwrap_or_else(|| years.first().copied().unwrap_or(2015));
        let end_year = end_year.unwrap_or_else(|| years.last().copied().unwrap_or(2025));
        validate_year_range(start_year, end_year)?;

        Ok(Some(Self {
            root,
            start_year,
            end_year,
            days: parse_days(&days)?,
            overwrite,
            dry_run,
            delay: Duration::from_secs_f64(delay),
            timeout,
            session_env,
            user_agent,
        }))
    }
}

fn option_value(args: &[String], index: &mut usize, name: &str) -> Result<Option<String>> {
    let arg = &args[*index];
    if arg == name {
        *index += 1;
        return args.get(*index).cloned().map(Some).ok_or_else(|| {
            Box::new(SiteError::new(format!("{name} requires a value")))
                as Box<dyn std::error::Error>
        });
    }

    let prefix = format!("{name}=");
    Ok(arg.strip_prefix(&prefix).map(str::to_string))
}

fn help_requested(arg: &str) -> bool {
    matches!(arg, "--help" | "-h" | "help")
}

fn parse_days(spec: &str) -> Result<Vec<u8>> {
    let mut days = BTreeSet::new();
    for chunk in spec.split(',') {
        let chunk = chunk.trim();
        if chunk.is_empty() {
            continue;
        }

        if let Some((start, end)) = chunk.split_once('-') {
            let start = parse_day(start.trim())?;
            let end = parse_day(end.trim())?;
            if start > end {
                return Err(Box::new(SiteError::new(format!(
                    "invalid AoC day range: {chunk}"
                ))));
            }
            days.extend(start..=end);
        } else {
            days.insert(parse_day(chunk)?);
        }
    }

    Ok(days.into_iter().collect())
}

fn parse_day(value: &str) -> Result<u8> {
    let day = value
        .parse::<u8>()
        .map_err(|source| SiteError::new(format!("invalid AoC day {value:?}: {source}")))?;
    if !(1..=25).contains(&day) {
        return Err(Box::new(SiteError::new(format!("invalid AoC day: {day}"))));
    }
    Ok(day)
}

fn parse_parts(value: &str) -> Result<Vec<u8>> {
    match value {
        "all" => Ok(vec![1, 2]),
        "1" => Ok(vec![1]),
        "2" => Ok(vec![2]),
        other => Err(Box::new(SiteError::new(format!(
            "invalid --parts value: {other}; expected all, 1, or 2"
        )))),
    }
}

fn parse_year(value: &str, option: &str) -> Result<u16> {
    value.parse::<u16>().map_err(|source| {
        Box::new(SiteError::new(format!(
            "invalid {option} value {value:?}: {source}"
        ))) as Box<dyn std::error::Error>
    })
}

fn parse_nonnegative_f64(value: &str, option: &str) -> Result<f64> {
    let parsed = value.parse::<f64>().map_err(|source| {
        Box::new(SiteError::new(format!(
            "invalid {option} value {value:?}: {source}"
        ))) as Box<dyn std::error::Error>
    })?;
    if !parsed.is_finite() || parsed < 0.0 {
        return Err(Box::new(SiteError::new(format!(
            "{option} must be a non-negative finite number"
        ))));
    }
    Ok(parsed)
}

fn validate_year_range(start_year: u16, end_year: u16) -> Result<()> {
    if start_year > end_year {
        return Err(Box::new(SiteError::new(
            "--start-year must be <= --end-year",
        )));
    }
    Ok(())
}

fn discover_years(root: &Path) -> Result<Vec<u16>> {
    let mut years = Vec::new();
    if !root.exists() {
        return Ok(years);
    }

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        if !entry.path().is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        let Some(year) = name.strip_prefix("aoc") else {
            continue;
        };
        if let Ok(year) = year.parse::<u16>() {
            if (2000..=2099).contains(&year) {
                years.push(year);
            }
        }
    }
    years.sort_unstable();
    Ok(years)
}

fn resolve_root(site: &Site, root: PathBuf) -> PathBuf {
    if root.is_absolute() {
        root
    } else {
        site.root.join(root)
    }
}

fn has_content(path: &Path) -> bool {
    path.metadata().is_ok_and(|metadata| metadata.len() > 0)
}

fn problem_path(root: &Path, year: u16, day: u8, part: u8) -> PathBuf {
    root.join(format!("aoc{year}"))
        .join(format!("day{day:02}"))
        .join(format!("problem{part}.txt"))
}

fn input_path(root: &Path, year: u16, day: u8) -> PathBuf {
    root.join(format!("aoc{year}"))
        .join(format!("day{day:02}"))
        .join("input.txt")
}

fn display_path(site: &Site, path: &Path) -> String {
    path.strip_prefix(&site.root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn wait_between_requests(first_request: &mut bool, delay: Duration) {
    if *first_request {
        *first_request = false;
    } else if !delay.is_zero() {
        thread::sleep(delay);
    }
}

fn fetch_page(
    year: u16,
    day: u8,
    session: Option<&str>,
    user_agent: &str,
    timeout: f64,
) -> std::result::Result<String, FetchError> {
    curl_get(
        &format!("https://adventofcode.com/{year}/day/{day}"),
        session,
        user_agent,
        timeout,
    )
    .map_err(|error| error.with_context(format!("{year} day {day}")))
}

fn fetch_input(
    year: u16,
    day: u8,
    session: &str,
    user_agent: &str,
    timeout: f64,
) -> std::result::Result<String, FetchError> {
    curl_get(
        &format!("https://adventofcode.com/{year}/day/{day}/input"),
        Some(session),
        user_agent,
        timeout,
    )
    .map_err(|error| error.with_context(format!("{year} day {day} input")))
}

#[derive(Debug)]
enum FetchError {
    Http {
        code: u16,
        body: String,
        context: String,
    },
    Curl {
        message: String,
        context: String,
    },
}

impl FetchError {
    fn with_context(self, context: String) -> Self {
        match self {
            Self::Http { code, body, .. } => Self::Http {
                code,
                body,
                context,
            },
            Self::Curl { message, .. } => Self::Curl { message, context },
        }
    }

    fn is_locked_day(&self) -> bool {
        matches!(
            self,
            Self::Http {
                code: 404,
                body,
                ..
            } if body.contains(LOCKED_DAY_MARKER)
        )
    }
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http {
                code,
                body,
                context,
            } => write!(f, "HTTP {code} for {context}: {}", preview(body, 180)),
            Self::Curl { message, context } => write!(f, "{context}: {message}"),
        }
    }
}

fn curl_get(
    url: &str,
    session: Option<&str>,
    user_agent: &str,
    timeout: f64,
) -> std::result::Result<String, FetchError> {
    let timeout = timeout.to_string();
    let mut command = Command::new("curl");
    command.args([
        "-sS",
        "-L",
        "--max-time",
        &timeout,
        "-A",
        user_agent,
        "-w",
        "\n%{http_code}",
    ]);

    if session.is_some() {
        command.arg("--config").arg("-");
        command.stdin(Stdio::piped());
    }

    let mut child = command
        .arg(url)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|source| FetchError::Curl {
            message: format!("failed to run curl: {source}"),
            context: String::new(),
        })?;

    if let Some(session) = session {
        let Some(mut stdin) = child.stdin.take() else {
            return Err(FetchError::Curl {
                message: "failed to open curl config stdin".to_string(),
                context: String::new(),
            });
        };
        writeln!(
            stdin,
            "header = \"Cookie: session={}\"",
            curl_config_escape(session)
        )
        .map_err(|source| FetchError::Curl {
            message: format!("failed to write curl config: {source}"),
            context: String::new(),
        })?;
    }

    let output = child
        .wait_with_output()
        .map_err(|source| FetchError::Curl {
            message: format!("failed to read curl output: {source}"),
            context: String::new(),
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if !output.status.success() {
        return Err(FetchError::Curl {
            message: if stderr.is_empty() {
                format!("curl exited with {}", output.status)
            } else {
                stderr
            },
            context: String::new(),
        });
    }

    let Some((body, status)) = stdout.rsplit_once('\n') else {
        return Err(FetchError::Curl {
            message: "curl output did not include an HTTP status".to_string(),
            context: String::new(),
        });
    };
    let code = status
        .trim()
        .parse::<u16>()
        .map_err(|source| FetchError::Curl {
            message: format!("invalid HTTP status from curl: {source}"),
            context: String::new(),
        })?;

    if (200..=299).contains(&code) {
        Ok(body.to_string())
    } else {
        Err(FetchError::Http {
            code,
            body: body.to_string(),
            context: String::new(),
        })
    }
}

fn curl_config_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn preview(text: &str, limit: usize) -> String {
    let mut preview = text.chars().take(limit).collect::<String>();
    preview = preview.replace(['\n', '\r'], " ");
    preview.trim().to_string()
}

fn extract_articles(html: &str) -> Vec<String> {
    let mut articles = Vec::new();
    let mut offset = 0usize;

    while let Some(start) = html[offset..].find("<article") {
        let article_start = offset + start;
        let Some(tag_end_offset) = html[article_start..].find('>') else {
            break;
        };
        let tag_end = article_start + tag_end_offset;
        let start_tag = &html[article_start..=tag_end];
        offset = tag_end + 1;

        if !start_tag.contains("day-desc") {
            continue;
        }

        let Some(end_offset) = html[offset..].find("</article>") else {
            break;
        };
        let article_end = offset + end_offset;
        let text = html_fragment_to_text(&html[offset..article_end]);
        articles.push(format!("{}\n", trim_blank_edges(&text)));
        offset = article_end + "</article>".len();
    }

    articles
}

fn html_fragment_to_text(fragment: &str) -> String {
    let mut lines = Vec::new();
    let mut current = String::new();
    let mut in_pre = false;
    let mut offset = 0usize;

    while offset < fragment.len() {
        if let Some(tag_start_offset) = fragment[offset..].find('<') {
            let data_end = offset + tag_start_offset;
            append_html_data(
                &fragment[offset..data_end],
                in_pre,
                &mut current,
                &mut lines,
            );

            let tag_start = data_end;
            let Some(tag_end_offset) = fragment[tag_start..].find('>') else {
                append_html_data(&fragment[tag_start..], in_pre, &mut current, &mut lines);
                break;
            };
            let tag_end = tag_start + tag_end_offset;
            let raw_tag = fragment[tag_start + 1..tag_end].trim();
            handle_tag(raw_tag, &mut in_pre, &mut current, &mut lines);
            offset = tag_end + 1;
        } else {
            append_html_data(&fragment[offset..], in_pre, &mut current, &mut lines);
            break;
        }
    }

    finish_line(&mut current, &mut lines, false);
    trim_blank_edges(&lines.join("\n"))
}

fn handle_tag(raw_tag: &str, in_pre: &mut bool, current: &mut String, lines: &mut Vec<String>) {
    let closing = raw_tag.starts_with('/');
    let tag = raw_tag
        .trim_start_matches('/')
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim_end_matches('/')
        .to_ascii_lowercase();

    if closing {
        match tag.as_str() {
            "h2" | "p" | "pre" => {
                finish_line(current, lines, true);
                if tag == "pre" {
                    *in_pre = false;
                }
            }
            "li" => finish_line(current, lines, false),
            "ul" | "ol" => finish_line(current, lines, true),
            _ => {}
        }
    } else {
        match tag.as_str() {
            "h2" | "p" | "pre" => {
                finish_line(current, lines, false);
                if tag == "pre" {
                    *in_pre = true;
                }
            }
            "li" => {
                finish_line(current, lines, false);
                current.push_str("    ");
            }
            "br" => finish_line(current, lines, false),
            _ => {}
        }
    }
}

fn append_html_data(data: &str, in_pre: bool, current: &mut String, lines: &mut Vec<String>) {
    let data = decode_html_entities(data);
    if in_pre {
        for (index, line) in data.lines().enumerate() {
            if index > 0 {
                finish_line(current, lines, false);
            }
            current.push_str(line.trim_end());
        }
        return;
    }

    let text = collapse_whitespace(&data);
    if text.is_empty() {
        return;
    }

    if !current.is_empty()
        && !current.ends_with(' ')
        && !current.ends_with('\n')
        && !starts_with_closing_punctuation(&text)
    {
        current.push(' ');
    }
    current.push_str(&text);
}

fn starts_with_closing_punctuation(text: &str) -> bool {
    text.chars()
        .next()
        .is_some_and(|ch| matches!(ch, '.' | ',' | ';' | ':' | '!' | '?' | ')' | ']' | '}'))
}

fn finish_line(current: &mut String, lines: &mut Vec<String>, blank_after: bool) {
    let line = current.trim_end();
    if !line.is_empty() {
        lines.push(line.to_string());
    }
    current.clear();
    if blank_after && lines.last().is_none_or(|line| !line.is_empty()) {
        lines.push(String::new());
    }
}

fn trim_blank_edges(text: &str) -> String {
    let lines = text.lines().map(str::to_string).collect::<Vec<_>>();
    trim_blank_line_vec(&lines).join("\n").trim().to_string()
}

fn trim_blank_line_vec(lines: &[String]) -> &[String] {
    let mut start = 0usize;
    let mut end = lines.len();
    while start < end && lines[start].is_empty() {
        start += 1;
    }
    while end > start && lines[end - 1].is_empty() {
        end -= 1;
    }
    &lines[start..end]
}

fn collapse_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn decode_html_entities(value: &str) -> String {
    let mut decoded = String::with_capacity(value.len());
    let mut chars = value.char_indices().peekable();

    while let Some((index, ch)) = chars.next() {
        if ch != '&' {
            decoded.push(ch);
            continue;
        }

        let Some(end_offset) = value[index..].find(';') else {
            decoded.push('&');
            continue;
        };
        if end_offset > 16 {
            decoded.push('&');
            continue;
        }

        let entity = &value[index + 1..index + end_offset];
        if let Some(decoded_entity) = decode_entity(entity) {
            decoded.push(decoded_entity);
            while chars
                .peek()
                .is_some_and(|(next_index, _)| *next_index <= index + end_offset)
            {
                chars.next();
            }
        } else {
            decoded.push('&');
        }
    }

    decoded
}

fn decode_entity(entity: &str) -> Option<char> {
    match entity {
        "amp" => Some('&'),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "quot" => Some('"'),
        "apos" | "#39" => Some('\''),
        "nbsp" => Some(' '),
        _ if entity.starts_with("#x") || entity.starts_with("#X") => {
            u32::from_str_radix(&entity[2..], 16)
                .ok()
                .and_then(char::from_u32)
        }
        _ if entity.starts_with('#') => entity[1..].parse::<u32>().ok().and_then(char::from_u32),
        _ => None,
    }
}

fn print_problem_help() {
    println!(
        "\
Usage:
  site aoc-problems [options]

Options:
  --root PATH           AoC source root (default: wasm/adventofcode/src)
  --start-year YEAR     First year (default: 2025)
  --end-year YEAR       Last year (default: 2025)
  --days SPEC           Day range/list, e.g. 1-12 or 1,2,5 (default: 1-12)
  --parts all|1|2       Problem parts to download (default: 1)
  --overwrite           Rewrite non-empty problem files
  --dry-run             Print planned downloads without fetching
  --delay SECONDS       Delay between HTTP requests (default: 1)
  --timeout SECONDS     Curl timeout per request (default: 30)
  --session-env NAME    Environment variable for AoC session (default: AOC_SESSION)
  --user-agent VALUE    Request User-Agent
"
    );
}

fn print_input_help() {
    println!(
        "\
Usage:
  site aoc-inputs [options]

Options:
  --root PATH           AoC source root (default: wasm/adventofcode/src)
  --start-year YEAR     First year (default: discovered scaffold minimum)
  --end-year YEAR       Last year (default: discovered scaffold maximum)
  --days SPEC           Day range/list, e.g. 1-25 or 1,2,5 (default: 1-25)
  --overwrite           Rewrite non-empty input files
  --dry-run             Print planned downloads without fetching
  --delay SECONDS       Delay between HTTP requests (default: 1)
  --timeout SECONDS     Curl timeout per request (default: 30)
  --session-env NAME    Environment variable for AoC session (default: AOC_SESSION)
  --user-agent VALUE    Request User-Agent
"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_day_specs() {
        assert_eq!(parse_days("1,3-5").unwrap(), vec![1, 3, 4, 5]);
        assert!(parse_days("0").is_err());
        assert!(parse_days("8-2").is_err());
    }

    #[test]
    fn extracts_problem_articles() {
        let html = r#"
            <article class="day-desc"><h2>--- Day 1 ---</h2><p>Hello <code>&lt;x&gt;</code>.</p><pre><code>a
b</code></pre></article>
            <article class="day-desc"><p>Second&nbsp;part</p></article>
        "#;

        assert_eq!(
            extract_articles(html),
            vec![
                "--- Day 1 ---\n\nHello <x>.\n\na\nb\n".to_string(),
                "Second part\n".to_string()
            ]
        );
    }
}
