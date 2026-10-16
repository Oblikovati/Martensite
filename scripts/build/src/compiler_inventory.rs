//! Expanded unsafe diagnostics are collected separately from acceptance lints (#81).
pub(crate) mod baseline;
mod ownership;
mod parser;
mod provenance;

use std::path::Path;

use crate::cargo::{BuildTarget, CargoTasks};
use crate::error::{BuildError, BuildResult, io_error};
use crate::process::{CommandOutput, CommandRunner, CommandSpec};

/// Captures expanded diagnostics without weakening the normal Clippy gate.
/// Example: `capture_inventory(runner, root, &BuildTarget::Host)?`.
pub fn capture_inventory(
    runner: &dyn CommandRunner,
    root: &Path,
    target: &BuildTarget,
) -> BuildResult<()> {
    invalidate_report(root)?;
    let triple = match target {
        BuildTarget::Host => CargoTasks::new(runner, root).host_triple()?,
        BuildTarget::Triple(triple) => triple.clone(),
    };
    let spec = inventory_command(root, &triple);
    // Cargo does not reliably replay cached dependency warnings (#81).
    runner.run(&CommandSpec::new("cargo", root).args([
        "clean",
        "--target-dir",
        "target/unsafe-inventory/build",
    ]))?;
    let output = runner.capture_output(&spec)?;
    preserve_compiler_output(root, &output)?;
    require_compiler_success(&spec, &output)?;
    let stdout =
        std::str::from_utf8(&output.stdout).map_err(|error| output_error(error.to_string()))?;
    let diagnostics = parser::collect_stream(stdout, output.code).map_err(output_error)?;
    let diagnostics = ownership::classify(runner, root, diagnostics)?;
    let provenance = provenance::capture(runner, root, &spec)?;
    write_report(root, &triple, &diagnostics, &provenance)?;
    baseline::verify(
        root,
        &triple,
        &provenance.compiler,
        &diagnostics.workspace_summary,
    )
}

fn invalidate_report(root: &Path) -> BuildResult<()> {
    let path = root.join("target/unsafe-inventory/latest.json");
    match std::fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(cause) if cause.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(cause) => Err(io_error(&path, cause)),
    }
}

fn preserve_compiler_output(root: &Path, output: &CommandOutput) -> BuildResult<()> {
    let directory = root.join("target/unsafe-inventory");
    std::fs::create_dir_all(&directory).map_err(|cause| io_error(&directory, cause))?;
    for (name, bytes) in [
        ("compiler.stdout.jsonl", &output.stdout),
        ("compiler.stderr.txt", &output.stderr),
    ] {
        let path = directory.join(name);
        std::fs::write(&path, bytes).map_err(|cause| io_error(&path, cause))?;
    }
    Ok(())
}

fn require_compiler_success(spec: &CommandSpec, output: &CommandOutput) -> BuildResult<()> {
    if output.code == 0 {
        return Ok(());
    }
    Err(BuildError::CommandFailed {
        command: spec.display(),
        status: format!(
            "exit {}; compiler streams saved under target/unsafe-inventory",
            output.code
        ),
    })
}

fn inventory_command(root: &Path, triple: &str) -> CommandSpec {
    CommandSpec::new("cargo", root)
        .args([
            "check",
            "--locked",
            "--workspace",
            "--all-targets",
            "--target",
            triple,
            "--message-format=json",
            "--target-dir",
            "target/unsafe-inventory/build",
        ])
        .environment("RUSTFLAGS", Some("--force-warn unsafe_code"))
        .environment("CARGO_ENCODED_RUSTFLAGS", None)
}

fn output_error(detail: String) -> BuildError {
    BuildError::UnexpectedOutput {
        origin: "compiler unsafe inventory".into(),
        detail,
    }
}

