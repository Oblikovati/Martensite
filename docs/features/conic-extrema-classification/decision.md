# Kernel decision: conic minimum classification, version 2

Bug #406 showed that the elementary 2D point/curve solver marked a circle
maximum as a minimum and discarded all ellipse, hyperbola and parabola minima.
The rotated ellipse corpus returned the correct nearest root and distance,
but its false flag caused a sketch picker to discard it.

## One classification pipeline

`DifferentialV2` is the default for `ExtPElC`, `ExtPElC2d`, `ExtPC`, `ExtPC2d`,
`LocateExtPC` and `LocateExtPC2d`.
Each retained conic root passes through one shared local certificate. The
certificate evaluates position, tangent and second derivative on the analytic
curve. It checks stationarity in model distance units with
`precision::confusion()`. It then checks that a query displacement within this
resolution cannot change the second-variation sign. An exact binary64 predicate
classifies the supplied second variation:

```text
|C'(u)|² + (C(u) - Q) · C''(u)
```

A positive sign means a minimum; a negative sign means a maximum. A singular,
nonfinite, nonstationary or unresolved root has an explicit
`StationaryNature::Unresolved` with `Extrema_UncertifiedStationaryRoot`. Its
`is_min` accessor returns this error. The completed root search retains all
points and distances, including usable other roots. Generic `ExtPC` forwards
the same local evidence. Distance-only consumers can read distances without
fabricating a minimum classification. The certificate checks evaluated
coordinates and does not assert exact construction of transcendental values.
The same certificate serves all conic families in both dimensions.

The corrected policy replaces the constant and root-position flags, including
the old `NoSol == 0` classifier. It also replaces the 3D displaced-point test.
The root solver, coordinate evaluation, root order, parameter normalization and
deduplication remain unchanged. Lines retain their orthogonal-foot minimum.
There is no second root solver, recognizer or fallback.

## Replay and migration

`PointCurveClassification::LegacyV1` explicitly selects the previous stored
flags for replay. It preserves known wrong flags because earlier saved results
may depend on them. Call `set_classification_policy` before `perform` to select
this version. Changing policy invalidates the existing result. The generic
curve solver forwards this policy to its elementary solver. The locate solver
forwards it through the same curve solver, including owned-curve replay.

New computations use version 2. A consumer that stores solver behavior must
store the selected policy with its feature history. Recomputing legacy history
under version 2 is an explicit migration. Legacy selection is a result policy
on the same roots, and does not select another geometry engine. No existing
feature-history schema is changed by this kernel patch.

## Evidence

The regression first failed on main `e31cb82a`: six of ten returned root flags
were wrong. The corpus covers circle, rotated ellipse, hyperbola and parabola;
3D lifts; reversed parameter frames; repeat runs; and degenerate or unresolved
second variations. The certificate declines uncertain multiplicity; it does not
classify a small D2 by trying a higher derivative. Replay tests compare retained
geometry and squared-distance bits.

The rotated ellipse witness from independent OCCT projection is parameter
`0.68281751332396734`, foot `(2.918064003780093, -6.306597278227931)` and distance
`0.019024451883025126`. The test asserts this witness through `ExtPC2d`.
OCCT minimum flags are not used as an oracle because its older elementary
solver shares the flag defect. The exact-predicate tests include cancellation,
underflow and nonfinite input.

Recognizer delta: 0. Tolerance constants added: 0. Fallback sites added: 0.
Geometry type assertions added: 0. The metric checks use the existing modeling
resolution; exact signs use the existing predicate package.
