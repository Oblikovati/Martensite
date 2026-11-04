//! Typed refusals keep raster clipping's input and device limits inspectable.
use crate::RenderWorldClippingBehavior;
use std::fmt;

/// Unsupported selected-world-clipping requirement; `failure.reason`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RenderWorldClippingRequirement {
    /// A requested device lacks shader clip distances or enough slots.
    ClipDistanceCapacity,
    /// The draw is outside the built-in opaque triangle surface contract.
    UnsupportedDraw {
        /// Actual topology, shading, or overlay that prevented selection.
        actual: String,
    },
    /// A chain contains OR alternatives or there are more than eight planes.
    UnsupportedPlaneChains {
        /// Plane count and chain lengths from the submitted clipping state.
        actual: String,
    },
}

/// Versioned raster-world-clipping refusal; `error.world_clipping_failure()`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenderWorldClippingFailure {
    /// Requested behavior version.
    pub behavior: RenderWorldClippingBehavior,
    /// Failed capability or draw requirement.
    pub reason: RenderWorldClippingRequirement,
    /// Selected device's shaderClipDistance feature.
    pub shader_clip_distance: bool,
    /// Selected device's maxClipDistances.
    pub max_clip_distances: u32,
    /// Selected device's maxCombinedClipAndCullDistances.
    pub max_combined_clip_and_cull_distances: u32,
}
impl fmt::Display for RenderWorldClippingFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            output,
            "world clipping {:?} refused: {:?}; shaderClipDistance={}, maxClipDistances={}, maxCombinedClipAndCullDistances={}; expected supported opaque built-in Phong triangles or solid native-width lines with positive normal finite homogeneous W and at most eight single-plane AND chains",
            self.behavior,
            self.reason,
            self.shader_clip_distance,
            self.max_clip_distances,
            self.max_combined_clip_and_cull_distances
        )
    }
}
impl std::error::Error for RenderWorldClippingFailure {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn typed_world_clip_failures_are_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<RenderWorldClippingRequirement>();
        assert_send_sync::<RenderWorldClippingFailure>();
    }
}
