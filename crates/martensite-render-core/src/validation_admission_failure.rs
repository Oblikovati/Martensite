//! Validation admission failures name the absent capability before instance creation.
use std::fmt;

/// Khronos validation layer version reported by the Vulkan loader.
///
/// Example: `ValidationLayerVersion::from_spec((1 << 22) | (4 << 12) | 313)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ValidationLayerVersion {
    /// Major version.
    pub major: u32,
    /// Minor version.
    pub minor: u32,
    /// Patch (SDK header) version.
    pub patch: u32,
}

/// The checksum-pinned validation layer version verified for parallel replay.
pub const VERIFIED_VALIDATION_LAYER: ValidationLayerVersion = ValidationLayerVersion {
    major: 1,
    minor: 4,
    patch: 313,
};

impl ValidationLayerVersion {
    /// Decode a packed Vulkan `specVersion`; `ValidationLayerVersion::from_spec(spec).patch`.
    pub fn from_spec(spec: u32) -> Self {
        Self {
            major: (spec >> 22) & 0x7f,
            minor: (spec >> 12) & 0x3ff,
            patch: spec & 0xfff,
        }
    }
}

impl fmt::Display for ValidationLayerVersion {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(output, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// A required validation capability is absent or too old; inspect `error.validation_admission_failure()`.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum RenderValidationAdmissionFailure {
    /// The loader could not find the selected validation layer.
    MissingLayer {
        /// The requested Vulkan layer name.
        layer: String,
    },
    /// Neither the global nor selected-layer catalog provides this extension.
    MissingExtension {
        /// The enabled layer whose catalog was checked with the global catalog.
        layer: String,
        /// The required Vulkan instance extension name.
        extension: String,
    },
    /// The loader selected a validation layer older than the verified one.
    UnverifiedLayer {
        /// The requested Vulkan layer name.
        layer: String,
        /// The version the loader would enable.
        active: ValidationLayerVersion,
        /// The oldest version this renderer accepts.
        required: ValidationLayerVersion,
    },
}
impl fmt::Display for RenderValidationAdmissionFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingLayer { layer } => write!(
                output,
                "received unavailable layer {layer}; expected it installed for validated Vulkan instance creation"
            ),
            Self::MissingExtension { layer, extension } => write!(
                output,
                "received unavailable extension {extension} in global and {layer} catalogs; expected it for validated Vulkan instance creation"
            ),
            Self::UnverifiedLayer {
                layer,
                active,
                required,
            } => write!(
                output,
                "received {layer} version {active}; expected {required} or newer for validated Vulkan instance creation"
            ),
        }
    }
}
impl std::error::Error for RenderValidationAdmissionFailure {}
