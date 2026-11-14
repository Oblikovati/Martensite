# Coherent Bézier coefficient capture (#449)

The fresh source checks retained by #449 capture every Bézier pole and weight
through separate getters. A cubic capture takes ten reads of the same state.
The pole and weight getters also format a diagnostic member name on every
call, including successful reads.

`Geom2dBezierCurve::poles_weights_snapshot()` and
`GeomBezierCurve::poles_weights_snapshot()` copy the stored poles and optional
rational weights under one read guard. The shared Bézier member macro defines
both methods. Their returned `Array1` values own separate vectors:
`Array1::clone` copies with `Storage::Owned(self.as_slice().to_vec())`.
The capture caller converts those arrays into its existing owned coefficient
representation.

The snapshot reads stored coefficients. It leaves the evaluator override
binding untouched and does not invoke or clone an evaluator, reset caches,
normalize coefficients, or change rationality. Nonrational curves still return
no weight array. The existing blocking read and poison-recovery behavior is
preserved.

Every fresh live-geometry validation remains in place. This change reduces
work within each capture; it does not cache validity, hold leases across
numerical work, or change subdivision budgets and geometric tolerances.
A coherent snapshot of an entire graph is a separate ownership change.

The regression scope covers both dimensions, signed-zero preservation,
rational and nonrational weights, mutation of the source and returned arrays
in both directions, concurrent atomic pole/weight edits, unchanged evaluator
bindings, and fresh captures after source mutation. Performance qualification
uses the same accepted full-glyph input and explicit work policy as the
capability baseline, with compilation excluded from runtime measurements.

The helper is additive and belongs to the candidate feature bucket. Package
versions remain unchanged until the maintainer approves a release.

Focused checks passed with two build jobs, incremental compilation disabled,
and debug information disabled and stripped:

- `cargo test -p martensite-tkg2d -p martensite-tkg3d --test toolkit coefficient_snapshot -- --test-threads=2`: six passed.
- `cargo test -p martensite-tkbo --lib sketch_region_geometry_capture -- --test-threads=2`: two passed.

The full-glyph before/after comparison remains a qualification step; no
speedup is claimed for this followup until that measurement is complete.
