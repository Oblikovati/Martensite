# Planar qualification of wrapped profiles (#418)

Status, 2026-11-14: authored material capture is integrated into the private
pipeline. Quantitative clearance, derivative margins and deformation topology
remain open. The operation still returns `Unqualified`.

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

Reuse the adaptive-axis hull-separation approach in
`bezier_pair/hull_separation.rs`. Generalize its coefficient-count handling in
a separate structural change if needed. Cartesian axes are cheap first
candidates. Edge normals of the derivative hull can establish a strict cone
when a fixed Cartesian direction fails. Every candidate requires an outward
projection proof; choosing an axis is not acceptance evidence.

For clearance, a projection gap at least `delta * ||n||_1` implies Euclidean
separation of at least `delta`. Use certified expanded AABBs for broad-phase
pruning, then adaptive hull bounds for unresolved pairs. Cache subdivision
results by original source and exact interval, and check point witnesses before
spending a refinement budget. Record work and incomplete causes separately
from source identity.

The retained [Jacobian oracle](references/rational-join-cone-oracle.json)
checks the mixed fixture's right quadratic and upper cubic derivative controls
against `n=(-1,3/4)`. Their exact projections are
`1/8, 17/8, 21/8, 3, 3/8`, with minimum `1/8`. Bernstein convexity therefore
gives a strict common cone for these original curves. This is a feasibility
example for the proposed local check, not a topology certificate for the body.

Required controls for the remaining implementation include narrow gaps,
interior stationary points, tangencies, exhausted refinement, periodic images,
shared endpoint motion, and rejection of mismatched O2 source intervals.

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
