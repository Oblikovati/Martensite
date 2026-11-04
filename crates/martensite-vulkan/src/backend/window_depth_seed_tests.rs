use super::*;
use crate::{RenderViewportGridPrecision as Grid, RenderWindowDepthBehavior as Behavior};
fn parameters(
    behavior: Behavior,
) -> Option<super::super::window_depth_parameters::WindowDepthPipelineParameters> {
    let viewport = vk::Viewport {
        width: 256.,
        height: 256.,
        max_depth: 1.,
        ..Default::default()
    };
    super::super::window_depth_parameters::WindowDepthPipelineParameters::selected(
        behavior, viewport, false,
    )
    .unwrap()
}
fn kind(behavior: Behavior) -> PipelineVariant {
    let mut kind = PipelineVariant::from_index(0, false);
    kind.window_depth = parameters(behavior);
    kind
}
#[test]
fn replay_seed_and_actual_admission_stay_single_sample_for_both_earlier_versions() {
    let old = [
        Behavior::HardwareV1,
        Behavior::UnsnappedWindowDepthV2 {
            reference_grid: Grid::Fractional12,
            raster_grid: Grid::Fractional8,
        },
        Behavior::ReferenceWindowDepthV3 {
            reference_grid: Grid::Fractional12,
            raster_grid: Grid::Fractional8,
        },
    ];
    for behavior in old {
        assert_eq!(seed_samples(kind(behavior)), vk::SampleCountFlags::TYPE_1);
        assert!(admit_kind(kind(behavior), false, vk::SampleCountFlags::TYPE_1).is_ok());
    }
}
#[test]
fn only_v4_constructs_an_admitted_eight_sample_seed_without_weakening_actual_checks() {
    let behavior = Behavior::ClippedFanFramebufferV4 {
        fan_grid: Grid::Fractional8,
        raster_grid: Grid::Fractional8,
    };
    let mut kind = kind(behavior);
    kind.fixed_lighting = true;
    kind.lighting = true;
    kind.clipped = true;
    kind.world_clip_v2 = true;
    assert_eq!(seed_samples(kind), vk::SampleCountFlags::TYPE_8);
    assert!(admit_kind(kind, false, vk::SampleCountFlags::TYPE_8).is_ok());
    assert!(admit_kind(kind, false, vk::SampleCountFlags::TYPE_1).is_err());
    assert!(admit_kind(kind, false, vk::SampleCountFlags::TYPE_4).is_err());
}
