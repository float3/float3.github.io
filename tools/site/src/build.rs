use crate::report;
use crate::{
    ChildGuard, InstallMode, Mode, Result, Site, SiteError, os_args, remove_dir_if_exists,
    remove_file_if_exists, remove_license_files,
};
use serde_json::Value as JsonValue;
use std::collections::{BTreeSet, VecDeque};
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::Instant;
use toml::{Table, Value};

/// How many tool builds to have in flight at once.
///
/// Cargo serialises the compiles on its lock over the shared target directory,
/// so this is not about compiling in parallel: it is about having the next
/// compile queued behind the lock while the last one's wasm-opt is still
/// running. A handful is all that buys anything, and every extra one is
/// another wasm-opt holding a whole module in memory.
fn workers(tools: usize) -> usize {
    const MOST: usize = 4;

    thread::available_parallelism()
        .map(usize::from)
        .unwrap_or(1)
        .min(MOST)
        .min(tools)
        .max(1)
}

/// Removes anything in the package directory that is not one of this build's
/// tools.
///
/// Nothing had ever cleared it, so it still held a whole package from before
/// the split into one package per tool -- `wasm.js`, `wasm_bg.wasm` and a
/// LICENSE, a month stale. Being stale is the smaller half: a package left
/// behind for a tool that is no longer built goes on answering `ts/`'s `file:`
/// dependency on it, so the bundle can keep resolving a tool the build has
/// stopped producing, which is the shape of the `pokemon` bug the wiring check
/// exists to catch.
///
/// The live packages are left where they are for wasm-pack to write over.
/// Emptying the directory outright looks tidier and is not: bun installs a
/// `file:` dependency as a directory of symlinks pointing at the package file
/// by file, so removing one out from under `ts/node_modules` leaves every link
/// dangling, and the `bun install` that follows does not reliably mend them.
fn prune_wasm_packages(pkg: &Path, tools: &[String]) -> Result<()> {
    if !pkg.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(pkg)? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();

        // wasm-pack writes this, and `wasm/wasm/.gitignore` covers the
        // directory anyway; either way it is not something to delete.
        if name == ".gitignore" || tools.iter().any(|tool| *tool == name) {
            continue;
        }

        let path = entry.path();
        if path.is_dir() {
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }

    Ok(())
}

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

        self.wasm(mode)?;
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
        // Emptied here rather than in `build`, because this is the step that
        // fills it and `site wasm` runs on its own. webpack writes into the
        // directory without clearing it and names every wasm chunk after its
        // contents, so a directory nobody empties keeps a copy of every chunk
        // ever built: 110 MB of them had piled up before this moved.
        let content_js = self.root.join("content/js");
        remove_dir_if_exists(&content_js)?;

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

        prune_wasm_packages(&wasm_dir.join("pkg"), &tools)?;
        self.build_wasm_tools(&wasm_dir, &tools, &base_args)?;

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
        )?;

        // The Elm graph is the third thing that lands in content/js, and it
        // has to be built after the emptying above rather than before it.
        self.elm(mode)?;

        remove_license_files(&content_js)
    }

    /// Builds every tool's package, several at a time.
    ///
    /// They are independent -- one package per tool, each into its own output
    /// directory -- but they were built one after another, and for most of a
    /// build the machine had nothing to do. Cargo locks the shared target
    /// directory, so the compiles still take their turn; what now overlaps is
    /// one tool's wasm-bindgen and wasm-opt with the next tool's compile, and
    /// wasm-opt over the 10 MB Chinese dictionary is the longest single step
    /// there is.
    fn build_wasm_tools(
        &self,
        wasm_dir: &Path,
        tools: &[String],
        base_args: &[OsString],
    ) -> Result<()> {
        // The first one goes alone. wasm-pack installs wasm-bindgen and
        // wasm-opt into a shared cache the first time it needs them, and
        // several copies racing to populate it is not a race worth having.
        let (first, rest) = tools
            .split_first()
            .ok_or_else(|| SiteError::new("no tools to build"))?;
        print!("{}", self.build_wasm_tool(wasm_dir, first, base_args)?);

        let next = AtomicUsize::new(0);
        let broken = AtomicBool::new(false);
        let printing = Mutex::new(());
        let failures: Mutex<Vec<String>> = Mutex::new(Vec::new());

        thread::scope(|scope| {
            for _ in 0..workers(rest.len()) {
                scope.spawn(|| {
                    // Nothing new is started once a tool has failed. The
                    // failure is usually in code they all share, and twelve
                    // copies of one compiler error, each with a build log
                    // attached, is not twelve times as useful as one.
                    while !broken.load(Ordering::Relaxed) {
                        let index = next.fetch_add(1, Ordering::Relaxed);
                        let Some(tool) = rest.get(index) else { return };

                        match self.build_wasm_tool(wasm_dir, tool, base_args) {
                            // One lock, so a finished build's log lands in one
                            // piece rather than shuffled into another's.
                            Ok(log) => {
                                let _guard = printing.lock().expect("poisoned");
                                print!("{log}");
                            }
                            // As text: the error type here is not one a thread
                            // can carry back out.
                            Err(error) => {
                                broken.store(true, Ordering::Relaxed);
                                failures.lock().expect("poisoned").push(error.to_string());
                            }
                        }
                    }
                });
            }
        });

        let failures = failures.into_inner().expect("poisoned");
        if failures.is_empty() {
            return Ok(());
        }

        Err(Box::new(SiteError::new(failures.join("\n"))))
    }

    /// One tool's package, and the log of building it.
    fn build_wasm_tool(
        &self,
        wasm_dir: &Path,
        tool: &str,
        base_args: &[OsString],
    ) -> Result<String> {
        let mut args = base_args.to_vec();
        args.extend(os_args(&["--out-dir", &format!("pkg/{tool}"), "--"]));
        args.extend(os_args(&[
            "--features",
            "console_error_panic_hook",
            "--features",
            tool,
        ]));

        let log = self.run_captured(
            wasm_dir,
            "wasm-pack",
            &args,
            &[("RUSTFLAGS", r#"--cfg getrandom_backend="wasm_js""#)],
        )?;

        self.patch_wasm_package_name(&wasm_dir.join(format!("pkg/{tool}")), tool)?;

        // wasm-pack drops a .gitignore in every output directory
        remove_file_if_exists(&wasm_dir.join(format!("pkg/{tool}/.gitignore")))?;

        Ok(log)
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

    /// A tool the bundler never depends on builds fine and then silently does nothing.
    /// Asking only "does some file under ts/src name this package" is not enough: that
    /// is exactly what kept `pokemon` alive for four months after webpack stopped
    /// bundling it, because ts/src/pokemon.ts went on naming it into the void. So the
    /// question is whether a file webpack can actually *reach* imports it.
    fn check_wasm_tool_wiring(&self, tools: &[String]) -> Result<()> {
        let ts_dir = self.root.join("ts");
        let manifest = fs::read_to_string(ts_dir.join("package.json"))?.parse::<JsonValue>()?;
        let dependencies = manifest.get("dependencies").and_then(JsonValue::as_object);
        let sources = concatenated(&reachable_sources(&ts_dir)?)?;

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
                problems.push(format!("nothing webpack bundles imports \"{package}\""));
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
        // glsl2hlsl, textprocessing and tuningplayground are all members of the root
        // workspace, so one `--workspace` check covers them and compiles the shared
        // dependency graph once. Checking them a manifest at a time, each into its own
        // --target-dir, built that graph three more times over into 489 MB of
        // duplicate artifacts that CI got to pay for from cold on every run.
        //
        // The tests run over the workspace too. Running them for `site` alone is how
        // two red tests in recursive-ji-core went unnoticed long enough for the
        // tuning system underneath them to be renamed out from under the post.
        let mut check_args = os_args(&["check", "--locked", "--workspace"]);
        let mut test_args = os_args(&["test", "--locked", "--workspace"]);

        if self.ci {
            self.warn("checking without default features");
            check_args.push("--no-default-features".into());
            test_args.push("--no-default-features".into());
        }

        self.run(&self.root, "cargo", &check_args)?;
        self.run(&self.root, "cargo", &test_args)?;
        self.wasm(Mode::Dev)?;
        self.elm_test()?;
        self.check_typescript()
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
}

/// The `ts/src` files webpack actually pulls in: the union of the module graphs
/// rooted at the bundle's entry points. Anything outside this set is dead weight,
/// however convincingly it imports things.
fn reachable_sources(ts_dir: &Path) -> Result<BTreeSet<PathBuf>> {
    let src = ts_dir.join("src");
    let entries = webpack_entry_sources(ts_dir)?;
    let mut seen: BTreeSet<PathBuf> = entries.iter().cloned().collect();
    let mut queue: VecDeque<PathBuf> = entries.into();

    while let Some(path) = queue.pop_front() {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };

        for specifier in string_literals(&text) {
            if !specifier.starts_with('.') {
                continue;
            }
            let Some(next) = resolve_specifier(&path, &specifier) else {
                continue;
            };
            if next.starts_with(&src) && next.is_file() && seen.insert(next.clone()) {
                queue.push_back(next);
            }
        }
    }

    Ok(seen)
}

/// Where the bundle starts. webpack names `./dist/<name>.js`, which is what `tsc`
/// emits for `src/<name>.ts`.
fn webpack_entry_sources(ts_dir: &Path) -> Result<Vec<PathBuf>> {
    let config = fs::read_to_string(ts_dir.join("webpack.config.ts"))?;
    let src = ts_dir.join("src");

    let entries: Vec<PathBuf> = string_literals(&config)
        .iter()
        .filter_map(|literal| literal.strip_prefix("./dist/")?.strip_suffix(".js"))
        .map(|stem| src.join(format!("{stem}.ts")))
        .collect();

    if entries.is_empty() {
        return Err(Box::new(SiteError::new(
            "ts/webpack.config.ts declares no ./dist/*.js entry points",
        )));
    }

    if let Some(missing) = entries.iter().find(|entry| !entry.is_file()) {
        return Err(Box::new(SiteError::new(format!(
            "ts/webpack.config.ts names an entry with no source: {}",
            missing.display()
        ))));
    }

    Ok(entries)
}

/// `./thing.js` next to `importer`, as the `.ts` file `tsc` compiled it from.
fn resolve_specifier(importer: &Path, specifier: &str) -> Option<PathBuf> {
    let stem = specifier.strip_suffix(".js")?;
    let mut path = importer.parent()?.to_path_buf();

    for component in stem.split('/') {
        match component {
            "." => {}
            ".." => {
                path.pop();
            }
            name => path.push(name),
        }
    }

    let file = path.file_name()?.to_string_lossy().into_owned();
    path.set_file_name(format!("{file}.ts"));
    Some(path)
}

/// Every double-quoted literal in a source file. Import specifiers are always
/// quoted and never interpolated, so this is all the scanning here needs — and it
/// keeps the build system out of the business of parsing TypeScript.
fn string_literals(text: &str) -> Vec<String> {
    let mut literals = Vec::new();
    let mut rest = text;

    while let Some(start) = rest.find('"') {
        let after = &rest[start + 1..];
        let Some(end) = after.find('"') else {
            break;
        };
        if !after[..end].contains('\n') {
            literals.push(after[..end].to_string());
        }
        rest = &after[end + 1..];
    }

    literals
}

fn concatenated(paths: &BTreeSet<PathBuf>) -> Result<String> {
    let mut sources = String::new();
    for path in paths {
        sources.push_str(&fs::read_to_string(path)?);
    }
    Ok(sources)
}

/// Every `.ts` file under `dir`.
#[cfg(test)]
fn all_sources(dir: &Path) -> Result<BTreeSet<PathBuf>> {
    let mut sources = BTreeSet::new();

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            sources.extend(all_sources(&path)?);
        } else if path.extension().is_some_and(|extension| extension == "ts") {
            sources.insert(path);
        }
    }

    Ok(sources)
}

