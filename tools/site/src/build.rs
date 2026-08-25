use crate::report;
use crate::{
    os_args, remove_dir_if_exists, remove_file_if_exists, remove_files, ChildGuard, InstallMode,
    Mode, Result, Site, SiteError,
};
use serde_json::Value as JsonValue;
use std::fs;
use std::path::Path;
use std::time::Instant;
use toml::{Table, Value};

/// Features of `wasm/wasm` that configure the build rather than name a tool.
const NON_TOOL_FEATURES: [&str; 3] = ["default", "console_error_panic_hook", "mini-alloc"];

/// Tools listed under `[package.metadata.site] parked-tools`: kept in the tree,
/// left out of the build. A parked tool needs no webpack entry and no package
/// in ts/, since nothing is produced for it to import.
fn parked_tools(manifest: &Table) -> Vec<&str> {
    manifest
        .get("package")
        .and_then(Value::as_table)
        .and_then(|package| package.get("metadata"))
        .and_then(Value::as_table)
        .and_then(|metadata| metadata.get("site"))
        .and_then(Value::as_table)
        .and_then(|site| site.get("parked-tools"))
        .and_then(Value::as_array)
        .map(|names| names.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default()
}

impl Site {
    pub(crate) fn build(&self, mode: Mode) -> Result<()> {
        let started = Instant::now();

        if mode == Mode::Dev {
            self.warn("building in development mode");
        }

        remove_dir_if_exists(&self.root.join("content/js"))?;
        self.wasm(mode)?;
        remove_files(&self.root.join("content/js"))?;

        self.bun_install(&self.root, InstallMode::Locked)?;

        let mut args = os_args(&["quartz/bootstrap-cli.mts", "build"]);
        if mode == Mode::Dev {
            args.push("--serve".into());
        }
        let _typescript_watchers = if mode == Mode::Dev {
            self.start_typescript_watchers()?
        } else {
            Vec::new()
        };

        self.run_bun(&self.root, &args)?;

        let public = self.root.join("public");
        report::write(self, &public, started.elapsed().as_secs())
    }

    pub(crate) fn wasm(&self, mode: Mode) -> Result<()> {
        let wasm_dir = self.root.join("wasm/wasm");
        let mut base_args = os_args(&["build", "--target", "bundler"]);

        match mode {
            Mode::Prod => {
                base_args.push("--release".into());
            }
            Mode::Dev => {
                base_args.push("--dev".into());
                self.warn("building wasm in development mode");
            }
        }

        // One package per tool, where "the tools" is whatever `wasm/wasm` says it is.
        let tools = self.wasm_tools(&wasm_dir)?;
        self.check_wasm_tool_wiring(&tools)?;

        for tool in &tools {
            let mut args = base_args.clone();
            args.extend(os_args(&["--out-dir", &format!("pkg/{tool}"), "--"]));
            args.extend(os_args(&[
                "--features",
                "console_error_panic_hook",
                "--features",
                tool,
            ]));

            self.run_with_env(
                &wasm_dir,
                "wasm-pack",
                &args,
                &[("RUSTFLAGS", r#"--cfg getrandom_backend="wasm_js""#)],
            )?;

            self.patch_wasm_package_name(&wasm_dir.join(format!("pkg/{tool}")), tool)?;

            // wasm-pack drops a .gitignore in every output directory
            remove_file_if_exists(&wasm_dir.join(format!("pkg/{tool}/.gitignore")))?;
        }

        let ts_dir = self.root.join("ts");
        self.bun_install(&ts_dir, InstallMode::Locked)?;
        self.run_bun(&ts_dir, &os_args(&["run", "tsc"]))?;
        self.run_bun(
            &ts_dir,
            &os_args(&[
                "run",
                "webpack",
                "--config",
                "webpack.config.ts",
                "--mode",
                mode.webpack(),
            ]),
        )
    }

    fn start_typescript_watchers(&self) -> Result<Vec<ChildGuard>> {
        let ts_dir = self.root.join("ts");
        self.warn("watching TypeScript and webpack output for Quartz reloads");
        Ok(vec![
            self.spawn_bun(
                &ts_dir,
                &os_args(&["run", "tsc", "--watch", "--preserveWatchOutput"]),
            )?,
            self.spawn_bun(
                &ts_dir,
                &os_args(&[
                    "run",
                    "webpack",
                    "--config",
                    "webpack.config.ts",
                    "--mode",
                    "development",
                    "--watch",
                ]),
            )?,
        ])
    }

    /// The interactive tools are exactly the Cargo features of `wasm/wasm` that aren't
    /// build knobs, so adding a tool means adding a feature and an entry point — never
    /// remembering to also edit a list in here.
    fn wasm_tools(&self, wasm_dir: &Path) -> Result<Vec<String>> {
        let manifest = fs::read_to_string(wasm_dir.join("Cargo.toml"))?.parse::<Table>()?;

        let features = manifest
            .get("features")
            .and_then(Value::as_table)
            .ok_or_else(|| SiteError::new("wasm/wasm/Cargo.toml has no [features] table"))?;

        let parked = parked_tools(&manifest);

        let mut tools: Vec<String> = features
            .keys()
            .filter(|feature| !NON_TOOL_FEATURES.contains(&feature.as_str()))
            .filter(|feature| !parked.contains(&feature.as_str()))
            .cloned()
            .collect();
        tools.sort();

        if tools.is_empty() {
            return Err(Box::new(SiteError::new(
                "wasm/wasm/Cargo.toml declares no tool features",
            )));
        }

        Ok(tools)
    }

    /// A tool the bundler never depends on builds fine and then silently does nothing,
    /// which is how `pokemon` stayed missing from `ts/package.json` while still being
    /// imported — it only worked off a stale `node_modules`. Fail loudly instead.
    fn check_wasm_tool_wiring(&self, tools: &[String]) -> Result<()> {
        let ts_dir = self.root.join("ts");
        let manifest = fs::read_to_string(ts_dir.join("package.json"))?.parse::<JsonValue>()?;
        let dependencies = manifest.get("dependencies").and_then(JsonValue::as_object);
        let sources = read_sources(&ts_dir.join("src"))?;

        let mut problems = Vec::new();
        for tool in tools {
            let package = format!("wasm-{tool}");
            let expected = format!("../wasm/wasm/pkg/{tool}");

            match dependencies.and_then(|dependencies| dependencies.get(&package)) {
                None => problems.push(format!("ts/package.json is missing \"{package}\"")),
                Some(JsonValue::String(spec)) if spec != &format!("file:{expected}") => problems
                    .push(format!(
                        "\"{package}\" points at {spec}, expected file:{expected}"
                    )),
                Some(_) => {}
            }

            if !sources.contains(&format!("\"{package}\"")) {
                problems.push(format!("nothing under ts/src imports \"{package}\""));
            }
        }

        if problems.is_empty() {
            return Ok(());
        }

        Err(Box::new(SiteError::new(format!(
            "wasm tools declared in wasm/wasm/Cargo.toml are not wired into ts/:\n  {}",
            problems.join("\n  ")
        ))))
    }

    /// wasm-pack names every package after the crate; each per-tool copy needs its own
    /// name so `ts/` can depend on them side by side.
    fn patch_wasm_package_name(&self, pkg_dir: &Path, tool: &str) -> Result<()> {
        let pkg_json = pkg_dir.join("package.json");
        let mut manifest = fs::read_to_string(&pkg_json)?.parse::<JsonValue>()?;

        manifest
            .as_object_mut()
            .ok_or_else(|| SiteError::new(format!("{} is not a JSON object", pkg_json.display())))?
            .insert(
                "name".to_string(),
                JsonValue::String(format!("wasm-{tool}")),
            );

        fs::write(pkg_json, serde_json::to_string_pretty(&manifest)?)?;
        Ok(())
    }

    pub(crate) fn check(&self) -> Result<()> {
        let mut site_check_args = os_args(&[
            "check",
            "--locked",
            "--manifest-path",
            "tools/site/Cargo.toml",
        ]);
        let mut site_test_args = os_args(&[
            "test",
            "--locked",
            "--manifest-path",
            "tools/site/Cargo.toml",
        ]);

        if self.ci {
            site_check_args.push("--no-default-features".into());
            site_test_args.push("--no-default-features".into());
        }

        if self.ci {
            self.warn("checking site tool without default features");
        }

        self.run(&self.root, "cargo", &site_check_args)?;
        self.run(&self.root, "cargo", &site_test_args)?;
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

        if !local_tsc.is_file() || !local_eslint.is_file() {
            self.bun_install(&dir, InstallMode::Locked)?;
        }

        self.run_bun(
            &dir,
            &os_args(&["run", "tsc", "--noEmit", "--incremental", "false"]),
        )?;
        self.run_bun(&dir, &os_args(&["run", "eslint", "src"]))
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
}

/// Every `.ts` file under `dir`, concatenated — enough to answer "does anything import
/// this package" without teaching the build system to parse TypeScript.
fn read_sources(dir: &Path) -> Result<String> {
    let mut sources = String::new();

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            sources.push_str(&read_sources(&path)?);
        } else if path.extension().is_some_and(|extension| extension == "ts") {
            sources.push_str(&fs::read_to_string(&path)?);
        }
    }

    Ok(sources)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn site() -> Site {
        Site {
            root: crate::find_repo_root().expect("tests run inside the repository"),
            ci: false,
        }
    }

    #[test]
    fn derives_the_tool_list_from_the_wasm_crate() {
        let site = site();
        let tools = site.wasm_tools(&site.root.join("wasm/wasm")).unwrap();

        assert!(tools.contains(&"tuningplayground".to_string()));
        assert!(!tools.contains(&"default".to_string()));
        assert!(!tools.contains(&"mini-alloc".to_string()));
        assert!(!tools.contains(&"console_error_panic_hook".to_string()));
    }

    #[test]
    fn every_wasm_tool_is_wired_into_the_bundle() {
        let site = site();
        let tools = site.wasm_tools(&site.root.join("wasm/wasm")).unwrap();

        if let Err(error) = site.check_wasm_tool_wiring(&tools) {
            panic!("{error}");
        }
    }

    /// A parked tool is source kept for later, not a tool that is built. It has
    /// no webpack entry and no package under ts/, so if it ever leaked back into
    /// the built set the wiring check would fail on it, confusingly, since
    /// nothing is wrong with it.
    #[test]
    fn parked_tools_are_not_built() {
        let site = site();
        let wasm_dir = site.root.join("wasm/wasm");
        let manifest = fs::read_to_string(wasm_dir.join("Cargo.toml"))
            .unwrap()
            .parse::<Table>()
            .unwrap();

        let parked = parked_tools(&manifest);
        assert!(!parked.is_empty(), "expected at least one parked tool");

        let features = manifest["features"].as_table().unwrap();
        let built = site.wasm_tools(&wasm_dir).unwrap();

        for tool in &parked {
            assert!(
                features.contains_key(*tool),
                "{tool} is parked but has no feature to park"
            );
            assert!(
                !built.iter().any(|name| name == tool),
                "{tool} is parked but was built"
            );
        }
    }
}
