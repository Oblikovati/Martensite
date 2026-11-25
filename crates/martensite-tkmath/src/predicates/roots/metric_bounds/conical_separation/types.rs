// SPDX-License-Identifier: GPL-2.0-only
//! Decoded input, checked bounds and refusal types for carrier separation.
use super::{BoundedArithmeticFailure, CylindricalProfileFailure, admission};

/// Decoded analytic cone with a local frame and native placement.
#[derive(Clone, Copy, Debug)]
pub struct DecodedConeFrame {
    /// Decoded cone semi-angle in radians.
    pub semi_angle: f64,
    /// Reference radius at the local origin.
    pub radius: f64,
    /// Local origin in XYZ order.
    pub origin: [f64; 3],
    /// Local X, Y, and Z directions in that order.
    pub axes: [[f64; 3]; 3],
    /// Native placement of the analytic cone.
    pub location: DecodedCoaxialLocation,
}

/// Decoded native affine placement admitted by the coaxial certificate.
#[derive(Clone, Copy, Debug)]
pub struct DecodedCoaxialLocation {
    /// Native uniform scale. Admission requires one.
    pub scale: f64,
    /// Native XYZ translation. Only axial translation is admitted.
    pub translation: [f64; 3],
    /// Decoded native linear coefficients in row order.
    pub matrix: [[f64; 3]; 3],
}

/// Decoded rim-plane carrier perpendicular to the common axis.
#[derive(Clone, Copy, Debug)]
pub struct DecodedRimPlane {
    /// World Z elevation.
    pub elevation: f64,
    /// Decoded world unit normal, parallel to the common axis.
    pub normal: [f64; 3],
}

/// Decoded positive-rational tensor Bézier side with two depth rows.
#[derive(Clone, Copy, Debug)]
pub struct DecodedRuledSide<'a> {
    /// Stable native side key.
    pub id: u64,
    /// Native placement of the stored side surface.
    pub location: DecodedCoaxialLocation,
    /// Euclidean poles and positive weights of the two stored Bézier depth rows.
    /// The caller binds these to the native surface and orders first depth first.
    pub rows: [&'a [[f64; 4]]; 2],
    /// Decoded parameter ranges in curve-then-depth order.
    pub domain: [[f64; 2]; 2],
}

/// Decoded conical cap carrier and enclosing trim parameters.
#[derive(Clone, Copy, Debug)]
pub struct DecodedCapTrim {
    /// Stable native cap key.
    pub id: u64,
    /// Decoded analytic carrier supporting the trim.
    pub frame: DecodedConeFrame,
    /// Encloses every point of the decoded cap trim, as proved by the caller.
    pub axial_parameters: [f64; 2],
}

/// Expected side of the stock cone at every admitted tool patch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeparationSense {
    /// Positive signed cone residual outward from the stock.
    Outside,
    /// Positive signed cone residual inward into the stock.
    Inside,
}

/// Complete decoded source and work limits for a carrier separation proof.
#[derive(Clone, Copy, Debug)]
pub struct CarrierSeparationQuery<'a> {
    /// Decoded world-+Z stock cone, with identity placement.
    pub stock: DecodedConeFrame,
    /// Lower and upper world-axial stock rims.
    pub rims: [DecodedRimPlane; 2],
    /// Stable native side keys in strictly increasing order.
    pub sides: &'a [DecodedRuledSide<'a>],
    /// Stable native cap keys in strictly increasing order.
    pub caps: &'a [DecodedCapTrim],
    /// Requested side of the stock cone.
    pub sense: SeparationSense,
    /// Work ceilings applied to this query.
    pub options: SeparationOptions,
}

/// Independent work ceilings for the interval filter and exact fallback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeparationOptions {
    /// Maximum charged interval operations across the complete family.
    pub max_scalar_steps: u32,
    /// Maximum charged exact grid operations across the complete family.
    pub max_dyadic_operations: u32,
}

/// Work observed by one attempted carrier separation proof.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SeparationWork {
    pub(super) scalar_steps: u32,
    pub(super) dyadic_operations: u32,
    pub(super) exact_sides: usize,
}

/// Stable identity of the source being checked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Patch {
    /// The stock carrier or shared family admission.
    Stock,
    /// A ruled side identified by its stable native key.
    Side(u64),
    /// A cap trim identified by its stable native key.
    Cap(u64),
}

/// Typed reason a complete carrier separation proof could not be produced.
#[derive(Clone, Debug, PartialEq)]
pub enum SeparationCause {
    /// Side or cap count falls outside the admitted family size.
    PatchCount {
        /// Decoded side count.
        sides: usize,
        /// Decoded cap count.
        caps: usize,
    },
    /// Native keys in one collection are not strictly increasing.
    SourceOrder {
        /// Previous native key.
        previous: u64,
        /// Offending next native key.
        next: u64,
    },
    /// Frame or placement falls outside the admitted coaxial shape.
    UnsupportedFrame,
    /// A side lacks the full unit parameter rectangle.
    UnsupportedDomain {
        /// Decoded side parameter limits.
        domain: [[f64; 2]; 2],
    },
    /// The stock angle is outside the admitted range or its trigonometric sign is unresolved.
    InvalidAngle {
        /// Decoded stock semi-angle.
        angle: f64,
    },
    /// Side rows do not have matching admitted control counts.
    PoleCount {
        /// Control counts of the two depth rows.
        rows: [usize; 2],
    },
    /// A decoded coordinate is nonfinite or too large.
    Coordinate {
        /// Offending decoded scalar.
        value: f64,
    },
    /// A weight falls outside the admitted finite positive range.
    Weight {
        /// Offending decoded weight.
        value: f64,
    },
    /// A strictly positive relevant radius or cone threshold could not be established.
    PositiveRadius {
        /// Observed conservative lower endpoint or invalid decoded radius.
        lower: f64,
    },
    /// Stock rim elevations are not ordered.
    RimInterval {
        /// Decoded rim elevations in supplied order.
        elevations: [f64; 2],
    },
    /// The cap parameter enclosure is not strictly ordered.
    AxialParameters {
        /// Decoded cap parameter enclosure.
        bounds: [f64; 2],
    },
    /// The next bounded interval step would exceed the work ceiling.
    ScalarWorkLimit {
        /// Count that the next step would require.
        required: u32,
        /// Requested work ceiling.
        limit: u32,
    },
    /// Strict separation in the requested sense remains unproved.
    NormalSign {
        /// Requested separation direction.
        sense: SeparationSense,
        /// Tensor depth layer, or zero for a cap.
        layer: usize,
        /// Observed signed lower endpoint.
        lower: f64,
    },
    /// Strict separation from both stock rims remains unproved.
    RimSeparation {
        /// Observed lower and upper rim gap endpoints.
        gaps: [f64; 2],
    },
    /// An intermediate enclosure is not representable as a finite ordered interval.
    UnrepresentableEnclosure,
    /// The exact bounded arithmetic operation refused.
    Arithmetic(BoundedArithmeticFailure),
    /// The stock trigonometric enclosure did not complete.
    Trigonometric(Box<CylindricalProfileFailure>),
}

