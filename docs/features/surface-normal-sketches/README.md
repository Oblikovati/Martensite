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
