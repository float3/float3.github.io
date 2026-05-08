use crate::{Result, Site, SiteError};
use std::env;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

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
            InstallMode::Locked => os_args(&["install"]),
            InstallMode::Unlocked => os_args(&["install"]),
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
        if let Some(program) = bun_program() {
            return self.run(cwd, program, args);
        }

        Err(Box::new(SiteError::new(
            "could not find bun; install Bun or run bun install before building",
        )))
    }

    pub(crate) fn git_iso_date(&self, args: &[OsString]) -> Result<Option<String>> {
        let Some(output) = self.output_optional(&self.root, "git", args)? else {
            return Ok(None);
        };

        Ok(output
            .split('T')
            .next()
            .filter(|value| !value.is_empty())
            .map(str::to_string))
    }

    pub(crate) fn relative_git_path(&self, path: &Path) -> Result<String> {
        Ok(path
            .strip_prefix(&self.root)?
            .to_string_lossy()
            .replace('\\', "/"))
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

        let child = Command::new(program)
            .args(args)
            .current_dir(cwd)
            .spawn()
            .map_err(|source| {
                SiteError(format!(
                    "failed to run {}: {source}",
                    format_command(program, args)
                ))
            })?;

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

        let mut command = Command::new(program);
        command.args(args).current_dir(cwd);
        for (key, value) in envs {
            command.env(key, value);
        }

        let status = command.status().map_err(|source| {
            SiteError(format!(
                "failed to run {}: {source}",
                format_command(program, args)
            ))
        })?;

        if status.success() {
            Ok(())
        } else {
            Err(Box::new(SiteError(format!(
                "command failed with {status}: {}",
                format_command(program, args)
            ))))
        }
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
        let output = Command::new(program)
            .args(args)
            .current_dir(cwd)
            .stderr(Stdio::inherit())
            .output()
            .map_err(|source| {
                SiteError(format!(
                    "failed to run {}: {source}",
                    format_command(program, args)
                ))
            })?;

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
        Ok(Command::new(program)
            .args(args)
            .current_dir(cwd)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|source| {
                SiteError(format!(
                    "failed to run {}: {source}",
                    format_command(program, args)
                ))
            })?
            .success())
    }

    fn print_command(&self, cwd: &Path, program: &OsStr, args: &[OsString]) {
        let relative = cwd.strip_prefix(&self.root).unwrap_or(cwd);
        let label = if relative.as_os_str().is_empty() {
            ".".to_string()
        } else {
            relative.display().to_string()
        };

        println!("$ (cd {label}) {}", format_command(program, args));
    }
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

fn bun_program() -> Option<OsString> {
    if command_succeeds("bun", &["--version"]) {
        return Some("bun".into());
    }

    if cfg!(windows) {
        let path = env::var_os("USERPROFILE")
            .map(PathBuf::from)
            .map(|home| home.join(".bun/bin/bun.exe"));

        if let Some(path) = path {
            if path.is_file() && command_succeeds(path.as_os_str(), &["--version"]) {
                return Some(path.into_os_string());
            }
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
