//! Select a versioned interpolation contract for fragment world-plane clipping.
/// Fragment clipping interpolation; `RenderWorldClipInterpolationBehavior::ProjectedGrid12V1`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RenderWorldClipInterpolationBehavior {
    /// Preserve ordinary hardware smooth interpolation for clipping positions.
    #[default]
    HardwareV1,
    /// Use the projected 12-bit grid for supported Phong triangles; preserve FragmentV1 for other topologies.
    ProjectedGrid12V1,
    /// Compose projected-grid clipping with exact-axis Phong and empty-sample coverage; select V2 only with both matching policies.
    ProjectedGrid12V2,
}

impl RenderWorldClipInterpolationBehavior {
    /// Return the stable replay identifier; `behavior.behavior_version()` returns `1` by default.
    pub const fn behavior_version(self) -> u32 {
        match self {
            Self::HardwareV1 => 1,
            Self::ProjectedGrid12V1 => 2,
            Self::ProjectedGrid12V2 => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hardware_interpolation_remains_the_default_replay() {
        assert_eq!(
            RenderWorldClipInterpolationBehavior::default(),
            RenderWorldClipInterpolationBehavior::HardwareV1
        );
        assert_eq!(
            RenderWorldClipInterpolationBehavior::HardwareV1.behavior_version(),
            1
        );
        assert_eq!(
            RenderWorldClipInterpolationBehavior::ProjectedGrid12V1.behavior_version(),
            2
        );
        assert_eq!(
            RenderWorldClipInterpolationBehavior::ProjectedGrid12V2.behavior_version(),
            3
        );
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<RenderWorldClipInterpolationBehavior>();
    }
}
