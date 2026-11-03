//! Bounded low-degree coefficient construction never samples the supplied map.
use super::{
    Ratio,
    budget::Budget,
    polynomial::{Polynomial, integer},
    types::*,
};
pub(super) type Homogeneous = [Polynomial; 4];
fn basis(degree: usize, index: usize) -> &'static [i32] {
    match (degree, index) {
        (1, 0) => &[1, -1],
        (1, 1) => &[0, 1],
        (2, 0) => &[1, -2, 1],
        (2, 1) => &[0, 2, -2],
        (2, 2) => &[0, 0, 1],
        _ => unreachable!("validated degree one or two"),
    }
}
pub(super) fn validate(input: &SurfaceQueryInput) -> Result<(), SurfaceQueryCause> {
    let expected = if input.periodic_u { 3 } else { 1 };
    if input.patches.len() != expected {
        return Err(SurfaceQueryCause::Layout {
            patch: 0,
            field: "patch count",
            actual: input.patches.len(),
        });
    }
    for (index, patch) in input.patches.iter().enumerate() {
        validate_patch(patch, index)?;
    }
    Ok(())
}
fn validate_patch(patch: &SurfaceQueryPatch, index: usize) -> Result<(), SurfaceQueryCause> {
    if patch.degree.iter().any(|degree| !(1..=2).contains(degree)) {
        return Err(layout(
            index,
            "degree",
            patch.degree[0].max(patch.degree[1]),
        ));
    }
    let count = (patch.degree[0] + 1) * (patch.degree[1] + 1);
    for (field, actual) in [
        ("poles", patch.poles.len()),
        ("weights", patch.weights.len()),
    ] {
        if actual != count {
            return Err(layout(index, field, actual));
        }
    }
    validate_numbers(patch, index)
}
fn layout(patch: usize, field: &'static str, actual: usize) -> SurfaceQueryCause {
    SurfaceQueryCause::Layout {
        patch,
        field,
        actual,
    }
}
fn validate_numbers(patch: &SurfaceQueryPatch, index: usize) -> Result<(), SurfaceQueryCause> {
    for (i, pole) in patch.poles.iter().enumerate() {
        for value in pole {
            if !value.is_finite() {
                return Err(numeric(index, "pole", i, *value));
            }
        }
    }
    for (i, value) in patch.weights.iter().enumerate() {
        if !value.is_finite() || *value <= 0. {
            return Err(numeric(index, "weight", i, *value));
        }
    }
    Ok(())
}
fn numeric(patch: usize, field: &'static str, index: usize, value: f64) -> SurfaceQueryCause {
    SurfaceQueryCause::Numeric {
        patch,
        field,
        index,
        value,
    }
}
pub(super) fn homogeneous(
    patch: &SurfaceQueryPatch,
    budget: &mut Budget<'_>,
) -> Result<Homogeneous, SurfaceQueryCause> {
    let mut result = std::array::from_fn(|_| Polynomial::default());
    for u in 0..=patch.degree[0] {
        for v in 0..=patch.degree[1] {
            add_pole(&mut result, patch, [u, v], budget)?;
        }
    }
    Ok(result)
}
fn add_pole(
    result: &mut Homogeneous,
    patch: &SurfaceQueryPatch,
    index: [usize; 2],
    budget: &mut Budget<'_>,
) -> Result<(), SurfaceQueryCause> {
    let slot = index[0] * (patch.degree[1] + 1) + index[1];
    let weight = budget.retain(Ratio::from_finite(patch.weights[slot]))?;
    let mut coefficient = [integer(0), integer(0), integer(0), weight.clone()];
    for (axis, value) in coefficient.iter_mut().enumerate().take(3) {
        budget.step()?;
        *value = budget.retain(Ratio::from_finite(patch.poles[slot][axis]).multiply(&weight))?;
    }
    for (axis, value) in coefficient.iter().enumerate() {
        add_basis(&mut result[axis], value, patch.degree, index, budget)?;
    }
    Ok(())
}
fn add_basis(
    result: &mut Polynomial,
    value: &Ratio,
    degree: [usize; 2],
    index: [usize; 2],
    budget: &mut Budget<'_>,
) -> Result<(), SurfaceQueryCause> {
    for (u, a) in basis(degree[0], index[0]).iter().enumerate() {
        for (v, b) in basis(degree[1], index[1]).iter().enumerate() {
            budget.step()?;
            let term = budget.retain(value.multiply(&integer(a * b)))?;
            let sum = result
                .0
                .get(&[u, v])
                .map_or_else(|| term.clone(), |old| old.add(&term));
            result.insert([u, v], budget.retain(sum)?);
        }
    }
    Ok(())
}
