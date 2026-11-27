# Checked thread-affinity realm confinement for browser JS and WebGPU handles

Status: accepted for Milestone M10 (#630, #632, #634) on 2026-11-27.

## Context

Martensite's object model requires every managed object implementing `Transient` (`martensite-tkernel::standard::Transient`) to be `Any + Send + Sync`. Under `wasm32-unknown-unknown` with `+atomics`, browser handles (`web_sys::OffscreenCanvas`, `web_sys::HtmlCanvasElement`, and `wgpu` WebGPU handles backed by JS objects) are `!Send + !Sync` because JavaScript objects belong to the Web Worker or Window realm that created them.

Two repository invariants rule out ad-hoc wrappers:
- Workspace lints set `unsafe_code = "forbid"` (`Cargo.toml`). Both `martensite-render-core` and `martensite-webgpu` must contain zero custom `unsafe` blocks or `unsafe impl Send/Sync`.
- `scripts/build/src/global_state_guard.rs` (#62) forbids introducing new `thread_local!` or `static mut` items.

## Decision

Use the audited crate `send_wrapper = "=0.6.0"` (dual-licensed `MIT/Apache-2.0`, zero RustSec advisories) behind a small internal `RealmHandle<T>` wrapper in `martensite-tkservice` and `martensite-webgpu`.

1. `RealmHandle<T>` constructs a `send_wrapper::SendWrapper<T>` on the owning Orchestrator Worker or main thread and exposes checked fallible accessors (`try_get() -> Result<&T, ...>`) via `SendWrapper::valid()`.
2. Accessing a `RealmHandle<T>` on the owning thread performs a thread-ID comparison with no IPC or mutex overhead.
3. Accessing it from any other thread returns a typed error (`RenderError::ThreadConfined` in `martensite-webgpu` or a typed `Failure` in `martensite-tkservice`) instead of panicking or crossing JS realms unsafely.
4. Explicit `close()` methods drop the underlying JS/WebGPU resources on the owning thread before the wrapper leaves scope.

## Consequences

`WasmWindow`, `WebGpuRenderDriver`, and `WebGpuRenderView` satisfy `Transient: Send + Sync` and pass `thread-safety-manifest.tsv`, `unsafe_inventory`, `global_state_guard`, `cargo deny`, and `cargo audit` with zero custom `unsafe` code.
