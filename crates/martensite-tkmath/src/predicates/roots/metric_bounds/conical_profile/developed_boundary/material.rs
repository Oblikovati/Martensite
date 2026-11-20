// SPDX-License-Identifier: GPL-2.0-only
//! A polar triangle uses actual UV errors; both filled domains retain the same halfplane.
use super::super::super::cylindrical_profile::algebra;
use super::super::super::{
    ConicalDevelopedMaterialFailure, ConicalMaterialQuery, ConicalMaterialSpan,
};
use super::*;
use crate::predicates::roots::certificate_arithmetic::bernstein_bounds as polynomial;
mod binding;
use binding::compare;

#[cfg(test)]
pub(in crate::predicates::roots::metric_bounds) fn validate(
    query: &ConicalMaterialQuery<'_>,
    span: &ConicalMaterialSpan<'_>,
    context: &ConicalDevelopedBoundarySpan<'_>,
    domain: &ConicalChartDomainEvidence,
) -> Result<(), ConicalDevelopedMaterialFailure> {
    validate_with_refinement(
        query,
        span,
        context,
        domain,
        ConicalPrimitiveRefinement::UnsplitV1,
    )
}
pub(in crate::predicates::roots::metric_bounds) fn validate_with_refinement(
    query: &ConicalMaterialQuery<'_>,
    span: &ConicalMaterialSpan<'_>,
    context: &ConicalDevelopedBoundarySpan<'_>,
    domain: &ConicalChartDomainEvidence,
    refinement: ConicalPrimitiveRefinement,
) -> Result<(), ConicalDevelopedMaterialFailure> {
    let issued = context.evidence;
    compare(
        "actual reconstructed boundary source",
        BoundarySnapshot::capture(&context.source).bits,
        issued.source.bits.clone(),
    )?;
    compare(
        "actual issued boundary source",
        context.boundary.input.bits.clone(),
        issued.source.bits.clone(),
    )?;
    if !issued.domain.matches_selected(domain, refinement) {
        return compare(
            "actual developed chart authority",
            ChartBinding::capture_selected(domain, refinement).diagnostic_bits(),
            issued.domain.diagnostic_bits(),
        );
    }
    binding::boundary_authority(context, domain, refinement)?;
    binding::material_operands(query, span, context)?;
    Ok(())
}
#[cfg(test)]
pub(in crate::predicates::roots::metric_bounds) fn span_bounds(
    query: &ConicalMaterialQuery<'_>,
    span: &ConicalMaterialSpan<'_>,
    context: &ConicalDevelopedBoundarySpan<'_>,
    domain: &ConicalChartDomainEvidence,
    half_pi: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<([Interval; 2], Interval, Interval), CylindricalProfileFailure> {
    span_bounds_with_refinement(
        query,
        span,
        context,
        domain,
        half_pi,
        arithmetic,
        ConicalPrimitiveRefinement::UnsplitV1,
    )
}
pub(in crate::predicates::roots::metric_bounds) fn span_bounds_with_refinement(
    query: &ConicalMaterialQuery<'_>,
    _span: &ConicalMaterialSpan<'_>,
    context: &ConicalDevelopedBoundarySpan<'_>,
    domain: &ConicalChartDomainEvidence,
    half_pi: &Interval,
    arithmetic: &mut Arithmetic<'_>,
    refinement: ConicalPrimitiveRefinement,
) -> Result<([Interval; 2], Interval, Interval), CylindricalProfileFailure> {
    let retained = checked_retained(context, domain, arithmetic.fractional_bits(), refinement)?;
    let operands = &context.evidence.operands;
    let delta = displacement(
        query.reference_slant,
        operands,
        retained,
        &domain.sine,
        arithmetic,
    )?;
    let minimum = original_minimum(&retained.capture)?;
    // The corrected minimum contains both boundary images and every straight homotopy segment.
    let halfplane = arithmetic.subtract(&minimum, &delta)?;
    principal_phase(query, &operands.uv[0], &domain.sine, half_pi, arithmetic)?;
    Ok((operands.uv.clone(), halfplane, delta))
}
fn displacement(
    reference: f64,
    operands: &RetainedOperands,
    retained: &spatial::RetainedSpatial,
    sine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, CylindricalProfileFailure> {
    // The Euclidean polar triangle uses Lfit*sin(alpha), independently of signed normal depth.
    let weights = &retained.capture.weights;
    let lower = polynomial::lower(weights);
    let minimum = Interval {
        lower: lower.clone(),
        upper: lower,
    };
    let reference = arithmetic.scalar(reference)?;
    let shift = polynomial::scale(weights, &reference, arithmetic)?;
    let residual = algebra::sum(&operands.delta, &shift, arithmetic)?;
    let radial = arithmetic.divide(&algebra::magnitude(&residual), &minimum)?;
    let radius = arithmetic.divide(&algebra::magnitude(&retained.capture.numerator), &minimum)?;
    let radius = arithmetic.multiply(&radius, sine)?;
    let angular = arithmetic.multiply(&radius, &operands.angular)?;
    let source = arithmetic.add(&retained.evidence.bound, &retained.evidence.original)?;
    let error = arithmetic.add(&radial, &angular)?;
    Ok(arithmetic.add(&source, &error)?)
}
fn original_controls(
    capture: &spatial::SpatialCapture,
) -> Result<&[Vec<Interval>; 2], CylindricalProfileFailure> {
    match &capture.comparison {
        Some(comparison) => {
            comparison
                .restricted
                .as_ref()
                .ok_or(CylindricalProfileFailure::Count {
                    role: "one actual original restricted source capture",
                    actual: 0,
                })
        }
        None => Ok(&capture.source),
    }
}
fn principal_phase(
    query: &ConicalMaterialQuery<'_>,
    u: &Interval,
    sine: &Interval,
    half_pi: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), CylindricalProfileFailure> {
    let origin = arithmetic.scalar(query.chart_origin)?;
    let phase = arithmetic.subtract(u, &origin)?;
    let phase = arithmetic.multiply(&phase, sine)?;
    if phase.lower <= -half_pi.lower.clone() || phase.upper >= half_pi.lower {
        return Err(CylindricalProfileFailure::Value {
            role: "actual developed UV phase strictly in (-pi/2,pi/2)",
            index: 0,
            value: arithmetic.upper_f64(phase.upper)?,
        });
    }
    Ok(())
}

fn original_minimum(
    capture: &spatial::SpatialCapture,
) -> Result<Interval, CylindricalProfileFailure> {
    let original = original_controls(capture)?;
    let minimum = polynomial::lower(&original[0]);
    Ok(Interval {
        lower: minimum.clone(),
        upper: minimum,
    })
}

pub(in crate::predicates::roots::metric_bounds) fn half_pi(
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, CylindricalProfileFailure> {
    let pi = crate::predicates::roots::certificate_arithmetic::circular_constants::pi(arithmetic)?;
    Ok(arithmetic.fraction(&pi, 1, 2)?)
}

fn checked_retained<'a>(
    context: &'a ConicalDevelopedBoundarySpan<'_>,
    domain: &ConicalChartDomainEvidence,
    fractional_bits: u16,
    refinement: ConicalPrimitiveRefinement,
) -> Result<&'a spatial::RetainedSpatial, CylindricalProfileFailure> {
    context
        .boundary
        .retained_in_chart_with_refinement(&context.source, domain, fractional_bits, refinement)
        .ok_or(CylindricalProfileFailure::Count {
            role: "one retained actual chart spatial capture",
            actual: 0,
        })
}
