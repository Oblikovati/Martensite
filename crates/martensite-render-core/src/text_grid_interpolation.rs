//! Versioned text varyings keep the device-selected default available for replay.
/// Select how text texture coordinates are interpolated; `RenderTextInterpolationBehavior::ProjectedGrid12V2.behavior_version()`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RenderTextInterpolationBehavior {
    /// Use Vulkan's ordinary triangle varying interpolation.
    #[default]
    HardwareV1,
    /// Interpolate plain glyph-text UVs from projected vertices on a nearest-even 12-bit grid.
    ProjectedGrid12V2,
    /// Use the same grid evaluation with core flat triangle-corner transport.
    ProjectedGrid12FlatCornersV4,
}
/// A selected text interpolation behavior that cannot preserve its declared contract.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RenderTextInterpolationFailure {
    /// The device lacks `VK_KHR_fragment_shader_barycentric` or its feature bit.
    MissingBarycentric,
    /// Observed vertex attributes, stride, attribute offset, VS outputs and FS inputs.
    InsufficientLimits {
        /// The actual of this failure.
        actual: [u32; 5],
        /// The required of this failure.
        required: [u32; 5],
    },
    /// The selected draw has no registered glyph-text shader interface.
    UnsupportedInterface {
        /// The actual of this failure.
        actual: String,
    },
    /// The selected triangle lies outside the finite 12-bit shader domain.
    InvalidTriangle {
        /// The triangle of this failure.
        triangle: usize,
        /// The clip bits of this failure.
        clip_bits: [[u32; 4]; 3],
    },
}
impl std::fmt::Display for RenderTextInterpolationFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "text interpolation received {self:?}; expected the selected transport capability and registered finite positive-w triangles within ±4096 window pixels"
        )
    }
}
impl std::error::Error for RenderTextInterpolationFailure {}
impl RenderTextInterpolationBehavior {
    /// Record the selected text interpolation replay version; `behavior.behavior_version()`.
    pub const fn behavior_version(self) -> u32 {
        match self {
            Self::HardwareV1 => 1,
            Self::ProjectedGrid12V2 => 2,
            Self::ProjectedGrid12FlatCornersV4 => 4,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hardware_is_the_default_and_grid_twelve_is_explicit() {
        assert_eq!(
            RenderTextInterpolationBehavior::default(),
            RenderTextInterpolationBehavior::HardwareV1
        );
        assert_eq!(
            RenderTextInterpolationBehavior::HardwareV1.behavior_version(),
            1
        );
        assert_eq!(
            RenderTextInterpolationBehavior::ProjectedGrid12V2.behavior_version(),
            2
        );
        assert_ne!(
            RenderTextInterpolationBehavior::ProjectedGrid12V2,
            RenderTextInterpolationBehavior::default()
        );
    }
}
