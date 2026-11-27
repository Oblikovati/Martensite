# Multi-threaded `wasm32` worker topology, COOP/COEP, and panic contract

Status: accepted for Milestone M10 (#630, #631) on 2026-11-27.

## Context

Martensite's geometry kernel and visualization stack rely on shared-memory parallelism (`OSD_ThreadPool`, BVH construction, face meshing, and `BRepCheck`) and synchronous blocking primitives (`Mutex`, `Condvar`, `StandardCondition::wait`). Standard Rust prebuilt `std` for `wasm32-unknown-unknown` omits `+atomics`, and browser runtimes enforce two strict constraints on WebAssembly threads:

1. `SharedArrayBuffer` and `WebAssembly.Memory({ shared: true })` require cross-origin isolation via HTTP headers `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp` across Chromium, Firefox, and macOS Safari.
2. Calling `Atomics.wait` on the browser main window thread throws `TypeError`, and spawning a nested `web_sys::Worker` then immediately waiting on `Atomics.wait` before returning to the event loop deadlocks script delivery.

## Decision

1. **Separate nightly toolchain for `wasm32` atomics.** Keep `/rust-toolchain.toml` pinned to stable `1.99.0` for all native platforms. Pin `nightly-2026-09-25` with `rust-src` and `clippy` in [wasm-toolchain.toml](../../../scripts/build/wasm-toolchain.toml), invoked only by `cargo xtask` with `-C target-feature=+atomics,+bulk-memory,+mutable-globals` and `-Z build-std=panic_abort,std` until Rust stabilizes atomic `wasm32` `std`. All crate Rust source stays valid on stable MSRV `1.98.1`.
2. **Orchestrator Worker and pre-warmed Compute Worker Pool.** Browser hosts run Martensite inside an Orchestrator Dedicated Web Worker holding the transferred `OffscreenCanvas` (or keep canvas presentation on the main thread while delegating compute to workers). Before invoking any synchronous parallel kernel operation, the host awaits `martensite_tkernel::osd::parallel::init_wasm_worker_pool(count)` so compute workers finish loading and handshake asynchronously via `postMessage`. Once pre-warmed, synchronous `Atomics.wait` joins from the Orchestrator Worker complete without blocking worker bootstrap and without touching the browser main thread.
3. **COOP/COEP policy.** Require `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp`, which is supported across Chromium, Firefox, and macOS Safari. Do not rely on `credentialless`.
4. **Panic-abort contract for worker closures.** Because `-Z build-std=panic_abort,std` enforces `panic = "abort"`, `std::panic::catch_unwind` in `OSD_ThreadPool` does not catch panics on `wasm32`. Runtime errors across kernel and exchange operations return typed `Result<_, Failure>`; a panicking worker closure aborts the WASM instance.

## Consequences

Parallel kernel loops execute deterministically across multiple Web Workers on shared `WebAssembly.Memory` without modifying native thread-pool semantics. Headless Chromium spike measurements confirming both the deadlock hazard and the pre-warmed pool fix are recorded in [phase-0-spike.md](../../features/webassembly-webgpu/phase-0-spike.md).
