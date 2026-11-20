// SPDX-License-Identifier: GPL-2.0-only
//! Join the authored and retained cap contours in one unwrapped angle/slant plane.
use super::super::boundary_chart::ChartBinding;
use super::super::developed_boundary::{join_carriers, join_uv};
use super::*;
mod homotopy;
pub use homotopy::{
    ConicalCapClearanceRelation, ConicalCapHomotopyEvidence, ConicalCapHomotopyFailure,
    ConicalCapVolumeError, ConicalCapVolumeEvidence, ConicalCapVolumeFailure,
};
mod projection;
use projection::Projection;

/// One decoded cap trace, its source-bound spatial proof and its own fit chart.
#[derive(Clone, Copy)]
pub struct ConicalCapJoinSpan<'a> {
    /// Bound source, native boundary and decoded cap polynomials.
    pub cap: ConicalDevelopedBoundarySpan<'a>,
    /// Chart used to certify this particular span.
    pub chart: &'a ConicalChartDomainEvidence,
}

/// Contour order and common cap coordinates for joining separate local charts.
pub struct ConicalCapJoinsQuery<'a> {
    /// Decoded spans in wire order, without repeating the closing span.
    pub spans: &'a [ConicalCapJoinSpan<'a>],
    /// Length of each closed contour in spans.
    pub contour_lengths: &'a [usize],
    /// Shared slant origin of the cap polynomial coordinates.
    pub reference_slant: f64,
    /// Remaining shared exact arithmetic allowance.
    pub options: ConeClearanceOptions,
}

pub(super) struct CapBinding {
    reference: u64,
    support: [u64; 5],
    carriers: [[u64; 3]; 2],
    motion: DyadicBound,
    spans: Vec<CapSpanBinding>,
}
struct CapSpanBinding {
    chart: ChartBinding,
    uv: Vec<[u64; 2]>,
}

