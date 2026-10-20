# Geometric algorithms

`martensite-tkgeomalgo` provides the geometric algorithms on curves and surfaces:
intersections, hatching, 2d tangent constraint solving, filling, plate surfaces,
evolution laws, local continuity analysis and the owned sketch spline construction
(#208).

| Module | Capabilities |
| --- | --- |
| `int_curve`, `int_imp_par_gen`, `int_res2d`, `geom2d_int` | Intersections of 2d curves and their results. |
| `int_curve_surface`, `int_patch`, `int_polyh`, `int_walk`, `int_imp`, `int_start`, `int_surf`, `intf` | Curve/surface and surface/surface intersection: analytic cases, polyhedral starts, marching and the shared line and point records. |
| `approx_int`, `app_blend` | Approximation of intersection lines and swept blend surfaces. |
| `geom_api`, `geom2d_api`, `geom_int` | Interpolation, approximation, extrema, projection and intersection entry points on geometry. |
| `gcc_ana`, `gcc_ent`, `gcc_int`, `geom2d_gcc` | Lines and circles tangent to qualified curves, and the bisecting loci. |
| `hatch`, `hatch_gen`, `geom2d_hatch` | Hatching of 2d domains. |
| `geom_fill`, `geom_plate`, `plate`, `fair_curve` | Filling, skinning and sweeping, plate surfaces under constraints and fair curves. |
| `law` | Scalar evolution laws (constant, linear, S-shaped, interpolated, B-spline). |
| `local_analysis` | Continuity between two curves or two surfaces. |
| `topclass`, `toptrans` | Generic 2d classification and complex transitions. |
| `sketch_spline` | The owned sketch spline construction with certified incidences. |

## Ownership and thread safety

Geometry handles share mutable objects; cloning a handle keeps the shared identity.
The algorithm objects keep their inputs and results by handle, so isolating a result
from later edits of its inputs means copying the geometry. Completion flags and
numerical results keep their individual method contracts; they do not certify a
validated modelling operation.

The two blend approximation context flags (`set_context_spline_approx`,
`set_context_approx_with_no_tgt`) are process-wide settings, so their tests run in a
dedicated executable. The debug dump counters of the intersection polygons are
process-wide too; they only number dump output.

## Verification

Run `cargo test -p martensite-tkgeomalgo`. The ordinary tests cover the analytic
intersections, the marching tools, the constraint solvers, hatching, filling, plates,
laws and continuity, the sketch spline constructor and its certified incidences.

The geometry corpus (`tests/fixtures/geometry_corpus`) replays frozen requests of the
blend, fair curve, plate, local analysis and transition families and compares every
answer exactly; NaN tokens compare by classification. Its expected results follow
numerical behavior version 2; the eleven answers that changed with correctly rounded
elementary functions are attributed in
[`corpus-attribution`](corpus-attribution/README.md). The sketch incidence and spline
tests compare certified results against frozen reference values
(`tests/sketch_line_incidences/README.md`, `tests/sketch_spline/README.md`).

The `dataset_cases` target runs 128 regression case scripts against the Rust API, with
inputs from the public test dataset in the `external/martensite-test-data` submodule
or `$MARTENSITE_TEST_DATA_DIR`. A case passes when the verdict of its log is `OK` or
`BAD (known problem)`. Each case runs alone in a fresh child process under the
elapsed-time budget of its group; a script can restart that budget for a long step.
One case, `lowalgos_intss::bug565`, is ignored: its script outlasts its 60 s budget.
