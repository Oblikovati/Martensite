# Planar qualification of wrapped profiles (#418)

Status, 2026-11-14: authored material capture, local derivative cones and
nonadjacent developed-plane clearance are integrated into the private pipeline.
A certified true-period slab now covers every nonzero image for narrow
selections, and multiple-chart fitting supports long curves and parameter-seam
crossings. Complete boundary-image relations now cover connected multi-turn
profiles. Disconnected material classification and publication remain open. The
operation still returns `Unqualified`.

## Implemented material capture

The pipeline calls the existing `build_planar_regions` before fitting or native
wrapped construction. Original lines remain lines; quadratic and cubic source
polynomials become single-span clamped polynomial splines with knots zero and
one. Their poles are copied from the authored source. Prepared fit points and
split vertex labels are refused at this boundary.

The existing planar engine certifies contacts, branch order and region nesting.
It constructs all bounded faces, so an outer loop and an internal hole also
produce the separate face inside the hole. The wrapped operation selects its
material by the complete directed outer boundary and hole boundaries. Each
boundary is a cyclic sequence of authored curve IDs and traversal senses.
Canonical rotation and sorted hole identities make this selection independent
of loop start, hole input order and capture-local region IDs. Exactly one face
must match; an outside loop cannot silently become a hole.

The prototype request supplies directed outer and hole contours. This is an
internal representation, not a new restriction on the existing public sketch
APIs. General selection and disconnected material still belong to P4.

The reused planar builder can certify a valid region with a tangent return.
That result does not establish the positive join margin needed for the wrapped
homotopy. Additional exact-sign admission refuses a repeated endpoint pole
(zero parameter derivative) or collinear outward branches on the same ray.
A regular straight-through join remains admitted. These checks use the existing
exact `orient2d` predicate and coordinate comparisons; they apply no metric
tolerance to the sign decision.

The retained planar area keeps the builder's `RepresentationEnclosure` label
for spline boundaries. Its bound includes a representation estimate. It cannot
serve as the final certified represented-volume evidence.

## Moving-endpoint homotopy to implement

The original review used an exact endpoint-interpolation premise. Stored
binary64 endpoints generally move from the authored real-valued cylindrical
map. The implemented incidence check instead establishes that adjacent
reference profiles share the same moved endpoint. The following planar
argument permits that motion without reviving the archived seam/collar work.

Let `A_i(u)` be one original developed polynomial restricted to its exact
canonical interval. Let `B_i(u)` be the retained first-depth curve's XY
direction normalized onto the support cylinder, lifted to the authored chart,
with its retained height. O2 bounds `|B_i-A_i|_1` and
`|B_i'-A_i'|_1` over the piece's local parameter. All adjacent `B_i` endpoints
must agree, as checked after decoding. Use `H_i(u,t)=A_i(u)+t(B_i(u)-A_i(u))`.

For each piece, find a nonzero direction `n` with a certified derivative hull
projection `n dot A_i' > ||n||_infinity * eta_i`, where `eta_i` is that piece's
O2 derivative-error bound. The projection of every `H_i` is then strictly
monotone, which proves regularity and excludes a self-contact within the piece.

For each adjacent pair, establish the same strict projection condition for
both curves in their traversal directions using one common `n`. Relative to
their shared moving endpoint, the incoming curve lies strictly on one side of
the perpendicular line and the outgoing curve on the other. They meet only at
that endpoint for every `t`. This requires full derivative hull bounds, not
just endpoint tangent tests.

For nonadjacent pieces, certify original clearance `delta >= 4*epsilon`.
The two position-error bounds, each at most `epsilon/4`, leave positive
separation throughout the straight homotopy. These exclusions together prove
that each boundary remains embedded and that disjoint boundary components stay
disjoint. With the existing source nesting certificate, continuity preserves
material topology and orientation.

This argument is conditional until the implementation establishes every
stated bound, including periodic chart injectivity. Refinement for proof
hulls must not rename or change the canonical geometric intervals. Cases
without sufficient margins or exhausted work must refuse.

## Candidate directions and verification

The cone certificate reuses hull-normal generation from
`bezier_pair/hull_separation.rs`, generalized to variable control counts in
structural commit `dd63ce9c`. Shared Bernstein capture, dyadic restriction and
derivatives were extracted in the same commit. All 437 math library tests,
strict clippy and the native loop-with-hole control passed after that refactor.
The native control took 3.29 seconds.

