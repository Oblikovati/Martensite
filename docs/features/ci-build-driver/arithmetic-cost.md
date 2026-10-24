# Arithmetic and projection test costs

Follow-up to #250, tracked by #252 and #253. Measurements use the toolkit
sources at `aafe8a554958e5d04a0a626a2f9f98d7b32efe0a` on Linux x86_64 with
Rust 1.99.0. The host was shared with concurrent investigations; the results
are local comparisons, not forecasts for hosted CI or other platforms.

## Big-integer arithmetic

The candidate sets `profile.dev.package.num-bigint.opt-level = 3`. The test
profile inherits it. The dependency remains at 0.4.6. This change left kernel
compilation, assertions, tolerances and release profiles unchanged; the later
[TKMath optimization](assembly-test-cost.md) is measured separately.

| Workload | Baseline test seconds | Candidate test seconds | Results in each build |
| --- | ---: | ---: | --- |
| TKGeomAlgo toolkit, with certification report capture | 366.97 | 51.70 | 563 passed |
| TKMath unit tests | 15.89 | 6.16 | 297 passed |
| TKMath toolkit tests | 60.09 | 12.39 | 3,104 passed |
| TKMath doctests | 0.01 | 0.02 | 56 passed |

An earlier candidate run without report capture passed the same 563 TKGeomAlgo
tests in 44.31 seconds. The matched comparison above included identical capture
instrumentation in both builds. Both reference-contact and mapped-intersector
oracles remained enabled.

The captured reports cover all 108 offset/periodic-spline fixtures. After
ordering records by fixture ID, all 799,620 bytes matched between builds.
The reports include exact root equations, bounds, contact order, achieved errors
and refinement counts. Their SHA-256 was
`1398a9193de13a6ea2e7d263086eccd98de95e2ff16731dd75291c2a0cdba317`.
This establishes equality for those reports on this host, not every possible
input or target platform. Temporary print instrumentation was removed.

The full Cargo commands, including compilation, took 370.36 versus 53.07 seconds
for TKGeomAlgo and 89.28 versus 31.32 seconds for TKMath. User CPU fell from
899.59 to 186.97 seconds and from 308.41 to 114.33 seconds respectively.
Peak process RSS for those commands was 1,127,852 versus 1,132,764 KiB and
1,420,756 versus 1,419,680 KiB; these include compiler processes and do not
establish a test-runtime memory reduction.

Reproduce the baseline with the override set to zero and the candidate with
the override set to three:

```sh
cargo test --locked --config 'profile.dev.package.num-bigint.opt-level=0' -p martensite-tkgeomalgo --test toolkit
cargo test --locked --config 'profile.dev.package.num-bigint.opt-level=3' -p martensite-tkgeomalgo --test toolkit
cargo test --locked --config 'profile.dev.package.num-bigint.opt-level=0' -p martensite-tkmath
cargo test --locked --config 'profile.dev.package.num-bigint.opt-level=3' -p martensite-tkmath
```

The checked-in tests retain their independent assertions. No wall-clock threshold
was added to CI: scheduling noise would make it a flaky correctness gate. The
full workspace PR gate and later release platform matrix remain required.

## Polar projection

The fourth `pl_polar` fixture (zero-based index 3) reproduced the grid workload
without running all 164 requests. It passed its existing expected-output check
in 12.18 seconds wall time and 12.15 seconds user CPU, with 44,692 KiB peak RSS.
Temporary instrumentation counted 5,808 extrema initializations with 4,276
distinct grid dimensions/bounds; 1,532 repeated a previous key. The most common
key appeared 41 times. These counts alone do not establish cache-hit savings:
they omit the point, tolerance, surface identity and revision.

`GenExtPS::init_grid` already caches surface samples within an instance.
Polar projection creates fresh `ExtPS` instances, including searches on a
trimmed surface whose bounds move with the curve parameter. Its edge and cell
distances also depend on the point being projected. A cache keyed only by
surface identity would therefore be incorrect. Reuse must stay within an
operation and account for bounds, tolerances and geometry changes.

Optimizing TKGeomBase alone at level 1 reduced this fixture to 9.28 seconds,
while compilation rose from 8.77 to 30.55 seconds. This is a smaller benefit
than the big-integer change and does not justify adding it to #252. No toolkit
profile or numerical algorithm change is included. The broader grid work
remains under #253; no frozen fixture, sample count or tolerance was changed.

A broader TKGeomBase trial at level 1 passed all 2,018 toolkit tests in 613.96
seconds (628.13 seconds including compilation). It ran concurrently with other
validation and had no matched full-suite baseline, so it establishes fixture
coverage under that profile but does not establish a full-suite speedup.
