# Safe sleep interface investigation

Feature #80 / task #89, starting main `b2bd842`. Research only: no production
changes, dependency changes, or permanent system exceptions. The humanizer is
unavailable; its prose pass remains for review.

## Existing contract

`crates/martensite-tkernel/src/osd/timing/sleep/native.rs` performs wrapping
`i32` multiplication before conversion to the native unsigned duration:

| Entry | Unix microseconds | Windows milliseconds |
| --- | --- | --- |
| Seconds | `seconds.wrapping_mul(1000).wrapping_mul(1000) as useconds_t` | `seconds.wrapping_mul(1000) as u32` |
| Milliseconds | `milliseconds.wrapping_mul(1000) as useconds_t` | `milliseconds as u32` |

Do not clamp negative values, saturate, or multiply after widening. On the
supported GNU/Linux 64-bit targets the unsigned microsecond domain is `u32`;
its upper bound is 4,294,967,295 microseconds, or 4,294 seconds plus 967,295,000
nanoseconds. The actual entry-point products reach multiples of 64 or 8
microseconds, respectively, rather than every value in that domain.
The Unix function ignores the return status and does not retry. Error-side
errno remains observable even though the Rust return type is unit.

## GNU/Linux implementation

Pinned glibc revision
[`04e750e75b73957cf1c791535a3f4319534a52fc`, usleep.c](https://github.com/bminor/glibc/blob/04e750e75b73957cf1c791535a3f4319534a52fc/sysdeps/posix/usleep.c)
splits the unsigned microseconds into seconds and nanoseconds, then calls
`__nanosleep(&ts, NULL)` once. It accepts values exceeding one second and does
not bypass zero. Its source explicitly relies on nanosleep as a cancellation
point.

The same revision's
[Linux nanosleep.c](https://github.com/bminor/glibc/blob/04e750e75b73957cf1c791535a3f4319534a52fc/sysdeps/unix/sysv/linux/nanosleep.c)
calls `__clock_nanosleep_time64(CLOCK_REALTIME, 0, req, rem)`, assigning errno on
nonzero return. Its
[clock_nanosleep.c](https://github.com/bminor/glibc/blob/04e750e75b73957cf1c791535a3f4319534a52fc/sysdeps/unix/sysv/linux/clock_nanosleep.c)
uses the cancellation-aware syscall machinery. On native x86_64 and arm64
this is the relative `clock_nanosleep` syscall, not a direct `nanosleep`
syscall. Confirm actual installed libc versions and traces before acceptance.

Consequences: successful calls should retain incoming errno; EINTR returns
early without retry and sets errno; zero must retain the native call rather
than becoming an early return. Source does not establish a scheduler fairness
guarantee for a zero Unix sleep. Replacing it with `yield_now` is not proven
equivalent. Seccomp can distinguish syscall numbers, clock IDs, flags, and
pointer arguments, so matching a typical elapsed duration is insufficient.

## Safe candidates and constraints

The inspected installed rustix 1.1.5 source offers safe
`nanosleep(&Timespec) -> NanosleepRelativeResult` and
`clock_nanosleep_relative(ClockId, &Timespec) -> NanosleepRelativeResult`.
They return once with success, interruption plus remaining time, or error.
See the [versioned thread source](https://docs.rs/crate/rustix/1.1.5/source/src/thread/)
and its [raw Linux backend](https://docs.rs/crate/rustix/1.1.5/source/src/backend/linux_raw/thread/syscalls.rs).

- Raw-backend nanosleep on 64-bit Linux invokes `nanosleep`, changing the syscall
  from glibc's path. Raw `clock_nanosleep_relative(Realtime, ...)` matches the
  ordinary syscall and clock/flags, but bypasses libc pthread cancellation and
  returns errors without assigning libc errno. Restoring errno alone cannot
  repair missing cancellation behavior.
- The [libc backend](https://docs.rs/crate/rustix/1.1.5/source/src/backend/libc/thread/syscalls.rs)
  calls libc nanosleep and therefore retains that libc's cancellation and errno
  path. This is the strongest candidate for GNU/Linux, not yet a validated
  replacement. Unlike the original, it passes a non-null remainder pointer.
  Argument-sensitive filters can distinguish this; no exact equivalence is
  claimed for those environments.
- rustix's `use-libc` feature selects that backend; see
  [build.rs](https://docs.rs/crate/rustix/1.1.5/source/build.rs).
  The actual tkernel manifest already enables `std`, `fs`, `process`, `time`,
  and `use-libc`; there is no proposed new backend switch here. The candidate
  would need the missing `thread` feature. Audit and run dependency checks for
  that feature change. Raw-backend comparisons above explain alternatives, not
  the repository's currently selected backend.
- Both inspected backends initialize remainder storage only through the native
  call and assume it is initialized when EINTR occurs. Do not inject EINTR via
  strace/seccomp without also satisfying that output contract: skipping the
  syscall can leave the remainder uninitialized. Reading it in the wrapper
  would not be a safe regression-test technique, even if the caller ignores it.

Existing nix's seconds-only sleep does not express the fractional durations,
but its separate `time` feature provides another candidate:
[nix 0.31.3 `clock_nanosleep`](https://docs.rs/crate/nix/0.31.3/source/src/time.rs)
accepts `(ClockId, ClockNanosleepFlags, &TimeSpec)` and returns `Result<TimeSpec>`.
It initializes remainder to zero before calling libc `clock_nanosleep`, so
synthetic EINTR does not cause an uninitialized remainder read in this wrapper.
It retains libc cancellation, but supplies non-null remainder and converts
the positive return code through `Errno::from_raw`.

That error conversion is lossy: nix's
[errno mapping](https://docs.rs/crate/nix/0.31.3/source/src/errno.rs) maps unmatched
integers to `UnknownErrno` (zero). The glibc clock call returns the raw positive
error without setting errno; the original nanosleep layer sets that exact
integer. Reading thread errno after nix cannot recover the discarded return
value. For example, Linux seccomp can synthesize an unnamed error such as 4094:
[v6.8 seccomp.c](https://github.com/torvalds/linux/blob/v6.8/kernel/seccomp.c)
caps ERRNO action data at `MAX_ERRNO`, rather than limiting it to named errors.
Mapping the nix enum back into errno therefore cannot preserve every original
failure. Test an unknown injected error as well as EIO; incoming sentinel
12345 is not a suitable syscall-error injection because it exceeds that range.
The nix `time` feature is also absent from the current tkernel manifest.

[std::thread::sleep](https://doc.rust-lang.org/std/thread/fn.sleep.html) promises
at least the requested duration, may repeat interrupted calls, and currently
omits the syscall for zero on Unix. It is not a direct replacement.

## Apple and Windows are separate

Apple Libc
[`71bbe350ab79eef58113991d817ccc6165061a64`, usleep.c](https://github.com/apple-oss-distributions/Libc/blob/71bbe350ab79eef58113991d817ccc6165061a64/gen/FreeBSD/usleep.c)
uses the same microsecond division/remainder and calls `_nanosleep` with a null
remaining-time argument. In its
[nanosleep.c UNIX03 branch](https://github.com/apple-oss-distributions/Libc/blob/71bbe350ab79eef58113991d817ccc6165061a64/gen/nanosleep.c),
a non-null remainder causes a preliminary `clock_get_time` call with a separate
failure path. Cancellation variants also differ. Thus libc-backed rustix
nanosleep introduces work before the wait on that source path; Linux reasoning
cannot be transferred to macOS arm64. Installed-build applicability and native
failure behavior still need investigation.

Windows [Sleep documentation](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-sleep)
specifies zero as relinquishing the current time slice and `INFINITE` as no
timeout. In particular, milliseconds `-1` converts to `0xffffffff`, not a
finite 49-day duration. The seconds multiplier cannot reach this odd sentinel.
Preserve the unsigned call argument and these semantics. The std sleep
documentation's Windows zero behavior is encouraging but does not prove its
large-duration/sentinel behavior. No Windows replacement is established here.

## Native baseline fixtures before implementation

1. Record ordinary zero and short positive calls with incoming errno 0, EIO,
   and 12345; capture errno immediately after the call. Do not demand a tight
   wall-clock upper bound or infer scheduler fairness from a single run.
2. In an isolated original test executable, trace `clock_nanosleep` and
   `nanosleep`. Verify one call, clock/flags, null remainder, and exact requested
   timespec for zero, one, minus one, i32 extrema, and multiplication-boundary
   inputs. Compute expected wrapping values independently.
3. For potentially long cases, inject EIO/EPERM/ENOSYS into both possible sleep
   syscall names so no actual long wait occurs. Use a parent timeout as a
   backstop, verify the explicit injection marker and call count, and fail if
   interception is absent. Record target, tool version, and native syscall.
4. An EINTR injection is safe for the original null-remainder path and can pin
   its no-retry behavior. Do not reuse that fixture blindly against wrappers
   that read remainder storage. A real interruption fixture requires a safe
   established signal mechanism confined to a disposable child, without global
   test-process signal handlers. It is not implemented by this research.
5. Cancellation is a separate contract. Do not manufacture foreign unwinding
   through Rust frames or claim ordinary EINTR coverage proves cancellation.
   Prefer a matching libc call path and independent documented cancellation
   analysis; any native experiment must respect its language/runtime limits.
6. Run original and candidate native x86_64/arm64 fixtures unchanged where their
   assumptions apply. Synthetic injected errors prove only the tested class,
   not every native interruption or pointer-sensitive filter behavior. Apple,
   Windows, other libcs, and other Unix targets remain explicit exclusions.

Next: pin original traces, audit the required feature addition, and investigate a
safe established null-remainder sleep interface. The bounded search above
does not demonstrate impossibility. Do not move custom unsafe into a helper
crate or silently change cancellation/error behavior to claim removal.
