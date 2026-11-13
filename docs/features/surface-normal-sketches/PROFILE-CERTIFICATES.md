# Cylindrical profile certificates (#418)

The private wrapped-profile pipeline compares each decoded curve to its original
polynomial in the developed `(s,z)` plane. It restricts the original coefficients
to the exact dyadic interval using outward arithmetic. Rounded subdivision
points are construction inputs only; they cannot replace the authored source in
this check.

## Position and derivative bounds

For retained homogeneous coefficients `(X,Y,Z,W)`, positive `W`, stored radius
`r` and authored angle `theta = origin + s/R`, the certificate bounds the L1
residual `(X-r W cos(theta), Y-r W sin(theta))/W` by `e`. If `e < r`, the
angular discrepancy is at most `e/(r-e)`. Positive projection onto the authored
radial direction establishes the unique nearby angular lift. The position
report combines `R` times that bound with the height discrepancy from `Z/W`.
A result is refused when the combined bound exceeds the requested allowance.

Derivative bounds compare `(X Y'-Y X')/(X²+Y²)` and
`(Z' W-Z W')/W²` to the authored angular and height derivatives. All derivatives
use the native piece's local parameter in `[0,1]`. A finite discrepancy alone
proves neither regularity nor compatible joins.

V1 uses fixed-grid outward dyadic arithmetic. Scalar trigonometric enclosures
use Machin's identity for argument reduction and degree-64/65 Taylor series.
Polynomial phase enclosures use a degree-8 centered Taylor series with an
explicit remainder. Two fixed midpoint refinements tighten the residual hull;
they affect proof cells only. Integer growth, operation count and cancellation
can each cause a typed refusal. Exhaustion never changes the geometric span.

Reports retain the original source bits, chart parameters and exact interval.
Source matching distinguishes signed zero and omitted versus explicit weights.
The reference profile uses the retained XY direction normalized to the support
radius and its retained height. Native storage residuals have a separate
certificate and budget.

## Geometry sizing

Before construction, `plan_cylindrical_fit` chooses one chart center and a
dyadic subdivision level for each original curve. Planning uses fixed 256-bit
arithmetic, independent of the later certificate's precision. A bound on the
fourth derivative of `tau = tan((theta-center)/2)` gives the ideal cubic Hermite
error `M4 h^4/384`; the angular displacement is at most twice that error.
V1 reserves half the position allowance for this ideal fit. Each geometric
halving reduces the bound by sixteen. An insufficient level cap refuses the
plan.

The native fit shares the chosen angular polynomial and weights across depth
curves, and rotates its local half-angle chart by the planned center. Actual
stored coefficients must still pass the independent profile and storage checks;
the ideal remainder does not certify floating-point construction or endpoint
rounding.

## Evidence and limits

The mixed line/quadratic/cubic loop with a rectangular hole now uses 19 pieces
and 117 semantic bindings. The four outer curves use subdivision levels
`1,2,3,0`; each hole curve uses level zero. Every piece meets the unchanged
`epsilon/4` profile allowance and `epsilon/1000` storage allowance. Native boss
and pocket controls pass. The operation still returns `Unqualified`.

Controls cover exact generators, exact interval restriction, nonzero height and
angular discrepancies, a rational-circle derivative oracle, source binding,
resource refusals and thread-safe report types. Scalar trigonometric intervals
contain independent direct rational Taylor bounds at `-12.5`, `-0.5`, `0`,
`0.5` and `12.5`, without reusing production argument reduction.

The final library runs passed 427 TKMath tests and 16 TKSketch tests. The earlier
joint package run also passed 3,114 TKMath integration tests, 56 doctests and
32 sketch integration tests. Strict clippy passed for both packages and all
targets. The isolated debug loop-with-hole control took 3.28 seconds with peak
RSS 48,380 KiB. A run competing with package tests took 5.03 seconds. Both used
the same local Linux test binary. The retained approximate metrics script found
no function over 20 code lines, file over 500 lines or nesting above two in the
profile, wrapped-pipeline and bounded-arithmetic files. The complete PR gate
still needs to run before pushing.

Endpoint homotopy, global periodic clearance, regularity, demotion, radius/depth
rounding, actual trims, represented measures and publication remain open. These
certificates make no topology verdict. General cone profiles are outside this
first polynomial-cylinder increment; primitive conical APIs remain available.
