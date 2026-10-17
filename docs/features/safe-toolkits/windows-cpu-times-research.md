# Windows CPU-time query compatibility

Feature #80 / task #89 research at
`258bba200295a4bb29e8f1956490b2e2d5bbd50a`, covering
`crates/martensite-tkernel/src/osd/timing/cpu_native.rs`.
This note changes no production code or dependencies and reports no native
experiments. The unavailable humanizer prose pass remains for review.

## Finding

The existing winsafe 0.0.29 dependency provides safe `GetProcessTimes` and
`GetThreadTimes` methods. Their successful output path is a close match, but
their failure path hides information consumed by the current implementation.
Replacing failure with a default tuple is therefore not yet justified.
This is an unresolved compatibility question, not proof that safe removal is
impossible or an approved permanent system exception.

## Original behavior and wrapper difference

At the recorded revision, `windows_times` initializes four distinct FILETIMEs
to zero, calls the selected native query once, ignores its BOOL result, then
converts user and kernel fields in that order. It does not restore or inspect
LastError. Its comment describes zero on failure, but the code actually reads
whatever values remain, including any partial writes. See the
[pinned implementation](https://github.com/Oblikovati/Martensite/blob/258bba200295a4bb29e8f1956490b2e2d5bbd50a/crates/martensite-tkernel/src/osd/timing/cpu_native.rs).

Both winsafe methods also initialize all four output structs to zero and call
the corresponding native query once. They return `(creation, exit, kernel,
user)` only when BOOL is nonzero. On zero, `BoolRet::to_sysresult` reads
GetLastError and returns only that error; the output structs are discarded.
Consequently an `Err` cannot distinguish untouched zeros from partially
written CPU times. The inspected package's `.cargo_vcs_info.json` identifies
revision `71ed88c2a0d18b03ee452f6d4261f4c22f443483`. Sources:
[process method, lines 115–129](https://github.com/rodrigocfd/winsafe/blob/71ed88c2a0d18b03ee452f6d4261f4c22f443483/src/kernel/handles/hprocess.rs#L115),
[thread method, lines 131–145](https://github.com/rodrigocfd/winsafe/blob/71ed88c2a0d18b03ee452f6d4261f4c22f443483/src/kernel/handles/hthread.rs#L131),
[BoolRet conversion](https://github.com/rodrigocfd/winsafe/blob/71ed88c2a0d18b03ee452f6d4261f4c22f443483/src/kernel/privs.rs#L37), and
[FILETIME's derived zero default](https://github.com/rodrigocfd/winsafe/blob/71ed88c2a0d18b03ee452f6d4261f4c22f443483/src/kernel/structs.rs#L255).

Microsoft documents nonzero success, zero failure and GetLastError diagnostics
for both calls, but does not promise untouched output buffers on failure or
unconditional success for current-object handles. The live object's exit time
is undefined and must not become an assertion or observable result here.
User/kernel fields are 64-bit counts of 100-nanosecond units; process values
sum thread execution and can exceed elapsed wall time. See
[GetProcessTimes](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesstimes) and
[GetThreadTimes](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadtimes).

The wrapper adds no LastError write: its failure conversion performs only the
read in [winsafe GetLastError](https://github.com/rodrigocfd/winsafe/blob/71ed88c2a0d18b03ee452f6d4261f4c22f443483/src/kernel/funcs.rs#L509).
Still, do not assume native success preserves an incoming sentinel; compare
observations immediately after each call. LastError is per-thread, and success
side effects depend on the called API. See
[Microsoft GetLastError](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror).

## Preserve identity and conversion

Keep the platform getter for each pseudo handle, consumed on the calling
thread without opening, duplicating, caching across threads or closing it.
The pinned winsafe getters call the same native getters. Microsoft explicitly
recommends the process getter over hard-coded constants; a thread pseudo handle
refers to whichever thread uses it, not necessarily the thread that obtained
it. These facts support the existing getter migration, not an inference that
every subsequent query must succeed. See
[GetCurrentProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess) and
[GetCurrentThread](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthread).

Retain the original arithmetic exactly: combine high/low words into `u64`,
cast to `i64`, cast to `f64`, then multiply by `0.000_000_1`. Do not replace it
with unsigned conversion, integer duration conversion, division or separately
scaled words. The original signed cast makes bit-63 inputs negative even
though ordinary CPU observations do not reach that range. This is a
code-derived compatibility requirement, not a Windows guarantee about those
artificial values.

## Smallest next step and fixture plan

1. Pin the original successful process/thread observations on native Windows.
   Use before/after queries to bound the original CPU values rather than
   require equality between samples that advance. Assert user/kernel ordering,
   finite nonnegative ordinary samples, and nondecreasing values. Compare
   LastError with several named sentinels, capturing immediately and restoring
   the test thread's previous value before assertions. Do not compare exit
   times or require a minimum increment from a timed workload.
2. Pin pure conversion with literal words: zero, one tick, ten million ticks,
   low-word carry, counts around 2^53, `i64::MAX`, bit 63 and `u64::MAX`.
   Use independently derived expected floating-point bits where rounding is
   involved. These fixtures cover synthetic arithmetic, not enormous native
   CPU durations.
3. Before changing the query contract, identify a safe way to retain mutable
   outputs on failure, or obtain evidence sufficient to exclude changed
   failure outputs for every supported production case. A named fake can pin
   zero/partial/full writes and ignored status at a private safe seam, but it
   cannot establish actual Windows failure behavior. Invalid-handle tests alone
   likewise cannot prove current-pseudo-handle behavior. Do not add unsafe
   callbacks or interposition solely to manufacture a green migration test.
4. If an established safe interface preserving all outputs is found, keep
   one native dispatch and the conversion unchanged, then rerun the original
   fixtures, native inventories, PR gate and Standards/Spec review. If the
   only practical route changes failure behavior, request an explicit decision
   with the exact observable difference; do not silently call it a refactor.

The inspected alternatives are bounded. The already-used windows-sys 0.61.2
bindings retain pointer outputs but require unsafe calls. Microsoft's higher
level [windows 0.62.2 GetProcessTimes](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.GetProcessTimes.html)
also exposes an unsafe pointer-output function, so switching to it does not
remove first-party unsafe. This note does not exhaust every established
dependency or prove that none can preserve failure outputs. Reusing winsafe
would keep existing dependency trust; it would not make its internal FFI safe
Rust or authorize relocating custom unsafe into another crate.

## Focused alternative search

The recorded Cargo.lock and tkernel manifest were checked before looking for
another dependency. The relevant Windows interfaces already available are
winsafe and windows-sys. The lockfile also contains `filetime`, but that crate
exposes filesystem timestamps, not process/thread CPU queries; its name does
not make it an alternative FILETIME query interface. See the
[filetime public API](https://docs.rs/filetime/0.2.26/filetime/).
No manifest or lockfile changes were made for this search.

Two published CPU-clock libraries were then checked at exact upstream
revisions, rather than assuming that a safe public clock preserves native
output behavior:

- [cpu-time at d36927e](https://github.com/tailhook/cpu-time/blob/d36927e013565bd3cf1a06813e45e764558cf30a/src/windows.rs):
  `ProcessTime::try_now` and `ThreadTime::try_now` internally zero the CPU
  outputs, use current-object getters, and return an I/O error before exposing
  any outputs when BOOL is zero. On success they combine kernel and user into
  one unsigned Duration. The non-fallible `now` methods panic on failure.
  Neither interface accepts caller-owned FILETIMEs or returns partial outputs;
  the combined result also loses Martensite's separate user/system values.
- [howlong at 72a9fbe](https://github.com/xu-cheng/howlong/blob/72a9fbe91baaa0be99d1b387b08f85aa585c3c6f/src/clock/win.rs):
  its private process helper zeroes outputs and returns an error before
  returning the user/system pair on failure. Public clocks convert those
  outputs to unsigned nanosecond Durations; the combined process clock also
  queries wall time. ThreadClock combines user and system after checking
  status. These methods do not expose caller-owned initialized outputs, and
  the helper is not a public escape hatch.

Neither evaluated library supplies the required output-preserving safe call.
These are existing published candidates, not approved dependency additions:
maintenance, dependency and security review would still be required even if
their contracts matched. Other libraries may exist; this is not an exhaustive
ecosystem audit. A custom wrapper exported through a new crate would still
violate the removal policy, regardless of whether its caller is safe Rust.

## Decision needed for undocumented failure outputs

Distinguish three claims: the API documents failure; the current code consumes
outputs regardless of status; this investigation has not observed a partial
write on failure with a valid current-object pseudo handle. Missing output
guarantees do not prove that such a partial write is reachable. Conversely,
normal successful samples cannot prove it unreachable. The question should
not be silently settled either by inventing a failure scenario or by treating
zero fallback as already equivalent.

A maintainer may choose to define failed CPU queries as `(0.0, 0.0)` and exclude
undocumented failed-call output contents from the compatibility promise. That
would make the pinned winsafe wrapper a small implementation candidate, with
the exact conversion retained on success. Record this as an explicit failure
policy decision and assess its observable-behavior/migration requirements
before implementation; this note does not grant that approval. Alternatively,
retain the narrow boundary while seeking stronger platform evidence or an
established output-preserving safe interface. Neither choice makes a bounded
search proof of system impossibility, and no public API conflict has yet been
demonstrated here: the unresolved issue is the failure behavior of a private
system boundary.
