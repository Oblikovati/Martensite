// SPDX-License-Identifier: GPL-2.0-only
//! Share the ordinary checks before and after an operation's intersection proof.
use super::ledger::{checker_failure, stage_report};
use super::*;
use std::num::NonZeroUsize;

/// Outcome of raw tolerance admission, topology, and geometry.
pub enum BodyGeometryValidation<'a> {
    /// An earlier stage refused or was inconclusive. Later stages were skipped.
    Stopped(ValidationReport),
    /// Raw tolerance admission, exact topology, and sampled geometry passed on this body.
    Ready(OrderedBodyGeometry<'a>),
}

/// The admitted body, topology index and original resolution budget for later validation.
///
/// Keep the decoded body and its native storage unchanged until the checkpoint is consumed.
pub struct OrderedBodyGeometry<'a> {
    shape: &'a Shape,
    names: TopologyNames,
    root: TopologyRef,
    ledger: Ledger,
    census: Vec<EulerCensus>,
    budget: Option<f64>,
}

/// Run raw tolerance admission, exact topology, and sampled geometry in their ordinary order.
/// Example: `let state = begin_body_validation(&decoded, &resolution)?;`.
pub fn begin_body_validation<'a>(
    shape: &'a Shape,
    resolution: &Resolution,
) -> Result<BodyGeometryValidation<'a>, ResolutionRejected> {
    begin(shape, resolution, None)
}

/// Run the ordinary ordered checks, reusing only full-domain cylinder curve bounds
/// whose native geometry, placement and tolerance agree with the loaded representation.
/// Example: `let state = begin_body_validation_with_curve_bounds(&decoded, &resolution, &bounds)?;`.
pub fn begin_body_validation_with_curve_bounds<'a>(
    shape: &'a Shape,
    resolution: &Resolution,
    bounds: &CylindricalCurveBounds,
) -> Result<BodyGeometryValidation<'a>, ResolutionRejected> {
    begin_body_validation_with_scoped_curve_checks(shape, resolution, bounds, NonZeroUsize::MIN)
}

/// Run the ordinary ordered body checks with a per-invocation geometry worker limit.
///
/// Distinct face contexts may be checked after the serial topology stage. Refusals and
/// incomplete contexts are reported by the ordinary serial geometry checker.
/// Example: `begin_body_validation_with_scoped_curve_checks(&decoded, &resolution, &bounds, limit)?`.
pub fn begin_body_validation_with_scoped_curve_checks<'a>(
    shape: &'a Shape,
    resolution: &Resolution,
    bounds: &CylindricalCurveBounds,
    worker_limit: NonZeroUsize,
) -> Result<BodyGeometryValidation<'a>, ResolutionRejected> {
    begin(
        shape,
        resolution,
        Some(CurveChecks {
            bounds,
            worker_limit,
        }),
    )
}

fn begin<'a>(
    shape: &'a Shape,
    resolution: &Resolution,
    bounds: Option<CurveChecks<'_>>,
) -> Result<BodyGeometryValidation<'a>, ResolutionRejected> {
    let budget = tolerance_budget(resolution)?;
    let names = match TopologyNames::of(shape) {
        Ok(names) => names,
        Err(failure) => {
            return Ok(BodyGeometryValidation::Stopped(unindexable(
                shape, &failure, budget,
            )));
        }
    };
    let root = names.name(shape);
    if let Some(report) = admission::inspect(&names, root, budget) {
        return Ok(BodyGeometryValidation::Stopped(report));
    }
    Ok(check_geometry(shape, names, root, budget, bounds))
}

fn check_geometry<'a>(
    shape: &'a Shape,
    names: TopologyNames,
    root: TopologyRef,
    budget: Option<f64>,
    bounds: Option<CurveChecks<'_>>,
) -> BodyGeometryValidation<'a> {
    let mut stages = Stages {
        shape,
        names: &names,
        root,
        ledger: Ledger::new(),
    };
    let (passed, census) = stages.topology();
    if !passed || !stages.geometry(bounds) {
        return BodyGeometryValidation::Stopped(stages.ledger.finish(census, budget));
    }
    let ledger = stages.ledger;
    BodyGeometryValidation::Ready(OrderedBodyGeometry {
        shape,
        names,
        root,
        ledger,
        census,
        budget,
    })
}

impl OrderedBodyGeometry<'_> {
    /// Read the actual preliminary report with the two remaining stages marked `NotRun`.
    /// Example: `let prefix = ready.report();`.
    pub fn report(&self) -> ValidationReport {
        self.ledger.report(self.census.clone(), self.budget)
    }

    /// Check the admitted body's ordinary tolerance rules after a separate intersection proof.
    ///
    /// Returns the tolerance stage alone, using the original budget and body index.
    /// Example: `let tolerance = ready.check_late_tolerance();`.
    pub fn check_late_tolerance(self) -> StageReport {
        check_tolerance(&self.names, self.root, self.budget)
    }

    pub(super) fn finish_ordered(
        self,
        intersection: impl FnOnce(&mut Stages<'_>) -> bool,
    ) -> ValidationReport {
        let mut stages = Stages {
            shape: self.shape,
            names: &self.names,
            root: self.root,
            ledger: self.ledger,
        };
        if intersection(&mut stages) {
            let tolerance = check_tolerance(&self.names, self.root, self.budget);
            stages.ledger.record_stage(tolerance);
        }
        stages.ledger.finish(self.census, self.budget)
    }
}

fn check_tolerance(names: &TopologyNames, root: TopologyRef, budget: Option<f64>) -> StageReport {
    let stage = ValidationStage::Tolerance;
    match tolerance::tolerance_findings(names, budget) {
        Ok((findings, mode)) => stage_report(stage, mode, findings),
        Err(failure) => checker_failure(stage, root, &failure),
    }
}

#[cfg(test)]
mod tests;
