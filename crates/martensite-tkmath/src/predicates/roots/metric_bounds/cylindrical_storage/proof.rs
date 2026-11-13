// SPDX-License-Identifier: GPL-2.0-only
//! Bernstein hulls bound residuals of the stored operands, including rounded weights.
use super::{Arithmetic, Interval, error::Checked, polynomial::*};

type Homogeneous = [Vec<Interval>; 4];

pub(super) fn bounds(
    curves: &[Homogeneous; 2],
    radii: [f64; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[f64; 5]> {
    let radius_values = radii;
    let radii = [arithmetic.scalar(radii[0])?, arithmetic.scalar(radii[1])?];
    let weights = curves.each_ref().map(|curve| {
        let lower = lower(&curve[3]);
        Interval {
            lower: lower.clone(),
            upper: lower,
        }
    });
    let radial0 = radial(&curves[0], &radii[0], &weights[0], arithmetic)?;
    let radial1 = radial(&curves[1], &radii[1], &weights[1], arithmetic)?;
    let (ruling, height) = ruled(curves, &radii, &weights, arithmetic)?;
    let normal = normal_sweep(
        [radial0, radial1, ruling, height],
        radius_values,
        &radii,
        arithmetic,
    )?;
    Ok([radial0, radial1, ruling, height, normal])
}

fn normal_sweep(
    bounds: [f64; 4],
    radius_values: [f64; 2],
    radii: &[Interval; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<f64> {
    if bounds[0] >= radius_values[0] {
        return Err(
            super::CylindricalStorageFailure::ReferenceDirectionUnresolved {
                radial_bound: bounds[0],
                radius: radius_values[0],
            },
        );
    }
    let radial = arithmetic.scalar(bounds[0])?;
    let scaled = arithmetic.multiply(&radii[1], &radial)?;
    let last = arithmetic.upper_quotient(&scaled.upper, &radii[0].lower)?;
    let last = arithmetic.scalar(last)?;
    let ruling = arithmetic.scalar(bounds[2])?;
    let last = arithmetic.add(&last, &ruling)?;
    let xy = Interval {
        lower: 0.into(),
        upper: last.upper.max(radial.upper),
    };
    let height = arithmetic.scalar(bounds[3])?;
    let total = arithmetic.add(&xy, &height)?;
    arithmetic.upper_f64(total.upper).map_err(Into::into)
}

fn radial(
    curve: &Homogeneous,
    radius: &Interval,
    weight_min: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<f64> {
    let xx = product(&curve[0], &curve[0], arithmetic)?;
    let yy = product(&curve[1], &curve[1], arithmetic)?;
    let xy_squared = sum(&xx, &yy, arithmetic)?;
    let radius_squared = arithmetic.multiply(radius, radius)?;
    let ww = product(&curve[3], &curve[3], arithmetic)?;
    let rww = scale(&ww, &radius_squared, arithmetic)?;
    let residual = difference(&xy_squared, &rww, arithmetic)?;
    let weight_squared = arithmetic.multiply(weight_min, weight_min)?;
    let denominator = arithmetic.multiply(radius, &weight_squared)?;
    // |rho-r| = |X²+Y²-r²W²| / (W²*(rho+r)); rho+r >= r > 0.
    arithmetic
        .upper_quotient(&magnitude(&residual), &denominator.lower)
        .map_err(Into::into)
}

fn cross(
    curves: &[Homogeneous; 2],
    axis: usize,
    radii: Option<&[Interval; 2]>,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<Interval>> {
    let last = product(&curves[1][axis], &curves[0][3], arithmetic)?;
    let first = product(&curves[0][axis], &curves[1][3], arithmetic)?;
    let Some(radii) = radii else {
        return difference(&last, &first, arithmetic).map_err(Into::into);
    };
    let last = scale(&last, &radii[0], arithmetic)?;
    let first = scale(&first, &radii[1], arithmetic)?;
    difference(&last, &first, arithmetic).map_err(Into::into)
}

fn ruled(
    curves: &[Homogeneous; 2],
    radii: &[Interval; 2],
    weights: &[Interval; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<(f64, f64)> {
    let x = magnitude(&cross(curves, 0, Some(radii), arithmetic)?);
    let y = magnitude(&cross(curves, 1, Some(radii), arithmetic)?);
    let z = magnitude(&cross(curves, 2, None, arithmetic)?);
    let ww = arithmetic.multiply(&weights[0], &weights[1])?;
    let rww = arithmetic.multiply(&radii[0], &ww)?;
    let xy = arithmetic.add(
        &Interval {
            lower: x.clone(),
            upper: x,
        },
        &Interval {
            lower: y.clone(),
            upper: y,
        },
    )?;
    let ruling = arithmetic.upper_quotient(&xy.upper, &rww.lower)?;
    let height = arithmetic.upper_quotient(&z, &ww.lower)?;
    Ok((ruling, height))
}
