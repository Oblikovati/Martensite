# martensite-tksketch3d

A spatial sketch is an immutable snapshot of points and directed curves.
Points store model-space coordinates. Lines refer to their endpoints by ID, so
two segments that meet at a shared point can move together.

Reserve IDs in the document before calling the toolkit. Issue fresh IDs across
removals and undo. The kernel checks uniqueness within the current snapshot and
batch. Points and curves share one ID namespace, and both snapshot iterators use
ascending key order. Sketch markers and center markers have their own rows and
can also serve as endpoints.

Use `Edit::InsertFrozenPoint` to connect to a captured point from another sketch.
The caller resolves the source and supplies its position in destination model
units. The snapshot stores the captured source revision and coordinates. The
point can serve as a line endpoint and keeps its position if the source changes
or is deleted. Detach it before moving it. Detachment keeps the point ID and
incident curves.

Relations have a separate caller-reserved ID namespace. `ConstrainedSketch::apply`
captures Fix anchors and solves authored relations before adoption. `drag` uses
the prepared program for that accepted sketch and publishes a new snapshot only
when the geometry and the caller's physical tolerances pass an independent check.
Use `inspect_rank` outside interactive drag when numerical rank needs inspection.
Frozen points contribute exact fixed coordinates to either solve and keep their source stamp.
Use `ConstrainedSketch::restore` to prepare saved geometry with its authored
relation values. Restore checks the saved anchors and all physical relations
before publishing the supplied coordinates. A captured point may carry its
source incarnation and revision or its historical session generation.

Spatial line relations include signed axis parallelism, plane parallelism,
perpendicularity, equal length and driving angle. A plane-parallel relation
leaves offset free. Perpendicular lines may be skew. Angle authoring requires
a nonsingular starting angle, and the accepted branch remains in the recipe.

Point On Line constrains a distinct point to the infinite support. It leaves
axial motion free and checks distance against the exact authored endpoints.
Colinear stores the direction branch and uses endpoint IDs to count the
required equations. A relation between two lines with both endpoint IDs in
common keeps its own ID and has no solver rows. When angular bounds overlap
the requested tolerance, the toolkit compares the authored endpoint directions
with that tolerance using exact arithmetic. As a drag approaches a frame
boundary, the toolkit can prepare a new reference on an accepted step.
The replacement reuses the existing solver layout and counts against the
current tick. A refused step keeps the last accepted sketch.

Use `ConstrainedSketch::transform_points` for a rigid translation or rotation of
owned points. The operation checks the resulting geometry and existing hard
relations together before adopting. The length tolerance also caps each stored
position's error relative to the exact translation or the native rotation matrix.
A rotation keeps exact axis points in place and refreshes the prepared relation
frames for the next drag.

Use `Snapshot::mirror_copies` to reflect selected points and directed lines
across a work plane. Supply fresh IDs and include both endpoints of each
selected line. The copies keep point and curve roles. A copy of a frozen point
is editable, and the original retains its captured source. The result maps each
copy to its source and certifies that the authored geometry meets the supplied
position limit. Constraint recipes remain with their original entities.

Supply a positive finite minimum in model units when creating the snapshot.
Admission compares the squared distance of the supplied coordinates with the
squared minimum using exact arithmetic. Equality is allowed. Line geometry
also includes a binary64 length estimate with no certified error bound and
a support line with rounded direction. Use the authored endpoints for exact
checks. The toolkit refuses a segment when its displacement or estimated length
overflows, or a nonzero component disappears from its unit direction.

`Edit::InsertArc` places an open circle trim through an interior point or
around a named center and an oriented normal. The witness selects the branch.
For center placement, the arc sweeps positively about the normal, which also
chooses the plane at a semicircle. A three-point arc uses the snapshot minimum
for chord length and witness altitude. Center placement uses it for chord and
radius lengths. Supply an `ArcFitTolerance` to limit the deviation at authored
placement points. The output preserves their original coordinates and reports
the sampled deviation separately from the nominal circle. The point and line
solver refuses arc-bearing snapshots until it has an arc layout.

`Edit::SetCurveRole` marks a line or arc as construction geometry without
changing its ID or placement. A construction line stays available to
measurement and the constraint solver. The wire constructor refuses it in a
new modelling path, and an already detached recipe keeps its captured inputs.

Use `Snapshot::wire_recipe` with a directed list of line IDs and a pair-check
budget. A wire joins consecutive uses through shared point IDs and checks exact
contact on the authored segments. `WireRecipe::materialize` returns a fresh BRep
wire with its point and edge bindings. It verifies the stored native tolerance
and bounds the deviation of each native line trim from its authored segment.
Sweep and Loft can take independent exports from the same recipe.

An edit batch adopts a complete snapshot. It can traverse the whole draft and
reuses curve checks when the recipe and placement point bits match. A failed or
cancelled batch leaves the source available for the next edit.

Removing a point removes its incident curves. A batch with any deletion also
retires every unused defining point and keeps unselected markers. For a batch
that explicitly removes both a point and its curve, list the curve removal first.
