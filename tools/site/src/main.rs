mod aoc;
mod report;
mod tables;

use std::env;
use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub(crate) struct SiteError(String);

impl SiteError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for SiteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for SiteError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Dev,
    Prod,
}

impl Mode {
    fn webpack(self) -> &'static str {
        match self {
            Mode::Dev => "development",
            Mode::Prod => "production",
        }
    }

    fn default_for(ci: bool) -> Self {
        if ci {
            Mode::Prod
        } else {
            Mode::Dev
        }
    }
}

pub(crate) struct Site {
    pub(crate) root: PathBuf,
    ci: bool,
}

impl Site {
    fn new() -> Result<Self> {
        Ok(Self {
            root: find_repo_root()?,
            ci: env::var("GITHUB_ACTIONS").is_ok_and(|value| value == "true"),
        })
    }

    fn build(&self, mode: Mode) -> Result<()> {
        let started = Instant::now();

        if mode == Mode::Dev {
            self.warn("building in development mode");
        }

        remove_dir_if_exists(&self.root.join("content/js"))?;
        self.wasm(mode)?;
        remove_license_files(&self.root.join("content/js"))?;

        self.pnpm_install(&self.root, InstallMode::Locked)?;

        let mut args = os_args(&["quartz", "build"]);
        if mode == Mode::Dev {
            args.push("--serve".into());
        }
        self.run(&self.root, "pnpm", &args)?;

        let public = self.root.join("public");
        report::write(self, &public, started.elapsed().as_secs())
    }

