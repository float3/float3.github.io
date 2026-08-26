//! Compiling the Elm graph.
//!
//! `quartz-local/elm-graph` holds an Elm application -- the site graph: its
//! neighbourhood, its force simulation and its SVG -- and this turns it into
//! the `content/js/elm.js` that `ts/src/graph.ts` loads.
//!
//! It is two commands rather than one. `elm make --optimize` does Elm's own
//! optimising but no minifying, which the Elm guide is explicit about: the
//! output is meant to be handed to a minifier afterwards, and the compiler
//! makes that safe by guaranteeing there is no `eval` and no reflection in
//! what it emits. esbuild is already a dependency of the site, and it takes
//! the compiled graph from 163 KB to 54 KB, which is 19 KB over the wire.

use crate::process::command_succeeds;
use crate::{Mode, Result, Site, SiteError, os_args, remove_file_if_exists};
use std::ffi::OsString;
use std::fs;
use std::path::Path;

/// Where the compiled program lands, under the directory `site wasm` owns.
const OUTPUT: &str = "content/js/elm.js";

/// The compiler's own output, before esbuild. It sits beside the finished file
/// rather than in a temporary directory so that a failed build leaves it where
/// it can be read.
const RAW: &str = "content/js/elm.raw.js";

impl Site {
    pub(crate) fn elm(&self, mode: Mode) -> Result<()> {
        let source = self.root.join("quartz-local/elm-graph");
        if !source.join("elm.json").is_file() {
            return Err(Box::new(SiteError::new(
                "no elm.json in quartz-local/elm-graph",
            )));
        }

        let output = self.root.join(OUTPUT);
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent)?;
        }

        match mode {
            Mode::Prod => {
                let raw = self.root.join(RAW);
                self.compile(&source, &raw, mode)?;
                self.minify(&raw, &output)?;
                remove_file_if_exists(&raw)
            }
            Mode::Dev => {
                // Unoptimised and unminified: the compile is quicker, and the
                // JavaScript in the debugger is the JavaScript on disk.
                self.compile(&source, &output, mode)
            }
        }
    }

    fn compile(&self, source: &Path, output: &Path, mode: Mode) -> Result<()> {
        let program = elm_program().ok_or_else(|| {
            SiteError::new(
                "could not find elm; it is in the flake's dev shell, or see elm-lang.org/install",
            )
        })?;

        let mut args = os_args(&["make", "src/Main.elm"]);
        if mode == Mode::Prod {
            args.push("--optimize".into());
        } else {
            self.warn("building the elm graph in development mode");
        }
        args.push(format!("--output={}", output.display()).into());

        self.run(source, program, &args)?;

        let compiled = fs::read_to_string(output)?;
        fs::write(output, anchor_to_global(&compiled)?)?;
        Ok(())
    }

    /// esbuild, through bun, because that is how everything else in `ts/` is
    /// run and it is the same esbuild Quartz builds itself with.
    fn minify(&self, input: &Path, output: &Path) -> Result<()> {
        let args = os_args(&[
            "x",
            "esbuild",
            &input.display().to_string(),
            "--minify",
            // Elm emits ES5; there is nothing to gain from downlevelling it
            // further, and plenty to lose from letting a minifier try.
            "--target=es2020",
            &format!("--outfile={}", output.display()),
        ]);

        self.run_bun(&self.root, &args)
    }
}

/// The compiler, if it is installed. It is in the flake's dev shell, which is
/// what CI builds inside; asking it for its version is how `bun` is looked for
/// a few lines away in `process.rs`, and it answers the same question.
fn elm_program() -> Option<OsString> {
    command_succeeds("elm", &["--version"]).then(|| "elm".into())
}

/// What the compiled program hands itself as its global object, and what it is
/// changed to.
const TAIL: &str = "}(this));";
const ANCHORED: &str = "}(globalThis));";

/// Points the compiled program at `globalThis` instead of `this`.
///
/// Elm wraps its output in `(function (scope) { ... }(this))` and hangs `Elm`
/// off whatever it is handed. In a plain script tag `this` is the window and
/// that works -- but only there. esbuild reads the file as a module, where
/// top-level `this` is undefined, and folds the constant: the last line comes
/// out as `}(void 0));` and the program dies reading `Elm` of undefined before
/// the graph ever draws. Anything else that ever reads this file -- a bundler,
/// a `type="module"` tag, a future minifier -- has the same opinion.
///
/// `globalThis` is that same object under a name nobody can reinterpret. It is
/// an error rather than a fallback when the tail is not found: a compiler that
/// stops emitting it needs this looked at, not skipped quietly.
fn anchor_to_global(compiled: &str) -> Result<String> {
    let at = compiled.rfind(TAIL).ok_or_else(|| {
        SiteError::new(format!(
            "the compiled Elm does not end with {TAIL:?}; the fixup in tools/site/src/elm.rs needs revisiting"
        ))
    })?;

    let mut anchored = String::with_capacity(compiled.len() + ANCHORED.len());
    anchored.push_str(&compiled[..at]);
    anchored.push_str(ANCHORED);
    anchored.push_str(&compiled[at + TAIL.len()..]);
    Ok(anchored)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_program_is_handed_a_global_it_will_still_have_after_a_minifier() {
        let compiled = "(function (scope) {\n  scope['Elm'] = {};\n}(this));\n";
        let anchored = anchor_to_global(compiled).expect("the tail is there");

        assert!(anchored.ends_with("}(globalThis));\n"));
        assert!(!anchored.contains("}(this));"));
    }

    #[test]
    fn only_the_last_call_is_touched() {
        // `this` appears all over a compiled program; only the one call that
        // starts it is the one being repointed.
        let compiled = "var f = function () { return this.x }(this);\n}(this));";
        let anchored = anchor_to_global(compiled).expect("the tail is there");

        assert!(anchored.starts_with("var f = function () { return this.x }(this);"));
        assert!(anchored.ends_with("}(globalThis));"));
    }

    #[test]
    fn a_compiler_that_stops_emitting_it_is_an_error_rather_than_a_shrug() {
        assert!(anchor_to_global("console.log('not elm')").is_err());
    }

    #[test]
    fn the_output_lives_where_the_bundle_does() {
        // `site wasm` empties content/js and fills it; the compiled Elm is one
        // of the things it fills it with, so it has to be written after that
        // and named as part of the same directory.
        assert!(OUTPUT.starts_with("content/js/"));
        assert!(RAW.starts_with("content/js/"));
        assert_ne!(OUTPUT, RAW);
    }
}
