# Attribution of the changed plate length

`martensite-tkoffset` routes every transcendental call through `martensite-cmath`, whose results are
correctly rounded (numerical behavior version 2, #201). One recorded value of its script cases came
from the platform C library and moved: the length in the known problem of `hlr/poly_hlr/Plate`. No
assertion or tolerance changed, and every other test of `martensite-tkfeat` and `martensite-tkoffset`
stayed as it was.

## What changed

The case fills a surface from four boundary edges and a point, computes its hidden lines and checks
their length against 404.386 within 1e-7. It misses that check on both math libraries, and the miss
is what its `TODO` lines hold, as an exact line.

| Math | Length | The case's known problem |
| --- | --- | --- |
| Platform C library | 404.004 | `Error : The length of result shape is 404.004, expected 404.386` |
| Correctly rounded | 404.492 | `Error : The length of result shape is 404.492, expected 404.386` |

The check itself, 404.386 within 1e-7, is the same. A different length in either direction still
fails the case.

## Method

The method is the one of `../../portable-transcendentals/attribution/`:

1. **Replay with the platform library.** With the test-only `libm-attribution` feature of
   `martensite-cmath` and `MARTENSITE_CMATH_SELECT=platform`, all 2,528 tests of the two crates pass
   with the 404.004 line.
2. **Replay with the portable functions.** With `MARTENSITE_CMATH_SELECT=portable`, the same 2,528
   tests pass with the 404.492 line.
3. **Record the differing calls.** Each run logs every call whose two results differ in any bit, with
   its test, inputs and both results. The plate case has 52 and 50 of them
   (`calls-platform.jsonl.xz`, `calls-portable.jsonl.xz`).
4. **Check each call with MPFR.** `verify_mpfr.py` of the portable-transcendentals attribution
   (gmpy2 2.3.2, MPFR 4.2.2, GMP 6.3.0) evaluates every distinct call directly at binary64 and at
   256 bits rounded once. Both must agree. It also calls the running C library through `ctypes` to
   confirm the logged platform word.

## Results

| Check | Plate case | Both crates |
| --- | --- | --- |
| Distinct differing calls (both runs) | 55: `log` 47, `asin` 8 | 4,440,699, mostly `sinh` and `cosh` |
| Portable word = MPFR, platform word ≠ MPFR | **55 of 55** | **4,440,699 of 4,440,699** |
| MPFR direct vs 256-bit disagreements | 0 | 0 |
| Logged platform word ≠ glibc via `ctypes` | 0 | 0 |

Every differing call in the case has the verdict "portable correctly rounded, platform not", so the
changed length is attributed to platform results that were not correctly rounded. The record does not
trace which of the 55 calls moves the length.

`mpfr-summary.json` and `calls.tsv.xz` are the verifier's outputs for the plate case.
`mpfr-summary-both-crates.json` is its summary for both crates. Their call logs hold over 5 million
lines each and are not kept here. To check the plate case, copy the verifier beside the unpacked logs,
because it writes its outputs next to itself:

```sh
mkdir calls-platform calls-portable
xz -dc calls-platform.jsonl.xz > calls-platform/calls.jsonl
xz -dc calls-portable.jsonl.xz > calls-portable/calls.jsonl
cp ../../portable-transcendentals/attribution/verify_mpfr.py .
python3 -I verify_mpfr.py calls-platform calls-portable
```

The replays ran on Linux x86-64 with rustc 1.99.0, on a software renderer, before the volume property
stop of this release was added. That stop does not touch the length measured here, and both crates'
suites pass with it:

```sh
MARTENSITE_CMATH_SELECT=platform MARTENSITE_CMATH_ATTRIBUTION_LOG=calls-platform \
  cargo nextest run -p martensite-tkfeat -p martensite-tkoffset \
  --features martensite-cmath/libm-attribution
```

The platform run used the 404.004 line; the portable run is the same command with
`MARTENSITE_CMATH_SELECT=portable` and `calls-portable`.
