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

Selection forwards the callback into fit planning, global within-boundary
clearance, periodic boundary clearance and the material checker's probe and
winding work. The proof-work checkpoint below records the added controls.
Storage, cone and measure certificate calls still use their current
non-cancelling callbacks. Planar capture, native construction, encoding,
coverage, storage and member-check routines cannot yet be interrupted throughout
their work. Cancellation is observed when their enclosing stage returns; no
latency bound is claimed. Further propagation remains work before publication.
No work budget, tolerance or geometric check was relaxed.

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

## Planning and boundary proof checkpoint

The same operation-local callback now reaches the existing bounded arithmetic
inside a priori fit planning, the global within-boundary proof, periodic image
planning, and each periodic image's curve-pair proof. Default entry points
continue to delegate with a non-cancelling callback, preserving their existing
geometry, limits and work policies. Pre-cancelled selection preparation returns
its own typed cancellation before entering planar capture.

Six additional controls cancel after arithmetic begins in planning and within
clearance, at the transition to periodic work, and inside the final image of
the two-turn helical fixture. The last case preserves image turn 2 and the nested
arithmetic cancellation cause. The controls also cover pre-cancelled preparation
and authored-owner attribution for a cancelled fit. Callback counts select test
interruption points; they do not select geometry or become replay identity.

The affected sketch package passed 140 library and 32 integration tests on
Linux x86_64, taking 22.83 and 1.98 seconds excluding compilation. The ignored
subprocess worker was explicitly run by its parent replay test. Strict
all-target clippy passed. Existing default geometry, process replay, overlapping
operations and refusal-isolation controls remained green. No dependency suite
was rerun because dependency sources and build inputs were unchanged.
