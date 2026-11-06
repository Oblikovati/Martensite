//! Pure production rectangle math, with no Vulkan instance or device creation.
use martensite_vulkan::RenderError;
#[path = "../src/fixed_grid_point.rs"]
mod fixed_grid_point;

#[test]
fn hardware_point_replay_remains_default() {
    use martensite_vulkan::RenderPointRasterBehavior;
    assert_eq!(
        RenderPointRasterBehavior::default(),
        RenderPointRasterBehavior::HardwareQuadV1
    );
}

#[test]
fn explicit_fixed_grid_replay_keeps_hardware_version_selectable() {
    use martensite_vulkan::RenderPointRasterBehavior::{FixedGridAliasedV2, HardwareQuadV1};
    assert_eq!(HardwareQuadV1.behavior_version(), 1);
    assert_eq!(FixedGridAliasedV2.behavior_version(), 2);
    assert_ne!(FixedGridAliasedV2, HardwareQuadV1);
}
