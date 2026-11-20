// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::metric_bounds::MetricBezier;

/// Stored +Z cone datum; `ConicalTrimCarrier { radius: 2., axial_origin: 3., semi_angle: 0.5 }`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConicalTrimCarrier {
    /// Reference-circle radius, in model units.
    pub radius: f64,
    /// Reference-circle Z coordinate, in model units.
    pub axial_origin: f64,
    /// Actual stored angle in radians, strictly between zero and pi/2.
    pub semi_angle: f64,
}

/// Retained rational edge and polynomial `(U,V)` trace on one analytic cone.
#[derive(Clone, Copy)]
pub struct ConicalTrimQuery<'a> {
    /// Degree-one through nine edge on `[0,1]`.
    pub curve: MetricBezier<'a, 3>,
    /// Degree-one through three retained angular/meridional controls on `[0,1]`.
    pub pcurve: &'a [[f64; 2]],
    /// Actual decoded local carrier; rigid placement belongs to the caller.
    pub carrier: ConicalTrimCarrier,
    /// Nonnegative full-interval Euclidean discrepancy allowance.
    pub spatial_limit: f64,
    /// Unclamped settings of the shared residual engine.
    pub options: CylindricalProfileOptions,
}

/// Full-interval correspondence bound; no topology or injectivity verdict.
#[derive(Clone, Debug)]
pub struct CertifiedConicalTrim {
    pub(super) bounds: [f64; 3],
    pub(super) bits: Vec<u64>,
    pub(super) limit: f64,
    pub(super) options: CylindricalProfileOptions,
    pub(super) operations: u32,
}
impl CertifiedConicalTrim {
    /// XY L1, axial, and Euclidean position bounds in model units; `proof.residual_bounds()`.
    pub fn residual_bounds(&self) -> [f64; 3] {
        self.bounds
    }
    /// Original allowance; `proof.requested_spatial_limit()`.
    pub fn requested_spatial_limit(&self) -> f64 {
        self.limit
    }
    /// Original unclamped policy; `proof.requested_options()`.
    pub fn requested_options(&self) -> CylindricalProfileOptions {
        self.options
    }
    /// Charged operations including the cone meridian; `proof.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Bind the actual carrier and retained curves; `assert!(proof.matches_source(query))`.
    pub fn matches_source(&self, query: ConicalTrimQuery<'_>) -> bool {
        admit(query).is_ok() && self.bits == source_bits(query)
    }
}

pub(super) fn source_bits(query: ConicalTrimQuery<'_>) -> Vec<u64> {
    let mut bits = super::super::types::source_bits(CylindricalTrimQuery {
        curve: query.curve,
        pcurve: query.pcurve,
        radius: query.carrier.radius,
        spatial_limit: query.spatial_limit,
        options: query.options,
    });
    bits.extend([
        query.carrier.axial_origin.to_bits(),
        query.carrier.semi_angle.to_bits(),
    ]);
    bits
}

/// Unsupported input, arithmetic exhaustion or an unproved discrepancy.
#[derive(Clone, Debug, PartialEq)]
pub struct ConicalTrimError {
    /// Actual retained cone datum.
    pub carrier: ConicalTrimCarrier,
    /// Original full-interval allowance.
    pub requested_spatial_limit: f64,
    /// Original arithmetic policy.
    pub requested_options: CylindricalProfileOptions,
    /// Original shared numerical failure.
    pub cause: CylindricalProfileFailure,
}
impl std::fmt::Display for ConicalTrimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "conical trim carrier={:?}, limit={}: {}; expected positive-radius +Z cone and complete degree-1..=3 UV / degree-1..=9 rational edge correspondence",
            self.carrier, self.requested_spatial_limit, self.cause
        )
    }
}
impl std::error::Error for ConicalTrimError {}