    fn wasm(&self, mode: Mode) -> Result<()> {
        let wasm_dir = self.root.join("wasm/wasm");
        let mut wasm_args = os_args(&["build", "--target", "bundler"]);

        match mode {
            Mode::Prod => {
                wasm_args.push("--release".into());
            }
            Mode::Dev => {
                wasm_args.push("--dev".into());
                self.warn("building wasm in development mode");
            }
        }

        wasm_args.extend(os_args(&["--features", "console_error_panic_hook"]));

        self.run_with_env(
            &wasm_dir,
            "wasm-pack",
            &wasm_args,
            &[("RUSTFLAGS", r#"--cfg getrandom_backend="wasm_js""#)],
        )?;

        let ts_dir = self.root.join("ts");
        self.pnpm_install(&ts_dir, InstallMode::Locked)?;
        self.run(&ts_dir, "pnpm", &os_args(&["exec", "tsc"]))?;
        self.run(
            &ts_dir,
            "pnpm",
            &os_args(&[
                "exec",
                "webpack",
                "--config",
                "webpack.config.ts",
                "--mode",
                mode.webpack(),
            ]),
        )
    }

    fn generate(&self) -> Result<()> {
        self.links()?;
        self.indices()?;
        self.generate_chords()?;
        self.dates("content")
    }

    fn links(&self) -> Result<()> {
        let output_dir = self.root.join("content/misc/plaintext");
        fs::create_dir_all(&output_dir)?;

        self.collect_links(
            "content/notes/talks.md",
            "content/misc/plaintext/talks.txt",
            false,
        )?;
        self.collect_links(
            "content/notes/blogs.md",
            "content/misc/plaintext/blogs.txt",
            false,
        )?;
        self.collect_links(
            "content/notes/graphics-resources.md",
            "content/misc/plaintext/graphics-resources.txt",
            true,
        )
    }

    fn collect_links(&self, input: &str, output: &str, unique: bool) -> Result<()> {
        let source = fs::read_to_string(self.root.join(input))?;
        let mut links = extract_urls(&source);

        if unique {
            links.sort();
            links.dedup();
        }

        let mut body = String::new();
        for link in links {
            body.push_str(&link);
            body.push('\n');
        }

        fs::write(self.root.join(output), body)?;
        Ok(())
    }

    fn indices(&self) -> Result<()> {
        self.generate_index("media", "media")?;
        self.generate_index("blobs", "blobs")?;
        self.generate_index("plaintext", "plaintext")?;
        let trolley_count = self.generate_index("trolley", "trolley")?;
        self.update_trolley_count(trolley_count.saturating_sub(1))
    }

    fn generate_index(&self, dir: &str, title: &str) -> Result<usize> {
        let base = self.root.join("content/misc").join(dir);
        let mut entries = Vec::new();

        for entry in fs::read_dir(&base)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().into_owned();
            if name != "index.md" {
                entries.push(name);
            }
        }

        entries.sort();

        let existing_dates = existing_date_metadata(&base.join("index.md"))?;
        let mut body = format!("---\ntitle: {title}\n");
        for line in existing_dates {
            body.push_str(&line);
            body.push('\n');
        }
        body.push_str("tags:\n  - list\n---\n\n");
        for (index, entry) in entries.iter().enumerate() {
            let continuation = if index + 1 == entries.len() {
                ""
            } else {
                " \\"
            };
            body.push_str(&format!("[{entry}](/misc/{dir}/{entry}){continuation}\n"));
        }

        fs::write(base.join("index.md"), body)?;
        Ok(entries.len())
    }

    fn update_trolley_count(&self, count: usize) -> Result<()> {
        let path = self.root.join("ts/src/trolley.ts");
        let source = fs::read_to_string(&path)?;
        let replacement = format!("const NUM = {count}");
        let mut replaced = false;
        let mut body = String::new();

        for line in source.lines() {
            if !replaced && line.starts_with("const NUM =") {
                body.push_str(&replacement);
                replaced = true;
            } else {
                body.push_str(line);
            }
            body.push('\n');
        }

        if !replaced {
            body = format!("{replacement}\n{body}");
        }

        fs::write(path, body)?;
        Ok(())
    }

    fn generate_chords(&self) -> Result<()> {
        let dir = self.root.join("wasm/tuningplayground");
        let venv = dir.join("venv");
        self.run(&dir, "python", &os_args(&["-m", "venv", "venv"]))?;

        let python = if cfg!(windows) {
            venv.join("Scripts/python.exe")
        } else {
            venv.join("bin/python")
        };

        self.run(
            &dir,
            &python,
            &os_args(&["-m", "pip", "install", "--upgrade", "pip"]),
        )?;
        self.run(
            &dir,
            &python,
            &os_args(&["-m", "pip", "install", "-r", "music21/requirements.txt"]),
        )?;
        self.run(&dir, &python, &os_args(&["-m", "generate_chords"]))
    }

    fn dates(&self, target: &str) -> Result<()> {
        if self.ci {
            let shallow = self.output_optional(
                &self.root,
                "git",
                &os_args(&["rev-parse", "--is-shallow-repository"]),
            )?;
            if shallow.as_deref() == Some("true") {
                self.run(&self.root, "git", &os_args(&["fetch", "--unshallow"]))?;
            }
        }

        let mut files = Vec::new();
        collect_markdown_files(&self.root.join(target), &mut files)?;
        files.sort();

        for file in files {
            self.update_dates_for_file(&file)?;
        }

        Ok(())
    }

    fn update_dates_for_file(&self, file: &Path) -> Result<()> {
        let relative = self.relative_git_path(file)?;
        let created = self.git_iso_date(&os_args(&[
            "log",
            "--diff-filter=A",
            "--follow",
            "--format=%aI",
            "-1",
            "--",
            relative.as_str(),
        ]))?;

        let Some(created) = created else {
            self.warn(&format!("skipping untracked markdown file: {relative}"));
            return Ok(());
        };

        let updated = self
            .git_iso_date(&os_args(&[
                "log",
                "--invert-grep",
                "--grep=generate",
                "-1",
                "--format=%aI",
                "--",
                relative.as_str(),
            ]))?
            .unwrap_or_else(|| created.clone());

        let source = fs::read_to_string(file)?;
        let mut body = String::new();
        let mut inserted = false;

        for line in source.lines() {
            if line.starts_with("date:") || line.starts_with("updated:") {
                continue;
            }

            body.push_str(line);
            body.push('\n');

            if !inserted && line.starts_with("title:") {
                body.push_str(&format!("date: {created}\nupdated: {updated}\n"));
                inserted = true;
            }
        }

        if !inserted {
            body = format!("date: {created}\nupdated: {updated}\n\n{body}");
        }

        fs::write(file, body)?;
        Ok(())
    }

    fn update(&self) -> Result<()> {
        self.run(
            &self.root,
            "git",
            &os_args(&[
                "submodule",
                "update",
                "--remote",
                "--recursive",
                "--",
                "wasm/tuningplayground/music21",
            ]),
        )?;

        self.run_with_env(
            &self.root.join("wasm/wasm"),
            "wasm-pack",
            &os_args(&[
                "build",
                "--target",
                "bundler",
                if self.ci { "--release" } else { "--dev" },
            ]),
            &[("RUSTFLAGS", r#"--cfg getrandom_backend="wasm_js""#)],
        )?;

        self.run(&self.root, "pnpm", &os_args(&["update"]))?;
        self.run(&self.root, "pnpm", &os_args(&["audit", "fix"]))?;
        self.pnpm_install(&self.root, InstallMode::Unlocked)?;

        self.node_update(&self.root.join("ts"), "src")?;

        for crate_dir in [
            "wasm/tuningplayground",
            "wasm/tuningplayground/tuning_systems",
            "wasm/tuningplayground/keymapping",
            "wasm/textprocessing",
            "wasm/textprocessing/hangeul_conversion",
            "wasm/glsl2hlsl",
            "wasm/adventofcode",
            "wasm/wasm",
        ] {
            self.cargo_update(&self.root.join(crate_dir))?;
        }

        Ok(())
    }

    fn node_update(&self, dir: &Path, lint_target: &str) -> Result<()> {
        self.run(dir, "pnpm", &os_args(&["update"]))?;
        self.run(dir, "pnpm", &os_args(&["audit", "fix"]))?;
        self.pnpm_install(dir, InstallMode::Unlocked)?;
        self.run(
            dir,
            "pnpm",
            &os_args(&["exec", "prettier", lint_target, "--write"]),
        )?;
        self.run(
            dir,
            "pnpm",
            &os_args(&["exec", "eslint", lint_target, "--fix"]),
        )
    }

    fn cargo_update(&self, dir: &Path) -> Result<()> {
        self.run(dir, "cargo", &os_args(&["upgrade"]))?;
        self.run(dir, "cargo", &os_args(&["update", "--workspace"]))?;
        self.run(
            dir,
            "cargo",
            &os_args(&[
                "hack",
                "clippy",
                "--feature-powerset",
                "--fix",
                "--allow-dirty",
                "--allow-staged",
                "--all-targets",
                "--workspace",
                "--",
                "-D",
                "warnings",
            ]),
        )?;
        self.run(
            dir,
            "cargo",
            &os_args(&[
                "hack",
                "fix",
                "--feature-powerset",
                "--allow-dirty",
                "--allow-staged",
                "--all-targets",
                "--workspace",
            ]),
        )?;
        self.run(
            dir,
            "cargo",
            &os_args(&[
                "hack",
                "check",
                "--feature-powerset",
                "--all-targets",
                "--workspace",
            ]),
        )?;
        self.run(
            dir,
            "cargo",
            &os_args(&[
                "hack",
                "test",
                "--feature-powerset",
                "--release",
                "--verbose",
                "--all-targets",
                "--workspace",
                "--no-fail-fast",
                "--lib",
                "--bins",
                "--examples",
                "--tests",
                "--benches",
            ]),
        )?;
        self.run(dir, "cargo", &os_args(&["fmt", "--all"]))
    }

    fn commit(&self, message: Option<String>) -> Result<()> {
        if !self.ci {
            return Err(Box::new(SiteError::new(
                "commit is CI-only; review and commit local changes with git".to_string(),
            )));
        }

        let message = message
            .or_else(|| env::var("GITHUB_JOB").ok())
            .unwrap_or_else(|| "update generated files".to_string());

        self.run(
            &self.root,
            "git",
            &os_args(&[
                "config",
                "user.email",
                "github-actions[bot]@users.noreply.github.com",
            ]),
        )?;
        self.run(
            &self.root,
            "git",
            &os_args(&["config", "user.name", "github-actions[bot]"]),
        )?;
        self.run(&self.root, "git", &os_args(&["add", "-A"]))?;

        let has_changes = !self.status_success(
            &self.root,
            "git",
            &os_args(&["diff", "--cached", "--quiet"]),
        )?;

        if !has_changes {
            println!("No changes to commit");
            return Ok(());
        }

        self.run(
            &self.root,
            "git",
            &os_args(&["commit", "-m", message.as_str()]),
        )?;
        self.run(&self.root, "git", &os_args(&["push"]))
    }

    fn pnpm_install(&self, dir: &Path, mode: InstallMode) -> Result<()> {
        let args = match mode {
            InstallMode::Locked if self.ci => os_args(&["install", "--frozen-lockfile"]),
            InstallMode::Locked => os_args(&["install"]),
            InstallMode::Unlocked => os_args(&["install", "--no-frozen-lockfile"]),
        };

        self.run(dir, "pnpm", &args)
    }

    fn git_iso_date(&self, args: &[OsString]) -> Result<Option<String>> {
        let Some(output) = self.output_optional(&self.root, "git", args)? else {
            return Ok(None);
        };

        Ok(output
            .split('T')
            .next()
            .filter(|value| !value.is_empty())
            .map(str::to_string))
    }

    fn relative_git_path(&self, path: &Path) -> Result<String> {
        Ok(path
            .strip_prefix(&self.root)?
            .to_string_lossy()
            .replace('\\', "/"))
    }

    fn warn(&self, message: &str) {
        if self.ci {
            println!("::warning::{message}");
        } else {
            eprintln!("warning: {message}");
        }
    }

    fn run<P>(&self, cwd: &Path, program: P, args: &[OsString]) -> Result<()>
    where
        P: AsRef<OsStr>,
    {
        self.run_with_env(cwd, program, args, &[])
    }

    fn run_with_env<P>(
        &self,
        cwd: &Path,
        program: P,
        args: &[OsString],
        envs: &[(&str, &str)],
    ) -> Result<()>
    where
        P: AsRef<OsStr>,
    {
        let program = program.as_ref();
        self.print_command(cwd, program, args);

        let mut command = Command::new(program);
        command.args(args).current_dir(cwd);
        for (key, value) in envs {
            command.env(key, value);
        }

        let status = command.status().map_err(|source| {
            SiteError(format!(
                "failed to run {}: {source}",
                format_command(program, args)
            ))
        })?;

        if status.success() {
            Ok(())
        } else {
            Err(Box::new(SiteError(format!(
                "command failed with {status}: {}",
                format_command(program, args)
            ))))
        }
    }

    pub(crate) fn output_optional<P>(
        &self,
        cwd: &Path,
        program: P,
        args: &[OsString],
    ) -> Result<Option<String>>
    where
        P: AsRef<OsStr>,
    {
        let program = program.as_ref();
        let output = Command::new(program)
            .args(args)
            .current_dir(cwd)
            .stderr(Stdio::inherit())
            .output()
            .map_err(|source| {
                SiteError(format!(
                    "failed to run {}: {source}",
                    format_command(program, args)
                ))
            })?;

        if !output.status.success() {
            return Ok(None);
        }

        let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok((!value.is_empty()).then_some(value))
    }

    fn status_success<P>(&self, cwd: &Path, program: P, args: &[OsString]) -> Result<bool>
    where
        P: AsRef<OsStr>,
    {
        let program = program.as_ref();
        Ok(Command::new(program)
            .args(args)
            .current_dir(cwd)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|source| {
                SiteError(format!(
                    "failed to run {}: {source}",
                    format_command(program, args)
                ))
            })?
            .success())
    }

    fn print_command(&self, cwd: &Path, program: &OsStr, args: &[OsString]) {
        let relative = cwd.strip_prefix(&self.root).unwrap_or(cwd);
        let label = if relative.as_os_str().is_empty() {
            ".".to_string()
        } else {
            relative.display().to_string()
        };

        println!("$ (cd {label}) {}", format_command(program, args));
    }
}

#[derive(Clone, Copy)]
enum InstallMode {
    Locked,
    Unlocked,
}

fn main() {
    if let Err(error) = run_main() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run_main() -> Result<()> {
    let site = Site::new()?;
    let mut args = env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        args.push("build".to_string());
    }

    match args[0].as_str() {
        "build" => site.build(parse_mode(&args[1..], Mode::default_for(site.ci))?),
        "wasm" => site.wasm(parse_mode(&args[1..], Mode::default_for(site.ci))?),
        "generate" => site.generate(),
        "links" | "collect-links" => site.links(),
        "indices" => site.indices(),
        "report" => {
            let build_time = match args.get(1) {
                Some(value) => value.parse::<u64>().map_err(|source| {
                    SiteError::new(format!("invalid report build time {value:?}: {source}"))
                })?,
                None => 0,
            };
            report::write(&site, &site.root.join("public"), build_time)
        }
        "align-tables" => tables::align(&args[1..]),
        "aoc-problems" | "download-aoc-problems" => aoc::download_problem_text(&site, &args[1..]),
        "aoc-inputs" | "download-aoc-inputs" => aoc::download_inputs(&site, &args[1..]),
        "dates" => {
            let target = args.get(1).map_or("content", String::as_str);
            site.dates(target)
        }
        "update" | "update-and-lint" => site.update(),
        "commit" => {
            let message = (!args[1..].is_empty()).then(|| args[1..].join(" "));
            site.commit(message)
        }
        "help" | "--help" | "-h" => {
            print_help();
            Ok(())
        }
        other => Err(Box::new(SiteError(format!("unknown command: {other}")))),
    }
}

fn parse_mode(args: &[String], default: Mode) -> Result<Mode> {
    let mut mode = None;

    for arg in args {
        let parsed = match arg.as_str() {
            "--dev" | "dev" => Mode::Dev,
            "--prod" | "prod" | "--release" | "release" => Mode::Prod,
            other => {
                return Err(Box::new(SiteError(format!(
                    "unknown mode argument: {other}"
                ))))
            }
        };

        if mode.replace(parsed).is_some() {
            return Err(Box::new(SiteError::new(
                "mode can only be specified once".to_string(),
            )));
        }
    }

    Ok(mode.unwrap_or(default))
}

fn print_help() {
    println!(
        "\
site build system

Usage:
  cargo run --locked --manifest-path tools/site/Cargo.toml -- <command>

Commands:
  build [--dev|--prod]       build wasm assets and the Quartz site
  wasm [--dev|--prod]        build only the wasm and TypeScript bundle
  generate                   regenerate link lists, indices, chords, and dates
  links                      regenerate plaintext link lists
  indices                    regenerate misc indices and trolley count
  report [seconds]           write public/report.html build report
  align-tables LEFT RIGHT SEP merge matching lines from two files
  aoc-problems [options]     download scaffolded AoC problem statements
  aoc-inputs [options]       download scaffolded AoC puzzle inputs
  dates [path]               refresh markdown date metadata
  update                     run dependency updates and linters
  commit [message]           CI-only commit and push for generated files
"
    );
}

fn find_repo_root() -> Result<PathBuf> {
    let mut current = env::current_dir()?;

    loop {
        if current.join(".git").exists()
            && (current.join("quartz.config.ts").exists()
                || current.join("quartz.config.yaml").exists()
                || current.join("quartz.ts").exists())
        {
            return Ok(current);
        }

        if !current.pop() {
            break;
        }
    }

    Err(Box::new(SiteError(
        "could not find repository root from current directory".to_string(),
    )))
}

fn remove_dir_if_exists(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    Ok(())
}

fn remove_license_files(path: &Path) -> Result<()> {
    if !path.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with("LICENSE.txt") {
            fs::remove_file(entry.path())?;
        }
    }

    Ok(())
}

fn existing_date_metadata(path: &Path) -> Result<Vec<String>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let source = fs::read_to_string(path)?;
    Ok(source
        .lines()
        .filter(|line| line.starts_with("date:") || line.starts_with("updated:"))
        .map(str::to_string)
        .collect())
}