/// Which language feature each transform index needs, read out of the `#[cfg]`
/// attributes on the dispatch arms in `wasm/textprocessing/src/wasm/mod.rs`.
#[cfg(test)]
fn transform_features(root: &Path) -> Result<std::collections::BTreeMap<u32, String>> {
    let source = fs::read_to_string(root.join("wasm/textprocessing/src/wasm/mod.rs"))?;
    let mut features = std::collections::BTreeMap::new();
    let mut pending: Option<String> = None;

    for line in source.lines() {
        let line = line.trim();

        if let Some(rest) = line.strip_prefix("#[cfg(feature = ") {
            pending = rest
                .trim_end_matches(")]")
                .trim_matches('"')
                .to_string()
                .into();
            continue;
        }

        if let Some(rest) = line.strip_prefix('(')
            && let Some((index, _)) = rest.split_once(',')
            && let Ok(index) = index.trim().parse::<u32>()
        {
            let feature = pending.take().unwrap_or_else(|| "base".to_string());
            // An index may appear twice, once per direction; both arms
            // always carry the same gate.
            features.entry(index).or_insert(feature);
            continue;
        }

        pending = None;
    }

    Ok(features)
}

/// The index sets the TypeScript routes on, read back out of the source.
#[cfg(test)]
fn routed_indices(root: &Path, name: &str) -> Result<BTreeSet<u32>> {
    let source = fs::read_to_string(root.join("ts/src/textprocessing/index.ts"))?;
    let start = source
        .find(&format!("const {name} = new Set(["))
        .ok_or_else(|| SiteError::new(format!("ts/src/textprocessing/index.ts has no {name}")))?;
    let open = source[start..].find('[').unwrap() + start;
    let close = source[open..].find(']').unwrap() + open;

    Ok(source[open + 1..close]
        .split(',')
        .filter_map(|entry| entry.trim().parse::<u32>().ok())
        .collect())
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

    /// The check that `pokemon` would have failed. A `.ts` file webpack cannot reach
    /// is compiled by `tsc`, linted, and shipped to nobody; worse, it can go on
    /// satisfying the wiring check on behalf of a tool that is otherwise dead.
    #[test]
    fn no_typescript_is_unreachable_from_the_bundle() {
        let ts_dir = site().root.join("ts");
        let reachable = reachable_sources(&ts_dir).unwrap();
        let orphans: Vec<String> = all_sources(&ts_dir.join("src"))
            .unwrap()
            .difference(&reachable)
            .map(|path| path.display().to_string())
            .collect();

        assert!(
            orphans.is_empty(),
            "no webpack entry reaches these files:
  {}",
            orphans.join(
                "
  "
            )
        );
    }

    /// The TypeScript decides which of the three wasm packages to fetch for a
    /// given transform, and Rust decides which package actually answers for it.
    /// If those drift, the transform does not fail — it hands back the text it
    /// was given, which looks like a transform that does nothing. Keep them
    /// honest against each other.
    #[test]
    fn typescript_routes_every_transform_to_the_package_that_implements_it() {
        let root = site().root;
        let features = transform_features(&root).unwrap();

        for (language, set_name) in [("chinese", "chineseIndices"), ("korean", "koreanIndices")] {
            let expected: BTreeSet<u32> = features
                .iter()
                .filter(|(_, feature)| feature.as_str() == language)
                .map(|(index, _)| *index)
                .collect();
            let routed = routed_indices(&root, set_name).unwrap();

            assert!(
                !expected.is_empty(),
                "found no {language} arms at all, so this test is proving nothing"
            );

            assert_eq!(
                routed, expected,
                "{set_name} does not match the #[cfg(feature = \"{language}\")] arms in                  wasm/textprocessing/src/wasm/mod.rs"
            );
        }
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
