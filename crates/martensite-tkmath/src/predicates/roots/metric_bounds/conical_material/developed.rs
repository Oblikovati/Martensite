// SPDX-License-Identifier: GPL-2.0-only
//! Retained actual UV boundary comparisons feed the unchanged whole-material theorem.
use super::super::conical_profile::developed_boundary;
use super::*;
pub(super) mod error;
pub use error::{ConicalDevelopedMaterialError, ConicalDevelopedMaterialFailure};

/// Use actual accepted UV trace operands and the same filled-domain normal-map checks.
/// Independent compact UV topology remains a required caller certificate.
/// Example: `certify_conical_material_with_developed_boundaries(query, &spans, &domain, || false)?`.
pub fn certify_conical_material_with_developed_boundaries(
    query: ConicalMaterialQuery<'_>,
    contexts: &[ConicalDevelopedBoundarySpan<'_>],
    domain: &ConicalChartDomainEvidence,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalMaterialEvidence, ConicalDevelopedMaterialError> {
    certify_conical_material_with_developed_boundaries_and_refinement(
        query,
        contexts,
        domain,
        ConicalPrimitiveRefinement::UnsplitV1,
        &mut cancelled,
    )
}

/// Bind the executed refinement to every retained boundary before material checks.
/// Example: `certify_conical_material_with_developed_boundaries_and_refinement(query, &spans, &domain, refinement, || false)?`.
pub fn certify_conical_material_with_developed_boundaries_and_refinement(
    query: ConicalMaterialQuery<'_>,
    contexts: &[ConicalDevelopedBoundarySpan<'_>],
    domain: &ConicalChartDomainEvidence,
    refinement: ConicalPrimitiveRefinement,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalMaterialEvidence, ConicalDevelopedMaterialError> {
    input::admit(&query).map_err(|cause| error::numerical(None, cause, 0))?;
    validate(&query, contexts, domain, refinement)?;
    let policy = ArithmeticPolicy {
        fractional_bits: query.options.fractional_bits,
        max_operations: query.options.max_operations,
    };
    let mut arithmetic = Arithmetic::new(policy, &mut cancelled);
    let spatial = bounds::certify_developed(&query, contexts, domain, refinement, &mut arithmetic)?;
    Ok(ConicalMaterialEvidence {
        bits: fingerprint(&query, contexts),
        chart: Some(ChartBinding::capture_selected(domain, refinement)),
        spatial,
        operations: arithmetic.operations(),
    })
}
fn validate(
    query: &ConicalMaterialQuery<'_>,
    contexts: &[ConicalDevelopedBoundarySpan<'_>],
    domain: &ConicalChartDomainEvidence,
    refinement: ConicalPrimitiveRefinement,
) -> Result<(), ConicalDevelopedMaterialError> {
    if contexts.len() != query.spans.len() {
        return Err(error::binding(
            None,
            "one developed witness per actual span",
            vec![contexts.len() as u64],
            vec![query.spans.len() as u64],
        ));
    }
    for (index, (span, context)) in query.spans.iter().zip(contexts).enumerate() {
        developed_boundary::validate_with_refinement(query, span, context, domain, refinement)
            .map_err(|cause| error::failed(Some(index), cause, 0))?;
    }
    Ok(())
}
impl ConicalMaterialEvidence {
    /// Rebind all actual source, UV, chart, reference and retained-operand authority.
    /// Example: `assert!(proof.matches_developed(&query, &spans, &domain));`.
    pub fn matches_developed(
        &self,
        query: &ConicalMaterialQuery<'_>,
        contexts: &[ConicalDevelopedBoundarySpan<'_>],
        domain: &ConicalChartDomainEvidence,
    ) -> bool {
        self.matches_developed_with_refinement(
            query,
            contexts,
            domain,
            ConicalPrimitiveRefinement::UnsplitV1,
        )
    }
    /// Rebind the actual source, domain, grid and selected retained-operand behavior.
    /// Example: `assert!(proof.matches_developed_with_refinement(&query, &spans, &domain, refinement));`.
    pub fn matches_developed_with_refinement(
        &self,
        query: &ConicalMaterialQuery<'_>,
        contexts: &[ConicalDevelopedBoundarySpan<'_>],
        domain: &ConicalChartDomainEvidence,
        refinement: ConicalPrimitiveRefinement,
    ) -> bool {
        self.chart
            .as_ref()
            .is_some_and(|chart| chart.matches_selected(domain, refinement))
            && validate(query, contexts, domain, refinement).is_ok()
            && self.bits == fingerprint(query, contexts)
    }
}
fn fingerprint(
    query: &ConicalMaterialQuery<'_>,
    contexts: &[ConicalDevelopedBoundarySpan<'_>],
) -> Vec<u64> {
    let mut bits = input::bits(query);
    bits.extend([0x6465_7665_6c6f_7065, 1, contexts.len() as u64]);
    for context in contexts {
        let authority = developed_boundary::fingerprint(context.evidence);
        bits.push(authority.len() as u64);
        bits.extend(authority);
    }
    bits
}
