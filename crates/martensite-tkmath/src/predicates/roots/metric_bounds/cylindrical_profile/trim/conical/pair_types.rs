// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::metric_bounds::MetricBezier;

/// Two actual cone traces with one shared arithmetic allowance.
#[derive(Clone, Copy)]
pub struct ConicalTrimPairQuery<'a> {
    /// Independently retained rational edges in first/last depth order.
    pub curves: [MetricBezier<'a, 3>; 2],
    /// Independently retained polynomial `(U,V)` controls.
    pub pcurves: [&'a [[f64; 2]]; 2],
    /// Actual decoded analytic carriers for both depths.
    pub carriers: [ConicalTrimCarrier; 2],
    /// Original discrepancy allowances, one for each trace.
    pub spatial_limits: [f64; 2],
    /// Unclamped policy shared by the complete pair.
    pub options: CylindricalProfileOptions,
}
impl<'a> ConicalTrimPairQuery<'a> {
    pub(super) fn queries(self) -> [ConicalTrimQuery<'a>; 2] {
        std::array::from_fn(|depth| ConicalTrimQuery {
            curve: self.curves[depth],
            pcurve: self.pcurves[depth],
            carrier: self.carriers[depth],
            spatial_limit: self.spatial_limits[depth],
            options: self.options,
        })
    }
}

/// Both complete correspondence proofs; no partial pair can be published.
#[derive(Clone, Debug)]
pub struct CertifiedConicalTrimPair {
    pub(super) traces: [CertifiedConicalTrim; 2],
    pub(super) operations: u32,
}
impl CertifiedConicalTrimPair {
    /// Total charged work, counting reused enclosures once; `pair.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Own both proofs; `let [first, last] = pair.into_traces()`.
    /// Their work counts partition the pair: first-trace work, then incremental second-trace work.
    pub fn into_traces(self) -> [CertifiedConicalTrim; 2] {
        self.traces
    }
    /// Bind all actual geometry, limits and policy bits; `assert!(pair.matches_source(query))`.
    pub fn matches_source(&self, query: ConicalTrimPairQuery<'_>) -> bool {
        self.traces
            .iter()
            .zip(query.queries())
            .all(|(trace, input)| {
                trace.matches_source(input)
                    && trace.limit.to_bits() == input.spatial_limit.to_bits()
                    && trace.options == input.options
            })
    }
}

/// Original trace failure with its actual depth and the pair's completed work.
#[derive(Clone, Debug, PartialEq)]
pub struct ConicalTrimPairError {
    /// First trace is zero; second trace is one.
    pub depth: usize,
    /// Original input-bound singleton error.
    pub cause: Box<ConicalTrimError>,
    /// Actual total work before refusal, without partial certificates.
    pub operations: u32,
}
impl std::fmt::Display for ConicalTrimPairError {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            out,
            "cone trim depth={} after {} operations: {}",
            self.depth, self.operations, self.cause
        )
    }
}
impl std::error::Error for ConicalTrimPairError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.cause.as_ref())
    }
}
