# Captured-input work for conical text

Status: local whole-text qualification passed, 2026-11-20. Integration
and matched performance qualification remain pending.

`DirectedFamilyChartRequireCertifiedV2` selects `PositiveXFamilyChartV3`,
which admits up to 4096 original span occurrences in one positive-X chart.
The earlier `DirectedCommonChartRequireCertifiedV1` keeps its 128-span
ceiling. The chart geometry, zero lifts and angular range are unchanged;
this does not provide an atlas or periodic topology.

The new family selector takes `WrappedConeArithmeticGrantPolicy`.
`OriginalV1` retains the one-million aggregate ceiling. `InputBoundV1`
admits an explicitly requested aggregate against:

```
min(16_000_000, max(1_000_000, N * (L + 1) * 4096))
```

`N` counts selected original span occurrences in the captured regions;
`L` is the admitted maximum fit level, from zero through eight. Count and
multiplication are checked before charged proof work starts. Adaptive splits
and rejected fits do not change the allowance. The coefficient is a versioned
resource policy; it does not promise completion or bound CPU time.

The request's `proof.arithmetic.max_operations` remains the requested
aggregate. An excessive request is refused. Admission does not replace it
with the derived ceiling, retry a refusal or replenish the ledger. Each exact
primitive receives at most `min(remaining aggregate, 1_000_000)`; an exhausted
primitive still refuses when aggregate work remains. Precision, spatial and
integration limits retain their requested values.

A completed result exposes `arithmetic_grant_receipt()` with the policy,
original count, fit level, coefficient, requested allowance, input ceiling,
hard ceiling and primitive ceiling. An operation-owned source snapshot binds
the receipt to ordered regions, loop partitions, authored span identities,
raw pole bits, depths and fit settings. Publication checks that binding. The
snapshot adds retained memory in this explicitly selected mode; earlier modes
retain no grant snapshot. Final time and peak-memory qualification is pending.

The owned 239-curve, 18-region text fixture derives an 8,810,496-operation
ceiling at fit level eight. Its original one-million request exhausted during
fitting after source-capacity admission. That refusal remains a separate
regression control. With the explicit larger request, both normal senses passed fitting and
refused at the first cap trim of region 9001, curve 9027. The inward and
outward certified upper bounds were `5.3803e-6` and `5.4866e-6` model units
respectively, against a `5e-6` limit. These are bounds on
trace disagreement, so they do not establish the actual position error.
The operation returned no body in either case. The test diagnostics
record 6.21 s inward and 6.25 s outward for the refused operations.
Those runtimes do not qualify a completed operation.


## Trim correspondence

The [diagnostic](references/cone-text-239-capacity-2026-11-20/first-outward-trim-bound.json)
isolated the outward refusal to the angular term. The radius bound was about
12.8024, and the angular error bound was about `3.03e-7` radians. Their product
is about `3.88e-6` model units. The original composition scales this product
by an outward approximation of `sqrt(2)` for the XY L1 diagnostic. It then
adds that diagnostic to the axial bound, producing a position bound above
the requested limit.

FamilyV3 retains the XY L1 and axial diagnostics and uses a separate
Euclidean position bound. At each parameter, changing radius at fixed angle
moves the point by the absolute radial difference. Changing angle at fixed
native radius moves it by a chord no longer than the radius times the
absolute angle difference. Adding these two upper bounds, the axial bound,
and the stored-edge L1 error bounds the Euclidean discrepancy by the
triangle inequality. Interval arithmetic rounds the sum outward. The
decoded curves, coefficient binding, full-interval angular and meridian
proofs, and requested limit remain the same. OriginalV2 keeps its original
sum and arithmetic order.


The [next whole-operation run](references/cone-text-239-capacity-2026-11-20/whole-text-family-trim-results.json)
passed trim validation for both normal senses. Both then refused the
shared-ruling proof for region 2001, curves 2003 and 2004. The proof uses
the chord across the two sides as a shared projection direction and
requires the derivative along it to be strictly positive on both sides.
The certified lower bound was negative. The negative lower bound leaves the intersection question unresolved. The one-million refusal
control passed again.


An [outward probe](references/cone-text-239-capacity-2026-11-20/balanced-chord-probe.json)
chose the projection from the independently scaled chords of the two sides
and kept the same exact inequalities. It passed the join in region 2001,
then refused the same derivative condition at curves 13009 and 13010 in
region 13001. The direction heuristic and its test executable were removed.
The operation still needs a proof that covers both joins under one policy.


## Adjacent and nonincident sides

The family check projects the actual rational control rows. It chooses an
axis from the shortest angular interval containing the oriented control
steps projected perpendicular to the shared ruling. This floating-point
choice only supplies an axis. Outward interval arithmetic decides the sign
and margin of every required bound.

