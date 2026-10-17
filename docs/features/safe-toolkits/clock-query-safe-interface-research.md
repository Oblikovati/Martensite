# Safe wall and process clock queries

Research for Feature #80 / task #89 at
`b2bd8423dd1cd81cac001830349216bac100c525`. No production or dependency changes,
runtime failure experiments, or system-exception approvals accompany this note.
The humanizer skill is unavailable; its prose pass awaits review.

## Contracts in the current source

`osd/timing/wall_native.rs::time_of_day` initializes a timeval, calls
`gettimeofday` with null timezone, and returns signed seconds/microseconds only
when status is zero. Failure output is discarded. The caller samples twice on
first use: its OnceLock permanently stores the first call's seconds, or zero
if that call fails; the second query determines the returned value. Subsequent
query failures return 0.0. Preserve this sequence and subtraction before floating
conversion; replacing it with an epoch-sized floating timestamp loses precision.

`cpu_native.rs::tms_seconds` zeroes all four `tms` fields, ignores the return of
`times`, and divides self user/system ticks by its supplied `f64` divisor. The
process caller obtains that divisor from `_SC_CLK_TCK` first. Child ticks and
the elapsed-tick return are unused, but their underlying queries can still
affect failure and errno. No output allocation is needed in either function.

## Existing dependency interfaces

The current tkernel manifest pins nix 0.31.3 (`fs`, `feature`, `pthread`) and
rustix 1.1.5 (`std`, `fs`, `process`, `time`, `use-libc`). Inspection of those
versions' source found no public safe `gettimeofday` or `times` query. A private
rustix filesystem implementation uses gettimeofday; it is not a callable clock
interface. No new crate was added during this search.

