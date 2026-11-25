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

const ATTACHMENT_SOURCE: &str = include_str!(
    "../../../crates/martensite-tkv3d/tests/wrapped_text_regions/conical_text/attachment/certified.rs"
);
const ATTACHMENT_MODULES: [(&str, &str); 5] = [
    (
        include_str!("../../../crates/martensite-tkv3d/tests/toolkit.rs"),
        "wrapped_text_regions",
    ),
    (
        include_str!("../../../crates/martensite-tkv3d/tests/wrapped_text_regions.rs"),
        "wrapped",
    ),
    (
        include_str!("../../../crates/martensite-tkv3d/tests/wrapped_text_regions/wrapped.rs"),
        "conical_text",
    ),
    (
        include_str!("../../../crates/martensite-tkv3d/tests/wrapped_text_regions/conical_text.rs"),
        "attachment",
    ),
    (
        include_str!(
            "../../../crates/martensite-tkv3d/tests/wrapped_text_regions/conical_text/attachment.rs"
        ),
        "certified",
    ),
];
#[test]
fn cone_selection_keeps_existing_sketch_controls_and_stable_identity() {
    assert_eq!(Control::WrappedCone.id(), "wrapped-cone");
    assert_eq!(Control::WrappedCone.owner(), "martensite-tksketch");
    assert_eq!(
        select(&BTreeSet::from(["martensite-tksketch".into()]), &[]),
        [
            Control::SurfaceSketch,
            Control::WrappedFamily,
            Control::WrappedCone,
            Control::CertifiedCylinderAttachment
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

#[test]
fn certified_cone_command_selects_the_exact_ignored_toolkit_control() {
    let root = Path::new("/repo");
    let expected = sanitized_cargo(root, "test").args([
        "--no-fail-fast",
        "--package",
        "martensite-tkv3d",
        "--test",
        "toolkit",
        "--",
        CERTIFIED_CONE_ATTACHMENT_TEST,
        "--exact",
        "--ignored",
        "--test-threads=1",
        "--show-output",
    ]);
    assert_eq!(
        control_command(root, Control::CertifiedConeAttachment),
        expected
    );
}

#[test]
fn certified_cone_control_names_an_existing_ignored_threaded_test() {
    let test_name = "certified_attachment_replays_exactly_on_independent_threads";
    let modules = ATTACHMENT_MODULES.map(|(_, name)| name).join("::");
    assert_eq!(
        CERTIFIED_CONE_ATTACHMENT_TEST,
        format!("{modules}::{test_name}")
    );
    for (source, name) in ATTACHMENT_MODULES {
        assert!(source.contains(&format!("mod {name};")), "{name}");
    }
    let signature = format!("fn {test_name}()");
    let (before_test, _) = ATTACHMENT_SOURCE.split_once(&signature).unwrap();
    let attribute = before_test.trim_end().lines().last().unwrap();
    assert!(attribute.starts_with("#[ignore"), "{attribute}");
}

#[test]
fn certified_cone_failure_cannot_pass_after_detached_cone_succeeds() {
    let root = Path::new("/repo");
    let command = certified_cone_attachment_controls(root).display();
    let runner = runner().with_exit_code(&command, 66);
    let selected = [Control::WrappedCone, Control::CertifiedConeAttachment];
    let error = run_selected(&context_at(&runner, root), &selected).unwrap_err();
    let commands = runner.commands();
    assert!(error.to_string().contains("66"));
    assert_eq!(commands[3], wrapped_cone_controls(root).display());
    assert_eq!(commands[4], command);
}
