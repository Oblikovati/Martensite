# Retained Bezier coefficient rows

Bug [#464](https://github.com/Oblikovati/Martensite/issues/464) blocks exact conical
side certification when a boundary curve retains a weight one ULP below one and
the ruled surface normalizes it to one. The native-only reproduction and library
hashes are in [464-native-stages.json](references/464-native-stages.json).

## Diagnosis

A degree-one curve with weights `[1, next_down(1)]` is rational. Its copy retains
both weight bits. `GeomFillBezierCurves` passes those weights into the surface
constructor, which classifies weight differences with `real::epsilon(abs(w))`.
Curve classification uses `gp::resolution()`. The surface therefore becomes
polynomial with unit weights. The copied curve is unchanged.

The relevant constructor and filling sources matched main
`b63b9b8ba4dd7d25a951dd45bfd144b282715940`. The initial standalone probe used
existing feature-worktree libraries, whose paths and hashes are recorded; it was
not a separately rebuilt clean-main executable. Its stdin source and temporary
executable were discarded. Focused regressions now run in a separate main-based
worktree.

## Coefficient retention V1

The opt-in behavior is `StoredBezierCoefficientsV1`. Existing constructors,
filling entry points and binary readers keep their replay behavior.

`GeomBezierSurface::from_stored_coefficients` validates native pole-count and
weight-grid constraints, finite coordinates, and finite weights greater than
`gp::resolution()`. It retains nonconstant admissible grids exactly. Directional
flags follow exact weight variation in the corresponding surface parameter.
All-unit grids remain polynomial and use the existing shared unit-weight view.
Copies preserve the retained coefficients and evaluation remains native.

Constant non-unit grids receive
`BezierSurfaceCoefficientError::ConstantNonUnitWeights`. The existing binary
format omits weights when both rationality flags are false. The preserving API
refuses that case instead of changing the flags, format or supplied coefficients.
For a common nonzero weight `w`, the exact real Bezier quotient cancels `w` from
numerator and denominator; Bernstein partition of unity gives the polynomial
parameterization. The actual boundary constructor already canonicalizes `[2,2]`
to unit weights. A regression checks that state before filling and the final unit
rows. No additional canonicalization is hidden in the preserving surface API.

`GeomBezierSurface::ruled_with_stored_boundaries` consumes two
`StoredBezierBoundary` coefficient buffers. It admits each curve into private
owned state, builds the surface from those actual canonical rows, and returns
`StoredRuledGeometry` containing both curves and the surface. Equal degrees are
required; degree elevation would change the supplied coefficient rows. Finite
coordinates and original weights are checked before native handles escape.
Constant curve weights use the existing curve canonicalization, so both the
curves and the side use unit coefficients in that case.

The returned native geometry is mutable operation-local construction state,
not an immutable published result. Each curve and the surface have independent
storage. The feature caller must still validate, encode and freeze the result.
The shared `ruled_control_grid` primitive also serves the unchanged legacy Bezier
and B-spline filling paths; it performs no numerical admission itself.

## Binary replay and errors

`BinShapeSet::set_bezier_surface_weight_read_policy` selects
`BezierSurfaceWeightReadPolicy::PreserveStoredV1` for subsequent grouped reads.
It composes with existing frame policies and propagates through trimmed and
offset basis surfaces. Writing and binary format versions are unchanged.
Records that omitted weights decode as unit-weight records; missing historical
coefficients cannot be recovered.

The existing decoder returns `standard::Failure`, which has a native exception
category and message but no Rust error payload. A constant non-unit refusal keeps
the distinct `Geom_ConstantNonUnitBezierWeights` category, construction-error
ancestry and offending weight bits. The strict reader preserves that original
failure through recursive reads. The feature operation must match the category
through `BezierSurfaceCoefficientError::CONSTANT_NON_UNIT_EXCEPTION`, retain the
original failure as its cause and return a typed operation refusal. Message text
is not a dispatch protocol. Direct surface and patch construction return the Rust
coefficient error enum.

## Verification and remaining qualification

Owned controls check near-unit rows, unit grids, constant non-unit refusal,
malformed coefficients, copy isolation, directional flags and independent linear
rational values and first derivatives. Codec controls cover direct and grouped
reads, nested bases, unchanged legacy demotion, unit records, and every truncated
prefix's legacy error and cursor behavior. The curve-to-fill control proves that
constant curve weights are already unit at the actual caller boundary.

The feature must still qualify its original negative-axis signed-zero case with
the opt-in builder and decoder and preserve its exact row check. The [performance qualification](performance.md) records faster timings on the
tested native workloads and the unresolved peak-memory limitation. The affected-
package gate and actual feature caller qualification remain pending. No crate version, format version or release tag is
changed.
