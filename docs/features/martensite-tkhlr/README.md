# Hidden line removal

`martensite-tkhlr` computes the visible and hidden lines of shapes seen from a direction
or a point: interval sets, the polyhedral and exact hidden line algorithms, the contour
lines of surfaces and the conversion of the results back to shapes (#208).

| Module | Capabilities |
| --- | --- |
| `intrv` | Interval sets with bound edits, relative positions and boolean operations. |
| `top_bas`, `top_cnx` | Interference records and the edge-face transition. |
| `contap` | Apparent contours of surfaces: the contour search and walking and their records. |
| `hlr_algo` | The value classes of the algorithms: boxes, edge and wire blocks, edge statuses, interferences and the projector. |
| `hlr_topo_brep` | The outlined shape: contours, isolines and the edge splitting of faces. |
| `hlr_brep` | The exact and polyhedral hidden line algorithms, their curves, surfaces and intersectors, and the results as shapes. |
| `hlr_appli` | Reflect lines of a shape. |

## Ownership and thread safety

The algorithms keep their inputs by handle and build new shapes for their results.
The contour sampling range and the exception stack capture setting are process-wide;
the tests that observe them run in dedicated executables (`contap_hcont_tool`,
`contap_analytic_failure_capture`, `sampling_context`). A completed hidden line
computation does not validate its result as a modelling operation.

## Verification

Run `cargo test -p martensite-tkhlr`. The tests cover the interval sets, the
interference records, the contour records, functions, search and walking, the
hidden line value classes, the curve, surface and intersector tools of the exact
algorithm and the area limits. The `*_numeric` tests replay frozen requests against
frozen answers bit for bit; their inputs build primitive solids with `martensite-tkprim`,
a development dependency. The answers that changed with the correctly rounded
transcendental functions are attributed in [`corpus-attribution`](corpus-attribution/README.md). The `dataset_cases` target runs the two hidden-line
regression cases (`bugs_modalg_6::bug25214` and `bug25908`) with inputs from the public
test dataset in the `external/martensite-test-data` submodule; each runs alone in a
child process under its group's time budget, and its log must get the `OK` or
`BAD (known problem)` verdict.
