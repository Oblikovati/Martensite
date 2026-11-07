//! Ordered checks retain missing coverage instead of interpreting it as no defect.
use super::boolean_coverage::{CoverageStage, OperationCoverage};
use super::boolean_outcome::{
    BooleanIssue, BooleanRequest, RefusalClass, execution_status, kernel_issue, refused,
};
use martensite_tkbo::bop_algo::{BOPAlgoCheckerSI, SelfIntersectionFaceStatus};
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::Explorer,
    topods::{self, Shape, ShapeIterator},
    toptools::MapOfShape,
};
use martensite_tkernel::{message::ProgressRange, ncollection::List, standard::Failure};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_check::{BRepCheckAnalyzer, BRepCheckResult, BRepCheckStatus};

/// Local existing-engine checks; this is not an exact geometry certificate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum LocalCheckMode {
    ExistingOrderedChecksV1,
    DegeneratePcurveExclusionV2,
}

const CURRENT_CHECK_MODE: LocalCheckMode = LocalCheckMode::DegeneratePcurveExclusionV2;

#[derive(Debug)]
pub(super) struct ValidationReceipt {
    pub mode: LocalCheckMode,
    pub topology: usize,
    pub geometry: usize,
    pub self_faces: usize,
    pub self_policy_excluded: usize,
    pub self_terminated: usize,
    pub measured_incidences: usize,
    pub degenerated_incidences: usize,
}

pub(super) fn entities(shape: &Shape, kind: ShapeEnum) -> Result<Vec<Shape>, Failure> {
    Explorer::new(shape, kind, ShapeEnum::Shape).map(Iterator::collect)
}

pub(super) fn validate(
    request: BooleanRequest,
    shape: &Shape,
) -> Result<ValidationReceipt, BooleanIssue> {
    let mut coverage = OperationCoverage::default();
    match validate_observed_with_mode(request, shape, &mut coverage, CURRENT_CHECK_MODE) {
        Ok(receipt) => Ok(receipt),
        Err(issue) => Err(issue.with_coverage(coverage)),
    }
}

pub(super) fn validate_observed(
    request: BooleanRequest,
    shape: &Shape,
    coverage: &mut OperationCoverage,
) -> Result<ValidationReceipt, BooleanIssue> {
    validate_observed_with_mode(request, shape, coverage, CURRENT_CHECK_MODE)
}

pub(super) fn validate_observed_with_mode(
    request: BooleanRequest,
    shape: &Shape,
    coverage: &mut OperationCoverage,
    mode: LocalCheckMode,
) -> Result<ValidationReceipt, BooleanIssue> {
    coverage.mode = match mode {
        LocalCheckMode::ExistingOrderedChecksV1 => "ExistingOrderedChecksV1",
        LocalCheckMode::DegeneratePcurveExclusionV2 => "DegeneratePcurveExclusionV2",
    };
    let topology = observed_analyzer(request, shape, false, coverage)?;
    let geometry = observed_analyzer(request, shape, true, coverage)?;
    let (self_faces, self_policy_excluded, self_terminated) =
        observed_self_check(request, shape, coverage)?;
    let (measured_incidences, degenerated_incidences) =
        observed_tolerances(request, shape, coverage, mode)?;
    Ok(ValidationReceipt {
        mode,
        topology,
        geometry,
        self_faces,
        self_policy_excluded,
        self_terminated,
        measured_incidences,
        degenerated_incidences,
    })
}

fn observed_analyzer(
    request: BooleanRequest,
    shape: &Shape,
    geometry: bool,
    coverage: &mut OperationCoverage,
) -> Result<usize, BooleanIssue> {
    let stage = if geometry {
        CoverageStage::Geometry
    } else {
        CoverageStage::Topology
    };
    match analyzer_pass_observed(request, shape, geometry, coverage, stage) {
        Ok(count) => {
            coverage.completed(stage, count);
            Ok(count)
        }
        Err(issue) => {
            coverage.refused(stage, issue.class, issue.to_string());
            Err(issue)
        }
    }
}

