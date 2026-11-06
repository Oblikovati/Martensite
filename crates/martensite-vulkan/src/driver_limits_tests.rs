use super::*;
use Graphic3dTypeOfLimit as Limit;
/// Named fake device: the lavapipe capacities recorded by `vulkaninfo` on 2026-10-27.
fn lavapipe_like_device() -> RenderDeviceLimits {
    RenderDeviceLimits {
        max_texture_size: 16384,
        max_dump_size: [16384, 16384],
        max_msaa_samples: 4,
        max_combined_texture_units: 1_000_000,
        max_clip_planes: 8,
        max_color_attachments: 8,
        max_draw_buffers: 8,
        max_degree_of_anisotropy: 16,
        has_srgb: true,
        has_texture_multisampling: true,
        has_mesh_edges: true,
        has_blended_oit: true,
        has_blended_oit_msaa: true,
        sub_texel_precision_bits: 8,
        has_ray_tracing_textures: true,
        has_ray_tracing_adaptive_sampling: true,
        has_ray_tracing_adaptive_sampling_atomic: true,
    }
}
#[test]
fn device_limits_answer_every_device_backed_query() {
    let device = Some(lavapipe_like_device());
    for (kind, value) in [
        (Limit::MaxNbLights, 8),
        (Limit::MaxNbClipPlanes, 8),
        (Limit::MaxNbViews, 10_000),
        (Limit::MaxTextureSize, 16384),
        (Limit::MaxViewDumpSizeX, 4096),
        (Limit::MaxViewDumpSizeY, 4096),
        (Limit::MaxCombinedTextureUnits, 1_000_000),
        (Limit::MaxMsaa, 4),
        (Limit::HasPBR, 1),
        (Limit::HasRayTracing, 1),
        (Limit::HasRayTracingTextures, 1),
        (Limit::HasRayTracingAdaptiveSampling, 1),
        (Limit::HasRayTracingAdaptiveSamplingAtomic, 1),
        (Limit::HasSRGB, 1),
        (Limit::HasBlendedOit, 1),
        (Limit::HasBlendedOitMsaa, 1),
        (Limit::HasFlatShading, 1),
        (Limit::IsWorkaroundFBO, 0),
        (Limit::HasMeshEdges, 1),
    ] {
        assert_eq!(inquire(kind, 4096, device), value, "{kind:?}");
    }
}
#[test]
fn absent_device_answers_like_the_native_null_context() {
    for (kind, value) in [
        (Limit::MaxNbLights, 8),
        (Limit::MaxNbClipPlanes, 0),
        (Limit::MaxNbViews, 10_000),
        (Limit::MaxTextureSize, 0),
        (Limit::MaxViewDumpSizeX, 4096),
        (Limit::MaxCombinedTextureUnits, 1),
        (Limit::MaxMsaa, 0),
        (Limit::HasSRGB, 0),
        (Limit::HasFlatShading, 0),
        (Limit::IsWorkaroundFBO, 0),
        (Limit::HasMeshEdges, 0),
        (Limit::HasRayTracing, 0),
    ] {
        assert_eq!(inquire(kind, 4096, None), value, "{kind:?}");
    }
}
#[test]
fn missing_device_features_clear_their_flags() {
    let device = RenderDeviceLimits {
        has_srgb: false,
        has_mesh_edges: false,
        has_ray_tracing_adaptive_sampling_atomic: false,
        max_dump_size: [4096, 4096],
        ..lavapipe_like_device()
    };
    assert_eq!(inquire(Limit::HasSRGB, 4096, Some(device)), 0);
    assert_eq!(inquire(Limit::HasMeshEdges, 4096, Some(device)), 0);
    let atomic = Limit::HasRayTracingAdaptiveSamplingAtomic;
    assert_eq!(inquire(atomic, 4096, Some(device)), 0);
    assert_eq!(inquire(Limit::IsWorkaroundFBO, 4096, Some(device)), 1);
}
#[test]
fn sub_texel_precision_reports_the_device_bits() {
    assert_eq!(lavapipe_like_device().sub_texel_precision_bits(), 8);
}
