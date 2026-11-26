// SPDX-License-Identifier: GPL-2.0-only
//! Original chart trigonometry, lift and conservative model-space metric.
use super::{
    Checked, CylinderPointChart, CylinderPointError as Error, ExactVector, Ratio,
    SegmentCylinderWall, rounded,
};
use crate::predicates::roots::{
    DyadicBound,
    certificate_arithmetic::{
        bounded_dyadic::{Arithmetic, ArithmeticPolicy, Interval},
        rounding::outward_sqrt,
    },
    metric_bounds::cylindrical_profile::trigonometric,
};
use num_bigint::BigInt;

const POLICY: ArithmeticPolicy = ArithmeticPolicy {
    fractional_bits: 160,
    max_operations: 10_000,
};
type EnclosedVector = [Interval; 3];

pub(super) fn qualify(
    wall: &SegmentCylinderWall,
    angle: &Ratio,
    axial: &Ratio,
    retained: &ExactVector,
    limit: f64,
    cancelled: &mut (impl FnMut() -> bool + Send),
) -> Checked<([[f64; 2]; 3], f64)> {
    let mut arithmetic = Arithmetic::new(POLICY, cancelled);
    let (_, radial) = phase(wall, angle, &mut arithmetic)?;
    let point = world(wall, axial, &radial, &mut arithmetic)?;
    let distance = distance(&point, retained, limit, &mut arithmetic)?;
    let [x, y, z] = point
        .each_ref()
        .map(|interval| bounds(interval, &arithmetic));
    Ok(([x?, y?, z?], distance))
}

pub(super) fn chart(
    wall: &SegmentCylinderWall,
    uv: [f64; 2],
    angle: &Ratio,
    axial: &Ratio,
    cancelled: &mut (impl FnMut() -> bool + Send),
) -> Checked<CylinderPointChart> {
    let mut arithmetic = Arithmetic::new(POLICY, cancelled);
    let ([cosine, sine], radial) = phase(wall, angle, &mut arithmetic)?;
    let derivative = radial_components(wall, [&negative(&sine), &cosine], &mut arithmetic)?;
    let world = world(wall, axial, &radial, &mut arithmetic)?;
    let [a, b, c] = wall.axes[2].each_ref().map(rounded);
    Ok(CylinderPointChart {
        uv,
        world: vector_nominal(&world, &arithmetic)?,
        radial: vector_nominal(&radial, &arithmetic)?,
        tangents: [vector_nominal(&derivative, &arithmetic)?, [a?, b?, c?]],
    })
}

fn phase(
    wall: &SegmentCylinderWall,
    angle: &Ratio,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<([Interval; 2], EnclosedVector)> {
    let argument = arithmetic.rational(angle)?;
    let phase = trigonometric::scalar(&argument, arithmetic)?;
    let radial = radial_components(wall, phase.each_ref(), arithmetic)?;
    Ok((phase, radial))
}

fn radial_components(
    wall: &SegmentCylinderWall,
    phase: [&Interval; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<EnclosedVector> {
    let mut result = std::array::from_fn(|_| arithmetic.zero());
    for (axis, value) in result.iter_mut().enumerate() {
        let u = arithmetic.rational(&wall.axes[0][axis].multiply(&wall.radius))?;
        let v = arithmetic.rational(&wall.axes[1][axis].multiply(&wall.radius))?;
        let x = arithmetic.multiply(&u, phase[0])?;
        let y = arithmetic.multiply(&v, phase[1])?;
        *value = arithmetic.add(&x, &y)?;
    }
    Ok(result)
}

fn world(
    wall: &SegmentCylinderWall,
    axial: &Ratio,
    radial: &EnclosedVector,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<EnclosedVector> {
    let mut result = std::array::from_fn(|_| arithmetic.zero());
    for (axis, value) in result.iter_mut().enumerate() {
        let center = wall.origin[axis].add(&wall.axes[2][axis].multiply(axial));
        let center = arithmetic.rational(&center)?;
        *value = arithmetic.add(&center, &radial[axis])?;
    }
    Ok(result)
}

fn distance(
    point: &EnclosedVector,
    retained: &ExactVector,
    limit: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<f64> {
    let mut squared = BigInt::from(0);
    for (coordinate, world) in point.iter().zip(retained) {
        let actual = arithmetic.rational(world)?;
        let delta = arithmetic.subtract(coordinate, &actual)?;
        let magnitude = delta.magnitude();
        squared += &magnitude * &magnitude;
    }
    let squared = Ratio::from_dyadic(&DyadicBound::new(
        squared,
        -2 * i32::from(arithmetic.fractional_bits()),
    ));
    let cap = Ratio::from_finite(limit);
    let bound = if squared.sign().is_eq() {
        Some(0.)
    } else {
        outward_sqrt(&squared)
    };
    if squared > cap.multiply(&cap) {
        return Err(Error::Distance { bound, limit });
    }
    bound.ok_or(Error::Distance { bound, limit })
}

fn vector_nominal(vector: &EnclosedVector, arithmetic: &Arithmetic<'_>) -> Checked<[f64; 3]> {
    let [a, b, c] = vector.each_ref().map(|interval| {
        rounded(&Ratio::from_dyadic(&DyadicBound::new(
            &interval.lower + &interval.upper,
            -i32::from(arithmetic.fractional_bits()) - 1,
        )))
    });
    Ok([a?, b?, c?])
}

fn bounds(interval: &Interval, arithmetic: &Arithmetic<'_>) -> Checked<[f64; 2]> {
    Ok([
        -arithmetic.upper_f64(-&interval.lower)?,
        arithmetic.upper_f64(interval.upper.clone())?,
    ])
}

fn negative(value: &Interval) -> Interval {
    Interval {
        lower: -&value.upper,
        upper: -&value.lower,
    }
}
