# Application Framework & Visualization (`martensite-tklcaf`, `martensite-tkservice`) vs OCCT (p95 Profiling)

Benchmarked against OCCT 8.1.0 Release (`/tmp/occt-build-afde`), 100 samples per API (p95 in nanoseconds).

## Results (Before vs After Optimization)

| Public API | OCCT p95 (ns) | Baseline M p95 (ns) | Optimized M p95 (ns) | Speedup | Final Ratio (M/O) | Checksum |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `Graphic3d::camera_project_1000` | 6,887.6 | 57,817.7 | 17,900.3 | **3.23x** | 2.60x | OK |
| `Graphic3d::camera_matrices_100` | 9,267.2 | 14,252.4 | 11,780.4 | **1.21x** | 1.27x | OK |
| `TDF::lookup_500_labels` | 13,431.4 | 105,315.6 | 89,700.1 | **1.17x** | 6.68x | OK |
| `TDF::build_500_labels` | 35,285.4 | 329,688.4 | 318,291.3 | **1.04x** | 9.02x | OK |

## Key Optimizations Applied

1. **Read-lock fast path & zero-copy `Graphic3dCamera::project` (`martensite-tkservice/src/graphic3d/camera_matrices.rs`)**:
   - Replaced `matrices_d` (which acquired a write lock and copied the 520-byte `Graphic3dCameraMatrices<f64>` struct three times on every read) with `read_double_matrix` (read-lock fast path, returning only the requested 128-byte `Mat4<f64>`).
   - Rewrote `Graphic3dCamera::project` to transform points directly against `&s.matrices_d.orientation` and `&s.matrices_d.m_projection` under a single read lock with zero `Mat4` copies. Sped up `Graphic3d::camera_project_1000` by **3.23x** (8.31x → 2.60x M/O) and `camera_matrices_100` by **1.21x** (1.27x M/O).
2. **Single-lock `last_found_child` fast path in `TdfLabel::find_child` (`martensite-tklcaf/src/tdf/label_children.rs`)**:
   - Checked `last_found_child` before cloning `first_child` under a single `node.state.read()` lock, avoiding a redundant `RwLock` + `Arc` clone/drop on sequential label lookups (**1.17x** faster on `TDF::lookup_500_labels`).

## Remaining Gap Analysis

- `TDF::build_500_labels` / `lookup_500_labels` (~6.7–9.0x M/O) is dominated by per-node `Arc<TdfLabelNode>` + `RwLock<TdfLabelNodeState>` + `Weak` downgrade/upgrade for thread-safe OCAF label trees vs OCCT's raw-pointer arena allocator (`NCollection_IncAllocator`).

## Verification

- `cargo test --lib -p martensite-tkservice -- camera`: all passed.
- `cargo test --lib -p martensite-tklcaf -- label`: all passed.
- `cargo clippy --lib -p martensite-tkservice -p martensite-tklcaf -- -D warnings`: clean.
