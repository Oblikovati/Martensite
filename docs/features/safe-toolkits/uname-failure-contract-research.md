# System-name failure contracts

Research for Feature #80 / task #89 against starting main `beb3394`.
No production changes, failure reproductions, or exception approvals are part
of this note. The humanizer skill is unavailable; its prose pass awaits review.

## Decision supported by the evidence

Do not yet replace `system_names` with nix `uname().map_or(empty, ...)`.
That preserves ordinary successful queries and failures that write nothing,
but the Linux implementation has later failure exits after writing the record.
Apple independently has a partial-result contract. Neither observation proves
that safe implementation is impossible; the safe interface investigation is
still incomplete.

The existing function zeroes a stack `libc::utsname`, ignores `uname` status,
then collects `sysname` followed by `release` into byte vectors. Its output can
therefore contain bytes from a failed call. Preserve non-UTF-8 bytes, field
termination, errno side effects, and the order of the two output allocations.
This is the contract in `crates/martensite-tkernel/src/osd/process/host/native.rs`,
not an assertion that POSIX promises useful output after failure.

## GNU/Linux: separate failure classes

The pinned glibc revision
[`04e750e75b73957cf1c791535a3f4319534a52fc`](https://github.com/bminor/glibc/blob/04e750e75b73957cf1c791535a3f4319534a52fc/sysdeps/unix/syscalls.list)
generates the normal `uname` syscall wrapper. Its
[syscall template](https://github.com/bminor/glibc/blob/04e750e75b73957cf1c791535a3f4319534a52fc/sysdeps/unix/syscall-template.S)
returns minus one and sets errno on an error. It does not clear the destination.
Linux v6.8 maps both the
[x86_64 uname syscall](https://github.com/torvalds/linux/blob/v6.8/arch/x86/entry/syscalls/syscall_64.tbl)
and the [generic syscall used by arm64](https://github.com/torvalds/linux/blob/v6.8/include/uapi/asm-generic/unistd.h)
to `sys_newuname`.

In [Linux v6.8 `kernel/sys.c`](https://github.com/torvalds/linux/blob/v6.8/kernel/sys.c),
`newuname` snapshots the UTS record and copies it to the user buffer. It then
applies the optional `UNAME26` release override and the architecture/personality
override. Each copy can return `EFAULT`. A failed override follows a successful
whole-record copy, so failure does not inherently imply empty system names.
This is a source-level observation, not a reproduced failure on native CI.

The ordinary case has a fully writable, initialized stack destination. No
ordinary failure of that copy was demonstrated here. However, validity of the
destination alone is not sufficient proof for all supported kernel behavior:
[v6.8 usercopy helpers](https://github.com/torvalds/linux/blob/v6.8/include/linux/uaccess.h)
consult `should_fail_usercopy()` before copying. A later injected failure can
leave the earlier full record intact without invalidating the Rust stack object.
The [fault-injection implementation](https://github.com/torvalds/linux/blob/v6.8/lib/fault-inject-usercopy.c)
uses the kernel's configurable fault mechanism. This is distinct from corrupt
pointers, concurrent unmapping, or an application data race; none is required
for the source-level counterexample.

By contrast, a seccomp `SECCOMP_RET_ERRNO` decision skips syscall execution and
returns its configured error. With the original zeroed destination, such a
failure leaves both names empty. It does not exercise the later usercopy exits.
See the [v6.8 seccomp documentation](https://docs.kernel.org/6.8/userspace-api/seccomp_filter.html).
Tracing or user-notification supervisors that emulate calls are a separate
environmental contract, not equivalent to ordinary ERRNO filtering.

## Apple: independently partial results

Apple Libc
[`71bbe350ab79eef58113991d817ccc6165061a64`, `gen/uname.c`](https://github.com/apple-oss-distributions/Libc/blob/71bbe350ab79eef58113991d817ccc6165061a64/gen/uname.c)
queries `KERN_OSTYPE`, `KERN_HOSTNAME`, `KERN_OSRELEASE`, `KERN_VERSION`, and
`HW_MACHINE` in that order. It continues after a failed query and reports
failure if any query failed. A later failure can therefore coexist with earlier
system-name bytes. Replacing this with only two sysctls also omits intervening
and later queries and their errno effects. Actual partial failure on a macOS
runner remains untested; the inspected revision is not proof of every installed
libSystem build.

## Established safe interfaces inspected

- [nix 0.31.3 `uname`](https://docs.rs/nix/0.31.3/src/nix/sys/utsname.rs.html)
  zeroes a stack record but returns early on error, discarding its contents.
  Success accessors borrow raw `OsStr` bytes without allocating. It is a good
  successful-query interface but does not expose the failed record.
- [rustix 1.1.5 `uname`](https://docs.rs/rustix/1.1.5/rustix/system/fn.uname.html)
  is infallible at its public boundary. It does not provide a result containing
  both raw errno and a failed output record. The earlier host-query research
  records its libc backend's `ret_infallible` behavior; substituting this is not
  demonstrated equivalent failure handling.
- The [sysctl crate's safe trait](https://docs.rs/sysctl/0.7.1/sysctl/trait.Sysctl.html)
  provides owned value/raw-value queries, not a caller-supplied `utsname` record
  preserved after failure. It is a candidate for further Apple investigation,
  not evidence that the five-call sequence, fixed buffer lengths, partial bytes,
  and allocation/errno behavior have already been preserved.

An additional bounded source search inspected the published archives of two
dedicated wrappers (without adding either dependency):

- [uname 0.1.1, `src/lib.rs`](https://docs.rs/crate/uname/0.1.1/source/src/lib.rs)
  exposes `pub fn uname() -> io::Result<Info>` and
  `Info::new() -> io::Result<Self>`. It zeroes a local libc record, then returns
  `io::Error::last_os_error()` on failure without returning that record. On
  success it allocates five owned strings, including nodename before release,
  using lossy UTF-8 conversion. Its `From<utsname>` conversion accepts an already
  obtained record, not a buffer to receive the syscall result. Neither entry
  point solves partial output, raw bytes, or the original two-allocation order.
- [uname-rs 0.1.1, `src/lib.rs`](https://docs.rs/crate/uname-rs/0.1.1/source/src/lib.rs)
  exposes `Uname::new() -> std::io::Result<Self>`. Its raw record and extern call
  are private. Failure constructs an `ErrorKind::Other` message rather than
  exposing the record or a raw OS error. Success allocates vectors for six
  fields and validates UTF-8; conversion removes every NUL, rather than stopping
  at the first. The private ABI hardcodes six 65-byte fields without target
  selection; macOS compatibility is not established. This is not a suitable
  dependency substitution.

The source archives inspected were the immutable version downloads from
crates.io (`uname/0.1.1/download` and
`static.crates.io/crates/uname-rs/uname-rs-0.1.1.crate`); docs.rs source links
identify the corresponding versioned files. Neither wrapper accepts a mutable
caller-owned output buffer through a public safe query API. No additional
errno preservation guarantee was found; the extra allocation/conversion paths
would need their own evidence even apart from the decisive output mismatch.

No inspected established interface exposes the exact failed-record contract.
This bounded search does not establish a permanent system exception. Do not
copy a custom unsafe wrapper into another crate to claim removal. Before an
exception decision, investigate established interfaces with caller-owned safe
byte buffers and whether their failure outputs remain accessible. If changing
observable failure behavior is proposed instead, record the exact compatibility
decision under #88 for approval rather than silently changing it.

## Fixtures and exclusions

1. Keep the original native fixtures comparing `uname -s` and `uname -r` raw
   output with the returned bytes, and checking errno sentinels 0, EIO, and an
   unknown integer. Capture errno before assertions or command execution.
2. In a disposable child, use an established external syscall tracer's error
   injection or a safe seccomp interface to deny only `uname` with EPERM and
   ENOSYS. Run the original and candidate separately: expect empty names and
   the injected errno. This covers pre-execution failure only. Tool availability,
   filtering rules, target architecture, and actual injection must be recorded.
3. For the later-copy case, use a disposable VM with usercopy fault injection
   enabled. Set `UNAME26` on the child through an established safe personality
   interface, then target the override copy after the full-record copy. The
   [kernel fault-injection guide](https://docs.kernel.org/6.8/fault-injection/fault-injection.html)
   documents task filtering, call intervals, stack filters, and per-task
   `fail-nth`. Trace the actual failing copy; do not infer it from an EFAULT
   alone. Verify the original retains names and record errno. This experiment
   was not run; precise configuration depends on the VM kernel. Never enable
   global failure injection on the contributor machine or a shared runner.
4. A fake safe adapter returning partial bytes plus an error can test conversion
   logic, but cannot prove that a native wrapper exposes those bytes. Likewise,
   a libc interposer tests an interposed contract rather than native failure.
5. Do not make a stack pointer invalid, unmap live Rust storage, or introduce a
   race to manufacture EFAULT. No such reproduction is needed for this finding.
6. Record actual kernel/libc revisions on both Linux architectures and native
   macOS before claiming coverage. v6.8 is a pinned implementation reference,
   not a promise about all future kernels, musl, Android, or other Unix systems.

The next action is failure-contract validation and a broader safe-interface
search, not merging an empty-on-error replacement. Feature #80 remains open.
