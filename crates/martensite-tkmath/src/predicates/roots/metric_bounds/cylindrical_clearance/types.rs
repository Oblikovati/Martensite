// SPDX-License-Identifier: GPL-2.0-only
use super::super::CylindricalProfileInterval;

/// Original developed polynomial on one exact canonical interval.
#[derive(Clone, Copy)]
pub struct CylindricalClearanceSource<'a> {
    /// Original degree-one through degree-three polynomial control points.
    pub authored: &'a [[f64; 2]],
    /// Exact interval in that original polynomial.
    pub interval: CylindricalProfileInterval,
}

/// Pair domain to certify; input positions are local references, not persistent identities.
#[derive(Clone, Copy, Debug)]
pub enum CylindricalClearanceDomain<'a> {
    /// Every unordered distinct pair except the explicitly listed increasing unique pairs.
    Within {
        /// Local input pairs handled by separate shared-endpoint cone proofs.
        excluded: &'a [[usize; 2]],
    },
    /// Every ordered source pair across the original and one translated family, including self pairs.
    Translated {
        /// Exact supplied binary64 translation of the second family's developed coordinates.
        offset: [f64; 2],
    },
}

/// Explicit finite proof resources; `CylindricalClearanceOptions::default()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CylindricalClearanceOptions {
    /// Fixed-grid fractional bits, in `8..=4096`.
    pub fractional_bits: u16,
    /// Total hierarchy and curve-pair domain visits, at most one million.
    pub max_domains: u32,
    /// Total cached curve nodes, including roots; at most 65,536.
    pub max_curve_nodes: u32,
    /// Local dyadic refinement level, at most 32; independent of geometric source intervals.
    pub max_depth: u8,
    /// Charged interval operations, at most one million.
    pub max_operations: u32,
}
impl Default for CylindricalClearanceOptions {
    fn default() -> Self {
        Self {
            fractional_bits: 128,
            max_domains: 100_000,
            max_curve_nodes: 8192,
            max_depth: 20,
            max_operations: 1_000_000,
        }
    }
}

/// Certify original developed profiles before native construction.
#[derive(Clone, Copy)]
pub struct CylindricalClearanceQuery<'a> {
    /// One through 16,384 original polynomial sources.
    pub sources: &'a [CylindricalClearanceSource<'a>],
    /// Positive support radius naming the common developed chart.
    pub support_radius: f64,
    /// Finite chart origin, retained for later O2 source binding.
    pub angular_origin: f64,
    /// Complete requested pair domain, with at most 65,536 exclusions.
    pub domain: CylindricalClearanceDomain<'a>,
    /// Strictly positive Euclidean clearance to establish, never relaxed.
    pub requested_clearance: f64,
    /// Unchanged arithmetic and cache resource limits.
    pub options: CylindricalClearanceOptions,
}

/// Owned description of the certified pair domain.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CylindricalClearanceScope {
    /// Unordered distinct pairs, minus the report's excluded pairs.
    Within,
    /// Ordered pairs against a copy translated by these exact supplied binary64 values.
    Translated([f64; 2]),
}

impl<'a> CylindricalClearanceDomain<'a> {
    pub(super) fn offset(self) -> [f64; 2] {
        match self {
            Self::Within { .. } => [0., 0.],
            Self::Translated { offset } => offset,
        }
    }
    pub(super) fn exclusions(self) -> &'a [[usize; 2]] {
        match self {
            Self::Within { excluded } => excluded,
            Self::Translated { .. } => &[],
        }
    }
    pub(super) fn within(self) -> bool {
        matches!(self, Self::Within { .. })
    }
    pub(super) fn pair_count(self, sources: usize) -> usize {
        match self {
            Self::Within { excluded } => sources * (sources - 1) / 2 - excluded.len(),
            Self::Translated { .. } => sources * sources,
        }
    }
}
