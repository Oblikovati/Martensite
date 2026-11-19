// SPDX-License-Identifier: GPL-2.0-only
use super::*;

/// One ruled patch and its two actual polynomial cap-pcurve occurrences.
#[derive(Clone, Copy)]
pub struct CylindricalFluxSpan<'a> {
    /// First/last depth rational Beziers, with equal pole counts and equal positive weights.
    pub boundaries: [MetricBezier<'a, 3>; 2],
    /// Actual `(angle,height)` cap pcurves on their complete `[0,1]` domains.
    pub cap_pcurves: [&'a [[f64; 2]]; 2],
    /// Reverse the side's `du cross dv` orientation; cap roles stay first inward/last outward.
    pub side_reversed: bool,
}

/// Finite conditional flux policy; independent of spatial modelling tolerances.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CylindricalFluxOptions {
    /// Behavior1 remains the default; conical entries additionally admit outward aggregate behavior2.
    pub behavior_version: u32,
    /// Maximum stored degree, in `0..=16`.
    pub max_degree: u16,
    /// Maximum supplied spans, in `1..=4096`; no input is truncated.
    pub max_spans: u32,
    /// Finite nonnegative absolute error cap in volume units.
    pub absolute_error_limit: f64,
    /// Examined integration cells per ruled span, in `1..=65536`.
    pub max_subintervals_per_span: u32,
    /// Aggregate examined cells, at most one million; zero permits exact polynomial work only.
    pub max_total_subintervals: u32,
}
impl Default for CylindricalFluxOptions {
    fn default() -> Self {
        Self {
            behavior_version: 1,
            max_degree: 9,
            max_spans: 1024,
            absolute_error_limit: 1e-9,
            max_subintervals_per_span: 1024,
            max_total_subintervals: 8192,
        }
    }
}
impl CylindricalFluxOptions {
    pub(in crate::predicates::roots::metric_bounds::rational_integrals) fn engine(
        self,
        remaining: u32,
    ) -> RationalIntegralOptions {
        RationalIntegralOptions {
            behavior_version: self.behavior_version,
            max_degree: self.max_degree,
            max_curves: 1,
            absolute_error_limit: self.absolute_error_limit,
            max_subintervals: self.max_subintervals_per_span.min(remaining),
        }
    }
}

/// All supplied occurrences are integrated; coverage and geometric validity belong to the caller.
#[derive(Clone, Copy)]
pub struct CylindricalFluxQuery<'a> {
    /// Oriented ruled patches and cap boundary uses, in caller-declared order.
    pub spans: &'a [CylindricalFluxSpan<'a>],
    /// Actual positive ordered radii of local +Z cylindrical caps at the origin.
    pub radii: [f64; 2],
    /// Original explicit error/work limits.
    pub options: CylindricalFluxOptions,
}

/// One local span's effective remaining allowance and measured integration work.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CylindricalFluxWork {
    /// Minimum of the per-span cap and the aggregate cells remaining on entry.
    pub effective_limit: u32,
    /// Accepted plus refined integration cells actually examined.
    pub examined_subintervals: u32,
}

/// Certified signed boundary flux of `F(x)=x/3`, conditional on the declared occurrences.
/// This is volume only after separate closed-boundary, orientation and injectivity proofs.
#[derive(Clone, Debug)]
pub struct CertifiedCylindricalFlux {
    pub(super) scalar: publish::RoundedScalar,
    pub(super) bits: Vec<u64>,
    pub(super) options: CylindricalFluxOptions,
    pub(super) spans: usize,
    pub(super) work: Vec<CylindricalFluxWork>,
}
impl CertifiedCylindricalFlux {
    /// Outward signed flux enclosure; `proof.enclosure()`.
    pub fn enclosure(&self) -> [f64; 2] {
        self.scalar.enclosure
    }
    /// Deterministic lower-rounded enclosure midpoint; `proof.representative()`.
    pub fn representative(&self) -> f64 {
        self.scalar.representative
    }
    /// Certified error from that representative; `proof.absolute_error_bound()`.
    pub fn absolute_error_bound(&self) -> f64 {
        self.scalar.absolute_error_bound
    }
    /// Original unclamped policy; `proof.requested_options()`.
    pub fn requested_options(&self) -> CylindricalFluxOptions {
        self.options
    }
    /// Exact-rational or error-bounded integration evidence; `proof.evidence()`.
    pub fn evidence(&self) -> RationalIntegralEvidence {
        self.scalar.evidence
    }
    /// Aggregate examined ruled-span cells; `proof.integration_subintervals()`.
    pub fn integration_subintervals(&self) -> u64 {
        self.scalar.subintervals
    }
    /// Work receipts in supplied span order, as local references; `proof.span_work()`.
    pub fn span_work(&self) -> &[CylindricalFluxWork] {
        &self.work
    }
    /// Complete occurrence count; `proof.span_count()`.
    pub fn span_count(&self) -> usize {
        self.spans
    }
    /// Bind geometry/radii/orientation bits, independently of error/work policy; `proof.matches_source(query)`.
    pub fn matches_source(&self, query: CylindricalFluxQuery<'_>) -> bool {
        admission::admit(query).is_ok() && self.bits == binding::source_bits(query)
    }
}
