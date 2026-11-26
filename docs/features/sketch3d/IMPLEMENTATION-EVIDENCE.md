# Sketch3D implementation evidence

See the [source and generator evidence](SOURCE-IMPLEMENTATION-EVIDENCE.md) for later continuity, attachment, projection and curve work.

The production receipts continue the earlier [design and experiment evidence](EVIDENCE.md).

## E15. Native point and line adoption

The `martensite-tksketch3d` value layer landed locally at `55184872`. It holds authored points and lines in
immutable snapshots and validates one atomic batch. The line admission threshold uses the exact squared distance
of the authored binary64 endpoints. Length estimates and the nominal support direction are separate. Unchanged
lines reuse admitted geometry only when their directed IDs and endpoint coordinate bits match. The app owns
identity retirement across revisions and undo. The kernel checks current IDs and reuse within a batch.

The 21 toolkit tests, 12 exact-comparison controls and six triple-product regressions passed under Rust `1.98.1`.
The full TKMath suite and both-package strict Clippy passed. The same toolkit tests passed under pinned TSan in
a clean Linux container. Four AB/BA processes produced identical ordered native receipts. The SHA-256 was
`59037d02840a2f540ff11d755904de3c445071769c32fe88608fba5e0407a315`.

Two optimized benchmark runs per version measured 512 whole-snapshot adopts after warm-up, with one changed point
and all other lines unchanged. At 1,024 lines, p95 fell from `416–423 µs` to `98 µs`. At 256 lines it fell from
`106–109 µs` to `23–24 µs`. The timer excluded app edits, rendering and retiring the previous snapshot.

## E16. Frozen source points

The station-to-station fixture captures `(0,20,30)` mm and connects it to `(100,25,50)` mm in the destination.
After moving and deleting the source, the destination line retains both endpoint IDs, the captured coordinates,
and the source document, sketch, incarnation, revision and point IDs. A direct move of the frozen endpoint
refuses the entire edit batch. Detaching it preserves its coordinate bits and incident lines. Deletion and
orphan pruning clear the captured provenance. The kernel takes the resolved capture from the app, which owns
source resolution and checks the source revision before capture.

The four focused cases passed with the existing 21 toolkit tests on Rust `1.98.1`. Strict toolkit Clippy
and an independent review passed.

## E17. Native constraints and bounded drag

The native point and line constraint slice at `dad1a659` supports Fix, release, coincidence, same- and
opposite-direction parallelism, and driving length. The checked transaction captures Fix anchors from the
current sketch and verifies the final geometry and hard relations before publishing. A frozen point
contributes its exact coordinate bits to a solve and keeps its source stamp. Preview uses a prepared
program with explicit size and residual work limits. A point drag on a sphere has only two reachable
coordinate directions. The soft target solve uses those directions while the hard length equation stays
in the hard system. An extreme finite-coordinate test covers the rounding bounds used by the parallel check.

Three independent warm release processes on Linux x86-64 and Rust `1.98.1` ran 100 updates per size on a
pinned Ryzen AI Max+ 395 core. The solver, whole-snapshot publication and independent relation checks
were inside the timer. All 1,500 updates passed.

| Coordinates | p95 range in ms |
| ---: | ---: |
| 15 | 0.041–0.059 |
| 30 | 0.076–0.077 |
| 60 | 0.140–0.143 |
| 120 | 0.266–0.278 |
| 240 | 0.545–0.587 |

Peak RSS was 7,352–7,536 KiB. At 240 coordinates, the conservative residual bound peaked at 30,692,352
of the allowed 64,000,000. There were 34 native tests and 103 solver tests, with three opt-in tests excluded
from those counts. The focused benchmark ran separately. Strict Clippy and dependency checks passed.
This fixture measures a native line chain. Desktop input, GPU completion and presentation have their own gates.

## E18. Ordered native line wires

