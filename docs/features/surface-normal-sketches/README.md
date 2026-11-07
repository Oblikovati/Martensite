# Surface sketches and normal extrusion

Feature [#414](https://github.com/Oblikovati/Martensite/issues/414) adds sketches
on cylinders and cones. A feature grows from its sketch along the support's
normal at each point. Existing planar sketches and vector extrusions retain
their interfaces and behaviour.

The implementation starts with one complete cylindrical operation: a rectangle
in developed coordinates becomes a standalone solid, including when it crosses
the support's angular seam. Cone regions, general curves, holes, complete wraps,
and attachment to an existing body have separate acceptance tasks. They are not
implied by the first operation.

## Existing kernel support

`tkbo::profile_extrusion` accepts solved planar curves, builds bounded regions,
and extrudes a region along a fixed vector. It supplies history, validation and
measured volume. It does not solve a general sketch constraint system.

`tkg3d` supplies analytic cylinders and cones, normal-offset surfaces and the
`Adaptor3dCurveOnSurface` evaluator. These are geometry building blocks. The new
operation must also construct shared topology, account for periodicity, retain
source identity and validate the completed solid.

See [the geometry contract](DESIGN.md) and [the evidence plan](VERIFICATION.md).

## Delivery tasks

| Task | Scope | Depends on |
| --- | --- | --- |
| [#415](https://github.com/Oblikovati/Martensite/issues/415) | Geometry and ownership contract | None |
| [#416](https://github.com/Oblikovati/Martensite/issues/416) | Complete cylindrical rectangular normal extrusion | #415 |
| [#417](https://github.com/Oblikovati/Martensite/issues/417) | Conical support away from its apex | #416 |
| [#418](https://github.com/Oblikovati/Martensite/issues/418) | General curves and holes | #415, #416, #417 |
| [#419](https://github.com/Oblikovati/Martensite/issues/419) | Complete wraps and periodic regions | #418 |
| [#420](https://github.com/Oblikovati/Martensite/issues/420) | Bosses, pockets, support revisions and replay | #416, then applicable support tasks |

A task is complete only after its implementation and evidence are integrated.
No package version or release is authorized by the feature approval.

## Cylindrical rectangle

`surface_sketch::normal_extrude` accepts a rectangle in developed coordinates.
For a radius of 10, a width of 7 wraps through 0.7 radians. The interval
`[-1, 2]` reaches radii 9 and 12 with an outward support normal. A chart origin
of -0.3 makes this example cross the conventional zero-angle seam.

```rust
use martensite_tkbo::profile_extrusion::{DisplayTolerance, Resolution, ToleranceCap};
use martensite_tksketch::surface_sketch::*;

let rectangle = CylinderRectangle {
    support: CylinderSupport {
        origin: [0.0; 3],
        axis: [0.0, 0.0, 1.0],
        radial: [1.0, 0.0, 0.0],
        radius: 10.0,
        chart_origin: -0.3,
        normal: NormalSense::Outward,
    },
    region: SurfaceRegionId(1),
    s: [0.0, 7.0],
    z: [0.0, 4.0],
    corners: [1, 2, 3, 4].map(SurfaceVertexId),
    boundaries: [1, 2, 3, 4].map(SurfaceCurveId),
};
let options = NormalExtrusionOptions {
    resolution: Resolution::new(
        1e-6, 1e-7, 1e-9,
        DisplayTolerance { chordal: 1e-3, angular: 0.1 },
        ToleranceCap::Refuse(1e-5),
    ),
    volume_relative_error: 1e-9,
};
let result = normal_extrude(&rectangle, [-1.0, 2.0], options)?;
let detached = result.export_brep()?;
// `detached.entities` resolves the result's semantic history keys to native shapes.
```

The first and last depth roles follow the requested interval even for an inward
normal. `export_brep` creates fresh native owners each time. The result's metadata
and encoded body remain immutable when a caller changes an export.

Current admission excludes holes, general curves, cone supports, complete turns
and axis-crossing depth intervals. Small features also need clearance from the
native geometry tolerances. The operation reports typed refusals for unresolved
dimensions or nonfinite arithmetic. Its generic geometry checks are sampled;
volume uncertainty combines separately reported integration and construction
rounding estimates. `volume_error_estimates()` exposes both components, and
`effective_limits()` reports the linear and angular admission clearances.
Consult the returned validation
report and measured value when deciding whether that evidence meets an
application's requirements.
