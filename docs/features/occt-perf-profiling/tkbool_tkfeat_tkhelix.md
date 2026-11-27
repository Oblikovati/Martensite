# `martensite-tkbool` + `martensite-tkfeat` + `martensite-tkhelix` vs OCCT `TKBool` + `TKFeat` + `TKHelix` (p95 Profiling)

Benchmarked against OCCT 8.0.0 Release (`/git/oblikovati-workspace/occt-install-vis`), 50 samples per API (p95 in nanoseconds).

## Results (Before vs After Optimization)

| Public API | OCCT p95 (ns) | Baseline M p95 (ns) | Optimized M p95 (ns) | Speedup | Final Ratio (M/O) | Checksum |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `BRepFeat::make_prism_boss` | 1,001,153.0 | 2,573,578.0 | 2,566,139.5 | ~1.0x | 2.56x | OK |
| `HelixBRep::spiral_5turns` | 57,095.7 | 173,775.0 | 169,944.3 | **1.02x** | 2.98x | OK |
| `BRepAlgo::normal_proj_cyl` | 413,707.8 | 1,233,775.4 | 1,235,616.8 | ~1.0x | 2.99x | OK |
| `HelixBRep::helix_3turns` | 15,009.8 | 46,926.8 | 46,251.6 | **1.01x** | 3.08x | OK |
| `BRepFill::pipe_circle` | 50,721.8 | 156,433.4 | 157,894.4 | ~1.0x | 3.11x | OK |

*(These toolkits already benefit directly from the earlier optimizations in `tkernel`, `tkmath`, `tkg3d`, `tkgeombase`, `tkbrep`, `tkgeomalgo`, and `tktopalgo`.)*

## Key Optimization Applied

1. **Combined `sincos` in `martensite-tkhelix` (`helix_geom/helix_curve.rs`, `authored/law.rs`, `authored/sequence/law.rs`)**:
   - Replaced separate `martensite_cmath::cos(u)` and `martensite_cmath::sin(u)` calls in `HelixCurve::eval_d0`/`eval_d1`/`eval_d2` and the authored helix laws with `martensite_cmath::sincos(u)`.

## Verification

- `cargo test --lib -p martensite-tkhelix`: all passed.
- `cargo clippy --lib -p martensite-tkhelix -- -D warnings`: clean.
