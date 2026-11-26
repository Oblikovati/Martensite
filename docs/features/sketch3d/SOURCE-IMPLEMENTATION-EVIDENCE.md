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
admission gate. An exact monotone derivative check can certify eligible
unweighted identity-mapped cubics before root isolation. On the owned
111-coordinate preferred-motion workload, three processes measured
2.21–2.54 ms p95 compared with 4.89–6.19 ms for the original route.
Geometry, solve results and deterministic work receipts agreed bit for bit.

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
integral of the smoothstep axial velocity through both ramps. The retained
native wire reads back each span and certifies bounds of `5.82e−8` model
units for the pitch table and `3.30e−7` for the flat-end fixture. The
exact Math and Helix suites passed 5,007 tests, nine ThreadSanitizer
controls and strict Clippy.

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

## E46. Equation curves

TKG3d retains Cartesian polynomial curves as detached, directed native Bézier
edges on the original parameter interval. For cylindrical and spherical
equations it retains an ordered chain of checked cubic spans. The interval
proof checks position error and regularity against the authored equation.
The trim, vertices and native curve controls are read back after export.

The independent fixture set covers rotated source frames, signed angular
motion, zero derivatives, spherical poles, cancellation and deterministic
native replay. The two exact stages passed 5,387 tests, 20 ThreadSanitizer
checks and strict Rust 1.98.1 Clippy.

## E47. Split on a finite cylindrical wall

The line split keeps each exact ordered quadratic root until it can classify
the selected native wall or aperture. A root on either paired seam use or on
a retained rim is a valid contact. A root which falls in the open hole is
reported separately. The supported wall is a full turn with an optional
rectangular UV hole strictly inside its source chart.

The owned one-third parameter case certifies the retained cut at
`(0, −1, 1)` with zero placement error. The other root on the opposite
side is trimmed out. Native checks distinguish disjoint, rejected and
incomplete outcomes. The leaf passed 242 Sketch3D tests, six Math cases,
35 legacy BRep captures, 24 ThreadSanitizer tests and strict Clippy.

## E48. Include a named straight edge

Include captures the selected native edge, its original bounded line and
both named endpoint occurrences. The authored spatial curve and its points
are read-only until detached. Rebind requires a newer revision of the same
source scope and reuses each owned endpoint ID. Geometry and lineage use
the original source parameter order.

The fixture compares against independently calculated endpoint coordinates
and straight length, and refuses coordinate-near substitute vertices and
unrepresentable placement. The exact leaf passed 229 Sketch3D tests,
1,484 lower tests, seven ThreadSanitizer checks and strict Clippy. Two
processes reproduced identical evidence bytes.

## E49. Radius, Equal Radius and Fix Arc

The checked solver layout keeps the authored start, end and midpoint witness
of a through arc together with its Minor or Major branch. The native
adoption check tests the actual retained circle and driving radius
independently. Fix retains all three exact anchors. Equal Radius shares
intrinsic rows only when their authored point identities agree.

The owned fixture drives a Major arc against an independent fixed Minor
circle. It checks rank 17, one degree of freedom, the original circle bits
and the expected sagitta. Center-authored arcs, off-midpoint witnesses and
exact semicircles have typed layout refusals. The leaf passed 4,694 tests
and strict Rust 1.98.1 Clippy. Six hundred accepted preferred motion samples
replayed bit for bit across processes, including three concurrent lanes.
The combined stage verifies that a fixed arc survives an independent planar
attachment drag and exact reopen and detach.

## E50. Mixed line and circular-arc wires

The first mixed wire export materializes the two retained straight legs and
the center-defined cardinal quarter of an admitted Bend. It uses one vertex
per shared authored endpoint, preserves directed native trim intervals and
keeps the arc center and source use identities in the history. Pairs need a
certified tangent contact or strictly separated complete bounds before
adoption.

The checked native trim is `[0, π/2]`, with forward analytic tangent
transport bounded by `2⁻⁵²`. A changed radius or active range failed independent
native readback. The leaf passed 234 Sketch3D toolkit tests, 1,340 library
tests, 27 existing Prism, Sweep and Loft tests, five ThreadSanitizer cases
and strict Clippy. Two independent processes produced identical history
and geometry bytes.