The line-wire export at `dc13eeda` resolves directed occurrences by line ID, joins them by point ID and
checks authored segment contact with exact arithmetic within a caller-supplied pair budget. Native readback
verifies the occurrence order, orientation, vertex identities, trim ranges and stored `1e-7` tolerance. It
reports an outward bound on the whole affine trim relative to the authored segment. Authored incidence and
native trim deviation have separate checks. Native self-incidence and minimum clearance require their own proof.

The E10 nonplanar Sweep produced a valid shell at the expected stations and traced a face to its profile edge.
The E10 ruled Loft rebuilt after the second closed section moved. The two valid native solids had volumes 12
and 20 under the fixture's volume check. Both sections traced to output side faces, and later edits or native
vertex mutations left captured inputs and earlier exports intact. These controls use the production builders.
Full consumer validation, curved wires, incompatible sections and application-driven rebuild remain open.

All 35 toolkit tests passed on the wire lane. Independent re-review covered the late cancellation checks during
native point binding publication. Two fresh processes emitted identical 692-byte wire archives with SHA-256
`1ed6890c71ebe742d8e5b7e993d85dbccb6fbe3d73e311a5d8bbc62ae1fdba18`.

## E19. Authored circular arc placement

`731a1487` adds directed three-point and center-point arcs to the immutable spatial snapshot. The three-point
case checks the chord and witness altitude with exact predicates, constructs the nominal circle in scaled
local coordinates, and verifies that the witness is inside its trim. Center-point placement uses a supplied
normal and exact radial orientation. At a zero orientation, exact projected opposition admits a semicircle,
including an oblique frame whose rounded angle would otherwise move by one ULP. For each admitted arc, the
rounded carrier's placement samples fit within the caller's spatial limit. Authored coordinates remain
available independently, and the measured point deviation is a sampled estimate.

The toolkit fixtures cover both corpus placements, major and minor choices, non-XY and oblique semicircles,
very small and large coordinate scales, unrepresentable carriers, over-limit point fit, edit and deletion
closure, cancellation and signed-zero replay. The independent review found the oblique counterexample and
verified exact projected opposition on the corrected code. The point and line solver gives a typed refusal
for an arc layout. Arc equations, spline interaction, native curved wires and desktop authoring have separate
acceptance cases.

## E20. Construction participation

`0842c375` keeps spatial construction roles separate from the point and curve recipes. Changing a role
preserves geometry and identity. A line with the construction role can drive a length in the native solver.
New wire recipes refuse it with its line ID. A previously captured recipe still materializes its own
inputs. The tests also cover atomic cancellation, typed lookup, point-deletion closure and construction
arcs. Independent review, 64 Sketch3D tests and strict toolkit Clippy passed on Rust `1.98.1`.

## E21. Analytic planar spiral

The TKG3d spiral at `f54c64ac7` evaluates the finite radial law from the owned fixture. The radial pitch is
signed and the rotation sense is independent. Admission compares the exact authored turn law and the
rounded radian parameter law at their endpoints. Both have to keep a positive radius through the entire
interval. The curve evaluates native D0 through D3 and DN. For DN the four derivative phases repeat and
bound the work independently of derivative order.

The fixture checks the one-turn cardinal points, initial derivative, reversed rotation, oblique world
placement and an inward radius limit. Public JSON and failed-edit inputs verify that bad finite frames
refuse without changing valid oblique coefficients. All 16 focused tests passed on Rust `1.98.1` and under
pinned ThreadSanitizer. Strict Clippy passed. Two independent processes emitted the same evaluator bytes,
with SHA-256 `0732f1788cfa5599f2083429914deb02597ba94c7e8e1034fa1aca25a6563a40`.
The earlier full TKG3d run passed all 985 tests before the frame admission correction. BRep storage and
the desktop spiral command remain separate checks.

## E22. Checked rigid motion

