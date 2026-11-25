//! Native observations stay separate from geometric receipts and keep their original order (#527).
use super::{
    DevelopedRegionRequest, Failure, Refusal, Shape, Stage, ValidationReport, at, estimate,
    ordered_checks, resolution,
};
use martensite_tkbo::body_check::{
    CertifiedConicalBody, ConicalNativeMember, validate_body_with_certified_conical,
};

pub(in super::super) struct NativeQualification {
    pub report: ValidationReport,
    pub volume_estimate: f64,
    pub integration_estimate: f64,
}

pub(in super::super) fn qualify_native(
    request: &DevelopedRegionRequest,
    shape: &Shape,
) -> Result<NativeQualification, Refusal> {
    let report = at(
        request.region,
        Stage::Validation,
        ordered_checks(shape, request.tolerance),
    )?;
    let (volume_estimate, integration_estimate) =
        at(request.region, Stage::Measure, estimate(shape))?;
    Ok(NativeQualification {
        report,
        volume_estimate,
        integration_estimate,
    })
}

/// Keeps ordered native observations and consumes conical authority at the SI stage.
/// Example: `qualify_native_with_certified_conical(request, member, evidence)?`.
pub(in super::super) fn qualify_native_with_certified_conical(
    request: &DevelopedRegionRequest,
    member: &ConicalNativeMember,
    evidence: &CertifiedConicalBody,
) -> Result<NativeQualification, Refusal> {
    let report = at(
        request.region,
        Stage::Validation,
        certified_conical_checks(member, request.tolerance, evidence),
    )?;
    let (volume_estimate, integration_estimate) =
        at(request.region, Stage::Measure, estimate(&member.shape))?;
    Ok(NativeQualification {
        report,
        volume_estimate,
        integration_estimate,
    })
}

fn certified_conical_checks(
    member: &ConicalNativeMember,
    tolerance: f64,
    evidence: &CertifiedConicalBody,
) -> Result<ValidationReport, Failure> {
    validate_body_with_certified_conical(member, &resolution(tolerance), evidence).map_err(|e| {
        Failure::new(&format!(
            "{}={} must be finite and positive",
            e.field, e.value
        ))
    })
}

#[cfg(test)]
mod tests;
