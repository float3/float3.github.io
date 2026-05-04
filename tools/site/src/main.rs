mod aoc;
mod recursive_ji;
mod report;
mod tables;

use image::codecs::jpeg::JpegEncoder;
use image::{ImageReader, Rgb, RgbImage};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
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

struct PhotoPoisonOptions {
    input: PathBuf,
    output: PathBuf,
    manifest: PathBuf,
    strength: u8,
    quality: u8,
    dry_run: bool,
}

struct GalleryEntry {
    src: String,
    title: String,
    meta: String,
    width: u32,
    height: u32,
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

struct ChildGuard {
    child: Child,
    label: String,
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if matches!(self.child.try_wait(), Ok(Some(_))) {
            return;
        }

        if let Err(error) = self.child.kill() {
            eprintln!("warning: failed to stop {}: {error}", self.label);
            return;
        }

        if let Err(error) = self.child.wait() {
            eprintln!("warning: failed to wait for {}: {error}", self.label);
        }
    }
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

        let mut args = os_args(&["quartz/bootstrap-cli.mjs", "build"]);
        if mode == Mode::Dev {
            args.push("--serve".into());
        }
        let _typescript_watchers = if mode == Mode::Dev {
            self.start_typescript_watchers()?
        } else {
            Vec::new()
        };

