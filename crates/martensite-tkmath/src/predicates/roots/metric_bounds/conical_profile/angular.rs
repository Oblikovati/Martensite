// SPDX-License-Identifier: GPL-2.0-only
//! Candidate whole-span angular discrepancy bound on the unit parameter interval.
use super::angle::*;
use super::*;
use crate::predicates::roots::certificate_arithmetic::bernstein_bounds::difference;
use crate::predicates::roots::certificate_arithmetic::bernstein_bounds::lower;
use crate::predicates::roots::certificate_arithmetic::bernstein_bounds::product;
use crate::predicates::roots::certificate_arithmetic::bernstein_bounds::scale;
use crate::predicates::roots::certificate_arithmetic::bernstein_curve::derivative;
use crate::predicates::roots::metric_bounds::CylindricalProfileFailure;
use crate::predicates::roots::metric_bounds::cylindrical_profile::algebra;

pub(super) struct AngularSpanRequest<'a> {
    pub(super) source: &'a [[f64; 2]],
    pub(super) fit: &'a [f64],
    pub(super) anchor: AnchorRequest,
}
#[derive(Debug)]
pub(super) enum AngularSpanFailure {
    InvalidFit {
        count: usize,
        nonfinite: Option<(usize, u64)>,
    },
    MismatchedAnchor {
        source: Option<[u64; 2]>,
        anchor: [u64; 2],
        fit: u64,
        anchor_fit: u64,
    },
    Initial(InitialAngleFailure),
    Polynomial(CylindricalProfileFailure),
    Arithmetic(BoundedArithmeticFailure),
    UnresolvedDistance {
        lower: DyadicBound,
    },
}
impl std::fmt::Display for AngularSpanFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFit { count, nonfinite } => write!(
                f,
                "fit count={count}, nonfinite={nonfinite:?}; expected 1..=4 finite coefficients"
            ),
            Self::MismatchedAnchor {
                source,
                anchor,
                fit,
                anchor_fit,
            } => anchor_message(f, *source, *anchor, *fit, *anchor_fit),
            Self::Initial(cause) => write!(f, "initial phase: {cause}"),
            Self::Polynomial(cause) => write!(f, "bounded polynomial: {cause:?}"),
            Self::Arithmetic(cause) => write!(f, "bounded arithmetic: {cause:?}"),
            Self::UnresolvedDistance { lower } => write!(
                f,
                "distance lower={lower:?}; expected a strictly positive bound"
            ),
        }
    }
}
pub(super) fn anchor_message(
    f: &mut std::fmt::Formatter<'_>,
    source: Option<[u64; 2]>,
    anchor: [u64; 2],
    fit: u64,
    anchor_fit: u64,
) -> std::fmt::Result {
    write!(
        f,
        "source={source:?}, anchor={anchor:?}, fit={fit}, anchor_fit={anchor_fit}; expected matching endpoint bits"
    )
}
impl From<BoundedArithmeticFailure> for AngularSpanFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(cause)
    }
}
pub(super) struct AngularEvidence {
    pub(super) bound: Interval,
    pub(super) derivative: Interval,
}
pub(super) fn angular_from_captured(
    source: &[Vec<Interval>; 2],
    fit: &[Interval],
    q: &[Interval],
    anchor: Interval,
    sine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<AngularEvidence, AngularSpanFailure> {
    let denominator = angular_denominator(q, sine, arithmetic)?;
    let residual = angular_residual(source, fit, sine, q, arithmetic)?;
    let rate = arithmetic.divide(&magnitude_interval(&residual), &denominator)?;
    let bound = arithmetic.add(&magnitude_interval(&[anchor]), &rate)?;
    Ok(AngularEvidence {
        bound,
        derivative: rate,
    })
}
pub(super) fn admit_angular_request(
    request: &AngularSpanRequest<'_>,
) -> Result<(), AngularSpanFailure> {
    let nonfinite = request
        .fit
        .iter()
        .position(|value| !value.is_finite())
        .map(|index| (index, request.fit[index].to_bits()));
    if !(1..=4).contains(&request.fit.len()) || nonfinite.is_some() {
        return Err(AngularSpanFailure::InvalidFit {
            count: request.fit.len(),
            nonfinite,
        });
    }
    admit_angular_anchor(request)
}
pub(super) fn admit_angular_anchor(
    request: &AngularSpanRequest<'_>,
) -> Result<(), AngularSpanFailure> {
    let source = request.source.first().map(|p| p.map(f64::to_bits));
    let anchor = request.anchor.point.map(f64::to_bits);
    let fit = request.fit[0].to_bits();
    let anchor_fit = request.anchor.fit_half_angle_tangent.to_bits();
    if source != Some(anchor) || fit != anchor_fit {
        return Err(AngularSpanFailure::MismatchedAnchor {
            source,
            anchor,
            fit,
            anchor_fit,
        });
    }
    Ok(())
}
pub(super) fn capture_fit(
    values: &[f64],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<Interval>, AngularSpanFailure> {
    Ok(values
        .iter()
        .map(|value| arithmetic.scalar(*value))
        .collect::<Checked<Vec<_>>>()?)
}
pub(super) fn angular_denominator(
    q: &[Interval],
    sine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, AngularSpanFailure> {
    let qmin = lower(q);
    if qmin <= 0.into() {
        return Err(AngularSpanFailure::UnresolvedDistance {
            lower: DyadicBound::new(qmin, -i32::from(arithmetic.fractional_bits())),
        });
    }
    Ok(arithmetic.multiply(
        &Interval {
            lower: qmin.clone(),
            upper: qmin,
        },
        sine,
    )?)
}

pub(super) fn angular_residual(
    source: &[Vec<Interval>; 2],
    fit: &[Interval],
    sine: &Interval,
    radius: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<Interval>, AngularSpanFailure> {
    let cross = cross_product(source, arithmetic)?;
    let squared = product(fit, fit, arithmetic)?;
    let one = arithmetic.scalar(1.)?;
    let factor: Vec<_> = squared
        .iter()
        .map(|value| arithmetic.add(value, &one))
        .collect::<Checked<_>>()?;
    let first = product(&cross, &factor, arithmetic)?;
    let rate = derivative(fit, arithmetic)?;
    let second = product(radius, &rate, arithmetic)?;
    let second = scale(&second, sine, arithmetic)?;
    let two = arithmetic.scalar(2.)?;
    let second = scale(&second, &two, arithmetic)?;
    algebra::difference(&first, &second, arithmetic).map_err(AngularSpanFailure::Polynomial)
}

pub(super) fn cross_product(
    source: &[Vec<Interval>; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<Interval>> {
    let count = source[0].len().max(source[1].len());
    let x = elevate_constant(source[0].clone(), count);
    let y = elevate_constant(source[1].clone(), count);
    let dx = derivative(&x, arithmetic)?;
    let dy = derivative(&y, arithmetic)?;
    let first = product(&x, &dy, arithmetic)?;
    let second = product(&y, &dx, arithmetic)?;
    difference(&first, &second, arithmetic)
}
