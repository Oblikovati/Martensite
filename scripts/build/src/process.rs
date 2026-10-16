//! The single seam to the operating system's processes. Every git, cargo,
//! rustfmt and gh call goes through [`CommandRunner`] so pipelines can be
//! tested with a recording fake instead of real tools.

use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};

use crate::error::{BuildError, BuildResult};

/// A fully described child process: program, arguments and directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSpec {
    program: String,
    args: Vec<String>,
    cwd: PathBuf,
    environment: Vec<(String, Option<String>)>,
}

impl CommandSpec {
    /// Starts a command description run from `cwd`.
    ///
    /// ```ignore
    /// let spec = CommandSpec::new("git", root).args(["status", "--short"]);
    /// ```
    pub fn new(program: &str, cwd: &Path) -> Self {
        Self {
            program: program.to_owned(),
            args: Vec::new(),
            cwd: cwd.to_path_buf(),
            environment: Vec::new(),
        }
    }

    /// Appends one argument.
    pub fn arg(mut self, value: impl Into<String>) -> Self {
        self.args.push(value.into());
        self
    }

    /// Appends several arguments in order.
    pub fn args<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.args.extend(values.into_iter().map(Into::into));
        self
    }

    /// The command line as a user would type it; used in logs and errors.
    pub fn display(&self) -> String {
        std::iter::once(self.program.as_str())
            .chain(self.args.iter().map(String::as_str))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Overrides a child variable, or removes its inherited value.
    /// Example: `spec.environment("RUSTFLAGS", Some("-Zsanitizer=thread"))`.
    pub fn environment(mut self, key: &str, value: Option<&str>) -> Self {
        self.environment
            .push((key.into(), value.map(str::to_owned)));
        self
    }

    fn to_command(&self) -> Command {
        let mut command = Command::new(&self.program);
        command.args(&self.args).current_dir(&self.cwd);
        for (key, value) in &self.environment {
            match value {
                Some(value) => {
                    command.env(key, value);
                }
                None => {
                    command.env_remove(key);
                }
            }
        }
        command
    }
}

/// Runs child processes on behalf of the pipelines.
pub trait CommandRunner {
    /// Runs with inherited stdio; a non-zero exit is an error.
    fn run(&self, spec: &CommandSpec) -> BuildResult<()>;
    /// Returns stdout (stderr stays visible); a non-zero exit is an error.
    fn capture(&self, spec: &CommandSpec) -> BuildResult<String>;
    /// Runs with inherited stdio and returns the raw exit code.
    fn exit_code(&self, spec: &CommandSpec) -> BuildResult<i32>;
    /// Captures status and both byte streams, including unsuccessful exits.
    /// Example: `runner.capture_output(&spec)?` distinguishes Git diagnostics from clean diffs.
    fn capture_output(&self, spec: &CommandSpec) -> BuildResult<CommandOutput>;
}

/// A completed command whose nonzero status still needs tool-specific interpretation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutput {
    /// The normal exit code; signal termination remains a command failure.
    pub code: i32,
    /// Exact standard-output bytes, retained even when they are not UTF-8.
    pub stdout: Vec<u8>,
    /// Exact diagnostic bytes, retained even when they are not UTF-8.
    pub stderr: Vec<u8>,
}

/// The real [`CommandRunner`], backed by `std::process`.
pub struct ProcessRunner;

impl CommandRunner for ProcessRunner {
    fn run(&self, spec: &CommandSpec) -> BuildResult<()> {
        let status = spawn_status(spec)?;
        require_success(spec, status)
    }

    fn capture(&self, spec: &CommandSpec) -> BuildResult<String> {
        let output = spec
            .to_command()
            .stderr(Stdio::inherit())
            .output()
            .map_err(|cause| spawn_error(spec, cause))?;
        require_success(spec, output.status)?;
        String::from_utf8(output.stdout).map_err(|error| BuildError::UnexpectedOutput {
            origin: spec.display(),
            detail: format!("stdout is not UTF-8 ({error}); expected UTF-8 text"),
        })
    }

