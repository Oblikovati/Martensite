// SPDX-License-Identifier: GPL-2.0-only
//! The triangle inequality composes actual carrier error with decoded rational storage.
use super::super::super::cylindrical_profile::algebra;
use super::*;
use crate::predicates::roots::certificate_arithmetic::{
    bernstein_bounds as polynomial, bernstein_curve::derivative,
};
mod common;
mod meridian;
pub(super) use common::CommonTrace;
pub(super) mod phase;
pub(super) struct Bridge<'a, 'q> {
    pub(super) source: ConicalBoundaryQuery<'q>,
    pub(super) boundary: &'a ConicalBoundaryEvidence,
    pub(super) domain: &'a ConicalChartDomainEvidence,
    pub(super) capture: &'a spatial::SpatialCapture,
    pub(super) fit_phase: &'a Interval,
    pub(super) refinement: ConicalPrimitiveRefinement,
}
impl<'a, 'q> Bridge<'a, 'q> {
    pub(super) fn new(
        source: ConicalBoundaryQuery<'q>,
        boundary: &'a ConicalBoundaryEvidence,
        domain: &'a ConicalChartDomainEvidence,
        captured: (&'a spatial::SpatialCapture, &'a Interval),
        refinement: ConicalPrimitiveRefinement,
    ) -> Self {
        Self {
            source,
            boundary,
            domain,
            capture: captured.0,
            fit_phase: captured.1,
            refinement,
        }
    }
}
pub(super) fn residual<'a>(
    query: ConicalTrimQuery<'a>,
    depth: usize,
    input: &Bridge<'_, '_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<([f64; 3], CommonTrace<'a>), CylindricalProfileFailure> {
    let uv = capture_planar(query.pcurve, arithmetic)?;
    let angular = phase::bound_selected(
        &uv[0],
        input.source.angular_fit,
        input.fit_phase,
        input.refinement,
        arithmetic,
    )?;
    let radius = meridian::native_radius(query, &uv[1], input, arithmetic)?;
    let delta = meridian::meridian_delta(&uv[1], input, arithmetic)?;
    let common = CommonTrace::new(query.pcurve, uv, angular, delta);
    let bounds = complete(query, depth, input, &common, &radius, arithmetic)?;
    Ok((bounds, common))
}
pub(super) fn second_residual(
    query: ConicalTrimQuery<'_>,
    depth: usize,
    input: &Bridge<'_, '_>,
    common: &CommonTrace<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<([f64; 3], ()), CylindricalProfileFailure> {
    if !common.matches(query.pcurve) {
        return residual(query, depth, input, arithmetic).map(|(bounds, _)| (bounds, ()));
    }
    let radius = meridian::native_radius(query, &common.uv[1], input, arithmetic)?;
    complete(query, depth, input, common, &radius, arithmetic).map(|bounds| (bounds, ()))
}
fn complete(
    query: ConicalTrimQuery<'_>,
    depth: usize,
    input: &Bridge<'_, '_>,
    common: &CommonTrace<'_>,
    radius: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[f64; 3], CylindricalProfileFailure> {
    let [radial, axial, radius] =
        meridian::bounds(query, depth, &common.delta, radius, input, arithmetic)?;
    let storage = captured_storage(&input.boundary.storage()[depth], arithmetic)?;
    compose(
        radial,
        axial,
        radius,
        common.angular.clone(),
        storage,
        input.domain.version(),
        arithmetic,
    )
}

fn compose(
    radial: Interval,
    axial: Interval,
    radius: Interval,
    angular: Interval,
    storage: Interval,
    version: ConicalChartDomainVersion,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[f64; 3], CylindricalProfileFailure> {
    let radial_l1 = xy_l1_upper(&radial, arithmetic)?;
    let angular = arithmetic.multiply(&radius, &angular)?;
    let angular_l1 = xy_l1_upper(&angular, arithmetic)?;
    let xy = arithmetic.add(&radial_l1, &angular_l1)?;
    let xy = arithmetic.add(&xy, &storage)?;
    let z = arithmetic.add(&axial, &storage)?;
    let total = position_bound(version, &xy, &z, &radial, &angular, arithmetic)?;
    Ok([
        arithmetic.upper_f64(xy.upper)?,
        arithmetic.upper_f64(z.upper)?,
        arithmetic.upper_f64(total.upper)?,
    ])
}

fn position_bound(
    version: ConicalChartDomainVersion,
    xy_l1: &Interval,
    axial_and_storage: &Interval,
    radial: &Interval,
    angular: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    match version {
        ConicalChartDomainVersion::OriginalV2 => arithmetic.add(xy_l1, axial_and_storage),
        ConicalChartDomainVersion::FamilyV3 | ConicalChartDomainVersion::RecenteredV4 => {
            // Changing radius contributes its absolute difference. At fixed radius the
            // chord is at most radius times angle. Both facts hold in Euclidean norm.
            // The storage L1 bound also bounds Euclidean storage error.
            let xy = arithmetic.add(radial, angular)?;
            arithmetic.add(&xy, axial_and_storage)
        }
    }
}

#[cfg(test)]
mod tests;

pub(in super::super) fn captured_storage(
    value: &DyadicBound,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let (numerator, denominator) = value.ratio();
    let bits = usize::from(arithmetic.fractional_bits());
    let numerator = numerator << bits;
    let denominator = denominator << bits;
    let numerator = Interval {
        lower: numerator.clone(),
        upper: numerator,
    };
    let denominator = Interval {
        lower: denominator.clone(),
        upper: denominator,
    };
    arithmetic.divide(&numerator, &denominator)
}
fn scaled(
    values: &[Interval],
    factor: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<Interval>, CylindricalProfileFailure> {
    Ok(polynomial::scale(values, factor, arithmetic)?)
}

// 46341² = 2147488281 > 2·32768² = 2147483648, so this dyadic exceeds sqrt(2).
// Apply the XY norm conversion before adding the independent storage or axial bounds.
const XY_L1_SCALE_NUMERATOR: u64 = 46_341;
const XY_L1_SCALE_DENOMINATOR: u64 = 32_768;
fn xy_l1_upper(value: &Interval, arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
    arithmetic.fraction(value, XY_L1_SCALE_NUMERATOR, XY_L1_SCALE_DENOMINATOR)
}
