// SPDX-License-Identifier: GPL-2.0-only
//! Issued separate-family-extrema movement authority imports parent bounds once.
use super::*;
pub(super) mod admission;
mod execute;
type CapIndex = BTreeMap<ConicalActualCapId, usize>;
/// Immutable issued family policy; caller-supplied movement scalars are not authority.
#[derive(Clone)]
pub struct ConicalDirectedGammaEvidence {
    gamma: [f64; 2],
    options: ConeClearanceOptions,
    refinement: ConicalPrimitiveRefinement,
    operations: u32,
    records: std::sync::Arc<[GammaSpanCapture]>,
    index: std::sync::Arc<BTreeMap<ConicalActualSideId, usize>>,
    caps: std::sync::Arc<[ConicalActualCapCarrier]>,
    cap_index: std::sync::Arc<CapIndex>,
}
pub(in crate::predicates::roots::metric_bounds::conical_profile) struct GammaSpanCapture {
    pub(in crate::predicates::roots::metric_bounds::conical_profile) parent_storage: [Interval; 2],
    pub(in crate::predicates::roots::metric_bounds::conical_profile) depths: [Interval; 2],
    pub(in crate::predicates::roots::metric_bounds::conical_profile) cap_levels: [Interval; 2],
    pub(in crate::predicates::roots::metric_bounds::conical_profile) sine: Interval,
    pub(in crate::predicates::roots::metric_bounds::conical_profile) cosine: Interval,
    pub(in crate::predicates::roots::metric_bounds::conical_profile) id: ConicalActualSideId,
    pub(in crate::predicates::roots::metric_bounds::conical_profile) member: ConicalActualMemberId,
    pub(in crate::predicates::roots::metric_bounds::conical_profile) cap_ids:
        [ConicalActualCapId; 2],
    input: BoundarySnapshot,
    chart: ChartBinding,
    parent_options: ConeClearanceOptions,
    parent_operations: u32,
    parent_bounds: [DyadicBound; 2],
}
impl std::fmt::Debug for ConicalDirectedGammaEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConicalDirectedGammaEvidence")
            .field("gamma", &self.gamma)
            .field("options", &self.options)
            .field("operations", &self.operations)
            .finish()
    }
}
/// Issue the deterministic family bias after complete nearest-proof binding.
/// Example: `let gamma = certify_conical_directed_gamma(query, || false)?;`.
pub fn certify_conical_directed_gamma(
    query: ConicalDirectedGammaQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalDirectedGammaEvidence, ConicalActualPatchError> {
    let mut position = error::FailurePosition::default();
    let caps = admission::admit(&query, &mut position)
        .map_err(|v| position.error(v, ConicalActualPatchStage::Binding, query.options, 0))?;
    let mut arithmetic = Arithmetic::new(sector_api::policy(query.options), &mut cancelled);
    execute::issue(&query, caps, &mut arithmetic, &mut position).map_err(|v| {
        position.error(
            v,
            ConicalActualPatchStage::Gamma,
            query.options,
            arithmetic.operations(),
        )
    })
}
impl ConicalDirectedGammaEvidence {
    /// Actual deterministic physical-row magnitudes; `proof.gamma()`.
    pub fn gamma(&self) -> [f64; 2] {
        self.gamma
    }
    /// Actual charged producing work; `proof.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Original producing grant; `proof.options()`.
    pub fn options(&self) -> ConeClearanceOptions {
        self.options
    }
    /// Executed source refinement; `proof.primitive_refinement()`.
    pub fn primitive_refinement(&self) -> ConicalPrimitiveRefinement {
        self.refinement
    }
    pub(super) fn carrier(&self, id: ConicalActualCapId) -> Option<&ConicalActualCapCarrier> {
        self.cap_index
            .get(&id)
            .and_then(|&index| self.caps.get(index))
    }
    /// Compare all source, domain, mode, carrier and policy operands; `proof.matches(query)`.
    pub fn matches(&self, query: &ConicalDirectedGammaQuery<'_>) -> bool {
        self.options == query.options
            && self.refinement == query.refinement
            && self.caps.len() == query.caps.len()
            && self
                .caps
                .iter()
                .zip(query.caps)
                .all(|(a, b)| admission::same_carrier(a, b))
            && self.records.len() == query.spans.len()
            && self
                .records
                .iter()
                .zip(query.spans)
                .all(|(a, b)| self.record_matches(a, b, query.refinement))
    }
    /// Generate one actual representable Z; subsequent checked transition proves the move.
    /// Example: `let final_z = gamma.transform_z(0, nearest_z)?;`.
    pub fn transform_z(&self, row: usize, old: f64) -> Result<f64, ConicalActualPatchError> {
        let value = execute::transform(self.gamma, row, old)
            .map_err(|v| error::failed(v, ConicalActualPatchStage::Gamma, self.options, 0))?;
        Ok(value)
    }
    pub(in crate::predicates::roots::metric_bounds::conical_profile) fn span_binding_bits(
        &self,
        id: ConicalActualSideId,
    ) -> Option<Vec<u64>> {
        let record = self.span_capture(id)?;
        let mut bits = record.input.bits.clone();
        bits.extend(record.chart.diagnostic_bits());
        bits.extend([
            id.0,
            u64::from(record.parent_options.fractional_bits),
            u64::from(record.parent_options.max_operations),
            u64::from(record.parent_operations),
        ]);
        Some(bits)
    }
    pub(in crate::predicates::roots::metric_bounds::conical_profile) fn span_ids(
        &self,
    ) -> Vec<u64> {
        self.records.iter().map(|v| v.id.0).collect()
    }
    pub(in crate::predicates::roots::metric_bounds::conical_profile) fn span_capture(
        &self,
        id: ConicalActualSideId,
    ) -> Option<&GammaSpanCapture> {
        self.index.get(&id).and_then(|i| self.records.get(*i))
    }
    pub(in crate::predicates::roots::metric_bounds::conical_profile) fn same_authority(
        &self,
        other: &Self,
    ) -> bool {
        self.gamma.map(f64::to_bits) == other.gamma.map(f64::to_bits)
            && self.options == other.options
            && self.refinement == other.refinement
            && self.operations == other.operations
            && same_capture(&self.caps, &other.caps, admission::same_carrier)
            && same_capture(&self.records, &other.records, same_record)
    }
    fn record_matches(
        &self,
        record: &GammaSpanCapture,
        span: &ConicalDirectedGammaSpan<'_>,
        refinement: ConicalPrimitiveRefinement,
    ) -> bool {
        record.id == span.id
            && record.member == span.member
            && record.cap_ids == span.caps
            && record.input.matches(&span.previous)
            && record.chart.matches_selected(span.domain, refinement)
            && record.parent_options == span.parent.options()
            && record.parent_operations == span.parent.operations()
            && &record.parent_bounds == span.parent.storage()
            && span
                .parent
                .matches_chart_with_refinement(&span.previous, span.domain, refinement)
    }
    pub(in crate::predicates::roots::metric_bounds::conical_profile) fn checked_span(
        &self,
        id: ConicalActualSideId,
        previous: &ConicalBoundaryQuery<'_>,
        parent: &ConicalBoundaryEvidence,
        domain: &ConicalChartDomainEvidence,
        refinement: ConicalPrimitiveRefinement,
        bits: u16,
    ) -> Option<&GammaSpanCapture> {
        if self.options.fractional_bits != bits || self.refinement != refinement {
            return None;
        }
        let record = self.index.get(&id).and_then(|i| self.records.get(*i))?;
        let span = ConicalDirectedGammaSpan {
            id,
            member: record.member,
            previous: *previous,
            parent,
            domain,
            caps: record.cap_ids,
        };
        self.record_matches(record, &span, refinement)
            .then_some(record)
    }
}

fn same_capture<T>(
    left: &std::sync::Arc<[T]>,
    right: &std::sync::Arc<[T]>,
    equal: impl Fn(&T, &T) -> bool,
) -> bool {
    std::sync::Arc::ptr_eq(left, right)
        || (left.len() == right.len() && left.iter().zip(right.iter()).all(|(a, b)| equal(a, b)))
}

fn same_record(a: &GammaSpanCapture, b: &GammaSpanCapture) -> bool {
    a.id == b.id
        && a.member == b.member
        && a.cap_ids == b.cap_ids
        && a.input.bits == b.input.bits
        && a.chart == b.chart
        && a.parent_options == b.parent_options
        && a.parent_operations == b.parent_operations
        && a.parent_bounds == b.parent_bounds
        && a.parent_storage == b.parent_storage
        && a.depths == b.depths
        && a.cap_levels == b.cap_levels
        && a.sine == b.sine
        && a.cosine == b.cosine
}