`11a91acd` adds a checked edit for translating or rotating owned points in a constrained sketch. A shared
point moves once. Geometry admission and the hard-relation checks run on the full draft before an updated
sketch is published. The edit preserves anchors and captured source points, and prepares a new reference
frame for subsequent direction-constrained drag.

The rotation acts on offsets relative to its pivot. Exact axis points and zero-angle rotations keep their
original coordinates. The point-placement check compares the stored output with the exact translation or
with the native binary64 rotation matrix acting on exact offsets. It refuses error beyond the constraint
policy's length limit. Independent controls cover the owned translation and quarter turn, fixed and frozen
refusal, a large pivot, a lost translation unit at `2^53`, wrong selection and cancellation. The native
Parallel fixture also drags after a 90-degree rotation. The toolkit passed 74 tests on Rust `1.98.1`, plus
strict toolkit and TKMath Clippy. A separate package run exercised 1,269 TKMath library and 3,118 toolkit tests.

## E23. Checked relation replay

`b114f6ed` prepares stored geometry and typed relation values for warm drag. It checks relation identities,
operand types, dimensional limits and physical feasibility, then returns the original accepted coordinates.
The supplied Fix anchors are checked bit by bit. Restore uses no numerical solve. The preparation limit
applies before staging the relation list, and cancellation is checked before publication.

A frozen source can retain either a source incarnation and revision or a historical session generation.
The latter preserves the evidence available in an Oblikovati version 18 recipe. The restore fixture keeps
its frozen coordinate bits through a later warm drag. The other cases cover a signed-zero anchor mismatch,
contradictory saved length, malformed relations, size refusal and late cancellation. Independent review,
78 toolkit tests and strict Clippy passed on Rust `1.98.1`. Package and desktop adoption have their own gates.

## E24. Native parting silhouettes

`bb1b2a3a2` returns detached curves with caller-owned source occurrence IDs. The query checks that the
native root is a solid. It uses the original placed plane tangents for oriented face signs and recognizes
cylindrical walls only after checking their stored full UV trim and seam. Along a cylinder's pull axis it
returns both rims. For a radial pull, each returned line has an outward position bound over its whole
trim against the exact placed cylinder. The query compares that bound with the original face tolerance.
It rejects an unsupported or unproved contribution and distinguishes completed empty results.

The fixtures cover principal and diagonal box pulls, the cylinder seam, located geometry, planar and
cavity exclusions, a through-hole, invalid topology, cancellation and source isolation. Native validation
observes cancellation at call boundaries. Two processes produced identical receipt bytes with SHA-256
`7a1d8a15701e4793177cb929d4e92d1aed0ef3aa6ec518eda3434ae373726538`. On Rust `1.98.1`, all 20,361
TKHLR tests and 1,270 TKMath library tests passed. The 22 focused cases also passed pinned ThreadSanitizer.
Independent review and strict TKHLR and TKMath library Clippy passed. App command wiring remains open.

## E25. Ordinary extrusion from a spatial line wire

`tksketch::spatial_prism::extrude_wire` accepts one closed straight-line Sketch3D loop. Exact source
coplanarity precedes projection, and actual native geometry is checked against source XYZ. The ordinary
TKBO prism validates its result in the existing four stages. The adapter binds retained cap and side
trims and computes an outward volume enclosure from their oriented planar flux. Detaching the solid
also detaches the 26 history images in the oblique rectangle fixture.

The review regression requests a displacement halfway between stored coordinates at world X `2^27`.
The operation refuses the resulting cap-vertex trace under the original `1e-8` source limit. At the adjacent
exact displacement, the independent volume 45 lies in the enclosure. Both signs of the oblique sweep and
an oblique non-normal sweep also enclose their independent GProp masses. A cap-only mutation is refused
without a corresponding vertex or edge change.

