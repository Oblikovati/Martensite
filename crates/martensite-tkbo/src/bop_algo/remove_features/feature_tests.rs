//! Pin sequential parent history and real independent boss-removal expectations.
use super::super::helpers::explore;
use super::fixture::parent_fixture;
use super::*;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use martensite_tktopalgo::brep_gprop::BRepGProp;

#[test]
fn one_and_two_bosses_return_the_known_base_box_without_premature_history_finalization() {
    for profile in [6, 7] {
        let mut remover = parent_fixture(profile, false);
        let removed: Vec<_> = remover
            .features
            .iter()
            .flat_map(|feature| explore(feature, ShapeEnum::Face).unwrap())
            .collect();
        remover.remove_features().unwrap();
        assert_base_box(&remover);
        let history = remover.base.history.get().unwrap();
        for face in removed {
            assert!(!history.is_removed(&face).unwrap());
            let kept = explore(&remover.base.shape, ShapeEnum::Face).unwrap();
            assert!(!kept.iter().any(|image| image.is_same(&face)));
        }
    }
}
fn assert_base_box(remover: &BOPAlgoRemoveFeatures) {
    assert!(!remover.base.options.has_errors());
    assert!(!remover.base.options.has_warnings());
    assert_box_geometry(remover);
}
fn assert_box_geometry(remover: &BOPAlgoRemoveFeatures) {
    let result = &remover.base.shape;
    assert_eq!(explore(result, ShapeEnum::Solid).unwrap().len(), 1);
    assert!(
        BRepCheckAnalyzer::new(result, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    let mut volume = GProps::default();
    let error = BRepGProp::volume_properties_eps(result, &mut volume, 1e-9, true, false).unwrap();
    assert!(error <= 1e-9, "volume error={error}; expected <=1e-9");
    assert!(
        (volume.mass() - 500.).abs() <= 1e-9,
        "volume={}; expected 500",
        volume.mass()
    );
}

#[test]
fn prepared_parent_defaults_keep_completed_empty_and_warning_cases_distinct() {
    for profile in 0..5 {
        let mut remover = parent_fixture(profile, false);
        let original = remover.base.shape.clone();
        remover.remove_features().unwrap();
        assert!(!remover.base.options.has_errors());
        assert_eq!(remover.base.options.has_warnings(), profile != 0);
        assert!(remover.base.shape.is_same(&original));
        assert!(remover.base.history.get().is_some());
    }
}

#[test]
fn empty_parent_cancellation_is_a_reported_stop_before_history_creation() {
    use crate::bop_algo::{
        ProgressCompletion, progress_test_support::parent_census::parent_census_indicator,
    };
    let mut remover = parent_fixture(0, false);
    let (indicator, _) = parent_census_indicator(1);
    let completion = remover
        .remove_features_on_progress(&indicator.start())
        .unwrap();
    assert!(matches!(completion, ProgressCompletion::Cancelled { .. }));
    assert!(remover.base.options.has_errors());
    assert!(remover.base.history.is_null());
}

#[test]
fn a_caught_empty_rebuild_keeps_its_cause_and_still_removes_later_real_features() {
    use crate::bop_algo::ProgressCompletion;
    use martensite_tkernel::message::ProgressRange;
    let mut remover = parent_fixture(7, false);
    remover
        .features
        .insert(0, super::super::helpers::compound_of(&[]).unwrap());
    let completion = remover
        .remove_features_on_progress(&ProgressRange::default())
        .unwrap();
    let (cause, context) = match completion {
        ProgressCompletion::Failed {
            cause: Some(cause),
            context,
            ..
        } => (cause, context),
        other => panic!("status={other:?}; expected retained typed empty-rebuild failure"),
    };
    assert_eq!(cause.exception_type(), "Standard_ConstructionError");
    assert_eq!(context.unwrap().input_ordinal, Some(0));
    assert!(remover.base.options.has_warnings());
    assert_box_geometry(&remover);
}

#[test]
fn repeated_parallel_parent_runs_match_serial_geometry_and_preserve_source_vertices() {
    for _ in 0..8 {
        for parallel in [false, true] {
            let mut remover = parent_fixture(7, parallel);
            let before = parent_vertex_positions(&remover.input_shape);
            remover.remove_features().unwrap();
            assert_base_box(&remover);
            assert_eq!(parent_vertex_positions(&remover.input_shape), before);
            assert_box_boundary_faces(&remover.base.shape);
        }
    }
}
fn parent_vertex_positions(shape: &martensite_tkbrep::topods::Shape) -> Vec<[u64; 3]> {
    use martensite_tkbrep::{brep::BRepTool, topexp, topods, toptools::IndexedMapOfShape};
    let mut vertices = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Vertex, &mut vertices).unwrap();
    let mut points: Vec<_> = vertices
        .iter()
        .map(|vertex| {
            let point = BRepTool::pnt(&topods::vertex(vertex).unwrap()).unwrap();
            [
                point.x().to_bits(),
                point.y().to_bits(),
                point.z().to_bits(),
            ]
        })
        .collect();
    points.sort_unstable();
    points
}
fn assert_box_boundary_faces(shape: &martensite_tkbrep::topods::Shape) {
    for face in explore(shape, ShapeEnum::Face).unwrap() {
        let points = parent_vertex_positions(&face);
        assert!(points.len() >= 4);
        let on_boundary = (0..3).any(|axis| {
            [0_f64, [10., 10., 5.][axis]].into_iter().any(|coordinate| {
                points
                    .iter()
                    .all(|point| point[axis] == coordinate.to_bits())
            })
        });
        assert!(
            on_boundary,
            "face vertices={points:?}; expected one base-box boundary plane"
        );
    }
}
