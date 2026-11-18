// SPDX-License-Identifier: GPL-2.0-only
//! Composition of previously certified radial and angular error bounds.
use super::angle::*;
use super::spatial::*;
use super::*;
use crate::predicates::roots::certificate_arithmetic::bernstein_curve::derivative;

pub(super) struct FitBounds {
    pub(super) radius: Interval,
    pub(super) radial_rate: Interval,
    pub(super) angular_rate: Interval,
}

pub(super) fn fit_bounds_captured(
    request: &SpatialRequest<'_>,
    a: &[Interval],
    w: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<FitBounds, SpatialFailure> {
    let minimum = slant::positive_lower("weight", w, arithmetic).map_err(SpatialFailure::Slant)?;
    let radius = arithmetic.divide(&magnitude_interval(a), &minimum)?;
    let rate =
        super::slant_derivative::fit_rate(a, w, arithmetic).map_err(SpatialFailure::Slant)?;
    let denominator = arithmetic.multiply(&minimum, &minimum)?;
    let radial_rate = arithmetic.divide(&magnitude_interval(&rate), &denominator)?;
    let angle =
        angular::capture_fit(request.angular_fit, arithmetic).map_err(SpatialFailure::Angular)?;
    let rate = derivative(&angle, arithmetic)?;
    // The physical rate is 2t'/(1+t²); developing the cone scales it by sin(alpha) <= 1.
    let two = arithmetic.scalar(2.)?;
    let angular_rate = arithmetic.multiply(&magnitude_interval(&rate), &two)?;
    Ok(FitBounds {
        radius,
        radial_rate,
        angular_rate,
    })
}

pub(super) struct Bounds {
    pub(super) radial_error: Interval,
    pub(super) radial_rate_error: Interval,
    pub(super) angular_error: Interval,
    pub(super) angular_rate_error: Interval,
    pub(super) fit_radius: Interval,
    pub(super) fit_radial_rate: Interval,
    pub(super) fit_angular_rate: Interval,
}

// The developed polar angle scales the physical angle by sin(alpha) <= 1.
#[derive(Debug)]
pub(super) enum CombinedFailure {
    Spatial(SpatialFailure),
    UnalignedAnchor,
    MissingOriginal,
    JoinMismatch,
    DirectionUnresolved,
}
impl From<BoundedArithmeticFailure> for CombinedFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Spatial(SpatialFailure::Arithmetic(cause))
    }
}

