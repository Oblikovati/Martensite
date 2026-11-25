// SPDX-License-Identifier: GPL-2.0-only
//! Narrow-sign replay with the checked TKMath dyadic and Bernstein operations.
use super::*;
use crate::predicates::roots::certificate_arithmetic::{
    bernstein_bounds, bounded_dyadic::Interval,
};

type Columns = [Vec<Interval>; 3];

pub(super) fn side_layers(
    side: DecodedRuledSide<'_>,
    stock: DecodedConeFrame,
    trig: &Trig,
    denominator: Enclosure,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[Enclosure; 3], SeparationCause> {
    let [row_a, row_b] = side.rows;
    let a = lift(row_a, side.location, stock, trig, arithmetic)?;
    let b = lift(row_b, side.location, stock, trig, arithmetic)?;
    let denominator = Interval {
        lower: arithmetic.scalar(denominator.lo)?.lower,
        upper: arithmetic.scalar(denominator.hi)?.upper,
    };
    Ok([
        layer(&a, &a, &denominator, arithmetic)?,
        layer(&a, &b, &denominator, arithmetic)?,
        layer(&b, &b, &denominator, arithmetic)?,
    ])
}

fn lift(
    row: &[[f64; 4]],
    location: DecodedCoaxialLocation,
    stock: DecodedConeFrame,
    trig: &Trig,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Columns, SeparationCause> {
    let [cosine, sine] = &trig.exact;
    let [[a, b, _], [c, d, _], _] = location.matrix;
    let matrix = [
        arithmetic.scalar(a)?,
        arithmetic.scalar(b)?,
        arithmetic.scalar(c)?,
        arithmetic.scalar(d)?,
    ];
    let z0 = arithmetic.scalar(stock.origin[2])?;
    let translate = arithmetic.scalar(location.translation[2])?;
    let delta = arithmetic.subtract(&translate, &z0)?;
    let r = arithmetic.scalar(stock.radius)?;
    let rc = arithmetic.multiply(&r, cosine)?;
    let mut output = std::array::from_fn(|_| Vec::with_capacity(row.len()));
    for pole in row {
        let lifted = lift_pole(*pole, &matrix, [&delta, &rc, cosine, sine], arithmetic)?;
        for (column, value) in output.iter_mut().zip(lifted) {
            column.push(value);
        }
    }
    Ok(output)
}

fn lift_pole(
    [x, y, z, w]: [f64; 4],
    [a, b, c, d]: &[Interval; 4],
    [delta, rc, cosine, sine]: [&Interval; 4],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[Interval; 3], SeparationCause> {
    let x = arithmetic.scalar(x)?;
    let y = arithmetic.scalar(y)?;
    let z = arithmetic.scalar(z)?;
    let weight = arithmetic.scalar(w)?;
    let wx = combine(a, &x, b, &y, arithmetic)?;
    let wy = combine(c, &x, d, &y, arithmetic)?;
    let wx = arithmetic.multiply(&wx, cosine)?;
    let wy = arithmetic.multiply(&wy, cosine)?;
    let dz = arithmetic.add(&z, delta)?;
    let axial = arithmetic.multiply(&dz, sine)?;
    let q = arithmetic.add(rc, &axial)?;
    Ok([
        arithmetic.multiply(&wx, &weight)?,
        arithmetic.multiply(&wy, &weight)?,
        arithmetic.multiply(&q, &weight)?,
    ])
}

fn combine(
    a: &Interval,
    x: &Interval,
    b: &Interval,
    y: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, SeparationCause> {
    let ax = arithmetic.multiply(a, x)?;
    let by = arithmetic.multiply(b, y)?;
    Ok(arithmetic.add(&ax, &by)?)
}

fn layer(
    a: &Columns,
    b: &Columns,
    denominator: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Enclosure, SeparationCause> {
    let xx = bernstein_bounds::product(&a[0], &b[0], arithmetic)?;
    let yy = bernstein_bounds::product(&a[1], &b[1], arithmetic)?;
    let rr = bernstein_bounds::product(&a[2], &b[2], arithmetic)?;
    let sum = bernstein_bounds::sum(&xx, &yy, arithmetic)?;
    let coefficient = bernstein_bounds::difference(&sum, &rr, arithmetic)?;
    let hull = Interval {
        lower: bernstein_bounds::lower(&coefficient),
        upper: coefficient
            .iter()
            .map(|v| &v.upper)
            .max()
            .cloned()
            .unwrap_or_default(),
    };
    let signed = arithmetic.divide(&hull, denominator)?;
    outward(&signed)
}
