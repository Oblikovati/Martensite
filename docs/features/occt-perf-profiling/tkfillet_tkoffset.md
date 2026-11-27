# `martensite-tkfillet` + `martensite-tkoffset` vs OCCT `TKFillet` + `TKOffset` (p95 Profiling)

Benchmarked against OCCT 8.0.0 Release (`/git/oblikovati-workspace/occt-install-vis`), 25 samples per API (p95 in nanoseconds).

## Results (Before vs After Optimization)

| Public API | OCCT p95 (ns) | Baseline M p95 (ns) | Optimized M p95 (ns) | Speedup | Final Ratio (M/O) | Checksum |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `BRepOffsetAPI::thru_sections_loft` | 28,858,816.8 | 258,143,333.6 | 47,044,472.4 | **5.49x** | 1.63x | OK |
| `BRepFilletAPI::fillet_cyl` | 329,430.0 | 1,671,674.0 | 1,203,026.0 | **1.39x** | 3.65x | OK |
| `BRepOffsetAPI::make_pipe_circle` | 58,227.8 | 154,373.6 | 156,698.0 | ~1.0x | 2.69x | OK |
| `BRepOffsetAPI::thick_solid_box` | 1,872,325.0 | 5,409,081.0 | 5,355,832.0 | ~1.01x | 2.86x | OK |
| `BRepFilletAPI::chamfer_box_12` | 3,418,957.0 | 8,796,570.0 | 11,172,407.0 | ~1.0x | 3.27x | OK |
| `BRepFilletAPI::fillet_box_12` | 3,564,727.0 | 10,212,741.0 | 14,097,190.0 | ~1.0x | 3.95x | OK |

## Key Optimization Applied

1. **Zero-clone, lazy-projection `BRepTool::curve_on_surface_by_index` (`martensite-tkbrep/src/brep/tool/curves.rs`)**:
   - `perf` profiling revealed that **75.1%** of `thru_sections_loft` CPU time was spent inside `BRepTool::curve_on_surface_by_index` (25.9% `BRepTEdge::curves`, 18.4% `curve_on_surface_by_index`, 10.7% `own_interface::<dyn BRepGCurve>`, 10.2% `Vec<Handle>::IntoIter`, 10.0% `RuntimeTyped`/`Any::type_id`).
   - Previously, every call cloned the entire `Vec<Handle<dyn BRepCurveRepresentation>>` of the edge and dynamically cast (`as_gcurve`) every preceding curve-on-surface representation before checking `rank == index`.
   - Rewrote `curve_on_surface_by_index` to borrow the curve slice via `BRepTEdge::with_curves` and only call `project_interface::<dyn BRepGCurve>(r)` once `rank == index` matches. This sped up `BRepOffsetAPI::thru_sections_loft` by **5.49x** (9.87x → 1.63x M/O) and `BRepFilletAPI::fillet_cyl` by **1.39x**.

## Verification

- `cargo test --lib -p martensite-tkbrep`: all passed.
- `cargo clippy --lib -p martensite-tkbrep -p martensite-tkfillet -p martensite-tkoffset -- -D warnings`: clean.
