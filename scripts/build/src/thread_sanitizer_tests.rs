//! Command contracts keep sanitizer startup refusal from becoming a passing check.
use super::*;
use crate::context::tests::context_at;
use crate::fake_runner::RecordingRunner;

fn runner() -> RecordingRunner {
    let root = "/repo\n";
    RecordingRunner::default().with_stdout("git rev-parse --show-toplevel", root)
}

#[test]
fn startup_precedes_both_real_geometry_control_sets() {
    let runner = runner();
    run(&context_at(&runner, Path::new("/repo"))).unwrap();
    let commands = runner.commands();
    let manifest =
        Path::new("/repo").join("scripts/build/tests/fixtures/thread_sanitizer_startup/Cargo.toml");
    assert_eq!(
        &commands[1..],
        &[
            "rustc +nightly-2026-09-25 -Vv".to_owned(),
            format!("cargo +nightly-2026-09-25 run --locked -Zbuild-std --target x86_64-unknown-linux-gnu --manifest-path {}", manifest.display()),
            "cargo +nightly-2026-09-25 test --locked -Zbuild-std --target x86_64-unknown-linux-gnu --no-fail-fast --package martensite-tkg2d --package martensite-tkg3d --lib cache_inputs_tests -- --test-threads=1 --show-output".to_owned(),
        ]
    );
}

#[test]
fn startup_failure_prevents_any_geometry_control_execution() {
    let command = startup(Path::new("/repo")).display();
    let runner = runner().with_exit_code(&command, 101);
    let error = run(&context_at(&runner, Path::new("/repo"))).unwrap_err();
    assert!(error.to_string().contains("101"));
    assert!(
        !runner
            .commands()
            .iter()
            .any(|item| item.contains("cache_inputs_tests"))
    );
}

#[test]
fn actual_control_failure_is_not_accepted_as_sanitizer_success() {
    let command = controls(Path::new("/repo")).display();
    let runner = runner().with_exit_code(&command, 66);
    let error = run(&context_at(&runner, Path::new("/repo"))).unwrap_err();
    assert!(error.to_string().contains("66"));
    assert_eq!(runner.commands().len(), 4);
}

fn expected_instrumentation(root: &Path) -> CommandSpec {
    let target = root.join("target/thread-sanitizer");
    CommandSpec::new("cargo", root)
        .environment_os("CARGO_TARGET_DIR", Some(target.as_os_str()))
        .environment(
            "RUSTFLAGS",
            Some("-Zsanitizer=thread -Cforce-frame-pointers=yes"),
        )
        .environment("CARGO_ENCODED_RUSTFLAGS", None)
        .environment("RUSTC_WRAPPER", None)
        .environment("RUSTC_WORKSPACE_WRAPPER", None)
        .environment("TSAN_OPTIONS", None)
}

#[test]
fn child_environment_keeps_instrumentation_and_refuses_inherited_suppressions() {
    let root = Path::new("/repo");
    let expected = expected_instrumentation(root)
        .environment("CARGO_BUILD_JOBS", Some("2"))
        .environment("CARGO_INCREMENTAL", Some("0"))
        .environment("CARGO_PROFILE_DEV_DEBUG", Some("0"))
        .environment("CARGO_PROFILE_TEST_DEBUG", Some("0"))
        .environment("LD_PRELOAD", None)
        .environment("LD_LIBRARY_PATH", None);
    assert_eq!(
        sanitizer_environment(CommandSpec::new("cargo", root), root),
        expected
    );
}

#[test]
fn workflow_runs_the_pinned_sanitizer_and_requires_its_actual_job_result() {
    // A Windows checkout may use CRLF; only line-ending spelling is irrelevant (#313).
    let workflow = include_str!("../../../.github/workflows/martensite.yml").replace("\r\n", "\n");
    assert!(workflow.contains("  thread-sanitizer:\n    needs: plan\n"));
    assert!(workflow.contains(
        "rustup toolchain install nightly-2026-09-25 --profile minimal --component rust-src"
    ));
    assert!(workflow.contains("run: cargo xtask ci thread-sanitizer"));
    assert!(workflow.contains("needs: [plan, checks, tests, release-tests, thread-sanitizer]"));
    assert!(
        workflow.contains("cargo xtask ci gate --checks \"$SANITIZER_RESULT\" --tests skipped")
    );
    assert!(workflow.contains("SANITIZER_RESULT: ${{ needs.thread-sanitizer.result }}"));
}
