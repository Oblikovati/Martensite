//! Closed-box invariants pin the prepared rebuilding stage before progress changes.
use super::*;
use martensite_tkernel::standard::Handle;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

struct PreparedRebuild {
    remover: BOPAlgoRemoveFeatures,
    gap: FillGap,
    source: Shape,
}
fn prepared_rebuild(intersect: bool, fill_history: bool) -> PreparedRebuild {
    let source = BRepPrimAPIMakeBox::from_sizes(2., 3., 4.)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let mut remover = BOPAlgoRemoveFeatures::new();
    remover.base.shape = source.clone();
    remover.base.fill_history = fill_history;
    remover.base.history = Handle::new(History::new());
    let mut gap = FillGap::new(&compound_of(&[]).unwrap(), false);
    gap.history = Handle::new(History::new());
    gap.solids.add(source.clone());
    if intersect {
        let face = explore(&source, ShapeEnum::Face).unwrap().remove(0);
        gap.faces.add(face.clone(), [face].into_iter().collect());
        gap.has_adjacent_faces = true;
    }
    PreparedRebuild {
        remover,
        gap,
        source,
    }
}
fn assert_closed_box_rebuild(case: &PreparedRebuild) {
    let result = &case.remover.base.shape;
    assert!(!case.remover.base.options.has_errors());
    assert!(!case.remover.base.options.has_warnings());
    assert_eq!(explore(result, ShapeEnum::Solid).unwrap().len(), 1);
    assert_eq!(explore(result, ShapeEnum::Face).unwrap().len(), 6);
    assert!(
        BRepCheckAnalyzer::new(result, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    assert_eq!(explore(&case.source, ShapeEnum::Face).unwrap().len(), 6);
    assert_eq!(box_vertex_positions(result), expected_box_corners());
    assert_eq!(box_vertex_positions(&case.source), expected_box_corners());
    for face in explore(&case.source, ShapeEnum::Face).unwrap() {
        assert!(
            !case
                .remover
                .base
                .history
                .get()
                .unwrap()
                .is_removed(&face)
                .unwrap()
        );
    }
}
#[test]
fn prepared_rebuild_preserves_closed_box_and_source_history_for_both_volume_policies() {
    for intersect in [false, true] {
        for history in [false, true] {
            let mut case = prepared_rebuild(intersect, history);
            case.remover.remove_feature(&case.gap, true).unwrap();
            assert_closed_box_rebuild(&case);
        }
    }
}
#[test]
fn missing_reconstructed_neighbors_warn_without_replacing_the_current_shape() {
    let mut case = prepared_rebuild(false, true);
    case.gap.has_adjacent_faces = true;
    case.remover.remove_feature(&case.gap, true).unwrap();
    assert!(case.remover.base.options.has_warnings());
    assert!(case.remover.base.shape.is_same(&case.source));
    assert!(!case.remover.base.history.get().unwrap().has_modified());
}
#[test]
fn missing_parent_history_retains_the_typed_precondition_failure() {
    let mut case = prepared_rebuild(false, true);
    case.remover.base.history = Handle::default();
    let cause = case.remover.remove_feature(&case.gap, true).unwrap_err();
    assert_eq!(cause.exception_type(), "Standard_NullObject");
    assert!(case.remover.base.shape.is_same(&case.source));
}

fn box_vertex_positions(shape: &Shape) -> Vec<[u64; 3]> {
    use martensite_tkbrep::{brep::BRepTool, topexp, topods};
    let mut vertices = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Vertex, &mut vertices).unwrap();
    let mut positions: Vec<_> = vertices
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
    positions.sort_unstable();
    positions
}
fn expected_box_corners() -> Vec<[u64; 3]> {
    let mut positions = Vec::new();
    for x in [0_f64, 2.] {
        for y in [0_f64, 3.] {
            for z in [0_f64, 4.] {
                positions.push([x.to_bits(), y.to_bits(), z.to_bits()]);
            }
        }
    }
    positions.sort_unstable();
    positions
}

#[test]
fn a_rebuild_child_pulse_never_becomes_completed_after_raw_reconstruction() {
    use crate::bop_algo::{
        ProgressCompletion, progress_test_support::parent_census::parent_census_indicator,
    };
    let mut case = prepared_rebuild(false, true);
    let (indicator, journal) = parent_census_indicator(1);
    let completion = case
        .remover
        .remove_feature_on_progress(&case.gap, true, &indicator.start())
        .unwrap();
    assert!(journal.observed.load(std::sync::atomic::Ordering::SeqCst));
    assert!(!matches!(completion, ProgressCompletion::Completed));
    assert_eq!(box_vertex_positions(&case.source), expected_box_corners());
}

#[path = "feature_rebuild_census.rs"]
mod census;

#[test]
fn a_silent_face_validation_stop_keeps_shape_and_history_unpublished() {
    use crate::bop_algo::{
        ProgressCompletion, ProgressOperation,
        progress_test_support::parent_census::parent_census_indicator,
    };
    let mut case = prepared_rebuild(false, true);
    let (indicator, _) = parent_census_indicator(32);
    let completion = case
        .remover
        .remove_feature_on_progress(&case.gap, true, &indicator.start())
        .unwrap();
    assert!(matches!(
        completion,
        ProgressCompletion::Cancelled {
            operation: ProgressOperation::RemoveFeatures,
            ..
        }
    ));
    assert!(case.remover.base.shape.is_same(&case.source));
    assert!(!case.remover.base.options.has_warnings());
    assert!(!case.remover.base.history.get().unwrap().has_modified());
    assert_eq!(box_vertex_positions(&case.source), expected_box_corners());
}
