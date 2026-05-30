use crate::{os_args, InstallMode, Result, Site, SiteError};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
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
