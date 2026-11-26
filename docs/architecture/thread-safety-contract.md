# Martensite Thread-Safety Contract (#61)

This document defines Martensite's concurrency contract for callers, contributors, and CI enforcement.

## Core Guarantees

1. **Published results are immutable, `Send`, and `Sync`.** Reading published geometry, topology, and meshes does not require external locking. Removing remaining internal `SharedState` (`RwLock`) wrappers on published geometry is tracked in Wave 1 (`#27`, `#65`).
2. **Independent operations run concurrently.** Calling kernel operations on separate threads with independent inputs or shared immutable inputs must not race.
3. **Results are thread-independent.** A result must not depend on the calling thread, prior operations on that thread, or concurrent operations on other threads.
4. **Mutation stays inside operation-local builders.** Builders mutate local state before publication; once published, results are immutable.
5. **Every `unsafe impl Send` and `unsafe impl Sync` is justified and tested.** Each implementation carries a `// SAFETY:` invariant comment (enforced by `clippy::undocumented_unsafe_blocks`) and Miri or ThreadSanitizer coverage (`docs/features/thread-safety/unsafe-send-sync-audit.md`, `#63`).

## Public Type Classification

Every `pub struct`, `pub enum`, and `pub union` in `crates/martensite-*/src/` is registered in `docs/architecture/thread-safety-manifest.tsv` under one of three classes:

- **`shareable` (`Send + Sync`):** Safe to move across threads and share by immutable reference across threads.
- **`movable` (`Send`, `!Sync`):** Safe to move across threads, but requires exclusive ownership (`&mut self`) for access.
- **`thread-confined` (`!Send`, `!Sync`):** Must remain on the creating thread. Every `thread-confined` entry must record a non-empty reason (e.g., `Rc`/`RefCell` interior state or native OS/windowing FFI handle).

## Enforcement

- **Compile-time assertions:** Core public types across foundation, math, geometry, B-rep, and Boolean crates carry compile-time `Send`/`Sync` assertions (`crates/martensite-tkbo/tests/thread_safety_contract.rs` and `crates/martensite-tkernel/tests/unsafe_send_sync_audit.rs`).
- **CI manifest gate:** `cargo xtask pr` checks `docs/architecture/thread-safety-manifest.tsv` against every changed Rust source file under `crates/martensite-*/src/`. Any new public type without a manifest entry, or any `thread-confined` entry without a reason, fails CI.