    fn exit_code(&self, spec: &CommandSpec) -> BuildResult<i32> {
        let status = spawn_status(spec)?;
        // A signal-terminated child has no code; treat it as a hard failure.
        status.code().ok_or_else(|| failure(spec, status))
    }

    fn capture_output(&self, spec: &CommandSpec) -> BuildResult<CommandOutput> {
        let output = spec
            .to_command()
            .output()
            .map_err(|cause| spawn_error(spec, cause))?;
        let code = output
            .status
            .code()
            .ok_or_else(|| failure(spec, output.status))?;
        Ok(CommandOutput {
            code,
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }
}

fn spawn_status(spec: &CommandSpec) -> BuildResult<ExitStatus> {
    spec.to_command()
        .status()
        .map_err(|cause| spawn_error(spec, cause))
}

fn require_success(spec: &CommandSpec, status: ExitStatus) -> BuildResult<()> {
    if status.success() {
        return Ok(());
    }
    Err(failure(spec, status))
}

fn failure(spec: &CommandSpec, status: ExitStatus) -> BuildError {
    BuildError::CommandFailed {
        command: spec.display(),
        status: status.to_string(),
    }
}

fn spawn_error(spec: &CommandSpec, cause: std::io::Error) -> BuildError {
    BuildError::CommandSpawn {
        command: spec.display(),
        cause,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn environment_changes_are_child_only_and_not_logged() {
        let before = std::env::var_os("RUSTFLAGS");
        let spec = CommandSpec::new("cargo", Path::new("."))
            .environment("RUSTFLAGS", Some("-Zsanitizer=thread"))
            .environment("CARGO_ENCODED_RUSTFLAGS", None);
        let command = spec.to_command();
        let values: std::collections::BTreeMap<_, _> = command.get_envs().collect();
        assert_eq!(
            values[std::ffi::OsStr::new("RUSTFLAGS")],
            Some(std::ffi::OsStr::new("-Zsanitizer=thread"))
        );
        assert_eq!(
            values[std::ffi::OsStr::new("CARGO_ENCODED_RUSTFLAGS")],
            None
        );
        assert_eq!(std::env::var_os("RUSTFLAGS"), before);
        assert_eq!(spec.display(), "cargo");
    }

    #[test]
    fn display_joins_program_and_arguments() {
        let spec = CommandSpec::new("cargo", Path::new(".")).args(["test", "--locked"]);
        assert_eq!(spec.display(), "cargo test --locked");
    }

    #[test]
    fn missing_program_is_a_spawn_error_naming_it() {
        let spec = CommandSpec::new("martensite-no-such-program", Path::new("."));
        let error = ProcessRunner.run(&spec).unwrap_err();
        assert!(matches!(error, BuildError::CommandSpawn { ref command, .. }
            if command == "martensite-no-such-program"));
    }

    #[test]
    fn capture_returns_stdout_of_a_real_process() {
        let spec = CommandSpec::new("cargo", Path::new(".")).arg("--version");
        let stdout = ProcessRunner.capture(&spec).unwrap();
        assert!(stdout.starts_with("cargo "), "got {stdout:?}");
    }

    #[test]
    fn failing_process_reports_its_command() {
        // `git diff --quiet` fails without printing, keeping test logs clean.
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a"), "a").unwrap();
        std::fs::write(dir.path().join("b"), "b").unwrap();
        let spec = CommandSpec::new("git", dir.path()).args([
            "diff",
            "--no-index",
            "--quiet",
            "--",
            "a",
            "b",
        ]);
        let error = ProcessRunner.capture(&spec).unwrap_err();
        assert!(matches!(error, BuildError::CommandFailed { .. }), "{error}");
    }

    #[test]
    fn captured_failure_preserves_status_and_diagnostic() {
        let directory = tempfile::tempdir().unwrap();
        let spec = CommandSpec::new("git", directory.path()).args([
            "diff",
            "--no-index",
            "--check",
            "--",
            "/dev/null",
            "missing.txt",
        ]);
        let output = ProcessRunner.capture_output(&spec).unwrap();
        assert_ne!(output.code, 0);
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains("missing.txt"));
    }
}
