# Walking-curve storage

Walking lines keep their constructor-supplied curve handle outside the lock for
mutable vertex metadata (#294). Point reads borrow this fixed handle. The curve
itself remains shared and mutable: its point access still takes its own locks,
and edits through any owner remain visible to the others.

Public `curve()` calls still clone the same handle. Null and out-of-range errors
retain their types and messages. Interrupted vertex edits retain the existing
poison-recovery behavior. Field order preserves destruction of the curve owner
before the vertex metadata.

## Verification

The `walking_curve_storage` tests cover public handle identity and reference
counts, live point edits, null and index failures, recovery after an interrupted
vertex edit, and concurrent readers with a writer. Compile-time checks require
both the walking line and its handle to be `Send + Sync`. The concurrent test
checks whole point snapshots and compares the final state with serial writes.

Five contract tests passed in each of two processes before and after the change;
their recorded observations matched byte for byte. Complete section-result
streams also matched before and after in two processes.

On Linux x86_64 with Rust 1.99.0, five standard Release batches of 200,000 point
reads produced these milliseconds:

| Storage | Batches | Median |
| --- | --- | --- |
| Curve inside metadata lock | 16.445, 16.289, 16.165, 16.830, 16.352 | 16.352 |
| Fixed curve handle | 6.957, 6.828, 6.931, 6.758, 7.505 | 6.931 |

The manual probe has no timing assertion. Run it with:

```sh
cargo test --release -p martensite-tkgeomalgo --test toolkit repeated_walking_point_read_cost -- --ignored --nocapture
```

The complete optimized workload in #294 still exceeds its 100 ms limit.
Its slower operation took 161.347 and 163.524 ms before this change, then
136.818 and 141.466 ms afterward. These are two processes on a shared host;
they establish a local improvement without closing the performance issue.

## ThreadSanitizer

The five contract tests also passed with ThreadSanitizer and no reported race.
The instrumented binary used nightly Rust 1.101.0 (2026-10-06),
`-Zsanitizer=thread`, and `-Zbuild-std=std,panic_unwind` on
`x86_64-unknown-linux-gnu`. The manual timing probe remained ignored.

The host instrumented process crashed before test discovery. A default container
reported an incompatible address-space layout. Running the same binary in a
read-only, network-disabled Ubuntu 24.04 container with a temporary `/tmp` and
`--security-opt seccomp=unconfined` allowed the sanitizer to set its process's
address-space layout. `TSAN_OPTIONS=halt_on_error=1` was set. No host preload or
security setting was changed. The container image was pinned to
`sha256:008173c23f95b170204355c12626cb5a965d779a7e1283b09e9cffbb1bf33ca3`.
