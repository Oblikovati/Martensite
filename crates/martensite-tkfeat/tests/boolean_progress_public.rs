//! External feature receivers retain their own Boolean program and caller range.
use martensite_tkbo::bop_algo::{self, BOPAlgoPaveFiller, BuilderHooks, ProgressCompletion};
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkfeat::brep_feat::BRepFeatBuilder;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;
use std::sync::atomic::Ordering;
#[allow(dead_code)] // These tests read only part of the shared recording helpers.
#[path = "support/progress_recording.rs"]
mod recording;
fn source_vertex(x: f64) -> Shape {
    BRepBuilderAPIMakeVertex::new(Point3::new(x, 0., 0.))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape()
}
fn feature_receiver(sources: &[Shape; 2], fuse: i32, second_part: bool) -> BRepFeatBuilder {
    let mut feature = BRepFeatBuilder::new();
    feature.init_with_tool(&sources[0], &sources[1]);
    feature.set_operation_with_flag(fuse, second_part);
    feature
}
fn require_selected_sources(feature: &BRepFeatBuilder, sources: &[Shape; 2], expected: &[usize]) {
    let parts: Vec<_> = ShapeIterator::new(feature.shape(), true, true).collect();
    assert_eq!(parts.len(), expected.len());
    for index in expected {
        assert!(parts.iter().any(|part| part.is_same(&sources[*index])));
    }
    assert!(!feature.has_errors());
}
#[test]
fn whole_feature_entry_obeys_all_four_independent_disjoint_set_laws() {
    let sources = [source_vertex(0.), source_vertex(10.)];
    for (fuse, second, expected) in [
        (0, false, vec![0]),
        (1, false, vec![0, 1]),
        (0, true, vec![]),
        (1, true, vec![1]),
    ] {
        let mut feature = feature_receiver(&sources, fuse, second);
        assert_eq!(
            BuilderHooks::perform_with_progress(&mut feature, &ProgressRange::default()).unwrap(),
            ProgressCompletion::Completed
        );
        require_selected_sources(&feature, &sources, &expected);
    }
}
#[test]
fn prepared_feature_entry_uses_the_same_actual_receiver() {
    let sources = [source_vertex(0.), source_vertex(10.)];
    let mut feature = feature_receiver(&sources, 0, false);
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(&sources.iter().cloned().collect());
    filler.perform();
    require_prepared_completion(&mut feature, &mut filler);
    require_selected_sources(&feature, &sources, &[0]);
}
#[test]
fn whole_feature_entry_retains_actual_one_pulse_cancellation() {
    let sources = [source_vertex(0.), source_vertex(0.)];
    let mut feature = feature_receiver(&sources, 1, false);
    let (indicator, trace) = recording::pulse_indicator(1);
    assert!(matches!(
        BuilderHooks::perform_with_progress(&mut feature, &indicator.start()).unwrap(),
        ProgressCompletion::Cancelled { .. }
    ));
    assert!(trace.break_observed.load(Ordering::SeqCst));
    assert!(feature.boolean_operation().pave_filler().is_some());
}

fn require_prepared_completion(feature: &mut BRepFeatBuilder, filler: &mut BOPAlgoPaveFiller) {
    let completion =
        bop_algo::perform_with_filler_progress(feature, filler, &ProgressRange::default()).unwrap();
    assert_eq!(completion, ProgressCompletion::Completed);
}
fn closed_box(x: f64) -> Shape {
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
    BRepPrimAPIMakeBox::from_points(Point3::new(x, 0., 0.), Point3::new(x + 2., 3., 4.))
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}
#[test]
fn whole_feature_entry_retains_two_disjoint_closed_solid_sources() {
    let sources = [closed_box(0.), closed_box(10.)];
    let mut feature = feature_receiver(&sources, 1, false);
    assert_eq!(
        BuilderHooks::perform_with_progress(&mut feature, &ProgressRange::default()).unwrap(),
        ProgressCompletion::Completed
    );
    require_selected_sources(&feature, &sources, &[0, 1]);
    let faces = martensite_tkbrep::topexp::Explorer::new(
        feature.shape(),
        martensite_tkg3d::top_abs::ShapeEnum::Face,
        martensite_tkg3d::top_abs::ShapeEnum::Shape,
    )
    .unwrap();
    assert_eq!(faces.count(), 12);
}
#[test]
fn whole_feature_entry_keeps_the_actual_external_parent() {
    let sources = [source_vertex(0.), source_vertex(10.)];
    let mut feature = feature_receiver(&sources, 1, false);
    let (indicator, trace) = recording::indicator(0);
    let root = indicator.start();
    let parent = ProgressScope::with_literal(&root, "External feature caller", 10., false);
    assert_eq!(
        BuilderHooks::perform_with_progress(&mut feature, &parent.next(3.)).unwrap(),
        ProgressCompletion::Completed
    );
    assert!(trace.events.lock().unwrap().iter().any(|(_, names)| {
        names.windows(2).any(|pair| {
            pair[0] == "Performing Boolean operation" && pair[1] == "External feature caller"
        })
    }));
}
#[test]
fn empty_feature_input_is_a_failure_not_a_completed_empty_operation() {
    let mut feature = BRepFeatBuilder::new();
    assert!(matches!(
        BuilderHooks::perform_with_progress(&mut feature, &ProgressRange::default()).unwrap(),
        ProgressCompletion::Failed { .. }
    ));
    assert!(feature.has_errors());
}
