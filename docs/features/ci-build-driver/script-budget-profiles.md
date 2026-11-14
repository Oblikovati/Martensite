# Script-budget test profiles

`cargo xtask pr` and per-package CI now choose script test optimization in
`scripts/build/src/cargo/script_budget_profiles.rs` (#447). The policy retains
the eleven packages previously listed in the workflow: TKBO, Boolean cases,
TKBool, TKFillet, TKFeat, TKOffset, TKDESTL, TKMeshVS, TKDESTEP, TKDEIGES and
TKDEOBJ. Their test commands set both `CARGO_PROFILE_DEV_OPT_LEVEL=1` and
`CARGO_PROFILE_TEST_OPT_LEVEL=1`. The second setting prevents an inherited test
profile from silently cancelling the intended optimization.

Other selected packages keep their ambient profile. Mixed selections are split
into two Cargo commands without expanding the selected package set. Workspace
selections resolve actual members before grouping, so a reduced workspace never
requests an absent script package. Exclusions apply to both groups. Every group
retains `--no-fail-fast`, and the driver runs subsequent groups after a failure.
Case-runner feature coverage and recorded renderer-font configuration remain
attached to their selected packages.

The workflow delegates the package policy to the driver. Its release-wide
level-1 setting remains in place for all packages in each release shard. Clippy,
ordinary build commands and release-build optimization retain their existing
settings. No case body, assertion, budget, ignore marker, geometry code, package
version or release tag changed.

## Bug #447 investigation

The earlier feature gate used the unoptimized, stripped-debug profile for the
Boolean suite. That matched CI's debug-symbol settings but omitted the package
job's level-1 optimization. The claim that it matched the complete Boolean CI
profile was therefore incomplete. The local gate's command did not carry the
workflow's package optimization rule. A recording-runner regression failed on
that missing environment before the fix.

The following measurements used main `919b8e31ef0cdc974de3f838be091d7a26ab5c7a`,
Rust 1.99.0 on Linux x86_64, and test-data revision
`34d65afd72cd3ca8ac1f9d17e5ca408b30e9b4b6`. Compilation used two jobs, disabled
incremental compilation and stripped debug symbols. The native case sources
and budgets were unchanged. Measurements ran on a shared development host;
concurrent feature work limits performance comparisons.

| Profile and execution | Result | Elapsed | User CPU |
| --- | --- | ---: | ---: |
| Unoptimized exact bug32701 | Passed | 21.86 s | 21.59 s |
| Unoptimized full normal corpus | 3,821 passed, 12 ignored | 608.87 s | 8,621.97 s |
| Unoptimized four-case campaign, two CPUs, default libtest threads | Passed | 109.22 s | 198.42 s |
| Unoptimized four-case campaign, one CPU, four libtest threads | bug32701 failed | 197.26 s | 194.93 s |
| Level 1 exact bug32701 | Passed | 4.74 s | 4.63 s |
| Level 1 four-case campaign, one CPU, four libtest threads | Passed | 55.70 s | 54.09 s |
| Level 1 full normal corpus | 3,829 passed, 4 ignored | 348.08 s | 2,120.25 s |

The normal main run did not reproduce the historical failure. In that run the
bug32701 child reached 44.27 seconds elapsed and 38.14 seconds CPU before its
passing exit. In the forced one-CPU/four-thread campaign it reached 58.99 seconds
elapsed with only 14.13 seconds CPU, then exited 2 under its unchanged 60-second
policy. This establishes elapsed-budget contention for the constrained run.
It does not establish a native regression or a normal-run failure on current
main. The campaign used real `perf_modalg::bug25788`, `perf_modalg::bug26542`
and `simplify::A3` siblings; no synthetic CPU load was added.

Rust reported 32 available workers on the normal host and one under the
one-CPU affinity. The forced campaign explicitly requested four test threads.
The two-CPU campaign kept libtest's default. The child's four threads include
the harness and process-policy workers; the case's face intersection and direct
Boolean configurations disable geometry parallelism. These data do not isolate
why its user CPU time increased in the full parallel corpus.

The fix addresses the measured local/CI configuration mismatch. It introduces
no case-specific serialization. The existing `alone` gate remains unchanged.

## Verification

The regression failed because the old Boolean test command had no optimization
environment. Driver controls now check all eleven package policies, unchanged
ordinary-package commands, mixed selection and feature coverage, exclusions,
reduced workspaces, continued execution after a group failure and workflow
ownership of the policy. All 337 driver tests and strict all-target clippy passed.
The same constrained campaign that failed unoptimized passed all four cases
with level 1 in 55.70 seconds; its bug32701 child passed after reaching a sampled
17.03 seconds elapsed and 4.04 seconds CPU. The full optimized corpus passed 3,829 cases with 4 ignored in 348.08 seconds.
Eight existing optimized-only cases ran that the unoptimized profile excluded.
No ignore annotations changed. These runtime measurements exclude compilation;
the fresh optimized profile took 4 minutes 21 seconds to build, and a paired
cold-build comparison was not performed.

The local `cargo xtask pr` gate passed source and expanded compiler inventory,
formatting, whitespace, clippy, build and 337 driver tests. Final test-helper
extraction and CRLF normalization of the workflow assertion were followed by
another full driver test run and strict clippy. The coordinator will qualify the
combined integration tree before pushing.

[Structured measurements](references/bug447-measurements.json) retain aggregate
CPU and maximum-process RSS from `time -v`; maximum-process RSS is not the sum
of simultaneous child memory. The [normal](references/bug447-normal-process.tsv),
[constrained](references/bug447-constrained-process.tsv) and
[optimized constrained](references/bug447-optimized-process.tsv) process samples
record elapsed/CPU seconds and thread counts. The temporary watcher and CPU
parallelism probe were removed after investigation.
