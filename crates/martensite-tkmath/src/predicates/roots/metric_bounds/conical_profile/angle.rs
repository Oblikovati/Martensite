// SPDX-License-Identifier: GPL-2.0-only
//! Outward principal-angle bounds for an apex-relative binary64 endpoint.
use super::*;
use crate::predicates::roots::certificate_arithmetic::circular_constants;

#[derive(Debug)]
pub(super) enum InitialAngleFailure {
    InvalidPoint([f64; 2]),
    ReductionOutOfRange,
    InvalidSupport {
        semi_angle: f64,
        origin: f64,
    },
    InvalidFit {
        center: f64,
        half_angle_tangent: f64,
    },
    Trigonometric(crate::predicates::roots::metric_bounds::CylindricalProfileFailure),
    Arithmetic(BoundedArithmeticFailure),
}
impl std::fmt::Display for InitialAngleFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPoint(point) => write!(f, "point={point:?}; expected finite and nonzero"),
            Self::ReductionOutOfRange => write!(f, "angle reduction exceeded supported range"),
            Self::InvalidSupport { semi_angle, origin } => write!(
                f,
                "semi_angle={semi_angle}, origin={origin}; expected finite support and valid semi-angle"
            ),
            Self::InvalidFit {
                center,
                half_angle_tangent,
            } => write!(
                f,
                "center={center}, tangent={half_angle_tangent}; expected finite fit"
            ),
            Self::Trigonometric(cause) => write!(f, "trigonometric: {cause:?}"),
            Self::Arithmetic(cause) => write!(f, "arithmetic: {cause:?}"),
        }
    }
}
impl From<BoundedArithmeticFailure> for InitialAngleFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(cause)
    }
}
pub(super) type AngleResult<T> = Result<T, InitialAngleFailure>;

#[derive(Clone, Copy)]
pub(super) struct AnchorRequest {
    pub(super) point: [f64; 2],
    pub(super) semi_angle: f64,
    pub(super) origin: f64,
    pub(super) developed_turn: i32,
    pub(super) physical_turn: i32,
    pub(super) fit_center: f64,
    pub(super) fit_half_angle_tangent: f64,
}

pub(super) struct AnchorCapture {
    pub(super) error: Interval,
    pub(super) sine: Interval,
    pub(super) fit_phase: Interval,
}
pub(super) fn anchor_with_sine(
    request: &AnchorRequest,
    arithmetic: &mut Arithmetic<'_>,
) -> AngleResult<(Interval, Interval)> {
    anchor_metric(request, None, arithmetic).map(|proof| (proof.error, proof.sine))
}
#[cfg(test)]
pub(super) fn anchor_in_chart(
    request: &AnchorRequest,
    domain: &ConicalChartDomainEvidence,
    arithmetic: &mut Arithmetic<'_>,
) -> AngleResult<(Interval, Interval)> {
    anchor_capture_in_chart(request, domain, arithmetic).map(|proof| (proof.error, proof.sine))
}
pub(super) fn anchor_capture_in_chart(
    request: &AnchorRequest,
    domain: &ConicalChartDomainEvidence,
    arithmetic: &mut Arithmetic<'_>,
) -> AngleResult<AnchorCapture> {
    anchor_metric(request, Some(&domain.sine), arithmetic)
}
fn anchor_metric(
    request: &AnchorRequest,
    sine: Option<&Interval>,
    arithmetic: &mut Arithmetic<'_>,
) -> AngleResult<AnchorCapture> {
    if !request.fit_center.is_finite() || !request.fit_half_angle_tangent.is_finite() {
        return Err(InitialAngleFailure::InvalidFit {
            center: request.fit_center,
            half_angle_tangent: request.fit_half_angle_tangent,
        });
    }
    let (target, sine) = physical_anchor(request, sine, arithmetic)?;
    let target = shift_branch(target, request.physical_turn, arithmetic)?;
    let fit = fit_phase(request, arithmetic)?;
    let error = arithmetic.subtract(&target, &fit)?;
    Ok(AnchorCapture {
        error,
        sine,
        fit_phase: fit,
    })
}
fn physical_anchor(
    request: &AnchorRequest,
    sine: Option<&Interval>,
    arithmetic: &mut Arithmetic<'_>,
) -> AngleResult<(Interval, Interval)> {
    match sine {
        Some(sine) => physical_metric(
            request.point,
            request.semi_angle,
            request.origin,
            request.developed_turn,
            Some(sine),
            arithmetic,
        ),
        None => physical_with_sine(
            request.point,
            request.semi_angle,
            request.origin,
            request.developed_turn,
            arithmetic,
        ),
    }
}
pub(super) fn fit_phase(
    request: &AnchorRequest,
    arithmetic: &mut Arithmetic<'_>,
) -> AngleResult<Interval> {
    let fit = initial_angle([1., request.fit_half_angle_tangent], arithmetic)?;
    let fit = arithmetic.fraction(&fit, 2, 1)?;
    let center = arithmetic.scalar(request.fit_center)?;
    Ok(arithmetic.add(&fit, &center)?)
}

