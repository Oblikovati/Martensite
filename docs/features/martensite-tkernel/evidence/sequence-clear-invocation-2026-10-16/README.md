# Sequence clear invocation regression

[Bug #77](https://github.com/Oblikovati/Martensite/issues/77) was reproduced on
main `dd94c907` through the safe public API before implementation or issue
creation. A cancelled empty token finished a later nonempty clear with `Ok(())`.
The expanded regression also counted zero destructions of its live value where
one was required. No allocator-address reuse or invalid pointer was needed.

The token checked its sequence's identity and whether any clear was active.
That did not distinguish two invocations on the same sequence. A fresh local
`Arc<()>` identity now belongs to the existing liveness scope and its token.
Validation compares that identity before visiting, detaching or finishing.
Retained old tokens keep their identity allocation alive, preventing reuse.
The saved registry pointer, traversal and allocator behavior are unchanged.

Six independent regressions check stale finish, owned destruction, stale visit
and detach, repeated empty/cancelled operations, cancellation with a pending
node, and a valid token after moving the sequence. They use known integer
contents, refusal before state changes and an independently counted destructor.

Local Linux checks, Rust 1.99.0, two build jobs, locked dependencies:

- `cargo xtask pr --base main`: formatting, strict Clippy, affected build,
  **5,070 tests and 294 doctests passed**, no failures or ignores.
- `cargo test -p martensite-build --all-targets --locked`: **80 passed**.
- Pinned `nightly-2026-09-25` Miri with `-Zmiri-strict-provenance`:
  **all six new regressions passed**, with default isolation and no interpreter
  errors or leaks.

The first PR gate found the new module registration out of alphabetical order;
the formatter's ordering was applied before the final gate. Both that initial
failure and the original red regressions are retained. `SUMMARY.json` binds the
commands, current source/configuration hashes and compressed logs.

This branch uses the main baseline's 0.3.0 package versions. No versions,
manifests, dependency lock, original checkout, or release state were changed.
The coordinator must integrate the fix and check the combined candidate before
its release matrix. These checks do not prove cross-thread behavior or cover
every foundation operation under Miri.
