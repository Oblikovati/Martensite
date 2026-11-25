// SPDX-License-Identifier: GPL-2.0-only
//! Bind the original rational source roots to the detached, certified cutter.
use super::*;

type Completed = CurvedBandResult<RationalWrappedResult>;

/// Cut a rational quadratic aperture through a full cylindrical band and its periodic seam.
/// Accepts one closed boundary of explicit lines and open, clamped positive-rational quadratics.
/// The reference level controls exact dyadic subdivision of the original curve parameter.
/// Example: `let cut = normal_extrude_cylinder_band_with_rational_hole(&band, &profile, options, 3)?;`.
pub fn normal_extrude_cylinder_band_with_rational_hole(
    band: &FullTurnBand,
    request: &WrappedNormalRequest,
    options: NormalExtrusionOptions,
    reference_level: u8,
) -> Result<Completed, SurfaceSketchFailure> {
    let plan = AperturePlan::admit(
        band,
        request,
        options,
        Scope::OneCrossed,
        SourceModel::Rational,
    )
    .map_err(|cause| admission_failure(band, cause))?;
    let tool = normal_extrude_rational_cylinder(request, reference_level, || false)
        .map_err(|cause| rational_failure(&plan, cause))?;
    finish_rational(plan, tool)
}

/// Cut a rational quadratic aperture through a full conical band and its periodic seam.
/// Accepts one closed boundary of explicit lines and open, clamped positive-rational quadratics
/// crossing the zero ray of a positive-X developed chart.
/// Example: `let cut = normal_extrude_cone_band_with_rational_hole(&band, &profile, options, 3)?;`.
pub fn normal_extrude_cone_band_with_rational_hole(
    band: &FullTurnBand,
    request: &WrappedConeNormalRequest,
    options: NormalExtrusionOptions,
    reference_level: u8,
) -> Result<Completed, SurfaceSketchFailure> {
    let plan = AperturePlan::admit_cone(
        band,
        request,
        options,
        Scope::OneCrossed,
        SourceModel::Rational,
    )
    .map_err(|cause| admission_failure(band, cause))?;
    let tool = normal_extrude_rational_cone(request, reference_level, || false)
        .map_err(|cause| rational_failure(&plan, cause))?;
    finish_rational(plan, tool)
}

fn finish_rational(
    plan: AperturePlan,
    tool: RationalWrappedResult,
) -> Result<Completed, SurfaceSketchFailure> {
    let angular_error = angle_from_chord(plan.minimum_radius(), tool.spatial_error().upper)
        .map_err(|cause| plan.stock.sweep.failure(SurfaceStage::Validation, cause))?;
    let evidence = FitEvidence {
        authored_volume: tool.authored_volume(),
        angular_error,
    };
    let detached = tool
        .export_native()
        .map_err(|cause| export_failure(&plan, cause))?;
    finish(plan, tool, evidence, &detached)
}

fn rational_failure(plan: &AperturePlan, cause: RationalWrappedFailure) -> SurfaceSketchFailure {
    plan.stock.sweep.failure(
        SurfaceStage::Construction,
        incomplete("certified rational wrapped aperture", cause.to_string()),
    )
}
