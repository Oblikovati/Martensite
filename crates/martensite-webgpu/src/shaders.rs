//! Modular WGSL shader library (< 150 KB total) using WebGPU Core only (#634, #635).

/// Unified surface vertex + fragment shader (Unlit / Phong / OpenPBR v1.1, clipping, mesh edges,
/// PCF shadows, and WBOIT dual-target output).
pub const SURFACE_WGSL: &str = include_str!("shaders/surface.wgsl");

/// Fullscreen background pass (corner gradient, 2D image, procedural skydome).
pub const BACKGROUND_WGSL: &str = include_str!("shaders/background.wgsl");

/// Instanced screen-space quad expansion for wide (`width > 1.0`) and stippled lines.
pub const WIDE_LINE_WGSL: &str = include_str!("shaders/wide_line.wgsl");

/// Instanced screen-space quad expansion for large points and textured marker sprites.
pub const POINT_SPRITE_WGSL: &str = include_str!("shaders/point_sprite.wgsl");

/// Weighted Blended OIT fullscreen composite (`Rgba16Float` accum + `R16Float` revealage).
pub const OIT_COMPOSITE_WGSL: &str = include_str!("shaders/oit_composite.wgsl");

/// `Depth32Float` shadow-map depth-only pass.
pub const SHADOW_DEPTH_WGSL: &str = include_str!("shaders/shadow_depth.wgsl");

/// `RenderShaderGrid` procedural coordinate grid.
pub const GRID_WGSL: &str = include_str!("shaders/grid.wgsl");

/// Compute shader for instanced placement frustum/visibility compaction (`draw_indexed_indirect`).
pub const PLACEMENT_CULL_WGSL: &str = include_str!("shaders/placement_cull.wgsl");

/// Compute shader for OpenPBR ray & progressive path tracing into `Rgba16Float` storage textures.
pub const RAYTRACE_RENDER_WGSL: &str = include_str!("shaders/raytrace_render.wgsl");

/// Fullscreen display pass for `Rgba16Float` raytrace accumulation buffers.
pub const RAYTRACE_DISPLAY_WGSL: &str = include_str!("shaders/raytrace_display.wgsl");

/// Stereo pair composition and subview blit fullscreen pass.
pub const STEREO_SUBVIEW_WGSL: &str = include_str!("shaders/stereo_subview.wgsl");

/// Total uncompressed WGSL source size across all built-in shaders in bytes.
pub fn total_wgsl_bytes() -> usize {
    SURFACE_WGSL.len()
        + BACKGROUND_WGSL.len()
        + WIDE_LINE_WGSL.len()
        + POINT_SPRITE_WGSL.len()
        + OIT_COMPOSITE_WGSL.len()
        + SHADOW_DEPTH_WGSL.len()
        + GRID_WGSL.len()
        + PLACEMENT_CULL_WGSL.len()
        + RAYTRACE_RENDER_WGSL.len()
        + RAYTRACE_DISPLAY_WGSL.len()
        + STEREO_SUBVIEW_WGSL.len()
}
