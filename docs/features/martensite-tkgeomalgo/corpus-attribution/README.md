# Attribution of the changed geometry-corpus answers

The geometric algorithm crate routes every transcendental call through
`martensite-cmath`, whose results are correctly rounded (numerical behavior version 2,
#201). Eleven frozen answers of the geometry corpus changed, and with them the two
fixture seals that hash the answer files:

| File | Lines | Requests |
| --- | --- | --- |
| `crates/martensite-tkgeomalgo/tests/fixtures/geometry_corpus/reference-06.txt` | 91, 94, 103, 111, 112, 113, 116, 129, 134 | nine `fc_batten` requests |
| `crates/martensite-tkgeomalgo/tests/fixtures/geometry_corpus/reference-blend.txt` | 26, 95 | two `ab_appsurf` requests |

| Seal | Before | After |
| --- | --- | --- |
| `geometry_corpus_replay::frozen_request_and_expected_bytes_are_unchanged` (answers) | `0x5b50d4f6dd735d17` | `0xbfd735992e3a7a6b` |
| `complete_blend_families_in_original_order` (answers) | `0xe9bf9047f128ae18` | `0x019f1b0c2c24fcbc` |

No other expected value of the three crates changed; their other 2,000+ tests pass
unchanged with the portable functions.

## Method

The method is the one of `../../portable-transcendentals/attribution/`:

1. **Replay with the platform library.** With the test-only `libm-attribution` feature
   of `martensite-cmath` and `MARTENSITE_CMATH_SELECT=platform`, every adapter call
   returns the platform C library result (glibc 2.39 on Linux x86-64). Both changed
   families, `geometry_corpus_replay::fc_batten_complete_original_family` and
   `complete_blend_families_in_original_order`, pass with the previous answer files
   and seals. The previous answers therefore come back from the previous calls.
2. **Replay with the portable functions.** With `MARTENSITE_CMATH_SELECT=portable`
   (the default), both families pass with the new answers and seals; so does the
   whole `geometry_corpus_replay` target.
3. **Record the differing calls.** In both runs every call whose two results differ
   in any bit is logged with its test, inputs and both results
   (`calls-platform/calls.jsonl`, `calls-portable/calls.jsonl`).
4. **Check each call with MPFR.** `verify_mpfr.py` of the portable-transcendentals
   attribution (gmpy2 2.3.2, MPFR 4.2.2, GMP 6.3.0) evaluates every distinct call
   directly at binary64 and at 256 bits rounded once; both must agree. It also calls
   the running C library through `ctypes` to confirm the logged platform word.

## Results

| Check | Result |
| --- | --- |
| Distinct differing calls (both runs) | 56: `pow` 53, `cos` 2, `sin` 1 |
| Portable word = MPFR, platform word ≠ MPFR | **56 of 56** |
| MPFR direct vs 256-bit disagreements | 0 |
| Logged platform word ≠ glibc via `ctypes` | 0 |

Per test, the changed families made these differing calls:

| Test | Platform run | Portable run |
| --- | --- | --- |
| `fc_batten_complete_original_family` | `pow` 37, `sin` 1 | `pow` 34, `sin` 1 |
| `complete_blend_families_in_original_order` | `pow` 1 | `pow` 1 |

The `fc_energy` and `tt_surface` families also made differing calls (`pow` 6 and
`cos` 2) whose results do not reach a printed answer. Every finite differing call has
the verdict "portable correctly rounded, platform not", so the eleven changed answers
are attributed to platform results that were not correctly rounded.
`mpfr-summary.json` and `calls.tsv.xz` are the verifier's outputs; running it again on
the two call directories gives identical bytes:

```sh
python3 -I ../../portable-transcendentals/attribution/verify_mpfr.py calls-platform calls-portable
```

(`verify_mpfr.py` writes its outputs next to itself; copy it here first to compare.)

The replays ran on Linux x86-64 with rustc 1.99.0:

```sh
MARTENSITE_CMATH_SELECT=platform MARTENSITE_CMATH_ATTRIBUTION_LOG=calls-platform \
  cargo test -p martensite-tkgeomalgo --features martensite-cmath/libm-attribution \
  --test blend-corpus --test toolkit -- \
  geometry_corpus_replay::fc_batten_complete_original_family complete_blend_families_in_original_order
MARTENSITE_CMATH_SELECT=portable MARTENSITE_CMATH_ATTRIBUTION_LOG=calls-portable \
  cargo test -p martensite-tkgeomalgo --features martensite-cmath/libm-attribution \
  --test blend-corpus --test toolkit -- geometry_corpus_replay complete_blend_families_in_original_order
```

The platform run used the answer files and seals before this change.
