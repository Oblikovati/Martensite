# Aggregate-copy isolation corpus

Run `cargo test -p martensite-tktopalgo --test toolkit brep_builder_api_copy_isolation`.
The tests use static assets only.

The completed fixture is a compound containing two distinct planar faces, each
with a wire referencing the same finite analytic line edge, and a reversed
occurrence of that edge. Both p-curves, the active triangulation, UVs, normals,
triangles, cached bounds, polygon nodes, node indices, parameters and deflections
are attached before copying or taking snapshots. `TopoDS_Builder::Add` freezes
component flags during fixture construction; that prior change is not attributed
to copying.

Actual `BRepBuilderAPICopy(true, true)` and its `Modified` images are exercised.
Tests mutate both the source and the copied geometry, representation locations,
flags, tolerances and mesh storage. They assert independently known values and
that the opposite hierarchy is unchanged. The copy keeps shared incidences
inside its own result and preserves the reversed occurrence. These faces are
representation fixtures, not a completed or validated region-to-solid result.

The four retained edge-copy rows are split between `reference_reuse.expected` and
`reference_mesh_failure.expected`; two byte-identical captures produced them. The
aggregate fixtures replay those state transitions and additionally check the earlier
published geometry and mesh snapshots after failures and recovery.

Two retained limits matter for consumers:

- A reused copier retains its existing modifier map. After a null-input failure,
  copying the same mutated source can return the old copied entity and old
  geometry while reporting completion. A fresh copier per request observes the
  revised geometry; the corpus preserves both behaviors.
- A nodes/parameters length mismatch is a genuine `Standard_DimensionMismatch`
  inside polygon copying. The previous successful result remains unchanged;
  the same builder can subsequently complete on its earlier valid source.

This is an independent corpus for the listed line/plane representations. It is
not a blanket isolation certificate for every geometry subtype, deferred mesh
provider, inactive triangulation, application cache, or unknown descendant.
The mapped copy operation does not establish a sealed immutable application
result: the application still needs an owned result wrapper and complete copy,
operation and healing history. It must not publish named caller-input handles
as isolated output.

Boundary cancellation belongs to the injected arrangement stages of the sketch-region
operation. Copy's mapped `Perform` currently uses a default progress range, so this
corpus does not claim in-flight copy cancellation.
