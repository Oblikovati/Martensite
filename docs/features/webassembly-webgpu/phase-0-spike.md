# Phase 0 vertical spike and dependency audit (#630)

Recorded on 2026-11-27 against `Google Chrome for Testing 156.0.8078.4` (`HeadlessChrome/156.0.0.0`) on Linux x86_64 (`AMD Ryzen AI Max+ 395 w/ Radeon 8060S`, RDNA-3 WebGPU over Vulkan).

## Dependency and supply-chain audit

Candidate crate set audited with `cargo deny --all-features check` and `cargo audit`:

| Crate | Pinned Version | License | Outcome |
| --- | --- | --- | --- |
| `wgpu` (`default-features = false`, `wgsl`, `webgpu` on wasm32, `vulkan`/`metal`/`dx12` on native) | `=30.0.2` | `MIT OR Apache-2.0` | Selected. Requires three reviewed transitive exceptions in `deny.toml`: `codespan-reporting 0.13.1` (`Apache-2.0`, via `naga`), `spirv 0.4.0+sdk-1.4.341.0` (`Apache-2.0`, via `naga`), and `foldhash 0.2.0` (`Zlib`, via `hashbrown`). |
| `wasm-bindgen` / `wasm-bindgen-futures` / `js-sys` / `web-sys` | `=0.2.130` / `=0.4.80` / `=0.3.107` | `MIT OR Apache-2.0` | Selected. Zero advisories and zero license exceptions. |
| `send_wrapper` | `=0.6.0` | `MIT/Apache-2.0` | Selected over `fragile` (`Apache-2.0` only). Zero advisories and zero license exceptions. |
| `swash` (backed by Google Fonts `skrifa 0.44.0`) | `=0.2.10` | `Apache-2.0 OR MIT` | Selected over `ttf-parser 0.25.1`, which triggered `RUSTSEC-2026-0192` (unmaintained, 2026-06-28), and over `ab_glyph_rasterizer` (`Apache-2.0` only). Zero advisories and zero license exceptions. |
| `flate2` (`rust_backend`) | `=1.1.10` | `MIT OR Apache-2.0` | Already pinned in workspace; used for pure-Rust WOFF1 inflation. |
| Web Worker pool | Internal `web-sys::Worker` shim | Workspace license | Selected over `wasm_thread` for direct control over pre-warmed worker bootstrap and zero extra transitive crates. |

Final check result with `ttf-parser` removed and the three `wgpu` transitive exceptions recorded in `deny.toml`:
- `cargo deny --all-features check`: `advisories ok, bans ok, licenses ok, sources ok` (120 locked crates).
- `cargo audit`: 0 vulnerabilities, 0 warnings across 121 dependencies.

## Headless Chromium vertical spike measurements

Served over local HTTP with `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp` in `HeadlessChrome/156.0.0.0`:

| Check | Measured Result |
| --- | --- |
| `crossOriginIsolated` / `SharedArrayBuffer` | `true` / `true` (`hardwareConcurrency = 32`) |
| Main-thread `Atomics.wait` guard | Confirmed `TypeError` thrown on main thread |
| Main-thread `HtmlCanvasElement` WebGPU clear | Pass (`rgba8unorm`, 0 validation errors) |
| Orchestrator Worker `OffscreenCanvas` WebGPU adapter | `amd` / `rdna-3`, 26 features, `maxStorageBuffersPerShaderStage = 16`, `minUniformBufferOffsetAlignment = 256`, `maxTextureDimension2D = 16384` |
| Orchestrator Worker WebGPU init time | `33.49 ms` |
| Orchestrator Worker WGSL render + `mapAsync` readback (`128x128`) | `5.14 ms`, center pixel `[51, 153, 204, 255]` exact match, 0 validation errors |
| Pre-warmed 4-worker pool startup (`WebAssembly.Memory({ shared: true })`) | `7.33 ms` |
| 4-worker parallel atomic reduction (2,000,000 iterations, synchronous `Atomics.wait` join on Orchestrator Worker) | `4.74 ms`, exact sum `2,000,000` |
| End-to-end spike duration / page errors | `87.54 ms` / `0` errors |

An initial spike run that blocked the Orchestrator Worker with `Atomics.wait` before the four nested workers posted their readiness message timed out at 30 s, confirming directly why `init_wasm_worker_pool(count).await` must complete asynchronously before synchronous parallel joins run.

The throwaway spike scripts were removed after recording these measurements.
