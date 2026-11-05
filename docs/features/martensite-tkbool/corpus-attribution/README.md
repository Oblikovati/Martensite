# Attribution of the changed topological operation replays

The Boolean, topological operation, fillet and naming crates route every transcendental call
through `martensite-cmath`, whose results are correctly rounded (numerical behavior version 2,
#201). Their recorded answers came from the platform C library, so two groups of baselines moved
when the crates moved to the portable functions. Every other baseline of these crates, including
every frozen progress and cancellation sequence of `martensite-tkbo` and every script case of
`martensite-tkfillet`, stayed as it was. No assertion or tolerance changed.

## What changed

Fourteen rows of the replays in `crates/martensite-tkbool/tests/fixtures/full_numeric`. Each replay
compares its answer with the recorded one bit for bit after mapping every NaN bit pattern to `NaN`.

| Answer file | Changed rows | Changed tokens | Largest absolute change | Differing calls (platform / portable) |
| --- | --- | --- | --- | --- |
| `brep_algo_0.tsv` | 2 of 78 | 16 | 2.47e-32 | 5 / 5 |
| `brep_fill_advanced_evolved_0.tsv` | 2 of 14 | 4 | 1.23e-32 | 230 / 223 |
| `brep_fill_draft_0.tsv` | 2 of 36 | 4 | 6.16e-33 | 1 / 1 |
| `brep_fill_filling_0.tsv` | 4 of 16 | 94 | 2.42e+00 | 96 / 96 |
| `brep_fill_pipe_shell_4.tsv` | 2 of 6 | 21 | 4.44e-16 | 757 / 758 |
| `brep_fill_pipe_shell_8.tsv` | 1 of 6 | 12 | 5.60e-11 | 5250 / 5244 |
| `top_ope_brep_tool_curve_tool_0.tsv` | 1 of 43 | 1 | 6.79e-13 | 5070 / 5073 |

Most rows move in the last bits of residuals and coordinates. One row of
`brep_fill_filling_0.tsv` moves further: with 96 differing `log` calls in the plate solver, one filling
surface converges to a different refinement. Its bounding values and several coordinates change by
up to 2.4, and its reported tolerance goes from 1.5e-5 to 2.7e-5. The inputs and the point and curve
counts stay the same.

One script case: `lowalgos_intss::bug567_1` in `martensite-boolean-cases`. On platform results it
finds 9 curves and fails on the number of edges, which its script does not list. On correctly
rounded results it finds 8 curves and 8 edges, which the script lists as known problems. The test
held the first outcome exactly and now holds the second exactly (`pinned_problem_case!`), so a
change either way still shows. Its neighbour `bug567_2` did not change.

## Method

The method is the one of `../../portable-transcendentals/attribution/`:

1. **Replay with the platform library.** With the test-only `libm-attribution` feature of
   `martensite-cmath` and `MARTENSITE_CMATH_SELECT=platform`, every test of `martensite-tkbool`
   and the `bug567` and `bugs_modalg_8` cases pass with the previous answers and the previous pin
   (724 of 724). The previous answers therefore come back from the previous calls.
2. **Replay with the portable functions.** With `MARTENSITE_CMATH_SELECT=portable`, the same tests
   pass with the new answers and the new pin (724 of 724).
3. **Record the differing calls.** In both runs every call whose two results differ in any bit is
   logged with its test, inputs and both results (`calls-platform.jsonl.xz`,
   `calls-portable.jsonl.xz`; 85,586 and 85,287 calls).
4. **Check each call with MPFR.** `verify_mpfr.py` of the portable-transcendentals attribution
   (gmpy2 2.3.2, MPFR 4.2.2, GMP 6.3.0) evaluates every distinct call directly at binary64 and at
   256 bits rounded once. Both must agree. It also calls the running C library through `ctypes` to
   confirm the logged platform word.

## Results

| Check | Result |
| --- | --- |
| Distinct differing calls (both runs) | 88,553: `pow` 44,848, `sin` 14,784, `cos` 13,138, `asin` 10,341, `acos` 2,727, `atan2` 2,451, `exp` 119, `log` 100, `atan` 37, `hypot` 5, `cosh` 2, `sinh` 1 |
| Portable word = MPFR, platform word ≠ MPFR | **88,553 of 88,553** |
| MPFR direct vs 256-bit disagreements | 0 |
| Logged platform word ≠ glibc via `ctypes` | 0 |
| `bug567_1` differing calls | 1,807 platform, 1,524 portable |

Every differing call has the verdict "portable correctly rounded, platform not", so the changed
rows and the changed case are attributed to platform results that were not correctly rounded. Many
differing calls do not reach a printed answer.

`mpfr-summary.json` and `calls.tsv.xz` are the verifier's outputs. To run it, unpack each call log
into its own directory and pass both:

```sh
mkdir calls-platform calls-portable
xz -dc calls-platform.jsonl.xz > calls-platform/calls.jsonl
xz -dc calls-portable.jsonl.xz > calls-portable/calls.jsonl
python3 -I ../../portable-transcendentals/attribution/verify_mpfr.py calls-platform calls-portable
```

The replays ran on Linux x86-64 with rustc 1.99.0:

```sh
MARTENSITE_CMATH_SELECT=platform MARTENSITE_CMATH_ATTRIBUTION_LOG=calls-platform \
  cargo nextest run -p martensite-tkbool -p martensite-boolean-cases \
  --features martensite-cmath/libm-attribution \
  -E 'package(martensite-tkbool) | (package(martensite-boolean-cases) & (test(/bug567_/) | test(/bugs_modalg_8::/)))'
```

The platform run used the answers and the pin before this change; the portable run is the same
command with `MARTENSITE_CMATH_SELECT=portable` and `calls-portable`.
