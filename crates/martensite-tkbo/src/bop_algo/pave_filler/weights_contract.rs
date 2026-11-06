//! Independently enumerated stage proportions distinguish counts from equal bars.
use super::{PISteps, SLOT_COUNT, populate_intersections, reserve_preparation, variable_weights};
use crate::bop_algo::{Glue, progress::analyze_progress_with};

#[test]
fn all_variable_slots_use_the_original_integer_count_expressions() {
    let actual = variable_weights([2, 3, 5, 7, 11, 13], false, false);
    assert_eq!(actual, expected_variable_weights());
}

#[test]
fn glue_full_and_avoided_pcurves_preserve_the_native_zero_slots() {
    let actual = variable_weights([2, 3, 5, 7, 0, 13], true, true);
    assert_eq!(actual[4], 0.0, "GlueFull supplies no EF candidates");
    assert_eq!(
        actual[7], 0.0,
        "forced EF weight follows actual EF candidate count"
    );
    assert_eq!(
        actual[8], 13.0,
        "GlueFull keeps one weight per FF candidate"
    );
    assert_eq!(actual[10], 0.0, "GlueFull suppresses block weight");
    assert_eq!(
        actual[11], 0.0,
        "AvoidBuildPCurve suppresses projection weight"
    );
}

#[test]
fn preparation_reserves_point_ninety_five_and_remaining_slots_sum_to_ninety_five() {
    let steps = normalized_counts([2, 3, 5, 7, 11, 13], false);
    assert_eq!(steps.get_step(0).to_bits(), 0.95_f64.to_bits());
    let total: f64 = steps.steps().as_slice().iter().sum();
    assert!(
        (total - 95.0).abs() < 1e-12,
        "normalized total {total:?}, expected 95"
    );
    assert!(
        steps.get_step(9) > steps.get_step(1),
        "FF weight must exceed VV weight"
    );
}

#[test]
fn empty_non_destructive_inputs_do_not_manufacture_positive_stage_weights() {
    let steps = normalized_counts([0; 6], true);
    assert_eq!(steps.steps().as_slice(), &[0.0; 14]);
}

fn expected_variable_weights() -> [f64; 13] {
    let repeat = 2.4000000000000004;
    [
        2.0, 6.0, 25.0, 35.0, 110.0, repeat, 10.0, 22.0, 390.0, 5.0, 65.0, 3.2, 0.5,
    ]
}

fn normalized_counts(counts: [i32; 6], non_destructive: bool) -> PISteps {
    let mut steps = PISteps::new(SLOT_COUNT).unwrap();
    analyze_progress_with(
        95.0,
        &mut steps,
        |whole, steps| reserve_preparation(non_destructive, whole, steps),
        |steps| populate_intersections(counts, Glue::Off, false, steps),
    );
    steps
}

#[test]
fn initialized_candidate_weights_leave_iterator_selection_unchanged() {
    for non_destructive in [false, true] {
        let filler = initialized_filler(non_destructive);
        let iterator = filler.iterator().unwrap();
        let selection = (iterator.expected_length(), iterator.more());
        let weights = super::intersection_weights(&filler).unwrap();
        let expected = if non_destructive { 0.0_f64 } else { 0.95_f64 };
        assert_eq!(weights.get_step(0).to_bits(), expected.to_bits());
        assert!(
            weights.get_step(9) > 0.0,
            "overlapping boxes must have FF candidates"
        );
        assert_eq!((iterator.expected_length(), iterator.more()), selection);
    }
}

#[test]
fn uninitialized_weights_report_missing_structure_without_fake_counts() {
    let filler = super::BOPAlgoPaveFiller::new();
    let result = super::intersection_weights(&filler);
    assert!(
        result.is_err(),
        "missing actual iterator must not manufacture weights"
    );
}

fn initialized_filler(non_destructive: bool) -> super::BOPAlgoPaveFiller {
    use martensite_tkernel::ncollection::List;
    use martensite_tkmath::gp::Point3;
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
    let mut first = BRepPrimAPIMakeBox::at_point(Point3::default(), 10.0, 10.0, 10.0).unwrap();
    let mut second =
        BRepPrimAPIMakeBox::at_point(Point3::new(5.0, 5.0, 5.0), 10.0, 10.0, 10.0).unwrap();
    let arguments = List::from_iter([
        first.shape().unwrap().clone(),
        second.shape().unwrap().clone(),
    ]);
    let mut filler = super::BOPAlgoPaveFiller::new();
    filler.set_non_destructive(non_destructive);
    filler.set_arguments(&arguments);
    filler.initialize().unwrap();
    filler
}
