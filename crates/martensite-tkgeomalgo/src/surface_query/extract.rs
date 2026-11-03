//! Direct extraction is admitted only after complete clamped/cyclic layout validation.
use super::types::{FaceQueryIncomplete as Cause, require};
use martensite_tkg3d::geom::StoredSplineSurfaceInput;
use martensite_tkmath::predicates::roots::surface_query::{SurfaceQueryInput, SurfaceQueryPatch};
pub(super) fn extract(
    input: &StoredSplineSurfaceInput,
    owner: usize,
) -> Result<SurfaceQueryInput, Cause> {
    let periodic = input.axes()[0].periodic();
    let count = if periodic { 3 } else { 1 };
    let patches = (0..count)
        .map(|span| patch(input, span, owner))
        .collect::<Result<_, _>>()?;
    Ok(SurfaceQueryInput {
        patches,
        periodic_u: periodic,
    })
}
fn patch(
    input: &StoredSplineSurfaceInput,
    span: usize,
    owner: usize,
) -> Result<SurfaceQueryPatch, Cause> {
    let degree = [
        input.axes()[0].degree() as usize,
        input.axes()[1].degree() as usize,
    ];
    let slots = patch_slots(input.shape(), degree, span);
    let poles = slots.iter().map(|index| input.poles()[*index]).collect();
    let weights = slots.iter().map(|index| input.weights()[*index]).collect();
    let weights = effective_weights(input, weights, owner)?;
    Ok(SurfaceQueryPatch {
        degree,
        poles,
        weights,
    })
}
fn patch_slots(shape: [usize; 2], degree: [usize; 2], span: usize) -> Vec<usize> {
    let mut slots = Vec::new();
    for u in 0..=degree[0] {
        for v in 0..=degree[1] {
            slots.push(((span * 2 + u) % shape[0]) * shape[1] + v);
        }
    }
    slots
}
fn effective_weights(
    input: &StoredSplineSurfaceInput,
    weights: Vec<f64>,
    owner: usize,
) -> Result<Vec<f64>, Cause> {
    if !input.rational().into_iter().any(|value| value) {
        return Ok(vec![1.; weights.len()]);
    }
    positive_weights(&weights, owner)?;
    let first = weights[0];
    if weights.iter().all(|value| *value == first) {
        return Ok(vec![1.; weights.len()]);
    }
    validate_rational_branch(&weights, first, owner)?;
    Ok(weights)
}
fn validate_rational_branch(weights: &[f64], first: f64, owner: usize) -> Result<(), Cause> {
    let epsilon = martensite_tkernel::standard::real::epsilon(first);
    require(
        weights.iter().any(|value| (*value - first).abs() > epsilon),
        owner,
        "unambiguous local rational evaluator branch",
        weights,
    )
}

fn positive_weights(weights: &[f64], owner: usize) -> Result<(), Cause> {
    require(
        weights.iter().all(|v| v.is_finite() && *v > 0.),
        owner,
        "positive finite rational evaluator weights",
        weights,
    )
}