The combined native stage on Rust 1.98.1 passed 6,711 tests and strict
all-target Clippy across the affected toolkits. Ten opt-in cases were ignored.

## E51. Point On Face on a finite cylindrical wall

The attached sketch retains the named captured face, source revision,
input reservation and unwrapped original UV. The ordinary hard sketch
program adds three rows and two parameter coordinates with their own
scales. After each accepted move, the solver reanchors the angular
chart. Publication independently checks actual XYZ and UV against the
closed rims and bounds their Euclidean separation.

The owned full-wall fixtures cross the original paired seam in both
directions. They check both rims and adjacent outside points, an
ambiguous half turn, a coupled ordinary Coincident relation, stale source
and late cancellation. A translated source fixture checks frozen
ownership. Independent processes reproduced identical geometry and
trim evidence. The original leaf passed 244 toolkit and 26
ThreadSanitizer tests. A separate mixed-capability test verifies that
point qualification explicitly refuses a wall with an interior hole.
The line Split path still admits that hole independently.

## E52. Tangency to a four-station fit cubic

A directed line and an outgoing four-station fit cubic share an authored
terminal point. The two angular rows use the fixed station operator's
analytic sensitivities in the same local frame as line and control-cubic
tangency. The accepted geometry must also pass the independently rebuilt
native endpoint and jet check. Replay and rigid motion inspect the same
two warm residuals after the rounded native geometry check. The
large-origin fixtures exercise a case where the affine candidate and
retained native derivative differ.

The exact dyadic implementation evaluates station velocity and pole
materialization without repeated general rational reduction. Across
interleaved optimized runs with 512 timed updates each, idle p95 changed
from 3.65–4.09 ms to 1.77–2.24 ms. With a controlled load on the sibling
thread, it changed from 4.19–6.56 ms to 1.88–2.89 ms. All twelve runs
accepted 600 updates and produced the same geometry and work receipt.
The maximum charged work remained 742,656 scalar units.

## E53. Normal extrusion from a mixed quarter-arc profile

The checked extrusion path accepts a single analytic cardinal quarter arc
and straight uses that form a coplanar, closed tangent loop. It builds a
native solid in either normal direction and reads back the circular side,
cap trims, topology, input lineage and ordered validation. An independent
area bound on the retained mixed profile determines the volume bound.
The straight-only prism entry point retains its existing contract.

The owned 30-by-30 profile with a radius-5 corner and depth 3 has area
`875 + 25π/4` and volume `2625 + 75π/4`. Both directions retain 10
vertices, 15 edges, 7 faces and 32 history records. The controls check
invalid input, cancellation, source isolation and repeated native output
in independent processes with different legacy precision settings. The
seven focused ThreadSanitizer controls passed.

Three optimized Rust 1.98.1 processes each measured 64 checked warm
extrusions on Linux x86-64. The per-process p95 was 54.385–63.367 ms,
pooled p50 was 43.558 ms, and pooled p95 was 61.347 ms. First-call
times were 35.028–52.273 ms, with peak process RSS 22.9–23.3 MiB.
The SMT sibling was 71.7–89.1% busy. Timing covers kernel extrusion
from an already exported wire. Source construction, external receipt
checks and teardown occurred outside the sample.

The combined attachment and fit stage passed 5,154 tests and strict
Rust 1.98.1 Clippy. The subsequent optimized-fit and mixed-prism stage
passed 386 focused toolkit and modeling tests and strict all-target
Clippy. Both stages left nine opt-in cases skipped.

## E54. Four-station Curve On Face on a planar trim

The source recipe retains a captured face revision, four directed UV stations,
and the reservation for each new or reused sketch point. The same fixed
interpolation operator produces the 3D cubic and its pcurve. Native readback
checks every retained pole and the active domain. Both paths need a common
strictly monotone chart direction and independent whole-span inclusion in
the original convex outer trim with at most one convex hole.

The owned holed fixture proves both strict clearance and exact tangency to
the retained inner rim. Four admissible stations alone cannot establish
clearance. A checked rejection fixture enters the open hole strictly between
stations. A work limit, duplicate station, changed native pcurve or active
range, stale source, and late cancellation each produce their own refusal.
The isolated leaf passed 323 toolkit, 154 BRep, 16 Math and 28
ThreadSanitizer controls, plus strict Rust 1.98.1 Clippy.

