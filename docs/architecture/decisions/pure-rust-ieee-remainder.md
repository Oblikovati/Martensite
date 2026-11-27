# Pure-Rust IEEE 754 `remainder` in `martensite-cmath`

Status: accepted for Milestone M10 (#630, #631) on 2026-11-27.

## Context

`martensite-cmath` provides correctly rounded transcendental functions backed by the pure-Rust `metallic` crate so kernel geometry produces identical bits on Linux, macOS, and Windows (#201). One `extern "C"` declaration remained in release builds: `ffi::remainder(f64, f64) -> f64` in [lib.rs](../../../crates/martensite-cmath/src/lib.rs#L97-L119). Browser `wasm32-unknown-unknown` has no platform C math library to link against, so keeping that `extern "C"` declaration blocks WebAssembly compilation.

## Decision

Replace `ffi::remainder` in release builds with a pure-Rust, zero-`unsafe` implementation of IEEE 754 binary64 remainder (`x - n * y` where `n` is the integer nearest `x / y`, with ties rounded to even).

1. Implement exact binary64 exponent reduction in integer bit arithmetic covering normal inputs, subnormals, large exponent deltas up to 2046, exact half-way ties to even, signed zero preservation (matching the sign of `numerator`), and IEEE special values (infinite numerator, zero denominator, or NaN input produce NaN; finite numerator with infinite denominator returns `numerator`).
2. Gate `host-libm` and `libm-attribution` test-only C FFI hooks with `#[cfg(not(target_arch = "wasm32"))]`.
3. Verify bit-for-bit equality against native platform `libm::remainder` across exhaustive edge cases (subnormals, powers of two, ties, huge exponent gaps, negative zeros, and randomized binary64 bit patterns) on Linux, macOS, and Windows.
4. Remove the `unsafe extern "C"` block from release builds of `martensite-cmath` and reconcile `docs/features/safe-toolkits/unsafe-source-baseline.json` and `compiler-baselines/*.json`.

## Consequences

`martensite-cmath` compiles cleanly on `wasm32-unknown-unknown` with zero `extern "C"` linkage and zero `unsafe` in default release builds. All existing native callers receive bit-identical IEEE 754 results.
