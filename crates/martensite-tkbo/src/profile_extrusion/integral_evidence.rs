// SPDX-License-Identifier: GPL-2.0-only
//! Immutable issued programs retain source lineage, original policy and the actual loan.
use super::*;
use crate::sketch_region_prism::GraphIntegralEvidence;
use std::sync::Arc;
mod snapshot;
/// Source-bound whole-orbit and whole-face integral generation evidence.
/// Example: `set.integral_program_evidence().map(|proof| proof.integration_subintervals())`.
#[derive(Clone, Debug, PartialEq)]
pub struct PlanarIntegralEvidence {
    pub(crate) requested: PlanarProofVersion,
    pub(crate) entry_allowance: u64,
    pub(crate) graph: Arc<GraphIntegralEvidence>,
    pub(crate) source: SketchProfile,
    pub(crate) source_bits: Vec<u64>,
    pub(crate) parents: Vec<(u64, ProfileCurveId, usize)>,
}
impl PlanarIntegralEvidence {
    /// Original selectable behavior, before the caller loan; `proof.requested_proof_version()`.
    pub fn requested_proof_version(&self) -> PlanarProofVersion {
        self.requested
    }
    /// Finite remaining caller allowance; `proof.entry_allowance()`.
    pub fn entry_allowance(&self) -> u64 {
        self.entry_allowance
    }
    /// Actual graph-wide grant; `proof.granted_subintervals()`.
    pub fn granted_subintervals(&self) -> u64 {
        self.graph.options.max_total_subintervals
    }
    /// Actual examined cells, charged once; `proof.integration_subintervals()`.
    pub fn integration_subintervals(&self) -> u64 {
        self.graph.consumed
    }
    /// Exact original source and all retained program roles; `proof.matches_profile(&profile)`.
    pub fn matches_profile(&self, profile: &SketchProfile) -> bool {
        self.source == *profile
            && self.source_bits == snapshot::bits(profile)
            && self.graph.matches_issued_sources()
            && self.valid_grant()
            && self.valid_lineage()
    }
    fn valid_grant(&self) -> bool {
        let Some(mut requested) = self.requested.integral_options() else {
            return false;
        };
        requested.max_total_subintervals =
            requested.max_total_subintervals.min(self.entry_allowance);
        self.graph.options == requested
    }
    pub(crate) fn issue(
        requested: PlanarProofVersion,
        entry_allowance: u64,
        graph: Arc<GraphIntegralEvidence>,
        source: &SketchProfile,
        parents: Vec<(u64, ProfileCurveId, usize)>,
    ) -> Self {
        Self {
            requested,
            entry_allowance,
            graph,
            source: source.clone(),
            source_bits: snapshot::bits(source),
            parents,
        }
    }
}

impl PlanarIntegralEvidence {
    fn valid_lineage(&self) -> bool {
        if self.parents.len() != self.graph.sources.len() {
            return false;
        }
        let mut indices = std::collections::BTreeMap::<ProfileCurveId, usize>::new();
        for ((source, curve, piece), actual) in self.parents.iter().zip(&self.graph.sources) {
            if !self.has_parent(*curve, *source, actual.id.0) {
                return false;
            }
            let expected = indices.entry(*curve).or_default();
            if *piece != *expected {
                return false;
            }
            *expected += 1;
        }
        true
    }
    fn has_parent(&self, curve: ProfileCurveId, source: u64, actual: u64) -> bool {
        source == actual
            && self
                .source
                .curves
                .iter()
                .any(|authored| authored.id() == curve)
    }
}
