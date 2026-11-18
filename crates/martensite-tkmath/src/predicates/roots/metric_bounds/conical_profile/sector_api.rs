// SPDX-License-Identifier: GPL-2.0-only
//! Source-bound certificate for a conditional single-turn conical sector.
use super::ConeClearanceOptions;
use super::DyadicBound;
use crate::predicates::roots::certificate_arithmetic::bounded_dyadic::ArithmeticPolicy;

pub use super::sector::ConicalSectorWedge;
pub use super::sector::SectorFailure as ConicalSectorFailure;

/// Authored developed-plane spans and the cone's semi-angle.
///
/// This certificate is a sufficient sector condition, not a complete body
/// validity or angle-lift proof. The caller must establish complete boundary
/// coverage, a consistent lift, and positive radius across the material and depths.
/// It accepts 1 through 128 spans, each with 2 through 4 control points, and
/// requires one strict quarter-turn wedge and a certified sine above `1/4`.
pub struct ConicalSectorQuery<'a> {
    /// Complete polynomial control polygons in developed-plane coordinates.
    pub spans: &'a [&'a [[f64; 2]]],
    /// Cone semi-angle in radians.
    pub semi_angle: f64,
    /// Bounded arithmetic precision and work limit.
    pub options: ConeClearanceOptions,
}

/// Source-bound proof that all supplied control polygons occupy one strict wedge.
///
/// With the query's stated caller obligations, the wedge maps into less than one
/// physical revolution. A refused query can still describe a valid region.
pub struct ConicalSectorEvidence {
    pub(super) sector: super::sector::SectorEvidence,
}

/// Certify the sufficient sector condition; `certify_conical_sector(query, || false)?`.
pub fn certify_conical_sector(
    query: ConicalSectorQuery<'_>,
    cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalSectorEvidence, ConicalSectorFailure> {
    let sector = super::sector::certify(
        query.spans,
        query.semi_angle,
        policy(query.options),
        cancelled,
    )?;
    Ok(ConicalSectorEvidence { sector })
}

impl ConicalSectorEvidence {
    /// Read the certified wedge for a continuous source-angle lift; `proof.wedge()`.
    pub fn wedge(&self) -> Option<ConicalSectorWedge> {
        self.sector.wedge()
    }
    /// Verify the original span bits, angle and policy; `proof.matches(&query)`.
    pub fn matches(&self, query: &ConicalSectorQuery<'_>) -> bool {
        self.sector
            .matches(query.spans, query.semi_angle, policy(query.options))
    }

    /// Check a nonnegative developed-plane displacement against the strict margin.
    /// Example: `proof.admits_bound(&bound)`.
    pub fn admits_bound(&self, bound: &DyadicBound) -> bool {
        self.sector.admits_bound(bound)
    }

    /// Report charged arithmetic work; `proof.operations()`.
    pub fn operations(&self) -> u32 {
        self.sector.operations
    }
}

pub(super) fn policy(options: ConeClearanceOptions) -> ArithmeticPolicy {
    ArithmeticPolicy {
        fractional_bits: options.fractional_bits,
        max_operations: options.max_operations,
    }
}
