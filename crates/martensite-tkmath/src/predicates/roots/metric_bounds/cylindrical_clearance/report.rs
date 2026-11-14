// SPDX-License-Identifier: GPL-2.0-only
use super::super::CylindricalProfileSource;
use super::*;

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Statistics {
    pub domains: u32,
    pub root_pairs: usize,
    pub curve_nodes: u32,
    pub curve_splits: u32,
    pub hierarchy_nodes: usize,
}

/// Certified lower clearance for the complete declared relation, including explicitly empty relations.
/// Excluded pairs, self-regularity, native incidence and periodic-translation error need separate proofs.
#[derive(Clone, Debug)]
pub struct CertifiedCylindricalClearance {
    sources: Vec<Vec<u64>>,
    radius: f64,
    origin: f64,
    clearance: f64,
    scope: CylindricalClearanceScope,
    excluded: Vec<[usize; 2]>,
    pair_count: usize,
    options: CylindricalClearanceOptions,
    statistics: Statistics,
    operations: u32,
}
impl CertifiedCylindricalClearance {
    /// Established lower bound, not an estimate of the tight minimum; `proof.clearance_lower_bound()`.
    pub fn clearance_lower_bound(&self) -> f64 {
        self.clearance
    }
    /// Declared relation, with any exact supplied translation; `proof.scope()`.
    pub fn scope(&self) -> CylindricalClearanceScope {
        self.scope
    }
    /// Original pairs requiring separate proof; `proof.excluded_pairs()`.
    pub fn excluded_pairs(&self) -> &[[usize; 2]] {
        &self.excluded
    }
    /// Number of distinct requested pairs; zero is a completed empty relation.
    /// Example: `proof.pair_count()`.
    pub fn pair_count(&self) -> usize {
        self.pair_count
    }
    /// Pairs discharged by certified hierarchy bounds; `proof.pruned_pairs()`.
    pub fn pruned_pairs(&self) -> usize {
        self.pair_count - self.statistics.root_pairs
    }
    /// Pairs whose polynomial hulls needed inspection; `proof.inspected_pairs()`.
    pub fn inspected_pairs(&self) -> usize {
        self.statistics.root_pairs
    }
    /// Total visited hierarchy and curve-pair domains; `proof.domain_visits()`.
    pub fn domain_visits(&self) -> u32 {
        self.statistics.domains
    }
    /// Cached curve nodes including roots; `proof.curve_nodes()`.
    pub fn curve_nodes(&self) -> u32 {
        self.statistics.curve_nodes
    }
    /// Subdivisions computed once and reused below roots; `proof.curve_splits()`.
    pub fn curve_splits(&self) -> u32 {
        self.statistics.curve_splits
    }
    /// Number of bounding hierarchy nodes; `proof.hierarchy_nodes()`.
    pub fn hierarchy_nodes(&self) -> usize {
        self.statistics.hierarchy_nodes
    }
    /// Charged interval operations; `proof.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Original unclamped settings; `proof.requested_options()`.
    pub fn requested_options(&self) -> CylindricalClearanceOptions {
        self.options
    }
    /// Compare original polynomial and exact interval bits; `proof.matches_sources(query.sources)`.
    pub fn matches_sources(&self, sources: &[CylindricalClearanceSource<'_>]) -> bool {
        self.sources.len() == sources.len()
            && self
                .sources
                .iter()
                .zip(sources)
                .all(|(old, new)| original_matches(old, new))
    }
    /// Bind the authored-clearance proof to the later retained O2 certificates and chart.
    /// Example: `assert!(proof.matches_profiles(&profiles));`.
    pub fn matches_profiles(&self, sources: &[CylindricalProfileSource<'_>]) -> bool {
        self.sources.len() == sources.len()
            && self.sources.iter().zip(sources).all(|(old, new)| {
                new.matches()
                    && new.support_radius.to_bits() == self.radius.to_bits()
                    && new.angular_origin.to_bits() == self.origin.to_bits()
                    && original_matches(
                        old,
                        &CylindricalClearanceSource {
                            authored: new.authored,
                            interval: new.interval,
                        },
                    )
            })
    }
    /// Declared support radius; `proof.support_radius()`.
    pub fn support_radius(&self) -> f64 {
        self.radius
    }
    /// Declared chart origin; `proof.angular_origin()`.
    pub fn angular_origin(&self) -> f64 {
        self.origin
    }
    pub(super) fn assemble(
        query: CylindricalClearanceQuery<'_>,
        pair_count: usize,
        statistics: Statistics,
        operations: u32,
    ) -> Self {
        let (scope, excluded) = owned_domain(query.domain);
        Self {
            sources: query.sources.iter().map(source_bits).collect(),
            radius: query.support_radius,
            origin: query.angular_origin,
            clearance: query.requested_clearance,
            scope,
            excluded,
            pair_count,
            options: query.options,
            statistics,
            operations,
        }
    }
}

fn source_bits(source: &CylindricalClearanceSource<'_>) -> Vec<u64> {
    let mut bits = vec![
        source.interval.index(),
        source.interval.level().into(),
        source.authored.len() as u64,
    ];
    bits.extend(source.authored.iter().flatten().map(|v| v.to_bits()));
    bits
}

fn original_matches(old: &[u64], source: &CylindricalClearanceSource<'_>) -> bool {
    (2..=4).contains(&source.authored.len()) && old == source_bits(source)
}

fn owned_domain(
    domain: CylindricalClearanceDomain<'_>,
) -> (CylindricalClearanceScope, Vec<[usize; 2]>) {
    match domain {
        CylindricalClearanceDomain::Within { excluded } => {
            (CylindricalClearanceScope::Within, excluded.to_vec())
        }
        CylindricalClearanceDomain::Translated { offset } => {
            (CylindricalClearanceScope::Translated(offset), Vec::new())
        }
    }
}
