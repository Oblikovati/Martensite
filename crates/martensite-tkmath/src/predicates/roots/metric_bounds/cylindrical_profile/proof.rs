// SPDX-License-Identifier: GPL-2.0-only
use super::{algebra::*, *};
use num_bigint::Sign;

type Homogeneous = [Vec<Interval>; 4];

pub(super) fn bounds(
    query: CylindricalProfileQuery<'_>,
    authored: &[Vec<Interval>; 2],
    stored: &Homogeneous,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[f64; 6]> {
    let weight = minimum_weight(stored);
    let trigonometric = trigonometric::enclose(&authored[0], arithmetic)?;
    let position = position(query, authored, stored, &trigonometric, &weight, arithmetic)?;
    let derivative = derivatives(query.support_radius, authored, stored, &weight, arithmetic)?;
    Ok([
        position[0],
        position[1],
        position[2],
        derivative[0],
        derivative[1],
        derivative[2],
    ])
}

fn position(
    query: CylindricalProfileQuery<'_>,
    authored: &[Vec<Interval>; 2],
    stored: &Homogeneous,
    trig: &[Vec<Interval>; 2],
    weight: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[f64; 3]> {
    let radius = arithmetic.scalar(query.stored_radius)?;
    let residual = xy_residual(&radius, stored, trig, weight, arithmetic)?;
    let gap = arithmetic.subtract(&radius, &residual)?;
    if gap.lower.sign() != Sign::Plus {
        return Err(CylindricalProfileFailure::DirectionUnresolved {
            residual: arithmetic.upper_f64(residual.upper)?,
            radius: query.stored_radius,
        });
    }
    let angular = arithmetic.divide(&residual, &gap)?;
    let height = height_residual(&authored[1], stored, weight, arithmetic)?;
    report_bounds(query.support_radius, angular, height, arithmetic)
}

pub(super) fn minimum_weight(stored: &Homogeneous) -> Interval {
    let minimum = polynomial::lower(&stored[3]);
    Interval {
        lower: minimum.clone(),
        upper: minimum,
    }
}

pub(super) fn xy_residual(
    radius: &Interval,
    stored: &Homogeneous,
    trig: &[Vec<Interval>; 2],
    weight: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let scaled = polynomial::scale(&stored[3], radius, arithmetic)?;
    let x = spatial_residual(&stored[0], &scaled, &trig[0], arithmetic)?;
    let y = spatial_residual(&stored[1], &scaled, &trig[1], arithmetic)?;
    let xy = arithmetic.add(&x, &y)?;
    let residual = arithmetic.divide(&xy, weight)?;
    Ok(residual)
}

pub(super) fn height_residual(
    height: &[Interval],
    stored: &Homogeneous,
    weight: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let height = spatial_residual(&stored[2], &stored[3], height, arithmetic)?;
    Ok(arithmetic.divide(&height, weight)?)
}

fn spatial_residual(
    coordinate: &[Interval],
    factor: &[Interval],
    target: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let target = polynomial::product(factor, target, arithmetic)?;
    refined_magnitude(&difference(coordinate, &target, arithmetic)?, arithmetic)
}

fn report_bounds(
    radius: f64,
    angular: Interval,
    height: Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[f64; 3]> {
    let angular = arithmetic.upper_f64(angular.upper)?;
    let height = arithmetic.upper_f64(height.upper)?;
    let radius = arithmetic.scalar(radius)?;
    let angle = arithmetic.scalar(angular)?;
    let s = arithmetic.multiply(&radius, &angle)?;
    let z = arithmetic.scalar(height)?;
    let total = arithmetic.add(&s, &z)?;
    Ok([angular, height, arithmetic.upper_f64(total.upper)?])
}

fn derivatives(
    radius: f64,
    authored: &[Vec<Interval>; 2],
    stored: &Homogeneous,
    weight: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[f64; 3]> {
    let xx = polynomial::product(&stored[0], &stored[0], arithmetic)?;
    let yy = polynomial::product(&stored[1], &stored[1], arithmetic)?;
    let norm = sum(&xx, &yy, arithmetic)?;
    let theta = rational_derivative(&stored[1], &stored[0], arithmetic)?;
    let theta = derivative_residual(&theta, &norm, &authored[0], arithmetic)?;
    let angular = arithmetic.upper_quotient(&theta.upper, &polynomial::lower(&norm))?;
    let ww = polynomial::product(&stored[3], &stored[3], arithmetic)?;
    let z = rational_derivative(&stored[2], &stored[3], arithmetic)?;
    let z = derivative_residual(&z, &ww, &authored[1], arithmetic)?;
    let weight_square = arithmetic.multiply(weight, weight)?;
    let height = arithmetic.upper_quotient(&z.upper, &weight_square.lower)?;
    let angular = arithmetic.scalar(angular)?;
    let height = arithmetic.scalar(height)?;
    report_bounds(radius, angular, height, arithmetic)
}

fn rational_derivative(
    numerator: &[Interval],
    denominator: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<Interval>> {
    let dn = derivative(numerator, arithmetic)?;
    let dd = derivative(denominator, arithmetic)?;
    let left = polynomial::product(&dn, denominator, arithmetic)?;
    let right = polynomial::product(numerator, &dd, arithmetic)?;
    difference(&left, &right, arithmetic)
}

fn derivative_residual(
    numerator: &[Interval],
    denominator: &[Interval],
    authored: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let authored_derivative = derivative(authored, arithmetic)?;
    let target = polynomial::product(&authored_derivative, denominator, arithmetic)?;
    Ok(magnitude(&difference(numerator, &target, arithmetic)?))
}

fn refined_magnitude(poles: &[Interval], arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
    let mut cells = vec![poles.to_vec()];
    // Fixed V1 proof cells do not change geometric subdivisions or source identity.
    for _ in 0..2 {
        let split = cells
            .iter()
            .map(|cell| Ok(source::bisect(cell, arithmetic)?))
            .collect::<Checked<Vec<_>>>()?;
        cells = split.into_iter().flatten().collect();
    }
    let bound = cells
        .iter()
        .fold(0.into(), |bound: num_bigint::BigInt, cell| {
            bound.max(polynomial::magnitude(cell))
        });
    Ok(Interval {
        lower: bound.clone(),
        upper: bound,
    })
}
