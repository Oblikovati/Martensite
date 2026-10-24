# martensite-tkernel and martensite-cmath

`martensite-tkernel` is the foundation toolkit: typed errors and failure
stacks, memory managers and allocators, the collection types, ASCII and
extended strings, operating-system services (files, paths, processes, timing,
signals, shared libraries, threads), messages and progress, quantities, units and
resource files. `martensite-cmath` is the small adapter to the platform C math
library that `martensite-tkernel` and the toolkits above it call when Rust's
own formulas can round differently (`asinh`, `acosh`, `atanh`, `remainder`).

## Tests

Each crate's tests are under `crates/<crate>/tests` and run with `cargo test`.
Most case files are modules of one `toolkit` executable, listed in
`tests/suites/`. Cases that change process-wide state (locale, signals,
allocator callbacks, the default messenger) are separate `[[test]]` targets,
with the reason next to each in `Cargo.toml`. `tests/case_listing.rs` fails
when a case file is in neither place.

Some tests compare output with reviewed baselines in
`crates/martensite-tkernel/tests/fixtures/`; see
[that README](../../../crates/martensite-tkernel/tests/fixtures/README.md).

Controlled sequence-clear tokens belong to one invocation. Calling `clear`
cancels the active invocation; its tokens cannot visit, detach or finish a later
clear. Rejected stale tokens preserve the current invocation and its live values
([#77](https://github.com/Oblikovati/Martensite/issues/77)).

Tests whose expected values exist only for one platform are compiled only
there, for example the glibc `printf` rows on x86_64 Linux.

## Unsafe code

Both crates are the workspace's FFI, allocator and raw-memory exception to the
`unsafe_code = "forbid"` rule. See "Unsafe code exception" in
[AGENTS.md](../../../AGENTS.md). Both enable clippy's
`undocumented_unsafe_blocks` lint, and every unsafe block and impl has a
`SAFETY` comment on Linux, macOS and Windows.

The `unsafe impl Send` and `unsafe impl Sync` of the collections (List,
Sequence, SparseArray, PackedMap, the allocators and their iterators) have no
dedicated Miri or loom test of cross-thread use yet, which "Ownership and
thread safety" in AGENTS.md asks for. Some tests already run under Miri
(`cfg(miri)` limits their size). Adding the cross-thread tests is planned work.

## Platform support and future work

Linux is complete. On macOS and Windows every API exists and compiles: a
platform without a port returns a typed unsupported error or an unknown value,
and keeps the Linux algorithm where that is meaningful. Each item below is a
`TODO(platform)` comment in the source. The list is the planned work; it is
not shipped as stubs.

### Windows

- Files and directories (`src/osd/file/native_std.rs`): the Win32 file node,
  file and directory backend (handles, ACL protections, `LockFileEx`,
  local-time moments) is not implemented. The backend runs the Unix algorithm
  over `std::fs` and has not been run on Windows.
- Directory iteration (`src/osd/file/iterators.rs`): the Windows build closes
  its `FindFirstFile` handle where the iterator is reset.
- Host queries (`src/osd/process/host.rs`, `src/osd/process/host/native.rs`):
  `GetVersionExW`, `GetComputerNameA`, `GlobalMemoryStatus` and Winsock
  `gethostbyname` are not implemented. `Host` records `ERROR_CALL_NOT_IMPLEMENTED`
  before it reaches them.
- Process queries (`src/osd/process/native.rs`): the Administrators SID check
  through the token APIs reports `ERROR_CALL_NOT_IMPLEMENTED` (120); the login
  name through `GetUserNameW` is not implemented.
- Memory counters (`src/osd/process/mem_info.rs`): `GlobalMemoryStatusEx`,
  `GetProcessMemoryInfo` and `_heapwalk` are not implemented, so `update` reports
  every counter as unknown.
- Signals (`src/osd/timing/signal_native.rs`): only the console Ctrl-C and
  Ctrl-Break handler is installed. Faults (SEH exceptions and `signal()`
  translation) are not turned into typed failures, because Rust cannot unwind
  out of a fault handler.
- Shared libraries (`src/osd/process/shared_library/windows_backend.rs`): the
  loader calls `GetModuleHandleW`, `LoadLibraryExW`, `GetProcAddress`,
  `FreeLibrary` and `FormatMessageW` behind an injected seam.
  It is tested with fakes and cross-compiled, not run on Windows.
- Locale conversion (`src/ncollection/utf_string/locale_windows.rs`): the
  `CP_ACP` conversion uses `MultiByteToWideChar` and `WideCharToMultiByte`
  behind an injected seam. It is tested with fake code pages and
  cross-compiled, not run on Windows.
- C formatting (`src/c_format/text.rs`, `src/c_format/validation/kinds.rs`):
  MSVC's C-locale `wctomb` maps units 0x80..0xFF to single bytes, and its
  `printf` has no `%m`. Only the glibc behavior is implemented; `%m` is rejected as
  unsupported on Windows.
- Heap release (`src/ncollection/allocator/free_outcome.rs`): Common, Heap and
  Aligned use the same Rust allocation backend on every platform. Their original
  live allocations are compatible; arena-interior addresses remain incompatible
  with heap release. The classifier regression runs on every platform (#78).

### Other targets

- `long double` formatting (`src/c_format/long_double.rs`): AArch64 and other
  non-x86 Unix targets store `long double` as IEEE binary128. Only the x86
  glibc spelling of `%La` is implemented; those targets use the binary64 spelling.
- Process CPU time (`src/osd/process/native.rs`): a host with neither the Unix
  nor the Windows clock API has no implementation.

### Release matrix

The release matrix targets Linux x86_64, macOS x86_64 and arm64, and Windows
x64. ARM64 release support is limited to macOS (#198). Run the manual `release`
dispatch on the exact candidate commit and require all four native targets to
pass before tagging. Target configuration is not evidence of a successful run;
retain the commands, diagnostics and results from that candidate.

### Windows staged document writes

The access-policy boundary refuses replacement of an existing destination with
named streams, including `Zone.Identifier`, before changing file metadata. The
permission error leaves the original bytes and streams intact. Saves to ordinary
files remain supported. Stream-enumeration errors also refuse publication rather
than assuming metadata is absent (#248).
