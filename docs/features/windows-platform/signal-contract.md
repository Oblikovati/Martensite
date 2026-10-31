# Windows signal contract

Tracked by #334 and the API-conflict inventory in #88. The maintainer approved
option 1 in this chat on 2026-10-31: keep faults fatal, report capabilities and
verify console cancellation and trap controls in isolated child processes.
That implementation decision did not authorize a version bump. The subsequent
0.5.0 release request authorized the release's changed-package version updates.

## Current interface and behavior

`Osd::set_signal`, `Osd::set_thread_local_signal`, `SignalState::set_signal`
and `SignalState::set_thread_local_signal` return `()`. Their `SignalHost`
adapter also returns `()` for handler installation. Windows installs a console
Ctrl-C/Ctrl-Break handler and configures the calling thread's floating-point
control word. `SignalState::control_break` reports a pending console interruption
at an ordinary Rust call boundary as `Result<(), Failure>`.

The original Windows placeholder requested an unhandled-exception filter, CRT
signal translators and `_set_se_translator` to turn hardware faults into exceptions.
The current Rust implementation does not recover from access violations,
illegal instructions, stack overflow or native floating-point faults.

## Constraint

Microsoft's [`_set_se_translator` contract](https://learn.microsoft.com/en-us/cpp/c-runtime-library/reference/set-se-translator?view=msvc-170)
requires C++ `/EHa` and a translator that throws an exception for a matching C++
catch handler. The existing Rust setup functions establish no protected
operation boundary and cannot return an error at an arbitrary later fault site.

Rust's [`catch_unwind` contract](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html)
does not guarantee recovery from a foreign exception: permitted foreign
unwinding may abort the process or return an opaque error. Changing an ABI to
allow unwinding does not establish reliable hardware-fault recovery. A
long-jump over Rust frames would bypass ownership cleanup and cannot implement
this interface safely. Moving that jump into a dependency does not resolve it.

## Evaluated options

1. Keep hardware faults fatal. Preserve the existing setup APIs and console
   cancellation behavior. Add an explicit capability query distinguishing
   console interruption, floating-point trap configuration and recoverable
   native-fault translation. Report the last as unavailable and remove the
   misleading expectation that installing a translator remains routine porting
   work. Verify console behavior and trap configuration in disposable child
   processes. This is additive and does not change the crate version here.
2. Add process-isolated recovery around named operations. A worker process
   performs the operation; its parent reports abnormal termination as a typed
   failure. Define serialization, cancellation, history and result publication
   for those operations before implementation. A process-wide signal setter
   cannot supply this behavior by itself. This adds operation APIs and requires
   a separate reviewed design; no version bump is authorized.

Neither option implements automatic in-process hardware-fault recovery. That
behavior is not safely implementable through the current setup-only interface.
Option 1 is the approved policy for this Windows implementation under
AGENTS.md's API-conflict rule. Foreign unwinding is not a system-API exception.

## Capability query

`Osd::signal_capabilities()` returns `SignalCapabilities` without installing
handlers or changing trap masks. Windows reports console interruptions and
floating-point trap control as available, and recoverable native faults as
unavailable. These fields describe mechanisms, not successful handler
installation, current trap settings or the presence of a console.

The existing setup methods retain their signatures. Console cancellation
continues to appear as `OSD_Exception_CTRL_BREAK` at `Osd::control_break`.
Hardware faults retain the operating system's fatal behavior.