        self.run(&self.root, "node", &args)?;

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
        self.sync_wasm_package_dependency()?;
        self.run(
            &ts_dir,
            "node",
            &os_args(&["node_modules/typescript/bin/tsc"]),
        )?;
        self.run(
            &ts_dir,
            "node",
            &os_args(&[
                "node_modules/webpack/bin/webpack.js",
                "--config",
                "webpack.config.mjs",
                "--mode",
                mode.webpack(),
            ]),
        )
    }

    fn start_typescript_watchers(&self) -> Result<Vec<ChildGuard>> {
        let ts_dir = self.root.join("ts");
        self.warn("watching TypeScript and webpack output for Quartz reloads");
        Ok(vec![
            self.spawn(
                &ts_dir,
                "node",
                &os_args(&[
                    "node_modules/typescript/bin/tsc",
                    "--watch",
                    "--preserveWatchOutput",
                ]),
            )?,
            self.spawn(
                &ts_dir,
                "node",
                &os_args(&[
                    "node_modules/webpack/bin/webpack.js",
                    "--config",
                    "webpack.config.mjs",
                    "--mode",
                    "development",
                    "--watch",
                ]),
            )?,
        ])
    }

    fn sync_wasm_package_dependency(&self) -> Result<()> {
        let source = self.root.join("wasm/wasm/pkg");
        let target = self.root.join("ts/node_modules/wasm");

        if !target.exists() {
            return Err(SiteError::new(format!(
                "missing {}; run pnpm install in ts before bundling wasm",
                target.display()
            ))
            .into());
        }

        for entry in fs::read_dir(&source)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if file_type.is_file() {
                let source_path = entry.path();
                let target_path = target.join(entry.file_name());
                if target_path.exists() {
                    fs::remove_file(&target_path)?;
                }
                fs::copy(source_path, target_path)?;
            }
        }

        Ok(())
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
        self.run(&dir, "cargo", &os_args(&["run", "-p", "chord_generator"]))
    }

    fn check(&self) -> Result<()> {
        self.run(
            &self.root,
            "cargo",
            &os_args(&[
                "check",
                "--locked",
                "--manifest-path",
                "tools/site/Cargo.toml",
            ]),
        )?;
        self.run(
            &self.root,
            "cargo",
            &os_args(&[
                "test",
                "--locked",
                "--manifest-path",
                "tools/site/Cargo.toml",
            ]),
        )?;
        self.wasm(Mode::Dev)?;
        self.check_typescript()?;

        for (manifest, target_name) in [
            ("wasm/glsl2hlsl/Cargo.toml", "glsl2hlsl"),
            ("wasm/textprocessing/Cargo.toml", "textprocessing"),
            ("wasm/tuningplayground/Cargo.toml", "tuningplayground"),
        ] {
            self.cargo_check_manifest(manifest, target_name)?;
        }

        Ok(())
    }

    fn check_typescript(&self) -> Result<()> {
        let dir = self.root.join("ts");
        let local_tsc = dir.join("node_modules/typescript/bin/tsc");
        let local_eslint = dir.join("node_modules/eslint/bin/eslint.js");

        if local_tsc.is_file() && local_eslint.is_file() {
            self.run(
                &dir,
                "node",
                &os_args(&[
                    "node_modules/typescript/bin/tsc",
                    "--noEmit",
                    "--incremental",
                    "false",
                ]),
            )?;
            return self.run(
                &dir,
                "node",
                &os_args(&["node_modules/eslint/bin/eslint.js", "src"]),
            );
        }

        self.pnpm_install(&dir, InstallMode::Locked)?;
        self.run_pnpm(
            &dir,
            &os_args(&["exec", "tsc", "--noEmit", "--incremental", "false"]),
        )?;
        self.run_pnpm(&dir, &os_args(&["exec", "eslint", "src"]))
    }

    fn cargo_check_manifest(&self, manifest: &str, target_name: &str) -> Result<()> {
        let target_dir = format!("target/check/{target_name}");
        self.run(
            &self.root,
            "cargo",
            &os_args(&[
                "check",
                "--locked",
                "--manifest-path",
                manifest,
                "--target-dir",
                target_dir.as_str(),
            ]),
        )
    }

    fn poison_photos(&self, args: &[String]) -> Result<()> {
        if args.iter().any(|arg| arg == "--help" || arg == "-h") {
            print_photo_poison_help();
            return Ok(());
        }

        let options = self.parse_photo_poison_options(args)?;
        self.ensure_photo_input_is_private(&options.input)?;
        fs::create_dir_all(&options.input)?;
        fs::create_dir_all(&options.output)?;

        let mut photos = Vec::new();
        collect_photo_files(&options.input, &mut photos)?;
        photos.sort();

        if photos.is_empty() {
            self.warn(&format!(
                "no source photos found in {}; drop originals there and rerun poison-photos",
                options.input.display()
            ));
            return Ok(());
        }

        let mut entries = Vec::new();
        let mut names = HashMap::<String, usize>::new();

        for photo in photos {
            let relative = photo.strip_prefix(&options.input).unwrap_or(&photo);
            let stem = photo
                .file_stem()
                .and_then(OsStr::to_str)
                .map_or("photo", |value| value);
            let base = sanitize_file_stem(stem);
            let count = names.entry(base.clone()).or_insert(0);
            *count += 1;
            let file_name = if *count == 1 {
                format!("{base}.jpg")
            } else {
                format!("{base}-{count}.jpg")
            };
            let target = options.output.join(&file_name);
            let title = title_from_stem(stem);

            println!(
                "poison {} -> {}",
                relative.display(),
                target.strip_prefix(&self.root).unwrap_or(&target).display()
            );

            let (width, height) = if options.dry_run {
                (0, 0)
            } else {
                poison_photo_file(
                    &photo,
                    &target,
                    hash64(relative.to_string_lossy().as_bytes()),
                    options.strength,
                    options.quality,
                )?
            };

            entries.push(GalleryEntry {
                src: format!("/photography/gallery/{file_name}"),
                title,
                meta: format!(
                    "protected export from {}",
                    relative.to_string_lossy().replace('\\', "/")
                ),
                width,
                height,
            });
        }

        if !options.dry_run {
            write_gallery_manifest(&options.manifest, &entries)?;
        }

        Ok(())
    }

    fn parse_photo_poison_options(&self, args: &[String]) -> Result<PhotoPoisonOptions> {
        let mut positionals = Vec::new();
        let mut strength = 4_u8;
        let mut quality = 92_u8;
        let mut manifest = self.root.join("content/photography/gallery.json");
        let mut dry_run = false;
        let mut index = 0;

        while index < args.len() {
            match args[index].as_str() {
                "--strength" => {
                    index += 1;
                    let value = args.get(index).ok_or_else(|| {
                        Box::new(SiteError::new("--strength requires a value")) as Box<dyn Error>
                    })?;
                    strength = value.parse::<u8>().map_err(|source| {
                        SiteError::new(format!("invalid strength {value:?}: {source}"))
                    })?;
                }
                "--quality" => {
                    index += 1;
                    let value = args.get(index).ok_or_else(|| {
                        Box::new(SiteError::new("--quality requires a value")) as Box<dyn Error>
                    })?;
                    quality = value.parse::<u8>().map_err(|source| {
                        SiteError::new(format!("invalid quality {value:?}: {source}"))
                    })?;
                }
                "--manifest" => {
                    index += 1;
                    let value = args.get(index).ok_or_else(|| {
                        Box::new(SiteError::new("--manifest requires a path")) as Box<dyn Error>
                    })?;
                    manifest = self.resolve_path(value);
                }
                "--dry-run" => dry_run = true,
                "--help" | "-h" => {
                    print_photo_poison_help();
                    return Err(Box::new(SiteError::new("help requested")));
                }
                value if value.starts_with('-') => {
                    return Err(Box::new(SiteError::new(format!(
                        "unknown poison-photos option: {value}"
                    ))));
                }
                value => positionals.push(value.to_string()),
            }
            index += 1;
        }

        if !(1..=32).contains(&strength) {
            return Err(Box::new(SiteError::new(
                "strength must be between 1 and 32",
            )));
        }
        if !(60..=100).contains(&quality) {
            return Err(Box::new(SiteError::new(
                "quality must be between 60 and 100",
            )));
        }
        if positionals.len() > 2 {
            return Err(Box::new(SiteError::new(
                "poison-photos accepts at most INPUT and OUTPUT paths",
            )));
        }

        Ok(PhotoPoisonOptions {
            input: positionals.first().map_or_else(
                || self.root.join("private/photography/originals"),
                |path| self.resolve_path(path),
            ),
            output: positionals.get(1).map_or_else(
                || self.root.join("content/photography/gallery"),
                |path| self.resolve_path(path),
            ),
            manifest,
            strength,
            quality,
            dry_run,
        })
    }

    fn resolve_path(&self, path: &str) -> PathBuf {
        let path = PathBuf::from(path);
        if path.is_absolute() {
            path
        } else {
            self.root.join(path)
        }
    }

    fn ensure_photo_input_is_private(&self, path: &Path) -> Result<()> {
        let normalized_root = normalize_path(&self.root)?;
        let normalized_path = normalize_path(path)?;
        let content_dir = normalized_root.join("content");

        if normalized_path.starts_with(&content_dir) {
            return Err(Box::new(SiteError::new(format!(
                "photo originals must not live under {}; use private/photography/originals or another untracked directory",
                content_dir.display()
            ))));
        }

        Ok(())
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

        self.run_pnpm(&self.root, &os_args(&["update"]))?;
        self.run_pnpm(&self.root, &os_args(&["audit", "fix"]))?;
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
        self.run_pnpm(dir, &os_args(&["update"]))?;
        self.run_pnpm(dir, &os_args(&["audit", "fix"]))?;
        self.pnpm_install(dir, InstallMode::Unlocked)?;
        self.run_pnpm(dir, &os_args(&["exec", "prettier", lint_target, "--write"]))?;
        self.run_pnpm(dir, &os_args(&["exec", "eslint", lint_target, "--fix"]))
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

        if !pnpm_available() && dir.join("node_modules").is_dir() {
            self.warn(&format!(
                "pnpm was not found; reusing existing dependencies in {}",
                dir.display()
            ));
            return Ok(());
        }

        self.run_pnpm(dir, &args)
    }

    fn run_pnpm(&self, cwd: &Path, args: &[OsString]) -> Result<()> {
        if command_succeeds("pnpm", &["--version"]) {
            return self.run(cwd, "pnpm", args);
        }

        if !corepack_pnpm_available() {
            return Err(Box::new(SiteError::new(
                "could not find pnpm; enable Corepack or run pnpm install before building",
            )));
        }

        let mut fallback_args = if cfg!(windows) {
            os_args(&["/C", "corepack", "pnpm"])
        } else {
            os_args(&["pnpm"])
        };
        fallback_args.extend(args.iter().cloned());

        if cfg!(windows) {
            self.run(cwd, "cmd", &fallback_args)
        } else {
            self.run(cwd, "corepack", &fallback_args)
        }
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

    fn spawn<P>(&self, cwd: &Path, program: P, args: &[OsString]) -> Result<ChildGuard>
    where
        P: AsRef<OsStr>,
    {
        let program = program.as_ref();
        self.print_command(cwd, program, args);

        let child = Command::new(program)
            .args(args)
            .current_dir(cwd)
            .spawn()
            .map_err(|source| {
                SiteError(format!(
                    "failed to run {}: {source}",
                    format_command(program, args)
                ))
            })?;

        Ok(ChildGuard {
            child,
            label: format_command(program, args),
        })
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
        "recursive-ji-music" | "rji-music" => recursive_ji::generate(&site, &args[1..]),
        "aoc-problems" | "download-aoc-problems" => aoc::download_problem_text(&site, &args[1..]),
        "aoc-inputs" | "download-aoc-inputs" => aoc::download_inputs(&site, &args[1..]),
        "poison-photos" | "photo-poison" => site.poison_photos(&args[1..]),
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
  indices                    regenerate misc indices and trolley count
  report [seconds]           write public/report.html build report
  align-tables LEFT RIGHT SEP merge matching lines from two files
  recursive-ji-music [OUTPUT]
                             render recursive just-intonation examples
  aoc-problems [options]     download scaffolded AoC problem statements
  aoc-inputs [options]       download scaffolded AoC puzzle inputs
  poison-photos [INPUT] [OUTPUT]
                             protect source photos and update gallery manifest
  check                      run Rust, TypeScript, and lint checks
  dates [path]               refresh markdown date metadata
  update                     run dependency updates and linters
  commit [message]           CI-only commit and push for generated files
"
    );
}

