use crate::{Result, Site, SiteError, fail};
use std::env;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::OnceLock;

pub(crate) struct ChildGuard {
    child: Child,
    label: String,
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if matches!(self.child.try_wait(), Ok(Some(_))) {
            return;
        }

        if let Err(error) = self.child.kill() {
            eprintln!("warning: failed to stop {}: {error}", self.label);
            return;
        }

        if let Err(error) = self.child.wait() {
            eprintln!("warning: failed to wait for {}: {error}", self.label);
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum InstallMode {
    Locked,
    Unlocked,
}

impl Site {
    pub(crate) fn bun_install(&self, dir: &Path, mode: InstallMode) -> Result<()> {
        let args = match mode {
            InstallMode::Locked if self.ci => os_args(&["ci"]),
            InstallMode::Locked | InstallMode::Unlocked => os_args(&["install"]),
        };

        if bun_program().is_none() && dir.join("node_modules").is_dir() {
            self.warn(&format!(
                "bun was not found; reusing existing dependencies in {}",
                dir.display()
            ));
            return Ok(());
        }

        self.run_bun(dir, &args)
    }

    pub(crate) fn run_bun(&self, cwd: &Path, args: &[OsString]) -> Result<()> {
        self.run(cwd, bun()?, args)
    }

    pub(crate) fn spawn_bun(&self, cwd: &Path, args: &[OsString]) -> Result<ChildGuard> {
        self.spawn(cwd, bun()?, args)
    }

    pub(crate) fn warn(&self, message: &str) {
        if self.ci {
            println!("::warning::{message}");
        } else {
            eprintln!("warning: {message}");
        }
    }

    pub(crate) fn run<P>(&self, cwd: &Path, program: P, args: &[OsString]) -> Result<()>
    where
        P: AsRef<OsStr>,
    {
        self.run_with_env(cwd, program, args, &[])
    }

    pub(crate) fn spawn<P>(&self, cwd: &Path, program: P, args: &[OsString]) -> Result<ChildGuard>
    where
        P: AsRef<OsStr>,
    {
        let program = program.as_ref();
        self.print_command(cwd, program, args);

        let child = command(cwd, program, args, &[])
            .spawn()
            .map_err(|source| launch_error(program, args, &source))?;

        Ok(ChildGuard {
            child,
            label: format_command(program, args),
        })
    }

    pub(crate) fn run_with_env<P>(
        &self,
        cwd: &Path,
        program: P,
        args: &[OsString],
        envs: &[(&str, &str)],
    ) -> Result<()>
    where
        P: AsRef<OsStr>,
    {
        let program = program.as_ref();
        self.print_command(cwd, program, args);

        let status = command(cwd, program, args, envs)
            .status()
            .map_err(|source| launch_error(program, args, &source))?;

        if status.success() {
            return Ok(());
        }

        fail(format!(
            "command failed with {status}: {}",
            format_command(program, args)
        ))
    }

    /// Runs a command and keeps everything it says until it is done.
    ///
    /// For commands that run alongside each other: twelve wasm builds sharing
    /// this process's stdout interleave line by line into something nobody can
    /// read. The caller prints the returned log when the command finishes, so
    /// each build's output arrives in one piece.
    pub(crate) fn run_captured<P>(
        &self,
        cwd: &Path,
        program: P,
        args: &[OsString],
        envs: &[(&str, &str)],
    ) -> Result<String>
    where
        P: AsRef<OsStr>,
    {
        let program = program.as_ref();
        let mut log = format!(
            "$ (cd {}) {}\n",
            self.location(cwd),
            format_command(program, args)
        );

        let output = command(cwd, program, args, envs)
            .output()
            .map_err(|source| launch_error(program, args, &source))?;

        log.push_str(&String::from_utf8_lossy(&output.stdout));
        log.push_str(&String::from_utf8_lossy(&output.stderr));

        if output.status.success() {
            return Ok(log);
        }

        fail(format!(
            "command failed with {}: {}\n{log}",
            output.status,
            format_command(program, args)
        ))
    }

    pub(crate) fn output_optional<P>(
        &self,
        cwd: &Path,
        program: P,
        args: &[OsString],
    ) -> Result<Option<String>>
    where
        P: AsRef<OsStr>,
    {
        let program = program.as_ref();
        let output = command(cwd, program, args, &[])
            .stderr(Stdio::inherit())
            .output()
            .map_err(|source| launch_error(program, args, &source))?;

        if !output.status.success() {
            return Ok(None);
        }

        let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok((!value.is_empty()).then_some(value))
    }

    pub(crate) fn status_success<P>(
        &self,
        cwd: &Path,
        program: P,
        args: &[OsString],
    ) -> Result<bool>
    where
        P: AsRef<OsStr>,
    {
        let program = program.as_ref();
        Ok(command(cwd, program, args, &[])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|source| launch_error(program, args, &source))?
            .success())
    }

    fn print_command(&self, cwd: &Path, program: &OsStr, args: &[OsString]) {
        println!(
            "$ (cd {}) {}",
            self.location(cwd),
            format_command(program, args)
        );
    }

    fn location(&self, cwd: &Path) -> String {
        let relative = cwd.strip_prefix(&self.root).unwrap_or(cwd);
        if relative.as_os_str().is_empty() {
            ".".to_string()
        } else {
            relative.display().to_string()
        }
    }
}

fn command(cwd: &Path, program: &OsStr, args: &[OsString], envs: &[(&str, &str)]) -> Command {
    let mut command = Command::new(program);
    command.args(args).current_dir(cwd);
    for (key, value) in envs {
        command.env(key, value);
    }
    command
}

fn launch_error(program: &OsStr, args: &[OsString], source: &std::io::Error) -> SiteError {
    SiteError::new(format!(
        "failed to run {}: {source}",
        format_command(program, args)
    ))
}

pub(crate) fn os_args(args: &[&str]) -> Vec<OsString> {
    args.iter().map(OsString::from).collect()
}

fn command_succeeds<P>(program: P, args: &[&str]) -> bool
where
    P: AsRef<OsStr>,
{
    Command::new(program)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

fn bun() -> Result<&'static OsStr> {
    bun_program().ok_or_else(|| {
        SiteError::new("could not find bun; install Bun or run bun install before building").into()
    })
}

/// Where bun is, looked up once per process: the answer costs a `bun --version`
/// and a build asks for it a dozen times.
fn bun_program() -> Option<&'static OsStr> {
    static BUN: OnceLock<Option<OsString>> = OnceLock::new();
    BUN.get_or_init(find_bun).as_deref()
}

fn find_bun() -> Option<OsString> {
    if command_succeeds("bun", &["--version"]) {
        return Some("bun".into());
    }

    if cfg!(windows) {
        let path = env::var_os("USERPROFILE")
            .map(PathBuf::from)
            .map(|home| home.join(".bun/bin/bun.exe"));

        if let Some(path) = path
            && path.is_file()
            && command_succeeds(path.as_os_str(), &["--version"])
        {
            return Some(path.into_os_string());
        }
    }

    None
}

fn format_command(program: &OsStr, args: &[OsString]) -> String {
    let mut parts = vec![format_os(program)];
    parts.extend(args.iter().map(|arg| format_os(arg.as_os_str())));
    parts.join(" ")
}

fn format_os(value: &OsStr) -> String {
    let value = value.to_string_lossy();
    if value.chars().any(char::is_whitespace) {
        format!("{value:?}")
    } else {
        value.into_owned()
    }
}
