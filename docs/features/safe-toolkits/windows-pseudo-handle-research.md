# Windows current-object pseudo handles

Research for Feature #80 / task #89 against
`0e5f37b96932b1885834d9e05f6560a887ed77c4`. The subsequent baseline at starting
main `c58eff4` extracts the original production getters without replacing their
calls and adds the Windows-only dependency for differential fixtures. Current
validation is recorded in [the baseline evidence](windows-pseudo-handle-wrapper.json).
No Windows runtime result is established yet. The humanizer is unavailable;
its prose pass remains for review.

## Recommendation

Do not replace `GetCurrentProcess()` and `GetCurrentThread()` with hardcoded
negative pointers solely to remove their two unsafe call sites. Rust can
construct such pointers safely, but Microsoft does not give the inspected
numeric values the required future-compatible Win32 contract. This is not
proof that an established safe wrapper cannot remove the calls.

## Current use and pinned bindings

`crates/martensite-tkernel/src/osd/timing/cpu_native.rs` passes the returned
pseudo handle immediately to `GetProcessTimes` or `GetThreadTimes` through
`windows_times`. It neither stores nor closes it. The output FILETIMEs start
at zero and query status is intentionally ignored. This investigation concerns
only obtaining the handle, not changing that output/failure contract.

Installed windows-sys 0.61.2 declares both functions as kernel32.dll system
imports; see its [Threading source](https://docs.rs/crate/windows-sys/0.61.2/source/src/Windows/Win32/System/Threading/mod.rs).
Its [Foundation source](https://docs.rs/crate/windows-sys/0.61.2/source/src/Windows/Win32/Foundation/mod.rs)
defines `HANDLE = *mut core::ffi::c_void`. These are foreign function bindings,
not safe Rust current-object constructors. Reusing `INVALID_HANDLE_VALUE` as a
process handle would conflate two semantically different uses of a bit pattern.

## What Microsoft promises

[GetCurrentProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess)
documents a current-process pseudo handle and describes minus one as the
current value. Crucially, it recommends calling the function instead of
hardcoding the value for future operating-system compatibility. A successful
test on today's Windows x64 therefore cannot establish equivalent future
behavior for a constant implementation.

[GetCurrentThread](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthread)
describes a special constant interpreted as the calling thread, but the
inspected page does not promise a numeric minus-two value. It explicitly says
another thread using the pseudo handle refers to itself, not the original
thread. A real duplicated handle has different ownership and identity rules.

Neither pseudo handle requires closing; the documented CloseHandle operation
has no effect for these pseudo handles. Duplicates must be closed. Preserve the
existing immediate-use semantics; do not manufacture an `OwnedHandle` whose
destructor implies ownership, or substitute an opened real handle with added
allocation, access checks, failure modes, and lifetime obligations.

The two getter pages define no failure sentinel or last-error reporting path.
[GetLastError guidance](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
explains that last error is per-thread and some successful functions can change
it. The inspected getter documentation does not explicitly specify a sentinel
preservation test. Establish actual original behavior on supported Windows
builds before claiming identical side effects. A constant performs no foreign
call and necessarily leaves the error slot alone, which is not by itself proof
about the removed API call.

The published interface is a DLL function, not a promised kernel-transition
sequence. No inspected source establishes that it must execute a syscall, or
that its present machine-code implementation is its permanent specification.
It is not documented here as a wait or cancellation point. Removing its import
would also change observability to DLL instrumentation; any such environment
exclusion must be explicit, not silently equated with ordinary API semantics.

## Safe Rust and existing alternatives

[`std::ptr::without_provenance_mut`](https://doc.rust-lang.org/std/ptr/fn.without_provenance_mut.html)
can safely construct an opaque raw pointer with a chosen address. Such a value
must never be dereferenced as allocated memory. This solves pointer construction
only: it cannot establish which sentinel an external platform interprets as a
current object. On x64, sign extension must also be correct; a zero-extended
32-bit minus-one bit pattern is not the same raw address as pointer-width minus
one. No constant implementation is proposed by this note.

The standard library's
[`AsHandle`](https://doc.rust-lang.org/std/os/windows/io/trait.AsHandle.html)
borrows existing owned objects; its inspected implementors do not offer a
current-process/current-calling-thread pseudo-handle constructor. Handles from
spawned children, files, or join handles are not replacements for an arbitrary
calling thread's pseudo handle. `std::process::id()` returns an identifier, not
a handle; opening that identifier introduces the real-handle differences above.

The currently pinned windows-sys binding itself does not provide a demonstrated
safe wrapper for these imports. Existing Unix nix/rustix interfaces do not
supply a Windows current-object handle. A broader established Windows wrapper
search remains warranted; it must inspect implementation, lifetime behavior,
and dependency cost rather than merely choosing a similarly named function.
Do not relocate these custom unsafe calls into a local helper dependency to
claim removal.

## Concrete established candidate: winsafe 0.0.29

The published winsafe 0.0.29 source provides a credible removal path without
hardcoding values. Its
[`HPROCESS::GetCurrentProcess`](https://docs.rs/crate/winsafe/0.0.29/source/src/kernel/handles/hprocess.rs)
returns `HPROCESS(unsafe { ffi::GetCurrentProcess() })`, and
[`HTHREAD::GetCurrentThread`](https://docs.rs/crate/winsafe/0.0.29/source/src/kernel/handles/hthread.rs)
does the corresponding thread call. Both methods are public safe functions.
The [FFI declarations](https://docs.rs/crate/winsafe/0.0.29/source/src/kernel/ffi.rs)
import the real kernel32 functions; these getters do not synthesize constants.

The [handle macro](https://docs.rs/crate/winsafe/0.0.29/source/src/macros/handles.rs)
defines transparent pointer wrappers with a safe inherent
`pub const fn ptr(&self) -> *mut std::ffi::c_void`. It defines no destructor for
these bare handle types. Consequently the candidate expressions
`winsafe::HPROCESS::GetCurrentProcess().ptr()` and
`winsafe::HTHREAD::GetCurrentThread().ptr()` need no unsafe code in Martensite,
no heap allocation, and no closing/duplication operation. The temporary wrapper
does not own a closable resource; consuming the raw pseudo handle immediately
on the calling thread preserves the present lifetime/use pattern.

The inspected getter-plus-accessor path adds only pointer wrapping/unwrapping
around the same DLL call. It does not query or set last error and adds no
cancellation operation. This is stronger source evidence than observing a
constant on one Windows build: the platform remains responsible for selecting
the pseudo value. Native linking and sentinel tests remain required; do not
infer all of winsafe is audited from this small path. Its safe GetProcessTimes
and GetThreadTimes methods are outside this proposal because changing failed
output handling would require separate analysis.

The versioned [manifest](https://docs.rs/crate/winsafe/0.0.29/source/Cargo.toml)
and [README](https://docs.rs/crate/winsafe/0.0.29/source/README.md) identify an
MIT-licensed, handwritten project maintained since 2019, with edition 2024 and
Rust 1.87 minimum. The inspected 0.0.29 package declares no Rust dependencies;
its `kernel` feature has no prerequisite features and is sufficient for these
types. It is still a pre-1.0 library, not a compatibility guarantee or safety
certification. Its broad kernel module increases compiled/trusted source beyond
these two methods; compile-time, binary, and audit costs have not been measured.

The baseline adds a Windows-only dependency pinned to `=0.0.29`, disables
defaults, and enables only `kernel`, leaving the existing windows-sys production
calls in place. The lockfile gained winsafe; `cargo deny --all-features check`
and `cargo audit` passed. These checks do not establish native compatibility or
audit the whole dependency for soundness. The new trust is an independently
maintained published wrapper, not copied Martensite unsafe. Replacing the two
production getters remains a separate step after baseline validation.

## Evidence required before any replacement

1. Keep an original native Windows x64 fixture that calls each existing getter,
   captures its pointer-width value, and immediately records last error after
   sentinels 0, a known error, and an application-defined value. Record OS build,
   architecture, windows-sys version, and toolchain. Do not infer native results
   from Linux cross-compilation.
2. Exercise the existing process/thread time queries on the main thread and
   spawned threads. Confirm that a thread pseudo handle is consumed by the
   calling thread and is never repurposed as another thread's identity.
3. Test the chosen safe wrapper against those fixtures without closing pseudo
   handles or retaining them in ownership wrappers. Preserve FILETIME conversion
   and failure output; this small change must not redesign CPU accounting.
4. Inspect compiler-emitted unsafe inventory and final Windows checks at the
   exact candidate revision. Pointer equality on one OS build is regression
   evidence, not a guarantee that hardcoded constants remain future-compatible.
5. If no exact safe wrapper is found, record the bounded search and unresolved
   system boundary for review. If narrowing future compatibility is proposed,
   request an explicit decision rather than asserting it is behavior-preserving.

The winsafe getter/accessor path is a concrete source-supported replacement
candidate for these two calls, pending native fixtures and dependency acceptance.
No production migration or permanent system exception is established yet.
