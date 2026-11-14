# Rational family replay and concurrent-operation controls

## Compared state

The private family controls compare the complete frozen B-rep bytes, sorted
semantic keys, authored history records, per-member coverage and ordered native
reports, native volume and integration-estimate bits, and the retained
flux/ideal-measure comparison certificates (#418). The fixture combines a
line/quadratic/cubic contour with a hole and a separate unit square.

The replay test constructs a parent baseline and starts two independent copies
of the test executable. One child reverses the selected-region order. Both
children must produce the same signature as the baseline. Commands configure
only their own child environments; they do not mutate the running process's
environment. The ignored worker test is invoked explicitly by the parent
control, rather than counted as an independently executed acceptance case.

The concurrent control starts two operation-local workers together, then runs
two complete family operations per worker with alternating selection order.
Every signature must equal a sequential baseline. A further sequential operation
must still match after the workers finish. The start barrier is used once so a
later worker failure cannot strand its peer at another barrier. These controls
compare actual geometry and history, not only result counts or volume.

## Sanitizer limitation

The repository's pinned `nightly-2026-09-25` ThreadSanitizer startup fixture was
compiled with `-Zbuild-std`, `-Zsanitizer=thread` and forced frame pointers for
`x86_64-unknown-linux-gnu`, following the environment settings in
`scripts/build/src/thread_sanitizer.rs`. Compilation completed, but startup
terminated with SIGSEGV (exit 139) before its `main_entered` marker. A direct run
with diagnostic verbosity and a syscall trace reproduced that pre-main crash.
The trace ended in a null-address `SEGV_MAPERR`; it did not identify the root
cause. No rational-family test ran under the sanitizer.

This failure is not passing race evidence. Sanitizer execution and a broader
concurrency stress campaign remain open. No host settings or sanitizer checks
were weakened to obtain a pass. The normal replay controls do not establish
absence of data races.

## Remaining scope

The family is still unqualified. The later
[cancellation and isolation controls](FAMILY-CANCELLATION.md) cover stage-boundary
cancellation, failures and source mutation. In-stage cancellation, aggregate
operation policy, the final publication contract, glyph scale, oblique frames,
general conical profiles and replacement deletion remain in the approved work. The earlier detached-export mutation control remains
separate. No production behavior, public API, package version, dependency or
release tag changed.

## Verification

`cargo test -p martensite-tksketch --quiet` passed 126 library and 32 integration
tests on Linux x86_64, taking 20.96 and 1.97 seconds excluding compilation. One
subprocess worker was ignored by the default harness and explicitly executed
twice by its parent test. Strict all-target clippy, formatting and whitespace
checks passed. The approximate scan found no changed family function over 20
lines, file over 500 lines or nesting above two. The full affected-package PR
gate remains required before a push.

Run the replay controls with:

```sh
cargo test -p martensite-tksketch --lib wrapped_profile::family::replay_tests
```
