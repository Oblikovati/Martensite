//! CPU-only refusal and independent analytic controls; none creates a viewer or resolves a font.
use super::{boolean_measurements, boolean_outcome::*, boolean_validation};
use martensite_tkbo::bop_algo::{
    Operation, ProgressCompletion, ProgressFailureClass, ProgressOperation, ProgressStage,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;

fn request() -> BooleanRequest {
    BooleanRequest {
        case: "owned_bop4_cpu",
        ordinal: 1,
        operation: Operation::Fuse,
        operands: ["owned_first", "owned_second"],
        authority: super::boolean_outcome::OperandAuthority::ProceduralSource,
    }
}

#[test]
fn typed_incomplete_execution_cannot_release_a_shape() {
    let status = ProgressCompletion::Incomplete {
        operation: ProgressOperation::CheckerSI,
        stage: ProgressStage::Worker,
        class: ProgressFailureClass::UnqualifiedStage,
        cause: None,
        context: None,
    };
    let issue = execution_status(request(), "SelfIntersection", status).unwrap_err();
    assert_eq!(issue.class, RefusalClass::Incomplete);
    assert_eq!(issue.stage, "SelfIntersection");
}

#[test]
fn typed_cancelled_execution_remains_distinct_from_failure() {
    let status = ProgressCompletion::Cancelled {
        operation: ProgressOperation::Builder,
        stage: ProgressStage::Images,
        context: None,
    };
    assert_eq!(
        execution_status(request(), "Execution", status)
            .unwrap_err()
            .class,
        RefusalClass::Cancelled
    );
    assert!(execution_status(request(), "Execution", ProgressCompletion::Completed).is_ok());
}

#[test]
fn missing_measurement_coverage_retains_typed_cause() {
    let cause = Failure::with_type(
        "BOP4_IncompleteCoverage",
        "distance=NotDone; expected measured distance",
        "",
    );
    let issue = kernel_issue(request(), "Tolerance", cause);
    assert_eq!(issue.class, RefusalClass::Incomplete);
    assert!(matches!(*issue.cause, BooleanCause::Kernel(_)));
}

#[test]
fn actual_general_pipeline_null_operands_refuse_publication() {
    let outcome = execute_boolean(request(), Shape::default(), Shape::default());
    let issue = match outcome {
        Ok(_) => panic!("null operands escaped; expected typed refusal"),
        Err(issue) => issue,
    };
    assert!(matches!(
        issue.class,
        RefusalClass::Failed | RefusalClass::Incomplete
    ));
    assert!(issue.to_string().contains("owned_bop4_cpu"));
    let _native_messages = (&issue.diagnostics().warnings, &issue.diagnostics().errors);
}

#[test]
fn real_nurbs_body_cannot_upgrade_self_search_coverage() {
    use martensite_tktopalgo::brep_builder_api::{
        BRepBuilderAPIMakeShape, BRepBuilderAPINurbsConvert,
    };
    let cube = crate::primitive_shapes::box_sized([1.; 3]).unwrap();
    let mut conversion = BRepBuilderAPINurbsConvert::with_shape(&cube, false).unwrap();
    let issue = boolean_validation::validate(request(), conversion.shape().unwrap()).unwrap_err();
    assert_eq!(issue.class, RefusalClass::Incomplete);
    assert_eq!(issue.stage, "SelfIntersection");
    assert!(matches!(
        *issue.cause,
        BooleanCause::Execution(ProgressCompletion::Incomplete {
            class: ProgressFailureClass::UnqualifiedStage,
            ..
        })
    ));
}

#[test]
fn sphere_cube_formula_uses_six_disjoint_caps() {
    let (volume, area) = boolean_measurements::sphere_cube_expectation();
    let cap = std::f64::consts::PI * (6. - 1. / 3.);
    assert!((volume - (1000. + 6. * cap)).abs() < 1e-12);
    assert!(
        (area - (600. + 72. * std::f64::consts::PI - 66. * std::f64::consts::PI)).abs() < 1e-12
    );
}

#[test]
fn blind_bore_formula_accounts_for_blind_bottom_disks() {
    let (volume, area) = boolean_measurements::blind_bore_expectation();
    let pi = std::f64::consts::PI;
    assert!((volume - (200. * 100. * 100. - 5. * pi * 10f64.powi(2) * 50.)).abs() < 1e-9);
    assert!(
        (area - (2. * (200. * 100. + 200. * 100. + 100. * 100.) + 5. * 2. * pi * 10. * 50.)).abs()
            < 1e-9
    );
}

#[test]
fn actual_general_union_checks_history_and_geometric_properties() {
    let first = crate::primitive_shapes::box_sized([2.; 3]).unwrap();
    let second = crate::primitive_shapes::box_at([1., 0., 0.], [2.; 3]).unwrap();
    let body = execute_boolean(request(), first, second)
        .expect("actual general union must pass ordered existing checks");
    check_union_properties(&body);
    check_union_coverage_and_history(&body);
}

#[test]
fn actual_general_empty_cut_is_not_a_failed_or_null_result() {
    let first = crate::primitive_shapes::box_sized([1.; 3]).unwrap();
    let mut request = request();
    request.operation = Operation::Cut;
    let outcome = execute_boolean(request, first.clone(), first);
    let issue = match outcome {
        Ok(_) => panic!("identical cut released material; expected completed empty"),
        Err(issue) => issue,
    };
    assert_eq!(issue.class, RefusalClass::CompletedEmpty);
}

struct Bop4ImmediateCancellation;
impl martensite_tkernel::message::ProgressIndicatorHooks for Bop4ImmediateCancellation {
    // This named fake only requests cancellation; it performs no display or external I/O.
    fn show(
        &self,
        _scope: &martensite_tkernel::message::ProgressScope<'_>,
        _position: f64,
        _force: bool,
    ) {
    }
    fn user_break(&self) -> bool {
        true
    }
}

#[test]
fn actual_general_cancellation_does_not_release_a_candidate() {
    use martensite_tkernel::message::ProgressIndicator;
    let progress = ProgressIndicator::new(Box::new(Bop4ImmediateCancellation));
    let first = crate::primitive_shapes::box_sized([2.; 3]).unwrap();
    let second = crate::primitive_shapes::box_at([1., 0., 0.], [2.; 3]).unwrap();
    let outcome = execute_boolean_on_progress(request(), first, second, &progress.start());
    let issue = match outcome {
        Ok(_) => panic!("cancelled operation released material; expected typed cancellation"),
        Err(issue) => issue,
    };
    assert_eq!(issue.class, RefusalClass::Cancelled);
}

fn check_union_properties(body: &CheckedLocalBody) {
    assert!(body.diagnostics().errors.is_empty());
    let values = body.measurements();
    assert!((values.volume - 12.).abs() < 1e-7);
    assert!((values.surface.properties().mass() - 32.).abs() < 1e-7);
    assert_eq!(
        values.volume_error,
        boolean_measurements::AchievedVolumeError::Unknown
    );
}

fn check_union_coverage_and_history(body: &CheckedLocalBody) {
    let checks = body.checks();
    assert!(
        checks.topology > 0
            && checks.geometry > 0
            && checks.self_faces > 0
            && checks.measured_incidences > 0
    );
    check_union_parent_relations(body);
    assert_eq!(
        checks.mode,
        boolean_validation::LocalCheckMode::DegeneratePcurveExclusionV2
    );
    assert_eq!(
        checks.self_policy_excluded + checks.self_terminated,
        checks.self_faces
    );
}

fn check_union_parent_relations(body: &CheckedLocalBody) {
    let history = &body.history().relations;
    assert!(!history.is_empty() && history.iter().all(|relation| !relation.parent.is_null()));
    let changed: usize = history
        .iter()
        .map(|relation| {
            relation.modified.len() + relation.generated.len() + usize::from(relation.deleted)
        })
        .sum();
    assert!(changed > 0);
}

#[test]
fn diagnostics_keep_both_operation_reports_in_source_order() {
    let first = KernelDiagnostics {
        warnings: "warning-first\n".into(),
        errors: String::new(),
    };
    let second = KernelDiagnostics {
        warnings: "warning-second\n".into(),
        errors: "error-second\n".into(),
    };
    let joined = first.followed_by(second);
    assert_eq!(joined.warnings, "warning-first\nwarning-second\n");
    assert_eq!(joined.errors, "error-second\n");
}
