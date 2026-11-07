//! Regression controls for #411: skipped captures cannot publish stale evidence.
use std::path::Path;

use super::check_expanded_inventory;
use crate::cargo::BuildTarget;
use crate::context::tests::context_at;
use crate::fake_runner::RecordingRunner;
use crate::git::ChangedFile;

const CLEAN: &str = "cargo clean --target-dir target/unsafe-inventory/build";
const CAPTURE: &str = "cargo check --locked --workspace --all-targets --target test-target --message-format=json --target-dir target/unsafe-inventory/build";
const REPORTS: [&str; 3] = [
    "latest.json",
    "compiler.stdout.jsonl",
    "compiler.stderr.txt",
];

fn inventory_runner(root: &Path) -> RecordingRunner {
    RecordingRunner::default()
        .with_stdout("git rev-parse --show-toplevel", root.to_str().unwrap())
        .with_stdout("git rev-parse HEAD", &"a".repeat(40))
        .with_stdout("rustc -vV", "host: test-target")
        .with_stdout(
            CAPTURE,
            "{\"reason\":\"build-finished\",\"success\":true}\n",
        )
        .with_stdout(
            "cargo metadata --format-version 1 --no-deps --locked",
            &crate::workspace::tests::sample_metadata(),
        )
}

fn changed(path: &str, deleted: bool, untracked: bool) -> ChangedFile {
    ChangedFile {
        path: path.into(),
        deleted,
        untracked,
    }
}

fn seed_evidence(root: &Path) -> std::path::PathBuf {
    let evidence = root.join("target/unsafe-inventory");
    std::fs::create_dir_all(evidence.join("build")).unwrap();
    for name in REPORTS {
        std::fs::write(evidence.join(name), "stale evidence").unwrap();
    }
    std::fs::write(evidence.join("build/keep"), "existing compilation").unwrap();
    evidence
}

const SKIPPED_PATHS: [&str; 4] = [
    "scripts/build/src/thread_sanitizer.rs",
    "scripts/build/src/publish.rs",
    "docs/features/safe-toolkits/closeout.md",
    "docs/features/safe-toolkits/unsafe-source-baseline.json",
];
const CAPTURE_CHANGES: [(&str, bool, bool); 4] = [
    ("crates/a/generated/input.bin", true, false),
    ("external/new/schema.xml", false, true),
    (
        "scripts/build/src/compiler_inventory/ownership.rs",
        false,
        false,
    ),
    (
        "docs/features/safe-toolkits/compiler-baselines/test-target.json",
        false,
        false,
    ),
];

#[test]
fn unrelated_changes_skip_capture_and_remove_only_current_reports() {
    for path in SKIPPED_PATHS {
        assert_skipped_reports_removed(path);
    }
}

fn assert_skipped_reports_removed(path: &str) {
    let root = tempfile::tempdir().unwrap();
    let evidence = seed_evidence(root.path());
    let runner = inventory_runner(root.path());
    check_expanded_inventory(
        &context_at(&runner, root.path()),
        &[changed(path, false, false)],
        &BuildTarget::Host,
    )
    .unwrap();
    assert_eq!(runner.commands(), ["git rev-parse --show-toplevel"]);
    assert!(REPORTS.iter().all(|name| !evidence.join(name).exists()));
    assert!(evidence.join("build/keep").exists());
}

#[test]
fn relevant_added_modified_and_deleted_inputs_capture_fresh_diagnostics() {
    for (path, deleted, untracked) in CAPTURE_CHANGES {
        assert_changed_input_captures(changed(path, deleted, untracked));
    }
}

fn assert_changed_input_captures(input: ChangedFile) {
    let root = tempfile::tempdir().unwrap();
    let evidence = seed_evidence(root.path());
    crate::compiler_inventory::baseline::install_empty_fixture(root.path());
    let runner = inventory_runner(root.path());
    let changes = [changed("README.md", false, false), input];
    check_expanded_inventory(
        &context_at(&runner, root.path()),
        &changes,
        &BuildTarget::Host,
    )
    .unwrap();
    assert_fresh_capture(&runner, &evidence);
}

fn assert_fresh_capture(runner: &RecordingRunner, evidence: &Path) {
    let commands = runner.commands();
    let clean = commands
        .iter()
        .position(|command| command == CLEAN)
        .unwrap();
    let capture = commands
        .iter()
        .position(|command| command == CAPTURE)
        .unwrap();
    assert!(clean < capture);
    let report = std::fs::read_to_string(evidence.join("latest.json")).unwrap();
    assert!(report.contains(&"a".repeat(40)));
    assert!(!report.contains("stale evidence"));
}

#[test]
fn relevant_compiler_failure_propagates_without_a_success_report() {
    let root = tempfile::tempdir().unwrap();
    let evidence = seed_evidence(root.path());
    let runner = inventory_runner(root.path()).with_exit_code(CAPTURE, 1);
    let result = check_expanded_inventory(
        &context_at(&runner, root.path()),
        &[changed("crates/a/src/lib.rs", false, false)],
        &BuildTarget::Host,
    );
    assert!(result.is_err());
    assert!(!evidence.join("latest.json").exists());
    assert!(runner.commands().contains(&CLEAN.into()));
    assert!(runner.commands().contains(&CAPTURE.into()));
}

#[test]
fn skipped_capture_fails_if_stale_evidence_cannot_be_removed() {
    let root = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(root.path().join("target/unsafe-inventory/latest.json")).unwrap();
    let runner = inventory_runner(root.path());
    let result = check_expanded_inventory(
        &context_at(&runner, root.path()),
        &[changed("README.md", false, false)],
        &BuildTarget::Host,
    );
    assert!(result.is_err());
    assert_eq!(runner.commands(), ["git rev-parse --show-toplevel"]);
}

const EARLY_FAILURES: [(&str, i32, &str); 3] = [
    (
        "git merge-base origin/main HEAD",
        1,
        "git merge-base origin/main HEAD",
    ),
    (
        "git -c core.safecrlf=false diff --check base",
        2,
        "whitespace errors",
    ),
    (
        "cargo test --locked --package martensite-build unsafe_inventory",
        1,
        "unsafe_inventory",
    ),
];

#[test]
fn pr_entry_removes_stale_reports_before_git_and_lint_failures() {
    for (command, status, expected) in EARLY_FAILURES {
        assert_early_failure_invalidates_reports(command, status, expected);
    }
}

fn early_failure_runner(root: &Path, command: &str, status: i32) -> RecordingRunner {
    inventory_runner(root)
        .with_stdout("git merge-base origin/main HEAD", "base")
        .with_stdout(
            "git diff --name-status --no-renames -z base",
            "M\0crates/a/src/lib.rs\0",
        )
        .with_exit_code(command, status)
}

fn assert_early_failure_invalidates_reports(command: &str, status: i32, expected: &str) {
    let root = tempfile::tempdir().unwrap();
    let evidence = seed_evidence(root.path());
    let runner = early_failure_runner(root.path(), command, status);
    let context = context_at(&runner, root.path());
    let error =
        super::validate_pr_checks(&context, "origin/main", &BuildTarget::Host, false).unwrap_err();
    assert!(error.to_string().contains(expected), "{error}");
    assert!(runner.commands().contains(&command.to_owned()));
    assert!(!runner.commands().contains(&CLEAN.to_owned()));
    assert!(!runner.commands().contains(&CAPTURE.to_owned()));
    assert!(REPORTS.iter().all(|name| !evidence.join(name).exists()));
    assert_eq!(
        std::fs::read_to_string(evidence.join("build/keep")).unwrap(),
        "existing compilation"
    );
}
