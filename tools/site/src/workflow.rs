//! What every command driven by a GitHub issue shares.
//!
//! `comment-from-issue` and `gallery-from-issue` are each handed one issue by
//! a workflow, read a marker out of it, apply it to the content directory, and
//! hand their results back as step outputs. The reading and the handing back
//! live here; what an issue has to contain, and what applying it means, live
//! with the command.
//!
//! Everything read here was written by a stranger. A value that reaches
//! `$GITHUB_OUTPUT` is fenced so that it cannot name a second output, and a
//! refusal is told apart from a fault so the workflow can report the one back
//! on the issue and leave the other for me.

use crate::{Result, Site, SiteError};
use serde_json::Value;
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

/// Long enough for any comment worth reading, short enough to bound the damage.
const MAX_BODY: usize = 64 * 1024;

/// A refusal, phrased for whoever will read it on the issue or pull request.
///
/// Separate from the ordinary error type so the workflow can tell "this input
/// was not acceptable", which it reports back and closes, from "this command
/// broke", which is mine to fix.
#[derive(Debug)]
pub(crate) struct Rejected(pub String);

impl std::fmt::Display for Rejected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for Rejected {}

pub(crate) fn reject<T>(message: impl Into<String>) -> Result<T> {
    Err(Box::new(Rejected(message.into())))
}

/// The issue the workflow was started by, and the content directory it applies
/// to.
///
/// Both arrive in the environment: `ISSUE_JSON` is the event's issue object,
/// and `CONTENT_DIR` is where the pages are, relative to the repository root.
pub(crate) fn issue_from_env(site: &Site) -> Result<(Value, PathBuf)> {
    let issue = serde_json::from_str(&env::var("ISSUE_JSON").unwrap_or_default())
        .map_err(|error| SiteError::new(format!("ISSUE_JSON is not valid JSON: {error}")))?;
    let content_dir = site
        .root
        .join(env::var("CONTENT_DIR").unwrap_or_else(|_| "content".into()));
    Ok((issue, content_dir))
}

/// Passes an error on, after telling the workflow about a refusal.
///
/// A refusal is reported back on the issue by the workflow, so it has to reach
/// it as an output before the command exits. A fault is left to fail the step.
pub(crate) fn report_rejection<T>(error: Box<dyn Error>) -> Result<T> {
    if error.downcast_ref::<Rejected>().is_some() {
        write_outputs(&[("rejected", error.to_string())])?;
    }
    Err(error)
}

/// Prints every output, and writes them for the workflow to read.
pub(crate) fn publish_outputs(outputs: &[(&str, String)]) -> Result<()> {
    for (key, value) in outputs {
        println!("{key}={value}");
    }
    write_outputs(outputs)
}

/// What a page put in the issue: the marker's payload, and the text after it.
#[derive(Debug)]
pub(crate) struct Submission {
    pub payload: Value,
    pub body: String,
}

/// Splits the machine-readable half of an issue body from the rest.
///
/// The workflow keys off the marker rather than off a label, because a label
/// set through `?labels=` in a prefilled issue URL is silently dropped for
/// anyone without triage permission on the repository — which is everyone this
/// exists for. An HTML comment, so GitHub renders the issue as just what the
/// reader wrote, with the machine-readable part invisible.
///
/// The shape is fixed by line rather than by searching for the closing `-->`:
///
/// ```text
/// <!--hilll.dev:comment
/// {"parent":"blog/page.md"}
/// -->
///
/// the comment
/// ```
///
/// The payload is whatever sits on the line after the marker, because
/// serialised JSON never contains a raw newline. Delimiting it by the first
/// `-->` instead looks equivalent and is not: a comment quoting a passage that
/// contains an arrow puts `-->` *inside the payload*, and the split lands in
/// the middle of the JSON. The body may still contain as many as it likes.
///
/// A comment needs text after the marker and a gallery submission needs files,
/// so neither check lives here.
pub(crate) fn parse_marked_issue(issue_body: &str, marker: &str) -> Result<Submission> {
    let text = issue_body.replace("\r\n", "\n");
    let opening = format!("<!--{marker}");

    let Some(start) = text.find(&opening) else {
        return reject(format!("this issue carries no {marker} marker"));
    };
    let after_marker = start + opening.len();

    // Past the remainder of the marker's own line, onto the payload's.
    let Some(payload_start) = text[after_marker..]
        .find('\n')
        .map(|at| after_marker + at + 1)
    else {
        return reject(format!("the {marker} marker is never closed"));
    };
    let payload_end = text[payload_start..]
        .find('\n')
        .map(|at| payload_start + at)
        .unwrap_or(text.len());

    let payload: Value = match serde_json::from_str(text[payload_start..payload_end].trim()) {
        Ok(value) => value,
        Err(_) => return reject(format!("the {marker} marker does not contain valid JSON")),
    };
    if !payload.is_object() {
        return reject(format!("the {marker} payload is not an object"));
    }

    let Some(close) = text[payload_end..].find("-->").map(|at| payload_end + at) else {
        return reject(format!("the {marker} marker is never closed"));
    };

    let body = text[close + 3..].trim().to_string();
    if body.len() > MAX_BODY {
        return reject(format!("the issue is longer than {MAX_BODY} bytes"));
    }

    Ok(Submission { payload, body })
}

