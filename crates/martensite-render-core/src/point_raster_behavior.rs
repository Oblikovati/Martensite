//! Explicit point coverage versions keep hardware replay independent of a fixed-grid contract.
/// Choose plain single-sample point coverage; `RenderPointRasterBehavior::FixedGridAliasedV2`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RenderPointRasterBehavior {
    /// Existing hardware point primitives, including the device's own coverage rules.
    #[default]
    HardwareQuadV1,
    /// An eight-bit center grid, half-down whole width, and bottom-zero aliased squares.
    FixedGridAliasedV2,
    /// A twelve-bit center grid and bottom-one adjustment for plain one-pixel points.
    /// Example: `RenderPointRasterBehavior::FixedGridUnit12V3.behavior_version()` returns three.
    FixedGridUnit12V3,
    /// V3 plain unit points plus twelve-bit unlit sprites with explicit interpolated UVs.
    /// Example: `RenderPointRasterBehavior::FixedGridSprites12V4.behavior_version()` returns four.
    FixedGridSprites12V4,
}

impl RenderPointRasterBehavior {
    /// Record the exact selected replay version; `behavior.behavior_version()`.
    pub const fn behavior_version(self) -> u32 {
        match self {
            Self::HardwareQuadV1 => 1,
            Self::FixedGridAliasedV2 => 2,
            Self::FixedGridUnit12V3 => 3,
            Self::FixedGridSprites12V4 => 4,
        }
    }
}

const _: fn() = || {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<RenderPointRasterBehavior>();
};
