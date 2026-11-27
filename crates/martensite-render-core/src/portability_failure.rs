//! Portability admission fails before device creation instead of changing vertex bytes.
use std::fmt;

/// A required subset query or fixed-layout constraint failed; match `error.portability_failure()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderPortabilityFailure {
    /// An advertised subset had no authoritative queried properties.
    MissingSubsetProperties,
    /// Existing binding strides cannot satisfy the reported alignment.
    IncompatibleVertexStrideAlignment {
        /// The device's queried minimum vertex binding stride alignment.
        observed: u32,
        /// Common divisor of the renderer's unchanged, audited binding strides.
        required_divisor: u32,
    },
    /// The frame needs a device feature the selected device does not offer.
    MissingDeviceFeature {
        /// The Vulkan feature member, for example `shaderImageFloat32AtomicAdd`.
        feature: &'static str,
    },
}

impl fmt::Display for RenderPortabilityFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSubsetProperties => formatter.write_str(
                "portability subset properties absent; expected queried minimum vertex stride alignment",
            ),
            Self::IncompatibleVertexStrideAlignment { observed, required_divisor } => write!(
                formatter,
                "minimum vertex stride alignment {observed}; expected a positive power-of-two divisor of {required_divisor}",
            ),
            Self::MissingDeviceFeature { feature } => {
                write!(formatter, "device feature {feature} absent; expected it offered and enabled")
            }
        }
    }
}
impl std::error::Error for RenderPortabilityFailure {}
