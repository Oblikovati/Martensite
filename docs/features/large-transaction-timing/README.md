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

## Storage, concurrency and entry lookup

The final implementation reuses the existing document-state lock. A safe enum
stores either transaction evidence or the existing entry-index table in the same
112-byte slot. The private document core remains 176 bytes and each label node
112 bytes on the measured Linux x86-64 build. Unit tests check the enum's size and
alignment against the old map and preserve the measured core/node sizes.

Index enablement or public manual registration permanently switches to indexed
storage and native traversal. Disabling automatic indexing clears bindings but
never recreates hint state, so an earlier cursor cannot become eligible again.
Manual registration still works while the public automatic-index flag is false.
Enable/disable/rebuild, missing lookup output, callback order and nested undo are
covered by owned public controls.

Marking holds the document-state lease through local and ancestor flag publication.
Reset and index transitions take the exclusive lease. Each shortcut holds a
revision-checked shared lease through its pointer decision. Leases end before
attribute callbacks. Revisions saturate and disable tracking before wrapping.
Lock ordering remains state before node. Documents still do not promise concurrent
application-level mutation. Twenty internal controls cover identity, fallback,
depth, saturation, reset/traversal races and index-transition invalidation.

The generic traversal keeps untouched siblings outside fallible result plumbing.
The existing attribute loop and per-attribute method retain the baseline's opt1
inlining; outlined calls produced a measured full-commit slowdown. No public API,
input size, tolerance, callback order or timing threshold changes.

## Measurements and qualification limits

Matched opt1 binaries use debug information disabled, CPU 3 and one exact case per
process. Comparisons include v0.13.0 (`d051ff699`), main (`28a1c720`) and the local
fix. Final fixtures use the same 188-test harness and equal-length artifact names.
Raw records and percentile-bootstrap summaries (10,000 resamples, seed 489) are
retained in [references](references), including every rejected design and failure.

The final six-block default-path confirmation records 72 processes. All 24
candidate runs pass. One original-release commit fails its unchanged 1000:1 guard;
its exit 101 remains recorded, and the campaign wrapper exits 1. It is not accepted
validation. Large sparse commit medians fall from 1,977.5 to 25 µs and abort from
2,208.5 to 36.5 µs. Direct-child commit falls from 1,806 to 38 µs and abort from
1,757 to 8 µs. Deep commit-only medians are 1,352 versus 1,326.5 µs per 100 commits.
Creation and full-creation-commit paired intervals span zero. Normal allocator
accounting falls by 784 bytes in the large fixtures; that counter is glibc
`mallinfo2().uordblks`, not a census of live Rust allocations.

A separate final 12-block indexed comparison records 72 successful processes.
The 100,000-label full commit falls from 22,389.5 to 22,298.5 µs, paired difference
−97.5 µs with 95% interval [−124.5, −47.5]. Indexed sparse commit, abort, creation,
deep commit and peak-RSS intervals span zero or improve. The parent's initial
build overlap is retained in the record; it is not silently removed.

Peak memory remains subject to measurement limits. The final libtest large-abort
comparison reports +140 KiB versus the release, interval [4, 210], despite lower
allocator accounting. Minimal production-style indexed workloads have identical
allocator heap and heap-mapping RSS; their wide kernel-peak intervals span zero.
They show small executable-page differences and lower retained deep stack use.
Post-timing retained-state snapshots are not certified peak bounds. A separate six-block comparison of the original million-label public workload
without libtest passed all 36 processes. GNU time peak-RSS paired differences
versus the release were −70 KiB [−180, 246] for commit and −100 KiB [−248, 46]
for abort. Heap-mapping RSS was identical and allocator accounting fell by 912
bytes. This did not reproduce the libtest increase; finite samples do not prove
universal time or memory parity. Concurrent compilation was recorded, so this
comparison supports memory attribution, not isolated timing. Earlier small deep
probes used Python child high-water accounting with a pre-exec floor; those peak
values are excluded from qualification, while their retained snapshots remain
recorded.

The original separate-hint design added 48 core bytes and was rejected. A private
hot-field layout experiment kept sizes unchanged but doubled indexed scan time;
it was reverted. These implementations and disposable binaries are not shipped.
The green checkpoint `341e1d5d` remains available for provenance and comparison.

The current source passes 23 unit tests, 188 integration tests, 48 doctests and
strict all-target Clippy. Independent Spec and Standards reviews found no
remaining semantics or lock-order defect. The full canonical PR gate passed on
`e985d3a5` against pinned main `28a1c720`. The pinned local TSan runtime fails before toolkit startup;
the old checkpoint's hosted control executed all 15 tests successfully. The
updated control must execute the expanded 20 tests before merge.

No versions changed. Publishing the production fix requires an explicitly approved
tklcaf package bump; unchanged dependents retain their package versions.
