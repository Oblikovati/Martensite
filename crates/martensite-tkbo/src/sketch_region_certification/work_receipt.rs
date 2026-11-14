// SPDX-License-Identifier: GPL-2.0-only
//! Value-only pair-work evidence copied from the validated shared census ledger.
use super::BoundaryCensus;

/// Shared pair work, distinct from geometric tolerances and per-pair remaining work.
/// Example: `let work = regions.proof_work(); assert!(work.consumed_pair_bisections <= u64::from(work.effective_pair_bisections));`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CensusWorkReceipt {
    /// Requested maximum shared pair bisections for this operation.
    pub requested_pair_bisections: u16,
    /// Applied maximum shared pair bisections; no implicit clamp is used.
    pub effective_pair_bisections: u16,
    /// Actual completed shared work, counting reused reports once.
    pub consumed_pair_bisections: u64,
}
impl CensusWorkReceipt {
    pub(crate) fn capture(census: &BoundaryCensus) -> Self {
        let maximum = census.requested.roots.max_bisections;
        Self {
            requested_pair_bisections: maximum,
            effective_pair_bisections: maximum,
            consumed_pair_bisections: census.bisections,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::sketch_region_certification::paired_tests::{curved_pair, paired};
    #[test]
    fn receipt_counts_unique_reports_instead_of_alias_domains() {
        let mut shapes = curved_pair();
        shapes.push(shapes[0].reversed());
        let census = paired(&shapes);
        let reports = census.paired.as_ref().unwrap();
        assert!(reports.reused_reports > 0);
        let independent: u64 = reports.reports.iter().map(|r| r.completed_work()).sum();
        let receipt = CensusWorkReceipt::capture(&census);
        assert_eq!(receipt.consumed_pair_bisections, independent);
        assert_eq!(
            receipt.requested_pair_bisections,
            reports.requested.roots.max_bisections
        );
        assert_eq!(
            receipt.effective_pair_bisections,
            receipt.requested_pair_bisections
        );
    }
    #[test]
    fn work_receipt_is_owned_send_and_sync() {
        static_assertions::assert_impl_all!(CensusWorkReceipt: Send, Sync, Copy);
    }
}
