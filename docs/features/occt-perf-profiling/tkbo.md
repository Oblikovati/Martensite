# `martensite-tkbo` vs OCCT `TKBO` (p95 Profiling)

Benchmarked against OCCT 8.0.0 Release (`/git/oblikovati-workspace/occt-install-vis`), 100 samples per API (p95 in nanoseconds).

## Results (Before vs After Optimization)

| Public API | OCCT p95 (ns) | Baseline M p95 (ns) | Optimized M p95 (ns) | Speedup | Final Ratio (M/O) | Checksum |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `BRepAlgoAPI::fuse_box_box` | 1,978,399.4 | 7,193,281.0 | 5,354,031.0 | **1.34x** | 2.71x | OK |
| `BRepAlgoAPI::cut_box_box` | 1,722,025.0 | 6,427,317.4 | 5,029,392.2 | **1.28x** | 2.92x | OK |
| `BRepAlgoAPI::cut_box_cylinder` | 1,305,830.0 | 4,477,339.7 | 3,665,744.3 | **1.22x** | 2.81x | OK |
| `BRepAlgoAPI::cut_cylinder_sphere` | 15,207,164.7 | 73,667,710.7 | 62,036,504.3 | **1.19x** | 4.08x | OK |
| `BRepAlgoAPI::section_box_sphere` | 295,623.7 | 898,239.3 | 767,443.7 | **1.17x** | 2.60x | OK |
| `BRepAlgoAPI::fuse_box_cylinder` | 1,239,150.0 | 4,210,145.3 | 3,864,115.0 | **1.09x** | 3.12x | OK |
| `BRepAlgoAPI::common_box_box` | 1,687,984.0 | 5,791,027.8 | 5,654,118.2 | **1.02x** | 3.35x | OK |
| `BRepAlgoAPI::section_box_box` | 867,978.4 | 2,206,363.4 | 2,229,129.6 | ~1.0x | 2.57x | OK |

## Key Optimizations Applied

1. **Zero-allocation `AppParCurves::bernstein` & `LeastSquare::errors` (`martensite-tkgeombase/src/app_par_curves/bernstein.rs`, `least_square/errors.rs`)**:
   - Replaced per-parameter heap `Vector<'static>` allocations in `bernstein`, `bernstein_matrix`, and `second_derivative_bernstein` with a reusable 32-element stack buffer (`fill_lower_basis`).
   - Removed the `Vec<Vec<f64>>` pole copy and per-point `offsets: Vec<f64>` heap allocation in `LeastSquare::errors` / `point_error`, using `[0.0; 3]` on the stack and direct matrix indexing into `self.mypoles`.
2. **Inlined `PLib::no_derivative_eval_polynomial`, `VectorBase`, and `DoubleTab` accessors (`martensite-tkmath/src/plib/horner.rs`, `plib/access.rs`, `math/vector.rs`, `math/double_tab.rs`)**:
   - Marked `plib::no_derivative_eval_polynomial`, `non_negative`, and `require_length` `#[inline]` with `#[cold] #[inline(never)]` error builders, removing ~5.7% call and check overhead in Boolean curve evaluation.
   - Marked `VectorBase::{at, change_at, value, change_value}` and `DoubleTab::{at, change_at}` `#[inline]`, eliminating ~4.1% call overhead in `Pso` and `LeastSquare` loops.
3. **Zero-allocation empty `FailureText` (`martensite-tkernel/src/standard/failure/text.rs`, `failure.rs`)**:
   - Added `FailureText::Empty` and `FailureText::from_str` so empty stack traces (`""`) and valid Rust `&str` messages avoid heap `Arc<str>` allocation and redundant `from_utf8` validation.

## Remaining Gap Analysis

- **Portable transcendentals (`#201`)**: `metallic::f64_::double::DoubleDouble` (`sin`/`cos`/`atan2`/`rem_pio2`) accounts for ~15% of total CPU time in curved Boolean intersections (`cut_cylinder_sphere`, `cut_box_cylinder`).
- **Thread-safe `SharedState` + `Handle` (`Arc`)**: Remaining `RwLock` synchronization on `GeomAdaptorCurve`/`Surface`, `CurveOnSurface`, and BRep topology accounts for the ~2.6–3.3x floor on planar Booleans.

## Verification

- `cargo test --lib -p martensite-tkbo -p martensite-tkgeombase -- app_par_curves`: passed.
- `cargo test --lib -p martensite-tkernel -- failure`: passed.
- `cargo test --lib -p martensite-tkmath -- plib`: passed.
- `cargo clippy --lib -p martensite-tkernel -p martensite-tkmath -p martensite-tkgeombase -p martensite-tkbo -- -D warnings`: clean.
