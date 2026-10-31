# Windows platform backends

Tracked by #334 on `release/0.5`.

The Windows PR gate builds `martensite-build` with all targets in
`target/build-driver-validation`. Other packages retain the caller's target
directory. This avoids replacing the running build-driver executable, which
Windows locks, while preserving build coverage (#339).

Use the Rust toolchain pinned in `rust-toolchain.toml` through rustup. On the
development machine, a standalone GNU Rust installation preceded rustup in
`PATH`; invoking `%USERPROFILE%\.cargo\bin\cargo.exe` selects the pinned MSVC
toolchain. The initial MSVC `cargo check --workspace --all-targets --locked`
passed on Windows x64. This checks compilation, not test execution or runtime
backend completeness.

## Process identity

`Process::user_name` reads the account of the effective Windows token through
WinSafe's `GetUserName` wrapper. It returns UTF-8 bytes and preserves native
error codes. It does not trust the `USERNAME` environment variable.

`Process::is_super_user` checks membership in the built-in Administrators group
through the effective token. Under UAC, a filtered administrator token can
return false. WinSafe owns the well-known SID allocation; no custom unsafe
code is needed for either query.

Windows x64 validation: both native identity unit tests and all seven tests
selected by `cargo test -p martensite-tkernel --test toolkit
osd_process_api_behavior` passed. `cargo deny --all-features check` and
`cargo audit` passed after enabling WinSafe's `advapi` feature. No dependency
version changed.

## Host queries

The Windows C-locale sentry now enables a per-thread CRT locale and restores
the previous name and mode on drop, including nested scopes and panic cleanup.
All four native regression tests passed, including previous-mode restoration
and simultaneous threads with different CRT locales. Windows iterator destruction also releases
its cursor and supports reinitialization; its native regression test passed.

Bug #337 is checked against native MSVC `snprintf`: byte-sized wide units above
ASCII are accepted, while an unencodable single character produces no output,
including field padding. Wide strings still fail if a unit cannot be encoded.
The native comparison passed for plain, right-padded and left-padded directives.

Windows computer names and available pagefile-backed memory use WinSafe.
IPv4 resolution uses `ToSocketAddrs`, which manages Winsock initialization.
The existing signed 32-bit memory API reports bytes capped at `i32::MAX`
instead of wrapping large values. Query failures record the native error.

## Process memory

Windows reads private committed bytes, working-set size and peak, and pagefile
usage and peak through WinSafe's Psapi wrapper. It sums committed and reserved
address regions for virtual size and busy blocks in the locked default process
heap for heap usage. The heap walk performs no allocations while holding the
lock. These are separate samples, not an atomic process snapshot. Unavailable
figures remain unknown, and disabled counters are not published.

The four process-memory regression tests passed on Windows x64, including the
native address-space and heap queries.

## Native version boundary

`Host::system_version` reports `Windows NT major.minor.build`. The standard
library and the installed WinSafe release do not expose the native version
query. A separate adapter calls `RtlGetVersion` with an initialized SDK
`OSVERSIONINFOW` structure and converts any failed NTSTATUS to a Win32 error.
This is an operating-system boundary under the existing tkernel exception;
the remaining Windows host-query module still forbids unsafe code.
See [Microsoft's API contract](https://learn.microsoft.com/en-us/windows/win32/devnotes/rtlgetversion).

## Native runtime coverage

- Signal and exception translation: see the [approved contract](signal-contract.md).

The Windows floating-point trap mask passed an enable/read/disable regression
test on a dedicated thread. The approved policy keeps hardware faults fatal;
`Osd::signal_capabilities()` reports available interruption and trap mechanisms
separately from recoverable native faults.

The Windows child-test helper lives below the timing integration suite and is
included explicitly. The toolkit's registration guard continues to reject
unregistered top-level case files (#340).

Win32 multi-axis raw input is opt-in through `WntWindow::enable_raw_input`.
Owned windows copy HID reports before native message cleanup and expose them
through `take_raw_input`, with acquisition errors and dropped-packet counts.
Queues retain at most 4096 reports/errors and 16 MiB of report bytes. Packets
are bounded to 1 MiB and checked for complete dimensions. Borrowed windows use
their application's loop and `WntWindow::read_raw_input`. Windows routes this
process-wide device usage to the last registered HWND. Physical HID delivery
still requires a device-backed run; synthetic packets and native registration
tests cover the software path.

Windows display connections now borrow the process HINSTANCE used for Win32
surfaces. They do not open an X server, intern X atoms or free the executable
module. WNT windows retain a display lease through native teardown; changing
the base window's display metadata does not replace this retained binding.
Reset remains refused while the window lease is held. Explicit borrowed module
bindings retain the caller's ownership contract.

Native Windows loader tests now verify borrowed-cache close, owned close and
reopen, symbol lookup and missing-library diagnostics. Bug #170 separates cached
borrowed handles from owned loader references; #336 clears closed handles before
reopening and restores symbol lookup on success. All three native loader tests
passed on Windows x64. Display routing passed three lifecycle tests and all
the WNT unit tests; its Clippy check passed as well.

Active-code-page conversion is now runtime-tested with ASCII, embedded-NUL
truncation, the retained extra terminator and insufficient output space. The
non-ASCII fixture covers Windows-1252 and UTF-8 active code pages; other active
code pages exercise the ASCII fixture. All five combined loader/code-page tests
and the tkernel Clippy check passed on Windows x64.

The Windows std file backend already uses native handles through `std::fs` and
native locks through `File::lock`. Bug #335 releases its descriptor registry
before blocking I/O; owned references keep in-flight handles alive. Windows
file moments now use WinSafe's UTC-to-local timezone conversion, including
historical daylight-saving rules exposed by that API.

File-node protection queries and updates now preserve the four typed Windows
principals through the backend. Queries inspect each principal's native ACL
rights. Setting protections replaces the DACL with protected explicit grants
for system, owner, primary group and everyone; unrelated grants and inherited
grants are replaced by this explicit policy. Identical principal SIDs combine
their grants. Read, write, execute and delete remain distinct. The adapter
reuses the existing owned security-descriptor utilities.

Typed creation protections now reach `CreateFileW` and `CreateDirectoryW` as
security attributes, so new nodes never appear with an inherited broad policy
before the requested policy is applied. The descriptor uses the effective
token's default owner and primary group. Reopening an existing file keeps its
ACL; append-only access is enforced by Windows. The four focused creation tests
passed on Windows x64.

Windows file access checks now open a non-creating, non-truncating handle with
the requested native read, write and execute rights. Windows checks the effective
token and ACL. Existence-only queries use metadata. Directory handles are enabled
without requesting backup privileges.

Windows temporary nodes now use `tempfile`'s random-name retry helper with
native exclusive creation and private ACLs for system and the effective owner.
Group and world receive no data rights. Both slash conventions are accepted in
the template, whose leaf must end in `XXXXXX`; nodes live in the system temporary
directory. The OSD caller retains cleanup responsibility. Tests inject a root,
verify distinct names and private ACLs, and reject invalid templates before any
node is created. The pinned `tempfile` dependency is now used at runtime on Windows.

`WntWindow::from_handle` now borrows a live application-owned HWND and reads its
rectangle and parent. Mapping, unmapping, titles, invalidation and resize-border
classification use the native window. Dropping the wrapper does not destroy the
application's window. The application keeps its window and message loop alive.
WinSafe supplies the calls; the integer-to-HWND adapter neither dereferences the
identifier nor takes ownership.

`WntWindow::new` registers the native window class and starts a dedicated owner
thread. That thread creates the HWND, pumps its messages and destroys it before
exiting. Drop requests shutdown and joins the owner, including after a wrapper
move to another thread. The callback delegates native messages to DefWindowProc
without retaining application pointers. Native class registration persists for
the executable's lifetime. The WNT unit tests passed on Windows x64,
including concurrent creation and destruction from a different thread.

The active image layer uses portable PNG, JPEG, BMP, GIF and SGI decoders on
Windows too. No load/save call routes through a missing WinCodec backend.
`ImageIwicPalette` is an unused opaque public marker, retained for source
compatibility; removing it is separate public-API work. A native WIC backend
would be an alternative implementation or additional-format feature, rather
than a missing path for the declared formats. The complete tkservice suite
passed on Windows with the portable image implementation.

## Final Windows validation

Validation used Windows x64 and rustup's pinned MSVC toolchain,
`rustc 1.99.0 (b940084d7 2026-09-28)`, on `release/0.5` at
`0c4f89ef050d4508c8ed4f6ae563496057ea6074` plus these working-tree changes.
The dataset submodule stayed at `c58ee236484bce7b385392454f976d001f8443c4`.

`cargo xtask pr --base origin/release/0.5` completed with exit status 0. It
passed source and expanded compiler inventories, whitespace, Rust formatting,
Clippy, all-target builds and unit, integration and doc tests for all 21
affected workspace packages. The unchanged `martensite-cmath` package had
already passed its 42 toolkit tests and 30 doctests in the workspace baseline.
The two additional HID type-bound assertions passed a focused unit check and
service test-target Clippy. `cargo deny --all-features check` and `cargo audit`
also passed for the dependency changes.

The full gate log is `target/windows-platform-validation/windows-final-layout-gate.log`.
The [native-boundary review](unsafe-boundary-review.md) records the measured
compiler evidence and its hash. Physical HID delivery, other operating systems
and Linux ThreadSanitizer execution are excluded from this Windows run. A
release still requires its full supported-platform matrix.