/// Prove the same positive derivative projection on both sides of every closed
/// join throughout the straight interpolation in unwrapped angle/slant coordinates.
/// Example: `let joins = certify_conical_cap_joins(query, || false)?;`.
pub fn certify_conical_cap_joins(
    query: ConicalCapJoinsQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalJoinsEvidence, ConicalChartJoinsError> {
    chart::admit_components(query.contour_lengths, query.spans.len(), query.options)?;
    let mut arithmetic = Arithmetic::new(sector_api::policy(query.options), &mut cancelled);
    let (directions, binding) = execute(&query, &mut arithmetic)
        .map_err(|cause| ConicalChartJoinsError::new(cause, arithmetic.operations()))?;
    Ok(evidence(
        query,
        binding,
        directions,
        arithmetic.operations(),
    ))
}

struct Capture<'a, 'q> {
    query: &'q ConicalCapJoinsQuery<'a>,
    support: [u64; 5],
    motion: num_bigint::BigInt,
    carriers: [[u64; 3]; 2],
    spans: Vec<CapSpanBinding>,
}

impl Capture<'_, '_> {
    fn projection(
        &mut self,
        index: usize,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Result<Projection, ConicalJoinsFailure> {
        let span = &self.query.spans[index];
        let operands = operands(self.query, span)
            .filter(|operands| operands.carriers == self.carriers)
            .ok_or(ConicalJoinsFailure::Boundary { index })?;
        if common_support(&span.cap.source) != self.support {
            return Err(ConicalJoinsFailure::Boundary { index });
        }
        let projection = Projection::capture(operands.uv, operands.capture, arithmetic)
            .map_err(|cause| failure(index, cause))?;
        self.include_motion(self.query.reference_slant, &operands, index, arithmetic)?;
        self.spans.push(CapSpanBinding {
            chart: ChartBinding::capture_selected(
                span.chart,
                ConicalPrimitiveRefinement::UnsplitV1,
            ),
            uv: operands.uv.to_vec(),
        });
        Ok(projection)
    }
    fn include_motion(
        &mut self,
        reference: f64,
        operands: &super::super::developed_boundary::JoinOperands<'_>,
        index: usize,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Result<(), ConicalJoinsFailure> {
        let bound = operands
            .motion_upper(reference, arithmetic)
            .map_err(|cause| failure(index, cause))?;
        if bound > self.motion {
            self.motion = bound;
        }
        Ok(())
    }
}

fn operands<'a>(
    query: &ConicalCapJoinsQuery<'_>,
    span: &'a ConicalCapJoinSpan<'_>,
) -> Option<super::super::developed_boundary::JoinOperands<'a>> {
    join_uv(
        &span.cap,
        span.chart,
        query.reference_slant,
        query.options.fractional_bits,
    )
}

fn common_support(query: &ConicalBoundaryQuery<'_>) -> [u64; 5] {
    [
        query.semi_angle.to_bits(),
        query.chart_origin.to_bits(),
        query.depths[0].to_bits(),
        query.depths[1].to_bits(),
        u64::from(query.outward),
    ]
}

fn execute(
    query: &ConicalCapJoinsQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(Vec<[f64; 2]>, CapBinding), ConicalJoinsFailure> {
    let first = &query.spans[0];
    let mut capture = Capture {
        query,
        support: common_support(&first.cap.source),
        motion: 0.into(),
        carriers: join_carriers(first.cap.evidence),
        spans: Vec::with_capacity(query.spans.len()),
    };
    let mut directions = Vec::with_capacity(query.spans.len());
    let mut start = 0;
    for length in query.contour_lengths {
        let end = start + length;
        prove_ring(&mut capture, start..end, arithmetic, &mut directions)?;
        start = end;
    }
    Ok((
        directions,
        CapBinding {
            reference: query.reference_slant.to_bits(),
            support: capture.support,
            carriers: capture.carriers,
            motion: DyadicBound::new(capture.motion, -i32::from(arithmetic.fractional_bits())),
            spans: capture.spans,
        },
    ))
}

fn prove_ring(
    capture: &mut Capture<'_, '_>,
    ring: std::ops::Range<usize>,
    arithmetic: &mut Arithmetic<'_>,
    directions: &mut Vec<[f64; 2]>,
) -> Result<(), ConicalJoinsFailure> {
    let first = capture.projection(ring.start, arithmetic)?;
    let mut previous = None;
    for index in ring.clone().skip(1) {
        let next = capture.projection(index, arithmetic)?;
        let before = previous.as_ref().unwrap_or(&first);
        directions.push(
            before
                .join(&next, arithmetic)
                .map_err(|cause| failure(index - 1, cause))?,
        );
        previous = Some(next);
    }
    let last = previous.as_ref().unwrap_or(&first);
    directions.push(
        last.join(&first, arithmetic)
            .map_err(|cause| failure(ring.end - 1, cause))?,
    );
    Ok(())
}

fn evidence(
    query: ConicalCapJoinsQuery<'_>,
    binding: CapBinding,
    directions: Vec<[f64; 2]>,
    operations: u32,
) -> ConicalJoinsEvidence {
    ConicalJoinsEvidence {
        chart: None,
        cap_charts: Some(Box::new(binding)),
        input: query
            .spans
            .iter()
            .map(|span| BoundarySnapshot::capture(&span.cap.source))
            .collect(),
        contour_lengths: query.contour_lengths.to_vec(),
        options: query.options,
        directions,
        operations,
    }
}

impl ConicalJoinsEvidence {
    /// Work policy for this decoded-cap join certificate.
    /// Example: `let original_policy = proof.cap_join_options();`.
    pub fn cap_join_options(&self) -> Option<ConeClearanceOptions> {
        self.cap_charts.as_ref().map(|_| self.options)
    }

    /// Rebind the decoded spans, individual charts and join policy.
    /// Example: `assert!(proof.matches_cap_joins(&query))`.
    pub fn matches_cap_joins(&self, query: &ConicalCapJoinsQuery<'_>) -> bool {
        let Some(binding) = &self.cap_charts else {
            return false;
        };
        self.options == query.options
            && self.contour_lengths == query.contour_lengths
            && self.input.len() == query.spans.len()
            && binding.reference == query.reference_slant.to_bits()
            && self
                .input
                .iter()
                .zip(&binding.spans)
                .zip(query.spans)
                .all(|((source, old), span)| rebind(source, old, span, query, binding.carriers))
    }
}

fn rebind(
    source: &BoundarySnapshot,
    old: &CapSpanBinding,
    span: &ConicalCapJoinSpan<'_>,
    query: &ConicalCapJoinsQuery<'_>,
    carriers: [[u64; 3]; 2],
) -> bool {
    source.matches(&span.cap.source)
        && old.chart.matches(span.chart)
        && operands(query, span)
            .is_some_and(|actual| old.uv == actual.uv && actual.carriers == carriers)
}
