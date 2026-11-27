//! Explicit interpolation versions preserve hardware replay and independent lighting policies.
/// Choose smooth Phong interpolation; `RenderPhongInterpolationBehavior::UnsnappedV2`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RenderPhongInterpolationBehavior {
    /// Driver interpolation using the device's raster precision.
    #[default]
    HardwareV1,
    /// Perspective interpolation from unsnapped window positions for finite positive-w triangles.
    /// Coverage, depth, vertex calculations and lighting arithmetic are unchanged.
    UnsnappedV2,
    /// V2 interpolation plus exact constant-projected-axis ordinary-FILL degenerates.
    /// Original geometry is retained; degenerate helper execution terminates before division.
    UnsnappedV3,
}
impl RenderPhongInterpolationBehavior {
    /// Record the immutable behavior version; `behavior.behavior_version()`.
    pub const fn behavior_version(self) -> u32 {
        match self {
            Self::HardwareV1 => 1,
            Self::UnsnappedV2 => 2,
            Self::UnsnappedV3 => 3,
        }
    }
}
/// Refuse unsupported interpolation before submitting the selected draw.
/// Example: `matches!(failure, RenderPhongInterpolationFailure::MissingBarycentric)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RenderPhongInterpolationFailure {
    /// The device lacks the required extension or feature bit.
    MissingBarycentric,
    /// A selected draw has no registered V2 shader/input interface.
    UnsupportedInterface {
        /// Actual rejected interface, retained for diagnostics.
        actual: String,
    },
    /// CPU domain screening rejected the selected triangle.
    InvalidTriangle {
        /// Interpolation behavior whose domain rejected the triangle.
        behavior: RenderPhongInterpolationBehavior,
        /// Direct triangle offset in the selected draw.
        triangle: usize,
        /// Screened clip values as IEEE Float32 bits, preserving nonfinite values.
        clip_bits: [[u32; 4]; 3],
    },
}
impl std::fmt::Display for RenderPhongInterpolationFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTriangle { behavior, .. } => write!(
                f,
                "Phong {behavior:?} received {self:?}; expected barycentric support and a registered finite positive-w nondegenerate triangle interface"
            ),
            _ => write!(
                f,
                "Phong interpolation received {self:?}; expected barycentric support and a registered finite positive-w nondegenerate triangle interface"
            ),
        }
    }
}
impl std::error::Error for RenderPhongInterpolationFailure {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hardware_replay_is_default_and_version_two_is_explicit() {
        let _: &(dyn Send + Sync) = &RenderPhongInterpolationBehavior::default();
        assert_eq!(
            RenderPhongInterpolationBehavior::default(),
            RenderPhongInterpolationBehavior::HardwareV1
        );
        assert_eq!(
            RenderPhongInterpolationBehavior::default().behavior_version(),
            1
        );
        assert_eq!(
            RenderPhongInterpolationBehavior::UnsnappedV2.behavior_version(),
            2
        );
    }
    #[test]
    fn v3_is_a_separate_explicit_behavior_version() {
        assert_eq!(
            RenderPhongInterpolationBehavior::UnsnappedV3.behavior_version(),
            3
        );
        assert_ne!(
            RenderPhongInterpolationBehavior::UnsnappedV3,
            RenderPhongInterpolationBehavior::default()
        );
        let _: &(dyn Send + Sync) = &RenderPhongInterpolationBehavior::UnsnappedV3;
    }
    #[test]
    fn typed_refusal_retains_its_domain() {
        let failure = RenderPhongInterpolationFailure::MissingBarycentric;
        let error: crate::RenderError = failure.clone().into();
        assert_eq!(error.phong_interpolation_failure(), Some(&failure));
        assert!(error.to_string().contains("positive-w"));
    }

    #[test]
    fn invalid_triangle_diagnostic_names_selected_behavior() {
        let failure = RenderPhongInterpolationFailure::InvalidTriangle {
            behavior: RenderPhongInterpolationBehavior::UnsnappedV3,
            triangle: 753,
            clip_bits: [[0; 4]; 3],
        };
        assert!(failure.to_string().contains("UnsnappedV3"));
        assert!(!failure.to_string().contains("UnsnappedV2"));
    }
}
