# Offset modeling

`martensite-tkoffset` builds shapes at a distance from others: offset shells and thick solids, pipes
along a spine, lofts through sections, drafted faces and tangent blends.

| Module | Capabilities |
| --- | --- |
| `brep_offset` | The offset of faces and the maker of offset shapes and thick solids, by arcs or by intersections, and the simple offset. |
| `brep_offset_api` | The entry points: offset shapes and wires, thick solids, pipes and pipe shells, lofts, drafts, filling, and the middle path. |
| `draft` | The modification that drafts the faces of a shape. |
| `bi_tgte` | Tangent blends along edges. |

## Ownership and thread safety

The builders take their shapes by handle and build a new shape. A builder is used from one thread.

## Verification

Run `cargo test -p martensite-tkoffset` with light optimization, `CARGO_PROFILE_DEV_OPT_LEVEL=1`, because
the CPU budgets of the script cases assume it. `tests/dataset_cases` holds the offset, loft, sweep, draft
and regression script cases on a memory raster, each alone in a child process and graded on its whole
output. `tests/viewer_cases` holds the scripts that end on a shaded view; they need the
[renderer test setup](../martensite-vulkan/README.md) and an empty validation report.

The text of the bottle and snowflake samples is loaded by path from the FreeFont files in the test data,
so these cases do not depend on the fonts installed on a machine. The cap faces of a split are oriented
by the offset faces, so `offset/compshape/A3` gives its valid result on every run and its clean verdict
is held strictly.

`hlr/poly_hlr/Plate` is a known problem whose length moved from 404.004 to 404.492 with correctly
rounded math: the case still gives its old line with the platform C library. Its check and tolerance
are unchanged.
