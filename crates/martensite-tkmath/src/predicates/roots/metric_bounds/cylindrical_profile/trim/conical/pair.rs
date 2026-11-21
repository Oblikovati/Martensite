// SPDX-License-Identifier: GPL-2.0-only
//! Finish and accept the first trace before admitting any second-trace input.
use super::capture::{CapturedTrace, Reuse};
use super::*;

/// Certify both actual traces under one work limit, reusing identical coefficient inputs.
/// The first proof completes before second-trace admission; either error publishes no pair.
/// Example: `let pair = certify_conical_trim_pair(query, || false)?;`.
pub fn certify_conical_trim_pair(
    query: ConicalTrimPairQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedConicalTrimPair, ConicalTrimPairError> {
    let [first, second] = query.queries();
    admit_trace(first, 0, 0)?;
    let policy = ArithmeticPolicy {
        fractional_bits: query.options.fractional_bits,
        max_operations: query.options.max_operations,
    };
    let mut arithmetic = Arithmetic::new(policy, &mut cancelled);
    finish_pair(first, second, &mut arithmetic)
}

fn finish_pair(
    first: ConicalTrimQuery<'_>,
    second: ConicalTrimQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<CertifiedConicalTrimPair, ConicalTrimPairError> {
    let (first_trace, captured) = compute_trace(first, 0, None, arithmetic)?;
    admit_trace(second, 1, arithmetic.operations())?;
    let reuse = Reuse {
        query: first,
        captured: &captured,
    };
    let (second_trace, _) = compute_trace(second, 1, Some(reuse), arithmetic)?;
    Ok(CertifiedConicalTrimPair {
        traces: [first_trace, second_trace],
        operations: arithmetic.operations(),
    })
}

fn admit_trace(
    query: ConicalTrimQuery<'_>,
    depth: usize,
    operations: u32,
) -> Result<(), ConicalTrimPairError> {
    admit(query).map_err(|cause| trace_error(query, depth, operations, cause))
}

fn compute_trace(
    query: ConicalTrimQuery<'_>,
    depth: usize,
    reuse: Option<Reuse<'_, '_>>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(CertifiedConicalTrim, CapturedTrace), ConicalTrimPairError> {
    let start = arithmetic.operations();
    let (bounds, captured) = capture::residual(query, reuse, arithmetic)
        .map_err(|cause| trace_error(query, depth, arithmetic.operations(), cause))?;
    accept_position(bounds[2], query.spatial_limit)
        .map_err(|cause| trace_error(query, depth, arithmetic.operations(), cause))?;
    let trace = CertifiedConicalTrim {
        bounds,
        bits: types::source_bits(query),
        limit: query.spatial_limit,
        options: query.options,
        operations: arithmetic.operations() - start,
    };
    Ok((trace, captured))
}

fn trace_error(
    query: ConicalTrimQuery<'_>,
    depth: usize,
    operations: u32,
    cause: CylindricalProfileFailure,
) -> ConicalTrimPairError {
    ConicalTrimPairError {
        depth,
        operations,
        cause: Box::new(ConicalTrimError {
            carrier: query.carrier,
            requested_spatial_limit: query.spatial_limit,
            requested_options: query.options,
            cause,
        }),
    }
}

#[cfg(test)]
mod tests;
