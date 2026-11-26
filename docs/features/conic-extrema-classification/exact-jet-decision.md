# Kernel decision: exact stationary-jet certificates

This decision supersedes [the initial classification decision](decision.md).
Bug #406 uses `DifferentialV2` for new computations and explicit `LegacyV1`
selection for replay. A completed search retains each root, point and distance.
An unresolved local class carries a named failure through `classification`
and `is_min`; other roots remain available.

## One exact accumulator

The predicate `certify_distance_stationary_jet` evaluates both metric checks
from one exact accumulator over the supplied binary64 coordinates. For
`r=C-Q`, tangent `t=C'`, second derivative `s=C''`, and query displacement
scale `δ`, it computes:

```text
normal residual: r·t
second variation: q=|t|²+r·s
stationarity: (r·t)² <= δ²|t|²
resolved variation: q² > δ²|s|²
```

Every finite binary64 is an integer multiple of `2^-1074`. Both sides of each
comparison have the same fourth-power scale. Integer comparisons therefore
need no rounded subtraction, norm, square root or intermediate product.
The permitted displacement norm is `|δ|`; zero requests exact stationarity.
The kernel uses the existing `precision::confusion()` as its query resolution.

This deletes the provisional `distance_second_variation_sign` entry point
and the separate rounded `certify_stationarity` and `certify_second_variation`
checks. One jet predicate returns a resolved sign, singular tangent,
nonstationarity or a variation inside the resolution band. The conic solver
maps the latter states to `Extrema_UncertifiedStationaryRoot` for that root.
No root solver, geometry recognizer or fallback is added.

## Cancellation witnesses

For `B=5*2^50+1`, use ellipse radii `a=8*(B+1)`, `b=8*B` and query `(16,0)`.
At parameter zero, `b²-a*(a-16)=64` exactly, but rounded products subtract to
`2^58`. The rounded magnitude clears a false conditioning bound. The public
regression fails against local candidate `7afb3e85`; the exact comparison must
keep this root unresolved.

For point `(2^54,2^54)`, query `(1,0)` and tangent `(1,-1)`, the exact normal
residual is -1 although rounded point subtraction loses it. The same accumulator
must refuse this nonstationary jet. Tests also check exact boundary equality,
adjacent binary64 scales, subnormal signs and nonfinite input indices.

## Limits

The certificate concerns supplied binary64 positions and derivatives. It does
not certify construction of transcendental values or classify uncertain root
multiplicity. It reports uncertainty locally and preserves the distance-only
query path. Root generation, order, coordinates and replay flags use the same
existing operation pipeline. The recognizer, tolerance-constant, fallback-site
and geometry-type-assertion deltas are all zero.
