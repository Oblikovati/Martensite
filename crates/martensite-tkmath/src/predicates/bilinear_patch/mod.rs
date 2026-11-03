//! Global injectivity of a continuous bilinear patch through an exact projection.
mod input;
mod projection;
use super::OrientationSign;
pub use input::{BilinearInputError, BilinearPatchInput};
pub use projection::CoordinateProjection;

/// The supported input did not establish injectivity; this does not assert a fold.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BilinearInconclusive {
    /// Positive weights differ, so the map need not be bilinear.
    NonConstantWeights,
    /// A span or its reciprocal cannot be represented as finite binary64.
    UnrepresentableSpan {
        /// Zero-based parameter direction, U then V.
        direction: usize,
    },
    /// No coordinate projection has four consistently strict corner signs.
    NoStrictProjection,
}
/// An owned conclusion for the supplied numeric patch only.
#[derive(Clone, Debug, PartialEq)]
pub enum BilinearExclusion {
    /// Exact corner signs establish global injectivity of the represented map.
    Certified(BilinearCertificate),
    /// The supplied input remains owned, but injectivity was not established.
    Inconclusive {
        /// The owned numeric representation that was checked.
        input: BilinearPatchInput,
        /// The admission or exact-projection condition that was not established.
        reason: BilinearInconclusive,
    },
}
/// A certificate owns its numeric input; it has no mutable geometry aliases.
#[derive(Clone, Debug, PartialEq)]
pub struct BilinearCertificate {
    input: BilinearPatchInput,
    projection: CoordinateProjection,
    orientation: OrientationSign,
}
impl BilinearCertificate {
    /// Read the certified representation; `certificate.input().corners()`.
    pub fn input(&self) -> &BilinearPatchInput {
        &self.input
    }
    /// Read the injective coordinate projection; `certificate.projection()`.
    pub fn projection(&self) -> CoordinateProjection {
        self.projection
    }
    /// Read the common exact corner sign; `certificate.orientation()`.
    pub fn orientation(&self) -> OrientationSign {
        self.orientation
    }
}
/// Certify the continuous map, without sampling; `certify_bilinear_patch(input)`.
pub fn certify_bilinear_patch(input: BilinearPatchInput) -> BilinearExclusion {
    if let Some(reason) = input.restriction() {
        return BilinearExclusion::Inconclusive { input, reason };
    }
    for projection in CoordinateProjection::ALL {
        if let Some(orientation) = projection.strict_orientation(input.corners()) {
            return BilinearExclusion::Certified(BilinearCertificate {
                input,
                projection,
                orientation,
            });
        }
    }
    BilinearExclusion::Inconclusive {
        input,
        reason: BilinearInconclusive::NoStrictProjection,
    }
}
#[cfg(test)]
mod identity_tests;
#[cfg(test)]
mod tests;