At exact source `3b55f9ed`, 1,024 optimized kernel runs on Linux x86-64 and Rust `1.98.1` included the
four-line recipe, ordinary extrusion, native trim checks, volume, and detached history. Warm p50/p95 were
`10.038/11.862 ms`, with peak RSS `21,108 KiB`. For 40 independent processes the first operation p95 was
`11.982 ms` and process wall p95 `16.344 ms`. No samples refused. The host ran other builds. The same
fixture had boundary trace at most `2.032e-14` model units and an actual volume enclosure radius of
`7.532e-13` around 75. All 10 focused native cases, two volume unit tests, strict Clippy and the concurrent
detachment ThreadSanitizer control passed. Curved profiles and desktop timing have their own gates.

## E26. Directed line relations

`86b3a321` adds signed axis and plane parallelism, perpendicularity, equal length and driving angles.
Equal length compares exact authored squared lengths within a closed spatial limit. Angular feasibility
uses the original endpoint coordinates and checks the accepted branch. The Angle fixture covers radians
near both open interval endpoints, and centered finite differences check all five new native equation
families. A spin to a distant point on the same angular cone may defer while leaving the accepted sketch
available.

For flat perpendicular and plane-parallel seeds, cold initialization checks candidate moves against every
incident native segment and compiled residual domain. The connected regressions cover a collapsed
neighbor, an Angle singularity, shared endpoints, fixed and frozen inputs, and cancellation at every poll.
The operation still invokes the solver once. Saved values go through the same structural and physical checks
on restore, without moving the sketch.

The Rust `1.98.1` fixture ran 1,500 updates per revision across three matched processes on a loaded Linux
host. The final mixed-relation p95 at 240 coordinates was `0.597–0.607 ms`. The Angle p95 was
`0.183–0.196 ms`. Every update adopted and charged work and scaled error matched the previous revision.
The measured scope includes native geometry admission and independent relation checks. Root-to-display
drag and the remaining curve relations have their own gates.

## E27. Open control cubic

The first owned spatial control spline uses four point IDs in one clamped cubic. The first and last
poles retain endpoint identity, and the snapshot preserves construction participation and original
coordinate bits. Admission certifies nonzero speed over the full parameter interval. Point removal
cascades through incident curves and cancellation leaves the original snapshot available.

The exact evaluator forms fixed-degree dyadic Bernstein values and derivatives, then rounds each
requested coordinate once to binary64. Regression cases include opposing extreme poles, subnormal
parameters, endpoint tangent and acceleration cancellation, and the signed rounding overflow midpoint.
The admitted evaluator exposes D0 through D2. Curve constraints and native curved wires require their
own proofs.

At `59f5869a`, three independent optimized runs of 4,096 calls each measured `4.92–5.46 µs` p95 for the
ordinary spatial jet fixture and at most `6.60 µs` across the numerical controls. Peak process RSS was
`5.40 MiB`. This measures warmed evaluation of an admitted curve. Rust `1.98.1` passed 4,473 tests,
strict Clippy and bitwise replay. The concurrent rebuild/evaluation control passed pinned ThreadSanitizer.

## E28. Directed line split at a work plane

`ee5f619ef` classifies a directed line from exact signed distances to the work plane. A transverse
interior hit yields two ordered fragments. The original outer endpoint IDs and construction role
remain in the new snapshot. Both fragments share a caller-reserved cut point and carry the original
line and source interval. Endpoint touches and misses return the original sketch. An overlap is refused.

An exact normalized parameter has adjacent floating-point bounds, including a subnormal interval
whose rounded parameter would be an endpoint. Authored cut placement uses the position limit and a
certified outward error bound. The normal edit gate still requires a nonzero length for each fragment.
The controls cover the `x=0` split, reversed direction, an oblique plane, large coordinates, no-change
outcomes, identity collision and cancellation at every poll. Rust `1.98.1` passed 1,276 math unit tests,
3,118 math toolkit tests, 88 Sketch3D tests and strict Clippy. Two fresh processes produced identical
receipts with SHA-256 `7923e08494f11645d6913f7daafd5f5b6ffc332b7ed8e72db65ace8e0779dcd6`.

