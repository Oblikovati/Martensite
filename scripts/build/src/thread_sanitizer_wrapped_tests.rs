//! An affected sketch must run its ignored family campaign alongside the existing replay control.
use super::*;
use crate::thread_sanitizer_selection::select;
use std::collections::BTreeSet;

const WRAPPED_STRESS_TEST: &str = "surface_sketch::wrapped_profile::family::replay_tests::stress::different_geometry_with_shared_authored_ids_replays_under_contention";
const WRAPPED_STRESS_SOURCE: &str =
    include_str!("../../../crates/martensite-tksketch/src/wrapped_profile/family/replay_stress.rs");

fn sketch_selection() -> Vec<Control> {
    select(&BTreeSet::from(["martensite-tksketch".into()]), &[])
}

#[test]
fn affected_sketch_selection_runs_all_replay_controls_after_startup() {
    let runner = runner();
    let context = context_at(&runner, Path::new("/repo"));
    run_selected(&context, &sketch_selection()).unwrap();
    let commands = runner.commands();
    assert!(commands[2].contains("thread_sanitizer_startup/Cargo.toml"));
    assert!(commands[3].contains("--test surface_sketch -- replay::parallel_operations"));
    assert!(
        commands
            .iter()
            .any(|command| command.contains(WRAPPED_STRESS_TEST)),
        "affected sketch commands omit ignored wrapped-family stress: {commands:?}"
    );
    assert_eq!(
        commands[6],
        certified_cylinder_attachment_controls(Path::new("/repo")).display()
    );
    assert_eq!(commands.len(), 7);
}

#[test]
fn wrapped_family_command_selects_the_actual_ignored_campaign_with_pinned_instrumentation() {
    let root = Path::new("/repo");
    let expected = sanitized_cargo(root, "test").args([
        "--no-fail-fast",
        "--package",
        "martensite-tksketch",
        "--lib",
        WRAPPED_STRESS_TEST,
        "--",
        "--exact",
        "--ignored",
        "--test-threads=1",
        "--show-output",
    ]);
    assert_eq!(control_command(root, Control::WrappedFamily), expected);
}

#[test]
fn wrapped_family_selector_names_an_existing_ignored_campaign() {
    assert!(
        WRAPPED_STRESS_SOURCE
            .contains("fn different_geometry_with_shared_authored_ids_replays_under_contention()")
    );
    assert!(WRAPPED_STRESS_SOURCE.contains("#[ignore = "));
}

#[test]
fn selected_wrapped_family_failure_cannot_pass_the_sanitizer_gate() {
    let command = wrapped_family_controls(Path::new("/repo")).display();
    let runner = runner().with_exit_code(&command, 66);
    let error = run_selected(
        &context_at(&runner, Path::new("/repo")),
        &sketch_selection(),
    )
    .unwrap_err();
    assert!(error.to_string().contains("66"));
    assert_eq!(runner.commands().len(), 5);
    assert!(runner.commands()[3].contains("--test surface_sketch"));
    assert!(runner.commands()[4].contains(WRAPPED_STRESS_TEST));
}

#[test]
fn startup_failure_prevents_selected_sketch_controls() {
    let command = startup(Path::new("/repo")).display();
    let runner = runner().with_exit_code(&command, 101);
    let context = context_at(&runner, Path::new("/repo"));
    assert!(run_selected(&context, &sketch_selection()).is_err());
    let commands = runner.commands();
    assert_eq!(commands.len(), 3);
    assert_eq!(commands[2], command);
}

#[test]
fn unrelated_affected_leaf_does_not_schedule_sketch_stress() {
    let selected = select(&BTreeSet::from(["martensite-tkdeobj".into()]), &[]);
    let runner = runner();
    let context = context_at(&runner, Path::new("/repo"));
    let before = runner.commands();
    run_selected(&context, &selected).unwrap();
    assert_eq!(runner.commands(), before);
    assert!(!selected.contains(&Control::WrappedFamily));
    assert!(!selected.contains(&Control::WrappedCone));
    assert!(!selected.contains(&Control::CertifiedCylinderAttachment));
}

#[test]
fn certified_cylinder_selector_names_an_existing_nonignored_control() {
    let feature = include_str!(
        "../../../crates/martensite-tksketch/src/wrapped_profile/api/footprint/feature/mod.rs"
    );
    let module = include_str!(
        "../../../crates/martensite-tksketch/src/wrapped_profile/api/footprint/feature/cylinder_certified.rs"
    );
    let tests = include_str!(
        "../../../crates/martensite-tksketch/src/wrapped_profile/api/footprint/feature/cylinder_certified/tests.rs"
    );
    assert!(feature.contains("mod cylinder_certified;"));
    assert!(module.contains("mod tests;"));
    let (_, name) = CERTIFIED_CYLINDER_ATTACHMENT_TEST
        .rsplit_once("::")
        .unwrap();
    assert!(tests.contains(&format!("#[test]\nfn {name}()")));
}
