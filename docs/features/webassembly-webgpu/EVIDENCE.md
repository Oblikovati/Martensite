# Chromium WebAssembly and WebGPU baseline evidence

This document records the measured build, threading, memory, and rendering results for Milestone 25 (#630 through #636) on Linux `x86_64` using headless Chromium and native Mesa lavapipe.

## Test environment

- Host OS: Linux `x86_64` (32 logical CPUs reported by `navigator.hardwareConcurrency`)
- Native Rust toolchain: stable `1.99.0` (`/rust-toolchain.toml`)
- Isolated WASM toolchain: `nightly-2026-09-25` with `rust-src` and `wasm32-unknown-unknown` (`scripts/build/wasm-toolchain.toml`)
- Linker flags: `-C target-feature=+atomics,+bulk-memory,+mutable-globals -C link-arg=--shared-memory -C link-arg=--max-memory=1073741824 -C link-arg=--import-memory` plus TLS exports
- Browser baseline: headless Chromium `148.0.7778.96` launched via Playwright with `--enable-unsafe-webgpu --use-angle=vulkan --enable-features=Vulkan` (AMD RDNA-3 WebGPU adapter)
- Native software reference: Mesa lavapipe (`VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json`)
- Harness command: `python3 tests/wasm_e2e/run_chromium_e2e.py`

## Artifact size and linear memory

Measured on the release `martensite-wasm-e2e` `cdylib` (`opt-level = "s"`, `lto = "thin"`, `codegen-units = 1`), which links `martensite-tkernel`, `martensite-tkmath`, `martensite-tkbrep`, `martensite-tkprim`, `martensite-tktopalgo`, `martensite-tkmesh`, `martensite-tkservice`, `martensite-tkv3d`, `martensite-render-core`, and `martensite-webgpu`:

| Metric | Measured value |
| --- | --- |
| Uncompressed `.wasm` binary | 17,107,047 bytes (16.31 MiB) |
| Gzip-compressed `.wasm` (`gzip -9`) | 4,747,652 bytes (4.53 MiB) |
| Built-in WGSL shader library (`total_wgsl_bytes()`) | 31,458 bytes (30.7 KiB, under 150 KiB budget) |
| Peak WASM linear memory after worker pool, BVH, BRep, and WebGPU passes | 21,364,736 bytes (20.37 MiB) |

## Multi-threaded WebAssembly worker scaling

The Orchestrator Worker pre-warms 7 Compute Web Workers via `init_wasm_worker_pool_from_url` (8 total execution threads including the Orchestrator) and executes 2,400,000 trigonometric and square-root kernel evaluations through `martensite_tkernel::osd::Parallel::scope_or_pool`:

| Active threads | Elapsed time (ms) | Speedup vs 1 thread | Determinism hash |
| --- | --- | --- | --- |
| 1 thread | 78.81 | 1.00x | `413432508` |
| 4 threads | 42.47 | 1.86x | `413432508` |
| 8 threads (Orchestrator + 7 Compute Workers) | 11.74 | 6.71x | `413432508` |

Main-thread non-blocking verification:

- `self.crossOriginIsolated`: `true`
- `Atomics.wait` invocations on browser main thread: `0` (`mainThreadBlocked: false`)
- 16,384-element `BvhIndexedBoxSet` build inside the Orchestrator Worker: `78.59 ms` (`8191` BVH nodes)
- `BRepPrimAPIMakeSphere(1.0)` construction: `2.06 ms`

## WebGPU rendering and validation results

Rendered to a 256x192 worker-owned `OffscreenCanvas` bound through `WasmWindow::from_offscreen_canvas` and read back without blocking via `WebGpuRenderViewExt::capture_async`:

| Stage | Chromium WebGPU (ms) | Lit pixels (non-background) | WebGPU validation errors |
| --- | --- | --- | --- |
| `WebGpuRenderDriver::new_async` initialization | 32.73 | n/a | 0 |
| OpenPBR Surface v1.1 shaded BRep sphere (first frame + pipeline compile + async readback) | 36.21 | 3,648 | 0 |
| Compute OpenPBR BVH ray/path tracer (BVH upload + compute pass + display + async readback) | 22.17 | 3,028 | 0 |

## Native and workspace verification summary

| Gate | Scope | Outcome |
| --- | --- | --- |
| `cargo xtask wasm-check` | All 41 WASM-eligible crates on `wasm32-unknown-unknown` (`+atomics`, `-Z build-std=panic_abort,std`) | Passed |
| `cargo test -p martensite-webgpu` (Mesa lavapipe) | 6 integration tests covering OpenPBR, Blinn-Phong + 4x MSAA, compute raytracing, wide/stippled lines, point sprites, mesh edges, WBOIT, clipping + stencil capping, stereo, subviews, WGSL user shaders, SPIR-V and policy refusals, and realm confinement | 6/6 passed, 0 validation errors |
| `cargo test -p martensite-render-core --lib` | Extracted backend-agnostic Graphic3d lowering and scene unit suite | 796/796 passed |
| `cargo test -p martensite-tkservice` | `WasmWindow`, DOM input translation, pure-Rust WOFF1 + `swash` typography | 554/554 passed |
| `cargo test -p martensite-build` | Build driver, `unsafe_inventory`, `thread_safety_manifest`, `global_state_guard`, `wasm-time-guard`, and CI workflow assertions | 492/492 passed |
| `cargo audit` | Full `Cargo.lock` dependency graph (253 crates) | 0 vulnerabilities |
