# Planar qualification of wrapped profiles (#418)

Status, 2026-11-14: authored material capture, local derivative cones and
nonadjacent developed-plane clearance are integrated into the private pipeline.
A certified true-period slab now covers every nonzero image for narrow
selections. Wider periodic classification and publication remain open. The
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
period bound. The prototype's angular admission restriction remains in force;
this checkpoint does not establish the full-wrap or general-cone cases.

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

Next, certify the true cylindrical period and cover every relevant translated
image before relaxing angular admission. Boundary separation alone does not
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
it does not prove material overlap. The current angular admission remains in
place. General image relations, periodic containment classification and a priori
multiple-chart fitting are still required for wider selections.

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
