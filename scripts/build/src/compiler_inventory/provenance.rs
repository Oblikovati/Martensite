//! Revision and compiler observations for interpreting, not certifying, a report.
use std::path::Path;

use serde::Serialize;

use crate::error::BuildResult;
use crate::process::{CommandRunner, CommandSpec};

#[derive(Debug, Serialize)]
pub(super) struct Provenance {
    revision: String,
    working_tree_status: String,
    pub(super) compiler: String,
    command: String,
    rustflags: &'static str,
    observation: &'static str,
}

pub(super) fn capture(
    runner: &dyn CommandRunner,
    root: &Path,
    command: &CommandSpec,
) -> BuildResult<Provenance> {
    let revision = runner.capture(&CommandSpec::new("git", root).args(["rev-parse", "HEAD"]))?;
    let revision = checked_revision(&revision)?;
    let working_tree_status = runner.capture(&CommandSpec::new("git", root).args([
        "status",
        "--porcelain=v1",
        "--untracked-files=all",
    ]))?;
    let compiler = capture_compiler(runner, root)?;
    Ok(Provenance {
        revision,
        working_tree_status,
        compiler,
        command: command.display(),
        rustflags: "--force-warn unsafe_code",
        observation: "Observed after compilation; dirty files are listed, not snapshotted. Not proof against concurrent edits.",
    })
}

fn capture_compiler(runner: &dyn CommandRunner, root: &Path) -> BuildResult<String> {
    let compiler = runner.capture(&CommandSpec::new("rustc", root).arg("-vV"))?;
    if !compiler.lines().any(|line| line.starts_with("host: ")) {
        return Err(super::output_error(format!(
            "compiler {compiler:?}; expected rustc -vV with host"
        )));
    }
    Ok(compiler)
}

fn checked_revision(value: &str) -> BuildResult<String> {
    let revision = value.trim();
    if ![40, 64].contains(&revision.len()) || !revision.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(super::output_error(format!(
            "revision {value:?}; expected full hexadecimal Git object ID"
        )));
    }
    Ok(revision.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake_runner::RecordingRunner;

    #[test]
    fn provenance_keeps_dirty_status_and_compiler_identity() {
        let runner = RecordingRunner::default()
            .with_stdout("git rev-parse HEAD", &"a".repeat(40))
            .with_stdout(
                "git status --porcelain=v1 --untracked-files=all",
                " M crates/lib.rs\n?? new.rs\n",
            )
            .with_stdout("rustc -vV", "rustc pinned\nhost: test-host\n");
        let spec = CommandSpec::new("cargo", Path::new("/repo")).arg("check");
        let record = capture(&runner, Path::new("/repo"), &spec).unwrap();
        assert_eq!(record.revision, "a".repeat(40));
        assert!(record.working_tree_status.contains("?? new.rs"));
        assert!(record.compiler.contains("host: test-host"));
        assert_eq!(record.command, "cargo check");
    }

    #[test]
    fn provenance_refuses_missing_or_abbreviated_revision() {
        for value in ["", "abc123", &"z".repeat(40)] {
            assert!(checked_revision(value).is_err());
        }
        assert!(checked_revision(&"b".repeat(64)).is_ok());
    }

    #[test]
    fn provenance_refuses_missing_compiler_identity() {
        let runner = RecordingRunner::default().with_stdout("git rev-parse HEAD", &"a".repeat(40));
        let spec = CommandSpec::new("cargo", Path::new("/repo"));
        assert!(capture(&runner, Path::new("/repo"), &spec).is_err());
    }
}
