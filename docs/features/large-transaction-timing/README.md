# Sparse document transaction traversal (#489)

The Linux v0.13.1 matrix failed both existing large-transaction timing checks.
Commit took 7,547 µs against a 3,943 µs limit; abort took 7,525 µs against
4,045 µs. Retrying the failed job reproduced both failures. A scoped hosted
comparison with one and two test threads also failed. The tklcaf crate was
unchanged from v0.13.0, so these observations establish an existing cost problem,
not a newly introduced source regression. The hosted records are in
[489-hosted-unchanged.json](references/489-hosted-unchanged.json).

The original traversal visits the root's broad sibling list even when only one
attribute changed. Each unrelated sibling incurs a node lock. The candidate
retains one exact modified-node identity per document and follows its ancestor
path when that evidence is sufficient. It retains the existing child order and
attribute callbacks. A second dirty node, manual public flag mutation, nested
commit, failed walk, revision exhaustion or path deeper than 16 selects the
original traversal. Deep first marks are refused at admission. The native
lifecycle guard stores no ancestor array; only an eligible sparse helper captures
that bounded array under a current-revision lease. No fixture, timing ratio, tolerance or public API changes.

## Concurrency and callbacks

Marking holds a hint lease through publication of local and ancestor flags.
Reset and traversal entry take the exclusive lease. Each shortcut holds a
revision-checked shared lease through the actual pointer decision; a new distinct
mark cannot publish between validation and skipping siblings. Leases are released
before attribute callbacks. Nested transactions invalidate the outer shortcut.
The revision saturates and permanently disables the hint before wrapping.
Lock ordering is hint before node. Documents still do not promise concurrent
application-level mutation; these controls preserve marking safety at the
existing shared entry points.

Fifteen internal controls cover identity, fallback, depth, saturation and
channel-coordinated reset/traversal races. Eleven owned history controls cover
ancestor and sibling changes during callbacks, nested transactions, failure and
undo. They pass unchanged on the candidate, current main and v0.13.0. A wide
direct-child control moves the lookup cache away from the modified label. A
256-level control exercises the native fallback. The original full-size timing
fixtures and 1000:1 assertions remain byte-for-byte unchanged.

The build driver adds the affected `document-transactions` ThreadSanitizer
control for these internal lease tests. Local pinned-runtime startup terminated
with SIGSEGV before executing toolkit code. That is unavailable local evidence;
hosted sanitizer success is still required before merge.

## Performance evidence and limits

Matched binaries use opt-level 1, debug information disabled, CPU 3, one exact
test per process and the same 181-test harness. Both v0.13.0 (`d051ff699`) and
main (`28a1c720`) are recorded. GNU time measures peak RSS; temporary
`MemInfo::HeapUsage` probes ran after the original timed intervals. This counter
uses glibc `mallinfo2().uordblks`; it is allocator accounting, not a census of live
Rust allocations. Probe sources were removed before the canonical gate.
Compressed raw records and summaries retain every campaign, including rejected
designs, in [references](references).

The earlier fixed-storage campaign found a deep commit-only increase of +15 µs
per 100 commits against main, interval [5.5, 46], and an abort peak increase in
three blocks. The refined design removes ancestor-array movement from native
traversals. Its final campaign records 12 balanced blocks and 144 processes.
All 48 candidate runs passed. One original release abort failed its unchanged
timing guard: 3,018 µs exceeded creation/1000, 2,962.316 µs. That exit 101 and
its raw measurements remain in the dataset; it is not accepted validation.
The campaign wrapper exits 1 to retain that fact.

| Workload | v0.13.0 median | Main median | Candidate median |
| --- | ---: | ---: | ---: |
| Large sparse commit | 2,284 µs | 1,996 µs | 27 µs |
| Large sparse abort | 2,095 µs | 2,009 µs | 41 µs |
| Large creation, commit fixture | 2,988,942.5 µs | 3,035,440 µs | 2,977,631 µs |
| Direct-child sparse commit | 1,724 µs | 1,709 µs | 34 µs |
| Direct-child sparse abort | 1,746 µs | 1,694 µs | 9 µs |
| Deep commit, 100 repeats | 1,322.5 µs | 1,321 µs | 1,325 µs |
| Deep mutation and commit, 100 repeats | 2,330.5 µs | 2,341 µs | 2,091.5 µs |

Paired candidate-minus-baseline differences use a percentile bootstrap of the
median, 10,000 resamples, seed 489. The deep commit-only difference is +1.5 µs
with 95% intervals [-0.5, 6.5] against the release and [-3, 8.5] against main.
Creation and full-creation-commit intervals span zero or improve. The prior
repeatable deep slowdown was not detected in this campaign.

Large-commit peak RSS differs by +88 KiB [-36, 156] against the release and
−86 KiB [-232, 10] against main. Abort differs by +86 KiB [-50, 254] and
−34 KiB [-120, 54], respectively. These finite measurements detect no peak-RSS
increase; intervals spanning zero do not prove universal memory parity.

The fixed storage cost is explicit: private `DataCore` grows from 176 to
224 bytes, while each label node remains 112 bytes. Normal allocator accounting
increases by 80 bytes per document. Disabling glibc tcache identically in a
separate attribution control reduces the difference to exactly 48 bytes, matching
the core growth. That control ran concurrently with compilation and supplies no
timing evidence. No per-node index or allocation is added. This constant document
cost remains a tradeoff under the user's no-degradation requirement.

The prior exact-source canonical PR gate passed against `28a1c720`; its
formatting-only predecessor failed before package checks. The refined source has
15 passing internal controls, 181 passing integration tests and strict all-target
Clippy. Its new canonical gate and hosted sanitizer result must pass before push
and merge, respectively. Independent Spec and Standards reviews found no
remaining source defect. No universal performance-parity claim is made.

No version change is included. Publishing the production fix requires an approved
tklcaf package version change; unchanged dependent packages must retain their
versions. Probe cleanup is recorded in
[489-probe-cleanup.json](references/489-probe-cleanup.json).
