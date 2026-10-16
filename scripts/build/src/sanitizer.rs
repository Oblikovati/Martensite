//! Opt-in toolkit race checks; this is not the kernel-wide corpus gate (#66).

use std::path::Path;

use crate::error::{BuildError, BuildResult};
use crate::process::{CommandRunner, CommandSpec};
use crate::targets::PRIMARY_TARGET;

const TOOLCHAIN: &str = "nightly-2026-09-25";

/// Runs instrumented toolkit unit tests without changing the parent environment.
/// Example: `validate(runner, root, "x86_64-unknown-linux-gnu")?`.
pub fn validate(runner: &dyn CommandRunner, root: &Path, triple: &str) -> BuildResult<()> {
    if triple != PRIMARY_TARGET.triple {
        return Err(BuildError::Usage(format!(
            "sanitizer target {triple:?}; expected {:?}",
            PRIMARY_TARGET.triple
        )));
    }
    runner.run(&CommandSpec::new("rustup", root).args([
        "toolchain",
        "install",
        TOOLCHAIN,
        "--profile",
        "minimal",
        "--component",
        "rust-src",
    ]))?;
    runner.run(&test_command(root, triple))
}

fn test_command(root: &Path, triple: &str) -> CommandSpec {
    CommandSpec::new("cargo", root)
        .arg(format!("+{TOOLCHAIN}"))
        .args([
            "test",
            "--locked",
            "-Zbuild-std",
            "--target",
            triple,
            "--target-dir",
            "target/toolkit-tsan",
            "-p",
            "martensite-tkernel",
            "--lib",
        ])
        .environment("RUSTFLAGS", Some("-Zsanitizer=thread"))
        .environment("CARGO_ENCODED_RUSTFLAGS", None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake_runner::RecordingRunner;

    #[test]
    fn sanitizer_failures_are_not_accepted_as_success() {
        let root = Path::new("/repo");
        let command = test_command(root, PRIMARY_TARGET.triple).display();
        let runner = RecordingRunner::default().with_exit_code(&command, 66);
        let error = validate(&runner, root, PRIMARY_TARGET.triple).unwrap_err();
        assert!(matches!(error, BuildError::CommandFailed { status, .. } if status == "66"));
    }

    #[test]
    fn failed_toolchain_installation_stops_before_tests() {
        let command =
            format!("rustup toolchain install {TOOLCHAIN} --profile minimal --component rust-src");
        let runner = RecordingRunner::default().with_exit_code(&command, 1);
        assert!(validate(&runner, Path::new("/repo"), PRIMARY_TARGET.triple).is_err());
        assert_eq!(runner.commands(), [command]);
    }

    #[test]
    fn installs_pinned_compiler_before_instrumented_tests() {
        let runner = RecordingRunner::default();
        validate(&runner, Path::new("/repo"), PRIMARY_TARGET.triple).unwrap();
        let commands = runner.commands();
        assert_eq!(commands.len(), 2);
        assert!(commands[0].contains(TOOLCHAIN));
        assert!(commands[0].contains("--component rust-src"));
        assert_eq!(
            commands[1],
            test_command(Path::new("/repo"), PRIMARY_TARGET.triple).display()
        );
        assert!(commands[1].contains("-Zbuild-std"));
    }

    #[test]
    fn rejects_unsupported_target_before_installation() {
        let runner = RecordingRunner::default();
        assert!(validate(&runner, Path::new("/repo"), "aarch64-apple-darwin").is_err());
        assert!(runner.commands().is_empty());
    }
}
