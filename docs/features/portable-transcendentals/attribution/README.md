# Attribution proof for the changed baselines (#201)

This is the admission record for numerical behavior version 2. Every expected
output that changed between the original commit `534e7d6` and the candidate
branch is traced to individual elementary-function results, and each of those
results is checked against MPFR. The maintainer approved behavior version 2 and
a recorded migration (no version-1 replay) on 2026-10-19. The approval only
counts with this proof attached.

## Method

1. **Instrumentation.** The test-only `libm-attribution` feature of
   `martensite-cmath` (`src/attribution.rs`) evaluates each adapter call twice:
   once with the portable function, and once with the platform entry point the
   kernel called before #201. Those entry points are the Rust standard methods,
   which call glibc on Linux, `asinh`/`acosh`/`atanh` from the C library, and the
   in-tree port of glibc 2.39 `cbrt`. When the two results differ in any bit,
   the call is logged with the test name, the input words and both result words.
   `MARTENSITE_CMATH_SELECT=platform|portable` chooses which result is returned.
   No release build, CI gate or default feature set enables the feature. Without
   it, the `observed!` macro expands to the portable expression alone.
2. **Replay** (`replay.sh`). Both runs execute the whole workspace test suite:
   - `platform_original`: the original test sources and expected fixtures, with
     platform results. This must reproduce the old outputs.
   - `portable_candidate`: the candidate test sources and fixtures, with portable
     results. This must reproduce the new outputs.

   All 73 changed fixture files are compared exactly by their consumers. The one
   exception is NaN tokens, which are compared by classification. The production
   sources are identical in both runs, so the selected libm results are the only
   variable.
3. **MPFR** (`verify_mpfr.py`, gmpy2 2.3.2, MPFR 4.2.2, GMP 6.3.0, pinned in
   `../reference-proof/requirements.txt`). Each distinct differing call
   (function + input words, both runs together) is evaluated two ways: directly at
   the destination precision with MPFR's correct rounding, and at 256 bits and
   then rounded once. The two must agree. The script also calls the running
   C library through `ctypes` to confirm that the logged platform word really is
   what glibc returns.
4. **Rows** (`attribute_rows.py`). Each changed row is mapped to the test that
   replays it, through its paired request line and family. The row is marked
   `independent_validation: attributed` only when four conditions hold. Its test
   passed in both runs. The test made at least one differing call. Every finite
   differing call in it has the verdict "portable correctly rounded, platform
   not". No reference disagreement occurred.

## Results

Environment: Linux x86_64, glibc 2.39, rustc 1.99.0, a CPU with FMA
(`environment.json`). The replayed crates are those of `76679cd`; later commits
on the branch change only test formatting and documentation.

| Check | Result |
| --- | --- |
| `portable_candidate` | 13,225 passed, 0 failed, 25 ignored |
| `platform_original` | 13,187 passed, 1 failed, 25 ignored |
| Changed fixture rows attributed to rounding | **971 of 975** (73 files, consumed by 73 tests) |
| Rows not explained by rounding | **4**, all NaN signs of domain errors, not admitted and owned by #203 |
| Distinct differing calls | 1,372,286 |
| Finite calls: portable = MPFR, glibc ≠ MPFR | **1,371,767 of 1,371,767** |
| MPFR direct vs 256-bit disagreements | 0 |
| Logged platform word ≠ glibc called via `ctypes` | 0 (cbrt and powi are not glibc calls, so they are not checked) |

The single `platform_original` failure is the new `martensite-cmath` doctest
`cbrt(-27.0) == -3.0`. That doctest documents the portable contract, and the
glibc kernel returns `-3.0000000000000004` for it. It is not an original test.

Finite differing calls by function: sinh 702,704; cosh 483,419; atan2 58,785;
atan 52,168; sin 35,582; cos 27,516; asin 5,079; asinh 4,786; tanh 825;
hypot 436; acos 308; pow 57; exp 33; log1p 18; log10 12; atanh 10; acosh 9;
log2 6; cbrt 5; log 3; tan 2; expm1 2; exp2 2.

Of the 519 non-finite differing calls, only the two domain-error NaN signs
described below change a fixture row; fixture comparisons classify numeric NaN
words, so the others cannot:

- 504 differ only in NaN sign or payload (`nan_payload_or_sign_only`).
- 14 have a signaling-NaN argument. glibc raises invalid and returns NaN, while
  the portable result follows the C99 identities that MPFR also gives:
  `hypot(±inf, sNaN) = +inf` and `pow(1, sNaN) = pow(sNaN, ±0) = 1`. These
  calls come from the new special-value contract tests in `martensite-cmath`.