## E55. Directed line-to-arc tangency

A named incoming line meets a Minor or Major midpoint-witness arc at one
shared endpoint. Two angular rows fit the outgoing tangent in a refreshed
local frame. Acceptance independently compares the exact authored tangent
and the derivative of the retained native circle at that endpoint, in the
same directed convention. A 54-bit component ratio fixture reaches the
exact authored direction but exceeds the requested sine limit after native
rounding. The solver reports the refusal and keeps the accepted snapshot.

The six owned controls cover both arc branches, reference-frame refresh,
replay, a wrong shared identity, and circular G2 incompatibility with a
straight line. Rust 1.98.1 passed 4,456 Math and 257 Sketch3D tests, with
nine opt-in cases skipped, plus strict all-target Clippy.

## E56. Point On Face on a finite spherical band

The checked solver binds a point to an unperforated full-turn spherical band
with two finite latitude rims away from the poles. It retains the original
face occurrence, continuous longitude lift and source revision. Three
ordinary hard rows share the periodic drag path with the cylinder. The
spherical qualification checks the stored native trim and a positive
surface Jacobian bound independently before publishing a candidate.

The radius-2 fixture uses native curved meridian seams and latitudes
`[0, 0.5]`. It tests both seam senses, exact equator with zero achieved
distance, the adjacent outside representable value, a separate Euclidean
`3–4–5` error case, near-pole refusal, stale and late-cancelled edits,
rebind and detach. The leaf passed 322 toolkit tests with nine opt-in
cases skipped, strict production Clippy and 52 ThreadSanitizer cases
across sphere, cylinder and planar attachment. Two independent replay
processes produced identical receipts.

The native lower tests passed all 15 recorded capture cases. A rounded
world point on a nonzero rim is refused when its trim side cannot be
resolved. Partial bands and polar mounts have separate typed refusals.
The combined planar, spherical and arc stage passed 6,972 tests and
strict Rust 1.98.1 all-target Clippy. Nine opt-in tests were skipped.

## E57. Radius continuation through an exact half turn

The authored Radius record can explicitly allow a free midpoint-witness arc
to cross a half turn. The supported family keeps the witness free and at
least one chord endpoint movable, with at most one axis constraint on the
chord. Publication independently reads the actual native circle and sweep.
The two existing hard rows retain their rank through the half turn. Ordinary
Minor and Major branch rules and rigid-edit preservation still apply.

The owned family crosses from Minor to Major and back after reopen. It
retains one fixed endpoint and a chord axis, with rank 7 and two degrees of
freedom. Direct endpoint locks and other coupled relations receive typed
refusals. A native planar attachment control verifies the same refusal on an
arc point and independently accepts attachment of an unrelated point.
The composed stage passed 360 tests and strict Rust 1.98.1 Clippy. Nine
opt-in cases were skipped.

## E58. Three-station Curve On Face on a planar trim

The three-point authoring case retains the original start, midpoint and end
identities and their fixed parameters `[0, 1/2, 1]`. The admitted edge and its
pcurve both retain native degree two. They share the same fit operator and
carry separate measured station-error bounds. The four-station cubic keeps
its existing recipe and solver continuation.

The owned top-face fixture checks literal native degree, control coordinates,
parameters, replay and detached ownership. The holed fixtures prove a
closed-rim tangency and reject entry strictly between three valid stations.
They also check reversal, a tight distance limit and typed refusal of
unsupported quadratic tangent and curvature relations. The composed
half-turn control refuses a crossing Radius whose arc shares a real station
with the qualified planar quadratic.

The isolated leaf passed 343 Sketch3D, 1,374 Math and 154 BRep tests and
strict Rust 1.98.1 Clippy. Nine opt-in Sketch3D cases were skipped. The
39 native face ThreadSanitizer cases passed with reproducible detached
archives. Five or more stations require a separate piecewise fit contract.

The combined source stage passed 5,233 tests with nine opt-in skips,
including the crossing-arc incident quadratic fixture. Strict all-target
Clippy, `cargo deny --all-features check` and `cargo audit` passed.