impl From<BoundedArithmeticFailure> for SeparationCause {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(cause)
    }
}

/// Incomplete separation proof with the failing source and work already spent.
#[derive(Debug, PartialEq)]
pub struct SeparationError {
    /// Stock, side, or cap where the operation stopped.
    pub patch: Patch,
    /// Typed cause of the refusal.
    pub cause: SeparationCause,
    /// Operations charged before the refusal.
    pub work: SeparationWork,
}

/// Enclosed cone and rim gaps of one positive-rational side.
#[derive(Clone, Debug, PartialEq)]
pub struct SideSeparation {
    pub(super) id: u64,
    /// Bounds per tensor depth layer after dividing by a global positive denominator.
    /// The minimum of all entries is a whole-patch signed normal bound.
    pub(super) signed_normal_lower: [f64; 3],
    pub(super) zero_normal_upper: f64,
    pub(super) rim_lower: [f64; 2],
}

/// Enclosed cone and rim gaps of one decoded conical cap trim.
#[derive(Clone, Debug, PartialEq)]
pub struct CapSeparation {
    pub(super) id: u64,
    pub(super) signed_normal_lower: f64,
    pub(super) rim_lower: [f64; 2],
}

/// Sealed complete numerical evidence for the exact decoded query.
#[derive(Clone, Debug, PartialEq)]
pub struct CarrierSeparationReceipt {
    pub(super) source: Vec<u64>,
    pub(super) sides: Vec<SideSeparation>,
    pub(super) caps: Vec<CapSeparation>,
    pub(super) work: SeparationWork,
}

impl SideSeparation {
    /// Read the original side key. Example: `side.id()`.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Read per-layer signed lower bounds. Their minimum bounds the entire side.
    /// Example: `side.signed_normal_lower()[0]`.
    pub fn signed_normal_lower(&self) -> &[f64; 3] {
        &self.signed_normal_lower
    }

    /// Read the absolute normal-residual upper bound for the first depth row.
    /// Example: `side.zero_normal_upper()`.
    pub fn zero_normal_upper(&self) -> f64 {
        self.zero_normal_upper
    }

    /// Read the lower and upper stock-rim gap bounds, in that order.
    /// Example: `side.rim_lower()[0]`.
    pub fn rim_lower(&self) -> &[f64; 2] {
        &self.rim_lower
    }
}

impl CapSeparation {
    /// Read the original cap key. Example: `cap.id()`.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Read the signed lower bound across the enclosed trim. Example: `cap.signed_normal_lower()`.
    pub fn signed_normal_lower(&self) -> f64 {
        self.signed_normal_lower
    }

    /// Read the lower and upper stock-rim gap bounds, in that order.
    /// Example: `cap.rim_lower()[0]`.
    pub fn rim_lower(&self) -> &[f64; 2] {
        &self.rim_lower
    }
}

impl SeparationWork {
    /// Read the charged interval work. Example: `work.scalar_steps()`.
    pub fn scalar_steps(&self) -> u32 {
        self.scalar_steps
    }

    /// Read the charged exact arithmetic work. Example: `work.dyadic_operations()`.
    pub fn dyadic_operations(&self) -> u32 {
        self.dyadic_operations
    }

    /// Read how many side checks entered the exact fallback. Example: `work.exact_sides()`.
    pub fn exact_sides(&self) -> usize {
        self.exact_sides
    }
}

impl CarrierSeparationReceipt {
    /// Read the side bounds in source order. Example: `proof.sides()`.
    pub fn sides(&self) -> &[SideSeparation] {
        &self.sides
    }

    /// Read the cap bounds in source order. Example: `proof.caps()`.
    pub fn caps(&self) -> &[CapSeparation] {
        &self.caps
    }

    /// Read the completed work accounting. Example: `proof.work()`.
    pub fn work(&self) -> &SeparationWork {
        &self.work
    }

    /// Check that another admissible decoded query has identical recorded bits.
    /// Example: `proof.matches_source(decoded_query)`.
    pub fn matches_source(&self, query: CarrierSeparationQuery<'_>) -> bool {
        admission::admit_family(query).is_ok()
            && query
                .sides
                .iter()
                .all(|side| admission::admit_side(*side).is_ok())
            && query
                .caps
                .iter()
                .all(|cap| admission::admit_cap(*cap, query.stock).is_ok())
            && self.source == admission::source_bits(query)
    }
}
