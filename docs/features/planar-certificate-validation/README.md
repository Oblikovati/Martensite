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
while the census is borrowed, so source geometry must still be checked
fresh for every segment on every certificate lookup. The regression tests
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
