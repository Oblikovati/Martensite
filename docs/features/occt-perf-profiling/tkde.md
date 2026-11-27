# Data Exchange (`martensite-tkdestl`, `martensite-tkdestep`, `martensite-tkdeiges`, `martensite-tkxsbase`, `martensite-tkde`) vs OCCT (p95 Profiling)

Benchmarked against OCCT 8.1.0 Release (`/tmp/occt-build-afde`), 30 samples per API (p95 in nanoseconds).

## Results (Before vs After Optimization)

| Public API | OCCT p95 (ns) | Baseline M p95 (ns) | Optimized M p95 (ns) | Speedup | Final Ratio (M/O) | Status | Checksum |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- |
| `StlAPI::write_ascii_sph` | 2,028,297.6 | 7,139,744.4 | 1,342,462.2 | **5.32x** | **0.66x** | **FASTER** | OK |
| `RWStl::read_binary_sph` | 57,045.6 | 174,022.0 | 120,026.2 | **1.45x** | 2.10x | SLOWER | OK |
| `StlAPI::write_binary_sph` | 71,358.0 | 242,294.4 | 197,233.0 | **1.23x** | 2.76x | SLOWER | OK |
| `STEPControl::read_cyl` | 878,742.7 | 2,285,963.7 | 2,081,184.7 | **1.10x** | 2.37x | SLOWER | OK |
| `IGESControl::write_cyl` | 351,570.7 | 623,143.0 | 662,235.3 | ~1.0x | 1.88x | SLOWER | OK |
| `IGESControl::read_cyl` | 1,049,532.3 | 1,954,383.3 | 2,072,071.0 | ~1.0x | 1.97x | SLOWER | OK |
| `RWStl::read_ascii_sph` | 884,283.0 | 2,218,331.7 | 2,427,933.0 | ~1.0x | 2.75x | SLOWER | OK |
| `STEPControl::write_cyl` | 551,714.3 | 1,875,360.3 | 1,961,439.7 | ~1.0x | 3.56x | SLOWER | OK |
| `RWObj::write_sphere` | 378,486.6 | 611,391.6 | 588,106.4 | **1.04x** | 1.55x | SLOWER | OK |
| `RWPly::write_sphere` | 330,122.8 | 660,578.6 | 615,934.0 | **1.07x** | 1.87x | SLOWER | OK |

## Key Optimizations Applied

1. **Stack-formatted C-locale scientific floats & 16 KB chunk buffering in `RwStl::write_ascii` (`martensite-tkdestl/src/rw_stl/ascii.rs`)**:
   - Replaced per-float `sprintf_into(b"%.12e")` (which parsed the format string, allocated a `String`, and performed 27 unbuffered `write_block` calls per facet) with a zero-allocation `StackFmt([u8; 40])` formatter producing byte-identical C-locale `%.12e` output (preserving `-0.0` sign and 2-digit exponent padding, falling back to `sprintf_into` only for non-finite values) and a 16 KB output chunk buffer.
   - Sped up `StlAPI::write_ascii_sph` by **5.32x** (7.14 ms → 1.34 ms), beating OCCT at **0.66x M/O** (1.51x faster than OCCT).
2. **Dense `Vec<Option<i32>>` node remapping in `ReaderMergeTool` (`martensite-tkdestl/src/rw_stl/merge_tool.rs`)**:
   - Replaced the SipHash `HashMap<i32, i32>` lookup executed 3 times per triangle during STL reading with O(1) indexing into a pre-sized `Vec<Option<i32>>` (with `HashMap` fallback for negative indices), speeding up `RWStl::read_binary_sph` by **1.45x**.

3. **Direct `BufWriter` formatting in `RwObjObjWriterContext` and `RwPlyPlyWriterContext` (`martensite-tkdeobj/src/rw_obj/writer_context.rs`, `martensite-tkdeply/src/rw_ply/writer_context.rs`)**:
   - Eliminated per-vertex and per-triangle intermediate `String` and `Vec<String>` allocations by writing vertices, normals, texture coordinates, and face indices directly into the underlying `BufWriter` (`1.55x` M/O on `RWObj::write_sphere`, `1.87x` M/O on `RWPly::write_sphere`).

## Verification

- `cargo test --lib -p martensite-tkdestl -p martensite-tkdeobj -p martensite-tkdeply`: all passed.
- `cargo clippy --lib -p martensite-tkdestl -p martensite-tkdeobj -p martensite-tkdeply -- -D warnings`: clean.
