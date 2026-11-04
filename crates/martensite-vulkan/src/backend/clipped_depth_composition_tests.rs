use super::*;
use crate::{RenderClipping, RenderVertex, RenderViewportGridPrecision as Grid};
fn v4() -> Depth {
    Depth::ClippedFanFramebufferV4 {
        fan_grid: Grid::Fractional8,
        raster_grid: Grid::Fractional8,
    }
}
fn unlit_clipped() -> PrimitiveDraw {
    let mut draw = PrimitiveDraw::new(
        vec![RenderVertex::new([0.; 3], [1.; 4]); 3],
        vec![0, 1, 2],
        crate::identity_clip(),
    )
    .unwrap();
    draw.fixed_function_lighting = true;
    draw.clipping =
        Some(RenderClipping::new(vec![vec![[0., 0., 1., 0.]]], crate::identity_clip()).unwrap());
    draw
}
fn fixed_clipped() -> PrimitiveDraw {
    let mut draw = unlit_clipped();
    draw.lighting = Some(super::super::fixed_vertex_sources::fixed_test_lighting(
        3,
        crate::RenderShading::Phong,
    ));
    draw
}
#[test]
fn bootstrap_admits_the_required_stage_without_changing_the_requested_global_policy() {
    assert_eq!(
        device_clipping(v4(), Clip::FragmentV1),
        Clip::RasterClipDistanceV2
    );
    for requested in [Clip::RasterClipDistanceV2, Clip::RasterClipDistanceV3] {
        assert_eq!(device_clipping(v4(), requested), requested);
    }
}
#[test]
fn only_selected_fixed_family_owns_raster_clipping_under_fragment_replay() {
    assert_eq!(
        draw_clipping(v4(), Clip::FragmentV1, &fixed_clipped()),
        Clip::RasterClipDistanceV2
    );
    assert_eq!(
        draw_clipping(v4(), Clip::FragmentV1, &unlit_clipped()),
        Clip::FragmentV1
    );
    let mut capping = fixed_clipped();
    capping.capping_pass = crate::RenderCappingPass::Surface;
    assert_eq!(
        draw_clipping(v4(), Clip::FragmentV1, &capping),
        Clip::FragmentV1
    );
}
#[test]
fn explicit_raster_requests_and_every_earlier_depth_version_preserve_their_policy() {
    for requested in [Clip::RasterClipDistanceV2, Clip::RasterClipDistanceV3] {
        assert_eq!(draw_clipping(v4(), requested, &unlit_clipped()), requested);
    }
    for depth in earlier_versions() {
        assert_eq!(device_clipping(depth, Clip::FragmentV1), Clip::FragmentV1);
        assert_eq!(
            draw_clipping(depth, Clip::FragmentV1, &fixed_clipped()),
            Clip::FragmentV1
        );
    }
}

fn earlier_versions() -> [Depth; 3] {
    [
        Depth::HardwareV1,
        Depth::UnsnappedWindowDepthV2 {
            reference_grid: Grid::Fractional12,
            raster_grid: Grid::Fractional8,
        },
        Depth::ReferenceWindowDepthV3 {
            reference_grid: Grid::Fractional12,
            raster_grid: Grid::Fractional8,
        },
    ]
}

fn owned_pipeline_kind() -> super::super::pipeline_variants::PipelineVariant {
    let mut kind = super::super::pipeline_variants::PipelineVariant::from_index(0, false);
    kind.window_depth =
        super::super::window_depth_parameters::WindowDepthPipelineParameters::selected(
            v4(),
            ash::vk::Viewport {
                width: 256.,
                height: 256.,
                max_depth: 1.,
                ..Default::default()
            },
            false,
        )
        .unwrap();
    kind.world_clip_v2 = true;
    kind
}
#[test]
fn effective_metadata_comes_from_the_actual_selected_pipeline_kind() {
    let selected = owned_pipeline_kind();
    assert_eq!(
        pipeline_modes(Clip::FragmentV1, selected),
        [Clip::FragmentV1, Clip::RasterClipDistanceV2]
    );
    let mut other = selected;
    other.window_depth = None;
    other.world_clip_v2 = false;
    assert_eq!(
        pipeline_modes(Clip::FragmentV1, other),
        [Clip::FragmentV1; 2]
    );
    assert_eq!(
        pipeline_modes(Clip::RasterClipDistanceV3, selected),
        [Clip::RasterClipDistanceV3; 2]
    );
}

#[test]
fn incidence_version_owns_only_the_same_actual_fixed_clipped_family() {
    let depth = Depth::ClippedIncidenceFramebufferV6 {
        fan_grid: Grid::Fractional12,
        raster_grid: Grid::Fractional8,
    };
    assert_eq!(
        device_clipping(depth, Clip::FragmentV1),
        Clip::RasterClipDistanceV2
    );
    assert_eq!(
        draw_clipping(depth, Clip::FragmentV1, &fixed_clipped()),
        Clip::RasterClipDistanceV2
    );
    assert_eq!(
        draw_clipping(depth, Clip::FragmentV1, &unlit_clipped()),
        Clip::FragmentV1
    );
    assert_eq!(
        draw_clipping(depth, Clip::RasterClipDistanceV3, &fixed_clipped()),
        Clip::RasterClipDistanceV3
    );
}
#[test]
fn clip_space_version_needs_the_matrices_its_planes_are_derived_from() {
    let requested = Clip::RasterFixedFunctionClipSpaceV5;
    assert_eq!(device_clipping(Depth::HardwareV1, requested), requested);
    let mut draw = fixed_clipped();
    assert_eq!(
        draw_clipping(Depth::HardwareV1, requested, &draw),
        Clip::FragmentV1
    );
    draw.fixed_function_factors = crate::RenderProjectionFactors::new(
        crate::identity_clip(),
        crate::identity_clip(),
        crate::identity_clip(),
    )
    .ok();
    assert_eq!(
        draw_clipping(Depth::HardwareV1, requested, &draw),
        requested
    );
}
#[test]
fn fixed_function_version_serves_only_fixed_triangles_and_requires_the_clip_stage() {
    let requested = Clip::RasterFixedFunctionV4;
    assert_eq!(device_clipping(Depth::HardwareV1, requested), requested);
    assert_eq!(
        draw_clipping(Depth::HardwareV1, requested, &fixed_clipped()),
        requested
    );
    let mut programmable = fixed_clipped();
    programmable.fixed_function_lighting = false;
    assert_eq!(
        draw_clipping(Depth::HardwareV1, requested, &programmable),
        Clip::FragmentV1
    );
}
