# Topological naming and document attributes

`martensite-tkcaf` extends the document framework of `martensite-tklcaf` with shapes.

| Module | Capabilities |
| --- | --- |
| `tnaming` | Named shapes and their evolution, names, selections and the iterators over them. |
| `tdata_xtd` | Construction geometry, constraints, patterns, placement, presentation and triangulation attributes. |
| `app_std` | The standard application. |

## Ownership and thread safety

Named shape history reads shapes by handle.

## Verification

Run `cargo test -p martensite-tkcaf`. The driver cases in `tests/caf_driver_cases` replay document
scripts as ordinary tests. The tests depend on `martensite-tkfillet` and `martensite-tkbool` for
real naming histories, and the library depends only on `martensite-tkbo` to classify wires.