fn write_report(
    root: &Path,
    triple: &str,
    diagnostics: &ownership::ClassifiedDiagnostics,
    provenance: &provenance::Provenance,
) -> BuildResult<()> {
    let directory = root.join("target/unsafe-inventory");
    std::fs::create_dir_all(&directory).map_err(|cause| io_error(&directory, cause))?;
    let path = directory.join("latest.json");
    let report = serde_json::json!({"target":triple, "provenance":provenance,
        "scope":"workspace all-targets, default features; dependencies included by package_id",
        "measurement":"emitted diagnostics, including repeated compilation units; not unique sites",
        "summary_measurement":"Emitted primary spans grouped by package, target, source, compiler diagnostic and expansion depth; repeated compilations remain counted.",
        "summary_limitations":"Equal-count substitutions within one group are not detected by summary counts; full diagnostics retain locations and macro expansion chains.",
        "status":"collection complete; baseline comparison follows report publication; normal Clippy remains a separate gate",
        "diagnostics":diagnostics});
    let bytes =
        serde_json::to_vec_pretty(&report).map_err(|error| output_error(error.to_string()))?;
    std::fs::write(&path, bytes).map_err(|cause| io_error(&path, cause))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake_runner::RecordingRunner;

    #[test]
    fn collector_uses_locked_workspace_scope_and_writes_target_report() {
        let directory = tempfile::tempdir().unwrap();
        baseline::install_empty_fixture(directory.path());
        let spec = inventory_command(directory.path(), "test-target");
        assert!(
            spec.display()
                .contains("--locked --workspace --all-targets --target test-target")
        );
        let runner = RecordingRunner::default()
            .with_stdout("git rev-parse HEAD", &"a".repeat(40))
            .with_stdout("rustc -vV", "host: test-target\n")
            .with_stdout(
                "cargo metadata --format-version 1 --no-deps --locked",
                r#"{"workspace_members":["workspace"],"packages":[{"id":"workspace","name":"workspace","version":"1.0.0"}]}"#,
            )
            .with_stdout(
                &spec.display(),
                "{\"reason\":\"build-finished\",\"success\":true}\n",
            );
        capture_inventory(
            &runner,
            directory.path(),
            &BuildTarget::Triple("test-target".into()),
        )
        .unwrap();
        let bytes =
            std::fs::read(directory.path().join("target/unsafe-inventory/latest.json")).unwrap();
        let report: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(report["target"], "test-target");
        let commands = runner.commands();
        assert_eq!(
            commands[0],
            "cargo clean --target-dir target/unsafe-inventory/build"
        );
        assert_eq!(commands[1], spec.display());
        assert_eq!(
            report["diagnostics"],
            serde_json::json!({"workspace_summary":[], "workspace":[], "dependencies":[]})
        );
    }

    #[test]
    fn failed_compiler_does_not_publish_a_report() {
        let directory = tempfile::tempdir().unwrap();
        let spec = inventory_command(directory.path(), "test-target");
        let runner = RecordingRunner::default().with_exit_code(&spec.display(), 101);
        assert!(
            capture_inventory(
                &runner,
                directory.path(),
                &BuildTarget::Triple("test-target".into())
            )
            .is_err()
        );
        assert!(
            !directory
                .path()
                .join("target/unsafe-inventory/latest.json")
                .exists()
        );
    }

    #[test]
    fn failed_refresh_removes_old_report_and_preserves_both_compiler_streams() {
        let directory = tempfile::tempdir().unwrap();
        let report_dir = directory.path().join("target/unsafe-inventory");
        std::fs::create_dir_all(&report_dir).unwrap();
        std::fs::write(report_dir.join("latest.json"), "old report").unwrap();
        let spec = inventory_command(directory.path(), "test-target");
        let runner = RecordingRunner::default()
            .with_exit_code(&spec.display(), 101)
            .with_stdout(&spec.display(), "compiler diagnostic")
            .with_stderr(&spec.display(), "link failed");
        let error = capture_inventory(
            &runner,
            directory.path(),
            &BuildTarget::Triple("test-target".into()),
        )
        .unwrap_err();
        assert!(error.to_string().contains("exit 101"));
        assert!(!report_dir.join("latest.json").exists());
        assert_eq!(
            std::fs::read_to_string(report_dir.join("compiler.stdout.jsonl")).unwrap(),
            "compiler diagnostic"
        );
        assert_eq!(
            std::fs::read_to_string(report_dir.join("compiler.stderr.txt")).unwrap(),
            "link failed"
        );
    }

    #[test]
    fn malformed_successful_refresh_cannot_leave_an_old_report() {
        let directory = tempfile::tempdir().unwrap();
        let report_dir = directory.path().join("target/unsafe-inventory");
        std::fs::create_dir_all(&report_dir).unwrap();
        std::fs::write(report_dir.join("latest.json"), "old report").unwrap();
        let spec = inventory_command(directory.path(), "test-target");
        let runner = RecordingRunner::default().with_stdout(&spec.display(), "{truncated");
        assert!(
            capture_inventory(
                &runner,
                directory.path(),
                &BuildTarget::Triple("test-target".into())
            )
            .is_err()
        );
        assert!(!report_dir.join("latest.json").exists());
    }
}
