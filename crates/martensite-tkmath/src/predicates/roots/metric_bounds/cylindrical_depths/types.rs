// SPDX-License-Identifier: GPL-2.0-only
/// Exact endpoint rounding composed with the existing normal-sweep certificate.
#[derive(Clone, Debug)]
pub struct CertifiedCylindricalDepths {
    radius: f64,
    depths: [f64; 2],
    stored: [f64; 2],
    errors: [f64; 2],
    combined: f64,
    limit: f64,
}
impl CertifiedCylindricalDepths {
    /// Original support radius; `proof.support_radius()`.
    pub fn support_radius(&self) -> f64 {
        self.radius
    }
    /// Original signed depth interval; `proof.requested_depths()`.
    pub fn requested_depths(&self) -> [f64; 2] {
        self.depths
    }
    /// Radius inputs from the retained storage proof; `proof.stored_radii()`.
    pub fn stored_radii(&self) -> [f64; 2] {
        self.stored
    }
    /// Outward bounds on each exact `stored - (radius + depth)` discrepancy.
    /// Example: `proof.radius_rounding_bounds()`.
    pub fn radius_rounding_bounds(&self) -> [f64; 2] {
        self.errors
    }
    /// Bound for linear depth interpolation relative to the exact requested radii.
    /// Example: `assert!(proof.normal_sweep_error_bound() <= proof.requested_limit());`.
    pub fn normal_sweep_error_bound(&self) -> f64 {
        self.combined
    }
    /// Unchanged combined storage allowance; `proof.requested_limit()`.
    pub fn requested_limit(&self) -> f64 {
        self.limit
    }
    pub(super) fn assemble(
        radius: f64,
        depths: [f64; 2],
        stored: [f64; 2],
        errors: [f64; 2],
        combined: f64,
        limit: f64,
    ) -> Self {
        Self {
            radius,
            depths,
            stored,
            errors,
            combined,
            limit,
        }
    }
}

/// Incomplete depth-budget evidence; no successful geometry accompanies this error.
#[derive(Clone, Debug, PartialEq)]
pub enum CylindricalDepthFailure {
    /// A supplied scalar violated its finite, positivity or sign requirement.
    Value {
        /// Failed scalar role.
        field: &'static str,
        /// Endpoint or scalar position.
        index: usize,
        /// Offending input.
        value: f64,
    },
    /// The requested interval must be strictly increasing.
    DepthOrder {
        /// Original requested depth endpoints.
        depths: [f64; 2],
    },
    /// An exact requested radius reaches or crosses the axis.
    NonpositiveRadius {
        /// Endpoint position.
        index: usize,
        /// Original support radius.
        radius: f64,
        /// Original signed offset.
        depth: f64,
    },
    /// A conservative endpoint could not fit finite binary64.
    UnrepresentableBound {
        /// Exact conservative endpoint that did not fit finite binary64.
        bound: crate::predicates::roots::DyadicBound,
    },
    /// The unchanged combined storage allowance was not established.
    Uncertified {
        /// Conservative upper bound.
        bound: f64,
        /// Original allowance.
        requested: f64,
    },
}
impl std::fmt::Display for CylindricalDepthFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "cylindrical depth budget: {self:?}; expected finite positive radii, increasing depths and combined bound within requested limit"
        )
    }
}
impl std::error::Error for CylindricalDepthFailure {}