fn analyzer_pass_observed(
    request: BooleanRequest,
    shape: &Shape,
    geometry: bool,
    coverage: &mut OperationCoverage,
    observed_stage: CoverageStage,
) -> Result<usize, BooleanIssue> {
    let stage = if geometry { "Geometry" } else { "Topology" };
    let analyzer = BRepCheckAnalyzer::new(shape, geometry, false, geometry)
        .map_err(|cause| kernel_issue(request, stage, cause))?;
    if !analyzer
        .is_valid()
        .map_err(|cause| kernel_issue(request, stage, cause))?
    {
        return Err(refused(
            request,
            stage,
            format!("candidate={shape:?}"),
            "all analyzer statuses NoError",
            RefusalClass::Failed,
        ));
    }
    check_body_kind(request, shape)?;
    required_contexts_observed(&analyzer, shape, coverage, observed_stage)
        .map_err(|cause| kernel_issue(request, stage, cause))
}

fn required_contexts_observed(
    analyzer: &BRepCheckAnalyzer,
    shape: &Shape,
    coverage: &mut OperationCoverage,
    stage: CoverageStage,
) -> Result<usize, Failure> {
    let mut count = 0;
    for (parent, child) in [
        (ShapeEnum::Edge, ShapeEnum::Vertex),
        (ShapeEnum::Face, ShapeEnum::Wire),
        (ShapeEnum::Face, ShapeEnum::Edge),
        (ShapeEnum::Face, ShapeEnum::Vertex),
        (ShapeEnum::Solid, ShapeEnum::Shell),
    ] {
        for context in entities(shape, parent)? {
            count += required_children_observed(analyzer, &context, child, coverage, stage)?;
        }
    }
    Ok(count)
}

fn check_context(
    analyzer: &BRepCheckAnalyzer,
    entity: &Shape,
    parent: &Shape,
) -> Result<(), Failure> {
    let checker = analyzer.result(entity)?;
    let checker = checker.get().ok_or_else(|| {
        Failure::new("context checker=None; expected a required subshape checker")
    })?;
    checker.init_context_iterator()?;
    while checker.more_shape_in_context() {
        if checker.contextual_shape()?.is_same(parent) {
            return check_context_status(checker, entity, parent);
        }
        checker.next_shape_in_context();
    }
    Err(Failure::with_type(
        "BOP4_IncompleteCoverage",
        &format!("entity={entity:?} parent={parent:?}; expected an observed required context"),
        "",
    ))
}

fn check_context_status(
    checker: &dyn BRepCheckResult,
    entity: &Shape,
    parent: &Shape,
) -> Result<(), Failure> {
    if checker
        .status_on_shape()?
        .iter()
        .all(|status| *status == BRepCheckStatus::NoError)
    {
        return Ok(());
    }
    Err(Failure::new(&format!(
        "entity={entity:?} parent={parent:?}; expected NoError context status"
    )))
}

fn check_body_kind(request: BooleanRequest, shape: &Shape) -> Result<(), BooleanIssue> {
    let kind = shape
        .shape_type()
        .map_err(|cause| kernel_issue(request, "BodyKind", cause))?;
    if kind == ShapeEnum::Solid {
        return Ok(());
    }
    if !matches!(kind, ShapeEnum::Compound | ShapeEnum::CompSolid) {
        return Err(refused(
            request,
            "BodyKind",
            format!("kind={kind:?}"),
            "closed material solids or containers of solids",
            RefusalClass::Failed,
        ));
    }
    check_container(request, shape)
}

fn check_container(request: BooleanRequest, shape: &Shape) -> Result<(), BooleanIssue> {
    let mut children = ShapeIterator::new(shape, true, true);
    let mut count = 0;
    while let Ok(child) = children.value() {
        check_body_kind(request, child)?;
        count += 1;
        children.next();
    }
    if count == 0 {
        return Err(refused(
            request,
            "BodyKind",
            "container children=0".into(),
            "nonempty material body for this display case",
            RefusalClass::CompletedEmpty,
        ));
    }
    Ok(())
}

fn observed_self_check(
    request: BooleanRequest,
    shape: &Shape,
    coverage: &mut OperationCoverage,
) -> Result<(usize, usize, usize), BooleanIssue> {
    let mut checker = BOPAlgoCheckerSI::new();
    checker.set_arguments(&[shape.clone()].into_iter().collect::<List<Shape>>());
    checker.options_mut().set_run_parallel(false);
    let receipt = checker.perform_with_progress_observed(&ProgressRange::default());
    coverage.self_receipt(
        &receipt,
        checker.options().fuzzy_value(),
        checker.options().run_parallel(),
    );
    finish_self_check(request, receipt, coverage)
}