pub(super) fn authored_endpoint(
    request: &SpatialRequest<'_>,
    at_end: bool,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[Interval; 2], CombinedFailure> {
    let original = request.original.ok_or(CombinedFailure::MissingOriginal)?;
    super::original::admit_interval(original)
        .map_err(|cause| CombinedFailure::Spatial(SpatialFailure::Original(cause)))?;
    let source = admit_source(original.poles, arithmetic)
        .map_err(|cause| CombinedFailure::Spatial(SpatialFailure::Source(cause)))?;
    let mut result = [arithmetic.zero(), arithmetic.zero()];
    for (axis, value) in result.iter_mut().enumerate() {
        let span = crate::predicates::roots::certificate_arithmetic::bernstein_curve::restrict(
            &source[axis],
            original.index,
            original.level,
            arithmetic,
        )?;
        *value = span[if at_end { span.len() - 1 } else { 0 }].clone();
    }
    Ok(result)
}

pub(super) fn authored_join(
    first: &SpatialRequest<'_>,
    second: &SpatialRequest<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<bool, CombinedFailure> {
    let end = authored_endpoint(first, true, arithmetic)?;
    let start = authored_endpoint(second, false, arithmetic)?;
    Ok(end
        .iter()
        .zip(start)
        .all(|(a, b)| a.lower == a.upper && b.lower == b.upper && a.lower == b.lower))
}

pub(super) fn fit_join(
    first: &SpatialRequest<'_>,
    second: &SpatialRequest<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<bool, SpatialFailure> {
    admit_components(first)?;
    admit_components(second)?;
    if first.anchor.semi_angle != second.anchor.semi_angle
        || first.anchor.origin != second.anchor.origin
        || first.anchor.fit_center != second.anchor.fit_center
        || first.depths != second.depths
        || first.outward != second.outward
        || first.angular_fit[first.angular_fit.len() - 1] != second.angular_fit[0]
    {
        return Ok(false);
    }
    slant_fit_join(first, second, arithmetic)
}

pub(super) fn source_derivatives(
    source: &[Vec<Interval>; 2],
    original: ConicalOriginalSpan<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<[Interval; 2]>, CombinedFailure> {
    let mut axes = Vec::new();
    for axis in source {
        let span = crate::predicates::roots::certificate_arithmetic::bernstein_curve::restrict(
            axis,
            original.index,
            original.level,
            arithmetic,
        )
        .map_err(|cause| CombinedFailure::Spatial(SpatialFailure::Arithmetic(cause)))?;
        axes.push(
            derivative(&span, arithmetic)
                .map_err(|cause| CombinedFailure::Spatial(SpatialFailure::Arithmetic(cause)))?,
        );
    }
    Ok(
        crate::predicates::roots::certificate_arithmetic::bernstein_curve::align_planar(
            &axes[0], &axes[1],
        ),
    )
}

pub(super) fn directional_margin(
    poles: &[[Interval; 2]],
    direction: [f64; 2],
    error: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<bool, CombinedFailure> {
    let x = arithmetic.scalar(direction[0])?;
    let y = arithmetic.scalar(direction[1])?;
    let norm = arithmetic.add(
        &magnitude_interval(std::slice::from_ref(&x)),
        &magnitude_interval(std::slice::from_ref(&y)),
    )?;
    let tolerance = arithmetic.multiply(&norm, error)?;
    for pole in poles {
        let first = arithmetic.multiply(&pole[0], &x)?;
        let second = arithmetic.multiply(&pole[1], &y)?;
        let dot = arithmetic.add(&first, &second)?;
        if dot.lower <= tolerance.upper {
            return Ok(false);
        }
    }
    Ok(true)
}

pub(super) struct PreparedProjection<'a> {
    request: &'a SpatialRequest<'a>,
    poles: Vec<[Interval; 2]>,
    error: Interval,
}

pub(super) fn prepare_projection<'a>(
    request: &'a SpatialRequest<'a>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<PreparedProjection<'a>, CombinedFailure> {
    let original = request.original.ok_or(CombinedFailure::MissingOriginal)?;
    let bounds = source_bounds(request, arithmetic)?;
    let error = derivative_error(&bounds, arithmetic)?;
    let source = admit_source(original.poles, arithmetic)
        .map_err(|cause| CombinedFailure::Spatial(SpatialFailure::Source(cause)))?;
    let poles = source_derivatives(&source, original, arithmetic)?;
    Ok(PreparedProjection {
        request,
        poles,
        error,
    })
}

pub(super) fn prepared_join_direction(
    first: &PreparedProjection<'_>,
    second: &PreparedProjection<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[f64; 2], CombinedFailure> {
    if !authored_join(first.request, second.request, arithmetic)?
        || !fit_join(first.request, second.request, arithmetic).map_err(CombinedFailure::Spatial)?
    {
        return Err(CombinedFailure::JoinMismatch);
    }
    let derivatives = [first.poles.clone(), second.poles.clone()];
    for direction in
        crate::predicates::roots::certificate_arithmetic::planar_hulls::candidate_directions(
            &derivatives,
            arithmetic.fractional_bits(),
        )
    {
        if directional_margin(&first.poles, direction, &first.error, arithmetic)?
            && directional_margin(&second.poles, direction, &second.error, arithmetic)?
        {
            return Ok(direction);
        }
    }
    Err(CombinedFailure::DirectionUnresolved)
}

pub(super) fn source_bounds(
    request: &SpatialRequest<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Bounds, CombinedFailure> {
    let (spatial, capture) =
        certify_spatial_captured(request, arithmetic).map_err(CombinedFailure::Spatial)?;
    let comparison = capture.comparison;
    let view = ProjectionView {
        source: &capture.source,
        q: &capture.q,
        numerator: &capture.numerator,
        weights: &capture.weights,
        slant: &capture.slant,
        angular: &capture.angular,
    };
    bounds_from_view(
        request,
        &spatial,
        view,
        comparison,
        &capture.sine,
        arithmetic,
    )
}

pub(super) struct ProjectionView<'a> {
    pub(super) source: &'a [Vec<Interval>; 2],
    pub(super) q: &'a [Interval],
    pub(super) numerator: &'a [Interval],
    pub(super) weights: &'a [Interval],
    pub(super) slant: &'a Interval,
    pub(super) angular: &'a Interval,
}

pub(super) fn bounds_from_view(
    request: &SpatialRequest<'_>,
    spatial: &SpatialEvidence,
    view: ProjectionView<'_>,
    comparison: Option<super::original::OriginalComparison>,
    sine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Bounds, CombinedFailure> {
    let fit_rate = radial_derivative_bound(view.q, view.numerator, view.weights, arithmetic)
        .map_err(SpatialFailure::Slant)
        .map_err(CombinedFailure::Spatial)?;
    let fit = fit_bounds_captured(request, view.numerator, view.weights, arithmetic)
        .map_err(CombinedFailure::Spatial)?;
    let (radial, angular, original_rate) =
        comparison_bounds(comparison, view.source, sine, arithmetic)?;
    let radial_rate_error = arithmetic.add(&fit_rate, &original_rate)?;
    assemble_bounds(
        spatial,
        radial_rate_error,
        fit,
        (radial, angular),
        (view.slant, view.angular),
        arithmetic,
    )
}

pub(super) fn add_bound(
    a: &Interval,
    b: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, CombinedFailure> {
    arithmetic
        .add(a, b)
        .map_err(|cause| CombinedFailure::Spatial(SpatialFailure::Arithmetic(cause)))
}

pub(super) fn assemble_bounds(
    spatial: &SpatialEvidence,
    radial_rate_error: Interval,
    fit: FitBounds,
    original: (Interval, Interval),
    fit_errors: (&Interval, &Interval),
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Bounds, CombinedFailure> {
    let (radial, angular) = original;
    Ok(Bounds {
        radial_error: add_bound(&radial, fit_errors.0, arithmetic)?,
        radial_rate_error,
        angular_error: add_bound(&angular, fit_errors.1, arithmetic)?,
        angular_rate_error: add_bound(
            &spatial.angular_derivative,
            &spatial.original_angular_derivative,
            arithmetic,
        )?,
        fit_radius: fit.radius,
        fit_radial_rate: fit.radial_rate,
        fit_angular_rate: fit.angular_rate,
    })
}

pub(super) fn derivative_error(
    bounds: &Bounds,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    // The unit polar frame is 1-Lipschitz in angle, including its perpendicular.
    let turn_speed = arithmetic.add(&bounds.fit_angular_rate, &bounds.angular_rate_error)?;
    let radius_term = arithmetic.multiply(&bounds.radial_error, &turn_speed)?;
    let rotation = arithmetic.multiply(&bounds.fit_radius, &bounds.fit_angular_rate)?;
    let rotation = arithmetic.add(&rotation, &bounds.fit_radial_rate)?;
    let rotation = arithmetic.multiply(&rotation, &bounds.angular_error)?;
    let rate = arithmetic.multiply(&bounds.fit_radius, &bounds.angular_rate_error)?;
    let result = arithmetic.add(&bounds.radial_rate_error, &radius_term)?;
    let result = arithmetic.add(&result, &rotation)?;
    arithmetic.add(&result, &rate)
}

fn slant_fit_join(
    first: &SpatialRequest<'_>,
    second: &SpatialRequest<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<bool, SpatialFailure> {
    let a = slant::capture(first.slant_numerator, arithmetic).map_err(SpatialFailure::Slant)?;
    let b = slant::capture(second.slant_numerator, arithmetic).map_err(SpatialFailure::Slant)?;
    let aw = slant::capture(first.weights, arithmetic).map_err(SpatialFailure::Slant)?;
    let bw = slant::capture(second.weights, arithmetic).map_err(SpatialFailure::Slant)?;
    slant::positive_lower("weight", &aw, arithmetic).map_err(SpatialFailure::Slant)?;
    slant::positive_lower("weight", &bw, arithmetic).map_err(SpatialFailure::Slant)?;
    let left = arithmetic.multiply(&a[a.len() - 1], &bw[0])?;
    let right = arithmetic.multiply(&b[0], &aw[aw.len() - 1])?;
    Ok(left.lower == left.upper && right.lower == right.upper && left.lower == right.lower)
}

fn comparison_bounds(
    comparison: Option<super::original::OriginalComparison>,
    source: &[Vec<Interval>; 2],
    sine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(Interval, Interval, Interval), CombinedFailure> {
    let (radial, angular, original_rate) = match comparison {
        Some(comparison) => {
            let (radial, angular, rate, anchor) =
                super::original::polar_from_comparison(comparison, source, sine, arithmetic)
                    .map_err(|cause| CombinedFailure::Spatial(SpatialFailure::Original(cause)))?;
            if anchor.turn_adjustment != Some(0) {
                return Err(CombinedFailure::UnalignedAnchor);
            }
            (radial, angular, rate)
        }
        None => (arithmetic.zero(), arithmetic.zero(), arithmetic.zero()),
    };
    Ok((radial, angular, original_rate))
}

use super::slant_derivative::radial_derivative_bound;
