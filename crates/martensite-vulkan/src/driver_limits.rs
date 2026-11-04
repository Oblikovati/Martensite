//! Route the ordinary driver query without confusing a capture bound with texture capacity.
//! Values answer `Graphic3dGraphicDriver::inquire_limit`; without a Vulkan device
//! (an injected backend) the native null-context answers apply, except texture size, which
//! stays 0 (unknown) instead of the invented native 1024.
use crate::RenderDeviceLimits;
use martensite_tkservice::graphic3d::{Graphic3dShaderProgram, Graphic3dTypeOfLimit};
/// Native reports a fixed view budget; the structure manager separately caps view ids at 32.
const REPORTED_VIEW_LIMIT: i32 = 10_000;
/// Answer one limit; for example `inquire(MaxMsaa, 4096, Some(limits))`.
pub(crate) fn inquire(
    limit: Graphic3dTypeOfLimit,
    capture_limit: u32,
    device: Option<RenderDeviceLimits>,
) -> i32 {
    match limit {
        // Lights live in a storage buffer; native reports the default program light count.
        Graphic3dTypeOfLimit::MaxNbLights => Graphic3dShaderProgram::THE_MAX_LIGHTS_DEFAULT,
        Graphic3dTypeOfLimit::MaxNbViews => REPORTED_VIEW_LIMIT,
        Graphic3dTypeOfLimit::MaxViewDumpSizeX | Graphic3dTypeOfLimit::MaxViewDumpSizeY => {
            capture_limit as i32
        }
        _ => device.map_or_else(|| without_device(limit), |caps| device_limit(caps, limit)),
    }
}
fn without_device(limit: Graphic3dTypeOfLimit) -> i32 {
    // Native returns 1 combined unit without a context; every other device answer is 0.
    i32::from(limit == Graphic3dTypeOfLimit::MaxCombinedTextureUnits)
}
fn device_limit(caps: RenderDeviceLimits, limit: Graphic3dTypeOfLimit) -> i32 {
    match limit {
        Graphic3dTypeOfLimit::MaxNbClipPlanes => caps.max_clip_planes(),
        Graphic3dTypeOfLimit::MaxTextureSize => caps.max_texture_size(),
        Graphic3dTypeOfLimit::MaxCombinedTextureUnits => caps.max_combined_texture_units(),
        Graphic3dTypeOfLimit::MaxMsaa => caps.max_msaa_samples(),
        _ => i32::from(device_flag(caps, limit)),
    }
}
fn device_flag(caps: RenderDeviceLimits, limit: Graphic3dTypeOfLimit) -> bool {
    match limit {
        Graphic3dTypeOfLimit::HasSRGB => caps.has_srgb(),
        Graphic3dTypeOfLimit::HasBlendedOit => caps.has_blended_oit(),
        Graphic3dTypeOfLimit::HasBlendedOitMsaa => caps.has_blended_oit_msaa(),
        Graphic3dTypeOfLimit::HasFlatShading => caps.has_flat_shading(),
        Graphic3dTypeOfLimit::IsWorkaroundFBO => caps.is_workaround_fbo(),
        Graphic3dTypeOfLimit::HasMeshEdges => caps.has_mesh_edges(),
        // Metallic-roughness shading and its environment need only Vulkan 1.3 sampled float
        // formats (backend/pbr_environment), like GlContext::HasPBR on GL 3.0+.
        Graphic3dTypeOfLimit::HasPBR => true,
        Graphic3dTypeOfLimit::HasRayTracing => caps.has_ray_tracing(),
        Graphic3dTypeOfLimit::HasRayTracingTextures => caps.has_ray_tracing_textures(),
        Graphic3dTypeOfLimit::HasRayTracingAdaptiveSampling => {
            caps.has_ray_tracing_adaptive_sampling()
        }
        Graphic3dTypeOfLimit::HasRayTracingAdaptiveSamplingAtomic => {
            caps.has_ray_tracing_adaptive_sampling_atomic()
        }
        _ => false,
    }
}
#[cfg(test)]
#[path = "driver_limits_tests.rs"]
mod tests;
