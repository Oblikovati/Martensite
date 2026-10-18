# Safe-toolkit migration: partial closeout

On 2026-10-18 the maintainer directed us to release the accepted work, reject
performance degradation, and close Feature #80 with its remaining constraints
documented. The release is 0.3.2, based on main `0d4548b`. This ends this migration
effort; it does **not** establish the original objective of 100% safe toolkit Rust.
The outstanding tasks and bugs remain open. Their unchecked requirements are not
waived, and custom storage is not reclassified as a system API exception.

## What ships

The accepted production changes were already integrated into main. The closeout
adds documentation and release metadata, not another storage or executor rewrite.

- **CellIndex ownership (#82):** a safe typed owner replaced the raw heap owner
  without changing the pinned size, exposed pointer view, failure shape or value
  behavior. The existing allocator-api2 dependency supplies fallible ownership.
- **Sequence registry (#86):** checked `NonNull` construction replaced an unchecked
  conversion of the same `Box::into_raw` result. Allocation and destruction paths
  remain unchanged; the wider raw registry migration is unfinished.
- **Native records (#89):** explicit field initialization or existing safe defaults
  replaced selected zero-initialization blocks for account, calendar, file, uname,
  memory and system-information records. Existing native calls and conversions
  remain where a matching replacement was not established.
- **Native interfaces (#89):** safe standard-library or pinned rustix, nix and
  winsafe interfaces replaced selected environment/error reads, path operations,
  IDs/configuration queries, sleep and Windows clock/system queries. Each bounded
  conversion retained its behavior fixtures; this is not a claim that every FFI
  call was removed or that the dependencies themselves contain no unsafe code.
- **Verification (#81):** source and compiler-expanded inventories, per-target
  baselines, dependency reporting, failure-safe capture and compatibility fixtures
  now guard subsequent work. Reports count emitted compiler spans, including
  repeated compilation units, not unique unsafe sites.

Detailed evidence is kept in this directory's per-change JSON files and the
associated merged PRs. Some JSON files record pre-merge pending checks; the final
PR checks and exact-release-SHA matrix are the authoritative integration evidence.
Research and benchmark sources under `experiments/` are not replacement production
implementations. Already integrated output-seek and release-license fixes also
ship from main and are separately listed in the changelog.

## Performance evidence and its limits

The policy was to preserve or reduce cost, not trade speed or memory for a smaller
unsafe count. Green correctness CI alone was not sufficient for a storage rewrite.

For CellIndex, the identical release workload reduced whole-process allocations
from **240,293 to 160,276** and allocated bytes from **19,215,867 to 12,174,549**.
Observed elapsed samples fell from 43.2–47.1 ms to 5.5–6.4 ms; peak RSS samples
overlapped (baseline 9,872–10,224 KiB; candidate 9,708–9,904 KiB). These include
test-harness costs and shared-host noise, not an isolated end-to-end CAD benchmark.
The ownership/layout fixtures stayed unchanged and 11 focused Miri tests passed.
See [baseline](cell-index-baseline.json) and [candidate](cell-index-safe-owner.json).

The small record and wrapper changes preserve allocation strategy and call shape
where documented; the path-permission conversion, for example, uses the same
single libc call through rustix's pinned libc backend. Their evidence is source
inspection plus behavior/native tests, not a universal cycle-by-cycle performance
proof. This release makes no blanket speedup claim across all APIs or platforms.

## Rejected or deferred candidates

### Persistent worker dispatch (#87, PR #107)

The scoped Rayon candidate removes borrowed-job lifetime erasure and manual
thread-safety assertions, but its measured resource costs were unacceptable.
For the same 1,000-launch workload across 1, 3, 8 and 32 participants:

| Metric | Existing executor | Candidate |
| --- | ---: | ---: |
| Allocations | 799 | 12,484 |
| Allocated bytes | 222,350 | 4,055,701 |
| Peak live heap bytes | 142,930 | 449,767 |
| Median peak RSS, KiB | 6,876 | 7,560 |
| Bytes retained at exit | 596 | 64,572 |

Shared-host timing medians also showed slowdowns (about 5.8% at 32 participants);
these timings are diagnostic, while the allocation increase is independently
reproducible. Retention did not grow between the sampled 100 and 1,000 pool cycles,
but that is not proof of a general bound.

A minimized warmed two-participant probe isolated the submission cost: 100 launches
added 100 scope-job allocations (6,400 bytes), plus one queue allocation (1,520
bytes). Pinned Rayon scope submission creates a heap job for the submitted closure.
The larger retained footprint includes crossbeam-epoch state. Removing failure
collection or optimizing caller-only dispatch did not eliminate the root cost.

Alternatives had to preserve borrowed lifetimes, physical worker identity, caller
participation, reservations, nested progress, deterministic failure order and
bounded stack use. Broadcast waited on unrelated busy workers; other explored
scoped pools lacked targeted dispatch or nested progress. Those failed trials do
not prove that a better safe executor is impossible. PR #107 is excluded; its
branch and measurement evidence remain available. Bugs #106 and #109 are not
claimed fixed by shipping the existing executor.

### Clone-free bulk copy (#83)

The allocator-api2 0.4.0 vector lacks a matching safe bulk-copy publication path
that bypasses custom `Clone` while preserving allocator selection, capacity and
allocation count. Tested `copied()`/`resize_with` paths compiled to scalar loops
with per-element length/capacity work; a final slice copy added another full pass.
Helper measurements showed roughly 2–8× slowdowns, not an acceptable replacement.
`extend_from_slice` invokes `Clone`; a temporary boxed slice changes allocation
and capacity; filling spare capacity still needs unsafe length publication.
An established upstream safe bulk-copy interface is a possible follow-up, not an
approved local dependency that merely hides custom unsafe. Padding preservation
was not established as a public safe-API contract; the blocker is cost and API fit.

The separate Array1 slice-copy candidate also needs a raw-alias decision (#184,
#88). Original code passed the differential Stacked Borrows probe while the slice
reborrow candidate failed; reacquiring the pointer and Tree Borrows passed. This
is model-dependent compatibility evidence, not a blanket claim of undefined
behavior or impossibility. The candidate is excluded.

### Owned List storage (#86)

The safe prototype improved many phases but regressed `iter_mut`: the safe
measurement rerun observed 280 µs versus 158 µs and one allocation versus zero.
It builds an O(slot-capacity) table of disjoint mutable references before following
link order. A direct-borrow table trial still allocated and remained slower.
The apparent overall win is not like-for-like: the prototype bypasses physical
allocator routing, which accounts for much of the existing list's cost (about
five allocations per node in the measured Common allocator path).

It also does not preserve raw `base_storage()` traversal, all cursor/arena reuse
contracts, or all destructor-panic behavior. Therefore neither its faster total
time nor its lower peak heap justifies merging it. See
[List feasibility and measurements](list-family-safe-storage.md).

### Bounded account-name extraction (#89)

Candidate `a94555990766e550f5a32907b16cf1f4a2a67ed0` replaces a foreign C-string
dereference with checked slicing of the owned buffer. Four-platform native run
[39118068198](https://github.com/Oblikovati/Martensite/actions/runs/39118068198)
passed; all four clean exact-revision inventory artifacts matched their baselines.
Local matched allocation counts were equal, and Linux byte/errno comparisons
passed for success and an injected account-file access failure.

The explicit ARM byte/errno comparison and timing tests were ignored by ordinary
CI. Shared-host timing medians are not controlled no-regression evidence. This
candidate is therefore **not included**, despite green CI. Its branch
`experiment/89-bounded-account-name` retains the code and evidence.

## API and system constraints that remain

- **Allocator ownership (#84):** public unsafe extension traits, raw adoption/free,
  physical provider routing, callbacks, arena reset and failure behavior cannot be
  replaced by a logical byte-accounting reservation without changing the contract.
  The reservation prototype is not an allocator replacement.
- **Contiguous storage (#85):** ordered in-place destruction, retained zero-stride
  slots, unpublished-tail panic behavior, stable addresses and raw views need
  compatible owners or explicit migrations, without adding `Clone`/`Default` bounds.
- **Public raw/layout interfaces (#88):** generic repr(C) named vector fields with
  const borrowed-array views, matrix mapped `&Self` views, intrusive node projection,
  raw cursors and public unsafe functions/traits are API conflicts. Safe bodies
  alone do not remove unsafe declarations or make a crate satisfy `forbid`.
- **Process-wide state (#95):** safe Unix environment mutation cannot rely on
  undocumented caller synchronization. A proposed unsafe process-mutation boundary
  plus a safe owned child-process environment requires a reviewed API migration.
  Locale and other process-global state need similarly enforceable contracts.
- **System adapters (#89):** error/errno behavior, raw non-UTF8 names, fixed buffers,
  interruption/retry policy, handle ownership, partial output and numerical behavior
  prevent blind wrapper substitution. Loader ownership/execution concerns (#170,
  #171) and host-query shared storage (#150) remain separate bugs. Platform libm,
  locale, clocks, memory queries and remaining native calls are not all proven
  unavoidable merely because this effort stops.

Costs and compatibility preferences are **not** system API exceptions. The current
tkernel/cmath allowances and source/compiler ratchets remain in force; no new
permanent exception or public API break is approved by this closeout.

## Follow-up disposition and release gates

#82 is complete. #81 and #83–#90 remain open with their unmet criteria; bugs remain
open independently. #80 is closed as a maintainer-directed partial delivery, not
as satisfaction of its original all-safe objective. PR #107 and unmerged
experiments are preserved, not merged or deleted as completed work.

The release requires `cargo xtask pr`, a local `cargo xtask release --version 0.3.2`,
and a green four-platform release rehearsal on the exact main commit before its
tag. The tag workflow must publish the three source crates and checksums. Final
run/tag links are recorded on #80 after publication. No experimental performance
exception is part of that approval. The humanizer skill was unavailable; its
prose pass is left to maintainer review.