Cartesian axes are tried before generating hull directions. If they fail,
normalized directions from the derivative control hull supply candidates.
Every candidate requires an outward projection proof; choosing an axis is not
acceptance evidence.

For clearance, a positive projection gap `g` with
`g^2 >= delta^2 * ||n||_2^2` proves Euclidean separation of at least `delta`.
The implementation checks this inequality with outward dyadic bounds, without
a square-root estimate. Certified control-box hierarchies prune groups of
pairs; unresolved polynomial domains use adaptive hull bounds. Subdivision
results are cached per original source and exact interval. Point witnesses can
refute the margin before extending a cover. Work and incomplete causes remain
separate from source identity.

The retained [Jacobian oracle](references/rational-join-cone-oracle.json)
checks the mixed fixture's right quadratic and upper cubic derivative controls
against `n=(-1,3/4)`. Their exact projections are
`1/8, 17/8, 21/8, 3, 3/8`, with minimum `1/8`. Bernstein convexity therefore
gives a strict common cone for these original curves. This is a feasibility
example for the proposed local check, not a topology certificate for the body.

Required controls for the remaining implementation include narrow gaps,
interior stationary points, tangencies, exhausted refinement, periodic images,
shared endpoint motion, and rejection of mismatched O2 source intervals.

## Implemented local cone certificate

`certify_cylindrical_cone` admits one piece or two consecutive pieces. Every
source must match the original polynomial, exact interval, support radius and
angular origin retained by its O2 certificate. Paired sources must declare the
same radius/origin chart. The report retains the O2 source bits and the
physical developed derivative-error bound used in the calculation.

For each original restricted derivative control `d`, the certificate computes
an outward lower bound on `n dot d - ||n||_infinity * eta`. All source bounds
must be strictly positive. Each reported margin is an exact dyadic lower bound;
no binary64 conversion can round a small positive margin to zero. The chosen
finite direction, requested arithmetic settings, interval-operation count and
number of attempted directions accompany the report. Exhaustion or an
unresolved cone refuses without changing geometric subdivisions or span IDs.

One positive cone excludes a self-contact within its piece throughout the
straight homotopy. A common cone for two pieces excludes a second adjacent
contact when they share one endpoint in the same developed chart. Native shared
vertex identity and pole agreement were established earlier. Their original
authored endpoints also agree, so O2's unique nearby angular lifts select the
same endpoint branch. These local facts still require the nonadjacent-clearance
proof before they establish topology for the complete profile.

The private pipeline checks one cone for each prepared span and one for each
adjacent pair, using maps keyed by authored curve and canonical interval. The
19-piece fixture passes all 19 regularity cones and 19 adjacent-pair cones.
Missing proofs, relabelled intervals and changed authored coefficients refuse.
The cone code neither recaptures mutable native objects nor repeats the O2
trigonometric calculation.

Closed-form controls cover unit generator margins, subtraction of a `1/8`
height derivative error, local interval scaling, consecutive generators,
opposite branches and an interior stationary point. An oblique quadratic
control has zero margin on both Cartesian axes but a positive margin along
`(1,1)` after subtracting its retained O2 error. Source/chart mismatch,
precision/work limits, cancellation and thread-safe report types have controls.
The complete profile and body remain unqualified.

## Current evidence

The package run passed 30 sketch library tests and 32 integration tests. Ten
planar controls passed, including loop-start and hole-order invariance,
undeclared crossing, stationary endpoint and tangent-return refusals, and
regular straight-through admission. The final focused run passed eleven planar
controls, including refusal of a nonoriginal interval hidden behind an authored
curve label, and preservation of the area's estimated evidence label. Strict
all-target clippy passed. The retained approximate metrics script found no
wrapped-profile file over 500 lines, function over 20 code lines or nesting
above two.

The isolated debug mixed loop-with-hole operation, now including planar
capture, took 3.43 seconds and peak RSS 51,100 KiB. The five-second walking-
skeleton limit is unchanged. Scale and oblique-frame measurements remain P5.

## Local cone verification checkpoint

The joint package run passed 3,114 math integration tests, 56 math doctests,
34 sketch library tests and 32 sketch integration tests. The final math library run passed all 449 tests, including twelve cone
controls and the two-source cap before candidate generation. Strict all-target
clippy passed. All 19 piece cones and 19 adjacent-pair cones in the native
fixture have positive exact dyadic margins. Three native composition controls
refuse missing O2 evidence, relabelled intervals and changed authored poles.

