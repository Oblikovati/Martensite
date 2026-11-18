// SPDX-License-Identifier: GPL-2.0-only
//! Cauchy-Schwarz separation uses squared quantities, with no square-root estimate.
use super::*;
use context::Context;
use num_bigint::{BigInt, Sign};

pub(super) fn separated(
    context: &mut Context<'_, '_>,
    sources: [usize; 2],
    nodes: [usize; 2],
) -> Checked<bool> {
    let hulls = [
        context.curves[sources[0]].nodes[nodes[0]].poles.clone(),
        context.curves[sources[1]].nodes[nodes[1]].poles.clone(),
    ];
    for normal in hulls::candidate_directions(&hulls, context.query.options.fractional_bits) {
        if direction(context, normal, &hulls)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn direction(
    context: &mut Context<'_, '_>,
    normal: [f64; 2],
    hulls: &[Vec<Point>; 2],
) -> Checked<bool> {
    let n = [
        context.arithmetic.scalar(normal[0])?,
        context.arithmetic.scalar(normal[1])?,
    ];
    let a = range(&hulls[0], &n, &mut context.arithmetic)?;
    let b = range(&hulls[1], &n, &mut context.arithmetic)?;
    let shift = dot(&context.shift, &n, &mut context.arithmetic)?;
    let b = context.arithmetic.add(&b, &shift)?;
    let gap = (&a.lower - &b.upper).max(&b.lower - &a.upper);
    if gap.sign() != Sign::Plus {
        return Ok(false);
    }
    meets_squared_clearance(context, &n, gap)
}

#[inline(always)]
fn meets_squared_clearance(context: &mut Context<'_, '_>, n: &Point, gap: BigInt) -> Checked<bool> {
    let gap = Interval {
        lower: gap.clone(),
        upper: gap,
    };
    let square = context.arithmetic.multiply(&gap, &gap)?;
    let norm_square = dot(n, n, &mut context.arithmetic)?;
    let threshold = context
        .arithmetic
        .multiply(&context.delta_square, &norm_square)?;
    Ok(square.lower >= threshold.upper)
}

fn range(poles: &[Point], n: &Point, arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
    let mut result = dot(&poles[0], n, arithmetic)?;
    for pole in &poles[1..] {
        let value = dot(pole, n, arithmetic)?;
        result.lower = result.lower.min(value.lower);
        result.upper = result.upper.max(value.upper);
    }
    Ok(result)
}

pub(super) fn dot(a: &Point, b: &Point, arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
    let x = arithmetic.multiply(&a[0], &b[0])?;
    let z = arithmetic.multiply(&a[1], &b[1])?;
    Ok(arithmetic.add(&x, &z)?)
}
