# Offset modeling

`martensite-tkoffset` builds new shapes from existing ones: offset shells and thick solids, pipes along a
spine, lofts through sections, drafted faces and tangent blends.

| Module | Capabilities |
| --- | --- |
| `brep_offset` | The offset of faces and the maker of offset shapes and thick solids, by arcs or by intersections, and the simple offset. |
| `brep_offset_api` | The entry points: offset shapes and wires, thick solids, pipes and pipe shells, lofts, drafts, filling, and the middle path. |
| `draft` | The modification that drafts the faces of a shape. |
| `bi_tgte` | Tangent blends along edges. |

## Ownership and thread safety

The builders take their shapes by handle and build a new shape. A builder is used from one thread.

## Verification

Run `cargo test -p martensite-tkoffset` with `CARGO_PROFILE_DEV_OPT_LEVEL=1`, because the CPU budgets of the
script cases assume light optimization.

`tests/dataset_cases` holds the offset, loft, sweep, draft and regression script cases. Each runs alone in a
child process, draws on a memory raster and is graded on its whole output. `tests/viewer_cases` holds the
scripts that end on a shaded view. They run on Linux only, need the
[renderer test setup](../martensite-vulkan/README.md) and must leave the validation report empty.

The bottle and snowflake samples load their text by path from the FreeFont files in the test data, so
they do not depend on the fonts installed on a machine.

When a split cap face closes an offset shell, the offset faces decide the shell's orientation. The
same input therefore gives the valid solid on every run, and `offset/compshape/A3` holds a strict clean
verdict.

`hlr/poly_hlr/Plate` is a known problem whose length moved from 404.004 to 404.492 with correctly
rounded math, with its check and tolerance unchanged. [The attribution](corpus-attribution/README.md)
records the proof.
