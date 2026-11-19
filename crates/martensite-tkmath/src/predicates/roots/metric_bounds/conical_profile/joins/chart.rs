// SPDX-License-Identifier: GPL-2.0-only
//! #552: reuse checked chart captures in the unchanged developed-coordinate join theorem.
use super::*;
#[cfg(test)]
pub(in super::super) mod tests;
/// Chart-mode refusal with the original typed cause and all work actually consumed.
#[derive(Debug)]
pub struct ConicalChartJoinsError {
    /// Global affected span, when a particular contour/span failed.
    pub index: Option<usize>,
    /// Actual shared arithmetic work before refusal; never a requested allowance.
    pub operations: u32,
    /// Original legacy-shaped join failure; boxing affects only this new refusal.
    pub cause: Box<ConicalJoinsFailure>,
}
impl ConicalChartJoinsError {
    fn new(cause: ConicalJoinsFailure, operations: u32) -> Self {
        let index = match &cause {
            ConicalJoinsFailure::Boundary { index } | ConicalJoinsFailure::Proof { index, .. } => {
                Some(*index)
            }
            _ => None,
        };
        Self {
            index,
            operations,
            cause: Box::new(cause),
        }
    }
    /// Whether the original local arithmetic observed cancellation; `error.is_cancelled()`.
    pub fn is_cancelled(&self) -> bool {
        matches!(self.cause.as_ref(), ConicalJoinsFailure::Proof { cause, .. }
            if matches!(cause.arithmetic_cause(), Some(BoundedArithmeticFailure::Cancelled)))
    }
}
impl std::fmt::Display for ConicalChartJoinsError {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let ConicalJoinsFailure::Contours { count, .. } = self.cause.as_ref()
            && *count > 128
        {
            return write!(
                out,
                "chart contours={count}; expected 1..=128 closed contours; operations={}",
                self.operations
            );
        }
        write!(
            out,
            "chart join index={:?}, operations={}: {}",
            self.index, self.operations, self.cause
        )
    }
}
impl std::error::Error for ConicalChartJoinsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.cause.as_ref())
    }
}
/// Certify every declared closed contour using its retained chart-mode boundary captures.
/// Example: `certify_conical_joins_in_chart(query, &domain, || false)?`.
pub fn certify_conical_joins_in_chart(
    query: ConicalJoinsQuery<'_>,
    domain: &ConicalChartDomainEvidence,
    cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalJoinsEvidence, ConicalChartJoinsError> {
    certify_conical_joins_in_chart_with_refinement(
        query,
        domain,
        ConicalPrimitiveRefinement::UnsplitV1,
        cancelled,
    )
}
/// Certify closed contours under the explicitly selected primitive refinement.
/// Example: `certify_conical_joins_in_chart_with_refinement(query, &domain, refinement, || false)?`.
pub fn certify_conical_joins_in_chart_with_refinement(
    query: ConicalJoinsQuery<'_>,
    domain: &ConicalChartDomainEvidence,
    refinement: ConicalPrimitiveRefinement,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalJoinsEvidence, ConicalChartJoinsError> {
    admit_components(&query)?;
    let mut arithmetic = Arithmetic::new(sector_api::policy(query.options), &mut cancelled);
    let directions = execute(&query, (domain, refinement), &mut arithmetic)
        .map_err(|cause| ConicalChartJoinsError::new(cause, arithmetic.operations()))?;
    let mut proof = joins_evidence(query, directions, arithmetic.operations());
    proof.chart =
        Some(super::super::boundary_chart::ChartBinding::capture_selected(domain, refinement));
    Ok(proof)
}
impl ConicalJoinsEvidence {
    /// Rebind query, precision and actual executed chart authority; `proof.matches_chart(&query, &domain)`.
    pub fn matches_chart(
        &self,
        query: &ConicalJoinsQuery<'_>,
        domain: &ConicalChartDomainEvidence,
    ) -> bool {
        self.matches_chart_with_refinement(query, domain, ConicalPrimitiveRefinement::UnsplitV1)
    }
    /// Rebind full source and selected primitive authority; `proof.matches_chart_with_refinement(&query, &domain, refinement)`.
    pub fn matches_chart_with_refinement(
        &self,
        query: &ConicalJoinsQuery<'_>,
        domain: &ConicalChartDomainEvidence,
        refinement: ConicalPrimitiveRefinement,
    ) -> bool {
        self.chart
            .as_ref()
            .is_some_and(|old| old.matches_selected(domain, refinement))
            && self.matches_inputs(query)
            && query.spans.iter().all(|span| {
                span.boundary
                    .matches_chart_with_refinement(&span.query, domain, refinement)
            })
    }
}
fn admit_components(query: &ConicalJoinsQuery<'_>) -> Result<(), ConicalChartJoinsError> {
    admit_policy(query.options).map_err(|cause| ConicalChartJoinsError::new(cause, 0))?;
    admit_contour_lengths(query).map_err(|cause| ConicalChartJoinsError::new(cause, 0))?;
    if query.contour_lengths.len() > 128 {
        return Err(ConicalChartJoinsError::new(
            contour_count_failure(query, Some(query.spans.len())),
            0,
        ));
    }
    let mut start = 0;
    for count in query.contour_lengths {
        if *count > 128 {
            return Err(component_size(start, *count));
        }
        start += count;
    }
    Ok(())
}
fn component_size(index: usize, actual: usize) -> ConicalChartJoinsError {
    let mut error = ConicalChartJoinsError::new(ConicalJoinsFailure::SpanCount { actual }, 0);
    error.index = Some(index);
    error
}
fn execute(
    query: &ConicalJoinsQuery<'_>,
    selection: (&ConicalChartDomainEvidence, ConicalPrimitiveRefinement),
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<[f64; 2]>, ConicalJoinsFailure> {
    let requests = requests(query)?;
    let mut directions = Vec::with_capacity(query.spans.len());
    let mut start = 0;
    for count in query.contour_lengths {
        let end = start + count;
        prove_contour(
            query,
            &requests,
            selection,
            start..end,
            arithmetic,
            &mut directions,
        )?;
        start = end;
    }
    Ok(directions)
}
fn prove_contour<'a>(
    query: &ConicalJoinsQuery<'a>,
    requests: &'a [super::super::spatial::SpatialRequest<'a>],
    selection: (&ConicalChartDomainEvidence, ConicalPrimitiveRefinement),
    contour: std::ops::Range<usize>,
    arithmetic: &mut Arithmetic<'_>,
    directions: &mut Vec<[f64; 2]>,
) -> Result<(), ConicalJoinsFailure> {
    let first = projection(query, requests, selection, contour.start, arithmetic)?;
    let mut previous = None;
    for index in contour.clone().skip(1) {
        let next = projection(query, requests, selection, index, arithmetic)?;
        let before = previous.as_ref().unwrap_or(&first);
        directions.push(
            super::super::developed::prepared_join_direction(before, &next, arithmetic)
                .map_err(|cause| failure(index - 1, cause))?,
        );
        previous = Some(next);
    }
    let last = previous.as_ref().unwrap_or(&first);
    directions.push(
        super::super::developed::prepared_join_direction(last, &first, arithmetic)
            .map_err(|cause| failure(contour.end - 1, cause))?,
    );
    Ok(())
}

fn projection<'a>(
    query: &ConicalJoinsQuery<'a>,
    requests: &'a [super::super::spatial::SpatialRequest<'a>],
    selection: (&ConicalChartDomainEvidence, ConicalPrimitiveRefinement),
    index: usize,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<super::super::developed::PreparedProjection<'a>, ConicalJoinsFailure> {
    super::super::developed::prepare_projection_in_chart_selected(
        &requests[index],
        &query.spans[index],
        selection.0,
        selection.1,
        arithmetic,
    )
    .map_err(|cause| failure(index, cause))
}
