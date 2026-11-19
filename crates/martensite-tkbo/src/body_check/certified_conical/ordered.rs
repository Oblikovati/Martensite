// SPDX-License-Identifier: GPL-2.0-only
//! Explicit certified mode replaces only the ordered self-intersection stage.
use super::*;
use crate::body_check::{
    CheckMode, Finding, FindingKind, Ledger, ResolutionRejected, Severity, Stages,
    ValidationReport, ValidationStage, admission, naming::TopologyNames, tolerance_budget,
    unindexable,
};
use crate::profile_extrusion::Resolution;

/// Require a complete rebound certificate inside the ordinary ordered body checks.
/// Geometry remains sampled; an exact SI stage does not make the whole report exact.
/// Example: `let report = validate_body_with_certified_conical(&member, &resolution, &proof)?;`.
pub fn validate_body_with_certified_conical(
    member: &ConicalNativeMember,
    resolution: &Resolution,
    evidence: &CertifiedConicalBody,
) -> Result<ValidationReport, ResolutionRejected> {
    validate_member(member, resolution, || evidence.rebind_member(member))
}
fn validate_member(
    member: &ConicalNativeMember,
    resolution: &Resolution,
    mut rebind: impl FnMut() -> Result<(), ConicalBindingError>,
) -> Result<ValidationReport, ResolutionRejected> {
    let budget = tolerance_budget(resolution)?;
    let names = match TopologyNames::of(&member.shape) {
        Ok(names) => names,
        Err(cause) => return Ok(unindexable(&member.shape, &cause, budget)),
    };
    let root = names.name(&member.shape);
    if let Some(report) = admission::inspect(&names, root, budget) {
        return Ok(report);
    }
    Ok(check(member, &names, budget, &mut rebind))
}
fn check(
    member: &ConicalNativeMember,
    names: &TopologyNames,
    budget: Option<f64>,
    mut rebind: impl FnMut() -> Result<(), ConicalBindingError>,
) -> ValidationReport {
    let mut stages = Stages {
        shape: &member.shape,
        names,
        root: names.name(&member.shape),
        ledger: Ledger::new(),
    };
    let (passed, census) = stages.topology();
    if passed && stages.geometry() && certified_stage(&mut stages, member, &mut rebind) {
        stages.tolerance(budget);
    }
    stages.ledger.finish(census, budget)
}
fn certified_stage(
    stages: &mut Stages<'_>,
    member: &ConicalNativeMember,
    rebind: &mut impl FnMut() -> Result<(), ConicalBindingError>,
) -> bool {
    let findings = match rebind() {
        Ok(()) => Vec::new(),
        Err(cause) => vec![finding(stages, member, &cause)],
    };
    stages.ledger.record(
        ValidationStage::SelfIntersection,
        CheckMode::Exact,
        findings,
    )
}
fn finding(
    stages: &Stages<'_>,
    member: &ConicalNativeMember,
    cause: &ConicalBindingError,
) -> Finding {
    let subject = subject(stages, member, cause);
    Finding {
        severity: Severity::Inconclusive,
        kind: FindingKind::SearchIncomplete {
            reason: cause.to_string(),
        },
        subject,
        context: None,
    }
}

#[cfg(test)]
mod tests;

fn subject(
    stages: &Stages<'_>,
    member: &ConicalNativeMember,
    cause: &ConicalBindingError,
) -> crate::body_check::TopologyRef {
    let side = cause.side.and_then(|id| {
        member
            .sides
            .iter()
            .find(|(candidate, _)| *candidate == id)
            .map(|(_, face)| face)
    });
    let cap = cause.cap.and_then(|id| {
        member
            .caps
            .iter()
            .find(|cap| cap.id == id)
            .map(|cap| &cap.face)
    });
    side.or(cap)
        .map_or(stages.root, |face| stages.names.name(face))
}
