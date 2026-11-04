//! Axial separation and a degree-one angular cycle prove injectivity modulo U aliases.
use super::{
    budget::Budget,
    hulls::{endpoint_positive, positive},
    polynomial::{Polynomial, integer},
    prepare::Homogeneous,
    projection::derivative,
    types::*,
};
fn unproved(patch: usize, relation: &'static str) -> SurfaceQueryCause {
    SurfaceQueryCause::Unproved { patch, relation }
}
fn halfplane(h: &Homogeneous, budget: &mut Budget<'_>) -> Result<bool, SurfaceQueryCause> {
    for a in [-1, 0, 1] {
        for b in [-1, 0, 1] {
            if a == 0 && b == 0 {
                continue;
            }
            let sum = h[0]
                .scale(&integer(a), budget)?
                .add(&h[1].scale(&integer(b), budget)?, budget)?;
            if positive(&sum, budget)? {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
fn span(h: &Homogeneous, index: usize, budget: &mut Budget<'_>) -> Result<(), SurfaceQueryCause> {
    if !positive(&h[3], budget)? {
        return Err(unproved(index, "positive denominator"));
    }
    if !derivative(h, 2, 0, budget)?.0.is_empty() {
        return Err(unproved(index, "U-independent axial coordinate"));
    }
    if !endpoint_positive(&derivative(h, 2, 1, budget)?, budget)? {
        return Err(unproved(index, "strict axial order"));
    }
    angular_span(h, index, budget)
}
fn angular_span(
    h: &Homogeneous,
    index: usize,
    budget: &mut Budget<'_>,
) -> Result<(), SurfaceQueryCause> {
    let cross = h[0]
        .multiply(&h[1].derivative(0, budget)?, budget)?
        .subtract(
            &h[1].multiply(&h[0].derivative(0, budget)?, budget)?,
            budget,
        )?;
    if !positive(&cross, budget)? {
        return Err(unproved(index, "positive angular derivative"));
    }
    if !halfplane(h, budget)? {
        return Err(unproved(index, "axis-excluding angular halfplane"));
    }
    Ok(())
}
fn endpoint(
    h: &Homogeneous,
    side: usize,
    budget: &mut Budget<'_>,
) -> Result<Homogeneous, SurfaceQueryCause> {
    Ok([
        h[0].endpoint(0, side, budget)?,
        h[1].endpoint(0, side, budget)?,
        h[2].endpoint(0, side, budget)?,
        h[3].endpoint(0, side, budget)?,
    ])
}
fn joined(
    left: &Homogeneous,
    right: &Homogeneous,
    budget: &mut Budget<'_>,
) -> Result<bool, SurfaceQueryCause> {
    let left = endpoint(left, 1, budget)?;
    let right = endpoint(right, 0, budget)?;
    for axis in 0..3 {
        let residual = left[axis]
            .multiply(&right[3], budget)?
            .subtract(&right[axis].multiply(&left[3], budget)?, budget)?;
        if !residual.0.is_empty() {
            return Ok(false);
        }
    }
    Ok(true)
}
fn vertex(h: &Homogeneous, budget: &mut Budget<'_>) -> Result<[Polynomial; 2], SurfaceQueryCause> {
    Ok([
        h[0].endpoint(0, 0, budget)?.endpoint(1, 0, budget)?,
        h[1].endpoint(0, 0, budget)?.endpoint(1, 0, budget)?,
    ])
}
fn winding(hs: &[Homogeneous], budget: &mut Budget<'_>) -> Result<(), SurfaceQueryCause> {
    for i in 0..3 {
        let a = vertex(&hs[i], budget)?;
        let b = vertex(&hs[(i + 1) % 3], budget)?;
        let cross = a[0]
            .multiply(&b[1], budget)?
            .subtract(&a[1].multiply(&b[0], budget)?, budget)?;
        if !positive(&cross, budget)? {
            return Err(unproved(i, "origin-containing endpoint triangle"));
        }
    }
    Ok(())
}
pub(super) fn prove(
    hs: &[Homogeneous],
    budget: &mut Budget<'_>,
) -> Result<SurfaceQueryTheorem, SurfaceQueryCause> {
    for (index, h) in hs.iter().enumerate() {
        span(h, index, budget)?;
        if !joined(h, &hs[(index + 1) % hs.len()], budget)? {
            return Err(unproved(index, "exact complete rational endpoint gluing"));
        }
    }
    winding(hs, budget)?;
    Ok(SurfaceQueryTheorem::AxialAngularCycle)
}
