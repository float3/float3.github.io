//! Compiles the background shaders the way the page does, so a GLSL mistake
//! fails `cargo test` instead of leaving a visitor with a blank background.
//!
//! The shaders are template literals in `ts/src/background/shaders.ts` and
//! `fluid.ts`. `buildFragmentSource` in `renderer.ts` puts each behind a prelude
//! of uniforms and the shared helpers, and after it a `main()` that calls its
//! `mainImage`. This reads all of those pieces out of the TypeScript, assembles
//! them the same way, and hands each result to glslang, Khronos's reference
//! compiler, as the GLSL ES 3.00 that WebGL2 compiles.

#[cfg(test)]
mod tests {
    use glslang::{
        Compiler, CompilerOptions, Shader, ShaderInput, ShaderMessage, ShaderSource, ShaderStage,
        SourceLanguage, Target,
    };
    use std::fs;
    use std::path::Path;

    /// The text of the template literal that opens with `` const <name> = ` ``,
    /// up to the backtick that closes it on a line of its own or at the end of
    /// one.
    fn literal(source: &str, name: &str) -> String {
        let open = format!("const {name} = `");
        let start = source
            .find(&open)
            .unwrap_or_else(|| panic!("no `const {name} = \\`` literal"))
            + open.len();
        let end = source[start..]
            .find("`\n")
            .unwrap_or_else(|| panic!("{name} is never closed"));
        source[start..start + end].to_string()
    }

    /// Every `const NAME = \`...\`` literal in a file, by name.
    fn literals(source: &str) -> Vec<(String, String)> {
        let mut found = Vec::new();
        for line in source.lines() {
            let line = line.trim_start_matches("export ");
            let Some(rest) = line.strip_prefix("const ") else {
                continue;
            };
            let Some((name, value)) = rest.split_once(" = ") else {
                continue;
            };
            if value == "`" && name.chars().all(|ch| ch.is_ascii_uppercase() || ch == '_') {
                found.push((name.to_string(), literal(source, name)));
            }
        }
        found
    }

    /// The fragment prelude with its knob uniforms written out, as the
    /// `KNOB_UNIFORMS.map(...)` line in it produces them.
    fn prelude(renderer: &str) -> String {
        let knobs_line = renderer
            .lines()
            .find(|line| line.starts_with("const KNOB_UNIFORMS = ["))
            .expect("renderer.ts has no KNOB_UNIFORMS");
        let knobs: Vec<&str> = knobs_line
            .trim_start_matches("const KNOB_UNIFORMS = [")
            .trim_end_matches(']')
            .split(',')
            .map(|knob| knob.trim().trim_matches('"'))
            .collect();
        let uniforms = knobs
            .iter()
            .map(|knob| format!("uniform float u_{knob};"))
            .collect::<Vec<_>>()
            .join("\n");

        let start = renderer
            .find("const FRAGMENT_PRELUDE = `")
            .expect("renderer.ts has no FRAGMENT_PRELUDE")
            + "const FRAGMENT_PRELUDE = `".len();
        let end = start
            + renderer[start..]
                .find("\n`\n")
                .expect("FRAGMENT_PRELUDE is never closed");
        let text = &renderer[start..end + 1];
        let knob_template =
            "${KNOB_UNIFORMS.map((knob) => `uniform float u_${knob};`).join(\"\\n\")}";
        assert!(
            text.contains(knob_template),
            "FRAGMENT_PRELUDE no longer declares the knobs the way this test writes them out"
        );
        let text = text.replace(knob_template, &uniforms);
        assert!(
            !text.contains("${"),
            "FRAGMENT_PRELUDE interpolates something new"
        );
        text
    }

    /// The `main()` that `buildFragmentSource` appends to a `mainImage` body.
    fn epilogue(renderer: &str) -> String {
        let open = "const epilogue = hasMainImage\n    ? `";
        let start = renderer
            .find(open)
            .expect("buildFragmentSource no longer starts its epilogue the way this test reads it")
            + open.len();
        let end = renderer[start..].find('`').unwrap();
        renderer[start..start + end].to_string()
    }

    fn compile(stage: ShaderStage, source: &str) -> Result<(), String> {
        let compiler = Compiler::acquire().expect("glslang");
        let source = ShaderSource::from(source.to_string());
        let options = CompilerOptions {
            source_language: SourceLanguage::GLSL,
            target: Target::None(None),
            version_profile: None,
            messages: ShaderMessage::DEFAULT,
        };
        let input = ShaderInput::new(&source, stage, &options, None, None)
            .map_err(|error| error.to_string())?;
        Shader::new(compiler, input)
            .map(|_| ())
            .map_err(|error| error.to_string())
    }

    /// glslang's `ERROR: 0:<line>:` counted from the start of the body instead
    /// of the assembled source, which is the line a reader of `shaders.ts` needs.
    fn relative_to_body(log: &str, name: &str, offset: usize) -> String {
        log.lines()
            .filter(|line| line.starts_with("ERROR: 0:"))
            .map(|line| {
                let rest = &line["ERROR: 0:".len()..];
                match rest.split_once(':') {
                    Some((number, message)) => match number.parse::<usize>() {
                        Ok(number) if number > offset => {
                            format!("{name} line {}:{message}", number - offset)
                        }
                        _ => format!("{name} (prelude or helpers):{message}"),
                    },
                    None => line.to_string(),
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn every_background_shader_compiles_as_webgl2_would() {
        let root = crate::find_repo_root().unwrap();
        let dir = root.join("ts/src/background");
        let read = |file: &str| fs::read_to_string(Path::new(&dir).join(file)).unwrap();
        let renderer = read("renderer.ts");
        let shaders = read("shaders.ts");
        let fluid = read("fluid.ts");

        assert!(
            renderer
                .contains(r"return `${FRAGMENT_PRELUDE}\n${GLSL_HELPERS}\n${body}\n${epilogue}`"),
            "buildFragmentSource assembles a shader differently now; update this test to match"
        );
        let prelude = prelude(&renderer);
        let helpers = literal(&shaders, "GLSL_HELPERS");
        let epilogue = epilogue(&renderer);

        let mut failures = Vec::new();
        if let Err(log) = compile(ShaderStage::Vertex, &literal(&renderer, "VERTEX_SOURCE")) {
            failures.push(format!("VERTEX_SOURCE:\n{log}"));
        }

        let head = format!("{prelude}\n{helpers}\n");
        let offset = head.matches('\n').count();
        let bodies: Vec<(String, String)> = literals(&shaders)
            .into_iter()
            .chain(literals(&fluid))
            .filter(|(name, _)| name != "GLSL_HELPERS")
            .collect();
        assert!(bodies.len() >= 20, "found only {} shaders", bodies.len());

        for (name, body) in &bodies {
            assert!(
                !body.contains("${"),
                "{name} interpolates, so it cannot be checked as written"
            );
            assert!(body.contains("mainImage"), "{name} has no mainImage");
            let source = format!("{head}{body}\n{epilogue}");
            if let Err(log) = compile(ShaderStage::Fragment, &source) {
                failures.push(relative_to_body(&log, name, offset));
            }
        }

        assert!(
            failures.is_empty(),
            "shaders that do not compile:\n{}",
            failures.join("\n\n")
        );
    }
}
