//! Window-depth reconstruction is independent of projection, coverage and bias units.
use crate::RenderViewportGridPrecision;

/// An explicit depth-plane operation; hardware replay remains the default.
/// Example: `RenderWindowDepthBehavior::HardwareV1.behavior_version()` returns `1`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RenderWindowDepthBehavior {
    /// Preserve fixed-function interpolation and slope bias without reconstruction.
    #[default]
    HardwareV1,
    /// Preserve XY/W and varyings while reconstructing the declared reference window plane.
    /// Device-minimum precision is never substituted for either explicitly selected grid.
    /// A declared-grid-empty triangle replays its finite original clip coordinates; an
    /// actual no-fragment guarantee requires independent evidence for the device grid.
    UnsnappedWindowDepthV2 {
        /// Grid on which the operation defines its reference plane.
        reference_grid: RenderViewportGridPrecision,
        /// Physical raster grid independently bound by the caller's device evidence.
        raster_grid: RenderViewportGridPrecision,
    },
    /// Evaluate the declared reference plane directly per fragment, retaining original clip positions.
    /// Bias operands are the actual ordered draw state; Native24UnitsV2 is required.
    /// Every submitted primitive must complete operation-local GPU validation before
    /// offscreen publication or GUI state commits/presentation; missing evidence is incomplete.
    ReferenceWindowDepthV3 {
        /// Independently declared reference grid.
        reference_grid: RenderViewportGridPrecision,
        /// Independently evidenced raster grid.
        raster_grid: RenderViewportGridPrecision,
    },
    /// Evaluate clipped fixed-Smooth3 fans at actual samples, preserving pixel-center colors.
    /// The first admitted domain is rounded rigid cameras, orthographic projection, LAST,
    /// bottom-up D24S8/8x and Native24UnitsV2; unsupported compositions refuse explicitly.
    /// Under FragmentV1 it owns pre-raster clipping for selected fixed draws only; other
    /// families retain the requested clipping policy, and explicit raster policies stay strict.
    ClippedFanFramebufferV4 {
        /// Explicit reference grid for fan membership, not a plane-coordinate snap.
        fan_grid: RenderViewportGridPrecision,
        /// Independently evidenced physical raster grid.
        raster_grid: RenderViewportGridPrecision,
    },
    /// Evaluate raw OpenGL clip inputs in binary64 before reference-grid quantization.
    /// Input is the actual binary32 VS clip BEFORE any GL-to-Vulkan depth remap.
    /// Ordered binary64 operations and nearest-even grid rounding define this owned
    /// contract; it is neither exact-real arithmetic nor native hardware emulation.
    Float64ReferenceWindowDepthV5 {
        /// The reference grid of this failure.
        reference_grid: RenderViewportGridPrecision,
        /// The raster grid of this failure.
        raster_grid: RenderViewportGridPrecision,
    },
    /// Clip using actual VS incidences and Q12 fan membership, retaining V4 as replay.
    /// The first domain keeps V4's fixed-Smooth3/MSAA/LAST constraints and never
    /// substitutes a reconstructed inverse-plane incidence for an actual VS value.
    ClippedIncidenceFramebufferV6 {
        /// Explicit Q12 reference fan grid; other values refuse this version.
        fan_grid: RenderViewportGridPrecision,
        /// Independently evidenced physical raster grid, currently Q8.
        raster_grid: RenderViewportGridPrecision,
    },
}

impl RenderWindowDepthBehavior {
    /// Return the stable replay version; `RenderWindowDepthBehavior::default().behavior_version()` is `1`.
    pub const fn behavior_version(self) -> u32 {
        match self {
            Self::HardwareV1 => 1,
            Self::UnsnappedWindowDepthV2 { .. } => 2,
            Self::ReferenceWindowDepthV3 { .. } => 3,
            Self::ClippedFanFramebufferV4 { .. } => 4,
            Self::Float64ReferenceWindowDepthV5 { .. } => 5,
            Self::ClippedIncidenceFramebufferV6 { .. } => 6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_replay_and_explicit_grid_pair_have_distinct_versions() {
        let selected = RenderWindowDepthBehavior::UnsnappedWindowDepthV2 {
            reference_grid: RenderViewportGridPrecision::Fractional12,
            raster_grid: RenderViewportGridPrecision::Fractional8,
        };
        assert_eq!(
            RenderWindowDepthBehavior::default(),
            RenderWindowDepthBehavior::HardwareV1
        );
        assert_eq!(RenderWindowDepthBehavior::default().behavior_version(), 1);
        assert_eq!(selected.behavior_version(), 2);
    }

    #[test]
    fn immutable_depth_selection_is_send_and_sync() {
        fn require_traits<T: Send + Sync>() {}
        require_traits::<RenderWindowDepthBehavior>();
    }
    #[test]
    fn direct_fragment_depth_keeps_geometry_replay_as_version_two() {
        let direct = RenderWindowDepthBehavior::ReferenceWindowDepthV3 {
            reference_grid: RenderViewportGridPrecision::Fractional12,
            raster_grid: RenderViewportGridPrecision::Fractional8,
        };
        assert_eq!(direct.behavior_version(), 3);
    }
}
