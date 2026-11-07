# Surface sketches and normal extrusion

Feature [#414](https://github.com/Oblikovati/Martensite/issues/414) adds sketches
on cylinders and cones. A feature grows from its sketch along the support's
normal at each point. Existing planar sketches and vector extrusions retain
their interfaces and behaviour.

The supported regions are rectangles on a developed cylinder and annular
sectors on a developed cone. Both become standalone solids and can cross the
support's angular seam. General curves, holes, complete wraps and attachment to
an existing body have separate acceptance tasks.

## Existing kernel support

`tkbo::profile_extrusion` accepts solved planar curves, builds bounded regions,
and extrudes a region along a fixed vector. It supplies history, validation and
measured volume. It does not solve a general sketch constraint system.

`tkg3d` supplies analytic cylinders and cones, normal-offset surfaces and the
`Adaptor3dCurveOnSurface` evaluator. These are geometry building blocks. The new
operation must also construct shared topology, account for periodicity, retain
source identity and validate the completed solid.

See [the geometry contract](DESIGN.md), [the cone sector contract](CONE.md) and
[the evidence plan](VERIFICATION.md).

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

Current admission excludes holes, general curves, complete turns and
axis-crossing depth intervals. Small features also need clearance from the
native geometry tolerances. The operation reports typed refusals for unresolved
dimensions or nonfinite arithmetic. Its generic geometry checks are sampled;
volume uncertainty combines separately reported integration and construction
rounding estimates. `volume_error_estimates()` exposes both components, and
`effective_limits()` reports the linear and angular admission clearances.
Consult the returned validation
report and measured value when deciding whether that evidence meets an
application's requirements.

## Conical sector

`normal_extrude_cone` accepts an annular sector of the developed cone. Slant
distances use model units; `developed_angle` uses radians. A 30-degree cone
maps a developed-angle span of 0.35 radians to a physical span of 0.7 radians.
Using the options from the cylindrical example:

```rust
let sector = ConeSector {
    support: ConeSupport {
        apex: [0.0; 3],
        axis: [0.0, 0.0, 1.0],
        radial: [1.0, 0.0, 0.0],
        semi_angle: std::f64::consts::FRAC_PI_6,
        chart_origin: -0.3,
        normal: NormalSense::Outward,
    },
    region: SurfaceRegionId(2),
    slant: [8.0, 12.0],
    developed_angle: [0.0, 0.35],
    corners: [11, 12, 13, 14].map(SurfaceVertexId),
    boundaries: [11, 12, 13, 14].map(SurfaceCurveId),
};
let cone_result = normal_extrude_cone(&sector, [-1.0, 2.0], options)?;
let cone_body = cone_result.export_brep()?;
```

Normal depth changes both radius and axial position. The operation refuses
axis contact, unresolved apex/angle limits and any result that fails required
validation. A Cartesian rectangle on the unrolled cone is not yet supported;
its boundaries generally need the curve-mapping work tracked by #418. See the
[conical sector contract](CONE.md) for the coordinate map and supported domain.
