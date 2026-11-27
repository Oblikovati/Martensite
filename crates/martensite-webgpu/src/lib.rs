//! Zero-unsafe WebGPU (`wgpu`) visualization renderer for Martensite (#634, #635).
//!
//! Implements `martensite_render_core::RenderDeviceBackend` on both browser WebGPU
//! (`wasm32-unknown-unknown` with `OffscreenCanvas` and `HtmlCanvasElement`) and native desktop
//! GPU backends (Linux Vulkan/lavapipe, macOS Metal, Windows DX12/Vulkan), staying strictly
//! within the WebGPU Core feature floor shared by Chromium, Firefox, and macOS Safari.

/// Compute pipelines for HiZ/placement culling, OpenPBR ray/path tracing, stereo, and subviews.
pub mod compute_passes;
/// Initialized `wgpu` instance, adapter, device, queue, and validation-scope tracking.
pub mod context;
/// Public driver constructors (`WebGpuRenderDriver`, `WebGpuDriverFactory`) and `capture_async`.
pub mod driver;
/// GPU uniform, vertex, lighting, material, and raytrace buffer packing.
pub mod gpu_types;
/// WebGPU driver creation options and WebGPU Core capability floor.
pub mod options;
/// Pipeline caches for surfaces, backgrounds, wide lines, points, capping, text, grids, and OIT.
pub mod pipelines;
/// Checked thread-affinity realm confinement and native blocking executor helpers.
pub mod realm;
/// `RenderDeviceBackend` implementation driving `RenderFrame` execution on `wgpu`.
pub mod renderer;
/// Modular WGSL shader sources (< 150 KB total) compiled with `override` specialization.
pub mod shaders;
/// `RenderWindowPresenter` implementation for `WasmWindow` (`OffscreenCanvas` / `HtmlCanvasElement`).
pub mod surface;
/// Offscreen render targets, MSAA, WBOIT, shadow maps, and async/blocking host readback.
pub mod targets;

pub use context::WebGpuContext;
pub use driver::{
    WEBGPU_DRIVER_NAME, WebGpuDriverFactory, WebGpuRenderDriver, WebGpuRenderViewExt,
};
pub use options::{WebGpuOptions, WebGpuPowerPreference};
pub use renderer::WebGpuRenderer;
