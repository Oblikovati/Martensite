// SPDX-License-Identifier: GPL-2.0-only
//! One operation-local rational storage preparation, with independently checked depth rows.
use super::combined::{CombinedFailure, CombinedRequest};
use super::storage::{Coefficients, ConicalStorageFailure, ConicalStorageQuery};
use super::*;
mod bound;
mod prepare;
#[cfg(test)]
mod tests;
type Failure = ConicalStorageFailure;
struct PairStorage {
    angular: [Vec<Interval>; 3],
    radial: Vec<Interval>,
    axial: Vec<Interval>,
    weights: Vec<Interval>,
    sine: Interval,
    cosine: Interval,
    denominators: StorageDenominators,
}
struct StorageDenominators {
    ideal: Vec<Interval>,
    stored: Vec<Interval>,
    stored_minimum: Interval,
    product: Interval,
    error: Option<Interval>,
}
pub(super) fn certify(
    request: &CombinedRequest<'_>,
    domain: &ConicalChartDomainEvidence,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[Interval; 2], CombinedFailure> {
    let first = &request.edges[0];
    storage::admit_storage(first).map_err(|cause| at_depth(0, cause))?;
    let context =
        prepare::capture(first, domain, arithmetic).map_err(|cause| at_depth(0, cause))?;
    let cutoff = cutoff(request.spatial.limit, arithmetic).map_err(|cause| at_depth(0, cause))?;
    let first =
        discrepancy(first, &context, &cutoff, arithmetic).map_err(|cause| at_depth(0, cause))?;
    storage::admit_storage(&request.edges[1]).map_err(|cause| at_depth(1, cause))?;
    let last = discrepancy(&request.edges[1], &context, &cutoff, arithmetic)
        .map_err(|cause| at_depth(1, cause))?;
    Ok([first, last])
}
fn at_depth(depth: usize, cause: Failure) -> CombinedFailure {
    CombinedFailure::Storage { depth, cause }
}
fn discrepancy(
    request: &ConicalStorageQuery<'_>,
    context: &PairStorage,
    cutoff: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, Failure> {
    let ideal = prepare::ideal_row(request, context, arithmetic)?;
    let stored = prepare::stored_row(request, &context.denominators.stored, arithmetic)?;
    if let Some(bound) = bound::conditioned(&ideal, &stored, context, cutoff, arithmetic)? {
        return Ok(bound);
    }
    let total = storage::residual_sum(&ideal, &stored, arithmetic)?;
    Ok(arithmetic.divide(&total, &context.denominators.product)?)
}

fn cutoff(limit: f64, arithmetic: &mut Arithmetic<'_>) -> Result<Interval, Failure> {
    let limit = arithmetic.scalar(limit)?;
    Ok(arithmetic.fraction(&limit, 1, 1000)?)
}
