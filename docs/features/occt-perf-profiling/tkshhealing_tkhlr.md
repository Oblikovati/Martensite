# `martensite-tkshhealing` + `martensite-tkhlr` vs OCCT `TKShHealing` + `TKHLR` (p95 Profiling)

Benchmarked against OCCT 8.0.0 Release (`/git/oblikovati-workspace/occt-install-vis`), 100 samples per API (p95 in nanoseconds).

## Results (Before vs After Optimization)

| Public API | OCCT p95 (ns) | Baseline M p95 (ns) | Optimized M p95 (ns) | Speedup | Final Ratio (M/O) | Checksum |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `ShapeFix::fix_box` | 337,161.3 | 2,118,433.2 | 1,202,827.0 | **1.76x** | 3.57x | OK |
| `ShapeFix::fix_cylinder` | 122,183.2 | 766,412.0 | 448,319.1 | **1.71x** | 3.67x | OK |
| `ShapeUpgrade::unify_box` | 54,690.3 | 312,519.6 | 228,762.6 | **1.37x** | 4.18x | OK |
| `ShapeAnalysis::tolerance_box` | 2,394.8 | 10,455.6 | 8,312.0 | **1.26x** | 3.47x | OK |
| `HLRBRep::algo_sphere` | 527,743.2 | 2,404,926.6 | 1,987,487.6 | **1.21x** | 3.77x | OK |
| `HLRBRep::algo_cylinder` | 123,611.0 | 499,959.6 | 416,621.4 | **1.20x** | 3.37x | OK |
| `HLRBRep::algo_box` | 52,766.7 | 207,609.3 | 195,700.3 | **1.06x** | 3.71x | OK |

## Key Optimizations Applied

1. **Lazy `empty_copied()` in `ShapeBuildReShape::rebuild` & `BRepTools::ReShape::rebuild` (`martensite-tkshhealing/src/shape_build/reshape/apply.rs`, `martensite-tkbrep/src/brep_tools/reshape/apply.rs`)**:
   - Both re-shapers previously called `shape.empty_copied()?` eagerly on every visited Solid, Shell, Face, Wire, and Edge before traversing children, copying every edge's `BRepCurve3D` and `BRepCurveOnSurface` representations (and evaluating 2D endpoint UVs) only to discard the copy whenever `!modified`.
   - Deferred `empty_copied()` until the first modified child is encountered, buffering unchanged children and materializing the copy on demand. Eliminated >15% of total runtime in `ShapeFix` and `ShapeUpgrade`.
2. **Single-lock `BRepTEdge::empty_copy` & `CurveOnSurfaceState::copy_from_current_binding` (`martensite-tkbrep/src/brep/tedge.rs`, `curve_on_surface.rs`)**:
   - Borrowed `fields.curves` directly instead of cloning `Vec<Handle<dyn BRepCurveRepresentation>>` and consolidated 4 field reads/writes into a single read/write lock pair.
   - Consolidated 4 `fields.read()` lock acquisitions in `CurveOnSurfaceState::copy_from_current_binding` into one.
3. **Inlined `DoubleTab::value` & `change_value` (`martensite-tkmath/src/math/double_tab.rs`)**:
   - Marked `#[inline]` to eliminate call overhead inside `AppCont::LeastSquare` and `HLRBRep` polynomial approximation loops.

## Verification

- `cargo test --lib -p martensite-tkshhealing -p martensite-tkhlr -p martensite-tkbrep`: all passed.
- `cargo clippy --lib -p martensite-tkmath -p martensite-tkbrep -p martensite-tkshhealing -p martensite-tkhlr -- -D warnings`: clean.
