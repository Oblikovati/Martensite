//! Frame ray-tracing pass bridged directly to the unified OpenPBR v1.1 compute path tracer
//! (`hw_raytrace`): hardware `VK_KHR_ray_query` TLAS traversal on RT GPUs with automatic
//! software-BVH compute fallback, 3-pass SVGF spatiotemporal denoiser, and AgX/ACES/Filmic
//! HDR display tone mapping.
use super::frame::FrameResources;
use super::hw_raytrace::{
    ComputeRaytraceCaches, ComputeRaytraceDraw, ComputeRaytraceInputs, prepare_compute_raytrace,
    record_compute_raytrace,
};
use super::pbr_environment::PbrEnvironment;
use super::runtime::VulkanRuntime;
use crate::RenderError;
use crate::raytrace::pt_scene::PathtraceCameraUbo;
use ash::vk;
use std::sync::Arc;

/// Persistent compute ray-tracing caches owned by `VulkanRenderer`.
pub(super) type RaytraceCaches = ComputeRaytraceCaches;

/// Prepared ray-tracing pass attached to `FrameResources`.
pub(super) type RaytraceFrameDraw = ComputeRaytraceDraw;

/// Auxiliary environment handle passed into `prepare_raytrace`.
pub(super) struct RaytraceImageSources {
    pub pbr_env: Arc<PbrEnvironment>,
}

/// Prepares the unified OpenPBR compute path-tracing pass when `frame.raytrace` is active.
pub(super) fn prepare_raytrace(
    runtime: &Arc<VulkanRuntime>,
    caches: &RaytraceCaches,
    frame: &crate::RenderFrame,
    (resources, sources): (&FrameResources, RaytraceImageSources),
) -> Result<Option<RaytraceFrameDraw>, RenderError> {
    let Some(raytrace) = frame.raytrace.as_deref() else {
        return Ok(None);
    };
    let ubo = build_camera_ubo(raytrace, resources.attachment_extent);
    let accum_frames = raytrace
        .path_tracing
        .as_ref()
        .map_or(0, |p| p.accum_frames.max(0) as u32);
    let draw = prepare_compute_raytrace(
        runtime,
        caches,
        ComputeRaytraceInputs {
            span: raytrace.span,
            eye_targets: raytrace.eye_targets,
            extent: resources.attachment_extent,
            requested_backend: raytrace.backend,
            ubo,
            scene: &raytrace.pt_scene,
            pbr_env: sources.pbr_env,
            denoiser_enabled: raytrace.denoiser_enabled,
            denoiser_atrous_iterations: raytrace.denoiser_atrous_iterations,
            denoiser_temporal_alpha: raytrace.denoiser_temporal_alpha,
            reset_history: raytrace.reset_history,
            tone_mapping: raytrace.tone_mapping,
            exposure: raytrace.exposure,
            white_point: raytrace.white_point,
            accum_frames,
        },
    )?;
    Ok(Some(draw))
}

/// Records the unified OpenPBR compute path-tracing + SVGF + tone-mapping pass.
pub(super) fn record_raytrace(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
    _clear: [f32; 4],
    draw: &RaytraceFrameDraw,
) {
    record_compute_raytrace(runtime, command, resources, draw);
}

fn build_camera_ubo(
    raytrace: &crate::raytrace::RaytraceFrame,
    extent: vk::Extent2D,
) -> PathtraceCameraUbo {
    let use_env_bg =
        raytrace.uniforms.environment.for_background || raytrace.constants.cubemap_background;
    let is_ortho = raytrace.constants.orthographic;
    let flags = u32::from(use_env_bg) | (u32::from(is_ortho) << 1);
    let (eye_origin, eye_view, eye_right, eye_up) = if is_ortho {
        let [lb, rb, lt, rt] = raytrace.uniforms.camera.origins;
        let d = raytrace.uniforms.camera.directions[0];
        let d_len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt().max(1e-6);
        let center: [f32; 3] = std::array::from_fn(|i| 0.25 * (lb[i] + rb[i] + lt[i] + rt[i]));
        let right: [f32; 3] = std::array::from_fn(|i| 0.25 * ((rb[i] - lb[i]) + (rt[i] - lt[i])));
        let up: [f32; 3] = std::array::from_fn(|i| 0.25 * ((lt[i] - lb[i]) + (rt[i] - rb[i])));
        (
            [center[0], center[1], center[2], 0.0],
            [d[0] / d_len, d[1] / d_len, d[2] / d_len, 1.0],
            [right[0], right[1], right[2], 1.0],
            [up[0], up[1], up[2], 1.0],
        )
    } else {
        let eye = &raytrace.uniforms.eye;
        (
            [
                eye.origin[0],
                eye.origin[1],
                eye.origin[2],
                if raytrace.constants.depth_of_field {
                    raytrace.uniforms.aperture_radius
                } else {
                    0.0
                },
            ],
            [
                eye.view[0],
                eye.view[1],
                eye.view[2],
                raytrace.uniforms.focal_plane_distance.max(1e-3),
            ],
            [
                eye.side[0] * eye.size[0],
                eye.side[1] * eye.size[0],
                eye.side[2] * eye.size[0],
                eye.size[0],
            ],
            [
                eye.vertical[0] * eye.size[1],
                eye.vertical[1] * eye.size[1],
                eye.vertical[2] * eye.size[1],
                eye.size[1],
            ],
        )
    };
    let accum = raytrace.uniforms.path_tracing.accum_samples.max(0) as u32;
    let seed = raytrace.uniforms.path_tracing.frame_seed as u32;
    let max_bounces = raytrace.constants.bounces.max(1) as u32;
    let clamp_val = if raytrace.uniforms.path_tracing.max_radiance > 0.0 {
        raytrace.uniforms.path_tracing.max_radiance
    } else {
        30.0
    };
    PathtraceCameraUbo {
        eye_origin,
        eye_view,
        eye_right,
        eye_up,
        view_proj: raytrace.uniforms.camera.view_projection.data(),
        prev_view_proj: raytrace.prev_view_proj,
        back_top: raytrace.uniforms.back_top,
        back_bottom: raytrace.uniforms.back_bottom,
        ambient: [
            raytrace.uniforms.ambient[0],
            raytrace.uniforms.ambient[1],
            raytrace.uniforms.ambient[2],
            raytrace.uniforms.scene_epsilon.max(1e-5),
        ],
        counts: [
            extent.width,
            extent.height,
            raytrace.pt_scene.lights.len() as u32,
            raytrace.pt_scene.triangles.len() as u32,
        ],
        settings: [max_bounces, seed, accum, flags],
        misc: [clamp_val, 1.0, 0.01, 1000.0],
    }
}
