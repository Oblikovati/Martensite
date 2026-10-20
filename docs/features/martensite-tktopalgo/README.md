# Topological algorithms

`martensite-tktopalgo` provides the algorithms on boundary representation shapes:
point and solid classification, shape construction and sewing, validity checks,
distance and proximity extrema, bounding boxes, global properties and the medial axis
of faces (#208).

| Module | Capabilities |
| --- | --- |
| `brep_lib` | Makers of vertices, edges, wires, faces, shells and solids; same-parameter, tolerance, orientation and edge fusion tools; point clouds on shapes. |
| `brep_builder_api` | The construction interface over `brep_lib`, the copy, transformation and NURBS modifications, sewing and shapes built from meshes. |
| `brep_check` | Validity checks of vertices, edges, wires, faces, shells and solids, and the shape analyzer. |
| `brep_class`, `brep_class3d`, `brep_top_adaptor` | Classification of points against faces and solids, and the face topology tools of the intersections. |
| `brep_extrema` | Minimum distance between shapes, overlap and self-intersection of triangulated shapes. |
| `brep_int_curve_surface`, `int_curves_face` | Intersections of curves and lines with the faces of a shape. |
| `brep_bnd_lib` | Axis-aligned and oriented bounding boxes of shapes. |
| `brep_gprop` | Global properties: length, area, volume, centre of mass and inertia of shapes. |
| `brep_approx` | Approximation of the intersection lines of two faces. |
| `bisector`, `mat`, `mat2d`, `brep_mat2d` | Bisecting loci and the medial axis of planar faces. |

## Ownership and thread safety

The algorithms take shapes by handle and build new topology; shapes and geometry
remain shared between inputs and results unless an algorithm copies them. A
successful status reports the algorithm's own completion; it does not validate the
result as a modelling operation, which `brep_check` does separately.

The default precision and plane of `brep_lib` are process-wide settings. Tests that
change them run in dedicated executables (`brep_lib_makers`), as does the test that
changes the process-wide exception stack capture (`extrema_exception_capture`).

## Verification

Run `cargo test -p martensite-tktopalgo`. The ordinary tests cover the makers and
their statuses, sewing, the checks, classification, extrema, bounding boxes, global
properties with closed forms, the face topology tools, the curve/face intersections
and the medial axis. Shape and face records built from numbers (`tests/common`,
`tests/solid_records`, `tests/gprop_records`) give the behavior tests their inputs.

The `dataset_cases` target runs 131 regression cases that restore their inputs from
the public test dataset (release 7.9.0) in the `external/martensite-test-data`
submodule, or from `$MARTENSITE_TEST_DATA_DIR`; one input is an issue attachment in the
submodule's `github-issues/` folder. Each case runs alone in a fresh child process
under the elapsed-time budget of its group. View steps of the case scripts are skipped;
every kernel check still runs.

The `dataset_timing` target holds the classifier timing case
`bugs_modalg_6::bug27884`, which needs the boxed 2D classifier to be 4, 3, 15 and 15
percent faster than the unboxed one. It is ignored until #207 removes the edge-to-plane
projection cost that dominates the measurement; run it with `--ignored`.
