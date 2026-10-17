# Safe Windows sleep boundary

Feature #80 / task #89 research at
`5bdee0600a7b5f9460987d61b57c43255bf74208`, for
`crates/martensite-tkernel/src/osd/timing/sleep/native.rs`.
No production or dependency changes or native experiments accompany
this note. The unavailable humanizer prose pass remains for review.

## Exact candidate

The pinned winsafe 0.0.29 source has a safe `Sleep(milliseconds: u32)` wrapper
whose body calls the native function once with the unchanged argument. It
performs no duration conversion, error read, retry, allocation, or cancellation
check. See registry `winsafe-0.0.29/src/kernel/funcs.rs:1599`, and
[upstream revision 71ed88c2a0d18b03ee452f6d4261f4c22f443483](https://github.com/rodrigocfd/winsafe/blob/71ed88c2a0d18b03ee452f6d4261f4c22f443483/src/kernel/funcs.rs).
That revision is recorded in the package's `.cargo_vcs_info.json`.

Replacing only `sleep_windows`'s unsafe call with this wrapper is a
source-backed compatibility candidate. Keep its callers unchanged:
`seconds.wrapping_mul(1_000) as u32` for seconds, and `milliseconds as u32`
for milliseconds. In particular, negative inputs are not rejected, saturated,
or clamped. Milliseconds `-1` becomes `u32::MAX`, the native infinite-wait
sentinel. The seconds multiplication wraps before the cast; its results are
multiples of eight modulo 2^32, so it cannot produce that odd sentinel.

The wrapper is under winsafe's `kernel` feature. PR #159 separately proposes
that Windows-only dependency; check its merged state and exact version before
implementation. Reuse it if available, rather than introducing a second pin.
This removes first-party unsafe while trusting the established dependency's
FFI boundary; it does not remove unsafe from the dependency graph or authorize
moving custom unsafe into a dependency.

## Native semantics that must remain

Microsoft documents zero as relinquishing the current time slice to ready
threads, with immediate return possible when none are ready. `INFINITE` does
not time out. Sleep is not an alertable wait; it returns no status. Short waits
can be shorter than their nominal duration because of timer resolution, and
ready threads need not be scheduled immediately. Therefore neither a precise
elapsed-time assertion nor an assertion that another thread must run is a
valid deterministic baseline. See
[Sleep](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-sleep).

Do not substitute an alertable wait: ordinary queued user APCs and extended
I/O completion callbacks have different dispatch behavior in alertable waits.
Microsoft describes this distinction in
[SleepEx](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-sleepex).
Keeping the same Sleep call adds no cooperative cancellation or polling.
This is not a claim that a process or thread cannot be forcibly terminated,
nor an audit of special APC mechanisms.

Sleep's documentation supplies no LastError result contract. The safe wrapper
adds no LastError writes or reads, so retain the native side effect rather than
adding save/restore or clearing. Verify observed behavior on Windows before and
after the replacement. Capture LastError immediately after the call, before
assertion/reporting code. Microsoft explains why successful API calls cannot
generally be assumed to preserve LastError in
[GetLastError](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror).

## Why not standard-library sleep here

`std::thread::sleep` accepts a `Duration` and promises at least that duration;
its current documentation says Windows zero-duration calls invoke Sleep.
That does not establish an unchanged u32 sentinel contract or exact native
call sequence for all input durations. A finite `Duration` representing
`u32::MAX` milliseconds is not semantically an infinite wait. The documented
minimum-duration promise also differs from native Sleep's timer-resolution
description. Prefer the direct safe wrapper for this preservation task; no
claim that std sleep is unsafe is implied. See
[std::thread::sleep](https://doc.rust-lang.org/std/thread/fn.sleep.html).

## Baseline and replacement gates

1. Before replacement, add Windows-only fixtures for seconds zero and
   milliseconds zero/one. Record return completion and LastError for zero, a
   known error, and another nonzero named error. An application-defined sentinel
   needs a separate safe test route: winsafe's raw `ERROR` construction requires
   unsafe, which must not be added just for this fixture. Record that exclusion.
   Save and restore the test thread's original error around observation. Do not assert exact duration or
   guaranteed scheduler fairness.
2. Pin conversion arithmetic without sleeping: zero, one, minus one,
   `i32::MIN`, `i32::MAX`, and seconds around the multiplication overflow
   boundary. Use pure conversion fixtures or a named recording fake at a small
   private seam; assert the exact u32 argument and one dispatch. Verify the
   milliseconds infinite sentinel only as a value. Never actually sleep for
   huge or infinite inputs, including in a worker thread that cannot be joined.
3. Keep original fixtures unchanged when replacing only the boundary call.
   Obtain Windows native results on the exact baseline and replacement commits;
   compare full unsafe inventories. Run `cargo xtask pr` and relevant tests,
   with other native targets confirming unchanged Unix behavior.
4. Review the established dependency and run required dependency checks if its
   manifest/features change. Complete Standards/Spec review before integration
   and remove the replaced first-party call after validation passes.

Boundary-value fake tests establish mapping, not native infinite-wait behavior.
No APC, cancellation, scheduler-fairness or LastError runtime result is claimed
by this research. Those exclusions do not make the safe wrapper impossible;
the implementation uses the same native primitive and needs scoped validation.
