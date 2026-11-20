// SPDX-License-Identifier: GPL-2.0-only
//! Source-bound opt-in cone metric; legacy boundary receipts keep their original trace.
use super::boundary::{admit_query, combined_request, evidence, proof_error};
use super::boundary_snapshot::BoundarySnapshot;
use super::*;
#[cfg(test)]
mod anchor_tests;
#[cfg(test)]
mod refinement_tests;
#[cfg(test)]
pub(super) mod tests;
#[derive(PartialEq)]
pub(super) struct ChartBinding {
    version: ConicalChartDomainVersion,
    span_limit: usize,
    source: Vec<Vec<[u64; 2]>>,
    support: [u64; 6],
    options: ConeClearanceOptions,
    operations: u32,
    refinement: ConicalPrimitiveRefinement,
}
impl ChartBinding {
    pub(super) fn capture(domain: &ConicalChartDomainEvidence) -> Self {
        Self::capture_selected(domain, ConicalPrimitiveRefinement::UnsplitV1)
    }
    pub(super) fn capture_selected(
        domain: &ConicalChartDomainEvidence,
        refinement: ConicalPrimitiveRefinement,
    ) -> Self {
        Self {
            version: domain.version(),
            span_limit: domain.span_limit(),
            source: domain.sector.source.clone(),
            support: support(domain),
            options: domain.options(),
            operations: domain.operations(),
            refinement,
        }
    }
    pub(super) fn matches(&self, domain: &ConicalChartDomainEvidence) -> bool {
        self.matches_selected(domain, ConicalPrimitiveRefinement::UnsplitV1)
    }
    pub(super) fn matches_selected(
        &self,
        domain: &ConicalChartDomainEvidence,
        refinement: ConicalPrimitiveRefinement,
    ) -> bool {
        self.version == domain.version()
            && self.span_limit == domain.span_limit()
            && self.refinement == refinement
            && self.source == domain.sector.source
            && self.support == support(domain)
            && self.options == domain.options()
            && self.operations == domain.operations()
    }
}
/// Bound a cone boundary with its executed quantitative chart authority.
/// Example: `certify_conical_boundary_in_chart(query, &domain, options, || false)?`.
pub fn certify_conical_boundary_in_chart(
    query: ConicalBoundaryQuery<'_>,
    domain: &ConicalChartDomainEvidence,
    options: ConeClearanceOptions,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalBoundaryEvidence, ConicalBoundaryError> {
    certify_conical_boundary_in_chart_with_refinement(
        query,
        domain,
        options,
        ConicalPrimitiveRefinement::UnsplitV1,
        &mut cancelled,
    )
}
/// Select explicit primitive refinement while retaining the complete source-bound chart proof.
/// Example: `certify_conical_boundary_in_chart_with_refinement(query, &domain, options, refinement, || false)?`.
pub fn certify_conical_boundary_in_chart_with_refinement(
    query: ConicalBoundaryQuery<'_>,
    domain: &ConicalChartDomainEvidence,
    options: ConeClearanceOptions,
    refinement: ConicalPrimitiveRefinement,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalBoundaryEvidence, ConicalBoundaryError> {
    let input = admit_query(&query, options)?;
    if let Err(cause) = admit_domain(&query, domain, options) {
        return Err(proof_error(cause, 0, options, input));
    }
    execute(query, domain, options, input, refinement, &mut cancelled)
}
fn execute(
    query: ConicalBoundaryQuery<'_>,
    domain: &ConicalChartDomainEvidence,
    options: ConeClearanceOptions,
    input: BoundarySnapshot,
    refinement: ConicalPrimitiveRefinement,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Result<ConicalBoundaryEvidence, ConicalBoundaryError> {
    let mut arithmetic = Arithmetic::new(sector_api::policy(options), cancelled);
    let result = combined::certify_combined_in_chart_selected(
        &combined_request(&query),
        domain,
        refinement,
        &mut arithmetic,
    );
    match result {
        Ok(result) => Ok(chart_evidence(
            result,
            options,
            input,
            query.original.map_or(0, |v| v.level),
            domain,
            refinement,
        )),
        Err(cause) => Err(proof_error(cause, arithmetic.operations(), options, input)),
    }
}
fn chart_evidence(
    result: combined::CombinedEvidence,
    options: ConeClearanceOptions,
    input: BoundarySnapshot,
    level: u8,
    domain: &ConicalChartDomainEvidence,
    refinement: ConicalPrimitiveRefinement,
) -> ConicalBoundaryEvidence {
    let mut proof = evidence(result, options, input, level);
    proof.chart = Some(ChartBinding::capture_selected(domain, refinement));
    proof
}

fn support(domain: &ConicalChartDomainEvidence) -> [u64; 6] {
    [
        domain.sector.angle,
        domain.chart_origin().to_bits(),
        domain.fit_center().to_bits(),
        domain.developed_turn() as u64,
        domain.physical_turn() as u64,
        u64::from(domain.options().fractional_bits),
    ]
}
fn admit_domain(
    query: &ConicalBoundaryQuery<'_>,
    domain: &ConicalChartDomainEvidence,
    options: ConeClearanceOptions,
) -> Result<(), combined::CombinedFailure> {
    admit_support(query, domain, options)?;
    admit_membership(query, domain)
}
fn requested_support(query: &ConicalBoundaryQuery<'_>, options: ConeClearanceOptions) -> [u64; 6] {
    [
        query.semi_angle.to_bits(),
        query.chart_origin.to_bits(),
        query.fit_center.to_bits(),
        query.developed_turn as u64,
        query.physical_turn as u64,
        u64::from(options.fractional_bits),
    ]
}
fn admit_support(
    query: &ConicalBoundaryQuery<'_>,
    domain: &ConicalChartDomainEvidence,
    options: ConeClearanceOptions,
) -> Result<(), combined::CombinedFailure> {
    let actual = requested_support(query, options);
    let expected = support(domain);
    if let Some(index) = actual.iter().zip(expected).position(|(a, b)| *a != b) {
        return Err(binding_error(
            "chart support, lift and precision",
            &actual,
            &expected,
            index,
        ));
    }
    Ok(())
}
fn original_member(original: &[[f64; 2]], domain: &ConicalChartDomainEvidence) -> bool {
    domain.sector.source.iter().any(|source| {
        source
            .iter()
            .copied()
            .eq(original.iter().map(|p| p.map(f64::to_bits)))
    })
}
fn admit_membership(
    query: &ConicalBoundaryQuery<'_>,
    domain: &ConicalChartDomainEvidence,
) -> Result<(), combined::CombinedFailure> {
    let original = query.original.map_or(query.source, |source| source.poles);
    if original_member(original, domain) {
        return Ok(());
    }
    let actual: Vec<_> = original.iter().flatten().map(|v| v.to_bits()).collect();
    let expected = domain_source_bits(domain);
    let index = actual
        .iter()
        .zip(&expected)
        .position(|(a, b)| a != b)
        .unwrap_or(actual.len().min(expected.len()));
    Err(binding_error(
        "original span membership in executed chart domain",
        &actual,
        &expected,
        index,
    ))
}

fn domain_source_bits(domain: &ConicalChartDomainEvidence) -> Vec<u64> {
    domain
        .sector
        .source
        .iter()
        .flatten()
        .flatten()
        .copied()
        .collect()
}
fn binding_error(
    field: &'static str,
    actual: &[u64],
    expected: &[u64],
    index: usize,
) -> combined::CombinedFailure {
    combined::CombinedFailure::Binding {
        depth: 0,
        field,
        mismatch: combined::BitsMismatch {
            actual_len: actual.len(),
            expected_len: expected.len(),
            index,
            actual: actual.get(index).copied(),
            expected: expected.get(index).copied(),
        },
    }
}
impl ConicalBoundaryEvidence {
    #[cfg(test)]
    pub(super) fn retained_in_chart(
        &self,
        query: &ConicalBoundaryQuery<'_>,
        domain: &ConicalChartDomainEvidence,
        fractional_bits: u16,
    ) -> Option<&spatial::RetainedSpatial> {
        self.retained_in_chart_with_refinement(
            query,
            domain,
            fractional_bits,
            ConicalPrimitiveRefinement::UnsplitV1,
        )
    }
    pub(super) fn retained_in_chart_with_refinement(
        &self,
        query: &ConicalBoundaryQuery<'_>,
        domain: &ConicalChartDomainEvidence,
        fractional_bits: u16,
        refinement: ConicalPrimitiveRefinement,
    ) -> Option<&spatial::RetainedSpatial> {
        if self.options.fractional_bits != fractional_bits
            || domain.options().fractional_bits != fractional_bits
            || !self.matches_chart_with_refinement(query, domain, refinement)
        {
            return None;
        }
        self.retained.as_deref()
    }
    /// Rebind a legacy receipt, excluding the opt-in chart metric; `proof.matches_legacy(&query)`.
    pub fn matches_legacy(&self, query: &ConicalBoundaryQuery<'_>) -> bool {
        self.chart.is_none() && self.matches(query)
    }

    /// Rebind this executed metric authority and full query; `proof.matches_chart(&query, &domain)`.
    pub fn matches_chart(
        &self,
        query: &ConicalBoundaryQuery<'_>,
        domain: &ConicalChartDomainEvidence,
    ) -> bool {
        self.matches(query) && self.chart.as_ref().is_some_and(|old| old.matches(domain))
    }
    /// Bind the actual executed refinement as well as the full chart inputs.
    /// Example: `assert!(proof.matches_chart_with_refinement(&query, &domain, refinement))`.
    pub fn matches_chart_with_refinement(
        &self,
        query: &ConicalBoundaryQuery<'_>,
        domain: &ConicalChartDomainEvidence,
        refinement: ConicalPrimitiveRefinement,
    ) -> bool {
        self.matches(query)
            && self
                .chart
                .as_ref()
                .is_some_and(|old| old.matches_selected(domain, refinement))
    }
    /// Actual primitive refinement, or no chart refinement on a legacy proof.
    /// Example: `let actual = proof.primitive_refinement()`.
    pub fn primitive_refinement(&self) -> Option<ConicalPrimitiveRefinement> {
        self.chart.as_ref().map(|chart| chart.refinement)
    }
}
// Preserve the legacy receipt's observable Debug fields and order.
impl std::fmt::Debug for ConicalBoundaryEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConicalBoundaryEvidence")
            .field("total", &self.total)
            .field("ideal", &self.ideal)
            .field("original", &self.original)
            .field("angular_derivative", &self.angular_derivative)
            .field(
                "original_angular_derivative",
                &self.original_angular_derivative,
            )
            .field("total_angular_derivative", &self.total_angular_derivative)
            .field("authored_level", &self.authored_level)
            .field("anchor", &self.anchor)
            .field("storage", &self.storage)
            .field("operations", &self.operations)
            .field("options", &self.options)
            .field("input", &self.input)
            .finish()
    }
}

impl ChartBinding {
    pub(super) fn diagnostic_bits(&self) -> Vec<u64> {
        let mut bits = vec![1, self.source.len() as u64];
        for span in &self.source {
            bits.push(span.len() as u64);
            bits.extend(span.iter().flatten().copied());
        }
        bits.extend(self.support);
        bits.extend([
            u64::from(self.options.fractional_bits),
            u64::from(self.options.max_operations),
            u64::from(self.operations),
        ]);
        if self.refinement != ConicalPrimitiveRefinement::UnsplitV1 {
            bits.extend([2, self.refinement.tag()]);
        }
        if self.version != ConicalChartDomainVersion::OriginalV2 {
            bits.extend([4, self.version.tag(), self.span_limit as u64]);
        }
        bits
    }
}
