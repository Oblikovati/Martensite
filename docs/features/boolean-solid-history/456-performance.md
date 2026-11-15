# Performance of enabled history (#456)

The matched campaign compared native Boolean source from main `77053b4cd` with
fix `61fd7100`. The baseline libraries came from the #446 lane; its area-only
cross-check change is not called by these primitives or native Boolean paths.
This is a scoped native-path comparison, not whole-checkout equality with main.
Both library sets used the optimized test profile (level 1), with level-1,
debug-free temporary probes. Library and executable hashes are in the raw record.

The measured window was 2026-11-15 03:47:24.878–03:48:06.522 UTC (41.644 s), on
pinned CPU 3 while the coordinator avoided local CPU-intensive work. The 304
observations include 24 balanced before/after pairs for each history-enabled
case, eight pairs per disabled-history control, warmups and two file-cold pairs
per configuration. Non-destructive mode was true and parallel execution false.
Each timed operation constructed fresh local inputs; no input flags were restored.

## Workloads and acceptance

The cases were two overlapping side-10 boxes offset by `(5,5,5)`; the recorded
side-10 box and radius-2 through-cylinder; 64 separated unit boxes at x = 3i;
and 12 side-2 boxes in a chain at x = i. History was explicitly on or off.
Every configuration passed native topology, then geometry, then self-intersection
and small-edge checks, followed by tolerance hierarchy checks. All before/after
result geometry bytes matched. Measured maximum tolerances are retained; no
modelling or acceptance limit was widened.

The baseline's incorrect solid deletion relations differ intentionally from the
corrected history in the merged cases. The unchanged 64-solid case retains its
existing exact-survival history on both versions. Geometry and operation inputs
are matched; no history output was normalized to manufacture equality.

## Timing and memory

Mean native build time per operation with history enabled:

| Workload | Before (ms) | After (ms) | Change |
| --- | ---: | ---: | ---: |
| Overlapping boxes | 14.076 | 13.954 | −0.86% |
| Box and cylinder | 8.144 | 8.039 | −1.29% |
| 64 disconnected boxes | 75.061 | 73.922 | −1.52% |
| 12-box chain | 106.795 | 105.559 | −1.16% |

All enabled-history paired build-time intervals were below zero in this sample.
Disabled-history controls also trended about 1% faster, so these results are not
attributed to an algorithmic speedup. No timing regression was observed on these
workloads. The native build timer excludes fixture construction and destruction;
the separately recorded process CPU and elapsed measurements include them.
Externally observed elapsed time also includes the 4 ms process-polling granularity.

Warm peak-RSS medians decreased on all four enabled cases. Sampled anonymous
medians changed by +12, +6, +6 and −4 KiB; the corresponding disabled controls
changed by +14, −6, −2 and −4 KiB. File-cold total-RSS observations remained
variable and are preserved without subtraction or normalization. No allocation
count was measured. Exact peak-memory parity and universal performance parity
are not established by these primitive, sequential, test-profile workloads.
The native high-water RSS includes executable-backed pages; anonymous peaks are
sampled observations, not certified upper bounds.

## Preexisting input-state qualification (#461)

The disconnected inputs change their observable `Free` flag from true to false
on both the baseline and the fix, immediately during the operation and before
any diagnostic checker. Every complete serialized source differs by exactly
one byte; the other flags and all geometry bytes remain unchanged. The same
behavior reproduces with only two unit boxes and is tracked separately by #461.
It differs from #457's result invalidation after later source mutation.

The raw changes were retained and characterized, not restored or hidden. Fresh
inputs for every timed operation avoid measuring a mixture of initial and
already-frozen states. These input-flag changes predate #456. The benchmark does not establish full
non-destructive input isolation.

## Evidence and cleanup

`references/456-matched-performance.json.gz` preserves every raw observation and
its validation/provenance metadata. `456-performance-summary.json` contains the
compact derived results; the full derived statistics are also retained compressed.
Owned result geometry and before/after Free-flag snapshots support the acceptance
comparison. The temporary Rust implementation and both executables were removed
after checking active process and file users; `456-performance-cleanup.json`
records their hashes and removal. No experiment implementation is retained.
