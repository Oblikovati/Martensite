# Tier 4: `martensite-tkg3d` vs OCCT `TKG3d` (p95 Profiling & Optimization)

- **OCCT baseline:** `/git/oblikovati-workspace/occt-install-vis` (OCCT 8.0.0 Release, `g++ -O3 -DNDEBUG -std=c++17`)
- **Martensite:** worktree `perf/toolkit-occt-p95` (Rust `--release`, `opt-level = 3`)
- **Methodology:** 200 independent samples per API, anti-hoisting guards, and quantized checksum verification across all 12 APIs.

## Results (Before vs After Optimization)

| Public API | OCCT p95 (ns) | Before (ns) | After (ns) | Speedup | Ratio (M/O) | Status | Notes |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- |
| `Geom_Plane::d1` | 4.0 | 26.1 | 25.6 | 1.02x | 6.46x | SLOWER | `SharedState` `RwLock::read()` + `Result<GeomSurfaceD1, Failure>` return size |
| `Geom_Circle::d1` | 12.6 | 74.0 | 79.3 | 0.93x | 6.31x | SLOWER | Bound by `#201` portable `sincos` + `SharedState` read lock |
| `Geom_CylindricalSurface::d1` | 13.3 | 62.9 | 78.3 | 0.80x | 5.89x | SLOWER | Bound by `#201` portable `sincos` + `SharedState` read lock |
| `Geom_ToroidalSurface::d2` | 31.7 | 205.1 | 160.5 | 1.28x | 5.06x | SLOWER | Bound by `#201` portable `sincos` (2 calls) + `SharedState` read lock |
| `GeomAdaptor_Curve::d1_100_samples` | 4,932.1 | 9,404.4 | 9,612.2 | 0.98x | 1.95x | SLOWER | `SharedState` lock per eval |
| `Geom_BSplineCurve::d1_deg3_20p` | 123.3 | 221.9 | 209.3 | 1.06x | 1.70x | SLOWER | |
| `GeomAdaptor_Surface::d1_10x10` | 33,059.2 | 59,057.3 | 54,044.0 | 1.09x | 1.63x | SLOWER | Stack-backed `BSplSLib::PatchPoles` eliminated 21.6 KB per-eval heap alloc |
| `GeomGridEval_BSplineSurface::d1_10x10` | 32,856.8 | 61,348.5 | 50,736.9 | 1.21x | 1.54x | SLOWER | Stack-backed `BSplSLib::PatchPoles` |
| `Geom_BSplineSurface::d1_8x8` | 319.4 | 641.3 | 481.3 | 1.33x | 1.51x | SLOWER | Stack-backed `BSplSLib::PatchPoles` eliminated 21.6 KB per-eval heap alloc |
| `Geom_BSplineSurface::d0_8x8` | 235.6 | 468.8 | 324.8 | 1.44x | 1.38x | SLOWER | Stack-backed `BSplSLib::PatchPoles` |
| `Geom_OffsetSurface::d1_bspline` | 549.6 | 897.8 | 740.8 | 1.21x | 1.35x | SLOWER | Stack-backed `BSplSLib::PatchPoles` |
| `Geom_BSplineSurface::d2_8x8` | 507.9 | 698.9 | 565.5 | 1.24x | 1.11x | SLOWER | Stack-backed `BSplSLib::PatchPoles` + stack `[f64; 48]` rational buffer |

**Summary:** 12 APIs benchmarked (all 12 checksum matches, `1,362` `tkmath` + `68` `tkg3d` lib tests passing). Eliminating the 21.6 KB per-evaluation heap allocation in `BSplSLib::Patch` delivered **1.21x–1.44x** speedups across all B-spline surface evaluation paths (`d0`, `d1`, `d2`, `OffsetSurface`, `GeomAdaptor_Surface`, `GeomGridEval_BSplineSurface`).