Let `d` be the minimum positive projected control step over both rows,
`n` the curve degree, and `w_min` and `w_max` bounds on the common positive
weights in the rows. The projected derivative is bounded below by
`n * d * (w_min / w_max)^2`. In the derivative numerator, the pair of
indices `i < j` contributes a positive weight product times
`(j - i) * (p_j - p_i)`. Consecutive differences at least `d` bound this
term below by `d * (j - i)^2` before weighting. The sum of the unweighted
Bernstein pair terms equals `n * u * (1 - u)` by the binomial variance
identity. The same bound applies at the endpoints by continuity. Common
weights across depth make each ruling a convex average of the matching
row differences. The check retains the existing coupled injectivity
inequalities and exact shared-ruling binding.

The [whole-operation run](references/cone-text-239-capacity-2026-11-20/retained-adjacency-whole-text.json)
passed every adjacency in both normal senses. It reached a nonincident
side pair whose complete control boxes overlap in region 9001, curves
9016 and 9018. The legacy hierarchy stops at such a leaf.

FamilyV3 can subdivide that product in the existing hierarchy. It bisects
one of the four patch parameters per step, in a fixed order, using
homogeneous de Casteljau subdivision along each rational row and
homogeneous averaging across the linear ruling. A node is excluded only
when its two outward control hulls are strictly disjoint. Both children
cover the parent domain, including the midpoint. A product with any
unresolved leaf at depth 24 refuses. The existing one-million arithmetic
ceiling for this primitive still bounds the work, and arithmetic polls
retain cancellation. The result counts each fully excluded side pair once.


[Both full-text runs](references/cone-text-239-capacity-2026-11-20/subpatch-separation-whole-text.json)
passed the side coverage with subdivision. They refused the cap pair in
regions 1001 and 10001 because the cross-member check required disjoint
phase intervals. The operation returned no body. The original one-million
aggregate refusal control still passed.


## Cap separation

All admitted cap carriers use the common cone axis and semi-angle. In this
frame the slant coordinate of the retained cap is

`r*sin(alpha) + z*cos(alpha) = r0*sin(alpha) + z0*cos(alpha) + v`.

The cap bounds this coordinate outward over the complete filled UV rectangle.
FamilyV3 first checks strict separation along slant or normal depth for
different members, then uses the existing phase test. Equality of physical
points would imply equality of each coordinate, so one strict gap suffices.
The same-member depth check and the family angular-width check remain in
force. Overlapping envelopes still refuse. Arithmetic and cancellation
refusals from side subdivision retain both native side identities.


## Whole-operation result

The [whole-text qualification](references/cone-text-239-capacity-2026-11-20/full-text-qualification.json)
passed both normal senses and the original fixed-allowance refusal control.
The test checks all 18 region reports, including exact side and cap
self-intersection coverage, sampled native geometry, analytic endpoint
positions, independent authored-volume bounds, exported history ownership
and input isolation. Both runs used the original `1e-5` spatial limit and
1017 integration subintervals. Inward used 3,071,263 arithmetic operations
and outward used 3,057,275, out of the requested 8,810,496.

The diagnostic call times were 13.86 s inward and 13.97 s outward. These
are single observations from the assertion-enabled build. No matched
runtime or peak-memory comparison was run. The fixture uses one positive-X
chart and a canonical cone frame. Wider and periodic profiles, edited
support or placement, and attached-body replay still have separate gates.

## Text on a selected support solid

Two TKV3D controls use the same owned 239-curve fixture and the public
`ConeFaceAttachment::apply_wrapped_profile` entry. The support is a native
frustum with an eighth-radian semi-angle and a full turn around its axis.
The chart seam is 2.5 radians from the support's angular datum. The profile
stays within the slant ends and clear of the paired native seam.
Both calls keep the original spatial limit of `1e-5`, depth range
`[0, 0.25]`, and explicit 8,810,496-operation request.

Both normal directions passed. Fuse produced a boss and Cut produced a
pocket, each as one validated solid. The detached history contains a face
image for every selected last-depth cap. The controls check the original
authored-volume bounds and the direction of the Boolean volume change
using native quadrature. The captured support record and the request
compare equal before and after each call.

The assertion-enabled Linux x86_64 build took 136.89 s for the attached
boss and 139.74 s for the pocket. The corresponding standalone calls in
the same execution took 13.65 s and 13.80 s. The API timers exclude setup,
compilation and test assertions. These are single observations on the
shared host. They establish a starting point for attached-text
performance. The 5.4 s target applies to the separate standalone `O.`
fixture.

A separate stage probe passed the boss with a 140.06 s API time using the
same fixture. It recorded the following wall times.

| Stage | Seconds |
| --- | ---: |
| Resolve and admit the selected support | <0.01 |
| Extrude, place and trim the tool | 63.80 |
| Export the tool | 0.06 |
| Run native Fuse | 16.13 |
| Compose history | 0.02 |
| Encode and decode the result | 0.15 |
| Validate the decoded solid | 59.77 |
| Check byte-for-byte archive replay | 0.07 |

Tool construction and final body validation account for about 88% of
this run. The tool timer includes the canonical extrusion and the
placed tool's own native validation. A separate 12-second CPU sample
of the pocket captured 2,356 leaf samples and none in the repeated
polygon segment-box calculation. That window leaves its share across
the whole call unresolved. The timers and transient profiles were
removed after collection. Full multiline determinism across processes
and matched peak-memory measurements remain open.
