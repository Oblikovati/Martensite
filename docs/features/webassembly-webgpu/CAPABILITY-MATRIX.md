# Cross-browser capability matrix

Martensite targets the intersection of WebAssembly, Web Worker, Canvas, cross-origin isolation, and WebGPU Core features supported across current desktop releases of Chromium, Firefox, and macOS Safari. Optional extensions outside WebGPU Core are never requested.

## WebAssembly and threading floor

| Capability | Purpose in Martensite | Chromium | Firefox | macOS Safari |
| --- | --- | --- | --- | --- |
| WebAssembly Threads (`+atomics`, `+bulk-memory`, `+mutable-globals`) | Shared linear memory across Orchestrator and Compute Web Workers (`martensite-tkernel::osd::Parallel`, `ThreadPool`) | Supported | Supported | Supported |
| `SharedArrayBuffer` with `COOP: same-origin` and `COEP: require-corp` | Lock-free synchronization and `Condvar` parking inside Compute Web Workers | Supported | Supported | Supported |
| Dedicated Web Workers (`new Worker(url, { type: "module" })`) | Pre-warmed compute worker pool initialized by `init_wasm_worker_pool` | Supported | Supported | Supported |
| `OffscreenCanvas` and `transferControlToOffscreen` | Worker-owned canvas presentation through `WasmWindow::from_offscreen_canvas` | Supported | Supported | Supported |
| `performance.now()` and `Date.now()` | Monotonic timers (`MonotonicInstant`) and wall-clock timestamps (`unix_timestamp`) | Supported | Supported | Supported |
| `navigator.hardwareConcurrency` | Default compute worker pool sizing | Supported | Supported | Supported |

Main-thread blocking is never used. The browser main thread stays on UI and input dispatch, while `Atomics.wait` runs only inside Dedicated Web Workers.

## WebGPU Core rendering floor

`WebGpuContext::new_async` requests `wgpu::Features::empty()` and `wgpu::Limits::default().using_resolution(adapter.limits())`. Every rendering pass in `martensite-webgpu` stays inside this core floor:

| Capability | Martensite usage | Chromium | Firefox | macOS Safari |
| --- | --- | --- | --- | --- |
| `Rgba8UnormSrgb` color attachment and `Depth24PlusStencil8` depth-stencil | Main CAD viewport, 4x MSAA resolve, and odd-even stencil capping | Core | Core | Core |
| `Depth32Float` depth-only render pass and comparison sampling | Shadow maps and PCF shadow lookups in `surface.wgsl` | Core | Core | Core |
| `Rgba16Float` and `R16Float` color targets (`textureLoad`) | Weighted Blended OIT accumulation and revealage (`oit_composite.wgsl`) | Core | Core | Core |
| `Rgba16Float` write-only storage textures (`texture_storage_2d<rgba16float, write>`) | Compute OpenPBR ray and progressive path tracing ping-pong buffers (`raytrace_render.wgsl`) | Core | Core | Core |
| Compute shaders (`@compute @workgroup_size(...)`) and `DrawIndexedIndirect` | Placement frustum culling (`placement_cull.wgsl`) and compute raytracing | Core | Core | Core |
| WGSL pipeline-overridable constants (`override SHADING_MODE`, `override USE_WBOIT`) | Specializing Unlit, Blinn-Phong, OpenPBR v1.1, and WBOIT pipelines from one shader module | Core | Core | Core |
| Asynchronous staging buffer mapping (`GPUBuffer.mapAsync`) | Non-blocking host readback through `WebGpuRenderViewExt::capture_async` | Core | Core | Core |
| Validation error scopes (`pushErrorScope("validation")`) | Zero-validation-error assertions in tests and diagnostics | Core | Core | Core |

## Intentionally omitted Vulkan-only features

Features outside the shared WebGPU Core floor are either emulated in WGSL or rejected with a typed `RenderError`:

| Vulkan feature | WebGPU behavior in Martensite |
| --- | --- |
| Geometry shaders (`mesh_edges`) | Replaced by per-triangle barycentric coordinates in `surface.wgsl` |
| Wide hardware lines (`wide_lines`) and stipple | Replaced by instanced screen-space quad expansion in `wide_line.wgsl` |
| Point sizes (`large_points`) and marker sprites | Replaced by instanced screen-space quads in `point_sprite.wgsl` |
| `clip-distances` extension | Fragment shader AND/OR half-space chain evaluation in `surface.wgsl`; `RasterClipDistanceV2` is rejected with `RenderError` |
| `shader_float64` window depth reconstruction | `RenderWindowDepthBehavior::UnsnappedWindowDepthV2` is rejected with `RenderError` |
| Provoking-vertex-last extension | `RenderRasterPolicy::LastProvokingVertexV2` is rejected with `RenderError` |
| SPIR-V user shaders | Rejected with `RenderUserProgramFailure::SpirvUnsupportedOnWebGpu`; WebGPU executes WGSL user programs registered through `RenderUserProgramLibrary::insert_wgsl` |
| Hardware ray tracing (`VK_KHR_ray_query`) | Replaced by the WGSL compute BVH OpenPBR path tracer (`raytrace_render.wgsl`) |
