# Tier 6: `martensite-tkbrep` vs OCCT `TKBRep` (p95)

Benchmarked against OCCT 8.0.0 Release (`libTKBRep.so`, `g++ -O3 -DNDEBUG -std=c++17`) vs Martensite (`--release`, `opt-level = 3`), 100 samples per API, all 10/10 checksums verified identical.

## Baseline vs Optimized p95 Results

| Public API | OCCT p95 (ns) | Baseline Martensite (ns) | Optimized Martensite (ns) | Speedup | Ratio (M/O) | Status | Checksum |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- |
| `BRep_Tool::queries_50_faces` | 3,476.6 | 37,569.7 | 24,015.6 | **1.56x** | 6.91x | SLOWER | OK |
| `BRepTools::write_read_10_faces` | 270,549.6 | 1,150,481.4 | 1,156,692.3 | — | 4.28x | SLOWER | OK |
| `TopExp::map_shapes_50_faces` | 33,541.1 | 207,801.2 | 118,508.9 | **1.75x** | 3.53x | SLOWER | OK |
| `TopExp_Explorer::50_faces` | 15,161.3 | 115,548.9 | 40,538.2 | **2.85x** | 2.67x | SLOWER | OK |
| `BRepAdaptor_Curve::d1_100_bsp` | 5,446.7 | 14,233.9 | 13,886.1 | **1.03x** | 2.55x | SLOWER | OK |
| `TopExp::map_ancestors_50_faces` | 49,505.8 | 222,341.1 | 125,164.2 | **1.78x** | 2.53x | SLOWER | OK |
| `BinTools::write_read_10_faces` | 120,219.5 | 279,850.0 | 250,642.8 | **1.12x** | 2.08x | SLOWER | OK |
| `BRepTools::uv_bounds_50_faces` | 25,491.2 | 94,922.1 | 52,865.8 | **1.80x** | 2.07x | SLOWER | OK |
| `BRepAdaptor_Surface::d1_10x10` | 37,505.9 | 62,894.4 | 59,340.8 | **1.06x** | 1.58x | SLOWER | OK |
| `BRep_Builder::build_50_faces` | 139,415.5 | 281,790.1 | 218,313.5 | **1.29x** | 1.57x | SLOWER | OK |

## Optimizations Applied

1. **Lock-free `TShapeBase::shape_type()` (`topods/tshape.rs`):**
   - Stored the immutable `ShapeEnum` directly on `TShapeBase` instead of acquiring `SharedState::read()` on every `shape_type()` query across `TopExp_Explorer`, `TopoDS` casts, and `BRepTools`.
2. **Cached `has_current` on `ShapeIterator` + zero-allocation loop exit (`topods/iterator.rs`, `topexp/explorer.rs`, `topexp/mod.rs`):**
   - Cached `has_current: bool` inside `ShapeIterator::update_current_shape()` so `more()` and `value()` take 0 locks and `next()` takes 1 lock instead of 4.
   - Switched `TopExp_Explorer` (`advance_descent_step`, `current`) and `topexp::map_shapes`/`map_shapes_into_map` from `while let Ok(...) = iterator.value()` to `while iterator.more()`, eliminating a `Failure` + `format!` heap allocation on every exhausted node.
   - Hoisted the inner `Explorer` in `topexp::map_shapes_and_ancestors` (`topexp/ancestors.rs`) to reuse its stack across ancestors.
   - Result: `TopExp_Explorer` **2.85x**, `TopExp::map_ancestors` **1.78x**, `TopExp::map_shapes` **1.75x** faster.
3. **Zero-clone `BRepTEdge::with_curves` + direct `BRepCurve3D` cast (`brep/tedge.rs`, `brep/tool/curves.rs`, `brep/tface.rs`, `brep/tool/face.rs`):**
   - Added `BRepTEdge::with_curves` to borrow `&[Handle<dyn BRepCurveRepresentation>]` under the read lock without cloning the `Vec` of `Arc` handles on every `BRepTool` query.
   - Used `project_concrete::<BRepCurve3D>` directly inside `BRepTool::curve_with_location` (avoiding `Handle::cast::<dyn BRepGCurve>` `Arc` clone/drop) and fast-pathed identity locations.
   - Added `BRepTFace::surface_and_location` to read both under a single `RwLock`.
   - Result: `BRep_Tool::queries_50_faces` **1.56x** faster.
4. **Hoisted per-face surface lookup in `BRepTools::uv_bounds` (`brep_tools/uv_bounds.rs`):**
   - Fetched `BRepTool::surface_with_location` once per face in `add_uv_bounds` / `add_uv_bounds_of_wire` instead of twice per edge (**1.80x** faster).
5. **Single-lock stack-buffered `write_flags` (`toptools/shape_set_io.rs`):**
   - Read `TShapeBase::state()` once into a stack `[u8; 8]` buffer instead of allocating a `Vec<bool>` and locking 7–8 times per shape.

## Remaining Gap Analysis

- **`BRep_Tool::queries_50_faces` (6.91x):** Remaining overhead is `SharedState` (`RwLock`) per-object synchronization on `BRepTVertex`/`BRepTEdge`/`BRepTFace`/`BRepCurve3D` plus `Arc` ref-count increments when returning `Handle<dyn GeomCurve>`/`Handle<dyn GeomSurface>`, vs OCCT's unsynchronized pointer dereferences.
- **`BRepTools::write_read_10_faces` (4.28x):** ASCII BRep serialization/parsing goes through `IoStream` byte-by-byte formatting and `Standard::strtod` parsing.