fn print_photo_poison_help() {
    println!(
        "\
poison-photos

Usage:
  cargo run --manifest-path tools/site/Cargo.toml -- poison-photos [INPUT] [OUTPUT] [options]

Defaults:
  INPUT   private/photography/originals
  OUTPUT  content/photography/gallery

Options:
  --strength N       perturbation strength, 1-32, default 4
  --quality N        JPEG quality, 60-100, default 92
  --manifest PATH    gallery JSON path, default content/photography/gallery.json
  --dry-run          print planned work without writing images
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

fn normalize_path(path: &Path) -> Result<PathBuf> {
    if path.exists() {
        Ok(path.canonicalize()?)
    } else if let Some(parent) = path.parent() {
        let parent = normalize_path(parent)?;
        Ok(parent.join(
            path.file_name()
                .ok_or_else(|| SiteError::new(format!("invalid path: {}", path.display())))?,
        ))
    } else {
        Ok(path.to_path_buf())
    }
}

fn collect_photo_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    if !dir.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_photo_files(&path, files)?;
        } else if is_photo_file(&path) {
            files.push(path);
        }
    }

    Ok(())
}

fn is_photo_file(path: &Path) -> bool {
    let Some(extension) = path.extension().and_then(OsStr::to_str) else {
        return false;
    };

    matches!(
        extension.to_ascii_lowercase().as_str(),
        "jpg" | "jpeg" | "png" | "webp"
    )
}

