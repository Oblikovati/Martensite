// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::metric_bounds::MetricBezier;

/// Actual same-parameter traces on a local +Z cylinder at the origin.
#[derive(Clone, Copy)]
pub struct CylindricalTrimQuery<'a> {
    /// Retained 3D rational Bezier, degree one through nine on `[0,1]`.
    pub curve: MetricBezier<'a, 3>,
    /// Retained polynomial Bezier `(angle,height)` controls. The ranged API permits degree five.
    pub pcurve: &'a [[f64; 2]],
    /// Actual positive cylinder radius; placement/frame admission belongs to the caller.
    pub radius: f64,
    /// Nonnegative full-interval Euclidean discrepancy allowance.
    pub spatial_limit: f64,
    /// Explicit finite precision/work policy, shared with the profile enclosure engine.
    pub options: CylindricalProfileOptions,
}

/// Full-interval space-curve versus actual cylinder/pcurve discrepancy.
/// The conservative bound sums XY L1 and height residuals; it makes no topology claim.
#[derive(Clone, Debug)]
pub struct CertifiedCylindricalTrim {
    pub(super) bounds: [f64; 3],
    pub(super) bits: Vec<u64>,
    pub(super) ranges: [[f64; 2]; 2],
    pub(super) limit: f64,
    pub(super) options: CylindricalProfileOptions,
    pub(super) operations: u32,
}
impl CertifiedCylindricalTrim {
    /// Bound on `|X-r cos(U)| + |Y-r sin(U)|`; `proof.xy_residual_bound()`.
    pub fn xy_residual_bound(&self) -> f64 {
        self.bounds[0]
    }
    /// Bound on `|Z-V|`; `proof.height_residual_bound()`.
    pub fn height_residual_bound(&self) -> f64 {
        self.bounds[1]
    }
    /// Euclidean-distance upper bound, rounded after the interval sum; `proof.spatial_residual_bound()`.
    pub fn spatial_residual_bound(&self) -> f64 {
        self.bounds[2]
    }
    /// Original unclamped spatial cap; `proof.requested_spatial_limit()`.
    pub fn requested_spatial_limit(&self) -> f64 {
        self.limit
    }
    /// Original arithmetic settings; `proof.requested_options()`.
    pub fn requested_options(&self) -> CylindricalProfileOptions {
        self.options
    }
    /// Charged interval operations; `proof.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Compare geometry bits independently of limit/work policy; this does not recertify another limit.
    /// Example: `assert!(proof.matches_source(query));`.
    pub fn matches_source(&self, query: CylindricalTrimQuery<'_>) -> bool {
        self.matches_ranges(query, [[0., 1.]; 2])
    }

    /// Exact ranges certified on the space curve and pcurve. Example: `proof.ranges()`.
    pub fn ranges(&self) -> [[f64; 2]; 2] {
        self.ranges
    }

    /// Compare parent coefficients, support radius and active intervals.
    /// Example: `assert!(proof.matches_ranges(query, proof.ranges()));`.
    pub fn matches_ranges(&self, query: CylindricalTrimQuery<'_>, ranges: [[f64; 2]; 2]) -> bool {
        admit(query, 6).is_ok()
            && self.ranges.map(|pair| pair.map(f64::to_bits))
                == ranges.map(|pair| pair.map(f64::to_bits))
            && self.bits == source_bits(query)
    }
}

pub(super) fn source_bits(query: CylindricalTrimQuery<'_>) -> Vec<u64> {
    let mut bits = vec![
        query.radius.to_bits(),
        query.pcurve.len() as u64,
        query.curve.poles.len() as u64,
    ];
    bits.extend(query.pcurve.iter().flatten().map(|v| v.to_bits()));
    bits.extend(query.curve.poles.iter().flatten().map(|v| v.to_bits()));
    bits.push(query.curve.weights.map_or(u64::MAX, |w| w.len() as u64));
    bits.extend(
        query
            .curve
            .weights
            .into_iter()
            .flatten()
            .map(|v| v.to_bits()),
    );
    bits
}

/// No certificate accompanies invalid input or incomplete full-interval evidence.
#[derive(Clone, Debug, PartialEq)]
pub struct CylindricalTrimError {
    /// Actual cylinder radius.
    pub radius: f64,
    /// Original requested spatial allowance.
    pub requested_spatial_limit: f64,
    /// Original arithmetic settings.
    pub requested_options: CylindricalProfileOptions,
    /// Typed original admission/enclosure failure.
    pub cause: CylindricalProfileFailure,
}
impl std::fmt::Display for CylindricalTrimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "cylindrical cap trim radius={}, limit={}: {:?}; expected finite polynomial UV controls within the selected proof's degree limit, degree-1..=9 space controls, positive weights/radius and a complete spatial bound",
            self.radius, self.requested_spatial_limit, self.cause
        )
    }
}
impl std::error::Error for CylindricalTrimError {}
