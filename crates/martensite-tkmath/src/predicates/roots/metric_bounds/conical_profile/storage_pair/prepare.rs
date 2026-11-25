// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::certificate_arithmetic::bernstein_bounds::{product, scale, sum};
pub(super) fn capture(
    request: &ConicalStorageQuery<'_>,
    domain: &ConicalChartDomainEvidence,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<PairStorage, Failure> {
    let tangent = storage::capture(request.tangent, arithmetic)?;
    let numerator = storage::capture(request.numerator, arithmetic)?;
    let weights = storage::capture(request.weights, arithmetic)?;
    let angular = storage::angular_polynomials(&tangent, request.center, arithmetic)?;
    let denominators = denominators(request, &weights, &angular[2], arithmetic)?;
    let radial = scale(&numerator, &domain.sine, arithmetic)?;
    let axial = scale(&numerator, &domain.cosine, arithmetic)?;
    Ok(PairStorage {
        angular,
        radial,
        axial,
        weights,
        denominators,
        sine: domain.sine.clone(),
        cosine: domain.cosine.clone(),
    })
}
fn denominators(
    request: &ConicalStorageQuery<'_>,
    weights: &[Interval],
    angular_weight: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<StorageDenominators, Failure> {
    let ideal_weight = product(weights, angular_weight, arithmetic)?;
    let stored_weight = storage::capture(request.stored_weights, arithmetic)?;
    let minimum = storage::positive_weight("source weight", weights, arithmetic)?;
    let stored_minimum = storage::positive_weight("stored weight", &stored_weight, arithmetic)?;
    let denominator = arithmetic.multiply(&minimum, &stored_minimum)?;
    let weight_error = bound::difference_bound(&ideal_weight, &stored_weight, arithmetic)?;
    Ok(StorageDenominators {
        ideal: ideal_weight,
        stored: stored_weight,
        stored_minimum,
        product: denominator,
        error: weight_error,
    })
}
pub(super) fn ideal_row(
    request: &ConicalStorageQuery<'_>,
    context: &PairStorage,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Coefficients, Failure> {
    let [radial, axial] = meridian(request, context, arithmetic)?;
    Ok([
        product(&radial, &context.angular[0], arithmetic)?,
        product(&radial, &context.angular[1], arithmetic)?,
        product(&axial, &context.angular[2], arithmetic)?,
        context.denominators.ideal.clone(),
    ])
}
fn meridian(
    request: &ConicalStorageQuery<'_>,
    context: &PairStorage,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[Vec<Interval>; 2], Failure> {
    let [dc, ds] = depth_offsets(request, context, arithmetic)?;
    let radial = sum(
        &context.radial,
        &scale(&context.weights, &dc, arithmetic)?,
        arithmetic,
    )?;
    let axial = crate::predicates::roots::certificate_arithmetic::bernstein_bounds::difference(
        &context.axial,
        &scale(&context.weights, &ds, arithmetic)?,
        arithmetic,
    )?;
    Ok([radial, axial])
}
fn depth_offsets(
    request: &ConicalStorageQuery<'_>,
    context: &PairStorage,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[Interval; 2], Failure> {
    let signed = if request.outward {
        request.depth
    } else {
        -request.depth
    };
    let depth = arithmetic.scalar(signed)?;
    let dc = arithmetic.multiply(&depth, &context.cosine)?;
    let ds = arithmetic.multiply(&depth, &context.sine)?;
    Ok([dc, ds])
}
pub(super) fn stored_row(
    request: &ConicalStorageQuery<'_>,
    weights: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Coefficients, Failure> {
    let mut result: Coefficients = std::array::from_fn(|_| Vec::with_capacity(weights.len()));
    for (pole, weight) in request.stored_poles.iter().zip(weights) {
        for axis in 0..3 {
            let coordinate = arithmetic.scalar(pole[axis])?;
            result[axis].push(arithmetic.multiply(&coordinate, weight)?);
        }
    }
    result[3] = weights.to_vec();
    Ok(result)
}
