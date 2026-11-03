//! A positive symmetric projected Jacobian proves strict monotonicity on a convex square.
use super::{
    budget::Budget,
    hulls::positive,
    polynomial::{Polynomial, integer},
    prepare::Homogeneous,
    types::*,
};
pub(super) fn derivative(
    h: &Homogeneous,
    coordinate: usize,
    axis: usize,
    budget: &mut Budget<'_>,
) -> Result<Polynomial, SurfaceQueryCause> {
    let first = h[coordinate]
        .derivative(axis, budget)?
        .multiply(&h[3], budget)?;
    let second = h[coordinate].multiply(&h[3].derivative(axis, budget)?, budget)?;
    first.subtract(&second, budget)
}
fn candidate(
    h: &Homogeneous,
    coordinates: [usize; 2],
    signs: [i32; 2],
    budget: &mut Budget<'_>,
) -> Result<bool, SurfaceQueryCause> {
    let a = derivative(h, coordinates[0], 0, budget)?.scale(&integer(signs[0]), budget)?;
    let d = derivative(h, coordinates[1], 1, budget)?.scale(&integer(signs[1]), budget)?;
    if !positive(&a, budget)? || !positive(&d, budget)? {
        return Ok(false);
    }
    let b = derivative(h, coordinates[0], 1, budget)?.scale(&integer(signs[0]), budget)?;
    let c = derivative(h, coordinates[1], 0, budget)?.scale(&integer(signs[1]), budget)?;
    let mixed = b.add(&c, budget)?;
    let determinant = a
        .multiply(&d, budget)?
        .scale(&integer(4), budget)?
        .subtract(&mixed.multiply(&mixed, budget)?, budget)?;
    positive(&determinant, budget)
}
pub(super) fn prove(
    h: &Homogeneous,
    budget: &mut Budget<'_>,
) -> Result<SurfaceQueryTheorem, SurfaceQueryCause> {
    if !positive(&h[3], budget)? {
        return Err(unproved("positive denominator"));
    }
    for a in 0..3 {
        for b in 0..3 {
            if a == b {
                continue;
            }
            if let Some(theorem) = signed_projection(h, [a, b], budget)? {
                return Ok(theorem);
            }
        }
    }
    Err(unproved("strict symmetric projected Jacobian"))
}
fn unproved(relation: &'static str) -> SurfaceQueryCause {
    SurfaceQueryCause::Unproved { patch: 0, relation }
}
fn signed_projection(
    h: &Homogeneous,
    coordinates: [usize; 2],
    budget: &mut Budget<'_>,
) -> Result<Option<SurfaceQueryTheorem>, SurfaceQueryCause> {
    for first in [-1, 1] {
        for second in [-1, 1] {
            let signs = [first, second];
            if candidate(h, coordinates, signs, budget)? {
                return Ok(Some(SurfaceQueryTheorem::StrictProjection {
                    coordinates,
                    signs,
                }));
            }
        }
    }
    Ok(None)
}
