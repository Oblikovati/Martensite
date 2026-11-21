// SPDX-License-Identifier: GPL-2.0-only
//! Exact-parameter point witnesses refute a margin before adaptive search is extended.
use super::*;
use crate::predicates::roots::DyadicBound;
use context::Context;

pub(super) fn refuse_if_close(
    context: &mut Context<'_, '_>,
    sources: [usize; 2],
    nodes: [usize; 2],
) -> Checked<()> {
    let a = cache::samples(context, sources[0], nodes[0])?;
    let b = cache::samples(context, sources[1], nodes[1])?;
    for (i, x) in a.iter().enumerate() {
        for (j, z) in b.iter().enumerate() {
            compare(context, sources, nodes, [i, j], x, z)?;
        }
    }
    Ok(())
}

fn compare(
    context: &mut Context<'_, '_>,
    sources: [usize; 2],
    nodes: [usize; 2],
    samples: [usize; 2],
    a: &Point,
    b: &Point,
) -> Checked<()> {
    let x = difference(a, b, &context.shift, &mut context.arithmetic)?;
    let square = projection::dot(&x, &x, &mut context.arithmetic)?;
    if square.upper >= context.delta_square.lower {
        return Ok(());
    }
    let parameters =
        std::array::from_fn(|i| cache::parameter(context, sources[i], nodes[i], samples[i]));
    Err(CylindricalClearanceFailure::WitnessWithinMargin {
        sources,
        parameters: Box::new(parameters),
        squared_distance_upper: DyadicBound::new(
            square.upper,
            -i32::from(context.query.options.fractional_bits),
        ),
    })
}

fn difference(
    a: &Point,
    b: &Point,
    shift: &Point,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Point> {
    let x = arithmetic.subtract(&a[0], &b[0])?;
    let x = arithmetic.subtract(&x, &shift[0])?;
    let z = arithmetic.subtract(&a[1], &b[1])?;
    let z = arithmetic.subtract(&z, &shift[1])?;
    Ok([x, z])
}