fn collect_markdown_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_markdown_files(&path, files)?;
        } else if path
            .extension()
            .is_some_and(|extension| extension == OsStr::new("md"))
        {
            files.push(path);
        }
    }

    Ok(())
}

fn extract_urls(source: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut offset = 0;

    while let Some(start) = find_next_url(&source[offset..]) {
        let absolute_start = offset + start;
        let mut absolute_end = absolute_start;

        for (index, ch) in source[absolute_start..].char_indices() {
            if is_url_delimiter(ch) {
                break;
            }
            absolute_end = absolute_start + index + ch.len_utf8();
        }

        if absolute_end > absolute_start {
            let link = source[absolute_start..absolute_end]
                .trim_end_matches(['.', ',', ';'])
                .to_string();
            links.push(link);
        }

        offset = absolute_end.max(absolute_start + 1);
    }

    links
}

fn find_next_url(source: &str) -> Option<usize> {
    match (source.find("http://"), source.find("https://")) {
        (Some(http), Some(https)) => Some(http.min(https)),
        (Some(http), None) => Some(http),
        (None, Some(https)) => Some(https),
        (None, None) => None,
    }
}

fn is_url_delimiter(ch: char) -> bool {
    ch.is_whitespace() || matches!(ch, '<' | '>' | '"' | '\'' | '`' | '[' | ']' | '(' | ')')
}

fn os_args(args: &[&str]) -> Vec<OsString> {
    args.iter().map(OsString::from).collect()
}

fn format_command(program: &OsStr, args: &[OsString]) -> String {
    let mut parts = vec![format_os(program)];
    parts.extend(args.iter().map(|arg| format_os(arg.as_os_str())));
    parts.join(" ")
}

fn format_os(value: &OsStr) -> String {
    let value = value.to_string_lossy();
    if value.chars().any(char::is_whitespace) {
        format!("{value:?}")
    } else {
        value.into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_markdown_urls_without_trailing_punctuation() {
        let links =
            extract_urls(r#"see [one](https://example.com/a?b=1), "http://example.org/x"."#);

        assert_eq!(
            links,
            vec![
                "https://example.com/a?b=1".to_string(),
                "http://example.org/x".to_string()
            ]
        );
    }

    #[test]
    fn parses_build_modes() {
        assert_eq!(
            parse_mode(&["--prod".to_string()], Mode::Dev).unwrap(),
            Mode::Prod
        );
        assert_eq!(
            parse_mode(&["dev".to_string()], Mode::Prod).unwrap(),
            Mode::Dev
        );
        assert!(parse_mode(&["--nope".to_string()], Mode::Dev).is_err());
    }
}
