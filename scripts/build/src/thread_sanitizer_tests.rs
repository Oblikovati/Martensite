//! Command contracts keep sanitizer startup refusal from becoming a passing check.
use super::*;
use crate::context::tests::context_at;
use crate::fake_runner::RecordingRunner;

fn runner() -> RecordingRunner {
    let root = "/repo\n";
    RecordingRunner::default().with_stdout("git rev-parse --show-toplevel", root)
}

#[test]
fn startup_precedes_geometry_capture_and_group_controls() {
    let runner = runner();
    run_selected(
        &context_at(&runner, Path::new("/repo")),
        &crate::thread_sanitizer_selection::CONTROLS,
    )
    .unwrap();
    let commands = runner.commands();
    let manifest =
        Path::new("/repo").join("scripts/build/tests/fixtures/thread_sanitizer_startup/Cargo.toml");
    assert_eq!(
        &commands[1..],
        &[
            "rustc +nightly-2026-09-25 -Vv".to_owned(),
            format!("cargo +nightly-2026-09-25 run --locked -Zbuild-std --target x86_64-unknown-linux-gnu --manifest-path {}", manifest.display()),
            "cargo +nightly-2026-09-25 test --locked -Zbuild-std --target x86_64-unknown-linux-gnu --no-fail-fast --package martensite-tkg2d --package martensite-tkg3d --lib cache_inputs_tests -- --test-threads=1 --show-output".to_owned(),
            "cargo +nightly-2026-09-25 test --locked -Zbuild-std --target x86_64-unknown-linux-gnu --no-fail-fast --package martensite-tkservice --test toolkit -- capture_buffer_reference_checks --test-threads=1 --show-output".to_owned(),
            "cargo +nightly-2026-09-25 test --locked -Zbuild-std --target x86_64-unknown-linux-gnu --no-fail-fast --package martensite-tkservice --test toolkit -- graphic3d_group graphic3d_stable_group_nodes --test-threads=1 --show-output".to_owned(),
            "cargo +nightly-2026-09-25 test --locked -Zbuild-std --target x86_64-unknown-linux-gnu --no-fail-fast --package martensite-tkbrep --lib brep::face_input_capture -- --test-threads=1 --show-output".to_owned(),
            "cargo +nightly-2026-09-25 test --locked -Zbuild-std --target x86_64-unknown-linux-gnu --no-fail-fast --package martensite-tktopalgo --test toolkit -- closed_face_validation --test-threads=1 --show-output".to_owned(),
            "cargo +nightly-2026-09-25 test --locked -Zbuild-std --target x86_64-unknown-linux-gnu --no-fail-fast --package martensite-tkmath --lib predicates::roots::open_extrusion -- --test-threads=1 --show-output".to_owned(),
            "cargo +nightly-2026-09-25 test --locked -Zbuild-std --target x86_64-unknown-linux-gnu --no-fail-fast --package martensite-tkmath --lib predicates::roots::sketch_incidence::vertex_neighborhood -- --test-threads=1 --show-output".to_owned(),
            "cargo +nightly-2026-09-25 test --locked -Zbuild-std --target x86_64-unknown-linux-gnu --no-fail-fast --package martensite-tksketch --test surface_sketch -- replay::parallel_operations_match_sequential_value_metadata --exact --test-threads=1 --show-output".to_owned(),
        ]
    );
}

#[test]
fn startup_failure_prevents_geometry_and_reference_capture_execution() {
    let command = startup(Path::new("/repo")).display();
    let runner = runner().with_exit_code(&command, 101);
    let error = run_selected(
        &context_at(&runner, Path::new("/repo")),
        &crate::thread_sanitizer_selection::CONTROLS,
    )
    .unwrap_err();
    assert!(error.to_string().contains("101"));
    assert!(
        !runner
            .commands()
            .iter()
            .any(|item| item.contains("cache_inputs_tests")
                || item.contains("capture_buffer_reference_checks")
                || item.contains("open_extrusion")
                || item.contains("vertex_neighborhood"))
    );
}