pub(super) fn shift_branch(
    angle: Interval,
    turns: i32,
    arithmetic: &mut Arithmetic<'_>,
) -> AngleResult<Interval> {
    if turns == 0 {
        return Ok(angle);
    }
    let pi = circular_constants::pi(arithmetic)?;
    let twice_turns = num_bigint::BigInt::from(turns) * 2;
    let offset = arithmetic.integer_scale(&pi, &twice_turns)?;
    Ok(arithmetic.add(&angle, &offset)?)
}

pub(super) fn physical_with_sine(
    point: [f64; 2],
    semi_angle: f64,
    origin: f64,
    developed_turn: i32,
    arithmetic: &mut Arithmetic<'_>,
) -> AngleResult<(Interval, Interval)> {
    physical_metric(point, semi_angle, origin, developed_turn, None, arithmetic)
}
fn physical_metric(
    point: [f64; 2],
    semi_angle: f64,
    origin: f64,
    developed_turn: i32,
    cached_sine: Option<&Interval>,
    arithmetic: &mut Arithmetic<'_>,
) -> AngleResult<(Interval, Interval)> {
    admit_support(semi_angle, origin)?;
    let developed = initial_angle(point, arithmetic)?;
    let developed = shift_branch(developed, developed_turn, arithmetic)?;
    let (scaled, sine) = match cached_sine {
        Some(sine) => (arithmetic.divide(&developed, sine)?, sine.clone()),
        None => scale_developed(&developed, semi_angle, arithmetic)?,
    };
    let chart_origin = arithmetic.scalar(origin)?;
    Ok((arithmetic.add(&scaled, &chart_origin)?, sine))
}

