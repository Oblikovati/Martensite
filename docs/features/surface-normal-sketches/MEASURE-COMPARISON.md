# Retained-flux and ideal-volume comparison (#418)

The private cylindrical pipeline now composes its completed retained-flux and
ideal normal-measure certificates. It recaptures the actual decoded geometry,
binds both measures to the same first-depth curves and reports a certified
outward difference. The composed report owns both input certificates, preserving
their evidence and work receipts without copying their source records.

## Difference bound

Let `[Fl, Fu]` enclose the actual retained signed boundary flux, and `[Vl, Vu]`
enclose the ideal normal-extrusion volume of the normalized stored profile.
Then

```text
retained_flux - ideal_volume lies in [Fl - Vu, Fu - Vl].
```

The discrepancy bound is the larger absolute endpoint of that interval. A
separate subtraction encloses the retained flux representative minus the ideal
volume and gives its absolute error bound. Every scalar capture, subtraction
and final conversion is outward. An interval containing zero does not establish
zero discrepancy; its full extent contributes to the bound.

This comparison uses the actual retained cap radii, cap pcurves and ruled-side
coefficients through the flux certificate, and the original radius/depth inputs
through the ideal-measure certificate. Consequently, its discrepancy bound
covers the numerical consequences of stored supports and rulings, cap/profile
mismatch, native radius-addition rounding and the input enclosures' uncertainty.
It does not replace the separate spatial storage and trim obligations. Their
bounds remain required under the requested spatial error budget.

The input flux integration error and the ideal profile-displacement error remain
available on their original certificates. Neither is relabelled as the other.
Their uncertainty is already included when comparing the complete enclosures;
it is not subtracted from the comparison or added a second time. The comparison
is arithmetic evidence conditional on the closed-profile and boundary premises.
It does not prove body validity or geometric injectivity.

## Provenance

The comparison requires a nonempty one-to-one pairing of every supplied flux
span and original profile source. Both completed reports must match those inputs.
For each pair, a reconstructed O2 query additionally binds the actual retained
first-depth curve, its stored radius, original polynomial, exact interval,
support radius and angular origin. Two individually valid reports for different
retained first-depth curves cannot be combined.

The native stage reuses the flux-capture path. It checks both actual cap supports,
recaptures every ruled span and cap pcurve, verifies retained trace bindings and
requires the original semantic span sequence. It also requires complete stored
proof membership. Shape coordinates or traversal rank never supply identity.
Frozen incidence and representation coverage still runs before this stage.

The output owns the input certificates. Publication can retain the scalar
comparison, integration work, geometric discrepancy and original source records
as one immutable object. Public comparison types have compile-time `Send` and
`Sync` controls; these assertions do not establish concurrent-operation safety.

## Policy

The query explicitly selects `Report` or `Refuse(volume_allowance)`. Refusal
checks the larger of the discrepancy and representative-error bounds against the
unchanged finite nonnegative allowance. Reporting asserts no caller tolerance.
The private native pipeline explicitly requests reporting while publication is
unfinished. The mixed fixture independently requires both bounds below `1e-4`.

Comparison revision one admits 8 through 4,096 fractional bits, up to 4,096
paired sources and at most one million charged operations. Defaults are 128 bits
and 100,000 operations. Each paired-source check consumes work and observes
cancellation. The comparison records its own work separately from both input
proofs. Invalid policies, stale bindings, incomplete pairing, cancellation,
exhausted work and failed requested bounds return typed errors without a partial
successful comparison. No tolerance, precision or work limit is widened.

## Controls and remaining work

Independent scalar controls give exact zero and an exact half-unit difference.
The latter deliberately has incompatible cap traces, demonstrating that scalar
completion does not validate a body. Another control preserves the full
ideal-profile uncertainty in both difference bounds. Other controls cover
changed cap geometry, changed depths, distinct retained first-depth curves,
unequal occurrence counts, explicit refusal, invalid policy, cancellation and
aggregate exhaustion. Native controls reject changed retained pcurve ranges,
missing source proofs and reordered semantic span receipts.

The numerical O6 comparison is now available for the current private cylindrical
pipeline. The result remains `Unqualified`: frozen disconnected families,
full history, isolation, replay/concurrency and publication checks remain open.
General cones and authored curve classes, glyph scale, oblique frames and
replacement deletion gates also remain open. No dependency, public primitive API,
version or release tag changed.

## Verification

The affected-package run on Linux x86_64 passed 547 math library tests, 3,114
math integration tests, 56 math doctests, 77 sketch library tests and 32 sketch
integration tests. Strict all-target clippy passed for both packages. The final
native error-forwarding cleanup passed clippy and all 77 sketch library tests
again. There are 11 focused mathematical comparison controls and three new native
recapture controls; the six normal-measure controls also passed together.

The isolated debug P1 fixture, including the composed comparison and separate
native validation/volume estimate, took 3.39 seconds under `/usr/bin/time`, with
peak RSS 52,668 KiB and compilation excluded. It remains within the unchanged
five-second limit. Individual timings establish neither improvement nor
regression. The approximate style scan found all inspected files under 500
lines, functions at most 20 code lines and nesting at most two. The full
affected-package PR gate remains required before a push.
