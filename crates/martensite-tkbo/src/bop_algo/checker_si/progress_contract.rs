//! Closed-form membership, stored-graph laws and actual pulses qualify the concrete receiver.
use super::fixture::checker_case;
use super::*;
use crate::bop_algo::{
    AlertUserBreak, ProgressFailureClass,
    progress_test_support::{indicator, pulse_indicator},
};
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topods::{self, Compound, Shape},
};
use martensite_tkernel::standard::RuntimeTyped;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPIMakeVertex};
use std::sync::atomic::Ordering;
#[test]
fn checker_operation_and_shared_pair_records_have_scoped_worker_traits() {
    fn send<T: Send>() {}
    fn sync<T: Sync>() {}
    send::<BOPAlgoCheckerSI>();
    sync::<DataStructure>();
}
#[test]
fn an_interior_vertex_is_recorded_against_the_owning_box() {
    let mut case = checker_case(43, false);
    assert_eq!(
        case.perform(&ProgressRange::default()).unwrap(),
        ProgressCompletion::Completed
    );
    let structure = case.checker.data_structure_mut().unwrap();
    assert_eq!(structure.interf_vz().len(), 1);
    let (vertex, solid) = structure.interf_vz().value(0).unwrap().indices();
    assert!(structure.has_interference_between(vertex, solid));
    assert_eq!(
        structure.shape_info(solid).unwrap().shape_type(),
        ShapeEnum::Solid
    );
    let point = BRepTool::pnt(&topods::vertex(structure.shape(vertex).unwrap()).unwrap()).unwrap();
    assert_eq!([point.x(), point.y(), point.z()], [3., 3., 3.]);
}
#[test]
fn vertex_collection_worker_and_storage_pulses_keep_their_distinct_reports() {
    for (target, queries, reported) in [(1, 1, true), (2, 3, false), (3, 3, true)] {
        let mut case = checker_case(43, false);
        let (indicator, trace) = pulse_indicator(target);
        assert!(matches!(
            case.perform(&indicator.start()).unwrap(),
            ProgressCompletion::Cancelled { .. }
        ));
        assert_eq!(trace.queries.load(Ordering::SeqCst), queries);
        assert!(trace.break_observed.load(Ordering::SeqCst));
        assert_eq!(
            case.checker
                .options()
                .has_error(AlertUserBreak::static_type()),
            reported
        );
        assert_eq!(
            case.checker.data_structure_mut().unwrap().interf_vz().len(),
            0
        );
    }
}
#[test]
fn solid_workers_follow_any_and_every_subshape_record_predicates() {
    for (positive, negative, kind) in [
        (44, 47, ShapeEnum::Edge),
        (45, 48, ShapeEnum::Face),
        (46, 49, ShapeEnum::Solid),
    ] {
        let mut found = checker_case(positive, false);
        let mut empty = checker_case(negative, false);
        assert_eq!(
            found.perform(&ProgressRange::default()).unwrap(),
            ProgressCompletion::Completed
        );
        assert_eq!(
            empty.perform(&ProgressRange::default()).unwrap(),
            ProgressCompletion::Completed
        );
        assert!(solid_record_count(&mut found.checker, kind) > 0);
        assert_eq!(solid_record_count(&mut empty.checker, kind), 0);
    }
}
#[test]
fn ring_torus_is_excluded_and_spindle_search_remains_explicitly_incomplete() {
    let mut ring = checker_case(41, false);
    let mut spindle = checker_case(42, false);
    assert_eq!(
        ring.perform(&ProgressRange::default()).unwrap(),
        ProgressCompletion::Completed
    );
    assert!(matches!(
        spindle.perform(&ProgressRange::default()).unwrap(),
        ProgressCompletion::Incomplete {
            class: ProgressFailureClass::UnqualifiedStage,
            ..
        }
    ));
    assert_eq!(
        ring.checker
            .data_structure()
            .unwrap()
            .interferences()
            .extent(),
        0
    );
    let receipt = spindle.checker.perform_observed();
    assert_eq!(receipt.faces.len(), 1);
    assert!(!matches!(
        receipt.faces[0].termination,
        FaceTermination::PolicyExcluded
    ));
}
#[test]
fn whole_receiver_retains_self_check_intersection_and_solid_parent_names() {
    let mut case = checker_case(9, false);
    let (indicator, trace) = indicator(0);
    assert_eq!(
        case.perform(&indicator.start()).unwrap(),
        ProgressCompletion::Completed
    );
    let events = trace.events.lock().unwrap();
    for expected in [
        "Checking shape on self-intersection",
        "Performing intersection of shapes",
        "Performing Vertex-Solid intersection",
    ] {
        assert!(
            events
                .iter()
                .any(|(_, names)| names.iter().any(|name| name == expected)),
            "missing {expected}"
        );
    }
}
#[test]
fn a_null_shape_returns_an_owned_typed_failure_instead_of_completing() {
    let mut case = checker_case(14, false);
    let completion = case.perform(&ProgressRange::default()).unwrap();
    let ProgressCompletion::Failed {
        class: ProgressFailureClass::KernelFailure,
        cause: Some(cause),
        ..
    } = completion
    else {
        panic!("null shape completion={completion:?}; expected a typed kernel failure");
    };
    assert_eq!(cause.exception_type(), "Standard_NullObject");
    assert!(!cause.what().is_empty());
    assert!(
        case.checker
            .options()
            .has_error(AlertIntersectionFailed::static_type())
    );
}
#[test]
fn coincident_interior_vertex_groups_classify_once_and_retain_a_source_handle() {
    for count in 2..=4 {
        for coordinate in [2., 4.] {
            check_coincident_interior_group(count, coordinate);
        }
    }
}
fn solid_record_count(checker: &mut BOPAlgoCheckerSI, kind: ShapeEnum) -> usize {
    let structure = checker.data_structure_mut().unwrap();
    match kind {
        ShapeEnum::Edge => structure.interf_ez().len(),
        ShapeEnum::Face => structure.interf_fz().len(),
        _ => structure.interf_zz().len(),
    }
}
fn check_coincident_interior_group(count: usize, coordinate: f64) {
    let (mut checker, sources) = coincident_interior_checker(count, coordinate);
    assert_eq!(
        checker.perform_on_progress(&ProgressRange::default()),
        ProgressCompletion::Completed
    );
    let structure = checker.data_structure_mut().unwrap();
    assert_eq!(
        structure.interf_vz().len(),
        1,
        "count={count} coordinate={coordinate}"
    );
    let (vertex, _) = structure.interf_vz().value(0).unwrap().indices();
    assert!(
        sources
            .iter()
            .any(|source| structure.shape(vertex).unwrap().is_same(source))
    );
}
fn coincident_interior_checker(count: usize, coordinate: f64) -> (BOPAlgoCheckerSI, Vec<Shape>) {
    let mut compound = Compound::default();
    let builder = BRepBuilder::default();
    builder.make_compound(&mut compound);
    let mut cube = BRepPrimAPIMakeBox::from_sizes(10., 10., 10.).unwrap();
    builder.add(&mut compound, cube.shape().unwrap()).unwrap();
    let sources: Vec<_> = (0..count)
        .map(|_| {
            BRepBuilderAPIMakeVertex::new(Point3::new(coordinate, coordinate, coordinate))
                .unwrap()
                .vertex()
                .unwrap()
                .into_shape()
        })
        .collect();
    for source in &sources {
        builder.add(&mut compound, source).unwrap();
    }
    let mut checker = BOPAlgoCheckerSI::new();
    checker.options_mut().set_run_parallel(false);
    checker.set_arguments(&[compound.into_shape()].into_iter().collect());
    (checker, sources)
}
