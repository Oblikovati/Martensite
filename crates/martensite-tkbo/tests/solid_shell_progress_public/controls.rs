//! Raw receiver equivalence is distinct from isolation of a published operation result.
use super::{ProgressCompletion, recording, shell_fixtures, snapshot, solid_fixtures};
use martensite_tkernel::message::ProgressRange;

#[test]
fn raw_internal_free_flag_matches_native_default_and_ranged_mutation() {
    for ranged in [false, true] {
        assert_raw_internal_flag(ranged);
    }
}

fn assert_raw_internal_flag(ranged: bool) {
    let mut builder = solid_fixtures::builder("internal", false);
    let face = builder.shapes().iter().nth(6).unwrap().clone();
    assert!(face.free().unwrap());
    if ranged {
        let result = builder.perform_with_progress(&ProgressRange::default());
        assert!(matches!(result, ProgressCompletion::Completed));
    } else {
        builder.perform().unwrap();
    }
    assert!(
        !face.free().unwrap(),
        "ranged={ranged}; native raw Free flag"
    );
}

#[test]
fn unaffected_two_and_cavity_inputs_retain_reachable_bytes() {
    for recipe in ["two", "cavity"] {
        let mut builder = solid_fixtures::builder(recipe, false);
        let before = snapshot::shapes(builder.shapes().iter());
        builder.perform_with_progress(&ProgressRange::default());
        snapshot::assert_unchanged(recipe, &before, &snapshot::shapes(builder.shapes().iter()));
    }
}

#[test]
fn shell_real_nested_ranges_and_nonsticky_cancellation_are_external() {
    let mut splitter = shell_fixtures::irregular_splitter(false);
    let (indicator, trace) = recording::indicator(0);
    splitter.perform_with_progress(&indicator.start());
    assert!(recording::has_nested_shell_progress(&trace));
    assert_recording_formats(&trace);
    let (pulse, _) = recording::pulse_indicator(3);
    assert!(matches!(
        splitter.perform_with_progress(&pulse.start()),
        ProgressCompletion::Cancelled { .. }
    ));
    assert!(!pulse.root_scope().user_break());
}

#[test]
fn default_shell_and_explicit_default_range_match_serialized_results() {
    let mut legacy = shell_fixtures::two_irregular_blocks(false);
    let mut ranged = shell_fixtures::two_irregular_blocks(false);
    legacy.perform().unwrap();
    assert!(matches!(
        ranged.perform_with_progress(&ProgressRange::default()),
        ProgressCompletion::Completed
    ));
    assert_eq!(
        snapshot::shapes(legacy.shells().iter()),
        snapshot::shapes(ranged.shells().iter())
    );
}

fn assert_recording_formats(trace: &recording::Trace) {
    let quoted: Vec<String> = recording::native_events(trace)
        .iter()
        .map(|event| format!("{event:?}"))
        .collect();
    assert_eq!(recording::rendered_events(trace), quoted.join(","));
}

#[test]
fn default_solid_and_explicit_default_range_match_all_raw_recipes() {
    for recipe in ["two", "cavity", "internal"] {
        let mut legacy = solid_fixtures::builder(recipe, false);
        let mut ranged = solid_fixtures::builder(recipe, false);
        legacy.perform().unwrap();
        assert!(matches!(
            ranged.perform_with_progress(&ProgressRange::default()),
            ProgressCompletion::Completed
        ));
        assert_eq!(
            snapshot::shapes(legacy.areas().iter()),
            snapshot::shapes(ranged.areas().iter())
        );
        assert_eq!(
            snapshot::shapes(legacy.shapes().iter()),
            snapshot::shapes(ranged.shapes().iter())
        );
        assert_eq!(legacy.options().has_errors(), ranged.options().has_errors());
    }
}
