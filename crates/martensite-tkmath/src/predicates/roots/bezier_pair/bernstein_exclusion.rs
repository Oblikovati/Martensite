// SPDX-License-Identifier: GPL-2.0-only
//! Exact rectangular sign witnesses complement the unchanged version-1/2 pair solver (#539).
use super::equations::PairedEquations;
use super::source_certificate::SourceCertificate;
use super::work_budget::PairWorkBudget;
use super::*;
use crate::predicates::roots::{
    RootIsolationError, certificate_arithmetic::ExactRatio, error::poll,
};
use std::cmp::Ordering;
mod basis;
mod binding;
mod error;
mod execution;
mod types;
mod view;
pub use error::BezierPairExclusionFailure;
pub use types::{
    BezierPairExclusionCompletion, BezierPairExclusionPolicy, BezierPairExclusionReport,
};
pub use view::BezierPairBernsteinExclusion;

#[derive(Debug)]
struct ExclusionProof {
    equations: PairedEquations,
    domains: [SourceCertificate; 2],
    rectangle: [[f64; 2]; 2],
    axis: usize,
    divided_self_diagonal: bool,
    witness: basis::BernsteinWitness,
}
impl CapturedBezierPair {
    /// Try exact sign exclusion over the actual distinct or self-off-diagonal domain.
    /// An inconclusive report permits a caller-owned legacy fallback with its remaining budget.
    /// Example: `let report = query.certify_bernstein_exclusion(policy, || false);`.
    pub fn certify_bernstein_exclusion(
        &self,
        policy: BezierPairExclusionPolicy,
        mut cancelled: impl FnMut() -> bool,
    ) -> BezierPairExclusionReport {
        let budget = PairWorkBudget::new(policy.pair_policy.roots.max_bisections);
        let outcome = execution::execute(self, policy, &budget, &mut cancelled);
        BezierPairExclusionReport {
            source: self.clone(),
            policy,
            work: budget.receipt(),
            outcome,
        }
    }
}

#[cfg(test)]
mod tests;
