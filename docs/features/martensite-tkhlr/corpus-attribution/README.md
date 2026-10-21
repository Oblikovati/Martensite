# Attribution of the changed hidden-line and helix replay answers

The hidden line removal and helix crates route every transcendental call through
`martensite-cmath`, whose results are correctly rounded (numerical behavior version 2,
#201). 302 frozen answers of the request replays changed: 256 hidden-line answers and
46 helix answers. Each replay compares its answer with the frozen one bit for bit after
mapping every NaN bit pattern to `NaN`; no assertion or tolerance changed.

| Answer file | Changed rows |
| --- | --- |
| `crates/martensite-tkhelix/tests/fixtures/retained_numeric/reference-expected.txt` | 46 of 347 |
| `crates/martensite-tkhlr/tests/fixtures/algo_numeric/reference-expected.txt` | 65 of 360 |
| `crates/martensite-tkhlr/tests/fixtures/function_numeric/reference-expected.txt` | 68 of 1,296 |
| `crates/martensite-tkhlr/tests/fixtures/data_numeric/reference-expected.txt` | 34 of 320 |
| `crates/martensite-tkhlr/tests/fixtures/hider_numeric/reference-expected.txt` | 25 of 400 |
| `crates/martensite-tkhlr/tests/fixtures/outliner_numeric/reference-expected.txt` | 20 of 240 |
| `crates/martensite-tkhlr/tests/fixtures/search_numeric/reference-expected.txt` | 12 of 144 |
| `crates/martensite-tkhlr/tests/fixtures/edgeface_intersector_numeric/reference-expected.txt` | 9 of 1,000 |
| `crates/martensite-tkhlr/tests/fixtures/reflection_numeric/reference-expected.txt` | 7 of 120 |
| `crates/martensite-tkhlr/tests/fixtures/polygonal_numeric/reference-expected.txt` | 5 of 300 |
| `crates/martensite-tkhlr/tests/fixtures/boundary_walking_numeric/reference-expected.txt` | 4 of 432 |
| `crates/martensite-tkhlr/tests/fixtures/brep_surface_numeric/reference-expected.txt` | 4 of 900 |
| `crates/martensite-tkhlr/tests/fixtures/brep_records_numeric/reference-expected.txt` | 2 of 800 |
| `crates/martensite-tkhlr/tests/fixtures/brep_curve_numeric/reference-expected.txt` | 1 of 600 |

The other eleven answer files of the hidden-line replays did not change. Each replay
test reads its request and its answer by byte range, so the ranges of the tests that
follow a changed row in the same file moved with it; the request ranges did not change.

## Method

The method is the one of `../../portable-transcendentals/attribution/`:

1. **Replay with the platform library.** With the test-only `libm-attribution` feature
   of `martensite-cmath` and `MARTENSITE_CMATH_SELECT=platform`, every adapter call
   returns the platform C library result (glibc 2.39 on Linux x86-64). Every test of
   both crates (20,818) passes with the previous answer files and byte ranges. The
   previous answers therefore come back from the previous calls.
2. **Replay with the portable functions.** With `MARTENSITE_CMATH_SELECT=portable`
   (the default), every test of both crates passes with the new answers and ranges.
3. **Record the differing calls.** In both runs every call whose two results differ
   in any bit is logged with its test, inputs and both results
   (`calls-platform.jsonl.xz`, `calls-portable.jsonl.xz`; 30,770 and 30,780 calls).
4. **Check each call with MPFR.** `verify_mpfr.py` of the portable-transcendentals
   attribution (gmpy2 2.3.2, MPFR 4.2.2, GMP 6.3.0) evaluates every distinct call
   directly at binary64 and at 256 bits rounded once; both must agree. It also calls
   the running C library through `ctypes` to confirm the logged platform word.

## Results

| Check | Result |
| --- | --- |
| Distinct differing calls (both runs) | 13,263: `sin` 6,095, `cos` 5,592, `atan` 772, `atan2` 538, `asin` 131, `acos` 71, `sinh` 38, `cosh` 17, `pow` 5, `log` 4 |
| Portable word = MPFR, platform word ≠ MPFR | **13,263 of 13,263** |
| MPFR direct vs 256-bit disagreements | 0 |
| Logged platform word ≠ glibc via `ctypes` | 0 |

Per test target, the runs made these differing calls (the `toolkit` row adds the
`toolkit` targets of both crates; `rust_out` is a documentation test):

| Target | Platform run | Portable run |
| --- | --- | --- |
| `algo_numeric` | `acos` 46, `asin` 86, `atan` 616, `atan2` 379, `cos` 4279, `log` 4, `pow` 4, `sin` 4508 | `acos` 48, `asin` 86, `atan` 616, `atan2` 377, `cos` 4263, `log` 4, `pow` 4, `sin` 4540 |
| `boundary_walking_numeric` | `cos` 194, `sin` 297 | `cos` 194, `sin` 297 |
| `data_numeric` | `acos` 18, `asin` 52, `atan` 333, `atan2` 196, `cos` 2321, `log` 2, `pow` 2, `sin` 2295 | `acos` 18, `asin` 50, `atan` 333, `atan2` 196, `cos` 2313, `log` 2, `pow` 2, `sin` 2292 |
| `dataset_cases` | `cos` 52, `sin` 55 | `cos` 52, `sin` 55 |
| `function_numeric` | `cos` 14, `cosh` 81, `sinh` 108 | `cos` 14, `cosh` 81, `sinh` 108 |
| `hider_numeric` | `acos` 22, `asin` 54, `atan` 386, `atan2` 310, `cos` 2617, `log` 2, `pow` 2, `sin` 2698 | `acos` 22, `asin` 54, `atan` 389, `atan2` 308, `cos` 2623, `log` 2, `pow` 2, `sin` 2694 |
| `outliner_numeric` | `atan` 267, `atan2` 170, `cos` 919, `sin` 938 | `atan` 265, `atan2` 170, `cos` 921, `sin` 938 |
| `polygonal_numeric` | `cos` 7, `sin` 11 | `cos` 7, `sin` 11 |
| `reflection_numeric` | `acos` 2, `asin` 4, `atan` 103, `atan2` 60, `cos` 492, `sin` 522 | `acos` 2, `asin` 4, `atan` 105, `atan2` 60, `cos` 492, `sin` 520 |
| `rust_out` | `cos` 4 | `cos` 4 |
| `search_numeric` | `cos` 40, `sin` 93 | `cos` 40, `sin` 93 |
| `toolkit` | `acos` 27, `asin` 36, `atan` 1, `cos` 2071, `cosh` 66, `pow` 1, `sin` 2703, `sinh` 200 | `acos` 27, `asin` 36, `atan` 1, `cos` 2077, `cosh` 63, `pow` 1, `sin` 2701, `sinh` 203 |

Every differing call has the verdict "portable correctly rounded, platform not", so the
302 changed answers are attributed to platform results that were not correctly rounded.
Many differing calls do not reach a printed answer.

`mpfr-summary.json` and `calls.tsv.xz` are the verifier's outputs; running it again
gives identical bytes. To run it, unpack each call log into its own directory and pass both:

```sh
mkdir calls-platform calls-portable
xz -dc calls-platform.jsonl.xz > calls-platform/calls.jsonl
xz -dc calls-portable.jsonl.xz > calls-portable/calls.jsonl
python3 -I ../../portable-transcendentals/attribution/verify_mpfr.py calls-platform calls-portable
```

(`verify_mpfr.py` writes its outputs next to itself; copy it here first to compare.)

The replays ran on Linux x86-64 with rustc 1.99.0:

```sh
MARTENSITE_CMATH_SELECT=platform MARTENSITE_CMATH_ATTRIBUTION_LOG=calls-platform \
  cargo test -p martensite-tkhlr -p martensite-tkhelix --features martensite-cmath/libm-attribution
MARTENSITE_CMATH_SELECT=portable MARTENSITE_CMATH_ATTRIBUTION_LOG=calls-portable \
  cargo test -p martensite-tkhlr -p martensite-tkhelix --features martensite-cmath/libm-attribution
```

The platform run used the answer files and byte ranges before this change.