fn poison_photo_file(
    input: &Path,
    output: &Path,
    seed: u64,
    strength: u8,
    quality: u8,
) -> Result<(u32, u32)> {
    let image = ImageReader::open(input)?.with_guessed_format()?.decode()?;
    let source = image.to_rgba8();
    let (width, height) = source.dimensions();
    let poisoned = poison_pixels(&source, seed, strength);

    let file = File::create(output)?;
    let writer = BufWriter::new(file);
    let mut encoder = JpegEncoder::new_with_quality(writer, quality);
    encoder.encode_image(&poisoned)?;

    Ok((width, height))
}

fn poison_pixels(source: &image::RgbaImage, seed: u64, strength: u8) -> RgbImage {
    let (width, height) = source.dimensions();
    let mut rgb = Vec::with_capacity((width * height) as usize);
    let mut luma = Vec::with_capacity((width * height) as usize);

    for pixel in source.pixels() {
        let alpha = f32::from(pixel[3]) / 255.0;
        let color = [
            f32::from(pixel[0]) * alpha + 255.0 * (1.0 - alpha),
            f32::from(pixel[1]) * alpha + 255.0 * (1.0 - alpha),
            f32::from(pixel[2]) * alpha + 255.0 * (1.0 - alpha),
        ];
        luma.push(color[0] * 0.2126 + color[1] * 0.7152 + color[2] * 0.0722);
        rgb.push(color);
    }

    let mut output = RgbImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) as usize;
            let edge = local_edge_strength(&luma, width, height, x, y);
            let amplitude = f32::from(strength) * (0.45 + edge * 0.75);
            let mut color = [0_u8; 3];

            for channel in 0..3 {
                let channel_seed = seed
                    ^ u64::from(x).wrapping_mul(0x9E37_79B1_85EB_CA87)
                    ^ u64::from(y).wrapping_mul(0xC2B2_AE3D_27D4_EB4F)
                    ^ (channel as u64).wrapping_mul(0x1656_67B1_9E37_79F9);
                let noise = signed_noise(channel_seed);
                let checker = if ((x ^ y ^ channel as u32) & 1) == 0 {
                    1.0
                } else {
                    -1.0
                };
                let wave = ((x as f32 * 0.73
                    + y as f32 * 1.37
                    + channel as f32 * 2.11
                    + (seed & 1023) as f32)
                    .sin())
                    * 0.35;
                let delta = amplitude * (noise * 0.65 + checker * 0.18 + wave);
                color[channel] = (rgb[index][channel] + delta).clamp(0.0, 255.0).round() as u8;
            }

            output.put_pixel(x, y, Rgb(color));
        }
    }

    output
}

