//! Native observations stay separate from geometric receipts and keep their original order (#527).
use super::{
    DevelopedRegionRequest, Refusal, Shape, Stage, ValidationReport, at, estimate, ordered_checks,
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

#[cfg(test)]
mod tests;
