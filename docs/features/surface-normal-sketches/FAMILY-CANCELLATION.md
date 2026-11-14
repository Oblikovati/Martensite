# Family cancellation and input isolation

## Current cancellation contract

The private rational family orchestrator now accepts an operation-local
cancellation callback (#418). It observes cancellation before and after each
outer stage: selection, member construction, shared encoding, family coverage,
history, member storage, global source binding, member checks and completion.
Member stages identify their authored region. The final completion check runs
before the family is handed to its caller.

The first observed cancellation is latched. A later callback returning false
cannot resume that operation. If a stage returns a completed value after
cancellation was requested, the control drops that value and returns a typed
cancellation with the observed stage and region. It also refuses any later stage
on the same control. The callback and its state belong to the operation; no
global or thread-local cancellation state was introduced. The control has a
compile-time `Send` check.

Selection forwards the callback into the existing periodic material checker,
including its probe and winding work. Other existing certificate calls still
use their current non-cancelling callbacks. Native construction, encoding,
coverage, storage and member-check routines cannot yet be interrupted while
running inside their outer stage. Cancellation is observed when they return;
no latency bound is claimed. Propagating cancellation through those routines
remains work before publication. No work budget, tolerance or geometric check
was relaxed.

## Isolation evidence

Six geometry controls compare every request field before and after operations,
using bits for all floating-point values. They cover:

- cancellation at every outer checkpoint, including the second member after
  earlier native or certificate work;
- cancellation after a completed shared encoding and during nested selection
  polling, with no partial family returned;
- unchanged inputs after success, and unchanged retained sources and frozen
  geometry after the caller later mutates its source;
- failure in storage of a later member, leaving the original requests and
  decoded frozen geometry unchanged;
- unchanged successful replay after a planning refusal and late cancellation.

Two focused control tests additionally verify cleanup of a completed value,
refusal of later work, and latching when an inner operation ignores cancellation.
These establish the tested ownership paths. They do not prove absence of
allocation leaks or replace the remaining sanitizer and broader stress gates.
The detached-export mutation and separate-process replay controls remain in
[Frozen families](FROZEN-FAMILIES.md) and [Family replay](FAMILY-REPLAY.md).

## Verification and remaining work

On Linux x86_64, the affected sketch package passed 134 library tests and 32
integration tests in 22.69 and 2.02 seconds excluding compilation. The ignored
subprocess worker remains explicitly invoked by its parent test. Strict
all-target clippy, formatting and whitespace checks passed. The approximate
scan found no changed family function over 20 lines, file over 500 lines or
nesting above two. The full affected-package PR gate remains required before
pushing the branch.

The family remains unqualified. In-stage cancellation, aggregate operation
policy and the final publication contract remain open, along with the recorded
ThreadSanitizer startup failure, broader stress testing, general conical
profiles, additional curve classes, glyph scale, oblique frames and replacement
deletion. No public API, dependency, package version or release tag changed.