## E29. Work-plane mirror copies

`c2b9de8d` reflects points and directed lines across a work plane. The caller
reserves copy IDs. Shared endpoints, endpoint order and construction roles
survive the copy. The original retains any frozen source stamp and the copy
records its derivation from that point. The resulting line segments meet the
same position limit as their endpoints because reflection and segment
interpolation are affine.

The evaluator rounds each changed coordinate once from the exact rational
reflection. Moving the plane origin within the same plane preserves the result,
even when a tangential coordinate would overflow a local-frame conversion.
The result certifies the supplied bound for the authored geometry. The stored
native line carrier has its own export checks. Owned controls cover equivalent
plane origins, oblique and axis planes, signed rounding midpoints, overflow,
identity conflicts and cancellation.

Rust `1.98.1` passed 4,564 affected tests and strict Clippy, with three opt-in
tests skipped. All 17 mirror tests passed under pinned ThreadSanitizer. Two
processes produced identical replay receipts with SHA-256
`26710dc5856beb240319b1520e9b5cb14258a9dc6a4753e6d6cd40f0c1b9606c`.

## E30. Ruled solid loft from two line sections

`loft_ruled_pair` takes two closed planar line recipes with distinct caller section
IDs. The directed first occurrence anchors the correspondence. The input proof
requires a positive uniform scale between the ordered points in parallel planes.
The two planes must be distinct. The native ruled builder retains these section
uses, with cap, side and rail images tied back to both input recipes.

The result checks the solid in the required four stages. It encloses volume by
integrating the retained planar and bilinear trims and exports a detached shape
with its history. The independent slanted and scaled control has volume 28. An
upstream section edit changes the E10 control from volume 12 to 20. A raw pair
with a degenerate edge or conflicting traversal order is refused before the
native ruled history walk.

The original retained-output revision `279bf0e5` measured 116.939 ms p95 across
256 warmed operation-and-detachment samples on a loaded Linux host. Peak RSS
was 26,060 KiB. Thirty fresh processes measured 241.011 ms p95 for the first
kernel operation. The final input-guard revision passed all 145 legacy TKOffset
tests and strict Clippy. Rust `1.98.1` combined verification also passed the
seven Loft, ten prism, two direct adapter and one homothety tests. Desktop
rebuild and presentation require their own timings.

## E31. Point on line and directed colinearity

Point On Line uses two transverse rows and checks the final distance to the
authored line independently. Colinear uses four, two or zero rows for zero, one
or two shared endpoint IDs. Its stored direction branch is checked from the
actual endpoints. An exact squared-sine comparison resolves overlapping
outward bounds and admits equality at the caller's tolerance. A shared endpoint
supplies incidence directly, with no repeated equation.

An accepted pose can replace the local residual frame while keeping its
prepared layout. The replacement checks hard residual, rank and remaining
work before publication. In 7,200 accepted updates on a loaded host, frame
crossings measured `0.222–0.335 ms` p95 for Point On Line at 234 coordinates
and `0.440–0.489 ms` for Colinear at 240. Replay matched across processes. The
prepared-source and chart code passed concurrent replay under ThreadSanitizer.

## E32. Four fixed fit stations

Fit cubics retain station IDs and fixed parameters. A banded solve constructs
an affine interpolation operator once. Later edits reuse it, check chord
distortion and certify both regularity and the actual station miss of the
stored polynomial. One translated fixture has rounded D0 equal to the
authored station and an exact 32-unit miss. It accepts a cap of 32 and refuses
the next smaller binary64 value.

For the owned cubic, independent Lagrange derivatives and TKGeomAlgo
interpolation agreed with the accepted position and derivative fixtures. In
matched loaded runs, exact dyadic completion of the sensitivity weights sped
up the twelve-component jet by `9.36–14.81×` at p95. Each 1,179,696-byte
output stream matched byte for byte. The original fixture measured
`0.440–0.452 ms` p95 cold admission and `0.738–0.806 ms` for an accepted
station edit. Multi-span fitting and active fit-curve constraints require
their own support and fixtures.