#[test]
fn actual_control_failure_is_not_accepted_as_sanitizer_success() {
    let command = cache_controls(
        Path::new("/repo"),
        &["martensite-tkg2d", "martensite-tkg3d"],
    )
    .display();
    let runner = runner().with_exit_code(&command, 66);
    let error = run_selected(
        &context_at(&runner, Path::new("/repo")),
        &crate::thread_sanitizer_selection::CONTROLS,
    )
    .unwrap_err();
    assert!(error.to_string().contains("66"));
    assert_eq!(runner.commands().len(), 4);
    assert!(
        !runner
            .commands()
            .iter()
            .any(|item| item.contains("capture_buffer_reference_checks"))
    );
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
        workflow.contains("cargo xtask ci gate --checks \"$PLAN_RESULT\" --tests \"$SANITIZER_RESULT\" $HAS_SANITIZER_FLAG")
    );
    assert!(workflow.contains("SANITIZER_RESULT: ${{ needs.thread-sanitizer.result }}"));
}

#[test]
fn reference_capture_failure_refuses_the_required_sanitizer_check() {
    let command = capture_controls(Path::new("/repo")).display();
    let runner = runner().with_exit_code(&command, 66);
    let error = run_selected(
        &context_at(&runner, Path::new("/repo")),
        &crate::thread_sanitizer_selection::CONTROLS,
    )
    .unwrap_err();
    assert!(error.to_string().contains("66"));
    assert_eq!(runner.commands().len(), 5);
}
#[test]
fn reference_capture_uses_the_same_instrumentation_as_startup() {
    let root = Path::new("/repo");
    let command = capture_controls(root);
    let expected = sanitized_cargo(root, "test").args([
        "--no-fail-fast",
        "--package",
        "martensite-tkservice",
        "--test",
        "toolkit",
        "--",
        "capture_buffer_reference_checks",
        "--test-threads=1",
        "--show-output",
    ]);
    assert_eq!(command, expected);
}

#[test]
fn group_failure_refuses_the_required_sanitizer_check() {
    let command = group_controls(Path::new("/repo")).display();
    let runner = runner().with_exit_code(&command, 66);
    let error = run_selected(
        &context_at(&runner, Path::new("/repo")),
        &crate::thread_sanitizer_selection::CONTROLS,
    )
    .unwrap_err();
    assert!(error.to_string().contains("66"));
    assert_eq!(runner.commands().len(), 6);
}

#[test]
fn groups_use_the_same_instrumentation_with_both_actual_module_filters() {
    let root = Path::new("/repo");
    let command = group_controls(root);
    let expected = sanitized_cargo(root, "test").args([
        "--no-fail-fast",
        "--package",
        "martensite-tkservice",
        "--test",
        "toolkit",
        "--",
        "graphic3d_group",
        "graphic3d_stable_group_nodes",
        "--test-threads=1",
        "--show-output",
    ]);
    assert_eq!(command, expected);
}

#[test]
fn coherent_face_capture_failure_refuses_the_required_sanitizer_check() {
    let command = face_input_controls(Path::new("/repo")).display();
    let runner = runner().with_exit_code(&command, 66);
    let error = run_selected(
        &context_at(&runner, Path::new("/repo")),
        &crate::thread_sanitizer_selection::CONTROLS,
    )
    .unwrap_err();
    assert!(error.to_string().contains("66"));
    assert_eq!(runner.commands().len(), 7);
}
#[test]
fn coherent_face_capture_uses_the_same_pinned_instrumentation() {
    let root = Path::new("/repo");
    let expected = sanitized_cargo(root, "test").args([
        "--no-fail-fast",
        "--package",
        "martensite-tkbrep",
        "--lib",
        "brep::face_input_capture",
        "--",
        "--test-threads=1",
        "--show-output",
    ]);
    assert_eq!(face_input_controls(root), expected);
}

#[test]
fn owned_face_certificate_failure_refuses_the_required_sanitizer_check() {
    let command = face_certificate_controls(Path::new("/repo")).display();
    let runner = runner().with_exit_code(&command, 66);
    let error = run_selected(
        &context_at(&runner, Path::new("/repo")),
        &crate::thread_sanitizer_selection::CONTROLS,
    )
    .unwrap_err();
    assert!(error.to_string().contains("66"));
    assert_eq!(runner.commands().len(), 8);
}
#[test]
fn owned_face_certificate_group_uses_the_same_pinned_instrumentation() {
    let root = Path::new("/repo");
    let expected = sanitized_cargo(root, "test").args([
        "--no-fail-fast",
        "--package",
        "martensite-tktopalgo",
        "--test",
        "toolkit",
        "--",
        "closed_face_validation",
        "--test-threads=1",
        "--show-output",
    ]);
    assert_eq!(face_certificate_controls(root), expected);
}

