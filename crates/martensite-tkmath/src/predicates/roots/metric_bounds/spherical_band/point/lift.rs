// SPDX-License-Identifier: GPL-2.0-only
//! Outward original world lift, analytic tangents and exact-grid Euclidean metric.
use super::super::{
    EVec,
    trace::{add, enclose, scale},
};
use super::{
    Arithmetic, Checked, Error, ExactVector, R, SphericalBand, SphericalPointChart, grid, rounded,
};
use crate::predicates::roots::{
    DyadicBound,
    certificate_arithmetic::{bounded_dyadic::Interval, rounding::outward_sqrt},
    metric_bounds::cylindrical_profile::trigonometric,
};
use num_bigint::BigInt;

pub(super) fn qualify(
    band: &SphericalBand,
    [u, v]: [&R; 2],
    retained: &ExactVector,
    limit: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<([[f64; 2]; 3], f64)> {
    let uv = phases([u, v], arithmetic)?;
    let basis = basis(band, &uv[0], arithmetic)?;
    let point = world(band, &basis, &uv[1], arithmetic)?;
    let evidence = distance(&point, retained, limit, arithmetic)?;
    let [x, y, z] = point.each_ref().map(|value| outward(value, arithmetic));
    Ok(([x?, y?, z?], evidence))
}

pub(super) fn chart(
    band: &SphericalBand,
    uv: [f64; 2],
    exact_uv: [&R; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<SphericalPointChart> {
    let [longitude, latitude] = phases(exact_uv, arithmetic)?;
    let basis = basis(band, &longitude, arithmetic)?;
    let point = world(band, &basis, &latitude, arithmetic)?;
    let tangents = tangents(&basis, &latitude, arithmetic)?;
    let [radial, tangent, axial] = basis
        .each_ref()
        .map(|axis| vector_nominal(axis, arithmetic));
    Ok(SphericalPointChart {
        uv,
        world: vector_nominal(&point, arithmetic)?,
        basis: [radial?, tangent?, axial?],
        latitude_phase: [
            nominal(&latitude[0], arithmetic)?,
            nominal(&latitude[1], arithmetic)?,
        ],
        tangents: [
            vector_nominal(&tangents[0], arithmetic)?,
            vector_nominal(&tangents[1], arithmetic)?,
        ],
    })
}

fn phases(
    [first, second]: [&R; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[[Interval; 2]; 2]> {
    let u = arithmetic.rational(first)?;
    let v = arithmetic.rational(second)?;
    Ok([
        trigonometric::scalar(&u, arithmetic)?,
        trigonometric::scalar(&v, arithmetic)?,
    ])
}

fn basis(
    band: &SphericalBand,
    [cosine, sine]: &[Interval; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[EVec; 3]> {
    let [x, y, z] = band.axes.each_ref().map(|row| {
        enclose(
            &row.each_ref().map(|value| band.radius.multiply(value)),
            arithmetic,
        )
    });
    let [x, y, z] = [x?, y?, z?];
    let radial = combined([&x, &y], [cosine, sine], arithmetic)?;
    let tangent = combined([&x, &y], [&negative(sine), cosine], arithmetic)?;
    Ok([radial, tangent, z])
}

fn world(
    band: &SphericalBand,
    [radial, _, axial]: &[EVec; 3],
    [cosine, sine]: &[Interval; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<EVec> {
    let position = combined([radial, axial], [cosine, sine], arithmetic)?;
    let origin = enclose(&band.origin, arithmetic)?;
    add(&origin, &position, arithmetic)
}

fn tangents(
    [radial, longitude, axial]: &[EVec; 3],
    [cosine, sine]: &[Interval; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[EVec; 2]> {
    let u = scale(longitude, cosine, arithmetic)?;
    let v = combined([radial, axial], [&negative(sine), cosine], arithmetic)?;
    Ok([u, v])
}

fn combined(
    [a, b]: [&EVec; 2],
    [x, y]: [&Interval; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<EVec> {
    let a = scale(a, x, arithmetic)?;
    let b = scale(b, y, arithmetic)?;
    add(&a, &b, arithmetic)
}

fn negative(value: &Interval) -> Interval {
    Interval {
        lower: -&value.upper,
        upper: -&value.lower,
    }
}

fn distance(
    point: &EVec,
    retained: &ExactVector,
    limit: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<f64> {
    let mut squared = BigInt::from(0);
    for (ideal, actual) in point.iter().zip(retained) {
        let actual = arithmetic.rational(actual)?;
        let maximum = arithmetic.subtract(ideal, &actual)?.magnitude();
        squared += &maximum * &maximum;
    }
    let squared = R::from_dyadic(&DyadicBound::new(
        squared,
        -2 * i32::from(arithmetic.fractional_bits()),
    ));
    let bound = if squared.sign().is_eq() {
        Some(0.)
    } else {
        outward_sqrt(&squared)
    };
    let cap = R::from_finite(limit);
    if squared > cap.multiply(&cap) {
        return Err(Error::Distance { bound, limit });
    }
    bound.ok_or(Error::Distance { bound, limit })
}

fn outward(value: &Interval, arithmetic: &Arithmetic<'_>) -> Checked<[f64; 2]> {
    Ok([
        -arithmetic.upper_f64(-&value.lower)?,
        arithmetic.upper_f64(value.upper.clone())?,
    ])
}

fn nominal(value: &Interval, arithmetic: &Arithmetic<'_>) -> Checked<f64> {
    let midpoint = grid(&value.lower + &value.upper, arithmetic).divide(&R::from_finite(2.));
    rounded(&midpoint)
}

fn vector_nominal([x, y, z]: &EVec, arithmetic: &Arithmetic<'_>) -> Checked<[f64; 3]> {
    Ok([
        nominal(x, arithmetic)?,
        nominal(y, arithmetic)?,
        nominal(z, arithmetic)?,
    ])
}