## E33. Trim and Extend for straight lines

Trim uses the authored pick parameter to keep one side of a proper crossing.
Extend compares exact forward parameters across the supplied boundaries, including
collinear overlap intervals. It checks whether the crossing advances the
stored line after rounding. A literal boundary endpoint supplies the contact's
stored coordinate bits. Other contacts carry a certified placement bound.

The edit keeps the other endpoint and construction role, retires only the
selected source and its unused defining endpoint, and reports the retained
source interval separately from any extension. Shared and frozen endpoints
are protected. Replayed constraints keep their anchors and rank. A combined
fixture verified a shared fit station survives an accepted trim and protects
its endpoint when selected for replacement.

Rust `1.98.1` passed 4,735 combined TKMath, TKSketch3D and TKSolve tests and
strict Clippy, with seven opt-in tests skipped. Trim and Extend also passed
22 ThreadSanitizer controls with byte-identical replay.

## E34. Preferred cursor motion

The prepared solver can minimize world-space cursor error along the local
feasible directions. It reports whether the cursor target was reached or the
retained point was stationary in those directions. Before publishing, the
sketch checks its original hard relations, updates any incidence charts and
checks the retained values against the same work budget. A refused tick leaves
the last accepted preview available for the next sample.

The release benchmark replayed 6,480 accepted motions across exact and
preferred cursor paths on owned rail and circular fixtures. At 240 scalar
coordinates, off-model circular targets took 0.490–0.555 ms p95 and off-model
rail targets took 0.165–0.181 ms p95 across three loaded runs. Eight pinned
ThreadSanitizer controls passed. These timings cover the prepared kernel
operation.

## E35. Cardinal bends

`Snapshot::bend_lines` replaces the corner of two exclusive orthogonal
cardinal line legs with a tangent quarter arc. It keeps each far endpoint,
records the retained line intervals and attaches both original legs to the
arc's history. The exact radius, tangent directions and contact positions
have separate checks on the stored result. Overlong, shared or frozen corner
edits and unrepresentable contact points are refused before adoption.

The native bend and its separate math fixtures passed the focused tests,
ThreadSanitizer and deterministic replay on Rust 1.98.1.

## E36. Rounded Sweep from Sketch3D line wires

`sweep_rounded_shell` and `sweep_rounded_solid` consume a centered closed
rectangular section and an open path of three nonzero cardinal legs along
distinct axes. The sheet has two terminal loops. The solid adds planar caps
and qualifies the paired oriented edge uses across the closed shell. Each
result maps the retained path and section rows to the corresponding native
images after copy. The trace report records the measured bound independently
of the requested limit. The solid also bounds volume on the retained trims.

In the loaded Rust 1.98.1 release run, p95 was 70.110 ms for the sheet and
81.887 ms for the solid (96 operations each). Across 30 fresh processes per
body kind, the first native operation took 90.469 ms p95 for the sheet and
100.975 ms for the solid. The solid's maximum volume bound error was
`4.139e-13` in cubic fixture units, and maximum source trace bound was
`5.330e-15` in fixture units for both body kinds. The concurrent native
Sweep control passed ThreadSanitizer.

## E37. Native planar face attachment evidence

`capture_planar_attachment` binds one located planar face of an actual native
solid to its named source revision. It checks the original stored frame and
placement against the effective map and qualifies the native 3D curves and
pcurves of its outer trim and one convex hole. A query uses the original
affine chart to classify the projected point as material, a rim, hole
interior or outside. It reports a certified Euclidean distance bound. The
capture owns its evidence and refuses a different source revision.

Eight focused TKMath cases and twelve native TKPrim cases passed on Rust
1.98.1, with twelve native ThreadSanitizer controls. The combined E34–E37
stage passed 1,191 tests and strict Clippy, with 28 tests marked ignored.
