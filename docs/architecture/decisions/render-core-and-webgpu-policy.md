# Shared `martensite-render-core` extraction and WebGPU rendering policy

Status: accepted for Milestone M10 (#630, #633, #634, #635) on 2026-11-27.

## Context

`martensite-vulkan` currently mixes backend-agnostic `Graphic3d` scene lowering (`Graphic3dRenderDriver`, `Graphic3dRenderView`, `RenderFrame`, materials, lighting, clipping, capping, text layout, backgrounds, shadows, placement batching, and CPU raytrace BVH construction) with Vulkan 1.3 `ash` internals, 1,455 precompiled SPIR-V binaries, and `rayon`. Sharing that lowering layer with a browser WebGPU backend without duplicating scene logic requires separating the frontend from the Vulkan device backend.

## Decision

1. **Zero-`unsafe` `martensite-render-core` crate.** Extract all backend-agnostic lowering, scene representation, `RenderFrame` IR, OpenPBR material packing/LUTs, and CPU raytrace BVH construction from `martensite-vulkan` into `crates/martensite-render-core` (`unsafe_code = "forbid"`, zero `ash`, zero `wgpu`, zero `rayon`). Replace `rayon` calls in shared lowering with `martensite_tkernel::osd::parallel` so parallel lowering works on both native threads and the pre-warmed WASM worker pool.
2. **Backend trait seam and source compatibility.** Replace the hardcoded `DriverBackend` enum with typed `RenderDeviceBackend` and `RenderWindowPresenter` traits. Implement both traits for `VulkanRenderer` in `martensite-vulkan` and re-export all moved public types (`pub use martensite_render_core::*;`) so existing callers and all 60+ Vulkan regression suites pass unchanged.
3. **WebGPU `wgpu` crate and modular WGSL.** Implement `crates/martensite-webgpu` (`unsafe_code = "forbid"`) on `wgpu = "=30.0.2"` (`default-features = false`; `webgpu` and `wgsl` on `wasm32`; native GPU backends and `wgsl` on desktop so the full WGSL suite runs on Linux Mesa lavapipe in CI). Replace precompiled SPIR-V permutations with modular WGSL shaders (< 150 KB total) specialized via WGSL `override` constants.
4. **Cross-browser WebGPU feature floor and Chromium performance baseline.** Require only WebGPU Core features and guaranteed limits supported across the latest versions of Chromium, Firefox, and macOS Safari (`Depth24PlusStencil8`, 4x MSAA, 8 storage buffers per stage, 256-byte uniform buffer alignment, fragment-shader clipping without `clip-distances`, and `Rgba16Float` paths without `float32-filterable`). Use the latest Chromium release as the authoritative performance baseline for WebAssembly and WebGPU benchmarks.
5. **Explicit capability refusals and async readback.** Return typed `RenderError` values for Vulkan-only policies (`logic_op`, `window_depth` f64, `conservative_rasterization`), `RenderError::AsyncReadbackRequired` when synchronous `to_pixmap()` is called on `wasm32` (use `WebGpuRenderViewExt::capture_async`), and `RenderUserProgramFailure` when a SPIR-V-only user shader is passed to WebGPU.

## Consequences

Both `martensite-vulkan` and `martensite-webgpu` consume one authoritative `Graphic3d` lowering frontend. Native Vulkan fixtures stay untouched, while `martensite-webgpu` maintains its own reviewed visual invariant suite and zero-validation-error gate on both native lavapipe and headless Chromium.
