# Multi-threaded WebAssembly and WebGPU visualization

Martensite compiles its geometry, modelling, exchange, typography, and visualization crates to `wasm32-unknown-unknown` with WebAssembly Threads (`+atomics`, `+bulk-memory`, `+mutable-globals`) and renders interactive 3D CAD scenes through `martensite-webgpu` on WebGPU Core.

## Architecture overview

Browser deployments split execution across three realms:

1. The browser main thread owns the DOM, forwards pointer, wheel, touch, keyboard, and resize events to `WasmWindow`, transfers an `OffscreenCanvas` to the Orchestrator Worker, and never executes `Atomics.wait`.
2. The Orchestrator Worker runs the Rust CAD kernel and `WebGpuRenderer`. It calls `martensite_tkernel::osd::init_wasm_worker_pool` once during startup to pre-warm Compute Web Workers on shared `WebAssembly.Memory`, and calls `WebGpuRenderDriver::new_async` to bind the transferred `OffscreenCanvas`.
3. Pre-warmed Compute Web Workers park on a shared-memory `Condvar` inside `martensite_tkernel::osd::platform_thread` and execute parallel work submitted by `Parallel::scope_or_pool`, `ThreadPool`, BVH construction, meshing, and Boolean algorithms.

GPU rendering is separated from Vulkan through two crates:

- `crates/martensite-render-core` (zero `unsafe`) holds Graphic3d lowering, `RenderFrame`, `PrimitiveDraw`, `Graphic3dRenderDriver`, `Graphic3dRenderView`, `OpenPbrGpuMaterial`, `PtSceneData`, `RenderUserProgramLibrary` (SPIR-V and WGSL), and the `RenderDeviceBackend` trait.
- `crates/martensite-webgpu` (zero `unsafe`) implements `RenderDeviceBackend` and `RenderWindowPresenter` on `wgpu 30.0.2` using WebGPU Core only, with `WebGpuRealm<T>` checked thread-affinity confinement (`send_wrapper::SendWrapper`) and asynchronous readback (`WebGpuRenderViewExt::capture_async`).

Typography on `wasm32` runs in pure Rust inside `martensite-tkservice` through `swash`, `skrifa`, `read-fonts`, and a pure-Rust WOFF1 decoder (`flate2`). Applications load font bytes directly into memory via `FontFontMgr::register_font_bytes`.

## Required HTTP headers

Every document, worker script, and `.wasm` asset must be served with cross-origin isolation headers so browsers enable `SharedArrayBuffer`:

```http
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
Cross-Origin-Resource-Policy: same-origin
```

## Toolchains and verification commands

Native builds remain pinned to the stable toolchain in `/rust-toolchain.toml`. The nightly toolchain required for `-Z build-std=panic_abort,std` on `wasm32-unknown-unknown` is isolated in `scripts/build/wasm-toolchain.toml`.

Check all WASM-eligible crates from the repository root:

```bash
cargo xtask wasm-check
```

Run the native `martensite-webgpu` GPU test suite on Linux with Mesa lavapipe:

```bash
VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json cargo test -p martensite-webgpu
```

Build the release `wasm32 +atomics` end-to-end module and run it in headless Chromium:

```bash
python3 tests/wasm_e2e/run_chromium_e2e.py
```

## Related documents

- [Implementation plan](IMPLEMENTATION-PLAN.md)
- [Cross-browser capability matrix](CAPABILITY-MATRIX.md)
- [Chromium baseline evidence and benchmarks](EVIDENCE.md)
- [Phase 0 spike measurements](phase-0-spike.md)
- [ADR: Pure-Rust IEEE remainder](../../architecture/decisions/pure-rust-ieee-remainder.md)
- [ADR: wasm32 atomics worker pool](../../architecture/decisions/wasm32-atomics-worker-pool.md)
- [ADR: WASM realm confinement](../../architecture/decisions/wasm-realm-confinement.md)
- [ADR: render-core and WebGPU policy](../../architecture/decisions/render-core-and-webgpu-policy.md)
