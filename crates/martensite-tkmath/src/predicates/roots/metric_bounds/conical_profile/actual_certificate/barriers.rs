// SPDX-License-Identifier: GPL-2.0-only
//! Common D-coordinate bounds cover the entire family, including cross-member carriers.
use super::*;
use capture::SideCapture;
pub(super) struct Barriers {
    pub(super) dv_lower: Interval,
}
pub(super) fn certify(
    sides: &[SideCapture],
    sine: &Interval,
    cosine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Barriers> {
    let scale = scale(sides);
    let radial = arithmetic.subtract(&numeric::lower_point(&scale.radius), &scale.storage)?;
    numeric::positive("family actual radius", &radial, arithmetic)?;
    let dv_lower = dv_bound(sides, &scale, sine, cosine, arithmetic)?;
    numeric::positive("whole-family D_v", &dv_lower, arithmetic)?;
    family(sides, sine, arithmetic)?;
    Ok(Barriers { dv_lower })
}
fn dot_lower(
    radius: &Interval,
    storage: &Interval,
    error: &Interval,
    gap: &Interval,
    cosine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Interval> {
    let a = arithmetic.multiply(gap, cosine)?;
    let main = arithmetic.multiply(&numeric::lower_point(radius), &a)?;
    let first = arithmetic.multiply(&numeric::upper_point(radius), error)?;
    let second = arithmetic.multiply(&a, storage)?;
    let third = arithmetic.multiply(storage, error)?;
    let bound = arithmetic.subtract(&main, &first)?;
    let bound = arithmetic.subtract(&bound, &second)?;
    let bound = arithmetic.subtract(&bound, &third)?;
    numeric::positive("whole-family XY dot ruling", &bound, arithmetic)?;
    Ok(bound)
}
fn z_upper(
    sides: &[SideCapture],
    error: &Interval,
    gap: &Interval,
    sine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Interval> {
    let shifts = sides
        .iter()
        .map(|v| arithmetic.add(&v.shifts[0], &v.shifts[1]))
        .collect::<Checked<Vec<_>>>()?;
    let shift = numeric::hull(&shifts);
    let ideal = arithmetic.multiply(gap, sine)?;
    let bound = arithmetic.add(&numeric::neg(&ideal), error)?;
    let bound = arithmetic.add(&bound, &shift)?;
    if bound.upper >= 0.into() {
        return Err(error::unresolved(
            "whole-family Z ruling",
            &bound,
            "strictly negative",
            arithmetic.fractional_bits(),
        ));
    }
    Ok(bound)
}
fn family(
    sides: &[SideCapture],
    sine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<[Interval; 2]> {
    let bounds = family_extrema(sides);
    let mut margins = [arithmetic.zero(), arithmetic.zero()];
    margins[0] = lower_margin(&sides[0].depths[0], &bounds, sine, arithmetic)?;
    margins[1] = upper_margin(&sides[0].depths[1], &bounds, sine, arithmetic)?;
    for margin in &margins {
        numeric::positive("family side/all-cap barrier", margin, arithmetic)?;
    }
    Ok(margins)
}

struct Scale {
    radius: Interval,
    storage: Interval,
    error: Interval,
    gap: Interval,
}
fn scale(sides: &[SideCapture]) -> Scale {
    let radius = numeric::hull(&sides.iter().map(|v| v.radius.clone()).collect::<Vec<_>>());
    let storage = family_storage(sides);
    let error = numeric::max_magnitude(
        &sides
            .iter()
            .map(|v| v.ruling_error.clone())
            .collect::<Vec<_>>(),
    );
    let gap = numeric::hull(
        &sides
            .iter()
            .map(|v| v.depth_gap.clone())
            .collect::<Vec<_>>(),
    );
    Scale {
        radius,
        storage,
        error,
        gap,
    }
}
fn dv_bound(
    sides: &[SideCapture],
    scale: &Scale,
    sine: &Interval,
    cosine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Interval> {
    let dot = dot_lower(
        &scale.radius,
        &scale.storage,
        &scale.error,
        &scale.gap,
        cosine,
        arithmetic,
    )?;
    let dz = z_upper(sides, &scale.error, &scale.gap, sine, arithmetic)?;
    let rho = arithmetic.add(&numeric::upper_point(&scale.radius), &scale.storage)?;
    let numerator = arithmetic.multiply(cosine, &dot)?;
    let first = arithmetic.divide(&numerator, &rho)?;
    let second = arithmetic.multiply(sine, &dz)?;
    Ok(arithmetic.subtract(&first, &second)?)
}

struct FamilyExtrema {
    lower: num_bigint::BigInt,
    upper: num_bigint::BigInt,
    eta: [Interval; 2],
    shifts: [Interval; 2],
}
fn family_extrema(sides: &[SideCapture]) -> FamilyExtrema {
    let lower = sides
        .iter()
        .map(|v| v.cap_levels[0].upper.clone())
        .max()
        .unwrap_or_default();
    let upper = sides
        .iter()
        .map(|v| v.cap_levels[1].lower.clone())
        .min()
        .unwrap_or_default();
    let eta = std::array::from_fn(|row| parent_storage(sides, row));
    let shifts = std::array::from_fn(|row| minimum_shift(sides, row));
    FamilyExtrema {
        lower,
        upper,
        eta,
        shifts,
    }
}
fn lower_margin(
    depth: &Interval,
    bounds: &FamilyExtrema,
    sine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Interval> {
    let moved = arithmetic.multiply(&numeric::lower_point(sine), &bounds.shifts[0])?;
    let first = arithmetic.subtract(depth, &bounds.eta[0])?;
    let first = arithmetic.add(&first, &moved)?;
    let cap = Interval {
        lower: bounds.lower.clone(),
        upper: bounds.lower.clone(),
    };
    Ok(arithmetic.subtract(&first, &cap)?)
}
fn upper_margin(
    depth: &Interval,
    bounds: &FamilyExtrema,
    sine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Interval> {
    let moved = arithmetic.multiply(&numeric::lower_point(sine), &bounds.shifts[1])?;
    let last = arithmetic.add(depth, &bounds.eta[1])?;
    let last = arithmetic.subtract(&last, &moved)?;
    let cap = Interval {
        lower: bounds.upper.clone(),
        upper: bounds.upper.clone(),
    };
    Ok(arithmetic.subtract(&cap, &last)?)
}

fn family_storage(sides: &[SideCapture]) -> Interval {
    numeric::max_magnitude(
        &sides
            .iter()
            .flat_map(|v| v.storage.iter().cloned())
            .collect::<Vec<_>>(),
    )
}

fn parent_storage(sides: &[SideCapture], row: usize) -> Interval {
    numeric::max_magnitude(
        &sides
            .iter()
            .map(|v| v.parent_storage[row].clone())
            .collect::<Vec<_>>(),
    )
}
fn minimum_shift(sides: &[SideCapture], row: usize) -> Interval {
    numeric::lower_point(&numeric::hull(
        &sides
            .iter()
            .map(|v| v.shifts[row].clone())
            .collect::<Vec<_>>(),
    ))
}
