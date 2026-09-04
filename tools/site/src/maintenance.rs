use crate::{InstallMode, Result, Site, SiteError, fail, os_args};
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;
use std::{env, fs};
use toml::{Table, Value};

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
            return fail("commit is CI-only; review and commit local changes with git");
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

        fail(format!(
            "refusing to commit workflow changes from CI: {}. \
             Nothing this command generates is a workflow, so something has \
             written one; push it by hand after reading it",
            offending.join(", ")
        ))
    }
}

/// Lists every path dependency that more than one workspace member declares
/// for itself, which is the shape of a dependency that belongs in
/// `[workspace.dependencies]`.
pub(crate) fn parse_cargo_toml(site: &Site) -> Result<()> {
    let root = fs::read_to_string(site.root.join("Cargo.toml"))?.parse::<Table>()?;
    let members = root
        .get("workspace")
        .and_then(Value::as_table)
        .and_then(|workspace| workspace.get("members"))
        .and_then(Value::as_array)
        .ok_or_else(|| SiteError::new("Cargo.toml declares no workspace members"))?;

    let mut declared: BTreeMap<String, Vec<&str>> = BTreeMap::new();
    for member in members.iter().filter_map(Value::as_str) {
        let manifest =
            fs::read_to_string(site.root.join(member).join("Cargo.toml"))?.parse::<Table>()?;
        let Some(dependencies) = manifest.get("dependencies").and_then(Value::as_table) else {
            continue;
        };

        for (name, value) in dependencies {
            if value.get("path").and_then(Value::as_str).is_some() {
                declared.entry(name.clone()).or_default().push(member);
            }
        }
    }

    for (name, members) in declared {
        if members.len() <= 1 {
            continue;
        }
        println!("{name} is a path dependency of {} crates:", members.len());
        for member in members {
            println!("  {member}");
        }
    }

    Ok(())
}
