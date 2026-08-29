use crate::{InstallMode, Result, Site, SiteError, os_args};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};
use toml::Table;

impl Site {
    pub(crate) fn update(&self) -> Result<()> {
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

        self.run_bun(&self.root, &os_args(&["update"]))?;
        self.run_bun(&self.root, &os_args(&["audit"]))?;
        self.bun_install(&self.root, InstallMode::Unlocked)?;

        self.node_update(&self.root.join("ts"), "src")?;

        // Every Rust crate here is a member of the one root workspace, and each
        // command below carries --workspace, so running this from the root covers
        // all of them once. It used to run a directory at a time over a list that
        // named `wasm/tuningplayground/tuning_systems` -- a crate renamed to
        // chord_generator -- so the first thing `update` did was fail on a
        // directory that does not exist, and the six passes after it would have
        // been six repeats of the same workspace-wide work anyway.
        self.cargo_update(&self.root)
    }

    fn node_update(&self, dir: &Path, lint_target: &str) -> Result<()> {
        self.run_bun(dir, &os_args(&["update"]))?;
        self.run_bun(dir, &os_args(&["audit"]))?;
        self.bun_install(dir, InstallMode::Unlocked)?;
        self.run_bun(dir, &os_args(&["run", "prettier", lint_target, "--write"]))?;
        self.run_bun(dir, &os_args(&["run", "eslint", lint_target, "--fix"]))
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

    pub(crate) fn commit(&self, message: Option<String>) -> Result<()> {
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

        self.refuse_staged_workflow_changes()?;

        self.run(
            &self.root,
            "git",
            &os_args(&["commit", "-m", message.as_str()]),
        )?;
        self.run(&self.root, "git", &os_args(&["push"]))
    }

    /// Refuses to commit a change to a workflow.
    ///
    /// `git add -A` is right for these two callers — `generate` and `update`
    /// between them touch content, lockfiles, formatting and generated
    /// TypeScript, and enumerating that is a list that would go stale — but it
    /// means whatever ran just before this decides what gets pushed to the
    /// default branch. A generator or a dependency that wrote into
    /// `.github/workflows` would be writing the next run's own instructions,
    /// with `contents: write` in hand.
    ///
    /// GitHub already refuses such a push, because the token it mints for a run
    /// has no `workflow` scope. That is a fact about GitHub's side rather than
    /// about this repository, and it fails as a rejected push at the very end
    /// rather than as a sentence saying what happened. Nothing generated here
    /// has ever been a workflow.
    fn refuse_staged_workflow_changes(&self) -> Result<()> {
        let staged = Command::new("git")
            .args(["diff", "--cached", "--name-only"])
            .current_dir(&self.root)
            .output()?;

        let offending: Vec<String> = String::from_utf8_lossy(&staged.stdout)
            .lines()
            .map(str::trim)
            .filter(|path| {
                path.starts_with(".github/workflows/") || path.starts_with(".github/actions/")
            })
            .map(str::to_string)
            .collect();

        if offending.is_empty() {
            return Ok(());
        }

        Err(Box::new(SiteError::new(format!(
            "refusing to commit workflow changes from CI: {}. \
             Nothing this command generates is a workflow, so something has \
             written one; push it by hand after reading it",
            offending.join(", ")
        ))))
    }
}

pub(crate) fn parse_cargo_toml(site: &Site) -> Result<()> {
    // find all cargo.tomls
    let mut cargo_tomls = Vec::new();
    collect_files_with_name(&site.root, "Cargo.toml", &mut cargo_tomls)?;

    // find all duplicate dependencies that are not using the workspace version
    let mut dependencies: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for cargo_toml in cargo_tomls {
        let content = fs::read_to_string(&cargo_toml)?;
        let parsed = content.parse::<Table>()?;
        let empty_table = Table::new();
        let deps = parsed
            .get("dependencies")
            .and_then(|d| d.as_table())
            .unwrap_or(&empty_table);
        for (name, value) in deps {
            if value.get("path").and_then(|v| v.as_str()).is_some() {
                dependencies
                    .entry(name.clone())
                    .or_default()
                    .push(cargo_toml.clone());
            }
        }
    }

    for (name, cargo_tomls) in dependencies {
        if cargo_tomls.len() <= 1 {
            continue;
        }
        println!(
            "Dependency {name} is duplicated {} times in crates:",
            cargo_tomls.len()
        );
        for cargo_toml in &cargo_tomls {
            println!("  {}", display_path(site, cargo_toml.parent().unwrap()));
        }
    }

    Ok(())
}

fn collect_files_with_name(
    root: &std::path::PathBuf,
    name: &str,
    files: &mut Vec<PathBuf>,
) -> Result<()> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_files_with_name(&path, name, files)?;
        } else if path.file_name().and_then(|n| n.to_str()) == Some(name) {
            files.push(path);
        }
    }
    Ok(())
}

fn display_path(site: &Site, path: &Path) -> String {
    let p = match site.relative_git_path(path) {
        Ok(p) => p,
        Err(_) => path.display().to_string(),
    };

    p.to_string().replace(r"\\?\", "")
}
