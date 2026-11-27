# Tier 3: `martensite-tkg2d` vs OCCT `TKG2d` (p95 Profiling & Optimization)

- **OCCT baseline:** `/git/oblikovati-workspace/occt-install-vis` (OCCT 8.0.0 Release, `g++ -O3 -DNDEBUG -std=c++17`)
- **Martensite:** worktree `perf/toolkit-occt-p95` (Rust `--release`, `opt-level = 3`)
- **Methodology:** 200 independent samples per API, anti-hoisting guards, and quantized checksum verification across all 12 APIs.

## Results (Before vs After Optimization)

| Public API | OCCT p95 (ns) | Before (ns) | After (ns) | Speedup | Ratio (M/O) | Status | Notes |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- |
| `Geom2d_Circle::d1` | 13.6 | 104.4 | 80.1 | 1.30x | 5.90x | SLOWER | Switched `elclib::circle_d1_2d` to `sincos`; remaining gap is `#201` portable `sincos` + `SharedState` read lock |
| `Geom2d_Ellipse::d2` | 12.5 | 79.4 | 63.7 | 1.25x | 5.08x | SLOWER | Switched `elclib::ellipse_d2_2d` to `sincos`; remaining gap is `#201` portable `sincos` + `SharedState` read lock |
| `Geom2d_BSplineCurve::insert_knot` | 452.4 | 1,984.9 | 2,081.2 | 0.95x | 4.60x | SLOWER | Benchmark constructs fresh curve vs OCCT `Copy()` |
| `Geom2d_Line::d1` | 3.7 | 14.1 | 11.2 | 1.26x | 3.02x | SLOWER | `SharedState` `RwLock::read()` (~7 ns) for thread-safe interior mutability |
| `Geom2d_BSplineCurve::d0_deg3_20p` | 60.7 | 327.8 | 163.8 | 2.00x | 2.70x | SLOWER | Stack-allocated `bsplclib::cache_eval`, skipped periodic normalization when non-periodic |
| `Geom2d_BSplineCurve::d1_100_samples` | 11,940.1 | 32,252.2 | 27,595.5 | 1.17x | 2.31x | SLOWER | |
| `Geom2dGridEval_BSpline::d1_100` | 6,883.5 | 18,279.2 | 13,039.3 | 1.40x | 1.89x | SLOWER | Const-generic Horner `value_and_first_fixed` + stack `cache_eval` |
| `Geom2dAdaptor_Curve::d1_100_samples` | 5,009.8 | 14,699.6 | 9,207.0 | 1.60x | 1.84x | SLOWER | Stack-allocated `bsplclib::cache_eval` + `value_and_first_fixed` |
| `Geom2d_BezierCurve::d1_deg5` | 148.7 | 328.2 | 268.8 | 1.22x | 1.81x | SLOWER | Stack-allocated `cache_eval` |
| `Geom2d_BSplineCurve::d1_deg3_20p` | 124.3 | 391.7 | 209.4 | 1.87x | 1.69x | SLOWER | Skipped redundant periodic normalization + `value_and_first_fixed` |
| `Geom2d_BSplineCurve::d2_deg3_20p` | 153.4 | 598.6 | 235.6 | 2.54x | 1.54x | SLOWER | Stack-allocated `bsplclib` buffers + `value_to_second_fixed` |
| `Geom2d_OffsetCurve::d1_bspline` | 167.3 | 238.2 | 229.7 | 1.04x | 1.37x | SLOWER | |

**Summary:** 12 APIs benchmarked (all 12 checksum matches, 16/16 `martensite-tkg2d` lib tests passing). Achieved **2.54x** on `Geom2d_BSplineCurve::d2`, **2.00x** on `d0`, **1.87x** on `d1`, **1.60x** on `Geom2dAdaptor_Curve::d1_100_samples`, and **1.40x** on `Geom2dGridEval_BSpline::d1_100`.
