# `martensite-tkprim` + `martensite-tkmesh` vs OCCT `TKPrim` + `TKMesh` (p95 Profiling)

Benchmarked against OCCT 8.0.0 Release (`/git/oblikovati-workspace/occt-install-vis`), 100 samples per API (p95 in nanoseconds).

## Results (Before vs After Optimization)

| Public API | OCCT p95 (ns) | Baseline M p95 (ns) | Optimized M p95 (ns) | Speedup | Final Ratio (M/O) | Checksum |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `BRepPrimAPI::make_revol` | 645,255.6 | 1,600,867.4 | 1,270,071.5 | **1.26x** | 1.97x | OK |
| `BRepMesh::mesh_sphere` | 2,019,168.4 | 5,917,284.0 | 4,944,207.4 – 5,600,675.6 | **1.06–1.20x** | 2.25–2.77x | OK |
| `BRepMesh::mesh_torus` | 6,364,373.4 | 16,534,355.0 | 13,718,241.4 – 16,426,781.6 | **1.01–1.21x** | 2.14–2.58x | 2390 vs 2392 tris |
| `BRepMesh::mesh_cylinder` | 529,744.8 | 1,459,855.5 | 1,294,681.0 – 1,613,372.1 | **1.13x** | 2.27–3.05x | OK |
| `BRepPrimAPI::make_prism` | 22,803.2 | 91,030.9 | 78,827.4 – 85,302.9 | **1.07–1.15x** | 3.04–3.74x | OK |
| `BRepPrimAPI::make_box` | 18,174.6 | 52,462.1 | 49,026.2 | **1.07x** | 2.70x | OK |
| `BRepPrimAPI::make_cone` | 9,706.0 | 15,365.0 | 17,096.7 | ~1.0x | 1.76x | OK |
| `BRepPrimAPI::make_cylinder` | 7,337.8 | 15,517.3 | 15,191.1 | ~1.02x | 2.07x | OK |
| `BRepPrimAPI::make_sphere` | 5,279.8 | 11,214.2 | 11,874.0 | ~1.0x | 2.25x | OK |
| `BRepPrimAPI::make_torus` | 4,324.4 | 9,412.4 | 11,933.1 | ~1.0x | 2.76x | OK |
| `BRepMesh::mesh_box` | 265,446.0 | 643,912.6 | 738,073.1 – 910,378.6 | ~1.0x | 2.57–3.43x | OK |

## Key Optimizations Applied

1. **Lock-free `MeshLinkKey` (`martensite-tkmesh/src/brep_mesh/delaun_structure/`)**:
   - Replaced `MeshLinkKey(SharedState<BRepMeshEdge>)` (a per-edge `RwLock` acquired on every hash, equality check, `get_link`, and `add_link`) with `{ first_node: i32, last_node: i32, movability: AtomicU8 }`.
   - Combined `BRepMeshDataStructureOfDelaun::element_nodes` into a single `connectivity.read()` lock instead of two `get_link` calls, and added single-lock `link_movabilities_3`.
2. **Reduced vertex lookup locking (`martensite-tkmesh/src/brep_mesh/vertex_tool/`, `delaun/`, `mesh_algo/`)**:
   - Cached `MeshVertexCollectionSlot` directly on `BRepMeshVertexTool` so `find_key`, `extent`, `vertices`, and `change_vertices` skip the outer `VertexToolState` lock.
   - Eliminated per-lookup `Handle<MeshVertexCollection>` `Arc` clone/drop in `BRepMeshVertexInspector::get_vertex`, `set_vertex`, and `nb_vertices`.
   - Added batched `find_keys_3` / `get_nodes_3` and used them in `DeflectionPass::triangle_info` (also removing the duplicate `get_node` call via `node_point`), `BRepMeshDelaun::fan_side_metrics`, `add_fan_triangle`, and `add_triangle`.
3. **Zero-copy `CellFilter` / `CellMap` lookups and 2D fast path (`martensite-tkernel/src/ncollection/cell_filter/`, `chained_slots/lookup.rs`)**:
   - Added `ChainedSlots::lookup_by` and `lookup_with_bucket_by` so `CellMap::lookup_index` and `prepare_cell` hash `&[i32]` directly without constructing a 56-byte `CellRecord`/`CellIndex` on every lookup or existing-cell insertion.
   - Added `CellIndices::for_each_slice` and a direct 2D nested-loop fast path in `CellFilter::add_range`, `remove_range`, and `try_inspect_with`, eliminating per-cell `CellIndex` construction, cloning, and zeroing.
4. **Zero-clone `BRepTool` curve representation queries (`martensite-tkbrep/src/brep/tool/edge.rs`, `closed.rs`)**:
   - Migrated `BRepTool::range`, `range_on_surface`, `uv_points`, `set_uv_points`, `is_closed_on_surface`, and `is_closed_on_triangulation` from cloning `Vec<Handle<dyn BRepCurveRepresentation>>` to borrowing via `BRepTEdge::with_curves`.

## Remaining Gap Analysis

- **Portable transcendentals (`#201`)**: `metallic::f64_::double::DoubleDouble` (`sin`/`cos` during surface & curve evaluation in `mesh_sphere`, `mesh_torus`, `mesh_cylinder`, and `make_revol`) accounts for ~6–8% of total CPU time.
- **Thread-safe `SharedState` + `Handle` (`Arc`)**: Remaining per-call `RwLock` synchronization on `BRepMeshVertexTool`, `BRepMeshDataStructureOfDelaun`, and `BRepTEdge`/`BRepTFace` accounts for the ~2.0–2.8x floor relative to OCCT's unsynchronized raw pointers.

## Verification

- `cargo test --lib -p martensite-tkmesh -p martensite-tkprim -p martensite-tkbrep`: all passed.
- `cargo test --lib -p martensite-tkernel -- cell_filter`: 69/69 passed.
- `cargo clippy --lib -p martensite-tkernel -p martensite-tkbrep -p martensite-tkprim -p martensite-tkmesh -- -D warnings`: clean.
