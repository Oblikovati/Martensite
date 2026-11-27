# Tier 8: `martensite-tktopalgo` vs OCCT `TKTopAlgo` (p95)

Benchmarked against OCCT 8.0.0 Release (`libTKTopAlgo.so`, `g++ -O3 -DNDEBUG -std=c++17`) vs Martensite (`--release`, `opt-level = 3`), 100 samples per API, all 8/8 checksums verified identical.

## Baseline vs Optimized p95 Results

| Public API | OCCT p95 (ns) | Baseline Martensite (ns) | Optimized Martensite (ns) | Speedup | Ratio (M/O) | Status | Checksum |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- |
| `BRepGProp::volume_props_box` | 53,105.3 | 508,595.7 | 397,449.0 | **1.28x** | 7.48x | SLOWER | OK |
| `BRepBuilderAPI::transform_box` | 24,828.0 | 128,864.5 | 130,289.0 | — | 5.25x | SLOWER | OK |
| `BRepGProp::surface_props_box` | 98,805.4 | 390,882.8 | 395,258.0 | — | 4.00x | SLOWER | OK |
| `BRepBndLib::add_box` | 4,082.8 | 17,946.2 | 16,259.5 | **1.10x** | 3.98x | SLOWER | OK |
| `BRepCheck::analyzer_box` | 208,522.9 | 872,964.4 | 788,484.1 | **1.11x** | 3.78x | SLOWER | OK |
| `BRepExtrema::dist_box_box` | 1,352,003.8 | 3,862,383.6 | 4,383,635.8 | — | 3.24x | SLOWER | OK |
| `BRepBuilderAPI::make_poly_face` | 45,178.0 | 142,201.2 | 141,973.9 | — | 3.14x | SLOWER | OK |
| `BRepBuilderAPI::sew_box_solid` | 747,148.6 | 2,429,930.8 | 2,283,810.4 | **1.06x** | 3.06x | SLOWER | OK |

## Optimizations Applied

1. **Stack-allocated Gauss-Kronrod quadrature tables (`martensite-tkmath/src/math/kronrod_integration.rs`, `kronrod_integration/adaptive.rs`):**
   - Replaced 4 per-call heap `Vector` allocations (`gp`, `gw`, `kp`, `kw`) in `KronrodSingleIntegration::perform` / `perform_with_tolerance` with stack `[f64; 64]` / `[f64; 128]` buffers for all tabulated orders (`<= 123`), and guarded `Intervals` allocation in `refine`.
   - Improved `BRepGProp::volume_props_box` by **1.28x** (508.6 µs → 397.4 µs, 10.07x → 7.48x).
2. **Direct-buffer `gauss_points_and_weights` (`martensite-tktopalgo/src/brep_gprop/gauss_table.rs`):**
   - Populated the returned `Vec<f64>` directly via `Vector::from_buffer`, removing 2 redundant heap `Vector` allocations and copies per Gauss rule lookup.
3. **Zero-adaptor planar face check + lazy triangulation + zero-clone `Explorer` loops (`martensite-tktopalgo/src/brep_bnd_lib/add.rs`):**
   - Skipped `BRepTool::triangulation` when `!use_triangulation && !surface.is_null()`.
   - Tested `surface.is_kind(GeomPlane::static_type())` directly instead of constructing a full `BRepAdaptorSurface` just to test `SurfaceType::Plane`.
   - Replaced `for shape in Explorer` (cloning every `Shape`) with `while explorer.more() { ... explorer.next()?; }` (**1.10x** faster on `BRepBndLib::add_box`).

## Remaining Gap Analysis

- **`BRepGProp::volume_props_box` (7.48x):** By design, Martensite's `BRepGProp::volume_properties` routes geometric solids through error-bounded adaptive Gauss-Kronrod integration (`complete_volume_gk` with `GEOMETRIC_VOLUME_ERROR_LIMIT = 1e-5`, per the kernel's error-bounded integration invariant), whereas OCCT's `BRepGProp::VolumeProperties` uses non-adaptive fixed Gauss quadrature.
- **`BRepBuilderAPI::transform_box` (5.25x) / `BRepBndLib::add_box` (3.98x) / `BRepCheck` (3.78x):** Bounded by `TShape`/`BRep` `SharedState` locking and `Handle` `Arc` allocations during topology copy and edge/face adaptor construction.
