# Martensite constraint solver

TKSolve evaluates residual equations, analyzes numerical rank and solves constraints and coordinate preferences. The toolkit depends on the equation interface and works without a document or geometry kernel. The caller owns model units, equation domains and geometric acceptance.

## Provenance

The initial Rust source and tests came from Oblikovati `crates/solve` at `76f5e59a6fdff73500d1f977f0f5cbd46f7f51e6`. The latest source change at that revision was `8976d39fc802f692ed5a5a5fe8e32aabbc43db43`. The existing app can retain its solver import paths by re-exporting the same public modules.

The copied files retain their `GPL-2.0-only` notices. The maintainer approved the Apache-2.0 option for official Martensite release binaries in the Sketch3D D2 decision for [#611](https://github.com/Oblikovati/Martensite/issues/611). The ownership transfer depends on the D1 acceptance decision.

The imported Go-derived evidence is `tests/fixtures/go-ad.tsv` and `tests/fixtures/go-nonlinear.tsv`. Existing Rust behavior also retains the Go LM damping and residual stopping defaults and a zero-gradient convention at the nonsmooth zero of `sqrt`. A zero gradient at that point cannot establish local freedom. The structural diagnostics and independent finite-difference tests retain coverage for regular and degenerate inputs.
