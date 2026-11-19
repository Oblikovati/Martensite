// SPDX-License-Identifier: GPL-2.0-only
//! Immutable complete numeric inventory; TKBO independently owns filled-material/native authority.
use super::*;
/// Actual completed coverage counts, not a substitute for the immutable source certificate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConicalActualPatchCoverage {
    /// Individual sides covered by verified whole unions.
    pub side_self_domains: usize,
    /// Actual shared-ruling union occurrences proved injective.
    pub adjacent_unions: usize,
    /// Strictly hull-excluded unordered nonincident side pairs.
    pub nonincident_pairs: u64,
    /// All side/carrier pairs, including cross-member pairs.
    pub side_cap_pairs: u64,
    /// Entire cap UV rectangles proved injective.
    pub cap_self_domains: usize,
    /// All distinct cap pairs separated.
    pub cap_pairs: u64,
    /// Actual hierarchy nodes.
    pub hierarchy_nodes: usize,
    /// Charged hierarchy node-pair visits.
    pub hierarchy_visits: u64,
}
/// Complete numeric patch coverage, requiring independently rebound selected compact material.
/// This certifies cap rectangles and actual rational sides; TKBO binds filled trims and native identity.
pub struct ConicalActualPatchCertificate {
    pub(super) source_bits: Vec<u64>,
    pub(super) gamma: ConicalDirectedGammaEvidence,
    pub(super) sides: Vec<ConicalActualSideSnapshot>,
    pub(super) caps: Vec<ConicalActualCapSnapshot>,
    pub(super) adjacent: Vec<ConicalActualAdjacent>,
    pub(super) members: Vec<ConicalActualMemberId>,
    pub(super) options: ConeClearanceOptions,
    pub(super) operations: u32,
    pub(super) coverage: ConicalActualPatchCoverage,
}
impl std::fmt::Debug for ConicalActualPatchCertificate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConicalActualPatchCertificate")
            .field("members", &self.members)
            .field("options", &self.options)
            .field("operations", &self.operations)
            .field("coverage", &self.coverage)
            .finish()
    }
}
impl ConicalActualPatchCertificate {
    /// Compare complete sources, issuing authorities, modes and grants; `proof.matches(&query)`.
    pub fn matches(&self, query: &ConicalActualPatchQuery<'_>) -> bool {
        self.options == query.options
            && self.gamma.same_authority(query.gamma)
            && admission::admit(query, &mut error::FailurePosition::default()).is_ok()
            && self.source_bits == binding::bits(query)
    }
    /// Actual charged work, excluding already-paid ancestry; `proof.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Original producing remaining grant; `proof.options()`.
    pub fn options(&self) -> ConeClearanceOptions {
        self.options
    }
    /// Actual numeric coverage census; `proof.coverage()`.
    pub fn coverage(&self) -> &ConicalActualPatchCoverage {
        &self.coverage
    }
    /// Complete immutable actual side sources; `proof.native_sides()`.
    pub fn native_sides(&self) -> &[ConicalActualSideSnapshot] {
        &self.sides
    }
    /// Complete immutable actual cap sources; `proof.native_caps()`.
    pub fn native_caps(&self) -> &[ConicalActualCapSnapshot] {
        &self.caps
    }
    /// Actual declared shared-ruling inventory; `proof.adjacencies()`.
    pub fn adjacencies(&self) -> &[ConicalActualAdjacent] {
        &self.adjacent
    }
    /// Exact source membership; `proof.member_ids()`.
    pub fn member_ids(&self) -> &[ConicalActualMemberId] {
        &self.members
    }
}
