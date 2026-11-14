# Certified cylindrical boundary flux (#418)

The private rational pipeline retains a certified signed flux enclosure from its
decoded ruled patches and cap-pcurve occurrences. The public math entry point is
`certify_cylindrical_boundary_flux`. Its quantity is the boundary flux of
`F(x) = x/3`, with parameter multiplicity. Interpreting that quantity as body
volume still requires ideal-realization correspondence and injectivity proofs.
The private pipeline now checks [frozen incidence and representation
coverage](FROZEN-COVERAGE.md) before accumulating flux. The result remains `Unqualified`.

## Ruled-side reduction

The decoded side-net check requires two rows with the same retained weights.
Write their homogeneous coordinates as `N0/W` and `N1/W`. The ruled surface is
`S(u,v) = (N0 + v*(N1-N0))/W`. Integrating its oriented flux in `v` first gives

```text
integral_0^1 S dot (Su cross Sv) / 3 dv
    = det(N0, N0' + N1', N1-N0) / (6*W^3).
```

Terms containing `W'` cancel because they are parallel to `S`. Terms quadratic
in `v` vanish because their determinant repeats the depth direction. The
remaining one-dimensional numerator and denominator are exact integer
polynomials from the actual binary64 coefficients. Shared weights are admitted
before capture. Capture removes uniform powers of two independently from each
curve; those powers are aligned before combining the rows. This preserves the
common denominator without changing any geometry.

The existing rational Taylor-model integrator encloses this quotient. It
integrates constant denominators exactly and bounds other denominators with
exact remainders and certified positive Bernstein denominator coefficients.
A binary64 approximant chooses a Taylor model; acceptance uses its exact
remainder. No quadrature estimate or tessellation enters the certificate.
The side orientation is explicit relative to `du cross dv`.

## Analytic cap contribution

The caps are local +Z cylinders at the origin. Their outward parameter flux
density is `r^2/3`, so each directed polynomial UV occurrence contributes
`r^2/3` times its exact half Green integral. The first cap is inward and the
last cap outward. Both actual pcurves and both actual radii are used. Cap
contributions and side enclosures are summed with exact rational arithmetic;
only the final scalar report is rounded outward.

The mathematical report does not check whether those occurrences close, cover
a face once, bound material, or form a body. Open and collapsed parameterizations
can have completed scalar integrals. Such scalars do not validate geometry.
The native pipeline still keeps its separate sampled validation and native
volume estimate; the estimate is not reclassified as certified evidence.

## Native binding and resources

The private stage recaptures the decoded space curves, rechecks the ruled rows,
shared incidences and side trim domains, and copies the actual cap pcurve
controls after their class/range/unit-weight checks. Both cap residual
certificates must match the recaptured curves, UV controls and radii. Every
prepared span needs exactly one stored proof. Side orientation comes from the
declared native binding. The flux report retains source bits and supplied order; native bindings keep
the authored source keys. Coordinates and local positions do not define identity.

The prototype requests a `1e-9` absolute integration error in volume units,
separate from the spatial tolerance. It admits at most 4,096 spans and 1,024
examined cells per span, under one aggregate allowance of 8,192 cells. The math
interface validates explicit policies up to degree 16, 4,096 spans, 65,536 cells
per span and one million aggregate cells. A zero aggregate cell allowance can
complete constant-denominator work; it refuses required rational integration.
Required occurrences are never truncated, and exhausted work returns no partial
successful report.

Each side receives an exact width share of the total error cap divided by the
span count. The final midpoint rounding error must also meet the original cap.
The report records its enclosure, representative, achieved absolute error,
evidence kind, original policy and per-span effective/actual cell counts.
Effective cell limits are the minimum of the per-span limit and the aggregate
cells remaining on entry. All public report/query/error types have compile-time
`Send`/`Sync` controls; concurrent-operation stress remains a publication gate.

## Independent controls

A unit planar patch at `z=1` has signed flux `1/3`. A nonuniform rational
parameterization covers the same patch and retains that flux. For
`S=(u+v, u*v, 1+v/2)`, an independent exact symbolic determinant is
`u-v+u*v/2`; its unit-square flux is `1/24`. The retained Jacobian request and
result are in [the ruled oracle](references/rational-ruled-flux-oracle.json).
This case also exercises the independent homogeneous power-of-two alignment.
A separate exact control exercises both cap terms and gives flux `1/2`.

A bounded generated family contains 108 planar cases with zero widths/heights,
zero and negative flux, nonuniform weights and common weight gauges. Their
independent value is `width*height*z/3`. Other controls cover orientation reversal,
source-bit changes, invalid policies/radii/degree/count/weights, cancellation,
zero-cell work, aggregate exhaustion and per-span work receipts. Reducing the
aggregate allowance by one refuses the second required rational span.

The decoded mixed line/quadratic/cubic profile with a hole has all 19 spans in
its flux report and agrees with the independent ideal-volume control `11.10375`
within the existing fixture discrepancy threshold. Recapture refuses stale side
trim ranges and incomplete stored-proof membership. The original primitive,
seam-crossing and two-turn helical controls remain passing.

## Remaining measure and publication gates

The conditional flux enclosure is not yet a published volume measure. The
[source-matched comparison](MEASURE-COMPARISON.md) now bounds its discrepancy
from the ideal normal-extrusion volume. It retains the independent integration,
profile-displacement and comparison evidence. Disconnected material
classification, complete history, isolation and replay/concurrency verification
remain publication gates. Glyph scale, oblique frames and replacement deletion
also remain open. No version or release tag changed.

Verification on Linux x86_64 passed 521 math library tests, 3,114 math integration
tests, 56 math doctests, 55 sketch library tests and 32 sketch integration tests.
All 16 focused flux controls passed after the final export/diagnostic changes;
strict all-target clippy passed for both packages. The original isolated debug
fixture, including the new flux certificate and the separate native estimate,
took 3.54 seconds with peak RSS 52,428 KiB. The five-second limit is unchanged;
individual timings do not establish a performance improvement. The approximate
style scan found all inspected files under 500 lines, functions at most 20 code
lines and nesting at most two. The full affected-package PR gate remains required
before a push. No dependency, version or release tag changed.
