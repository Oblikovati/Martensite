// SPDX-License-Identifier: GPL-2.0-only
//! Compose the exact q/L reference body; native proxy validity remains a separate obligation.
use super::boundary_snapshot::BoundarySnapshot;
use super::*;
use crate::predicates::roots::metric_bounds::cylindrical_profile::trigonometric;
mod binding;
mod chart;
mod context;
pub use chart::{
    ConicalChartReferenceQuery, certify_conical_chart_reference,
    certify_conical_chart_reference_with_refinement,
};
use context::{ReferenceDomain, ReferenceInputs};
mod geometry;
#[cfg(test)]
mod tests;
mod topology;
/// One source-keyed region whose original planar material is certified by the composing operation.
pub struct ConicalReferenceMember<'a> {
    /// Complete original-interval/ideal-fit/decoded-edge correspondence in contour order.
    pub spans: &'a [ConicalJoinSpan<'a>],
    /// Outer contour then holes, with no repeated closing spans.
    pub contour_lengths: &'a [usize],
    /// Executed ideal closure and common monotone projections for every adjacent pair.
    pub joins: &'a ConicalJoinsEvidence,
}
/// Conditional reference-body inputs; the caller binds selected original planar material.
pub struct ConicalReferenceQuery<'a> {
    /// All selected members in source-key order.
    pub members: &'a [ConicalReferenceMember<'a>],
    /// Complete nonadjacent original-boundary separation across this family.
    pub clearance: &'a CertifiedCylindricalClearance,
    /// Executed original wedge certificate, including its strict perturbation margin.
    pub sector: &'a ConicalSectorEvidence,
    /// Original unsplit polynomials used by the sector proof.
    pub sector_spans: &'a [&'a [[f64; 2]]],
    /// Original requested physical spatial error.
    pub spatial_limit: f64,
    /// Remaining global arithmetic budget.
    pub options: ConeClearanceOptions,
}
/// Source-bound completed reference homotopy, full-depth normal map and storage allocation.
#[derive(Debug)]
pub struct ConicalReferenceEvidence {
    input: Vec<Vec<BoundarySnapshot>>,
    lengths: Vec<Vec<usize>>,
    spatial_limit: u64,
    options: ConeClearanceOptions,
    components: Vec<u64>,
    developed: f64,
    filled: f64,
    storage: f64,
    operations: u32,
}
/// A missing source binding, failed bound or incomplete arithmetic certificate.
#[derive(Debug)]
pub struct ConicalReferenceError {
    /// Actual member and local span when a particular input failed.
    pub offender: Option<[usize; 2]>,
    /// Specific failed obligation.
    pub cause: ConicalReferenceFailure,
    /// Arithmetic work completed before refusal.
    pub operations: u32,
}
/// Typed reference-body composition refusal.
#[derive(Debug)]
pub enum ConicalReferenceFailure {
    /// The named actual inputs do not have the required source correspondence.
    Binding {
        /// Failed source field.
        field: &'static str,
    },
    /// An outward upper bound exceeds its allocated limit.
    Bound {
        /// Failed bound.
        field: &'static str,
        /// Outward actual value.
        actual: f64,
        /// Original allowance.
        limit: f64,
    },
    /// Arithmetic did not finish under the remaining resources.
    Arithmetic(BoundedArithmeticFailure),
    /// Trigonometric enclosure did not finish.
    Trigonometric(CylindricalProfileFailure),
}
impl ConicalReferenceEvidence {
    /// Original spatial allowance; `proof.requested_spatial_limit()`.
    pub fn requested_spatial_limit(&self) -> f64 {
        f64::from_bits(self.spatial_limit)
    }
    /// Exact remaining policy used by this composition; `proof.options()`.
    pub fn options(&self) -> ConeClearanceOptions {
        self.options
    }

    /// Original-to-ideal developed displacement; `proof.developed_error()`.
    pub fn developed_error(&self) -> f64 {
        self.developed
    }
    /// Full filled reference normal-body discrepancy; `proof.filled_error()`.
    pub fn filled_error(&self) -> f64 {
        self.filled
    }
    /// Maximum decoded edge/ruling roundoff; `proof.storage_error()`.
    pub fn storage_error(&self) -> f64 {
        self.storage
    }
    /// Charged composition work, excluding previously charged receipts; `proof.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Rebind the complete inputs and already executed component proofs; `proof.matches(&query)`.
    pub fn matches(&self, query: &ConicalReferenceQuery<'_>) -> bool {
        binding::matches(self, &ReferenceInputs::from(query))
    }
}
/// Execute the conditional reference-body composition over the declared selected material.
/// Example: `let proof = certify_conical_reference(query, || false)?;`.
pub fn certify_conical_reference(
    query: ConicalReferenceQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalReferenceEvidence, ConicalReferenceError> {
    let query = ReferenceInputs::from(&query);
    binding::admit(&query)?;
    let policy = ArithmeticPolicy {
        fractional_bits: query.options.fractional_bits,
        max_operations: query.options.max_operations,
    };
    let mut arithmetic = Arithmetic::new(policy, &mut cancelled);
    let bounds = geometry::certify(&query, &mut arithmetic)
        .map_err(|cause| error(cause, arithmetic.operations()))?;
    Ok(binding::evidence(&query, bounds, arithmetic.operations()))
}
fn binding_failure(field: &'static str) -> ConicalReferenceFailure {
    ConicalReferenceFailure::Binding { field }
}
fn error(cause: ConicalReferenceFailure, operations: u32) -> ConicalReferenceError {
    ConicalReferenceError {
        offender: None,
        cause,
        operations,
    }
}
impl From<BoundedArithmeticFailure> for ConicalReferenceFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(cause)
    }
}
impl ConicalReferenceError {
    /// Whether bounded execution observed cancellation; `error.is_cancelled()`.
    pub fn is_cancelled(&self) -> bool {
        match &self.cause {
            ConicalReferenceFailure::Arithmetic(cause) => {
                matches!(cause, BoundedArithmeticFailure::Cancelled)
            }
            ConicalReferenceFailure::Trigonometric(CylindricalProfileFailure::Arithmetic(
                cause,
            )) => matches!(cause.as_ref(), BoundedArithmeticFailure::Cancelled),
            ConicalReferenceFailure::Binding { .. }
            | ConicalReferenceFailure::Bound { .. }
            | ConicalReferenceFailure::Trigonometric(_) => false,
        }
    }
}
impl std::fmt::Display for ConicalReferenceError {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            out,
            "reference member/span={:?}: {:?}; work={}",
            self.offender, self.cause, self.operations
        )
    }
}
impl std::error::Error for ConicalReferenceError {}
