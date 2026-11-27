//! Select a versioned world-plane clipping implementation.
/// Versioned world-plane clipping; `RenderWorldClippingBehavior::RasterClipDistanceV2`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RenderWorldClippingBehavior {
    /// Preserve the existing per-fragment world-plane discard.
    #[default]
    FragmentV1,
    /// Clip opaque built-in Phong triangles before sample coverage with simple AND planes.
    /// Inputs require finite positive normal homogeneous W;
    /// `config.with_world_clipping(RenderWorldClippingBehavior::RasterClipDistanceV2)`.
    RasterClipDistanceV2,
    /// Raster-clip built-in Phong triangles and solid built-in lines before coverage.
    /// `config.with_world_clipping(RenderWorldClippingBehavior::RasterClipDistanceV3)`.
    RasterClipDistanceV3,
    /// Clip triangles of fixed-function frames before coverage, as native `glClipPlane` does,
    /// and keep the programmable discard elsewhere: a fixed-function frame clips per
    /// sample in the driver, whereas programs discard per pixel (demo/samples/dimensions).
    /// `config.with_world_clipping(RenderWorldClippingBehavior::RasterFixedFunctionV4)`.
    RasterFixedFunctionV4,
    /// As version four with the distances native evaluates: `glClipPlane` reaches llvmpipe as a
    /// clip-space plane dotted with `gl_Position`, and the rounded ratio of two such distances
    /// places every clipped vertex (the depth ties of demo/samples/dimensions).
    /// `config.with_world_clipping(RenderWorldClippingBehavior::RasterFixedFunctionClipSpaceV5)`.
    RasterFixedFunctionClipSpaceV5,
}
impl RenderWorldClippingBehavior {
    /// Stable replay identifier; `behavior.behavior_version()` returns 1 by default.
    pub const fn behavior_version(self) -> u32 {
        match self {
            Self::FragmentV1 => 1,
            Self::RasterClipDistanceV2 => 2,
            Self::RasterClipDistanceV3 => 3,
            Self::RasterFixedFunctionV4 => 4,
            Self::RasterFixedFunctionClipSpaceV5 => 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fragment_discard_remains_default_and_raster_clipping_has_new_version() {
        assert_eq!(
            RenderWorldClippingBehavior::default(),
            RenderWorldClippingBehavior::FragmentV1
        );
        assert_eq!(
            RenderWorldClippingBehavior::FragmentV1.behavior_version(),
            1
        );
        assert_eq!(
            RenderWorldClippingBehavior::RasterClipDistanceV2.behavior_version(),
            2
        );
        assert_eq!(
            RenderWorldClippingBehavior::RasterClipDistanceV3.behavior_version(),
            3
        );
        assert_eq!(
            RenderWorldClippingBehavior::RasterFixedFunctionV4.behavior_version(),
            4
        );
        assert_eq!(
            RenderWorldClippingBehavior::RasterFixedFunctionClipSpaceV5.behavior_version(),
            5
        );
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<RenderWorldClippingBehavior>();
    }
}
