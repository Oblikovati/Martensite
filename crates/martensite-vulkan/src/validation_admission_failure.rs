//! Validation admission failures name the absent capability before instance creation.
use std::fmt;

/// A required validation capability is absent; inspect `error.validation_admission_failure()`.
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
        }
    }
}
impl std::error::Error for RenderValidationAdmissionFailure {}
