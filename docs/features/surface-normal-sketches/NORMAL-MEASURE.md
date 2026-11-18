# Conditional normal-extrusion measure (#418)

The private rational pipeline now retains a source-bound normal-volume enclosure
alongside its decoded boundary-flux certificate. The two quantities remain
separate. `certify_cylindrical_normal_measure` encloses an ideal realization from
the first-depth normalized stored profile and the original depth offsets. It
does not interpret the retained BRep flux as that ideal volume.

## Closed-profile area displacement

Write the authored developed curve as `(s,z)` and its normalized stored profile
as `(s+dx,z+dz)`. For complete closed contours with consistent lifted endpoints,

```text
A = integral s z' du
A_stored - A = integral (dx z' - s' dz + dx dz') du.
```

Integration by parts removes `s dz'`; its endpoint terms cancel around each
closed contour. Endpoints may move, but both authored and stored contours must
close with shared endpoint values. The mathematical routine does not establish
these premises. Native admission, source binding, shared incidence and frozen
coverage remain separate evidence.

For each source interval, the routine takes position and derivative discrepancies
from its retained O2 certificate. With `ex = R*angular_position_bound`,
`ez = height_position_bound` and `ezp = height_derivative_bound`, its area-error
contribution is bounded by

```text
ex*Var(z) + ez*Var(s) + ex*ezp.
```

For a Bernstein polynomial, `Var(f) <= sum abs(B[i+1]-B[i])`: integrate the
convex Bernstein bound for the derivative and cancel its degree factor. The
routine restricts the original polynomial to the exact dyadic source interval,
then encloses each difference outward. It never substitutes rounded subdivision
poles. Constant coordinates retain zero variation. When the height errors are
zero, this reduces to the review's angular-only bound.

The authored integral is enclosed by integrating the Bernstein coefficients of
`s*z'`. Every required piece contributes, including clockwise hole boundaries.
Signed and collapsed contours have scalar integrals; a completed scalar alone
does not establish a region or body.

## Exact requested depths

For support radius `R` and original offsets `d0 < d1`, the ideal factor is

```text
K = ((R+d1)^2 - (R+d0)^2)/(2R)
  = (d1-d0)*(2R+d0+d1)/(2R).
```

The second form avoids subtracting nearly equal squares. Radius additions and
all subsequent operations use outward dyadic arithmetic on the original
binary64 inputs. The factor does not use radii rounded during native construction.
Both ideal radii must be proven positive at the requested precision.

The Jacobian of `(r*cos(s/R), r*sin(s/R), z)` is `r/R`, giving the factor after
integration over radius. An independent exact symbolic determinant returned
`r*q*(c*c+s*s)`, where `q=1/R` and the oracle names `c` and `s`
denote cosine and sine. The unit-circle identity then gives `r/R`. The request and result are retained in
[the Jacobian oracle](references/normal-measure-jacobian.json).

The report encloses the authored signed volume `K*A` and the normalized-profile
signed volume `K*(A + [-area_error, area_error])`. It also reports the geometric
volume-discrepancy bound `K*area_error`. Arithmetic enclosure widths remain
visible. This discrepancy excludes retained-flux integration error and the
remaining comparison from the actual stored supports/rulings to the ideal.

## Policy and binding

The query explicitly chooses `Report` or `Refuse(volume_allowance)`. Reporting
does not assert a caller's volume tolerance. Refusal uses the unchanged finite,
nonnegative allowance. The native prototype explicitly requests reporting while
its publication contract is unfinished.

The work policy admits revision one, 8 through 4,096 fractional bits, at most
4,096 required pieces and at most one million aggregate charged operations.
Default arithmetic uses 128 bits and 100,000 operations. Work, precision and
source-count limits are never widened or clamped. Cancellation or exhausted work
returns a typed error without a partial successful measure.

Each source must match its immutable O2 certificate and the collection's common
support radius and angular origin. The report retains every O2 source record,
original depth bits, requested options, explicit error policy and aggregate work.
Its source matcher is independent of the work policy. Native evidence retains
the authored span keys. Public report/query/error types have compile-time
`Send` and `Sync` controls; concurrency tests remain a publication gate.

## Scope still open

This checkpoint supplies the closed-profile area and ideal normal-volume part of
O6. The [source-matched comparison](MEASURE-COMPARISON.md) now bounds the
retained-flux versus ideal-realization discrepancy through their full outward
enclosures. Separate spatial storage and trim obligations remain required. The
pipeline remains `Unqualified`. Frozen disconnected families, full history, isolation,
replay/concurrency, glyph scale, oblique frames and replacement deletion gates
remain open. No dependency, primitive API, version or release tag changed.

## Verification

The affected-package run on Linux x86_64 passed 536 math library tests, 3,114
math integration tests, 56 math doctests, 71 sketch library tests and 32 sketch
integration tests. All 15 focused measure controls passed after the final test
adapter cleanup. Three additional native binding controls passed: missing O2
membership, relabelled proofs and matching every prepared source. Strict
all-target clippy passed for both packages and again for the added sketch tests.

Independent controls include a rectangle with area `1/8` and ideal volume `3/16`,
a quadratic closed profile with area `1/24` and ideal volume `1/16`, and 16
rectangles covering negative orientation and collapsed dimensions. The quadratic
expectations use exact integer comparisons against the published binary64
bounds. Flattened normalized profiles have zero signed area, which exercises
conservative discrepancy bounds without implying injectivity. Separate controls
exercise constant and varying height errors, the angular-height cross term,
stale intervals, source order, cancellation, aggregate exhaustion, policy and
count refusal, nonpositive radii and depth offsets whose native radius additions
collapse to the same binary64 value.

The isolated debug P1 fixture took 3.43 seconds with peak RSS 52,648 KiB,
excluding compilation. The five-second limit is unchanged; a single timing does
not establish a performance improvement or regression. The approximate scan
found no inspected file over 500 lines, function over 20 code lines or nesting
over two. The full affected-package PR gate remains required before a push.
