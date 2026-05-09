use crate::report;
use crate::{
    os_args, remove_dir_if_exists, remove_file_if_exists, remove_license_files, ChildGuard,
    InstallMode, Mode, Result, Site,
};
use std::fs;
use std::time::Instant;

impl Site {
    pub(crate) fn build(&self, mode: Mode) -> Result<()> {
        let started = Instant::now();

        if mode == Mode::Dev {
            self.warn("building in development mode");
        }

        remove_dir_if_exists(&self.root.join("content/js"))?;
        self.wasm(mode)?;
        remove_license_files(&self.root.join("content/js"))?;

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
        remove_file_if_exists(&wasm_dir.join("pkg/.gitignore"))?;

        let ts_dir = self.root.join("ts");
        self.bun_install(&ts_dir, InstallMode::Locked)?;
        self.sync_wasm_package_dependency()?;
        self.run_bun(&ts_dir, &os_args(&["run", "tsc"]))?;
        self.run_bun(
            &ts_dir,
            &os_args(&[
                "run",
                "webpack",
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
            fs::create_dir_all(&target)?;
        }

        if fs::canonicalize(&source)? == fs::canonicalize(&target)? {
            return Ok(());
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
