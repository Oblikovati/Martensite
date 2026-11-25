// SPDX-License-Identifier: GPL-2.0-only
use super::super::super::{CylindricalProfileOptions, CylindricalTrimQuery, MetricBezier};
use super::super::{CylindricalRadialSense, CylindricalStorageFailure, CylindricalStorageOptions};
use super::super::{admission, types};
use crate::predicates::roots::certificate_arithmetic::bounded_failure::BoundedArithmeticFailure;

/// Actual ordered rails, zero-depth trim and strict radial prefix budget.
#[derive(Clone, Copy)]
pub struct CylindricalContactPrefixQuery<'a> {
    /// Actual first-depth rail, pcurve and stock radius in the stock's local +Z frame.
    pub seam: CylindricalTrimQuery<'a>,
    /// Actual far rail, on the unit parameter interval of the first rail.
    pub far_rail: MetricBezier<'a, 3>,
    /// Signed monotonic radial direction with depth measured from the seam.
    pub sense: CylindricalRadialSense,
    /// Conservative radial contact distance for the active pair, finite and nonnegative.
    pub contact_allowance: f64,
    /// The output prefix must end strictly before this normalized depth in `(0, 1]`.
    pub max_normalized_depth: f64,
    /// Explicit finite precision, input degree and arithmetic operation limits.
    pub options: CylindricalStorageOptions,
}

impl<'a> CylindricalContactPrefixQuery<'a> {
    pub(super) fn rails(self) -> [MetricBezier<'a, 3>; 2] {
        [self.seam.curve, self.far_rail]
    }
}

/// Source-bound outward upper endpoint for possible radial contact.
#[derive(Clone, Debug)]
pub struct CertifiedCylindricalContactPrefix {
    source: Vec<u64>,
    sense: CylindricalRadialSense,
    seam_options: CylindricalProfileOptions,
    options: CylindricalStorageOptions,
    seam_error: f64,
    derivative: f64,
    upper: f64,
    operations: u32,
}

impl CertifiedCylindricalContactPrefix {
    pub(super) fn new(
        query: CylindricalContactPrefixQuery<'_>,
        seam_error: f64,
        derivative: f64,
        upper: f64,
        operations: u32,
    ) -> Self {
        Self {
            source: source_bits(query),
            sense: query.sense,
            seam_options: query.seam.options,
            options: query.options,
            seam_error,
            derivative,
            upper,
            operations,
        }
    }

    /// Depths strictly above this endpoint exclude radial stock contact.
    /// Example: `assert!(prefix.normalized_depth_upper() < query.max_normalized_depth)`.
    pub fn normalized_depth_upper(&self) -> f64 {
        self.upper
    }

    /// Achieved XY seam error used in the radial bound. Example: `prefix.seam_xy_error()`.
    pub fn seam_xy_error(&self) -> f64 {
        self.seam_error
    }

    /// Signed squared-radius derivative lower bound used.
    /// Example: `assert!(prefix.signed_squared_radius_derivative_lower() > 0.)`.
    pub fn signed_squared_radius_derivative_lower(&self) -> f64 {
        self.derivative
    }

    /// Charged dyadic operations for prefix arithmetic. Example: `prefix.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }

    /// Compare the exact ordered rail, trim, signed direction, allowance and request bits.
    /// Example: `assert!(prefix.matches_source(query))`.
    pub fn matches_source(&self, query: CylindricalContactPrefixQuery<'_>) -> bool {
        self.sense == query.sense
            && self.seam_options == query.seam.options
            && self.options == query.options
            && (2..=4).contains(&query.seam.pcurve.len())
            && query.rails().into_iter().enumerate().all(|(depth, rail)| {
                admission::admit_curve(rail, depth, self.options.max_degree).is_ok()
            })
            && self.source == source_bits(query)
    }
}

fn source_bits(query: CylindricalContactPrefixQuery<'_>) -> Vec<u64> {
    let mut words = types::source_bits(query.rails(), [0.; 2]);
    words.extend([
        query.seam.radius.to_bits(),
        query.seam.spatial_limit.to_bits(),
        query.contact_allowance.to_bits(),
        query.max_normalized_depth.to_bits(),
        query.seam.pcurve.len() as u64,
    ]);
    words.extend(query.seam.pcurve.iter().flatten().map(|v| v.to_bits()));
    words
}

/// The unchanged radial-prefix request and the reason it could not be certified.
#[derive(Clone, Debug, PartialEq)]
pub struct CylindricalContactPrefixError {
    /// Requested radial direction.
    pub sense: CylindricalRadialSense,
    /// Supplied cylinder radius.
    pub radius: f64,
    /// Requested radial contact distance.
    pub contact_allowance: f64,
    /// Requested strict upper depth.
    pub max_normalized_depth: f64,
    /// Requested arithmetic and degree limits.
    pub requested_options: CylindricalStorageOptions,
    /// Input mismatch, arithmetic failure or incomplete bound.
    pub cause: CylindricalContactPrefixFailure,
}

/// Refusal to bound the full radial contact prefix.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum CylindricalContactPrefixFailure {
    /// The stored input or policy failed admission.
    Admission(CylindricalStorageFailure),
    /// The sweep does not bind the supplied ordered rails, local origin and sense.
    SweepSource,
    /// The trim does not bind the supplied first rail, cylinder, pcurve and seam limits.
    TrimSource,
    /// The stock cylinder radius must be finite and strictly positive.
    Radius(f64),
    /// A finite nonnegative contact allowance is required.
    ContactAllowance(f64),
    /// The strict maximum depth must be in `(0,1]`.
    MaximumDepth(f64),
    /// Either lower radial endpoint could not be shown strictly positive.
    RadiusAllowance {
        /// Achieved XY seam error used in the lower endpoint.
        seam_error: f64,
    },
    /// Finite directed arithmetic stopped without a certificate.
    Arithmetic(BoundedArithmeticFailure),
    /// The outward-rounded prefix did not finish strictly before the requested maximum.
    Uncertified {
        /// Conservative upper bound on the normalized depth of possible contact.
        upper: f64,
    },
}

impl From<CylindricalStorageFailure> for CylindricalContactPrefixFailure {
    fn from(cause: CylindricalStorageFailure) -> Self {
        Self::Admission(cause)
    }
}

impl From<BoundedArithmeticFailure> for CylindricalContactPrefixFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(cause)
    }
}

impl std::fmt::Display for CylindricalContactPrefixError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "cylindrical contact prefix {:?}, radius={}, contact={}, max depth={}: {:?}; expected matching local evidence, positive inner radii and a bound strictly below max depth",
            self.sense, self.radius, self.contact_allowance, self.max_normalized_depth, self.cause,
        )
    }
}

impl std::error::Error for CylindricalContactPrefixError {}

const _: fn() = || {
    fn shared<T: Send + Sync>() {}
    shared::<CylindricalContactPrefixQuery<'static>>();
    shared::<CertifiedCylindricalContactPrefix>();
    shared::<CylindricalContactPrefixError>();
    shared::<CylindricalContactPrefixFailure>();
};
