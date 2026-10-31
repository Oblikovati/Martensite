# Windows native-boundary inventory review

This review accompanies the source-inventory baseline update for #334, #170,
#336 and #337. It covers the Windows implementation in the current checkout,
not the unfinished unsafe-storage migration. No crate version is changed.
Display leases keep behavior version 1; geometry replay versions are unchanged.

## Source delta

| Source | Added unsafe tokens | Boundary and review |
| --- | ---: | --- |
| `c_format/text.rs` | 1 block | Test-only native `snprintf` oracle. Terminated formats and matching promoted arguments use a bounded writable buffer. |
| `ncollection/utf_string/locale_windows.rs` | 1 block | Test-only `GetACP` query, with no pointers or mutation. |
| `osd/document_file/windows_creation.rs` | 3 blocks | Native node creation and unique handle adoption. Descriptor allocations and terminated paths outlive each call; the invalid-handle sentinel is checked before ownership transfer. |
| `osd/document_file/windows_protection.rs` | 3 blocks | Borrowed ACL/SID views remain within owned, Windows-validated descriptors. Native access-mask queries neither retain nor own pointers. |
| `osd/process/host/native/windows_version.rs` | 2 blocks, 1 extern | Initialized SDK version record, correct size and calling convention; NTSTATUS conversion receives a scalar. Std and the installed WinSafe release expose no equivalent native-version query. |
| `standard/c_locale_sentry/locale_windows.rs` | 5 blocks, 1 extern | CRT per-thread locale configuration and locale calls. Returned names are copied before mutation. Restoration changes the name before disabling per-thread mode. The public sentry remains thread-confined. |
| `wnt/native.rs` | 2 blocks | One imports an opaque HWND without taking ownership; the used native queries reject invalid identifiers. One creates a test-only OS-class window with no application pointer. |
| `wnt/owned.rs` | 4 blocks | Class registration, native creation, dispatch and default procedure. The procedure has static lifetime; creation carries no application pointer. One owner thread pumps and destroys the HWND before joining. |
| `wnt/raw_input_native.rs` | 7 blocks | Native registration and bounded input/device queries. Aligned storage is checked against copied lengths before reading headers or bytes; the device discriminator guards the HID union member. Reports own their copied bytes. |
| `tests/osd_timing_api_behavior/windows_signals.rs` | 3 blocks | Test-only console membership, Ctrl-C ignore-state and event-generation calls. WinSafe creates a hidden child with a new console; membership is checked before generating events. A distinct exit code proves the selected child check ran. This integration-test delta appears in the compiler inventory. |

Five additional baseline entries only record `forbid(unsafe_code)` in the
safe access-check, temporary-node, host-query, memory-query and identity adapters.
They contain no unsafe operation. No new unsafe trait, unsafe function, or
unsafe `Send`/`Sync` implementation was introduced.

The executable and CRT are system boundaries. WinSafe supplies safe identity,
memory and window operations where available. Existing descriptor ownership
utilities are reused for file ACLs and creation. `tempfile` supplies random-name
collision retries; it does not hide new custom storage or pointer code.

## Evidence and limits

Native regressions passed for identity, host queries, memory sampling, native
version, ACLs, atomic creation, access checks, temporary nodes, loader ownership,
code-page conversion, CRT locale scopes, MSVC formatting and iterator cleanup.
Window tests passed for real HWND wrapping, creation, cross-thread drop, display
leases and raw registration. Synthetic raw packets cover bounds and queue
overflow; physical HID delivery remains unverified.

`cargo xtask pr --base origin/release/0.5` passed on Windows x64 MSVC, including
the source and compiler inventories, formatting, Clippy, all-target builds and
unit, integration and doc tests for all 21 affected workspace packages. The
unchanged `martensite-cmath` package passed its 42 toolkit tests and 30 doctests
in the workspace baseline run. The approved signal policy under #88 is recorded
in [the signal contract](signal-contract.md).

The capability query passed its state-preservation and `Send`/`Sync` check.
All 13 timing/signal tests passed, including real Ctrl-C and Ctrl-Break delivery
and floating-point controls in hidden child processes. The focused Clippy and
all 303 foundation doctests passed after rebuilding the current library.

The public HID report and batch types passed direct `Send`/`Sync` compile
checks, followed by service test-target Clippy. Native window tests covered
concurrent creation and destruction after transfer to another thread. The
repository's ThreadSanitizer job targets Linux; it does not execute these
Win32 FFI paths. Physical HID delivery remains outside this software validation.

## Compiler measurement

The pinned Windows MSVC measurement completed with 439 grouped workspace
diagnostic locations, compared with 428 in the previous baseline. Twelve
groups changed: eleven new groups and one additional native-code-page test span.
Every delta maps to a source boundary or oracle listed above. Runtime blocks
repeat in library and test compilation; test-only blocks appear once. No
other compiler group changed.

The approved signal-policy tests added one group containing three test-only
console calls to the preceding 438-group measurement. No existing group changed
or disappeared in that comparison.

Bug #340 moved the helper below its timing suite. A fresh native measurement
changed only that group's source path; all 439 groups and their counts were
preserved. The baseline records both paths and the preceding report hash.

The complete report is retained as `evidence/compiler-windows-native.json.gz`;
its SHA-256 is recorded in the Windows compiler baseline's review metadata.
The compiler identity stayed `rustc 1.99.0 (b940084d7 2026-09-28)`. Only the
Windows expectation was updated. Other platforms remain unvalidated by this
measurement. The PR gate passed lexical inventory, whitespace and rustfmt
before this comparison. Normal Clippy and tests remain separate required checks.
