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

    fn to_command(&self) -> Command {
        let mut command = Command::new(&self.program);
        command.args(&self.args).current_dir(&self.cwd);
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
}
