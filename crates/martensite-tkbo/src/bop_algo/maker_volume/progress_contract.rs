//! Owned closed-box controls pin both volume policies and the actual receiver's cancellation.
use super::*;
use crate::bop_algo::{
    perform_with_filler_progress,
    progress_test_support::{indicator, pulse_indicator},
};
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::sync::atomic::Ordering;
fn box_shape() -> Shape {
    BRepPrimAPIMakeBox::from_points(Point3::new(0., 0., 0.), Point3::new(2., 3., 4.))
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}
fn volume_case(intersect: bool) -> BOPAlgoMakerVolume {
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_intersect(intersect);
    maker.add_argument(&box_shape());
    maker
}
#[test]
fn both_volume_policies_build_one_closed_solid_with_their_actual_parent_scopes() {
    for intersect in [false, true] {
        let mut maker = volume_case(intersect);
        let (progress, trace) = indicator(0);
        assert_eq!(
            maker.perform_with_progress(&progress.start()).unwrap(),
            ProgressCompletion::Completed
        );
        assert_eq!(maker.shape().shape_type().unwrap(), ShapeEnum::Solid);
        assert!(!maker.bounding_box().is_null());
        assert_eq!(maker.faces().len(), 18);
        assert_eq!(
            Explorer::new(maker.shape(), ShapeEnum::Face, ShapeEnum::Shape)
                .unwrap()
                .count(),
            6
        );
        assert!(maker.has_filled_history());
        let events = trace.events.lock().unwrap();
        assert!(
            events
                .iter()
                .any(|(_, names)| names.iter().any(|name| name == "Building volumes"))
        );
        assert!(events.iter().any(|(_, names)| {
            names
                .iter()
                .any(|name| name == "Performing MakeVolume operation")
        }));
    }
}
#[test]
fn prepared_volume_runs_its_own_solid_builder_without_general_fuse_result_stages() {
    let mut maker = volume_case(true);
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(maker.arguments());
    filler.perform();
    assert!(!filler.options().has_errors());
    assert_eq!(
        perform_with_filler_progress(&mut maker, &mut filler, &ProgressRange::default()).unwrap(),
        ProgressCompletion::Completed
    );
    assert_eq!(maker.shape().shape_type().unwrap(), ShapeEnum::Solid);
    assert_eq!(maker.faces().len(), 18);
}
#[test]
fn whole_volume_retains_actual_intersection_cancellation_and_filler_diagnostics() {
    for intersect in [false, true] {
        let mut maker = volume_case(intersect);
        let (progress, trace) = pulse_indicator(1);
        let completion = maker.perform_with_progress(&progress.start()).unwrap();
        assert!(!matches!(completion, ProgressCompletion::Completed));
        assert_eq!(trace.queries.load(Ordering::SeqCst), 1);
        assert!(maker.pave_filler().is_some());
        assert!(maker.options().has_errors());
    }
}
#[test]
fn missing_volume_arguments_are_a_failed_operation_with_no_result() {
    for intersect in [false, true] {
        let mut maker = BOPAlgoMakerVolume::new();
        maker.set_intersect(intersect);
        assert!(matches!(
            maker
                .perform_with_progress(&ProgressRange::default())
                .unwrap(),
            ProgressCompletion::Failed { .. }
        ));
        assert!(maker.shape().is_null());
        assert!(maker.options().has_errors());
    }
}
