// SPDX-License-Identifier: GPL-2.0-only
//! Partial ordered stage; no successful region/body API.
//! Validation consumes an already-owned arrangement candidate, not caller inputs.
mod body;
mod contexts;
#[cfg(test)]
mod coverage_tests;
#[cfg(test)]
mod diagnostic_tests;
#[cfg(test)]
mod geometry_tests;
mod inspection;
pub(crate) mod outcome;
mod self_check;
#[cfg(test)]
mod self_check_tests;
#[cfg(test)]
mod source_relation_test_support;
#[cfg(test)]
mod source_relation_tests;
mod source_relations;
mod statuses;
#[cfg(test)]
mod tests;
mod tolerance;
mod tolerance_capture;
#[cfg(test)]
mod tolerance_test_support;
#[cfg(test)]
mod tolerance_tests;
mod tolerance_types;
#[cfg(test)]
mod traversal_tests;

use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tktopalgo::brep_check::{BRepCheckAnalyzer, CylindricalCurveBounds};
use std::num::NonZeroUsize;
pub(crate) use outcome::{
    AnalyticGap, BodyKind, OrderedCoverage, SourceRelation, StageCoverage, ValidationCause,
    ValidationFinding, ValidationStage,
};
pub(crate) use self_check::inspect_self_intersection;
pub(crate) use source_relations::source_relations;
pub(crate) use tolerance::inspect_planar_edge_residual;

/// Inspect an already-owned candidate without exposing diagnostic geometry as success.
/// Example: `validate_owned_candidate(&owned_face, BodyKind::PlanarRegion, &relations)`.
/// Passed means mapped coverage only; pending analytic certificates remain explicit.
pub(crate) fn validate_owned_candidate(
    shape: &Shape,
    kind: BodyKind,
    sources: &[SourceRelation],
) -> OrderedCoverage {
    let mut report = OrderedCoverage::pending();
    inspect_stage(shape, kind, sources, ValidationStage::Topology, &mut report);
    if report.topology != StageCoverage::Passed {
        return report;
    }
    inspect_stage(shape, kind, sources, ValidationStage::Geometry, &mut report);
    if report.geometry == StageCoverage::Passed {
        let findings = self_check::inspect_self_intersection(shape, sources);
        report.record(ValidationStage::SelfIntersection, findings);
    }
    report
}

/// Run one mapped BRepCheck stage into `report` (the body check reuses it for any body).
/// Example: `inspect_stage(&solid, BodyKind::Any, &[], ValidationStage::Topology, &mut report)`.
pub(crate) fn inspect_stage(
    shape: &Shape,
    kind: BodyKind,
    sources: &[SourceRelation],
    stage: ValidationStage,
    report: &mut OrderedCoverage,
) {
    inspect_stage_with_curve_bounds(shape, kind, sources, stage, report, None);
}

pub(crate) fn inspect_stage_with_curve_bounds(
    shape: &Shape,
    kind: BodyKind,
    sources: &[SourceRelation],
    stage: ValidationStage,
    report: &mut OrderedCoverage,
    curve_checks: Option<CurveChecks<'_>>,
) {
    if let (ValidationStage::Geometry, Some(checks)) = (stage, curve_checks) {
        let findings = inspect_curves(shape, kind, sources, checks);
        report.record(stage, findings);
        return;
    }
    let outcome = BRepCheckAnalyzer::new(shape, stage == ValidationStage::Geometry, false, true);
    report.record(stage, inspect_outcome(outcome, shape, kind, sources, stage));
}

#[derive(Clone, Copy)]
pub(crate) struct CurveChecks<'a> {
    pub bounds: &'a CylindricalCurveBounds,
    pub worker_limit: NonZeroUsize,
}

fn inspect_curves(
    shape: &Shape,
    kind: BodyKind,
    sources: &[SourceRelation],
    checks: CurveChecks<'_>,
) -> Vec<ValidationFinding> {
    let stage = ValidationStage::Geometry;
    let scoped =
        BRepCheckAnalyzer::try_scoped_cylindrical_checks(shape, checks.bounds, checks.worker_limit);
    if let Some(analyzer) = scoped {
        let findings = inspection::inspect_candidate(&analyzer, shape, kind, sources, stage);
        if findings.is_empty() {
            return findings;
        }
    }
    // Incomplete contexts and defects retain the ordinary serial diagnostic order.
    let outcome = BRepCheckAnalyzer::with_cylindrical_curve_bounds(shape, checks.bounds);
    inspect_outcome(outcome, shape, kind, sources, stage)
}

fn inspect_outcome(
    outcome: Result<BRepCheckAnalyzer, Failure>,
    shape: &Shape,
    kind: BodyKind,
    sources: &[SourceRelation],
    stage: ValidationStage,
) -> Vec<ValidationFinding> {
    match outcome {
        Ok(analyzer) => inspection::inspect_candidate(&analyzer, shape, kind, sources, stage),
        Err(failure) => vec![inspection::native_failure(shape, sources, stage, failure)],
    }
}

/// Inspect owned stage output with complete member attribution, without final acceptance.
/// Example: `validate_owned_arrangement(&owned, BodyKind::PlanarRegion)`.
pub(crate) fn validate_owned_arrangement(
    owned: &crate::sketch_region::OwnedArrangement,
    kind: BodyKind,
) -> OrderedCoverage {
    let relations = source_relations(&owned.relations);
    validate_owned_candidate(&owned.arrangement, kind, &relations)
}

#[cfg(test)]
mod source_arrangement_tests;
#[cfg(test)]
mod source_context_tests;
