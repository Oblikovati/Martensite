// SPDX-License-Identifier: GPL-2.0-only
//! Candidate ideal-to-stored rational cone-edge discrepancy, in local support axes.
use super::*;
use crate::predicates::roots::certificate_arithmetic::bernstein_bounds::product;
use crate::predicates::roots::certificate_arithmetic::bernstein_bounds::scale;
use crate::predicates::roots::certificate_arithmetic::bernstein_bounds::sum;
use crate::predicates::roots::metric_bounds::cylindrical_profile::algebra;
use crate::predicates::roots::metric_bounds::cylindrical_profile::trigonometric;

pub use super::storage_api::ConicalStorageError;
pub use super::storage_api::ConicalStorageEvidence;
pub use super::storage_api::ConicalStorageFailure;
pub use super::storage_api::ConicalStorageQuery;
pub use super::storage_api::certify_conical_storage;
impl From<BoundedArithmeticFailure> for ConicalStorageFailure {
    fn from(value: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(value)
    }
}
impl std::fmt::Display for ConicalStorageFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shape { .. } | Self::Nonfinite { .. } => self.input_message(f),
            Self::Support { semi_angle } => write!(
                f,
                "semi-angle={semi_angle}; expected finite angle in (0, pi/2)"
            ),
            Self::Policy {
                fractional_bits,
                max_operations,
            } => policy_message(f, *fractional_bits, *max_operations),
            Self::Weight { role, lower } => {
                write!(f, "{role} lower={lower:?}; expected strictly positive")
            }
            Self::Polynomial(cause) => write!(f, "polynomial: {cause:?}"),
            Self::Arithmetic(cause) => write!(f, "arithmetic: {cause:?}"),
        }
    }
}
impl ConicalStorageFailure {
    pub(super) fn input_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shape {
                tangent,
                numerator,
                weights,
                poles,
                stored_weights,
            } => shape_message(f, [*tangent, *numerator, *weights, *poles, *stored_weights]),
            Self::Nonfinite {
                role,
                index,
                axis,
                bits,
            } => nonfinite_message(f, role, *index, *axis, *bits),
            other => write!(f, "{other:?}"),
        }
    }
}
pub(super) fn policy_message(
    f: &mut std::fmt::Formatter<'_>,
    fractional_bits: u16,
    max_operations: u32,
) -> std::fmt::Result {
    write!(
        f,
        "precision={fractional_bits}, operations={max_operations}; expected precision 8..=4096 and operations <=1000000"
    )
}
pub(super) fn shape_message(
    f: &mut std::fmt::Formatter<'_>,
    [tangent, numerator, weights, poles, stored_weights]: [usize; 5],
) -> std::fmt::Result {
    write!(
        f,
        "tangent={tangent}, numerator={numerator}, weights={weights}, poles={poles}, stored_weights={stored_weights}; expected tangent 1..=4, matching numerator/weights 1..=10 and matching poles/weights 2..=16"
    )
}
pub(super) fn nonfinite_message(
    f: &mut std::fmt::Formatter<'_>,
    role: &str,
    index: usize,
    axis: usize,
    bits: u64,
) -> std::fmt::Result {
    write!(f, "{role}[{index}][{axis}] bits={bits}; expected finite")
}
pub(super) type Coefficients = [Vec<Interval>; 4];
pub(super) fn storage_discrepancy(
    request: &ConicalStorageQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, ConicalStorageFailure> {
    admit_storage(request)?;
    let (ideal, stored, denominator) = prepare_storage(request, arithmetic)?;
    let total = residual_sum(&ideal, &stored, arithmetic)?;
    Ok(arithmetic.divide(&total, &denominator)?)
}
pub(super) type PreparedStorage = (Coefficients, Coefficients, Interval);
pub(super) fn prepare_storage(
    request: &ConicalStorageQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<PreparedStorage, ConicalStorageFailure> {
    let (ideal, source_weights) = ideal_homogeneous(request, arithmetic)?;
    let stored = stored_homogeneous(request, arithmetic)?;
    let source_min = positive_weight("source weight", &source_weights, arithmetic)?;
    let stored_min = positive_weight("stored weight", &stored[3], arithmetic)?;
    let denominator = arithmetic.multiply(&source_min, &stored_min)?;
    Ok((ideal, stored, denominator))
}
pub(super) fn positive_weight(
    role: &'static str,
    values: &[Interval],
    arithmetic: &Arithmetic<'_>,
) -> Result<Interval, ConicalStorageFailure> {
    let lower = crate::predicates::roots::certificate_arithmetic::bernstein_bounds::lower(values);
    if lower <= 0.into() {
        return Err(ConicalStorageFailure::Weight {
            role,
            lower: DyadicBound::new(lower, -i32::from(arithmetic.fractional_bits())),
        });
    }
    Ok(Interval {
        lower: lower.clone(),
        upper: lower,
    })
}
pub(super) fn residual_sum(
    ideal: &Coefficients,
    stored: &Coefficients,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, ConicalStorageFailure> {
    let mut total = residual_axis(ideal, stored, 0, arithmetic)?;
    for axis in 1..3 {
        let residual = residual_axis(ideal, stored, axis, arithmetic)?;
        total = arithmetic.add(&total, &residual)?;
    }
    Ok(total)
}
pub(super) fn residual_axis(
    ideal: &Coefficients,
    stored: &Coefficients,
    axis: usize,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, ConicalStorageFailure> {
    let residual = rational_residual(
        &ideal[axis],
        &ideal[3],
        &stored[axis],
        &stored[3],
        arithmetic,
    )?;
    Ok(magnitude_interval(&residual))
}

pub(super) fn admit_storage(
    request: &ConicalStorageQuery<'_>,
) -> Result<(), ConicalStorageFailure> {
    admit_counts(request)?;
    if !request.semi_angle.is_finite()
        || request.semi_angle <= 0.
        || request.semi_angle >= std::f64::consts::FRAC_PI_2
    {
        return Err(ConicalStorageFailure::Support {
            semi_angle: request.semi_angle,
        });
    }
    admit_values(request)
}
pub(super) fn admit_counts(request: &ConicalStorageQuery<'_>) -> Result<(), ConicalStorageFailure> {
    if !(1..=4).contains(&request.tangent.len())
        || !(1..=10).contains(&request.numerator.len())
        || request.numerator.len() != request.weights.len()
        || !(2..=16).contains(&request.stored_poles.len())
        || request.stored_poles.len() != request.stored_weights.len()
    {
        return Err(ConicalStorageFailure::Shape {
            tangent: request.tangent.len(),
            numerator: request.numerator.len(),
            weights: request.weights.len(),
            poles: request.stored_poles.len(),
            stored_weights: request.stored_weights.len(),
        });
    }
    Ok(())
}
pub(super) fn admit_values(request: &ConicalStorageQuery<'_>) -> Result<(), ConicalStorageFailure> {
    for (role, values) in [
        ("tangent", request.tangent),
        ("numerator", request.numerator),
        ("weights", request.weights),
        ("stored weights", request.stored_weights),
        ("center and depth", &[request.center, request.depth][..]),
    ] {
        admit_finite_slice(role, values)?;
    }
    admit_finite_poles(request.stored_poles)
}
pub(super) fn admit_finite_slice(
    role: &'static str,
    values: &[f64],
) -> Result<(), ConicalStorageFailure> {
    for (index, value) in values.iter().copied().enumerate() {
        if !value.is_finite() {
            return Err(ConicalStorageFailure::Nonfinite {
                role,
                index,
                axis: 0,
                bits: value.to_bits(),
            });
        }
    }
    Ok(())
}
pub(super) fn admit_finite_poles(poles: &[[f64; 3]]) -> Result<(), ConicalStorageFailure> {
    for (index, pole) in poles.iter().enumerate() {
        admit_finite_pole(index, pole)?;
    }
    Ok(())
}
fn admit_finite_pole(index: usize, pole: &[f64; 3]) -> Result<(), ConicalStorageFailure> {
    for (axis, value) in pole.iter().copied().enumerate() {
        if !value.is_finite() {
            return Err(ConicalStorageFailure::Nonfinite {
                role: "stored pole",
                index,
                axis,
                bits: value.to_bits(),
            });
        }
    }
    Ok(())
}
pub(super) fn capture(
    values: &[f64],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<Interval>, ConicalStorageFailure> {
    Ok(values
        .iter()
        .map(|v| arithmetic.scalar(*v))
        .collect::<Checked<_>>()?)
}
pub(super) fn ideal_homogeneous(
    request: &ConicalStorageQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(Coefficients, Vec<Interval>), ConicalStorageFailure> {
    let tangent = capture(request.tangent, arithmetic)?;
    let numerator = capture(request.numerator, arithmetic)?;
    let weights = capture(request.weights, arithmetic)?;
    let [cosine, sine, angular_weight] = angular_polynomials(&tangent, request.center, arithmetic)?;
    let [radial, axial] = meridian_polynomials(&numerator, &weights, request, arithmetic)?;
    let ideal = [
        product(&radial, &cosine, arithmetic)?,
        product(&radial, &sine, arithmetic)?,
        product(&axial, &angular_weight, arithmetic)?,
        product(&weights, &angular_weight, arithmetic)?,
    ];
    Ok((ideal, weights))
}
pub(super) fn angular_polynomials(
    tangent: &[Interval],
    center: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[Vec<Interval>; 3], ConicalStorageFailure> {
    let squared = product(tangent, tangent, arithmetic)?;
    let one = arithmetic.scalar(1.)?;
    let ones = vec![one; squared.len()];
    let weight = sum(&ones, &squared, arithmetic)?;
    let cosine = crate::predicates::roots::certificate_arithmetic::bernstein_bounds::difference(
        &ones, &squared, arithmetic,
    )?;
    let two = arithmetic.scalar(2.)?;
    let sine = product(tangent, &vec![two; tangent.len()], arithmetic)?;
    rotate_angular(cosine, sine, weight, center, arithmetic)
}
pub(super) fn rotate_angular(
    cosine: Vec<Interval>,
    sine: Vec<Interval>,
    weight: Vec<Interval>,
    center: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[Vec<Interval>; 3], ConicalStorageFailure> {
    let phase = arithmetic.scalar(center)?;
    let [c, s] =
        trigonometric::enclose(&[phase], arithmetic).map_err(ConicalStorageFailure::Polynomial)?;
    let c1 = scale(&cosine, &c[0], arithmetic)?;
    let s1 = scale(&sine, &s[0], arithmetic)?;
    let x = crate::predicates::roots::certificate_arithmetic::bernstein_bounds::difference(
        &c1, &s1, arithmetic,
    )?;
    let c2 = scale(&cosine, &s[0], arithmetic)?;
    let s2 = scale(&sine, &c[0], arithmetic)?;
    Ok([x, sum(&c2, &s2, arithmetic)?, weight])
}
pub(super) fn meridian_polynomials(
    numerator: &[Interval],
    weights: &[Interval],
    request: &ConicalStorageQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[Vec<Interval>; 2], ConicalStorageFailure> {
    let alpha = arithmetic.scalar(request.semi_angle)?;
    let [c, s] =
        trigonometric::enclose(&[alpha], arithmetic).map_err(ConicalStorageFailure::Polynomial)?;
    let signed = if request.outward {
        request.depth
    } else {
        -request.depth
    };
    let depth = arithmetic.scalar(signed)?;
    let dc = arithmetic.multiply(&depth, &c[0])?;
    let ds = arithmetic.multiply(&depth, &s[0])?;
    meridian_terms(numerator, weights, &c[0], &s[0], &dc, &ds, arithmetic)
}
pub(super) fn meridian_terms(
    numerator: &[Interval],
    weights: &[Interval],
    cosine: &Interval,
    sine: &Interval,
    dc: &Interval,
    ds: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[Vec<Interval>; 2], ConicalStorageFailure> {
    let first = scale(numerator, sine, arithmetic)?;
    let second = scale(weights, dc, arithmetic)?;
    let radial = sum(&first, &second, arithmetic)?;
    let first = scale(numerator, cosine, arithmetic)?;
    let second = scale(weights, ds, arithmetic)?;
    let axial = crate::predicates::roots::certificate_arithmetic::bernstein_bounds::difference(
        &first, &second, arithmetic,
    )?;
    Ok([radial, axial])
}
pub(super) fn stored_homogeneous(
    request: &ConicalStorageQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Coefficients, ConicalStorageFailure> {
    let mut result: Coefficients =
        std::array::from_fn(|_| Vec::with_capacity(request.stored_poles.len()));
    for (pole, weight) in request.stored_poles.iter().zip(request.stored_weights) {
        let weight = arithmetic.scalar(*weight)?;
        for axis in 0..3 {
            let coordinate = arithmetic.scalar(pole[axis])?;
            result[axis].push(arithmetic.multiply(&coordinate, &weight)?);
        }
        result[3].push(weight);
    }
    Ok(result)
}
pub(super) fn rational_residual(
    ideal: &[Interval],
    ideal_weight: &[Interval],
    stored: &[Interval],
    stored_weight: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<Interval>, ConicalStorageFailure> {
    let a = product(ideal, stored_weight, arithmetic)?;
    let b = product(stored, ideal_weight, arithmetic)?;
    algebra::difference(&a, &b, arithmetic).map_err(ConicalStorageFailure::Polynomial)
}

#[cfg(test)]
#[path = "storage_tests.rs"]
mod tests;
