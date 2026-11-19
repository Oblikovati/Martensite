// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) fn capture(
    query: &ConicalBoundaryQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<FitCapture> {
    let p = numeric::capture(query.angular_fit, arithmetic)?;
    let numerator = numeric::capture(query.slant_numerator, arithmetic)?;
    let weights = numeric::capture(query.slant_weights, arithmetic)?;
    source_shape(&p, &numerator, &weights)?;
    let p_hull = numeric::hull(&p);
    let weight_hull = numeric::hull(&weights);
    numeric::positive("source denominator", &weight_hull, arithmetic)?;
    let square = numeric::square(&p_hull, arithmetic)?;
    let one = arithmetic.scalar(1.)?;
    let b = arithmetic.add(&one, &square)?;
    let p_rate = derivative(&p, arithmetic)?;
    Ok(FitCapture {
        p,
        numerator,
        weights,
        p_hull,
        weight_hull,
        b,
        p_rate,
    })
}
pub(super) fn assemble(
    query: &ConicalActualSideQuery<'_>,
    row: &super::super::super::boundary_transition::DirectedRowCapture,
    gamma: &GammaSpanCapture,
    fit: FitCapture,
    errors: [Interval; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<SideCapture> {
    let geometry = geometry(&fit, gamma, arithmetic)?;
    let coordinates = coordinates(query, row, arithmetic)?;
    Ok(assemble_capture(
        query,
        row,
        gamma,
        fit,
        (geometry, coordinates),
        errors,
        arithmetic.zero(),
    ))
}
fn slant_rate(
    fit: &FitCapture,
    slant: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Interval> {
    let numerator_rate = derivative(&fit.numerator, arithmetic)?;
    let weight_rate = derivative(&fit.weights, arithmetic)?;
    let first = arithmetic.divide(&numeric::hull(&numerator_rate), &fit.weight_hull)?;
    let second = arithmetic.multiply(slant, &numeric::hull(&weight_rate))?;
    let second = arithmetic.divide(&second, &fit.weight_hull)?;
    Ok(arithmetic.subtract(&first, &second)?)
}
fn coordinates(
    query: &ConicalActualSideQuery<'_>,
    row: &super::super::super::boundary_transition::DirectedRowCapture,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<[Vec<[Interval; 3]>; 2]> {
    let mut result: [Vec<[Interval; 3]>; 2] = std::array::from_fn(|_| Vec::new());
    for (depth, target) in result.iter_mut().enumerate() {
        coordinate_row(query, depth, &row.new_z[depth], target, arithmetic)?;
    }
    Ok(result)
}

struct Geometry {
    slant_rate: Interval,
    angle_rate: Interval,
    radius: Interval,
    depth_gap: Interval,
    depths: [Interval; 2],
}
fn source_shape(p: &[Interval], numerator: &[Interval], weights: &[Interval]) -> Outcome<()> {
    if p.is_empty() || numerator.is_empty() || weights.is_empty() {
        return Err(error::shape(
            "q/N/W",
            0,
            "nonempty admitted source coefficients",
        ));
    }
    Ok(())
}
fn geometry(
    fit: &FitCapture,
    gamma: &GammaSpanCapture,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Geometry> {
    let slant = arithmetic.divide(&numeric::hull(&fit.numerator), &fit.weight_hull)?;
    let slant_rate = slant_rate(fit, &slant, arithmetic)?;
    let angle_rate = arithmetic.fraction(&numeric::hull(&fit.p_rate), 2, 1)?;
    let angle_rate = arithmetic.divide(&angle_rate, &fit.b)?;
    let depths = gamma.depths.clone();
    let radius = radius(&slant, &depths, gamma, arithmetic)?;
    let depth_gap = arithmetic.subtract(&depths[1], &depths[0])?;
    Ok(Geometry {
        slant_rate,
        angle_rate,
        radius,
        depth_gap,
        depths,
    })
}
fn radius(
    slant: &Interval,
    depths: &[Interval; 2],
    gamma: &GammaSpanCapture,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Interval> {
    let depth_range = numeric::hull(depths);
    let radial = arithmetic.multiply(slant, &gamma.sine)?;
    let offset = arithmetic.multiply(&depth_range, &gamma.cosine)?;
    let radius = arithmetic.add(&radial, &offset)?;
    numeric::positive("ideal normal radius", &radius, arithmetic)?;
    Ok(radius)
}
fn assemble_capture(
    query: &ConicalActualSideQuery<'_>,
    row: &super::super::super::boundary_transition::DirectedRowCapture,
    gamma: &GammaSpanCapture,
    fit: FitCapture,
    captured: (Geometry, [Vec<[Interval; 3]>; 2]),
    errors: [Interval; 2],
    du_error: Interval,
) -> SideCapture {
    let (geometry, coordinates) = captured;
    let [derivative_error, ruling_error] = errors;
    SideCapture {
        source: snapshot::side(query),
        coordinates,
        p: fit.p_hull,
        slant_rate: geometry.slant_rate,
        angle_denominator: fit.b,
        angle_rate: geometry.angle_rate,
        radius: geometry.radius,
        depth_gap: geometry.depth_gap,
        depths: geometry.depths,
        storage: row.storage.clone(),
        shifts: row.signed_shifts.clone(),
        derivative_error,
        ruling_error,
        du_error,
        parent_storage: row.parent_storage.clone(),
        cap_levels: gamma.cap_levels.clone(),
    }
}

fn coordinate_row(
    query: &ConicalActualSideQuery<'_>,
    depth: usize,
    z: &[Interval],
    target: &mut Vec<[Interval; 3]>,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<()> {
    if z.len() != query.current.edges[depth].stored_poles.len() {
        return Err(error::shape(
            "final row capture",
            z.len(),
            "one captured Z per actual pole",
        ));
    }
    for (pole, z) in query.current.edges[depth].stored_poles.iter().zip(z) {
        target.push([
            arithmetic.scalar(pole[0])?,
            arithmetic.scalar(pole[1])?,
            z.clone(),
        ]);
    }
    Ok(())
}
