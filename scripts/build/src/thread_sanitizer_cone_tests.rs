//! Cone concurrency is an explicit affected-sketch control, independent of cylinder stress.
use super::*;
use crate::thread_sanitizer_selection::select;
use std::collections::BTreeSet;
const CONE_SOURCE: &str = include_str!(
    "../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/tests/completed/concurrency.rs"
);
const COMPLETED_SOURCE: &str = include_str!(
    "../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/tests/completed.rs"
);
#[test]
fn cone_selection_keeps_existing_sketch_controls_and_stable_identity() {
    assert_eq!(Control::WrappedCone.id(), "wrapped-cone");
    assert_eq!(Control::WrappedCone.owner(), "martensite-tksketch");
    assert_eq!(
        select(&BTreeSet::from(["martensite-tksketch".into()]), &[]),
        [
            Control::SurfaceSketch,
            Control::WrappedFamily,
            Control::WrappedCone
        ]
    );
}
#[test]
fn cone_command_selects_one_exact_nonignored_accepted_public_control() {
    let root = Path::new("/repo");
    let expected = sanitized_cargo(root, "test").args([
        "--no-fail-fast",
        "--package",
        "martensite-tksketch",
        "--lib",
        WRAPPED_CONE_TEST,
        "--",
        "--exact",
        "--test-threads=1",
        "--show-output",
    ]);
    assert_eq!(control_command(root, Control::WrappedCone), expected);
    assert!(!expected.display().contains("--ignored"));
}
#[test]
fn cone_control_names_the_existing_accepted_threaded_test() {
    assert!(WRAPPED_CONE_TEST.ends_with(
        "::overlapping_public_cone_operations_and_shared_readers_preserve_owned_evidence"
    ));
    assert!(CONE_SOURCE.contains(
        "fn overlapping_public_cone_operations_and_shared_readers_preserve_owned_evidence()"
    ));
    assert!(!CONE_SOURCE.contains("#[ignore"));
    assert!(COMPLETED_SOURCE.contains("mod concurrency;"));
}
#[test]
fn cone_failure_cannot_be_reported_as_sanitizer_success() {
    let command = wrapped_cone_controls(Path::new("/repo")).display();
    let runner = runner().with_exit_code(&command, 66);
    let error = run_selected(
        &context_at(&runner, Path::new("/repo")),
        &[Control::WrappedCone],
    )
    .unwrap_err();
    assert!(error.to_string().contains("66"));
    assert_eq!(runner.commands().len(), 4);
    assert_eq!(runner.commands()[3], command);
}
#[test]
fn startup_failure_prevents_the_cone_closure_from_running() {
    let command = startup(Path::new("/repo")).display();
    let runner = runner().with_exit_code(&command, 139);
    assert!(
        run_selected(
            &context_at(&runner, Path::new("/repo")),
            &[Control::WrappedCone]
        )
        .is_err()
    );
    assert_eq!(runner.commands().len(), 3);
    assert!(
        !runner
            .commands()
            .iter()
            .any(|command| command.contains(WRAPPED_CONE_TEST))
    );
}
