// SPDX-License-Identifier: GPL-2.0-only
//! Explicit certified mode replaces only the ordered self-intersection stage.
use super::*;
use crate::body_check::{
    BodyGeometryValidation, CheckMode, Finding, FindingKind, ResolutionRejected, Severity, Stages,
    ValidationReport, ValidationStage, begin_body_validation,
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
/// Rebind the canonical certificate under the exact stored, invertible placement matrix.
/// Topology, sampled geometry and tolerances are checked on the placed shape.
/// Example: `validate_body_with_transported_conical(&member, &resolution, &proof, &frame)?;`.
pub fn validate_body_with_transported_conical(
    member: &ConicalNativeMember,
    resolution: &Resolution,
    evidence: &CertifiedConicalBody,
    frame: &CertifiedRotatedFrameTransport,
) -> Result<ValidationReport, ResolutionRejected> {
    validate_member(member, resolution, || {
        let canonical = transport::strip_member(member, frame)?;
        evidence.rebind_member(&canonical)
    })
}

fn validate_member(
    member: &ConicalNativeMember,
    resolution: &Resolution,
    rebind: impl FnOnce() -> Result<(), ConicalBindingError>,
) -> Result<ValidationReport, ResolutionRejected> {
    Ok(match begin_body_validation(&member.shape, resolution)? {
        BodyGeometryValidation::Stopped(report) => report,
        BodyGeometryValidation::Ready(ordered) => {
            ordered.finish_ordered(|stages| certified_stage(stages, member, rebind))
        }
    })
}
fn certified_stage(
    stages: &mut Stages<'_>,
    member: &ConicalNativeMember,
    rebind: impl FnOnce() -> Result<(), ConicalBindingError>,
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
