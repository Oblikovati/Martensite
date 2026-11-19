# Planar certificate validation cost (#449)

The issue was reproduced from `main` commit
`919b8e31ef0cdc974de3f838be091d7a26ab5c7a` using the public
`build_planar_regions` API. The input came from the owned glyph fixture at
`2eb260c0` (`p5-owned-glyphs.tsv`, SHA-256
`c6061510a1f4df6b5aadf1ebed7aa9a8df43ddbb2409841300d06348f4c91dd8`).
The two disjoint `s` instances have 56 polynomial pieces and two contours.
They were accepted as two regions in 50.54 seconds, using 50.52 CPU seconds
and 39,688 KiB peak RSS.

For that input, all 1,596 numerical pair queries took about 0.25 seconds.
The following census and contact checks repeatedly validated the whole
work ledger. Its reuse check searched a growing vector for every report.
Repeating that quadratic check for every pair domain made this part cubic
in the number of pair domains, which itself grows quadratically with the
number of pieces.

The fix uses indexed membership flags and shares ledger validation during
an immutable census borrow. It also binds each segment to its unique captured
construction once, sharing the immutable source-history and matrix checks.
Each domain still validates its occurrence,
parameter-axis mapping, history, source coefficients, report and effective
policy. Native geometry can change through interior-mutable aliases even
while the census is borrowed. Borrowed certificate views still check source
geometry fresh for every segment on every certificate lookup. The regression tests
include copied-curve, stored-pcurve and support-plane alias mutations,
forged ledger entries and a 56-edge public profile with independent closed-form areas.
The same accepted two-glyph workload took 5.199 seconds (5.19 CPU seconds),
with 38,404 KiB peak RSS after the fix, about 9.7 times faster.
No pair domains, budget charges or correctness checks are omitted. No
numerical policy, tolerance, solver, public API or version changes.

The full fixture has 239 pieces and 20 contours. Investigation exposed a
separate limit: its shared 4,096-bisection budget runs out before pair
109/120 (zero-based). That pair completes individually with ten bisections;
with no remaining budget it correctly refuses with `SubdivisionLimit`.
This fix preserves that refusal. Full-fixture acceptance remains part of
the surface-normal sketch feature.

Measurements used Linux x86_64, Rust 1.99, two build jobs, incremental
compilation disabled, debug information disabled and stripped. Existing
package optimization overrides were retained. Timing is a local observation
under a shared host, rather than a certified runtime bound. Temporary
probes were removed after investigation.

Verification on Linux:

- `cargo test -p martensite-tkbo --lib sketch_region -- --test-threads=2`:
  1,591 passed; no failures or ignored tests.
- Final certification-only rebuild: 85 passed.
- The existing public `tests/profile_extrusion/main.rs` source suite was
  compiled with `rustc --test` against the rebuilt library and its native
  dependencies: 45 passed, including the new pair-matrix regression.
- `cargo clippy -p martensite-tkbo --lib -- -D warnings`, formatting and
  whitespace checks passed.

The broad unrelated Boolean test campaign was stopped; it supplies no
acceptance evidence. The integration gate and other platforms remain the
coordinator's checks before merge and release.

## Owned graph batches (#555)

The graph-only profile entry point constructs fresh native edges from owned
coefficients and returns a scalar graph certificate. Its native handles stay
inside that call. A private validation view borrows the request owner and
checks the complete source inventory before a read-only batch. Certificate
lookups in that batch reuse this source check. The next batch validates
the sources again; no validity state is stored on a returned census or
contact transaction.

Preparation that returns native handles keeps the live validation path.
Borrowed certificate views still recheck curve, pcurve and support geometry
on each lookup, including after an alias changes them. Both paths retain
the existing per-domain source, history, report and policy checks.

Contact transport also indexes immutable parent reports for each read-only
batch. Matching preserves the original source-vector order and full parent
identity. Equal parent-vector lengths and the original one-way membership
test remain significant, including their behavior with duplicate parents.
The index contains source-row positions and supplies no native freshness
authority.

