use crate::{os_args, InstallMode, Result, Site, SiteError};
use std::env;
use std::path::Path;

impl Site {
    pub(crate) fn update(&self) -> Result<()> {
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

        self.run_bun(&self.root, &os_args(&["update"]))?;
        self.run_bun(&self.root, &os_args(&["audit"]))?;
        self.bun_install(&self.root, InstallMode::Unlocked)?;

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

        self.run(
            &self.root,
            "git",
            &os_args(&["commit", "-m", message.as_str()]),
        )?;
        self.run(&self.root, "git", &os_args(&["push"]))
    }
}
