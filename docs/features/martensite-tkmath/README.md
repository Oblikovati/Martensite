# martensite-tkmath

`martensite-tkmath` is the math toolkit. It depends on `martensite-tkernel` and
`martensite-cmath`, and on three pinned crates: `num-bigint` and `num-complex`
for exact and complex arithmetic, and `rustc_apfloat` for the x87 extended
`long double` that the polynomial solvers compute in.

## Modules

- `gp`: points, vectors, directions, axes and frames, conics and quadrics,
  matrices, quaternions, and 2D and 3D transformations.
- `math`, `math_lin`, `math_sys`: vectors and matrices with arbitrary bounds,
  linear algebra (Gauss, Crout, Householder, SVD, Jacobi, eigenvalues), and
  nonlinear systems (Newton, fixed-size 2D/3D/4D Newton, Levenberg-Marquardt).
- `math_root`, `math_poly`, `math_opt`, `math_integ`, `math_utils`: scalar root
  finders, polynomial solvers, minimizers (Brent, Powell, BFGS, conjugate
  gradient, Newton, particle swarm, global search, Uzawa), quadrature
  (Gauss, Kronrod, double exponential) and their shared helpers.
- `bsplclib`, `bsplslib`, `plib`, `convert`, `elclib`, `elslib`, `cslib`:
  B-spline curve and surface libraries, polynomial bases (Hermite, Jacobi),
  B-spline conversion of conics and elementary surfaces, elementary curve and
  surface evaluation, and surface normals with 2D point classification.
- `bnd`, `bvh`: bounding boxes, oriented boxes, spheres and ranges, and
  bounding volume hierarchies with their builders, sorters and traversals.
- `poly`, `toploc`: triangulations, polygons, mesh connectivity and mesh tools,
  and shared location chains.
- `predicates`: exact signs and certified roots for extension topology,
  sketch incidences and Bezier curve pairs, with error-bounded integrals.

The `debug-checks` feature enables the matrix index and singularity checks
that release builds leave out. `DEBUG_CHECKS` reports it; Cargo unifies
features, so a consumer reads the dependency's value.

## Tests

The tests are under `crates/martensite-tkmath/tests` and run with `cargo test`.
Every case file is a module of the one `toolkit` executable, listed in
`tests/suites/` (or declared in `tests/toolkit.rs` for the `exact_roots` and
`sketch_line_incidences` trees). `tests/case_listing.rs` fails when a case file
is in neither place. Unit tests next to the source cover the private helpers.

The expected values come from independent oracles: closed forms, invariants,
exact rational generators (`oracle*` files, with the generator and a README
beside them), Jacobian results over exact rationals, and reviewed baselines in
`tests/fixtures/` (see [that README](../../../crates/martensite-tkmath/tests/fixtures/README.md)).
No test needs Python or any other process at run time; a few tests start their
own executable again to compare two processes.

Test names describe the mathematical operation and the exercised case.

The benchmarks under `benches/` replay the certified root and incidence
corpora; `cargo bench -p martensite-tkmath` runs them.

## Unsafe code

The crate has no unsafe code and uses the workspace lints.

## Platform support

The crate has no platform-specific code. Some values reproduce the GCC and
glibc results on x86-64 Linux in software (x87 `long double`, `cbrt`, complex
arithmetic, `std::sort` tie order), so they do not depend on the host C
library. The other floating-point results use Rust's `f64` methods, which call
the platform math library for the transcendental functions.

### Release matrix

So far the tests ran only on Linux x86_64. Windows x64 was checked with clippy
(all targets, `-D warnings`) but not run; Linux arm64 and macOS arm64 were not
built. The source has no `c_char` widening or other char-signedness dependent
code. Tests that compare exact bits of results computed through the platform
math library (`sin`, `cos`, `atan2`, `exp`, `pow`) may differ by one ulp on
macOS and Windows. Run the manual `release` dispatch of the workflow before the
release and record each failing test here, with its cause.

## Dependency checks

Run `cargo deny --all-features --locked check` and `cargo audit` when changing
dependencies. The workspace policy pins the Unicode license exception to the
build driver's `unicode-ident` version, so an upgrade requires another review.
Every publishable path dependency declares its package version.
