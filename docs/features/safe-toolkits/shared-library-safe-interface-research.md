# Shared-library safe-interface research

Feature #80 / task #89 at main `59a6228`, covering
`osd/process/shared_library.rs` and `shared_library/windows_backend.rs`.
No production changes, dependencies or native unload/race experiments were
performed. The unavailable humanizer prose pass remains for review.

## Confirmed ownership defect: Bug #170

On Windows, `set_name_bytes` caches a GetModuleHandleW result and resets
`closed`. `open_with` returns that non-null cached handle without loading or
acquiring a reference. `dl_close` then unconditionally forwards it to
FreeLibrary. Indeed, naming an already loaded module and closing it is enough;
dl_open is not required. See
[set_name_bytes and dl_close](https://github.com/Oblikovati/Martensite/blob/59a6228/crates/martensite-tkernel/src/osd/process/shared_library.rs)
and [Windows dispatch](https://github.com/Oblikovati/Martensite/blob/59a6228/crates/martensite-tkernel/src/osd/process/shared_library/windows_backend.rs).

Microsoft explicitly says GetModuleHandle does not increment the reference
count and its result must not be passed to FreeLibrary. It also warns that
another thread can unload the module before the borrowed handle is used.
This directly contradicts the existing close safety argument; it is not merely
a hypothetical wrapper mismatch. See
[GetModuleHandleW](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew).
The coordinator filed [Bug #170](https://github.com/Oblikovati/Martensite/issues/170).
No crash is needed to establish the invalid ownership operation, and none was
attempted. Actual premature unmapping depends on references held elsewhere.

Fixing only the unwanted FreeLibrary is insufficient: cached lookup and later
symbol use still require a module lifetime guarantee. A fix needs explicit
owned-versus-borrowed state and a race-safe reference acquisition strategy, or
removal of the borrowing shortcut. Review filename/basename selection as well:
the current cache strips directories, while uncached loading uses the full
converted path. Acquiring a reference must not silently select a different
same-basename DLL without documenting that bug fix.

## Loading safety is an API question

Public dl_open accepts arbitrary library names in safe Rust. Its SAFETY
comments establish pointer/string validity, not the preconditions of foreign
initialization routines. Likewise dl_close can execute foreign termination
code. libloading **0.8.9** explicitly requires its unsafe open caller to ensure
both initialization and termination routines are safe to execute. This is the
same hazard as invoking unknown foreign code, independent of whether symbols
are subsequently called. See the raw-source-verified
[libloading Unix open safety contract, lines 170–180](https://github.com/nagisa/rust_libloading/blob/0.8.9/src/os/unix/mod.rs#L170-L180).

The current generic safe API supplies no equivalent restriction or validation;
[Bug #171](https://github.com/Oblikovati/Martensite/issues/171) tracks this contract gap.
This is a safety-contract gap requiring maintainer review under #88; changing
dl_open to unsafe is a breaking API migration, not authorized by this note.
Returning a raw symbol address does not call the returned function, but symbol
lookup itself can execute foreign code. In glibc 2.39, do_sym calls _dl_sym_post;
that helper invokes an STT_GNU_IFUNC resolver and may invoke audit callbacks
before returning the address. See raw-source-verified
[dl-sym.c lines 157–170](https://github.com/bminor/glibc/blob/glibc-2.39/elf/dl-sym.c#L157-L170)
and [dl-sym-post.h lines 33–58](https://github.com/bminor/glibc/blob/glibc-2.39/elf/dl-sym-post.h#L33-L58).
Consequently safe dl_symb/dl_symb_bytes need the same explicit safety-policy
review, including global lookup before opening a library. Returning NonNull
does not establish resolver safety. Calling the returned function additionally
requires signature and lifetime correctness; that separate obligation does
not cover code already executed by lookup. This is source evidence of the
execution mechanism, not a runtime exploit or proof that ordinary platform
resolvers are unsound. A library-specific trusted loader may
be safe if its initialization/termination preconditions are established, but
that does not justify a generic safe arbitrary-name loader.

## Wrapper comparison

`libloading` is not currently a dependency. Its Unix platform open accepts
explicit flags and optional filename, so RTLD_LAZY/RTLD_NOW and dlopen(NULL)
can be represented, but opening remains unsafe. Its failure path consumes
dlerror into a returned error; symbol lookup also has its own dlerror protocol.
Automatic Drop closes the library, unlike this repository's non-unloading
drop/destroy behavior. Raw-handle conversion is unsafe. Therefore a mechanical
substitution would change safety, ownership and diagnostic consumption. See
[libloading 0.8.9 Unix implementation](https://github.com/nagisa/rust_libloading/blob/0.8.9/src/os/unix/mod.rs).

`dlopen2` was inspected at upstream revision
`c1ca06007390d89e9adf2a4400176eba51ba9c28`. Its raw Library exposes safe open and
open_with_flags, but that signature alone does not resolve arbitrary foreign
initializer safety. Its Unix get_sym clears dlerror before lookup and consumes
errors, converting messages lossily to strings. Its default and open_self
flags are RTLD_LOCAL | RTLD_LAZY; explicit flags apply to named open. These
do not preserve the repository's independent consumptive raw-byte dl_error
API or all null/global lookup cases. See
[raw public API](https://github.com/OpenByteDev/dlopen2/blob/c1ca06007390d89e9adf2a4400176eba51ba9c28/dlopen2/src/raw/common.rs)
and [Unix implementation](https://github.com/OpenByteDev/dlopen2/blob/c1ca06007390d89e9adf2a4400176eba51ba9c28/dlopen2/src/raw/unix.rs).
This is a rejected drop-in candidate, not a completed soundness audit of that
dependency.

Existing winsafe **0.0.29**, revision
`71ed88c2a0d18b03ee452f6d4261f4c22f443483`, exposes safe HINSTANCE::LoadLibrary
using LoadLibraryW and a FreeLibraryGuard. The repository instead calls
LoadLibraryExW with LOAD_WITH_ALTERED_SEARCH_PATH and retains handles on drop.
The inspected safe wrapper is consequently not an exact replacement. Its
GetModuleHandle does not independently solve owned lifetime acquisition;
unsafe pointer construction is not unsafe removal. See
[winsafe HINSTANCE, LoadLibrary lines 271–277](https://github.com/rodrigocfd/winsafe/blob/71ed88c2a0d18b03ee452f6d4261f4c22f443483/src/kernel/handles/hinstance.rs#L271-L277).

## Preserve versus repair

Current Unix behavior includes byte names truncated at the first NUL,
dlopen(NULL), and on glibc a pre-open null-handle global symbol lookup.
Do not replace that global scope with a main-program handle without proving
symbol-scope equivalence. Repeated opens overwrite handles without balancing
old references; destroy/drop do not unload. These are existing ownership
behaviors to catalogue, not automatically fix in an unsafe-removal refactor.
Windows stores retained errors globally, formats through reusable buffers and
ignores formatting/conversion failure; a Result-based wrapper is not the same
diagnostic API. Windows close leaves a stale non-null handle and reopening
reuses it while preserving closed=true: record this state-machine problem in
the ownership fix's scope rather than calling it successful reopening.

The existing fake tests explicitly pin cached-open reuse and forwarding of
cached or loaded handles to close. They establish dispatch, not correct OS
reference ownership. Change those expectations under Bug #170 with a named
reference-counting fake: borrowed handles must never be released as owned,
owned acquisition precedes use, failure leaves no usable handle, close is
once-only, and reopen cannot reuse a released handle. Include safe controlled
native fixtures only after selecting the ownership design; do not unload
system modules or induce races to prove the defect.

Next: fix Bug #170 through its dedicated bug workflow, then record the generic
open/lookup/close safety-contract decision under #88 and seek explicit migration approval.
Keep raw-byte/error-consumption and loader-mode fixtures separate from ownership
fixes. No evaluated wrapper removes all unsafe while preserving these contracts;
this bounded search is not proof that no established alternative exists.
