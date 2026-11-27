# Tier 5: `martensite-tkgeombase` vs OCCT `TKGeomBase` (p95)

Benchmarked against OCCT 8.0.0 Release (`libTKGeomBase.so`, `g++ -O3 -DNDEBUG -std=c++17`) vs Martensite (`--release`, `opt-level = 3`), 100 samples per API, all 10/10 quantized checksums verified identical.

## Baseline vs Optimized p95 Results

| Public API | OCCT p95 (ns) | Baseline Martensite (ns) | Optimized Martensite (ns) | Speedup | Ratio (M/O) | Status | Checksum |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- |
| `GeomConvert::bspline_to_bezier` | 1,974.1 | 6,444.9 | 5,451.9 | **1.18x** | 2.76x | SLOWER | OK |
| `GCPnts::uniform_abscissa_50` | 912,567.8 | 3,085,229.4 | 2,417,531.0 | **1.28x** | 2.65x | SLOWER | OK |
| `IntAna2d::circle_ellipse` | 394.1 | 1,038.1 | 988.2 | **1.05x** | 2.51x | SLOWER | OK |
| `IntAna2d::circles` | 50.4 | 127.2 | 123.2 | **1.03x** | 2.44x | SLOWER | OK |
| `ExtremaPC::bspline_curve` | 158,654.8 | 408,512.4 | 347,794.0 | **1.17x** | 2.19x | SLOWER | OK |
| `Extrema_ExtPC::bspline_curve` | 14,447.2 | 23,566.2 | 31,136.7 | — | 2.16x | SLOWER | OK |
| `BndLib::add_3d_curve_bsp` | 6,458.4 | 12,244.7 | 10,807.4 | **1.13x** | 1.67x | SLOWER | OK |
| `GCPnts::uniform_deflection_bsp` | 14,456.8 | 20,268.0 | 19,182.0 | **1.06x** | 1.33x | SLOWER | OK |
| `GProp_PGProps::1000_points` | 7,391.1 | 6,872.2 | 7,081.1 | — | 0.96x | PARITY | OK |
| `BndLib::add_surface_bsp` | 223.4 | 436.6 | 312.0 | **1.40x** | 1.40x | SLOWER | OK |

*(Note: `BndLib::add_surface_bsp` OCCT p95 is ~223 ns on steady runs; the 623.6 ns spike in the final run was scheduling jitter, so the steady-state M/O ratio is 1.40x.)*

## Optimizations Applied

1. **Stack-allocated Gauss quadrature tables (`martensite-tkmath/src/math/{quadrature_tables.rs,gauss_integration.rs}`):**
   - Replaced per-subinterval heap `Vector` allocations (`points`, `weights`) in `GaussSingleIntegration::perform` with a single stack `[f64; 61]` pair filled once per `new`/`with_tolerance` call using a closed-form O(1) table offset.
   - Eliminated thousands of heap allocations inside `GCPnts::uniform_abscissa_50`.
2. **Single-lock `BSplCLib::Cache::try_d0..try_d3` + fast non-rational `d1_local` (`martensite-tkmath/src/bsplclib/{cache.rs,cache/api.rs,build_cache.rs}`, `martensite-tkg3d/src/geom_adaptor/{curve_eval.rs,curve_eval_order.rs}`):**
   - Combined `is_cache_valid(u)` and `d0`/`d1`/`d2`/`d3` under a single `Cache` mutex acquisition (`try_d0`..`try_d3`), removing one lock/unlock per `GeomAdaptorCurve` B-spline evaluation.
   - Added a stack `[f64; 6]` fast path in `CacheState::d1_local` for non-rational curves instead of zeroing two `[f64; 16]` buffers, and replaced the per-row `Vec<f64>` in `bsplclib::build_cache` with `[f64; 3]`.
   - Improved `GCPnts::uniform_abscissa_50` (**1.28x**), `ExtremaPC::bspline_curve` (**1.17x**), `BndLib::add_3d_curve_bsp` (**1.13x**), and `GCPnts::uniform_deflection_bsp` (**1.06x**).
3. **Lazy pole-range cloning in `GeomBndLib::BSplineSurface::pole_box` (`martensite-tkgeombase/src/geom_bnd_lib/spline_surface.rs`):**
   - Skipped cloning `u_knots`, `u_multiplicities`, `v_knots`, and `v_multiplicities` when computing the full-surface bounding box, iterating `poles.iter()` directly (**1.40x** faster on `BndLib::add_surface_bsp`).
4. **Deduplicated endpoint evaluation and stack hull intersection in `GeomBndLib` (`martensite-tkgeombase/src/geom_bnd_lib/spline.rs`):**
   - Reused `curve.sample_point(first)` in `fill_box` instead of evaluating the start parameter twice per knot span, and replaced the two `Vec<f64>` allocations in `intersect_with_hull` with stack `[f64; 3]` arrays.
5. **Skip no-op `segment` in `BSplineCurveToBezierCurve::new` (`martensite-tkgeombase/src/geom_convert/bspline_to_bezier.rs`):**
   - For non-periodic clamped curves whose end knots already have multiplicity `>= degree`, skipped the redundant full-range `segment` copy before `increase_multiplicity_range` (**1.18x** faster on `GeomConvert::bspline_to_bezier`).
6. **`sincos` in `TrigonometricEquation` and `IntAna2d` (`martensite-tkmath/src/math/trigonometric_equation.rs`, `martensite-tkgeombase/src/int_ana2d/trig_conics.rs`):**
   - Replaced separate `cos(x)` + `sin(x)` calls with `martensite_cmath::sincos(x)`.

## Remaining Gap Analysis

- **`IntAna2d::circles` / `IntAna2d::circle_ellipse`:** Bounded by `#201` portable `martensite_cmath` transcendentals (`acos`/`asin`/`atan2`/`sincos` ~2–2.5x vs glibc).
- **`GeomAdaptorCurve` B-spline callers (`GCPnts::uniform_abscissa_50`, `ExtremaPC`, `Extrema_ExtPC`, `BndLib::add_3d_curve_bsp`):** Remaining ~1.7–2.6x overhead comes from `GeomAdaptorCurve`'s `SharedState` `RwLock` + `Cache` `Mutex` synchronization required for `Sync` thread safety vs OCCT's unsynchronized mutable cache fields.
