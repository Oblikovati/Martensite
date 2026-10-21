# Primitive solids and sweeps

`martensite-tkprim` builds the primitive solids (boxes, wedges, cylinders, cones,
spheres, tori, revolved bodies and half-spaces) and sweeps any shape along a vector or
around an axis, with the history of the generated shapes (#208).

| Module | Capabilities |
| --- | --- |
| `brep_prim` | The primitive builders: wedges and boxes, and the one-axis bodies (cylinder, cone, sphere, torus, revolution) with their faces, wires, edges and vertices. |
| `brep_prim_api` | The shape makers: `MakeBox`, `MakeWedge`, `MakeCylinder`, `MakeCone`, `MakeSphere`, `MakeTorus`, `MakeRevolution`, `MakeHalfSpace`, `MakePrism` and `MakeRevol`. |
| `brep_preview_api` | A box maker that accepts flat boxes and gives a vertex, an edge or a planar face when the box is thin. |
| `sweep` | The sweep shape structure: the numbered directing shape and its iterator. |
| `brep_sweep` | Linear and rotational sweeps of a shape: the generated geometry, the topology and the pcurves of the sweep. |

## Ownership and thread safety

Makers own their parameters and build new shapes; the input shapes of a sweep are
shared, not changed. The crate has no process-wide state. A maker that fails reports a
typed error; a built shape is not validated as a modelling operation by the maker.

## Verification

Run `cargo test -p martensite-tkprim`. The tests cover every maker (boxes, wedges,
cylinders, cones, spheres, tori, prisms and revolutions), the sweep history, global
properties and bounding boxes of primitives, and the boundary representation, shape
graph and topological algorithm cases that need primitive solids. The `dataset_cases`
target runs the primitive regression cases with inputs from the public test dataset in
the `external/martensite-test-data` submodule; each runs alone in a child process
under its group's time budget, and its log must get the `OK` or `BAD (known problem)`
verdict.