- 1 is `powi(sNaN, 1)`, where the old code returned the signaling NaN unchanged
  and the fixed sequence returns it quieted. `powi` is not a libm function.

### Rows not explained by rounding

Four expression-corpus rows changed for a reason other than rounding:
`crates/martensite-tkexpress/tests/semantic_corpus/fixtures/expected.txt` lines
235, 236, 274 and 275. Their requests are `acosh(0.25)`, `ACosh(0.25)`, `atanh(2)`
and `ATanh(2)`. In each one, the only changed token is the printed NaN text, which
went from `-nan` to `nan`. Both inputs are outside the function's domain. glibc
returns the x86 default NaN with the sign bit set (`fff8000000000000`), and the
portable function returns `7ff8000000000000`. MPFR has no NaN sign, so it cannot
decide between them. The replay attributes the change to exactly these two calls
(`nan_payload_or_sign_only`). These rows are recorded with
`cause: domain_error_nan_sign`. All other rows are recorded with
`cause: rounding`. A NaN sign is not a correctly rounded result, so these four
rows are **not admitted** by this proof
(`independent_validation: not_admitted_nan_sign_issue_203`). Issue #203 restores
their original bytes through an explicit domain-error NaN contract.

Each changed row's record in `../changed-rows-*.json` lists its test, the
pass status in both runs, the number of differing calls in that test, and the
counts per function. `test-call-digests.json` gives, for each of the 281 tests
that made a differing call, the call count and a SHA-256 over its sorted call
keys, so the set can be recomputed from a replay. `calls.tsv.xz` holds every
distinct call with its platform, portable and MPFR words and the verdicts.

## Changed test sources

Every changed Rust test source (173 files at `ac275e0`, plus the bivariate seal
fix) was reviewed hunk by hunk:

- 262 hunks are mechanical call substitutions (`x.sin()` →
  `martensite_cmath::sin(x)` and similar, plus formatting).
- 23 hunks add tests, and 1 hunk changes only a comment.
- 22 hunks change expected-output literals and nothing else. Twenty of them are
  FNV-1a seals over answer fixtures: the 19 construction corpora and the
  bivariate corpus. In each, the request seal, the row counts and the family
  counts are unchanged, and the sealed bytes are the attributed rows above. The
  other 2 hunks are the cube-root literals in
  `martensite-tkmath/tests/math_utils_core_api_behavior.rs`: `cbrt(2e100)`,
  `cbrt(27) = 3` and `cbrt(0.125) = 0.5`. Their test made three differing `cbrt`
  calls, all of them MPFR-verified.
- No hunk changes a test input, a comparison kind or a tolerance, and none
  removes or ignores a test.

Both runs exercise these sources: the original versions pass with platform
results, and the candidate versions pass with portable results. In three
round-trip checks in `cmath_behavior.rs` (`sinh(asinh(x))` and similar), the
inverse now comes from the same library as the function under test. The
tolerance is unchanged. The new correctly rounded reference tests cover those
functions independently.

The candidate branch also had a defect: the `bivariate_retained_corpus` seal
still pinned the original answer bytes, so that corpus failed before it replayed
any row. The seal now pins the reviewed bytes (`17538326652504145611`).

## Reproduce

```bash
export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0
docs/features/portable-transcendentals/attribution/replay.sh /tmp/replay   # about 1 h
python3 -m venv /tmp/mpfr && /tmp/mpfr/bin/pip install --require-hashes \
    -r docs/features/portable-transcendentals/reference-proof/requirements.txt
A=docs/features/portable-transcendentals/attribution
/tmp/mpfr/bin/python -I $A/verify_mpfr.py /tmp/replay/calls-platform_original \
    /tmp/replay/calls-portable_candidate          # writes calls.tsv.xz, mpfr-summary.json
cp $A/calls.tsv.xz /tmp/replay/
python3 -I $A/attribute_rows.py --evidence /tmp/replay   # updates ../changed-rows-*.json
```

The raw call logs (about 790 MB across 19 files) are not retained. Their
deduplicated content is `calls.tsv.xz`, and `environment.json` records their
line counts and the hashes of every retained file.

## Limits

- The proof covers the Linux x86_64 replay against glibc 2.39. Other platforms
  are qualified by the release matrix, which compares the same exact words on
  every target.
- Correct rounding is verified for every call the test suite reaches, not for
  every input. The dependency's wider claim is recorded in
  `../dependency-review/review.json`.