#[test]
fn open_support_failure_refuses_the_required_sanitizer_check() {
    let command = open_extrusion_controls(Path::new("/repo")).display();
    let runner = runner().with_exit_code(&command, 66);
    let error = run_selected(
        &context_at(&runner, Path::new("/repo")),
        &crate::thread_sanitizer_selection::CONTROLS,
    )
    .unwrap_err();
    assert!(error.to_string().contains("66"));
    assert_eq!(runner.commands().len(), 9);
}
#[test]
fn open_support_group_uses_the_same_pinned_instrumentation() {
    let root = Path::new("/repo");
    let expected = sanitized_cargo(root, "test").args([
        "--no-fail-fast",
        "--package",
        "martensite-tkmath",
        "--lib",
        "predicates::roots::open_extrusion",
        "--",
        "--test-threads=1",
        "--show-output",
    ]);
    assert_eq!(open_extrusion_controls(root), expected);
}

#[test]
fn vertex_incidence_failure_refuses_the_required_sanitizer_check() {
    let command = vertex_neighborhood_controls(Path::new("/repo")).display();
    let runner = runner().with_exit_code(&command, 66);
    let error = run_selected(
        &context_at(&runner, Path::new("/repo")),
        &crate::thread_sanitizer_selection::CONTROLS,
    )
    .unwrap_err();
    assert!(error.to_string().contains("66"));
    assert_eq!(runner.commands().len(), 10);
}
#[test]
fn vertex_incidence_group_keeps_the_pinned_sanitizer_settings() {
    let root = Path::new("/repo");
    let expected = sanitized_cargo(root, "test").args([
        "--no-fail-fast",
        "--package",
        "martensite-tkmath",
        "--lib",
        "predicates::roots::sketch_incidence::vertex_neighborhood",
        "--",
        "--test-threads=1",
        "--show-output",
    ]);
    assert_eq!(vertex_neighborhood_controls(root), expected);
}

#[test]
fn empty_selection_runs_no_sanitizer_commands() {
    let runner = runner();
    let context = context_at(&runner, Path::new("/repo"));
    let before = runner.commands();
    run_selected(&context, &[]).unwrap();
    assert_eq!(runner.commands(), before);
}

#[test]
fn cache_leaf_selection_excludes_unaffected_base_controls() {
    let runner = runner();
    run_selected(
        &context_at(&runner, Path::new("/repo")),
        &[Control::Cache3d],
    )
    .unwrap();
    let commands = runner.commands();
    assert_eq!(commands.len(), 4);
    assert!(commands[3].contains("--package martensite-tkg3d"));
    assert!(
        !commands
            .iter()
            .any(|command| command.contains("--package martensite-tkg2d"))
    );
}

#[test]
fn every_control_command_targets_its_declared_owner() {
    for control in crate::thread_sanitizer_selection::CONTROLS {
        let expected = format!("--package {} ", control.owner());
        let command = control_command(Path::new("/repo"), control).display();
        assert!(command.contains(&expected), "{control:?}: {command}");
        assert_eq!(
            command.matches("--package ").count(),
            1,
            "{control:?}: {command}"
        );
    }
}

#[test]
fn surface_sketch_control_uses_pinned_instrumentation_and_exact_replay_test() {
    let root = Path::new("/repo");
    let expected = sanitized_cargo(root, "test").args([
        "--no-fail-fast",
        "--package",
        "martensite-tksketch",
        "--test",
        "surface_sketch",
        "--",
        "replay::parallel_operations_match_sequential_value_metadata",
        "--exact",
        "--test-threads=1",
        "--show-output",
    ]);
    assert_eq!(control_command(root, Control::SurfaceSketch), expected);
}

#[test]
fn surface_sketch_failure_refuses_sanitizer_success() {
    let command = surface_sketch_controls(Path::new("/repo")).display();
    let runner = runner().with_exit_code(&command, 66);
    let context = context_at(&runner, Path::new("/repo"));
    let error = run_selected(&context, &[Control::SurfaceSketch]).unwrap_err();
    assert!(error.to_string().contains("66"));
    assert_eq!(runner.commands().len(), 4);
}
