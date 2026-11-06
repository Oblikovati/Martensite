# Attribution of the changed logo sample values

`martensite-tkxcaf` routes every transcendental call through `martensite-cmath`, whose results are
correctly rounded (numerical behavior version 2, #201). The recorded values of its logo sample came
from the platform C library, so two of them moved. Every other baseline of `martensite-tkde`,
`martensite-tkxsbase`, `martensite-tkvcaf` and `martensite-tkxcaf` stayed as it was. No assertion or
tolerance changed.

## What changed

Both values are in `tests/demo_samples/logo.rs`, on the connector surface between the sphere and the
loop. A plate solver builds that surface, and its `log` calls feed its refinement.

| Value | Check | Platform result | Recorded before | Portable result | Recorded now |
| --- | --- | --- | --- | --- | --- |
| Plate G0 error | Within 1e-9 | 5.48628878572169e-05 | 5.48628878572e-05 | 5.48295258890299e-05 | 5.48295258890299e-05 |
| Connector volume | Within 1e-5, relative, as measured to six digits | Inside the check around 1.53621 | 1.53621 | 1.53618583974108 | 1.53619 |

The surface moves by about 3e-6 on average (2.4e-5 of volume over an area of 7.6), which is inside
the 1e-4 tolerance the solver is given. The connector's area, its shape counts, the document dump,
every pixel probe of the saved view and the G1 error all stay as recorded.

## Method

The method is the same as in `../../portable-transcendentals/attribution/`:

1. **Replay with the platform library.** With the test-only `libm-attribution` feature of
   `martensite-cmath` and `MARTENSITE_CMATH_SELECT=platform`, all 598 tests of the four crates pass
   with the previous values. The previous G0 error comes back to all of its recorded digits.
2. **Replay with the portable functions.** With `MARTENSITE_CMATH_SELECT=portable`, the same 598
   tests pass with the new values.
3. **Record the differing calls.** In both runs every call whose two results differ in any bit is
   logged with its test, inputs and both results (`calls-platform.jsonl.xz`,
   `calls-portable.jsonl.xz`; 7,294 and 7,418 calls). The logo sample has 1,541 and 1,573 of them.
4. **Check each call with MPFR.** `verify_mpfr.py` of the portable-transcendentals attribution
   (gmpy2 2.3.2, MPFR 4.2.2, GMP 6.3.0) evaluates every distinct call directly at binary64 and at
   256 bits rounded once. Both must agree. It also calls the running C library through `ctypes` to
   confirm the logged platform word.

## Results

| Check | Result |
| --- | --- |
| Distinct differing calls (both runs) | 10,203: `sinh` 6,134, `log` 2,330, `cosh` 893, `sin` 441, `cos` 302, `atan2` 40, `asin` 32, `log10` 14, `asinh` 10, `atan` 4, `acos` 2, `pow` 1 |
| Portable word = MPFR, platform word ≠ MPFR | **10,203 of 10,203** |
| MPFR direct vs 256-bit disagreements | 0 |
| Logged platform word ≠ glibc via `ctypes` | 0 |
| Tests with differing calls | 6 |

Every differing call has the verdict "portable correctly rounded, platform not", so the two changed
values are attributed to platform results that were not correctly rounded. Many differing calls do
not reach a checked value: the pencil sample has the most, 5,745 and 5,837, and none of its values
moved.

`mpfr-summary.json` and `calls.tsv.xz` are the verifier's outputs. To run it, unpack each call log
into its own directory and pass both:

```sh
mkdir calls-platform calls-portable
xz -dc calls-platform.jsonl.xz > calls-platform/calls.jsonl
xz -dc calls-portable.jsonl.xz > calls-portable/calls.jsonl
python3 -I ../../portable-transcendentals/attribution/verify_mpfr.py calls-platform calls-portable
```

The replays ran on Linux x86-64 with rustc 1.99.0, on a software renderer:

```sh
MARTENSITE_CMATH_SELECT=platform MARTENSITE_CMATH_ATTRIBUTION_LOG=calls-platform \
  cargo nextest run -p martensite-tkde -p martensite-tkxsbase -p martensite-tkvcaf \
  -p martensite-tkxcaf -p martensite-cmath --features martensite-cmath/libm-attribution \
  -E 'not package(martensite-cmath)'
```

The platform run used the values before this change; the portable run is the same command with
`MARTENSITE_CMATH_SELECT=portable` and `calls-portable`.
