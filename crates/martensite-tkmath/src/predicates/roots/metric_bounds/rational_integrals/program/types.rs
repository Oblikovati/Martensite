// SPDX-License-Identifier: GPL-2.0-only
use super::super::publish::RoundedScalar;
use super::super::{OrientedMetricCurve, RationalIntegralEvidence, RationalIntegralOptions};

/// Explicit additive integration behavior, independent of standalone integration version1.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntegralProgramVersion {
    /// Ordered open components, one exact accumulation and one final outward rounding.
    V1,
}

/// Unclamped admission and aggregate work policy for an explicit integration program.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntegralProgramOptions {
    /// Selected additive behavior.
    pub version: IntegralProgramVersion,
    /// Existing per-component count, per-curve degree/cells and global scalar error limit.
    pub integration: RationalIntegralOptions,
    /// Maximum total components across all loops; allowed range1..64.
    pub max_components: u16,
    /// Aggregate examined cells, at most integration.max_curves*max_subintervals; zero allowed.
    pub max_total_subintervals: u64,
}

/// Ordered open components. Splitting supplies no closure, coverage or orientation proof.
#[derive(Clone, Copy)]
pub struct BoundaryIntegralProgramQuery<'a> {
    /// Every component is nonempty and separately respects the old occurrence maximum.
    pub components: &'a [&'a [OrientedMetricCurve<'a>]],
    /// Explicit program policy, never silently selected by a standalone call.
    pub options: IntegralProgramOptions,
}

/// One whole loop's ordered open components; its sign is computed only after their sum.
#[derive(Clone, Copy)]
pub struct IntegralProgramLoop<'a> {
    /// Ordered nonempty open chains; no component is assigned a separate orientation.
    pub components: &'a [&'a [OrientedMetricCurve<'a>]],
}

/// The first whole loop is outer; subsequent whole loops are holes.
#[derive(Clone, Copy)]
pub struct FaceIntegralProgramQuery<'a> {
    /// Nonempty ordered loops, bound by role as well as actual source bits.
    pub loops: &'a [IntegralProgramLoop<'a>],
    /// One aggregate work and error policy for the complete face scalar.
    pub options: IntegralProgramOptions,
}

/// Immutable scalar receipt; region validity remains an independent caller obligation.
#[derive(Clone, Debug)]
pub struct CertifiedIntegralProgram {
    pub(super) scalar: RoundedScalar,
    pub(super) options: IntegralProgramOptions,
    pub(super) bits: Vec<u64>,
    pub(super) occurrences: usize,
}
impl CertifiedIntegralProgram {
    /// Certified scalar enclosure; `let interval = proof.enclosure();`.
    pub fn enclosure(&self) -> [f64; 2] {
        self.scalar.enclosure
    }
    /// Deterministic lower-rounded midpoint; `let scalar = proof.representative();`.
    pub fn representative(&self) -> f64 {
        self.scalar.representative
    }
    /// Certified discrepancy from the midpoint; `let error = proof.absolute_error_bound();`.
    pub fn absolute_error_bound(&self) -> f64 {
        self.scalar.absolute_error_bound
    }
    /// Original explicit policy; `let options = proof.requested_options();`.
    pub fn requested_options(&self) -> IntegralProgramOptions {
        self.options
    }
    /// Unclamped policy; `assert_eq!(proof.effective_options(), proof.requested_options());`.
    pub fn effective_options(&self) -> IntegralProgramOptions {
        self.options
    }
    /// Numerical method, without a topology claim; `let method = proof.evidence();`.
    pub fn evidence(&self) -> RationalIntegralEvidence {
        self.scalar.evidence
    }
    /// Actually completed occurrences, including exact zero-cell curves; `let n = proof.completed_occurrences();`.
    pub fn completed_occurrences(&self) -> usize {
        self.occurrences
    }
    /// Aggregate examined rational cells; `let n = proof.integration_subintervals();`.
    pub fn integration_subintervals(&self) -> u64 {
        self.scalar.subintervals
    }
    /// Bind actual sources, order, partitions and policy; `assert!(proof.matches_boundary(query));`.
    pub fn matches_boundary(&self, query: BoundaryIntegralProgramQuery<'_>) -> bool {
        let loops = [IntegralProgramLoop {
            components: query.components,
        }];
        super::binding::same_options(self.options, query.options)
            && self.bits == super::binding::bits(&loops, false)
    }
    /// Bind whole-loop roles and sources; `assert!(proof.matches_face(query));`.
    pub fn matches_face(&self, query: FaceIntegralProgramQuery<'_>) -> bool {
        super::binding::same_options(self.options, query.options)
            && self.bits == super::binding::bits(query.loops, true)
    }
}

impl PartialEq for CertifiedIntegralProgram {
    fn eq(&self, other: &Self) -> bool {
        super::binding::same_options(self.options, other.options)
            && self.bits == other.bits
            && self.scalar == other.scalar
            && self.occurrences == other.occurrences
    }
}
