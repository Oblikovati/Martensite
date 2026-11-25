// SPDX-License-Identifier: GPL-2.0-only
//! A source-checked positive-weight Z-row move composes a complete immutable boundary.
use super::boundary_snapshot::BoundarySnapshot;
use super::*;
use std::sync::Arc;
mod binding;
mod capture;
mod error;
#[cfg(test)]
mod tests;
pub use error::{ConicalBoundaryZRowError, ConicalBoundaryZRowFailure};
/// Original nearest receipt, actual final rows and issued family movement authority.
#[derive(Clone, Copy)]
pub struct ConicalBoundaryZRowTransition<'a> {
    /// Stable side in the issued family policy.
    pub span: ConicalActualSideId,
    /// Unmodified nearest boundary receipt.
    pub parent: &'a ConicalBoundaryEvidence,
    /// Complete nearest query.
    pub previous: ConicalBoundaryQuery<'a>,
    /// Complete actual directed query.
    pub current: ConicalBoundaryQuery<'a>,
    /// Original quantitative chart authority.
    pub domain: &'a ConicalChartDomainEvidence,
    /// Explicit primitive refinement.
    pub refinement: ConicalPrimitiveRefinement,
    /// Issued family-wide representable movement policy.
    pub gamma: &'a ConicalDirectedGammaEvidence,
}
/// Complete final boundary and checked immutable transition operands.
pub struct ConicalBoundaryZRowEvidence {
    boundary: Arc<ConicalBoundaryEvidence>,
    capture: DirectedRowCapture,
    incremental: u32,
    cumulative: u64,
    remaining: ConeClearanceOptions,
}
pub(super) struct DirectedRowCapture {
    pub(super) span: ConicalActualSideId,
    pub(super) new_z: [Vec<Interval>; 2],
    pub(super) signed_shifts: [Interval; 2],
    pub(super) parent_storage: [Interval; 2],
    pub(super) storage: [Interval; 2],
    pub(super) retained: Arc<spatial::RetainedSpatial>,
    current: BoundarySnapshot,
    chart: boundary_chart::ChartBinding,
    gamma: ConicalDirectedGammaEvidence,
}
impl DirectedRowCapture {
    pub(super) fn matches_gamma(&self, expected: &ConicalDirectedGammaEvidence) -> bool {
        self.gamma.same_authority(expected)
    }
}
impl ConicalBoundaryZRowEvidence {
    /// Source side bound by this transition; `transition.span_id()`.
    pub fn span_id(&self) -> ConicalActualSideId {
        self.capture.span
    }
    /// Complete final boundary; `transition.boundary()`.
    pub fn boundary(&self) -> &ConicalBoundaryEvidence {
        &self.boundary
    }
    /// Share the same issued boundary without a second issuer; `transition.boundary_shared()`.
    pub fn boundary_shared(&self) -> Arc<ConicalBoundaryEvidence> {
        Arc::clone(&self.boundary)
    }
    /// Newly executed transition arithmetic, excluding parent work; `transition.incremental_operations()`.
    pub fn incremental_operations(&self) -> u32 {
        self.incremental
    }
    /// Parent plus transition arithmetic; `transition.cumulative_operations()`.
    pub fn cumulative_operations(&self) -> u64 {
        self.cumulative
    }
    /// Original incremental producing grant; `transition.remaining_options()`.
    pub fn remaining_options(&self) -> ConeClearanceOptions {
        self.remaining
    }
    /// Rebind actual final rows, domain, refinement and grid; `transition.matches_current(&query, &domain, refinement, bits)`.
    pub fn matches_current(
        &self,
        current: &ConicalBoundaryQuery<'_>,
        domain: &ConicalChartDomainEvidence,
        refinement: ConicalPrimitiveRefinement,
        bits: u16,
    ) -> bool {
        self.checked_actual_capture(current, domain, refinement, bits)
            .is_some()
    }
    pub(super) fn checked_actual_capture(
        &self,
        current: &ConicalBoundaryQuery<'_>,
        domain: &ConicalChartDomainEvidence,
        refinement: ConicalPrimitiveRefinement,
        bits: u16,
    ) -> Option<&DirectedRowCapture> {
        (self.boundary.options().fractional_bits == bits
            && domain.options().fractional_bits == bits
            && self.capture.current.matches(current)
            && self.capture.chart.matches_selected(domain, refinement)
            && self
                .boundary
                .matches_chart_with_refinement(current, domain, refinement))
        .then_some(&self.capture)
    }
}
/// Compose actual issued Z moves with a complete nearest receipt under the remaining grant.
/// Example: `let final_boundary = certify_conical_boundary_z_rows(transition, remaining, || false)?;`.
pub fn certify_conical_boundary_z_rows(
    transition: ConicalBoundaryZRowTransition<'_>,
    remaining: ConeClearanceOptions,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalBoundaryZRowEvidence, ConicalBoundaryZRowError> {
    let input = binding::admit(&transition, remaining)?;
    let mut arithmetic = Arithmetic::new(sector_api::policy(remaining), &mut cancelled);
    capture::execute(&transition, input, &mut arithmetic)
        .map_err(|cause| error::failed(cause, remaining, arithmetic.operations()))
}
impl std::fmt::Debug for ConicalBoundaryZRowEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConicalBoundaryZRowEvidence")
            .field("boundary", &self.boundary)
            .field("incremental", &self.incremental)
            .field("cumulative", &self.cumulative)
            .finish()
    }
}
