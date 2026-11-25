//! Ordered validation of the decoded, authoritative publication body.
use super::{
    failure::*,
    history::Bindings,
    result::SurfaceDiagnostic,
    sweep::{Sweep, SweepNames},
    tolerance::*,
};
use martensite_tkbo::{
    body_check::{ValidationReport, validate_body},
    profile_extrusion::{AchievedTolerance, ToleranceCap},
};
use martensite_tkbrep::topods;

pub(super) struct CheckedBody {
    pub report: ValidationReport,
    pub achieved: AchievedTolerance,
    pub diagnostics: Vec<SurfaceDiagnostic>,
}
pub(super) fn check_body(
    solid: &topods::Shape,
    bindings: &Bindings,
    admitted: &Sweep,
    semantic_check: &impl Fn(&Bindings) -> Checked<()>,
) -> Checked<CheckedBody> {
    super::history::validate_sweep_membership(admitted, solid, bindings)?;
    semantic_check(bindings)?;
    let report = ordered_report(solid, admitted)?;
    if matches!(admitted.names, SweepNames::FullTurn { .. }) {
        verify_band_census(&report)?;
    }
    let achieved = tolerances(bindings)?;
    let diagnostics = cap_diagnostics(achieved, admitted.options.resolution.cap)?;
    Ok(CheckedBody {
        report,
        achieved,
        diagnostics,
    })
}

fn verify_band_census(report: &ValidationReport) -> Checked<()> {
    if matches!(report.census.as_slice(), [body] if body.shells == 1 && body.genus == Some(1)) {
        return Ok(());
    }
    Err(incomplete(
        "one genus-one shell in a hole-free full-turn band",
        format!("{:?}", report.census),
    ))
}
fn ordered_report(solid: &topods::Shape, admitted: &Sweep) -> Checked<ValidationReport> {
    let mut resolution = admitted.options.resolution;
    if matches!(resolution.cap, ToleranceCap::Report(_)) {
        resolution.cap = ToleranceCap::NoLimit;
    }
    let report = validate_body(solid, &resolution).map_err(|error| {
        incomplete(
            "validated resolution",
            format!("{}={}", error.field, error.value),
        )
    })?;
    if !report.is_valid() {
        return Err(SurfaceFailureCause::BodyValidation(Box::new(report)));
    }
    Ok(report)
}