The focused controls compare owned and live certificates, rendered debug
bytes and callback traces. They also count actual source validations and
exercise source revisions, native alias edits and mismatched source
manifests. Focused verification on Linux x86_64 passed nine owned-path
controls, nine source-index controls and six existing borrowed-view
controls. The owned controls include the earlier-failure and native
callback-error cases. The final index uses contiguous candidate rows and sorted admission keys.
The earlier #449 measurements above describe that fix only.

### Matched public workloads

The before build used clean main `9e11ab3864c8786ce05b3bdf35123f4a87fc1b05`.
Both builds used Rust 1.99.0 with ordinary optimization level 0, the existing
Math/num-bigint level 3 and TKBO level 1 overrides, and debug information
disabled. Each study pinned both versions to one idle physical core.
The consumers called the same public API with the existing fixtures and
resolution; their timers excluded validation and export.

| Accepted workload | Median API time before | After | Median peak RSS before | After |
| --- | ---: | ---: | ---: | ---: |
| Two rectangles, 56 curves | 4.496840 s | 0.395095 s | 35,516 KiB | 35,444 KiB |
| Two-span parabola segment | 16.304809 ms | 15.365912 ms | 29,172 KiB | 29,316 KiB |
| Rational two-span half-disc | 27.221607 ms | 26.583895 ms | 29,436 KiB | 29,552 KiB |

The rectangle study alternated three processes per version. Each small
study alternated nine processes per version, with 32 calls per process;
the table reports the median process average. Every call checked its
independent area, source mapping, ordered validation and input isolation.
Public evidence and native BRep bytes matched across processes and across
all repeated small calls. The 56-curve API was about 11.4 times faster.
These are local measurements on a shared host, not runtime bounds.

Peak RSS was lower for the rectangle case. The small-case medians rose by
144 KiB and 116 KiB; the ranges overlapped: 28,892 to 29,288 versus
29,112 to 29,540 KiB for the parabola, and 29,132 to 29,724 versus
29,120 to 29,704 KiB for the half-disc. An additional ASLR-disabled memory
attribution control found the same sampled heap (504 KiB), anonymous
memory (84 KiB) and stack (132 KiB) maxima. Its extra resident pages were
in executable mappings. That control diagnoses residency and does not
replace the default-layout measurements or grant a memory exception.

[Raw measurements and provenance](555-performance.json) retain both the
initial tree-index study and the final contiguous-index study. The Linux
x86_64 PR gate passed with `cargo xtask pr --base
9e11ab3864c8786ce05b3bdf35123f4a87fc1b05`, including lint, clippy, builds
and affected-package tests. [Hosted checks](https://github.com/Oblikovati/Martensite/actions/runs/44470644480)
passed on `63a60f5253a98c68f8081ef05248d6cf7943db11`. On 2026-11-19 the
maintainer accepted the measured 144 KiB and 116 KiB small-case RSS increases
for [PR #556](https://github.com/Oblikovati/Martensite/pull/556), which merged
as `b7dc518b412036048e4f8582cf600aa040f86d64`. This records approval of that
specific tradeoff; it grants no general performance or memory exception.

## Census report reuse (#567)

Each Census enumeration owns an ordered index from its canonical source pair
and query mode to the first retained report address. Existing report keys enter
the index once. A cold solver result is indexed at its actual append position;
later occurrences reuse that position, including when its outcome is a capture
failure or incomplete proof. The index is discarded when enumeration ends.
Report vectors keep their original order and remain the evidence owners.

The index stores execution addresses within one Census. Source, policy and
revision checks still run through the existing certificate bindings. Solver
execution order, shared remaining budgets and cancellation callbacks stay in
the enumeration driver. The lookup tree is never traversed to publish geometry
or assign persistent identities.

Regression controls run indexed and literal lookups through the same solver
driver, comparing retained graph coefficients, roots, membership and exceptional
signs, contacts, policies, work and callbacks. They cover reversed aliases,
query modes, duplicate keys, cached failures and cancellation checkpoints.
A deterministic counter control expects 21,945 record visits for 210 cold misses
in the literal oracle; indexed lookups read the existing record inventory once.
These controls establish lookup semantics; whole-operation time and peak memory
require the same accepted workload and matching build provenance.
