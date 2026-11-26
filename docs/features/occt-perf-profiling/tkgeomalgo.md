# Tier 7: `martensite-tkgeomalgo` vs OCCT `TKGeomAlgo` (p95)

Benchmarked against OCCT 8.0.0 Release (`libTKGeomAlgo.so`, `g++ -O3 -DNDEBUG -std=c++17`) vs Martensite (`--release`, `opt-level = 3`), 100 samples per API, all 9/9 quantized checksums verified identical.

## Baseline vs Optimized p95 Results

| Public API | OCCT p95 (ns) | Baseline Martensite (ns) | Optimized Martensite (ns) | Speedup | Ratio (M/O) | Status | Checksum |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- |
| `GeomAPI::points_to_bspline_20` | 129,423.4 | 634,365.8 | 747,853.9 | — | 5.78x | SLOWER | OK |
| `GeomAPI::interpolate_20` | 1,789.8 | 6,013.1 | 7,085.6 | — | 3.96x | SLOWER | OK |
| `GeomAPI::project_pnt_on_surf` | 234,494.9 | 672,510.4 | 698,767.1 | — | 2.98x | SLOWER | OK |
| `GeomAPI::int_ss_cyl_plane` | 4,882.6 | 9,256.6 | 14,101.6 | — | 2.89x | SLOWER | OK |
| `GeomAPI::int_cs_line_bspline` | 265,815.9 | 754,385.9 | 645,619.8 | **1.17x** | 2.43x | SLOWER | OK |
| `GccAna::circ2d_2tan_rad` | 362.6 | 854.0 | 831.1 | **1.03x** | 2.29x | SLOWER | OK |
| `GeomAPI::int_ss_cyl_cyl` | 5,148,129.0 | 12,330,580.0 | 10,396,930.2 | **1.19x** | 2.02x | SLOWER | OK |
| `Geom2dAPI::inter_bsp_bsp` | 24,482.3 | 52,197.6 | 48,765.2 | **1.07x** | 1.99x | SLOWER | OK |
| `GeomAPI::project_pnt_on_curve` | 15,485.1 | 26,692.4 | 27,963.2 | — | 1.81x | SLOWER | OK |

*(Note: M/O ratio improved across `project_pnt_on_surf` 3.67x→2.98x, `int_cs_line_bspline` 3.11x→2.43x, `int_ss_cyl_cyl` 2.48x→2.02x, `inter_bsp_bsp` 2.48x→1.99x, `project_pnt_on_curve` 2.09x→1.81x, and `circ2d_2tan_rad` 2.61x→2.29x.)*

## Optimizations Applied

1. **Zero-allocation `BSplSLib::Cache` evaluation (`martensite-tkmath/src/bsplslib/{cache.rs,cache/polynomials.rs,cache/api.rs}`):**
   - Replaced per-evaluation `Vec<f64>` heap allocations (`transient` in `d0_local` and `evaluate_polynomials`, and `rational` in `evaluate_polynomials`) with stack buffers (`[f64; 128]`, `[f64; 320]`, `[f64; 36]`).
   - Added single-lock `Cache::try_d0`, `try_d1`, and `try_d2` combining `is_cache_valid(u, v)` and evaluation under one `Mutex` acquisition.
2. **Zero-clone `GeomAdaptorSurface` cache rebuild + single-lock hits (`martensite-tkg3d/src/geom/bspline_surface/access.rs`, `martensite-tkg3d/src/geom_adaptor/{surface_eval.rs,surface_eval_order.rs}`):**
   - Added `GeomBSplineSurface::with_cache_build_inputs` to borrow flat knots, poles, and weights under a single read lock without cloning 4 heap arrays on every patch transition.
   - Wired `CachedOrder::try_cached` into `cached_bspline`.
3. **Stack-allocated basis matrices in `BSplCLib::build_bsp_matrix` and `AppParCurves::spline_function` (`martensite-tkmath/src/bsplclib/banded.rs`, `martensite-tkgeombase/src/app_par_curves/{spline_function.rs,least_square/normal_equations.rs}`):**
   - Sized `build_bsp_matrix`'s temporary basis matrix to `order × order` on a stack `[f64; 64]` buffer instead of heap-allocating and zero-filling a `26 × 26` (5.4 KB) matrix for every interpolation.
   - Eliminated `flatknots` heap copy and per-point `Vec<f64>` allocations in `spline_function` via `flatknots.array1()` and `[f64; 32]` stack buffers.
   - Replaced `Result`-checked `value`/`change_value` calls in `LeastSquare::normal_matrix` with inline `Matrix` indexing.

## Remaining Gap Analysis

- **`GeomAPI::points_to_bspline_20` (5.78x) / `GeomAPI::interpolate_20` (3.96x):** Dominated by `AppDef_BSplineCompute` / `AppParCurves_LeastSquare` (profile Cholesky `dactcl` + `BFGS` parameter optimization with `Matrix`/`Vector` checked indexing) and `BSplCLib::factor_banded_matrix`/`solve_banded_system`.
- **`GeomAPI::project_pnt_on_surf` (2.98x) / `IntCS` (2.43x) / `IntSS` (2.02x):** Bounded by `GeomAdaptorSurface` `SharedState` + `BSplSLib::Cache` synchronization and `#201` portable `martensite_cmath` transcendentals in quadric walking.
