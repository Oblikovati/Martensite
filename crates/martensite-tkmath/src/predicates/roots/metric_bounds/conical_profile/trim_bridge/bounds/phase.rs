// SPDX-License-Identifier: GPL-2.0-only
use super::*;
mod conditioned;
#[cfg(test)]
mod tests;
#[cfg(test)]
pub(in super::super) fn bound(
    u: &[Interval],
    angular_fit: &[f64],
    fit_phase: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, CylindricalProfileFailure> {
    bound_selected(
        u,
        angular_fit,
        fit_phase,
        ConicalPrimitiveRefinement::UnsplitV1,
        arithmetic,
    )
}
pub(in super::super) fn bound_selected(
    u: &[Interval],
    angular_fit: &[f64],
    fit_phase: &Interval,
    refinement: ConicalPrimitiveRefinement,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, CylindricalProfileFailure> {
    // #553: the retained source squared radius is not the angular half-angle fit.
    let tangent = angular_fit
        .iter()
        .map(|value| arithmetic.scalar(*value))
        .collect::<Checked<Vec<_>>>()?;
    let residual = native_angular_residual(u, &tangent, arithmetic)?;
    let anchor = arithmetic.subtract(&u[0], fit_phase)?;
    conditioned::bound_selected(&residual, &anchor, refinement, arithmetic)
}
struct AngularResidual {
    numerator: Vec<Interval>,
    denominator: Vec<Interval>,
    unit: Interval,
}

fn native_angular_residual(
    u: &[Interval],
    tangent: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<AngularResidual, CylindricalProfileFailure> {
    let one = arithmetic.scalar(1.)?;
    let q2 = polynomial::product(tangent, tangent, arithmetic)?;
    let base = q2
        .iter()
        .map(|q| arithmetic.add(&one, q))
        .collect::<Checked<Vec<_>>>()?;
    let du = derivative(u, arithmetic)?;
    let first = polynomial::product(&du, &base, arithmetic)?;
    let dq = derivative(tangent, arithmetic)?;
    let two = arithmetic.scalar(2.)?;
    let second = scaled(&dq, &two, arithmetic)?;
    let residual = algebra::difference(&first, &second, arithmetic)?;
    Ok(AngularResidual {
        numerator: residual,
        denominator: base,
        unit: one,
    })
}