The isolated debug fixture with local cone certification took 3.45 seconds
and peak RSS 51,028 KiB. This remains inside the five-second walking-skeleton
limit. The approximate metrics check found no cone or wrapped-profile source
file over 500 lines, function over 20 code lines or nesting above two. The full
PR gate must still run before pushing; these package checks do not replace it.

## Implemented nonadjacent clearance

`certify_cylindrical_clearance` checks original polynomials before wrapped native
construction. Each source names its original controls and exact canonical
interval. The request retains the common support radius and angular origin for
later O2 binding. Original inputs are validated directly, so this stage does not
need a constructed native curve.

The within-family domain covers every unordered distinct pair except explicitly
listed increasing unique pairs. The wrapped pipeline excludes exactly its
cyclic adjacent pairs, which have separate cone proofs. A declared empty
relation completes with zero domain and arithmetic work; it is distinct from a
failed or incomplete proof. The translated-family domain covers all ordered
pairs, including a source against its own translated copy. Its supplied binary64
translation is exact input; uncertainty in a true cylindrical period remains a
separate obligation.

A hierarchy of certified original control boxes covers the complete requested
relation without materializing its pair matrix. Bounding boxes are formed from
outward restrictions of the original controls. Median splits and candidate
axes choose traversal only; every acceptance uses a certified bound. Remaining
curve pairs use squared projection tests. Original-parameter point witnesses
refuse a margin that is strictly too large. Witness parameters remain exact
dyadics even beyond binary64 endpoint precision.

Curve subdivisions and midpoint samples are cached below each root and reused
against other curves. Domains, cached nodes, local refinement level and interval
operations have explicit refused limits. Proof subdivision never changes the
fitted geometry, canonical span identities or history.

The private pipeline requires `delta >= 4*epsilon` for all 152 nonadjacent pairs
of the 19-piece fixture. It checks this before wrapped construction, then binds
the original controls, exact intervals, radius and chart to the retained O2
certificates after decoding. A changed source or interval cannot reuse the
proof. A hole only `1e-6` from the outer boundary refuses at the clearance stage
with requested `epsilon=1e-5`, before wrapped native construction.

Independent [Jacobian squared-distance oracles](references/rational-clearance-oracle.json)
give `25` for displacement `(3,4)` and `1/2` for `(1/2,-1/2)`. The first control
proves the exact clearance `5`; the second exercises both an accepted smaller
margin and a refuted larger one. A 64-curve control covers all 2,016 pairs by
hierarchy pruning with fewer than 300 domain visits and no polynomial pair
inspection. A quadratic arch against two separated lines computes one dyadic
split and retains five curve nodes, confirming reuse across opposing pairs.

The developed-plane homotopy now has source nesting, shared endpoint agreement,
positive piece/join margins and nonadjacent clearance for this fixture. Before
publication, cylinder periodic images must also be separated under a certified
period bound. The later slab and multiple-chart checkpoints below determine current periodic
admission. General periodic classification and general-cone cases remain open.

## Clearance verification checkpoint

The joint package run passed 470 math library tests, 3,114 math integration
tests, 56 math doctests, 37 sketch library tests and 32 sketch integration
tests. Twenty-one focused clearance controls cover hierarchy pruning, adaptive
axis separation, cache reuse, source/chart binding, exact witness parameters,
empty relations and unchanged resource refusals. Strict all-target clippy
passed for both packages.

The isolated debug fixture took 3.43 seconds and peak RSS 52,524 KiB, including
preconstruction clearance and its later O2 binding. The five-second limit is
unchanged. The approximate metrics script found no clearance or wrapped-profile
file over 500 lines, function over 20 code lines or nesting above two.

The periodic slab checkpoint below covers every image for narrow selections.
General image relations remain open for wider sources. Boundary separation alone does not
classify periodic material for disconnected selections: a small translated
component can lie strictly inside a larger selected component without touching
its boundary. P4 must prove material disjointness as well, reusing planar nesting
or a stronger certified global-box exclusion. No version change is authorized.


## Periodic slab checkpoint

The private pipeline now certifies an enclosing developed slab before native
construction. It reuses the outward Machin π bound from trigonometric argument
reduction. Exact original dyadic restrictions provide a control-hull width
upper bound `W`; interval multiplication encloses the true circumference
`T = 2πR`. The proof requires `T_lower - W >= 4ε`, with the same spatial
clearance as ordinary pair separation.

