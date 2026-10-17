# Windows thread queries and priority boundary

Feature #80 / task #89 research at
`c77e8d94dbb60dc2d32f910fc3c002813b21c7e8`, covering
`crates/martensite-tkernel/src/osd/parallel/thread/native.rs`.
The accompanying baseline extracts the existing processor-count cast and adds
fixtures without replacing native calls or changing dependencies. Native
experiments remain pending. The unavailable humanizer prose pass remains for review.

## Two direct candidates

The already-pinned winsafe 0.0.29 provides `GetCurrentThreadId() -> u32`,
whose body is one native call, and `GetSystemInfo() -> SYSTEM_INFO`, which
initializes a record, calls the native function once and returns the record.
Neither wrapper adds status interpretation or LastError access. Retain the
original `as usize` conversion for thread ID and `dwNumberOfProcessors as i32`
for processor count. The exact source revision from `.cargo_vcs_info.json` is
`71ed88c2a0d18b03ee452f6d4261f4c22f443483`:
[current ID](https://github.com/rodrigocfd/winsafe/blob/71ed88c2a0d18b03ee452f6d4261f4c22f443483/src/kernel/funcs.rs#L346),
[system information](https://github.com/rodrigocfd/winsafe/blob/71ed88c2a0d18b03ee452f6d4261f4c22f443483/src/kernel/funcs.rs#L816).

The winsafe SYSTEM_INFO is repr(C), with the architecture/reserved pair in
place of the native union's matching struct view; count remains u32. Its
`impl_default!` uses zero initialization internally, unlike SYSTEMTIME's
derived field Default. This is an established dependency trust boundary, not
a claim that the dependency contains no unsafe. See
[record layout](https://github.com/rodrigocfd/winsafe/blob/71ed88c2a0d18b03ee452f6d4261f4c22f443483/src/kernel/structs.rs#L956) and
[default macro](https://github.com/rodrigocfd/winsafe/blob/71ed88c2a0d18b03ee452f6d4261f4c22f443483/src/macros/structs.rs#L5).

Keep GetSystemInfo, not GetNativeSystemInfo or an available-parallelism query.
Microsoft recommends GetNativeSystemInfo for native architecture information
under WOW64, but that would change the query selected by the current code.
The documented count is logical processors in the current processor group,
not a promise of all machine CPUs or available scheduling capacity. Keep the
current signed cast, including synthetic high-bit wrapping, and do not cache
the result. See [GetSystemInfo](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsysteminfo)
and [SYSTEM_INFO](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/ns-sysinfoapi-system_info).
The current source comment about 64-bit count equivalence is not evidence for
changing APIs on other architectures.

## JoinHandle identity is a different problem

`HTHREAD::GetThreadId` is safe once given an HTHREAD. It calls GetThreadId once,
maps zero to an error by reading LastError, and otherwise returns the u32 ID.
Mapping its error back to zero would preserve the original scalar result;
the additional error read is not a write. See the
[pinned method](https://github.com/rodrigocfd/winsafe/blob/71ed88c2a0d18b03ee452f6d4261f4c22f443483/src/kernel/handles/hthread.rs#L73).
Microsoft documents zero on failure and query-access requirements in
[GetThreadId](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadid).

The unresolved step is constructing that wrapper from the borrowed live
JoinHandle. Pinned winsafe's handle constructor is `unsafe fn from_ptr`; the
inspected source has no BorrowedHandle/AsHandle conversion implementing this
bridge. Using it would replace one first-party unsafe call with another, not
remove unsafe. Its safety documentation also warns about ownership, so do not
assume wrapping an externally owned pointer satisfies its contract. See the
[handle macro](https://github.com/rodrigocfd/winsafe/blob/71ed88c2a0d18b03ee452f6d4261f4c22f443483/src/macros/handles.rs#L59).
Opening or duplicating another handle adds calls, resource lifetime and failure
paths. Substituting the caller's current ID identifies the wrong thread.
Capturing the worker's ID through a rendezvous might avoid a handle query but
changes thread-start coordination and requires a separate design/behavior
review; it is not this direct replacement.

## Priority has no pinned safe wrapper

A source search of winsafe 0.0.29 found SetThreadPriorityBoost, but no
SetThreadPriority wrapper, declaration or THREAD_PRIORITY constant family.
Priority boost control is a different operation and cannot substitute for it.
See the complete [HTHREAD methods](https://github.com/rodrigocfd/winsafe/blob/71ed88c2a0d18b03ee452f6d4261f4c22f443483/src/kernel/handles/hthread.rs)
and [kernel declarations](https://github.com/rodrigocfd/winsafe/blob/71ed88c2a0d18b03ee452f6d4261f4c22f443483/src/kernel/ffi.rs).

The existing boundary passes arbitrary i32 unchanged and ignores BOOL while
retaining native side effects. A future typed-priority interface must not
silently clamp, reject, normalize or skip unknown values, nor translate a
refused value into a public error or panic. This requirement is hypothetical
for another candidate: there is no pinned winsafe typed SetThreadPriority
interface to convert to. The borrowed-handle problem would remain too.
Microsoft documents ordinary priorities, additional realtime values and
current-thread-only background mode values. It documents zero failure and
LastError diagnostics; a restricted known-value whitelist is not equivalent
to sending every i32 to the OS. See
[SetThreadPriority](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority).

## Baseline and replacement gates

1. Pin original current ID on the parent and a live child. Use channels or
   barriers: the child reports its current ID and waits for release, the
   parent compares it with the existing JoinHandle query, then releases and
   joins it before assertions that could panic. Assert same-thread repeat
   identity, nonzero ID, and distinct IDs while both threads remain live.
   Do not sleep to guess that a thread started or assume IDs never get reused.
   Microsoft limits ID uniqueness to the thread's lifetime in
   [GetCurrentThreadId](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadid).
2. Observe each original getter with several named LastError sentinels and
   compare the safe candidate on the same test thread. Capture immediately,
   restore the prior value before assertions, and do not assume success must
   preserve the sentinel. Do not add arbitrary raw-error construction requiring
   unsafe solely for this fixture. Source equivalence and native observations
   are separate evidence; report the tested Windows architecture.
3. Keep the existing positive-count smoke test. Pin zero initialization of
   publicly accessible record fields and pure count casts at 0, 1, i32::MAX,
   0x80000000 and u32::MAX. Do not read padding/reserved bytes unsafely to prove
   layout. Avoid asserting equality across separately queried counts as a
   universal topology-stability guarantee. No WOW64 or multi-group native
   validation is implied by a Windows x64 CI run.
4. If priority work proceeds later, use a dedicated blocked child and a named
   recording fake for exact arbitrary-i32 dispatch/ignored-status behavior.
   Limit native priority experiments to reviewed bounded values on that child;
   do not set realtime/time-critical priority or mutate the test runner's
   scheduling policy. Assert completion and recorded effects, not scheduling
   order, fairness or elapsed timing. Preserve cleanup on every failure path.
5. Land the two direct query replacements separately from unresolved handle
   and priority redesign. Obtain original and candidate Windows runs, unchanged
   fixtures, exact-head compiler inventories, `cargo xtask pr`, and Standards/
   Spec review. Retain Unix behavior and public signatures.

This bounded inspection identifies two removable first-party calls and two
unresolved boundaries. It neither exhausts established safe dependencies nor
proves system impossibility. No new dependency, public API migration or
permanent exception is approved by this note.
