// SPDX-License-Identifier: GPL-2.0-only
use super::super::{
    CylindricalStorageFailure, CylindricalStorageOptions, MetricBezier, admission, types,
};
use crate::predicates::roots::certificate_arithmetic::bounded_failure::BoundedArithmeticFailure;

/// Direction of the actual rail motion from the first to the far depth.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CylindricalRadialSense {
    /// Squared radius increases with normalized depth.
    Outward,
    /// Squared radius decreases with normalized depth.
    Inward,
}

/// Common-weight ruled Bezier rails and the required signed radial bound.
#[derive(Clone, Copy)]
pub struct CylindricalRadialSweepQuery<'a> {
    /// Stored rails in first-depth, far-depth order on their common unit parameter domain.
    ///
    /// Both rails must have equal degree and equal positive weights at every index.
    /// Omitted weights are exactly one, and raw weight representation remains bound.
    /// The caller verifies that the native surface interpolates these rails linearly in depth.
    pub rails: [MetricBezier<'a, 3>; 2],
    /// XY origin of the +Z cylinder axis in the actual rail coordinate frame.
    pub axis_origin: [f64; 2],
    /// Sign to apply to the derivative along the stored first-to-far depth direction.
    pub sense: CylindricalRadialSense,
    /// Finite nonnegative strict lower threshold in squared units per unit depth.
    pub minimum: f64,
    /// Original finite precision, degree and arithmetic operation limits.
    pub options: CylindricalStorageOptions,
}

/// An outward-rounded signed derivative bound tied to the exact supplied rail and axis bits.
#[derive(Clone, Debug)]
pub struct CertifiedCylindricalRadialSweep {
    source: Vec<u64>,
    sense: CylindricalRadialSense,
    minimum: f64,
    options: CylindricalStorageOptions,
    lower: f64,
    operations: u32,
}

impl CertifiedCylindricalRadialSweep {
    pub(in super::super) fn new(
        query: CylindricalRadialSweepQuery<'_>,
        lower: f64,
        operations: u32,
    ) -> Self {
        Self {
            source: types::source_bits(query.rails, query.axis_origin),
            sense: query.sense,
            minimum: query.minimum,
            options: query.options,
            lower,
            operations,
        }
    }

    /// Certified lower bound over all rail and depth parameters.
    /// Example: `assert!(proof.signed_squared_radius_derivative_lower() > 0.)`.
    pub fn signed_squared_radius_derivative_lower(&self) -> f64 {
        self.lower
    }

    /// Original strict minimum. Example: `proof.requested_minimum()`.
    pub fn requested_minimum(&self) -> f64 {
        self.minimum
    }

    /// Original work policy. Example: `proof.requested_options()`.
    pub fn requested_options(&self) -> CylindricalStorageOptions {
        self.options
    }

    /// Charged interval operations. Example: `proof.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }

    /// Compare both raw rails, their order, the actual axis and the signed direction.
    ///
    /// A different requested margin needs its own comparison with the achieved bound.
    /// Example: `assert!(proof.matches_source(query.rails, query.axis_origin, query.sense))`.
    pub fn matches_source(
        &self,
        rails: [MetricBezier<'_, 3>; 2],
        axis_origin: [f64; 2],
        sense: CylindricalRadialSense,
    ) -> bool {
        self.sense == sense
            && rails.into_iter().enumerate().all(|(depth, rail)| {
                admission::admit_curve(rail, depth, self.options.max_degree).is_ok()
            })
            && self.source == types::source_bits(rails, axis_origin)
    }
}

/// Original proof inputs and the reason no complete signed radial bound was issued.
#[derive(Clone, Debug, PartialEq)]
pub struct CylindricalRadialSweepError {
    /// Unchanged strict minimum.
    pub requested_minimum: f64,
    /// Unchanged work limits.
    pub requested_options: CylindricalStorageOptions,
    /// Unchanged signed direction.
    pub sense: CylindricalRadialSense,
    /// Input or incomplete proof cause.
    pub cause: CylindricalRadialSweepFailure,
}

/// An unsuccessful radial certificate, including conservative bound refusals.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum CylindricalRadialSweepFailure {
    /// Shared stored curve or arithmetic policy admission refused.
    Admission(CylindricalStorageFailure),
    /// The axis origin had at least one nonfinite coordinate.
    AxisOrigin([f64; 2]),
    /// The requested strict minimum was nonfinite or negative.
    Minimum(f64),
    /// The two raw pole counts differed.
    PoleCounts([usize; 2]),
    /// Effective positive weights must agree at each shared control index.
    CommonWeights {
        /// Zero-based index of a mismatched weight pair.
        index: usize,
        /// Actual first-depth and far-depth weights.
        actual: [f64; 2],
    },
    /// The bounded arithmetic stopped before producing a complete certificate.
    Arithmetic(BoundedArithmeticFailure),
    /// The conservative whole-domain lower bound was not strictly above the requested minimum.
    Uncertified {
        /// Outward-rounded lower bound, including nonpositive values.
        lower: f64,
    },
}

impl From<CylindricalStorageFailure> for CylindricalRadialSweepFailure {
    fn from(cause: CylindricalStorageFailure) -> Self {
        Self::Admission(cause)
    }
}

impl From<BoundedArithmeticFailure> for CylindricalRadialSweepFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(cause)
    }
}

impl std::fmt::Display for CylindricalRadialSweepError {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            out,
            "cylindrical radial sweep {:?} refused {:?}; expected matched rails in a finite +Z frame and a signed derivative above {}",
            self.sense, self.cause, self.requested_minimum,
        )
    }
}

impl std::error::Error for CylindricalRadialSweepError {}

const _: fn() = || {
    fn shared<T: Send + Sync>() {}
    shared::<CylindricalRadialSweepQuery<'static>>();
    shared::<CylindricalRadialSense>();
    shared::<CertifiedCylindricalRadialSweep>();
    shared::<CylindricalRadialSweepError>();
    shared::<CylindricalRadialSweepFailure>();
};