For every nonzero integer image `k`, the slab gap is at least
`|k| T_lower - W >= T_lower - W`. This covers both signs and all image numbers
without a finite sampling assumption or a rounded period. A bounded planar
region enclosed by these sources lies in the slab too, so the proof also
excludes periodic containment of disconnected material in this case. The
certificate retains the original source proof and binds back to the later O2
inputs and chart. Slab width, circumference interval, achieved separation and
arithmetic cost remain separate measurements.

This additional proof uses a fresh finite arithmetic allowance from the original
policy. Exhaustion, cancellation and insufficient precision return incomplete
evidence. A slab wider than the permitted bound also returns incomplete evidence;
it does not prove material overlap. The multiple-chart checkpoint below removes the initial angular admission
restriction. The complete periodic checkpoint below handles wider connected selections.
Disconnected material containment remains open.

Controls distinguish binary64 `TAU` from the true circumference, restrict the
original source interval, preserve a large chart translation, refuse the
possible disconnected-containment case, and retain resource/cancellation
refusals. A radius of `1e-40` refuses at 128 fractional bits and passes at 256
bits under the unchanged margin. Independent exact-rational alternating sums
check the extracted Machin enclosure at 8, 64, 128 and 256 bits; the original
trigonometric enclosure control also passes.

Verification passed 482 math library tests, 3,114 math integration tests,
56 math doctests, 37 sketch library tests and 32 sketch integration tests.
The nine periodic slab controls passed again after the final test-helper edit;
strict all-target clippy passed for both packages. The isolated debug
loop-with-hole fixture took 3.51 seconds with peak RSS 51,940 KiB, within the
unchanged five-second limit. These individual timings do not establish a
performance improvement. The approximate style scan found all inspected files
under 500 lines, functions at most 20 code lines and nesting at most two.


## Multiple-chart fitting checkpoint

`plan_cylindrical_charts` uses the original angular polynomial to bound speed
with respect to its declared source interval. It chooses a uniform dyadic chart
level `L` such that `max|θ′| / 2^L <= 1`. The mean-value bound then limits every
chart's angular variation to one radian. Each chart receives an outward midpoint
center from its exact original control hull and the existing fourth-derivative
Hermite sizing calculation. This is a priori geometry selection; conformance
proof results never trigger a different fit.

Chart selection and every local fit share one finite 256-bit arithmetic
allowance. The requested geometric level includes both chart and fit
subdivisions. Exhausting either allowance refuses the complete plan. The result
records total work separately from each local fit's work. Original intervals
remain exact, including when the requested source was already a dyadic
subinterval. Increasing the work allowance leaves selected geometry unchanged.

Native preparation splits source controls along those canonical intervals,
keeps split-corner ancestry, assigns the chart center and performs the local fit
subdivisions. Every retained O2 proof still compares with the original authored
polynomial at the complete chart-plus-fit interval. The existing mixed fixture
keeps its 19 pieces and their subdivision levels. A single-chart control also
keeps the earlier center, ideal bound and local work receipt.

The private pipeline admits finite angular coordinates in place of the initial
`|s/R| <= 1.5` restriction. Required ordinary and periodic certificates still run
before native construction. Six-radian rectangles at angular intervals `[0, 6]`
and `[6, 12]` exercise long curves and the parameter seam. Their independent ideal
volume is `6 * (1.2² - 0.9²) / 2 = 1.89`; the native volume estimate is checked
within the unchanged requested `1e-3` tolerance. A 6.4-radian selection refuses
at the clearance stage because the slab cannot establish periodic separation.
This refusal remains incomplete evidence, not an overlap classification.

Planner controls cover a long line, an exact source subinterval, original cubic
chart centers, single-chart preservation, work exhaustion, cancellation and
unchanged geometry under a larger work allowance. The complete periodic checkpoint below covers wider connected profiles even
when their enclosing slabs overlap. Disconnected material classification and
publication remain open.


Verification passed the two package suites (488 math library tests before the
last two controls, 3,114 math integration tests, 56 math doctests, 42 sketch
library tests and 32 sketch integration tests). The final math library run
passed all 490 tests, including all eight chart-planning controls. The five
native subdivision controls passed again after the final helper extraction.
Strict all-target clippy passed for both packages. The isolated original debug
fixture took 3.36 seconds with peak RSS 51,632 KiB; the five-second limit remains
unchanged. The approximate style scan found no inspected file over 500 lines,
function over 20 code lines or nesting above two. No version was changed.