pub(super) fn admit_support(semi_angle: f64, origin: f64) -> AngleResult<()> {
    if !semi_angle.is_finite()
        || semi_angle <= 0.
        || semi_angle >= std::f64::consts::FRAC_PI_2
        || !origin.is_finite()
    {
        return Err(InitialAngleFailure::InvalidSupport { semi_angle, origin });
    }
    Ok(())
}
pub(super) fn scale_developed(
    developed: &Interval,
    semi_angle: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> AngleResult<(Interval, Interval)> {
    let alpha = arithmetic.scalar(semi_angle)?;
    let [_, sine] =
        crate::predicates::roots::metric_bounds::cylindrical_profile::trigonometric::enclose(
            &[alpha],
            arithmetic,
        )
        .map_err(InitialAngleFailure::Trigonometric)?;
    let scaled = arithmetic.divide(developed, &sine[0])?;
    Ok((scaled, sine[0].clone()))
}

pub(super) fn initial_angle(
    point: [f64; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> AngleResult<Interval> {
    if !point.iter().all(|value| value.is_finite()) || point == [0., 0.] {
        return Err(InitialAngleFailure::InvalidPoint(point));
    }
    let [x, y] = point;
    let [ax, ay] = [x.abs(), y.abs()];
    let swapped = ay > ax;
    let (small, large) = if swapped { (ax, ay) } else { (ay, ax) };
    let pi = circular_constants::pi(arithmetic)?;
    let acute = acute_angle(small, large, &pi, arithmetic)?;
    restore_quadrant(point, swapped, &pi, acute, arithmetic)
}
pub(super) fn restore_quadrant(
    [x, y]: [f64; 2],
    swapped: bool,
    pi: &Interval,
    acute: Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> AngleResult<Interval> {
    let first_quadrant = if swapped {
        let half = arithmetic.fraction(pi, 1, 2)?;
        arithmetic.subtract(&half, &acute)?
    } else {
        acute
    };
    let upper = if x < 0. {
        arithmetic.subtract(pi, &first_quadrant)?
    } else {
        first_quadrant
    };
    signed_angle(y, upper, arithmetic)
}
pub(super) fn signed_angle(
    y: f64,
    upper: Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> AngleResult<Interval> {
    if y < 0. {
        return Ok(arithmetic.subtract(&arithmetic.zero(), &upper)?);
    }
    Ok(upper)
}

pub(super) fn acute_angle(
    small: f64,
    large: f64,
    pi: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> AngleResult<Interval> {
    let small_interval = arithmetic.scalar(small)?;
    let large_interval = arithmetic.scalar(large)?;
    let ratio = arithmetic.divide(&small_interval, &large_interval)?;
    if small <= large * 0.5 {
        return atan_series(&ratio, arithmetic);
    }
    let reduced = reduce_ratio(&ratio, arithmetic)?;
    let correction = atan_series(&reduced, arithmetic)?;
    let quarter = arithmetic.fraction(pi, 1, 4)?;
    Ok(arithmetic.add(&quarter, &correction)?)
}

pub(super) fn reduce_ratio(
    ratio: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> AngleResult<Interval> {
    let one = arithmetic.scalar(1.)?;
    let numerator = arithmetic.subtract(ratio, &one)?;
    let denominator = arithmetic.add(ratio, &one)?;
    Ok(arithmetic.divide(&numerator, &denominator)?)
}

pub(super) fn atan_series(
    argument: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> AngleResult<Interval> {
    let one = arithmetic.scalar(1.)?;
    let two_thirds = arithmetic.fraction(&one, 2, 3)?;
    if argument.magnitude() > two_thirds.lower {
        return Err(InitialAngleFailure::ReductionOutOfRange);
    }
    let (sum, power) = series_sum(argument, arithmetic)?;
    let tail = power.magnitude();
    Ok(arithmetic.add(
        &sum,
        &Interval {
            lower: -tail.clone(),
            upper: tail,
        },
    )?)
}
pub(super) fn series_sum(
    argument: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> AngleResult<(Interval, Interval)> {
    let square = arithmetic.multiply(argument, argument)?;
    let mut power = argument.clone();
    let mut sum = arithmetic.zero();
    let terms = u64::from(arithmetic.fractional_bits()) / 2 + 4;
    for k in 0..terms {
        let term = arithmetic.fraction(&power, 1, 2 * k + 1)?;
        sum = if k % 2 == 0 {
            arithmetic.add(&sum, &term)?
        } else {
            arithmetic.subtract(&sum, &term)?
        };
        power = arithmetic.multiply(&power, &square)?;
    }
    Ok((sum, power))
}

pub use super::original::ConicalOriginalFailure;
pub use super::original::ConicalOriginalSpan;

pub use super::boundary::ConicalBoundaryError;
pub use super::boundary::ConicalBoundaryEvidence;
pub use super::boundary::ConicalBoundaryFailure;
pub use super::boundary::ConicalBoundaryFailureKind;
pub use super::boundary::ConicalBoundaryProofFailure;
pub use super::boundary::ConicalBoundaryQuery;
pub use super::boundary::ConicalBoundaryStage;
pub use super::boundary::ConicalJoinFailureKind;
pub use super::boundary::ConicalJoinProofFailure;
pub use super::boundary::ConicalJoinSpan;
pub use super::boundary::ConicalJoinsEvidence;
pub use super::boundary::ConicalJoinsFailure;
pub use super::boundary::ConicalJoinsQuery;
pub use super::boundary::certify_conical_boundary;
pub use super::boundary::certify_conical_joins;

pub use super::inverse::ConicalAuthoredContour;
pub use super::inverse::ConicalAuthoredFlux;
pub use super::inverse::ConicalAuthoredFluxError;
pub use super::inverse::ConicalAuthoredFluxQuery;
pub use super::inverse::ConicalAuthoredFluxStage;
pub use super::inverse::certify_conical_authored_flux;
