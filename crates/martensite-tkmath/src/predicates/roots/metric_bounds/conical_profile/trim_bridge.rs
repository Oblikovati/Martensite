// SPDX-License-Identifier: GPL-2.0-only
//! Checked decoded traces reuse the executed chart metric and actual storage bound.
use super::super::cylindrical_profile::trim::conical as trace;
use super::super::{
    CertifiedConicalTrim, CertifiedConicalTrimPair, ConicalTrimError, ConicalTrimPairQuery,
    ConicalTrimQuery, CylindricalProfileFailure,
};
use super::*;
mod binding;
pub(super) mod bounds;
mod error;
pub use error::{ConicalChartTrimPairError, ConicalChartTrimPairFailure};
#[cfg(test)]
mod tests;

/// Certify actual decoded cone traces against a fully bound executed chart receipt.
/// Existing generic trim certification remains independently selectable.
/// Example: `certify_conical_trim_pair_in_chart(pair, source, &boundary, &domain, || false)?`.
pub fn certify_conical_trim_pair_in_chart(
    pair: ConicalTrimPairQuery<'_>,
    source: ConicalBoundaryQuery<'_>,
    boundary: &ConicalBoundaryEvidence,
    domain: &ConicalChartDomainEvidence,
    cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedConicalTrimPair, ConicalChartTrimPairError> {
    certify_conical_trim_pair_in_chart_with_refinement(
        pair,
        source,
        boundary,
        domain,
        ConicalPrimitiveRefinement::UnsplitV1,
        cancelled,
    )
}

/// Certify decoded traces with the explicitly selected primitive refinement.
/// Example: `certify_conical_trim_pair_in_chart_with_refinement(pair, source, &boundary, &domain, refinement, || false)?`.
pub fn certify_conical_trim_pair_in_chart_with_refinement(
    pair: ConicalTrimPairQuery<'_>,
    source: ConicalBoundaryQuery<'_>,
    boundary: &ConicalBoundaryEvidence,
    domain: &ConicalChartDomainEvidence,
    refinement: ConicalPrimitiveRefinement,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedConicalTrimPair, ConicalChartTrimPairError> {
    run(
        pair,
        source,
        boundary,
        (domain, refinement),
        &mut cancelled,
        |traces, _, _, arithmetic| trace::accepted_pair(traces, arithmetic.operations()),
    )
}

/// Retain actual decoded angular/slant operands for the filled-material comparison.
/// The returned pair reports all actual work; the immutable operand capture adds no arithmetic.
/// Example: `certify_conical_trim_pair_with_developed_boundary(pair, source, &boundary, &domain, reference, || false)?`.
pub fn certify_conical_trim_pair_with_developed_boundary(
    pair: ConicalTrimPairQuery<'_>,
    source: ConicalBoundaryQuery<'_>,
    boundary: &ConicalBoundaryEvidence,
    domain: &ConicalChartDomainEvidence,
    reference_slant: f64,
    cancelled: impl FnMut() -> bool + Send,
) -> Result<(CertifiedConicalTrimPair, ConicalDevelopedBoundaryEvidence), ConicalChartTrimPairError>
{
    certify_conical_trim_pair_with_developed_boundary_and_refinement(
        pair,
        source,
        boundary,
        domain,
        reference_slant,
        ConicalPrimitiveRefinement::UnsplitV1,
        cancelled,
    )
}

/// Retain decoded operands under an explicit primitive refinement.
/// Example: `certify_conical_trim_pair_with_developed_boundary_and_refinement(pair, source, &boundary, &domain, reference, refinement, || false)?`.
pub fn certify_conical_trim_pair_with_developed_boundary_and_refinement(
    pair: ConicalTrimPairQuery<'_>,
    source: ConicalBoundaryQuery<'_>,
    boundary: &ConicalBoundaryEvidence,
    domain: &ConicalChartDomainEvidence,
    reference_slant: f64,
    refinement: ConicalPrimitiveRefinement,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<(CertifiedConicalTrimPair, ConicalDevelopedBoundaryEvidence), ConicalChartTrimPairError>
{
    run(
        pair,
        source,
        boundary,
        (domain, refinement),
        &mut cancelled,
        |traces, common, _, arithmetic| {
            let operations = arithmetic.operations();
            let evidence = super::developed_boundary::capture(
                pair,
                source,
                boundary,
                (domain, refinement),
                reference_slant,
                operations,
                common.into_retained(),
            );
            (trace::accepted_pair(traces, operations), evidence)
        },
    )
}
fn run<'a, T>(
    pair: ConicalTrimPairQuery<'a>,
    source: ConicalBoundaryQuery<'_>,
    boundary: &ConicalBoundaryEvidence,
    selection: (&ConicalChartDomainEvidence, ConicalPrimitiveRefinement),
    cancelled: &mut (dyn FnMut() -> bool + Send),
    finish: impl FnOnce(
        [CertifiedConicalTrim; 2],
        bounds::CommonTrace<'a>,
        &bounds::Bridge<'_, '_>,
        &mut Arithmetic<'_>,
    ) -> T,
) -> Result<T, ConicalChartTrimPairError> {
    let (domain, refinement) = selection;
    let first = pair.queries()[0];
    trace::admit(first).map_err(|cause| error::trace(first, 0, 0, cause))?;
    let captured = binding::retained_selected(
        &source,
        boundary,
        domain,
        pair.options.fractional_bits,
        refinement,
    )?;
    let policy = ArithmeticPolicy {
        fractional_bits: pair.options.fractional_bits,
        max_operations: pair.options.max_operations,
    };
    let mut arithmetic = Arithmetic::new(policy, cancelled);
    let input = bounds::Bridge::new(source, boundary, domain, captured, refinement);
    let (traces, common) = capture_pair(pair, &input, &mut arithmetic)?;
    Ok(finish(traces, common, &input, &mut arithmetic))
}
fn capture_pair<'a>(
    pair: ConicalTrimPairQuery<'a>,
    input: &bounds::Bridge<'_, '_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<([CertifiedConicalTrim; 2], bounds::CommonTrace<'a>), ConicalChartTrimPairError> {
    let [first, second] = pair.queries();
    let (first, common) = checked_trace(first, 0, input, arithmetic, |arithmetic| {
        bounds::residual(first, 0, input, arithmetic)
    })?;
    trace::admit(second)
        .map_err(|cause| error::trace(second, 1, arithmetic.operations(), cause))?;
    let (second, _) = checked_trace(second, 1, input, arithmetic, |arithmetic| {
        bounds::second_residual(second, 1, input, &common, arithmetic)
    })?;
    Ok(([first, second], common))
}

fn checked_trace<T>(
    query: ConicalTrimQuery<'_>,
    depth: usize,
    input: &bounds::Bridge<'_, '_>,
    arithmetic: &mut Arithmetic<'_>,
    compute: impl FnOnce(&mut Arithmetic<'_>) -> Result<([f64; 3], T), CylindricalProfileFailure>,
) -> Result<(CertifiedConicalTrim, T), ConicalChartTrimPairError> {
    binding::edge(query, depth, &input.source, arithmetic.operations())?;
    let start = arithmetic.operations();
    let result = compute(arithmetic).and_then(|(bounds, common)| {
        let trace = trace::accepted_trace(query, bounds, arithmetic.operations() - start)?;
        Ok((trace, common))
    });
    result.map_err(|cause| error::trace(query, depth, arithmetic.operations(), cause))
}
