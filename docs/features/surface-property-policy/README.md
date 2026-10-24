# Surface integration and representation policy

[Feature #277](https://github.com/Oblikovati/Martensite/issues/277) adds an explicit
choice of geometric integration independently from representation preference.

```rust
use martensite_tktopalgo::brep_gprop::{
    BRepGProp, SurfaceIntegration, SurfacePropertyOptions, SurfaceRepresentation,
};
let options = SurfacePropertyOptions {
    integration: SurfaceIntegration::FixedOrder,
    representation: SurfaceRepresentation::PreferExistingMesh,
    skip_shared: false,
};
let result = BRepGProp::surface_properties_with_options(shape, options)?;
let area = result.properties().mass();
```

Each face uses an existing mesh when preferred and available, with geometry as
fallback. Geometry preference likewise falls back to a mesh-only face. Fixed
orders apply to geometric faces; triangle measurements use their existing
polynomial integration. Neither preference creates or changes a mesh.

`SurfacePropertyOptions::default()` requests adaptive geometry at relative target
`1e-5`, prefers geometry, and counts every face occurrence. Both earlier
surface-property methods retain their defaults and numerical outputs. The
shared-face rule still distinguishes locations and ignores orientation.

The returned values own no shape, geometry or mesh handles. They remain valid
after source mutation. Requested options, selected face counts, error evidence
and effective-target information travel with the properties. Failure returns
no partial result. The new API rejects missing representations, nonfinite
properties or adaptive estimates, exhausted required intervals and a known incomplete adaptive signal.

## Completed empty queries

An input containing no faces completes successfully. `result.is_empty()` is true,
area and inertia are zero, and the numerical centroid convention is the input's
located origin. That point is not a physical center of mass for an empty domain.
A face with missing sources or an exhausted required computation returns an
error instead of a completed-empty result. Nonempty zero-area observations do
not become empty solely because their measured mass is zero.

## Error evidence

`EstimatedRelative` is the engine's maximum geometric face error estimate.
It is not a certified bound and does not assert that the requested target was
achieved. Fixed orders, triangle contributions, zero-area observations and
unavailable estimates yield `Unknown`. A mixed measurement containing triangles
has unknown aggregate error even if its geometric faces had estimates.

Adaptive requests must be finite and satisfy `0 < target < 0.001`; invalid values
are rejected before reading the shape. They are retained without an interface
clamp. The engine can relax internal absolute refinement budgets and does not
publish a final effective global target, so `effective_relative_target()` is
`None`. The [kernel decision](decision.md) records the existing limits.

These are representation measurements. They do not validate topology, mesh
quality, geometric consistency or modelling acceptance.

## Independent controls and replay

Tests distinguish curved geometry from coarse triangle sums, and check area,
centroid and inertia against cylinder, rectangle and right-triangle formulas.
They cover unmeshed, meshed, mixed and mesh-only faces, reversed/shared
occurrences, locations, input mutation and failure isolation. Property and
evidence bytes agree across two independent processes and one/four workers.

The elliptic prism has semi-axes 2 and 1 and height 3. Its true area is
`4*pi + 24*E(3/4)`. The [exact-rational calculation](ellipse_area_bounds.py)
uses Machin's identity, bounded alternating arctangent series, and a positive
elliptic-series tail bounded by its next term divided by `1-3/4`.
The [outward-rounded enclosure](ellipse-area-bounds.json) is approximately
`[41.6317152760022015491360831222934997725869,
41.6317152760022015491360831222934997754152]`.

The existing fixed and adaptive values are replay controls, separate from that
reference. The fixed result is approximate. The existing adaptive result and
its tiny reported estimate disagree with the enclosure; this feature preserves
that numerical behavior and labels the value as an estimate. It does not repair
the adaptive accuracy limitation or promote the estimate to a bound.

Run `cargo test -p martensite-tktopalgo surface_property_policy` for the focused
integration tests and `cargo test -p martensite-tktopalgo --lib surface_policy`
for the incomplete-observation guard. The rational proof can be regenerated
with `python3 docs/features/surface-property-policy/ellipse_area_bounds.py`;
Rust tests do not run Python.
