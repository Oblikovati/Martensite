//! WebGPU device options and cross-browser WebGPU Core capability floor (#634).
use martensite_render_core::{
    RenderBehaviorConfiguration, RenderError, RenderLineClippingBehavior, RenderRasterPolicy,
    RenderWindowDepthBehavior, RenderWorldClippingBehavior,
};

/// Power preference forwarded to `wgpu::RequestAdapterOptions`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WebGpuPowerPreference {
    /// Let the browser or OS pick the default adapter.
    #[default]
    None,
    /// Prefer integrated / low-power GPU.
    LowPower,
    /// Prefer discrete / high-performance GPU.
    HighPerformance,
}

impl WebGpuPowerPreference {
    /// Convert to `wgpu::PowerPreference`.
    pub(crate) fn to_wgpu(self) -> wgpu::PowerPreference {
        match self {
            Self::None => wgpu::PowerPreference::None,
            Self::LowPower => wgpu::PowerPreference::LowPower,
            Self::HighPerformance => wgpu::PowerPreference::HighPerformance,
        }
    }
}

/// Configuration for initializing a `WebGpuContext` and `WebGpuRenderDriver` (#634).
///
/// All defaults stay strictly inside the WebGPU Core feature floor supported across
/// the latest versions of Chromium, Firefox, and macOS Safari.
///
/// Example: `WebGpuOptions::default()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WebGpuOptions {
    /// Adapter power preference.
    pub power_preference: WebGpuPowerPreference,
    /// Request a software fallback adapter (useful for deterministic headless testing).
    pub force_fallback_adapter: bool,
    /// Enable WebGPU validation error scopes around submissions (`push_error_scope`).
    pub validation: bool,
    /// Optional cap on maximum capture dimension (clamped to adapter `max_texture_dimension_2d`).
    pub max_capture_dimension: u32,
}

impl Default for WebGpuOptions {
    fn default() -> Self {
        Self {
            power_preference: WebGpuPowerPreference::None,
            force_fallback_adapter: false,
            validation: true,
            max_capture_dimension: 8192,
        }
    }
}

/// Reject Vulkan-only policies that WebGPU Core intentionally omits (#634).
pub(crate) fn validate_webgpu_policies(
    raster: RenderRasterPolicy,
    _clipping: RenderLineClippingBehavior,
    behavior: &RenderBehaviorConfiguration,
) -> Result<(), RenderError> {
    if raster.requires_last_vertex() {
        return Err(RenderError::rejected(
            raster,
            "RenderRasterPolicy::LegacyFirstV1 (WebGPU Core omits provoking-vertex-last extension)",
        ));
    }
    if behavior.window_depth() != RenderWindowDepthBehavior::HardwareV1 {
        return Err(RenderError::rejected(
            behavior.window_depth(),
            "RenderWindowDepthBehavior::HardwareV1 (WebGPU Core omits shader_float64)",
        ));
    }
    if behavior.world_clipping() != RenderWorldClippingBehavior::FragmentV1 {
        return Err(RenderError::rejected(
            behavior.world_clipping(),
            "RenderWorldClippingBehavior::FragmentV1 (WebGPU Core omits clip-distances)",
        ));
    }
    Ok(())
}