## Complete periodic boundary relations

The private pipeline now uses `certify_periodic_clearance`. It derives a finite
set of positive image numbers from the original restricted control hulls and an
outward true-circumference interval. Let `W` be the enclosing developed S width,
`T_lower > 0` the circumference lower bound and `δ = 4ε` the requested clearance.
The planner examines every image `1..=N`, where
`N = floor((W + δ)_upper / T_lower)`. For every integer `k > N`, the slab gap
`k T_lower - W` already exceeds `δ`. The recorded first tail image and its lower
gap certify this infinite remainder. The result also records the requested
positive-image allowance. A narrow slab has `N = 0` and completes
without pair queries. The native limit is 16 examined images; the math interface
admits an explicit maximum up to 1,024 and refuses if required images exceed it.

For each examined image, the planner encloses the true shift `k * 2πR`, selects
one finite binary64 literal shift and records an outward translation error `σ`.
The literal relation must establish at least an upward-rounded `δ + σ` clearance.
The triangle inequality then gives physical clearance `δ` at the true shift.
Requested physical margin, effective comparison margin and translation error
are recorded separately. A literal witness below its strengthened threshold
refuses certification; it does not by itself prove a collision at the true shift.

Every examined relation contains all ordered source pairs, including a source
against its own translate. This also covers negative images by swapping the
ordered pair. No endpoint or same-contour exclusion applies across images.
Sources come from immutable original snapshots and retain their exact authored
intervals; the complete result binds to the retained O2 inputs and chart.
Planning and all image relations share one arithmetic, domain and cached-node
allowance. Each image receipt records the effective remaining allowance.
Exhaustion or cancellation returns no partial successful proof.

## Connected material and the periodic proof

The current planar selection requires exactly one bounded region with one
Jordan outer boundary and optional holes. Consider its bounded outer domain
`D` and a nonzero translation `D + kT`. The periodic boundary proof separates
their outer boundaries. By Jordan separation, overlapping interiors would
therefore force strict containment of one domain in the other. Translation
preserves their equal finite area, while strict containment with positive
boundary separation leaves an open area gap. Thus containment is impossible,
and the outer domains, including their selected material, are disjoint.
The same argument applies throughout a Jordan-preserving profile homotopy
whose displacement stays below the established boundary clearance.

This argument uses the existing single-outer selection. It does not discharge
cross-component containment for disconnected selections. The [periodic material gate](PERIODIC-MATERIAL.md) now classifies those
relations explicitly for original disconnected selections. Frozen-family
construction and retained-profile binding remain required. The math report
therefore promises boundary clearance only; it does not infer material topology.

The native positive control is the developed parallelogram with vertices
`(0,0)`, `(14,14)`, `(14,14.5)`, `(0,0.5)`. Its S width spans two full cylinder
turns and its enclosing slabs overlap. Two explicit image relations and the
certified tail pass. An independent Jacobian exact polygon calculation gives
area `7`; the retained request/result is in
[the helical oracle](references/rational-periodic-helical-oracle.json).
At support radius `1` and depths `[-0.1, 0.2]`, the independent ideal volume is
`7 * ((6/5)^2 - (9/10)^2) / 2 = 441/200 = 2.205`. The native estimate agrees within
the requested `1e-2` tolerance. Its isolated debug control took 4.66 seconds.
The estimate remains separate from the missing certified represented measure.

Controls also cover every required image and the infinite tail, an empty image
set, exact source subintervals, true/literal shift enclosure, the strengthened
comparison margin, forbidden image omission, same-source image contacts and
cancellation. Removing one unit from the total arithmetic, domain or cached-node
allowance refuses at the second image, demonstrating that the allowance is not
reset per image. Publication remains disabled.

Verification passed 498 math library tests, 3,114 math integration tests,
56 math doctests, 43 sketch library tests and 32 sketch integration tests.
All eight focused periodic-image controls and strict all-target clippy passed.
The original isolated debug fixture took 3.33 seconds with peak RSS 51,144 KiB,
within the unchanged five-second limit. These individual timings do not establish
a performance improvement. The approximate style scan found all inspected files
under 500 lines, functions at most 20 code lines and nesting at most two.
No dependency, version or release tag changed.