fn local_edge_strength(luma: &[f32], width: u32, height: u32, x: u32, y: u32) -> f32 {
    let center = luma[(y * width + x) as usize];
    let mut total = 0.0;
    let mut count = 0.0;
    let neighbors = [
        (x.checked_sub(1), Some(y)),
        (x.checked_add(1).filter(|value| *value < width), Some(y)),
        (Some(x), y.checked_sub(1)),
        (Some(x), y.checked_add(1).filter(|value| *value < height)),
    ];

    for (nx, ny) in neighbors {
        if let (Some(nx), Some(ny)) = (nx, ny) {
            total += (center - luma[(ny * width + nx) as usize]).abs();
            count += 1.0;
        }
    }

    if count == 0.0 {
        0.0
    } else {
        (total / count / 255.0).clamp(0.0, 1.0)
    }
}

fn signed_noise(seed: u64) -> f32 {
    let value = splitmix64(seed);
    let unit = (value >> 11) as f64 / ((1_u64 << 53) as f64);
    (unit as f32) * 2.0 - 1.0
}

fn splitmix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9E37_79B9_7F4A_7C15);
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}

fn hash64(bytes: &[u8]) -> u64 {
    let mut hash = 0xCBF2_9CE4_8422_2325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    }
    hash
}

fn sanitize_file_stem(stem: &str) -> String {
    let mut result = String::new();
    for ch in stem.chars() {
        if ch.is_ascii_alphanumeric() {
            result.push(ch.to_ascii_lowercase());
        } else if !result.ends_with('-') {
            result.push('-');
        }
    }

    let result = result.trim_matches('-');
    if result.is_empty() {
        "photo".to_string()
    } else {
        result.to_string()
    }
}

fn title_from_stem(stem: &str) -> String {
    let title = stem
        .replace(['_', '-'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    if title.is_empty() {
        "untitled photo".to_string()
    } else {
        title
    }
}

fn write_gallery_manifest(path: &Path, entries: &[GalleryEntry]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut body = String::from("[\n");
    for (index, entry) in entries.iter().enumerate() {
        let comma = if index + 1 == entries.len() { "" } else { "," };
        body.push_str("  {\n");
        body.push_str(&format!("    \"src\": {},\n", json_string(&entry.src)));
        body.push_str(&format!("    \"title\": {},\n", json_string(&entry.title)));
        body.push_str(&format!("    \"meta\": {},\n", json_string(&entry.meta)));
        body.push_str(&format!("    \"width\": {},\n", entry.width));
        body.push_str(&format!("    \"height\": {}\n", entry.height));
        body.push_str(&format!("  }}{comma}\n"));
    }
    body.push_str("]\n");

    fs::write(path, body)?;
    Ok(())
}

fn json_string(value: &str) -> String {
    let mut result = String::from("\"");
    for ch in value.chars() {
        match ch {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            ch if ch.is_control() => result.push_str(&format!("\\u{:04x}", ch as u32)),
            ch => result.push(ch),
        }
    }
    result.push('"');
    result
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

fn command_succeeds(program: &str, args: &[&str]) -> bool {
    Command::new(program)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

fn pnpm_available() -> bool {
    command_succeeds("pnpm", &["--version"]) || corepack_pnpm_available()
}

fn corepack_pnpm_available() -> bool {
    if cfg!(windows) {
        command_succeeds("cmd", &["/C", "corepack", "pnpm", "--version"])
    } else {
        command_succeeds("corepack", &["pnpm", "--version"])
    }
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
