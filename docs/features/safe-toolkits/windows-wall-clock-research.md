# Safe Windows wall-clock calls

Feature #80 / task #89 research, based on
`60c51f43a076bcd4e87eda6abac3ddd4489219f3` and
`crates/martensite-tkernel/src/osd/timing/wall_native.rs`.
No implementation, manifest, or dependency change is included. The humanizer
skill is unavailable; leave that prose pass to review.

## Candidate and trust scope

winsafe 0.0.29 exposes safe wrappers for all three existing Windows calls under
its `kernel` feature. The inspected registry package records upstream revision
`71ed88c2a0d18b03ee452f6d4261f4c22f443483` in `.cargo_vcs_info.json`.
Its [kernel functions](https://github.com/rodrigocfd/winsafe/blob/71ed88c2a0d18b03ee452f6d4261f4c22f443483/src/kernel/funcs.rs)
implement:

- `QueryPerformanceFrequency() -> SysResult<i64>`: zero-initialize an i64,
  call the same native function once, and return the output on nonzero status.
- `QueryPerformanceCounter() -> SysResult<i64>`: the same pattern for the counter.
- `GetTickCount64() -> u64`: return the native function's value directly.

Local source locations are `winsafe-0.0.29/src/kernel/funcs.rs:1499`, `:1485`
and `:969`, respectively. These wrappers neither cache nor convert units.
Using them would remove first-party call-site unsafe, while retaining trust in
the established dependency's FFI implementations. It would not make the whole
dependency graph unsafe-free. PR #159 separately proposes this Windows-only
dependency; verify its integration and exact pin before reusing it. Do not
duplicate the dependency or copy its unsafe implementation into this project.

## Preserve the existing state and arithmetic

The current `OnceLock<Option<i64>>` caches frequency success or failure once.
A failed frequency query therefore prevents counter queries on every later
call, rather than retrying initialization. When frequency is present, each
call queries the counter once; counter failure uses the tick-count fallback
without invalidating the cached frequency.

A candidate should keep that same static and initialize it with the safe
frequency wrapper's `.ok()`. Query the counter only in the `Some` branch,
and call the tick wrapper only when frequency is absent or counter fails.
Keep exactly `counter as f64 / frequency as f64` and
`0.001 * ticks as f64`; do not replace division with a cached reciprocal,
subtract a new epoch, change casts, retry, or guard against zero frequency.
Even synthetic zero/negative frequencies must preserve existing floating-point
results rather than silently changing the contract.

Microsoft documents frequency as fixed at boot and consistent across processors,
so caching is supported. Both performance APIs return nonzero on success and
are documented to succeed with valid parameters on Windows XP and newer.
Thus their failure branches are compatibility paths, not something ordinary
modern native runs are expected to exercise.
See [QueryPerformanceFrequency](https://learn.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancefrequency)
and [QueryPerformanceCounter](https://learn.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter).

The fallback returns milliseconds since startup. Microsoft documents
GetTickCount64 from Windows Vista onward; this was already required by the
existing import, not introduced by the wrapper. Keep its lower-resolution
fallback rather than substituting a different clock.
See [GetTickCount64](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-gettickcount64).

## LastError and call-order pitfalls

winsafe's [BoolRet conversion](https://github.com/rodrigocfd/winsafe/blob/71ed88c2a0d18b03ee452f6d4261f4c22f443483/src/kernel/privs.rs)
calls `GetLastError` only on zero status. Its safe `GetLastError` wrapper reads
the native value into `co::ERROR`; no `SetLastError`, formatting, allocation,
retry, or logging appears in this conversion. That adds a LastError read on
failure compared with the current code, but does not introduce a write.
Dropping the resulting error via `.ok()` requires no error-message rendering.

Microsoft defines LastError as per-thread and warns that APIs may modify it
even on success. Therefore do not add unconditional save/restore or clearing:
that would hide native side effects the current code leaves visible. A native
test should compare observed behavior, not assume every successful Windows
function preserves a sentinel. Read the value immediately after the tested call
before assertions, allocation, or test-reporting code.
See [GetLastError](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror).

The wrapper source supports equivalent ordinary native call ordering and output
handling. It does not prove equivalence under arbitrary function interposition,
which could observe the extra error read or supply unusual output. Do not
describe injected failures as naturally observed Windows failures.

## Required evidence before replacement

1. Add and run owned fixtures against the original implementation first. On
   Windows, bracket the returned wall time with independently obtained counter
   samples divided by the frequency, and verify repeated observations without
   requiring exact equality between separately sampled clocks. Record clock
   ordering assumptions and any exclusions.
2. In fresh child processes, test first-call initialization separately from
   warmed calls; the production static cannot be reset between tests. Capture
   LastError with zero and known nonzero errors, restoring the test thread's
   prior value before assertions. Application-defined values need a separately
   justified test route: winsafe's raw `ERROR` construction is unsafe, so do not
   add first-party unsafe just to manufacture a sentinel. Record that exclusion
   if no existing safe interface can supply one.
3. Use a named fake source and a local cache if a small private seam is needed
   to verify frequency-success caching, permanent frequency-failure caching,
   counter failure followed by recovery, no unnecessary counter/tick calls,
   exact arithmetic and call order. Keep the production static unchanged.
   These tests establish branch behavior, not native failure reachability.
4. Run unchanged baseline fixtures on the replacement, Windows native tests,
   all relevant toolkit tests and `cargo xtask pr`. Verify exact-commit unsafe
   inventories for every native target. Unix code should remain unchanged.
5. Check dependency policy if introducing or changing the dependency/features,
   and complete separate Standards/Spec review. Delete the replaced call sites
   after the gate passes; keep unresolved exclusions explicit.

This is a source-backed safe-wrapper candidate, not completed runtime proof or
a system-API impossibility claim. No native tests were executed for this note.
