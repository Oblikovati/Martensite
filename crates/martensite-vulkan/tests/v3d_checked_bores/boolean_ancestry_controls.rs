//! Genuine continuation and conversion observations retain ancestry without publishing incomplete geometry.
use super::{boolean_history, boolean_outcome::*, boolean_validation};
use martensite_tkbo::bop_algo::{Operation, ProgressCompletion, ProgressFailureClass};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPINurbsConvert;

fn request(ordinal: usize, operation: Operation, operands: [&'static str; 2]) -> BooleanRequest {
    BooleanRequest {
        case: "owned_bop4_ancestry",
        ordinal,
        operation,
        operands,
        authority: OperandAuthority::ProceduralSource,
    }
}

fn genuine_union() -> CheckedLocalBody {
    let first = crate::primitive_shapes::box_sized([2.; 3]).unwrap();
    let second = crate::primitive_shapes::box_at([1., 0., 0.], [2.; 3]).unwrap();
    execute_boolean(request(1, Operation::Fuse, ["b0", "b1"]), first, second)
        .expect("real general union must complete checked local acquisition")
}

#[test]
fn actual_union_then_cut_composes_all_initial_parent_bindings() {
    let previous = genuine_union();
    let tool = crate::primitive_shapes::box_at([1., 0., 0.], [1.; 3]).unwrap();
    let body = continue_boolean(
        request(2, Operation::Cut, ["previous", "tool"]),
        previous,
        tool,
    )
    .expect("real continuation must retain initial ancestry");
    assert!((body.measurements().volume - 11.).abs() < 1e-7);
    check_composed_bindings(&body);
}

#[test]
fn actual_nurbs_map_matches_the_converters_modified_shape_query() {
    let cube = crate::primitive_shapes::box_sized([1.; 3]).unwrap();
    let mut converter = BRepBuilderAPINurbsConvert::with_shape(&cube, false).unwrap();
    let history = boolean_history::observe_nurbs(
        request(1, Operation::Fuse, ["cube", "none"]),
        &cube,
        &mut converter,
    )
    .unwrap();
    check_actual_face_images(&cube, &mut converter, &history);
}

#[test]
fn converted_body_refuses_unqualified_self_search_with_actual_ancestry() {
    let outcome = convert_nurbs(request(1, Operation::Fuse, ["b0", "b1"]), genuine_union());
    let issue = match outcome {
        Ok(_) => panic!("converted shape escaped; expected actual self-search refusal"),
        Err(issue) => issue,
    };
    check_exact_nurbs_refusal(&issue);
    check_refused_ancestry(&issue);
    check_retained_validation(&issue);
}

fn check_snapshot(snapshot: &boolean_history::HistoryObservation) {
    assert!(!snapshot.relations.is_empty());
    assert!(
        snapshot
            .relations
            .iter()
            .all(|relation| !relation.parent.is_empty()
                && relation.invocation > 0
                && relation.operand < 2)
    );
    check_observed_relation_counts(snapshot);
}

fn check_composed_bindings(body: &CheckedLocalBody) {
    let snapshot = body.history().observation();
    assert_eq!(snapshot.steps, ["BOP", "BOP"]);
    for binding in ["b0", "b1", "tool"] {
        assert!(
            snapshot
                .relations
                .iter()
                .any(|relation| relation.binding == binding)
        );
    }
    assert!(
        !snapshot
            .relations
            .iter()
            .any(|relation| relation.binding == "previous")
    );
    check_snapshot(&snapshot);
}

fn check_actual_face_images(
    cube: &martensite_tkbrep::topods::Shape,
    converter: &mut BRepBuilderAPINurbsConvert,
    history: &boolean_history::LocalHistory,
) {
    let mut count = 0;
    for parent in
        boolean_validation::entities(cube, martensite_tkg3d::top_abs::ShapeEnum::Face).unwrap()
    {
        let expected = converter.modified_shape(&parent).unwrap();
        let actual = history
            .relations
            .iter()
            .find(|relation| relation.parent.is_same(&parent))
            .unwrap();
        assert!(actual.modified.iter().any(|image| image.is_same(&expected)));
        count += 1;
    }
    assert_eq!(count, 6);
}

fn check_exact_nurbs_refusal(issue: &BooleanIssue) {
    assert_eq!(issue.stage, "SelfIntersection");
    assert_eq!(issue.class, RefusalClass::Incomplete);
    assert!(matches!(
        &*issue.cause,
        BooleanCause::Execution(ProgressCompletion::Incomplete {
            class: ProgressFailureClass::UnqualifiedStage,
            ..
        })
    ));
}

fn check_observed_relation_counts(snapshot: &boolean_history::HistoryObservation) {
    let (modifications, generations, deletions) = observed_counts(snapshot);
    assert!(modifications + generations + deletions > 0);
    assert!(
        snapshot
            .relations
            .iter()
            .any(|relation| relation.native_supported)
    );
}

fn check_refused_ancestry(issue: &BooleanIssue) {
    let snapshot = issue
        .history_observation()
        .expect("actual composed ancestry must survive refusal without geometry handles");
    assert_eq!(snapshot.steps, ["BOP", "NurbsConvert"]);
    for binding in ["b0", "b1"] {
        assert!(
            snapshot
                .relations
                .iter()
                .any(|relation| relation.binding == binding)
        );
    }
    check_snapshot(snapshot);
}

fn observed_counts(snapshot: &boolean_history::HistoryObservation) -> (usize, usize, usize) {
    snapshot
        .relations
        .iter()
        .fold((0, 0, 0), |(modified, generated, deleted), relation| {
            (
                modified + relation.modified.len(),
                generated + relation.generated.len(),
                deleted + usize::from(relation.deleted),
            )
        })
}

fn check_retained_validation(issue: &BooleanIssue) {
    use super::boolean_coverage::{CoverageStage as Stage, StageState};
    let coverage = issue.coverage();
    for stage in [Stage::Topology, Stage::Geometry] {
        assert_eq!(coverage.stages[&stage].state, StageState::Completed);
        assert!(coverage.stages[&stage].observed_items > 0);
    }
    assert_eq!(
        coverage.stages[&Stage::SelfIntersection].state,
        StageState::Missing
    );
    check_later_unattempted(coverage);
    check_validation_options(coverage);
    check_self_observation(coverage.self_check.as_ref().unwrap());
}

fn check_later_unattempted(coverage: &super::boolean_coverage::OperationCoverage) {
    use super::boolean_coverage::{CoverageStage as Stage, StageState};
    for stage in [
        Stage::Tolerance,
        Stage::InputAuthority,
        Stage::SurfaceMeasurement,
        Stage::VolumeMeasurement,
        Stage::FiniteMeasurements,
    ] {
        assert_eq!(coverage.stages[&stage].state, StageState::Unattempted);
    }
}

fn check_validation_options(coverage: &super::boolean_coverage::OperationCoverage) {
    assert_eq!(
        coverage.analyzer_options,
        [(false, false, false), (true, false, true)]
    );
    assert_eq!(coverage.mode, "DegeneratePcurveExclusionV2");
}

fn check_self_observation(receipt: &super::boolean_coverage::SelfCheckObservation) {
    use martensite_tkbo::bop_algo::SelfIntersectionFaceStatus;
    assert!(receipt.level > 0 && !receipt.phase.is_empty());
    assert!(receipt.scheduled_faces && receipt.rejected_arguments.is_none());
    assert!(
        !receipt.faces.is_empty() && receipt.pairs.is_empty() && receipt.diagnostics.is_empty()
    );
    assert!(receipt.faces.iter().any(|face| matches!(
        face.status(),
        SelfIntersectionFaceStatus::Terminated {
            has_contacts: false
        }
    )));
    assert!(receipt.faces.iter().all(|face| face.source_index() >= 0));
    assert!(matches!(
        &receipt.completion,
        ProgressCompletion::Incomplete {
            stage: martensite_tkbo::bop_algo::ProgressStage::Worker,
            class: ProgressFailureClass::UnqualifiedStage,
            ..
        }
    ));
    assert!(!receipt.parallel && receipt.fuzzy.is_finite());
    // The actual active source can be absent after post-treatment, never synthesized.
    let _observed_active_source = receipt.active_source;
}
