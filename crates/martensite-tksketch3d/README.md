# martensite-tksketch3d

A spatial sketch is an immutable snapshot of points and directed line segments.
Points store model-space coordinates. Lines refer to their endpoints by ID, so
two segments that meet at a shared point can move together.

Reserve IDs in the document before calling the toolkit. Issue fresh IDs across
removals and undo. The kernel checks uniqueness within the current snapshot and
batch. Point and line IDs share one namespace, and both snapshot iterators use
ascending key order. Sketch markers and center markers have their own rows and
can also serve as endpoints.

Use `Edit::InsertFrozenPoint` to connect to a captured point from another sketch.
The caller resolves the source and supplies its position in destination model
units. The snapshot stores the captured source revision and coordinates. The
point can serve as a line endpoint and keeps its position if the source changes
or is deleted. Detach it before moving it. Detachment keeps the point ID and
incident lines.

Relations have a separate caller-reserved ID namespace. `ConstrainedSketch::apply`
captures Fix anchors and solves authored relations before adoption. `drag` uses
the prepared program for that accepted sketch and publishes a new snapshot only
when the geometry and the caller's physical tolerances pass an independent check.
Use `inspect_rank` outside interactive drag when numerical rank needs inspection.
Frozen points contribute exact fixed coordinates to either solve and keep their source stamp.

Supply a positive finite minimum in model units when creating the snapshot.
Admission compares the squared distance of the supplied coordinates with the
squared minimum using exact arithmetic. Equality is allowed. Line geometry
also includes a binary64 length estimate with no certified error bound and
a support line with rounded direction. Use the authored endpoints for exact
checks. The toolkit refuses a segment when its displacement or estimated length
overflows, or a nonzero component disappears from its unit direction.

Use `Snapshot::wire_recipe` with a directed list of line IDs and a pair-check
budget. A wire joins consecutive uses through shared point IDs and checks exact
contact on the authored segments. `WireRecipe::materialize` returns a fresh BRep
wire with its point and edge bindings. It verifies the stored native tolerance
and bounds the deviation of each native line trim from its authored segment.
Sweep and Loft can take independent exports from the same recipe.

An edit batch adopts a complete snapshot. It can traverse the whole draft and
reuses proof for lines whose identities and endpoint bits match. A failed or
cancelled batch leaves the source available for the next edit.

Removing a point removes its incident lines. A batch with any deletion also
retires every unused defining point and keeps unselected markers. For a batch
that explicitly removes both a point and its line, list the line removal first.
