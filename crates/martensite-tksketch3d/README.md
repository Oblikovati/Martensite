# martensite-tksketch3d

A spatial sketch is an immutable snapshot of points and directed line segments.
Points store model-space coordinates. Lines refer to their endpoints by ID, so
two segments that meet at a shared point can move together.

Reserve IDs in the document before calling the toolkit. Issue fresh IDs across
removals and undo. The kernel checks uniqueness within the current snapshot and
batch. Point and line IDs share one namespace, and both snapshot iterators use
ascending key order. Sketch markers and center markers have their own rows and
can also serve as endpoints.

Supply a positive finite minimum in model units when creating the snapshot.
Admission compares the squared distance of the supplied coordinates with the
squared minimum using exact arithmetic. Equality is allowed. Line geometry
also includes a binary64 length estimate with no certified error bound and
a support line with rounded direction. Use the authored endpoints for exact
checks. The toolkit refuses a segment when its displacement or estimated length
overflows, or a nonzero component disappears from its unit direction.

An edit batch adopts a complete snapshot. It can traverse the whole draft and
reuses proof for lines whose identities and endpoint bits match. A failed or
cancelled batch leaves the source available for the next edit.

Removing a point removes its incident lines. A batch with any deletion also
retires every unused defining point and keeps unselected markers. For a batch
that explicitly removes both a point and its line, list the line removal first.