/// Writes step outputs for the workflow to read.
///
/// One output at a time, each fenced by `render_output`: every value here is
/// built from something a stranger wrote — an issue payload, a refusal quoting
/// a path, the `author` line of a file in somebody's pull request — and in
/// `$GITHUB_OUTPUT` a newline in a value starts a new output. A value able to
/// carry one could name any output the workflow reads: the `file` the comment
/// run stages, the `dir` the gallery run commits. The workflows also stop at a
/// rejection before reading those, but that is an ordering property of two
/// YAML files, and this holds on its own.
pub(crate) fn write_outputs(outputs: &[(&str, String)]) -> Result<()> {
    let Ok(path) = env::var("GITHUB_OUTPUT") else {
        return Ok(());
    };

    use std::io::Write;
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;

    for (key, value) in outputs {
        write!(file, "{}", render_output(key, value))?;
    }

    Ok(())
}

/// One output, in whichever of the two forms the value needs.
///
/// `key=value` for a value that is one line, and the heredoc form for one that
/// is not, with a delimiter checked against the value: a fixed word is one the
/// value can contain, and a line equal to the delimiter closes the block early.
fn render_output(key: &str, value: &str) -> String {
    if !value.contains('\n') && !value.contains('\r') {
        return format!("{key}={value}\n");
    }

    let delimiter = delimiter_for(value);
    format!("{key}<<{delimiter}\n{value}\n{delimiter}\n")
}

/// A delimiter no line of the value is.
///
/// The counter is what makes this terminate: the random half only has to make a
/// deliberate collision impractical to arrange, and the counter makes an
/// accidental one impossible to sustain.
fn delimiter_for(value: &str) -> String {
    for attempt in 0.. {
        let delimiter = format!("SITEOUTPUT_{}_{attempt}", random_id());
        if !value.lines().any(|line| line.trim_end() == delimiter) {
            return delimiter;
        }
    }
    unreachable!("a fresh delimiter is found or the counter never ends")
}

/// Four bytes of hex.
///
/// Enough for a comment id, which only has to be unique within one page's
/// thread and is no secret, and for an output delimiter, which is checked
/// against the value it fences.
pub(crate) fn random_id() -> String {
    let mut bytes = [0u8; 4];
    getrandom(&mut bytes);
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn getrandom(bytes: &mut [u8; 4]) {
    // No `rand` dependency for four bytes: the clock and the process id are
    // mixed only to keep two runs in the same second apart, and collisions are
    // checked against the directory anyway.
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_nanos())
        .unwrap_or(0);
    let seed = (now as u64) ^ ((std::process::id() as u64) << 32);
    // SplitMix64, which is short enough to read and good enough for a filename.
    let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    bytes.copy_from_slice(&(z as u32).to_be_bytes());
}

/// An issue body in the shape a page writes: the marker, its payload, and the
/// text after it.
#[cfg(test)]
pub(crate) fn marked_issue(marker: &str, payload: &str, body: &str) -> String {
    format!("<!--{marker}\n{payload}\n-->\n\n{body}\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `$GITHUB_OUTPUT` is a format, and every value written to it here came
    /// from a stranger. A value that can start a line can name any output the
    /// workflow reads — the `file` the comment run stages, the `dir` the
    /// gallery run commits — so no value may be able to.
    #[test]
    fn an_output_value_cannot_invent_a_second_output() {
        assert_eq!(
            render_output("file", "content/a.comment.1.md"),
            "file=content/a.comment.1.md\n"
        );

        // The shape a comment file's `author` field can take: `frontmatter_field`
        // decodes a quoted value as a JSON string, and `"a\nREFUSALS\nfile=x"`
        // decodes to one carrying real newlines.
        let forged = "belongs to `alice\nREFUSALS\nfile=/etc/passwd`";
        let written = render_output("refusals", forged);

        let delimiter = written
            .lines()
            .next()
            .unwrap()
            .strip_prefix("refusals<<")
            .expect("a multi-line value takes the heredoc form")
            .to_string();

        // The value is fenced, and the fence is not a word the value contains.
        assert!(!forged.lines().any(|line| line == delimiter));
        assert!(written.ends_with(&format!("\n{delimiter}\n")));

        // Which is the whole point: read back the way GitHub reads it, exactly
        // one output arrives, and `file` is not among them.
        let parsed = parse_outputs(&written);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].0, "refusals");
        assert_eq!(parsed[0].1, forged);

        // And a value that happens to contain a delimiter gets a different one.
        let awkward = format!("first\n{delimiter}\nlast");
        let again = render_output("refusals", &awkward);
        let second = again
            .lines()
            .next()
            .unwrap()
            .strip_prefix("refusals<<")
            .unwrap();
        assert_ne!(second, delimiter);
        assert_eq!(parse_outputs(&again)[0].1, awkward);
    }

    /// `$GITHUB_OUTPUT` as the runner reads it: `key=value`, or `key<<DELIM`
    /// through to a line that is exactly `DELIM`.
    fn parse_outputs(written: &str) -> Vec<(String, String)> {
        let mut outputs = Vec::new();
        let mut lines = written.lines();

        while let Some(line) = lines.next() {
            if let Some((key, delimiter)) = line.split_once("<<") {
                let mut value = Vec::new();
                for line in lines.by_ref() {
                    if line == delimiter {
                        break;
                    }
                    value.push(line);
                }
                outputs.push((key.to_string(), value.join("\n")));
            } else if let Some((key, value)) = line.split_once('=') {
                outputs.push((key.to_string(), value.to_string()));
            }
        }

        outputs
    }

    #[test]
    fn splits_the_payload_from_the_body_by_line() {
        let issue = marked_issue(
            "hilll.dev:comment",
            r#"{"parent":"a.md","quote":"x --> y"}"#,
            "the --> comment",
        );
        let submission = parse_marked_issue(&issue, "hilll.dev:comment").unwrap();
        assert_eq!(submission.payload["quote"], "x --> y");
        assert_eq!(submission.body, "the --> comment");

        let other = parse_marked_issue(&issue, "hilll.dev:gallery").unwrap_err();
        assert!(other.downcast_ref::<Rejected>().is_some());
    }
}
