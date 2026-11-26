# Sketch3D source and generator evidence

The native fixtures use owned geometry and name the input occurrences directly.
The desktop scenarios remain in Oblikovati. See the
[implementation plan](IMPLEMENTATION-PLAN.md) and
[earlier kernel evidence](IMPLEMENTATION-EVIDENCE.md).

## E38. Work-plane Section

TKBO sections a supported finite planar face against a named work plane. The
operation checks the actual stored frames and the original datums of each
placement chain before classifying the intersection. A complete empty result
has its own outcome. Coplanar intervals and unsupported trims have typed
refusals.

The owned block and oblique square cuts verify each ancestor, the original
parameter charts, stored pcurves, and boundary-edge and vertex lineage. The
composed-datum control refuses a contradictory original transform even when
the resulting map would be valid. Fifteen native controls passed
ThreadSanitizer. The combined Section and continuity stage passed 4,872
tests and strict Rust 1.98.1 Clippy, with eleven tests marked ignored.

## E39. Directed endpoint continuity

Tangent and G2 relations join a line to an outgoing open control cubic or join
two control cubics at one authored endpoint. The stored branch uses directed
first derivatives. G2 also compares curvature in model units with its own
limit. The solver uses independent derivative equations and checks the
retained curves through the exact endpoint evaluator before adoption.

The native fixtures cover free and fixed reference controls, shared endpoint
identity, the reversed branch, cancellation, and direct first- and
second-derivative expectations. The fit-cubic solver layout has its own
admission gate.

## E40. Point On Face and straight Curve On Face

A Point On Face relation retains the source occurrence and revision, its input
reservation, and two original UV coordinates. The solver adds three spatial
rows. Publication separately certifies the projected world point and the
proposed UV lift against the actual finite trim, and reports each achieved
distance. The owned square-hole fixture accepts the rim, refuses the hole by
one ULP, and resumes the same gesture at a valid point.

For a straight Curve On Face, the exact trim check covers both affine paths
throughout the common parameter interval. A crossing into the open hole is
refused even if both endpoints are on material. Travel along a rim and a lone
vertex touch are admissible. The exported native edge owns its degree-one 3D
curve and pcurve on a detached original support chart. The verifier reads back
both actual control nets and bounds their distance across the span.

The combined feature fixtures covering the two operations passed 170 normal
toolkit tests, 21 lower math and native cases, ten ThreadSanitizer tests, and
strict Clippy. Independent archive replays matched bit for bit.

## E41. Split on a finite planar face

Split computes the exact intersection parameter of the directed source line
with the placed support. It classifies the exact trim point before rounding it
into the destination snapshot. The owned constant 1/3 control distinguishes a
true retained point from the adjacent rounded coordinate on a trim rim.
Outer-only and one-hole convex faces reuse the same captured trim and native
membership checks. Overlap, endpoint contacts, and trimmed-out intersections
retain distinct results.

The independent planar slice passed 218 toolkit tests, 22 lower exact Math
and native cases, and a concurrent recheck under ThreadSanitizer.

## E42. Axial line projection on a trimmed cylinder

TKBO projects a supported axial line onto an actual retained cylinder wall
in forward, two-sided, closest, or surface-normal mode. The operation builds
the original finite planar sweep and verifies the native Section result
against both independent source charts. It retains the original source
intervals, target face, paired seam uses, and hole rim identities. Closest
can return incomplete where the nearest stationary foot is inside a hole.

Across three fresh Rust 1.98.1 release processes, each with 100 calls of the
owned holed Normal fixture, p95 was 6.355–6.707 ms and process peak RSS was
19.5–19.8 MiB. Kernel debug and release produced identical geometry and
lineage receipts. Seven native tests, 1,452 lower tests, strict Clippy and the
native ThreadSanitizer run passed.

## E43. Finite helices

The checked TKHelix entry point accepts cylindrical and conical helices from
each supported pair of pitch, height and turns. Positive axial rise and
rotation sense are independent. The retained native spline has a certified
whole-domain position error against the exact authored pair and has positive
axial progress. Verification reads its actual native trim, terminal vertices
and stored tolerances. Existing legacy derivative replay remains recorded
under [#614](https://github.com/Oblikovati/Martensite/issues/614).

The 1.98.1 Math and Helix tests passed 4,996 executions. Eight pinned
ThreadSanitizer controls and strict Clippy also passed. Diagnostic one-turn
native fits took 8.7–10.1 ms on the owned cases and certified a position
bound below 7.85e-8 model units.

## E44. Ordered pitch tables and flat-end laws

The analytic sequence keeps caller-supplied row identities and rejects
overlapping authored zones by exact dyadic comparison. Ordered pitch rows
have continuous positions and retain the one-sided axial derivative at each
join. The owned flat-end curve uses constant-elevation end arcs and the
integral of the smoothstep axial velocity through both ramps. The native
sequence export has a separate receipt.

## E45. Normalized Sketch3D imports

A single checked import transaction accepts ordered point rows, a directed
polyline, explicit drawing line uses, or a four-station fit cubic. The caller
normalizes coordinates to model units and supplies reserved point and curve
IDs. Output lineage retains input row and curve order, including shared
drawing endpoints. The ordinary Snapshot checks govern atomic geometry
admission and cancellation.

The owned millimetre fixture retains coordinates (0,0,0), (4,0,0), and
(4,3,0) and both directed segments have total length 7. The fit fixture
checks the known cubic independently. Nonfinite coordinates, duplicate
identities, an endpoint missing from the import and a cancelled final check
leave the source snapshot untouched. The five focused tests and strict
Rust 1.98.1 Clippy passed.

The combined source stage passed 786 focused tests and strict Rust 1.98.1
Clippy across the affected toolkits. Ten established tests were marked ignored.
