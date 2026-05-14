mod aoc;
mod build;
mod content;
mod fsutil;
mod maintenance;
#[cfg(feature = "photos")]
mod photos;
#[cfg(not(feature = "photos"))]
mod photos {
    use crate::{Result, Site, SiteError};

    pub(crate) fn process(_: &Site, _: &[String]) -> Result<()> {
        Err(Box::new(SiteError::new(
            "process-photos requires the `photos` feature; rebuild without --no-default-features",
        )))
    }

    // pub(crate) fn setup_nightshade(_: &Site, _: &[String]) -> Result<()> {
    //     Err(Box::new(SiteError::new(
    //         "setup-nightshade requires the `photos` feature; rebuild without --no-default-features",
    //     )))
    // }
}
mod process;
mod recursive_ji;
mod report;
mod tables;

use std::env;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

pub(crate) use fsutil::{
    find_repo_root, remove_dir_if_exists, remove_file_if_exists, remove_license_files,
};
pub(crate) use process::{os_args, ChildGuard, InstallMode};

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
pub(crate) enum Mode {
    Dev,
    Prod,
}

impl Mode {
    pub(crate) fn webpack(self) -> &'static str {
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
    pub(crate) ci: bool,
}

impl Site {
    fn new() -> Result<Self> {
        Ok(Self {
            root: find_repo_root()?,
            ci: env::var("GITHUB_ACTIONS").is_ok_and(|value| value == "true"),
        })
    }
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
        "generate" => {
            site.generate()?;
            recursive_ji::generate(&site, &args[1..])
        }
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
        "recursive-ji-music" | "rji-music" => recursive_ji::generate(&site, &args[1..]),
        "aoc-problems" | "download-aoc-problems" => aoc::download_problem_text(&site, &args[1..]),
        "aoc-inputs" | "download-aoc-inputs" => aoc::download_inputs(&site, &args[1..]),
        "process-photos" => photos::process(&site, &args[1..]),
        // "setup-nightshade" => photos::setup_nightshade(&site, &args[1..]),
        "check" => site.check(),
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
                             --dev also watches TypeScript bundles
  wasm [--dev|--prod]        build only the wasm and TypeScript bundle
  generate                   regenerate link lists, indices, chords, and dates
  links                      regenerate plaintext link lists
  indices                    regenerate misc indices
  report [seconds]           write public/report.html build report
  align-tables LEFT RIGHT SEP merge matching lines from two files
  recursive-ji-music [OUTPUT]
                             render recursive just-intonation examples
  aoc-problems [options]     download scaffolded AoC problem statements
  aoc-inputs [options]       download scaffolded AoC puzzle inputs
  process-photos [INPUT] [OUTPUT]
                             classify and publish source photos
  check                      run Rust, TypeScript, and lint checks
  dates [path]               refresh markdown date metadata
  update                     run dependency updates and linters
  commit [message]           CI-only commit and push for generated files
"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

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
