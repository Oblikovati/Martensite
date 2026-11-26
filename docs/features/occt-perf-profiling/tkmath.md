# Tier 2: `martensite-tkmath` vs OCCT `TKMath` (p95 Profiling & Optimization)

- **OCCT baseline:** `/git/oblikovati-workspace/occt-install-vis` (OCCT 8.0.0 Release, `g++ -O3 -DNDEBUG -std=c++17`)
- **Martensite:** worktree `perf/toolkit-occt-p95` (Rust `--release`, `opt-level = 3`)
- **Methodology:** 200 independent samples per API (with warmup, `black_box` / `asm volatile` anti-hoisting, and quantized semantic checksum verification across all 24 APIs).

## Results (Before vs After Optimization)

| Public API | OCCT p95 (ns) | Before (ns) | After (ns) | Speedup | Ratio (M/O) | Status | Notes |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- |
| `math_Poly::quartic` | 459.8 | 639,623.5 | 116,220.4 | 5.50x | 252.76x | SLOWER | Early-exit bisection, value-only Horner, `scale(-1)`; remaining gap is software `rustc_apfloat` 80-bit `LongDouble` vs hardware x87 |
| `gp_Mat::inverted` | 5.7 | 26.6 | 24.8 | 1.07x | 4.32x | SLOWER | `Result<Matrix3, Failure>` return size overhead vs OCCT value return |
| `ElSLib::cylinder_d1` | 13.0 | 127.7 | 55.2 | 2.31x | 4.25x | SLOWER | Switched to `sincos` + `#[inline]`; remaining gap is `#201` portable `martensite_cmath::sincos` vs glibc |
| `ElCLib::ellipse_d2` | 14.4 | 63.8 | 52.4 | 1.22x | 3.62x | SLOWER | Switched to `sincos` + `#[inline]`; bound by `#201` portable `sincos` |
| `ElSLib::torus_d2` | 35.1 | 133.4 | 113.0 | 1.18x | 3.22x | SLOWER | Switched to `sincos` + `#[inline]`; bound by `#201` portable `sincos` (2 calls) |
| `ElCLib::circle_d1` | 19.1 | 74.6 | 53.0 | 1.41x | 2.77x | SLOWER | Switched to `sincos` + `#[inline]`; bound by `#201` portable `sincos` |
| `math_SVD::8x8` | 3,210.7 | 25,259.9 | 8,164.9 | 3.09x | 2.54x | SLOWER | Inlined `Matrix`/`DoubleTab`/`Vector` indexing + slice-indexed `rotate_columns` |
| `math_Jacobi::6x6` | 1,669.3 | 9,213.5 | 3,715.2 | 2.48x | 2.23x | SLOWER | Inlined `Matrix`/`DoubleTab`/`Vector` indexing + slice-indexed `rotate` |
| `BSplCLib::locate_parameter_50` | 12.8 | 24.2 | 28.3 | 0.86x | 2.21x | SLOWER | Checked array access |
| `TopLoc_Location::mult_and_trsf` | 71.5 | 138.4 | 151.2 | 0.92x | 2.11x | SLOWER | `Arc`/`Handle` datum list composition |
| `gp_Trsf::multiplied` | 12.5 | 23.7 | 24.9 | 0.95x | 1.99x | SLOWER | Inlined `Matrix3` ops and `Transform3::multiplied` |
| `BSplCLib::eval_bspline_basis_d3` | 30.3 | 72.3 | 60.0 | 1.21x | 1.98x | SLOWER | Direct slice indexing in `BasisRecurrence::scale` |
| `PLib::eval_poly_2var_4x4` | 46.5 | 99.0 | 89.8 | 1.10x | 1.93x | SLOWER | |
| `PLib::eval_polynomial_d7` | 9.8 | 68.7 | 17.7 | 3.88x | 1.81x | SLOWER | Const-generic `dim = 1..4` register specialization + `#[inline]` |
| `BSplSLib::d1_bicubic_8x8` | 377.6 | 966.7 | 622.6 | 1.55x | 1.65x | SLOWER | Stack `knots1`/`knots2` arrays + removed `patch.poles.clone()` |
| `BSplCLib::bezier_d1_deg5` | 160.4 | 303.3 | 243.8 | 1.24x | 1.52x | SLOWER | Stack-allocated `LocalSpan` buffers |
| `BSplCLib::d1_deg3_20p` | 122.3 | 393.1 | 179.2 | 2.19x | 1.47x | SLOWER | Stack-allocated `LocalSpan` knots/poles + `[f64; 16]` derivatives |
| `gp_Pnt::transformed_1000` | 2,783.4 | 9,696.3 | 3,966.9 | 2.44x | 1.43x | SLOWER | Inlined `Point3::transform`, `Transform3::apply`, `apply_coordinates`, `Matrix3` ops |
| `ElSLib::sphere_parameters` | 51.7 | 90.0 | 71.5 | 1.26x | 1.38x | SLOWER | |
| `math_Gauss::solve_8x8` | 252.0 | 3,096.8 | 326.9 | 9.47x | 1.30x | SLOWER | Direct slice-based LU elimination and forward/back substitution |
| `gp_Trsf::inverted` | 16.3 | 20.2 | 20.7 | 0.98x | 1.27x | SLOWER | |
| `Bnd_Box::add_and_is_out_1000` | 3,429.0 | 3,530.7 | 3,473.4 | 1.02x | 1.01x | PARITY | |
| `math_Gauss::solve_32x32` | 4,095.6 | 74,150.1 | 3,955.8 | 18.74x | 0.97x | PARITY | Direct slice-based LU elimination (AVX2 auto-vectorized) |
| `Bnd_OBB::is_out_obb` | 92.8 | 93.2 | 88.2 | 1.06x | 0.95x | FASTER | |

**Summary:** 24 APIs benchmarked (all 24 checksum matches, 1,362/1,362 `martensite-tkmath` tests passing). Achieved **18.7x** on `math_Gauss::solve_32x32`, **9.5x** on `math_Gauss::solve_8x8`, **5.5x** on `math_Poly::quartic`, **3.9x** on `PLib::eval_polynomial`, **3.1x** on `math_SVD`, **2.5x** on `math_Jacobi`, **2.4x** on `gp_Pnt::transformed`, **2.3x** on `ElSLib::cylinder_d1`, and **2.2x** on `BSplCLib::d1`.
