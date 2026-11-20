// SPDX-License-Identifier: GPL-2.0-only
//! Actual native UV operands retained after both independently accepted chart traces.
use super::boundary_snapshot::BoundarySnapshot;
use super::*;
mod binding;
mod material;
mod motion;
pub(in crate::predicates::roots::metric_bounds) use super::boundary_chart::ChartBinding;
pub(in crate::predicates::roots::metric_bounds) use material::{
    half_pi, span_bounds_with_refinement, validate_with_refinement,
};
#[cfg(test)]
pub(in crate::predicates::roots::metric_bounds) use material::{span_bounds, validate};

/// Immutable actual-UV angular proof and slant operands, not a material topology certificate.
#[derive(Debug)]
pub struct ConicalDevelopedBoundaryEvidence {
    source: BoundarySnapshot,
    domain: ChartBinding,
    pair: Vec<u64>,
    uv_bits: [Vec<[u64; 2]>; 2],
    carriers: [[u64; 3]; 2],
    reference: u64,
    options: ConeClearanceOptions,
    operations: u32,
    position_upper: f64,
    operands: RetainedOperands,
}
#[derive(Debug)]
pub(super) struct RetainedOperands {
    pub(super) uv: [Interval; 2],
    pub(super) angular: Interval,
    pub(super) delta: Vec<Interval>,
}
/// Checked original/source authority paired with its actual decoded UV witness.
#[derive(Clone, Copy)]
pub struct ConicalDevelopedBoundarySpan<'a> {
    /// Full query reconstructed against actual decoded edges.
    pub source: ConicalBoundaryQuery<'a>,
    /// Executed source-bound spatial and storage receipt.
    pub boundary: &'a ConicalBoundaryEvidence,
    /// Actual accepted trace operands, owned independently of any source borrow.
    pub evidence: &'a ConicalDevelopedBoundaryEvidence,
}
impl ConicalDevelopedBoundaryEvidence {
    /// Actual producing pair work, already charged once; `let work = evidence.operations();`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Bind both decoded cap polynomials and carriers without rebuilding their trim snapshots.
    /// Example: `assert!(proof.matches_decoded_caps(pcurves, carriers));`.
    pub fn matches_decoded_caps(
        &self,
        pcurves: [&[[f64; 2]]; 2],
        carriers: [ConicalTrimCarrier; 2],
    ) -> bool {
        self.carriers == carriers.map(carrier_bits)
            && self.uv_bits.iter().zip(pcurves).all(|(stored, poles)| {
                stored
                    .iter()
                    .copied()
                    .eq(poles.iter().map(|pole| pole.map(f64::to_bits)))
            })
    }
    /// Rebind actual geometry, chart, grid and reference bits; `assert!(evidence.matches(&pair, &source, &boundary, &domain, reference));`.
    pub fn matches(
        &self,
        pair: &ConicalTrimPairQuery<'_>,
        source: &ConicalBoundaryQuery<'_>,
        boundary: &ConicalBoundaryEvidence,
        domain: &ConicalChartDomainEvidence,
        reference: f64,
    ) -> bool {
        self.matches_with_refinement(
            pair,
            source,
            boundary,
            domain,
            reference,
            ConicalPrimitiveRefinement::UnsplitV1,
        )
    }
    /// Rebind actual operands and explicit primitive authority; `proof.matches_with_refinement(&pair, &source, &boundary, &domain, reference, refinement)`.
    pub fn matches_with_refinement(
        &self,
        pair: &ConicalTrimPairQuery<'_>,
        source: &ConicalBoundaryQuery<'_>,
        boundary: &ConicalBoundaryEvidence,
        domain: &ConicalChartDomainEvidence,
        reference: f64,
        refinement: ConicalPrimitiveRefinement,
    ) -> bool {
        self.source.matches(source)
            && boundary.matches_chart_with_refinement(source, domain, refinement)
            && self.options == boundary.options()
            && self.domain.matches_selected(domain, refinement)
            && self.pair == binding::pair_bits(pair)
            && self.reference == reference.to_bits()
    }
}
pub(super) struct JoinOperands<'a> {
    trace: &'a RetainedOperands,
    pub(super) uv: &'a [[u64; 2]],
    pub(super) capture: &'a spatial::SpatialCapture,
    pub(super) carriers: [[u64; 3]; 2],
    pub(super) position_upper: f64,
}

impl JoinOperands<'_> {
    pub(super) fn slant_hull(&self) -> &Interval {
        &self.trace.uv[1]
    }
}

pub(super) fn join_carriers(evidence: &ConicalDevelopedBoundaryEvidence) -> [[u64; 3]; 2] {
    evidence.carriers
}

// Join admission borrows the already bound cap controls once and performs no fingerprint serialization.
pub(super) fn join_uv<'a>(
    context: &'a ConicalDevelopedBoundarySpan<'_>,
    chart: &ConicalChartDomainEvidence,
    reference_slant: f64,
    bits: u16,
) -> Option<JoinOperands<'a>> {
    let issued = context.evidence;
    if !issued.source.matches(&context.source)
        || issued.reference != reference_slant.to_bits()
        || issued.options != context.boundary.options()
        || !issued.domain.matches(chart)
        || issued.uv_bits[0] != issued.uv_bits[1]
    {
        return None;
    }
    let captured = context.boundary.retained_in_chart_with_refinement(
        &context.source,
        chart,
        bits,
        ConicalPrimitiveRefinement::UnsplitV1,
    )?;
    Some(JoinOperands {
        trace: &issued.operands,
        uv: &issued.uv_bits[0],
        capture: &captured.capture,
        carriers: issued.carriers,
        position_upper: issued.position_upper,
    })
}
pub(super) fn capture(
    pair: ConicalTrimPairQuery<'_>,
    source: ConicalBoundaryQuery<'_>,
    boundary: &ConicalBoundaryEvidence,
    selection: (&ConicalChartDomainEvidence, ConicalPrimitiveRefinement),
    reference: f64,
    (operations, position_upper): (u32, f64),
    operands: RetainedOperands,
) -> ConicalDevelopedBoundaryEvidence {
    ConicalDevelopedBoundaryEvidence {
        source: BoundarySnapshot::capture(&source),
        domain: ChartBinding::capture_selected(selection.0, selection.1),
        pair: binding::pair_bits(&pair),
        uv_bits: pair
            .pcurves
            .map(|poles| poles.iter().map(|p| p.map(f64::to_bits)).collect()),
        carriers: pair.carriers.map(carrier_bits),
        reference: reference.to_bits(),
        options: boundary.options(),
        operations,
        position_upper,
        operands,
    }
}

fn carrier_bits(carrier: ConicalTrimCarrier) -> [u64; 3] {
    [carrier.radius, carrier.axial_origin, carrier.semi_angle].map(f64::to_bits)
}
pub(in crate::predicates::roots::metric_bounds) fn fingerprint(
    evidence: &ConicalDevelopedBoundaryEvidence,
) -> Vec<u64> {
    let mut bits = evidence.source.bits.clone();
    // The material binding verifies and stores the common chart once for every span.
    bits.push(evidence.pair.len() as u64);
    bits.extend(&evidence.pair);
    bits.extend([
        evidence.reference,
        u64::from(evidence.options.fractional_bits),
        u64::from(evidence.options.max_operations),
        u64::from(evidence.operations),
    ]);
    bits
}

pub(in crate::predicates::roots::metric_bounds) fn trig(
    domain: &ConicalChartDomainEvidence,
) -> [&Interval; 2] {
    [&domain.sine, &domain.cosine]
}
