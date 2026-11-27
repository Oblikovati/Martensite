# Tier 1: `martensite-tkernel` vs OCCT `TKernel` (p95 Profiling & Optimization)

- **OCCT baseline:** `/git/oblikovati-workspace/occt-install-vis` (OCCT 8.0.0 Release, `g++ -O3 -DNDEBUG -std=c++17`)
- **Martensite:** worktree `perf/toolkit-occt-p95` (Rust `--release`, `opt-level = 3`)
- **Methodology:** 200 independent samples per API (with warmup, `black_box` / `asm volatile` anti-hoisting, and bit-exact checksum verification across all 28 APIs).

## Results (Before vs After Optimization)

| Public API | OCCT p95 (ns) | Before (ns) | After (ns) | Ratio (M/O) | Status | Notes |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| `Quantity_Color::values_hls` | 18.8 | 141.7 | 142.5 | 7.59x | SLOWER | Bound by `#201` portable correctly-rounded `martensite_cmath::powf` |
| `IncAllocator::alloc_1000_reset` | 4,061.5 | 10,985.3 | 13,402.2 | 3.30x | SLOWER | Thread-safe `Mutex` synchronization in `BaseAllocator` |
| `Sequence::append_iter_1000` | 11,620.5 | 17,746.8 | 17,415.3 | 1.50x | SLOWER | Per-node heap alloc + `SequenceNodeHeader` atomic cache |
| `Vector::append_read_1000` | 1,419.1 | 3,017.6 | 2,062.3 | 1.45x | SLOWER | Improved 1.46x via single block lookup & inlining |
| `Quantity_Color::nearest_name` | 459.8 | 811.3 | 573.8 | 1.25x | SLOWER | Improved 1.41x via contiguous `SRGB_TABLE` (remaining ~120 ns is `pow`) |
| `Standard::strtod` | 60.6 | 119.9 | 72.6 | 1.20x | SLOWER | Improved 1.65x by removing upfront `c_prefix` NUL scan |
| `AsciiString::new_and_cat` | 30.5 | 41.9 | 34.7 | 1.14x | SLOWER | Improved 1.21x via `pop`/`extend_from_slice` instead of `insert_from_slice` |
| `Array2::fill_and_sum_32x32` | 495.5 | 547.2 | 548.0 | 1.11x | SLOWER | Checked bounds per cell |
| `Array1::fill_and_sum_1000` | 544.6 | 555.0 | 567.0 | 1.04x | PARITY | |
| `Handle::clone_drop_1000` | 8,171.1 | 8,177.1 | 8,309.4 | 1.02x | PARITY | |
| `Standard::hash_bytes_64` | 3.6 | 7.3 | 3.5 | 0.96x | PARITY | Improved 2.08x via `#[inline]` on `hash_bytes` / `murmur_hash_64a` |
| `DataMap::find_1000` | 1,459.2 | 2,038.3 | 1,380.5 | 0.95x | FASTER | Improved 1.48x via `#[inline]` on `find` and `DefaultHashPolicy` |
| `List::append_iter_1000` | 12,051.3 | 10,764.9 | 11,293.9 | 0.94x | FASTER | |
| `Map::contains_2000` | 2,943.9 | 2,619.3 | 2,668.9 | 0.91x | FASTER | |
| `Sequence::indexed_access_1000` | 1,511.6 | 4,121.2 | 1,199.6 | 0.79x | FASTER | Improved 3.44x via adjacent-index fast path in `SequenceTopology::find` |
| `PackedMap::add_1000_contains_2000` | 5,576.3 | 4,471.3 | 3,865.1 | 0.69x | FASTER | |
| `Mat4::inverted` | 7.6 | 39.1 | 4.4 | 0.58x | FASTER | Improved 8.89x via direct 12-minor Laplace expansion |
| `Standard_GUID::from_ascii` | 139.1 | 94.8 | 68.8 | 0.49x | FASTER | |
| `OSD_Path::parse_and_system_name` | 441.4 | 157.8 | 157.4 | 0.36x | FASTER | |
| `IndexedDataMap::add_and_find_1000` | 23,089.7 | 7,557.0 | 6,543.1 | 0.28x | FASTER | |
| `DataMap::bind_1000` | 21,302.4 | 5,549.3 | 5,870.9 | 0.28x | FASTER | |
| `Map::add_1000` | 19,985.9 | 5,308.9 | 5,457.1 | 0.27x | FASTER | |
| `IndexedMap::add_and_find_1000` | 22,782.2 | 5,757.6 | 5,810.7 | 0.26x | FASTER | |
| `AsciiString::upper_case` | 91.8 | 22.4 | 22.8 | 0.25x | FASTER | |
| `AsciiString::search` | 90.6 | 18.4 | 19.2 | 0.21x | FASTER | |
| `ExtendedString::from_utf8` | 142.2 | 20.9 | 22.0 | 0.15x | FASTER | |
| `Standard_GUID::to_c_string` | 171.1 | 246.3 | 6.5 | 0.04x | FASTER | Improved 37.9x by eliminating `Display` heap `String` allocation |
| `ProgressScope::1000_ticks_null` | 427.4 | 1,261.1 | 11.2 | 0.03x | FASTER | Improved 112x via `#[inline]` on `ProgressRange::default`/`drop` and `ProgressScope::drop` |

**Summary:** 28 APIs benchmarked (all 28 bit-exact checksum matches). Reduced slower APIs from 13 to 8 (20/28 at parity or up to 38x faster than OCCT).