rustix's safe
[`clock_gettime_dynamic(DynamicClockId) -> io::Result<Timespec>`](https://docs.rs/rustix/1.1.5/rustix/time/fn.clock_gettime_dynamic.html)
owns its stack result and can report errors. The selected
[libc backend](https://docs.rs/crate/rustix/1.1.5/source/src/backend/libc/time/syscalls.rs)
calls libc clock_gettime. This is a candidate for wall time with integer
`tv_nsec / 1000` truncation, not established syscall/failure equivalence.
Do not use its infallible clock interface for the original zero-on-error path.

nix provides
[`getrusage(UsageWho) -> Result<Usage>`](https://docs.rs/crate/nix/0.31.3/source/src/sys/resource.rs)
behind the currently absent `resource` feature. It returns an owned successful
record, discarding the local record on failure. Its self user/system timeval
accessors are useful for investigating CPU accounting, but are not already
the tick-quantized `times` contract. A feature addition needs the repository's
dependency-policy checks.

## GNU/Linux wall time: precision is only one requirement

Pinned glibc
[`04e750e75b73957cf1c791535a3f4319534a52fc`, gettimeofday.c](https://github.com/bminor/glibc/blob/04e750e75b73957cf1c791535a3f4319534a52fc/sysdeps/unix/sysv/linux/gettimeofday.c)
can resolve gettimeofday directly to its vDSO symbol, falling back to the
gettimeofday syscall when unavailable. Both
[x86_64](https://github.com/bminor/glibc/blob/04e750e75b73957cf1c791535a3f4319534a52fc/sysdeps/unix/sysv/linux/x86/gettimeofday.c)
and [aarch64](https://github.com/bminor/glibc/blob/04e750e75b73957cf1c791535a3f4319534a52fc/sysdeps/unix/sysv/linux/aarch64/gettimeofday.c)
select this IFUNC implementation. Other glibc paths can use clock_gettime
plus conversion; that does not prove these two targets do.

Consequently, clock_gettime with microsecond truncation plausibly matches the
normal timestamp domain but changes which native/vDSO entry point is queried.
Seccomp or tracing can distinguish fallback syscalls. A strace injection that
never intercepts a vDSO query proves nothing about error behavior. Compare raw
errno on successful calls and actually executed failure paths, including
unnamed errors; do not infer it merely from a Result type. The old code tests
status for equality to zero, not specifically minus one.

`SystemTime::now` is not a direct replacement: its
[public interface](https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.now)
does not expose a fallible clock-query result or raw errno. Deriving a duration
from the Unix epoch also requires correct signed/pre-epoch handling. A normal
current-time comparison alone cannot establish the original failure contract.

## GNU/Linux process time: unusual return treatment

glibc's pinned
[`times.c`](https://github.com/bminor/glibc/blob/04e750e75b73957cf1c791535a3f4319534a52fc/sysdeps/unix/sysv/linux/times.c)
uses an internal syscall, deliberately avoiding ordinary errno conversion.
It treats negative return values as potentially wrapped elapsed ticks and
changes minus one to zero. For an EFAULT-shaped return with a non-null pointer,
it touches the fields to distinguish invalid storage. Martensite ignores this
return entirely. Thus a generic fallible syscall wrapper may change errno even
when the returned pair looks identical. In particular, do not assume synthetic
EIO denial sets errno through this glibc wrapper; pin actual behavior first.

[Linux v6.8 `kernel/sys.c`](https://github.com/torvalds/linux/blob/v6.8/kernel/sys.c)
computes adjusted group CPU times, converts them to clock ticks, copies the tms
record, and separately returns elapsed ticks. Safe getrusage does not call this
entry point. Before substituting it, prove equivalent accounting and integer
quantization, not just approximately equal floating seconds. Microsecond
rounding followed by tick conversion can differ from a direct nanosecond-to-tick
conversion; inspect both conversion paths and platform constants.

An initialized destination alone is not a general proof that a failed native
copy left no fields. The existing function retains any writes even on failure;
a safe wrapper that discards failed output needs explicit compatibility analysis.
Do not manufacture invalid pointers or unmap live storage to test this.

## Apple differences

Apple Libc
[`71bbe350ab79eef58113991d817ccc6165061a64`, times.c](https://github.com/apple-oss-distributions/Libc/blob/71bbe350ab79eef58113991d817ccc6165061a64/gen/FreeBSD/times.c)
first queries `getrusage(SELF)` and writes self ticks. It then queries children
and finally gettimeofday to compute its return value. Failure in either later
query leaves the relevant self fields populated. Conversion uses integer
`sec * CLK_TCK + usec / (1000000 / CLK_TCK)` before the caller's division.
Skipping children or wall time drops errno side effects, despite unused outputs.
Replaying this sequence with safe interfaces is a plausible Apple-specific
direction, but gettimeofday and installed `CLK_TCK` semantics remain unresolved.

Apple's same-revision
[`gettimeofday.c`](https://github.com/apple-oss-distributions/Libc/blob/71bbe350ab79eef58113991d817ccc6165061a64/sys/gettimeofday.c)
tries commpage time before a syscall fallback. The null-timezone caller avoids
its timezone-cache path. A replacement clock API must not inadvertently add
timezone work or claim Linux's vDSO analysis covers macOS. These pinned sources
must be matched against actual native runner library versions before acceptance.

## Proposed evidence and next decision

1. Pin original behavior in isolated subprocesses so the wall clock OnceLock
   starts fresh. Test success and failure of first versus second query without
   changing the host clock. A fake query can pin arithmetic and initialization
   policy, but does not prove native failure equivalence.
2. Capture successful raw errno sentinels 0, EIO, and 12345 immediately after
   each original call. For CPU fixtures test supplied divisors separately,
   including zero/negative values, preserving existing floating behavior.
3. Trace original `times` calls with injected EIO, EPERM, and an unnamed error
   within Linux's syscall error range. Verify interception, field results, and
   incoming errno; do not assume conventional libc failure mapping. No unsafe
   test shim or global signal handler is needed for pre-execution injection.
4. For wall time, first establish whether a native call uses vDSO/commpage.
   Missing traced syscalls is an exclusion, not a passing injected-error test.
   Any fallback-only environment should be disposable and explicitly reported.
5. Investigate established safe gettimeofday/times interfaces further, including
   wrappers exposing initialized caller-owned output on failure. Compare a
   candidate's exact backend before considering another dependency. Do not move
   custom unsafe into a dependency or generated wrapper to satisfy the count.
6. Validate GNU/Linux x86_64 and arm64 separately, and macOS independently.
   Windows, other Unix libcs, and unsupported architectures are outside this
   research's proof scope. No full-migration or permanent-exception claim follows.

No inspected interface yet demonstrates the full observable contract. The
unresolved points are concrete query/error/quantization differences, not proof
that safe implementation is impossible. A behavioral migration would require
the exact compatibility decision under #88 rather than an implicit change.