fn finish_self_check(
    request: BooleanRequest,
    receipt: martensite_tkbo::bop_algo::SelfIntersectionObservation,
    coverage: &mut OperationCoverage,
) -> Result<(usize, usize, usize), BooleanIssue> {
    let findings = check_self_diagnostics(request, &receipt)
        .and_then(|()| check_face_termination(request, &receipt));
    let counts = counted_self_faces(&receipt);
    for _ in receipt.faces() {
        coverage.observed(CoverageStage::SelfIntersection);
    }
    let result = findings
        .and_then(|()| execution_status(request, "SelfIntersection", receipt.into_completion()));
    record_self_result(result, counts, coverage)
}

fn record_self_result(
    result: Result<(), BooleanIssue>,
    counts: (usize, usize, usize),
    coverage: &mut OperationCoverage,
) -> Result<(usize, usize, usize), BooleanIssue> {
    match result {
        Ok(()) => {
            coverage.completed(CoverageStage::SelfIntersection, counts.0);
            Ok(counts)
        }
        Err(issue) => {
            coverage.refused(
                CoverageStage::SelfIntersection,
                issue.class,
                issue.to_string(),
            );
            Err(issue)
        }
    }
}

fn check_self_diagnostics(
    request: BooleanRequest,
    receipt: &martensite_tkbo::bop_algo::SelfIntersectionObservation,
) -> Result<(), BooleanIssue> {
    if receipt.pairs().is_empty() && receipt.diagnostics().is_empty() {
        return Ok(());
    }
    let actual = format!(
        "pairs={:?} diagnostics={:?}",
        receipt.pairs(),
        receipt.diagnostics()
    );
    Err(refused(
        request,
        "SelfIntersection",
        actual,
        "no self-interference or native diagnostic",
        RefusalClass::Failed,
    ))
}

fn check_face_termination(
    request: BooleanRequest,
    receipt: &martensite_tkbo::bop_algo::SelfIntersectionObservation,
) -> Result<(), BooleanIssue> {
    let unfinished = receipt.faces().iter().any(|face| {
        matches!(
            face.status(),
            SelfIntersectionFaceStatus::NotDone | SelfIntersectionFaceStatus::NotAttempted
        )
    });
    if !unfinished {
        return Ok(());
    }
    Err(refused(
        request,
        "SelfIntersection",
        format!("faces={:?}", receipt.faces()),
        "complete required face work",
        RefusalClass::Incomplete,
    ))
}

// Analyzer contexts use the first matching identity occurrence, not each oriented alias.
fn required_children(
    analyzer: &BRepCheckAnalyzer,
    context: &Shape,
    kind: ShapeEnum,
) -> Result<usize, Failure> {
    required_children_observed(
        analyzer,
        context,
        kind,
        &mut OperationCoverage::default(),
        CoverageStage::Topology,
    )
}

#[cfg(test)]
#[path = "boolean_context_controls.rs"]
mod context_controls;

fn counted_self_faces(
    receipt: &martensite_tkbo::bop_algo::SelfIntersectionObservation,
) -> (usize, usize, usize) {
    let face_count = receipt.faces().len();
    let excluded = receipt
        .faces()
        .iter()
        .filter(|face| face.status() == SelfIntersectionFaceStatus::PolicyExcluded)
        .count();
    let terminated = receipt
        .faces()
        .iter()
        .filter(|face| matches!(face.status(), SelfIntersectionFaceStatus::Terminated { .. }))
        .count();
    (face_count, excluded, terminated)
}

fn observed_tolerances(
    request: BooleanRequest,
    shape: &Shape,
    coverage: &mut OperationCoverage,
    mode: LocalCheckMode,
) -> Result<(usize, usize), BooleanIssue> {
    match super::boolean_incidence::tolerances(request, shape, coverage, mode) {
        Ok((measured, degenerated)) => {
            coverage.completed(CoverageStage::Tolerance, measured + degenerated);
            Ok((measured, degenerated))
        }
        Err(issue) => {
            coverage.refused(CoverageStage::Tolerance, issue.class, issue.to_string());
            Err(issue)
        }
    }
}

fn required_children_observed(
    analyzer: &BRepCheckAnalyzer,
    context: &Shape,
    kind: ShapeEnum,
    coverage: &mut OperationCoverage,
    stage: CoverageStage,
) -> Result<usize, Failure> {
    let mut seen = MapOfShape::default();
    let mut count = 0;
    for entity in entities(context, kind)? {
        if !seen.add(entity.clone()) {
            continue;
        }
        check_context(analyzer, &entity, context)?;
        count += 1;
        coverage.observed(stage);
    }
    Ok(count)
}
