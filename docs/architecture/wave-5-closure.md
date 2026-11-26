# Wave 5 Final Closure & Disposition Notes

This note records the final Wave 5 deliverables and the verified status of the
remaining backlog items after PR #617 (Waves 0–3) and PR #618 (Wave 3 remainders + Wave 4).

## Implemented in Wave 5

- **#506 (Local `xtask` bounded build-artifact defaults)**: `scripts/build/src/cargo.rs`
  now applies `.github/workflows/martensite.yml`'s bounded artifact defaults
  (`CARGO_INCREMENTAL=0`, `CARGO_PROFILE_DEV_DEBUG=0`, `CARGO_PROFILE_TEST_DEBUG=0`,
  `CARGO_PROFILE_DEV_STRIP=debuginfo`, `CARGO_PROFILE_TEST_STRIP=debuginfo`) when the
  caller has not set an explicit environment override, verified by unit tests in
  `scripts/build/src/cargo/script_budget_profile_tests.rs`.
- **#471 (Single owner for `martensite-vulkan` shared support unit tests)**: Moved
  shared support-module unit tests into one owning binary (`support_units`), avoiding
  ~28,000 duplicate test executions across 116 Vulkan test binaries.
- **#458 (Translation-invariant rational Green integration)**: `martensite-tkmath`
  `rational_integrals::curve::shift_to_start_endpoint` shifts rational curves in exact
  `BigInt` arithmetic to their start endpoint and adds the exact closed-form boundary
  correction `(X(0)*Y(1) - Y(0)*X(1)) / (2*W(0)*W(1))`, certifying translated circles
  and ellipses within the default work allowance.
- **#126 (`nix 0.31.3` `Dir::from_fd` leak guard)**: Added a manifest-level regression
  guard in `martensite-tkernel` ensuring `nix`'s `dir` feature stays disabled until an
  upstream fix is adopted.

## Verified Already Resolved on `main`

- **#344**: Windows ACL execute-access fixture fixed on `main` (`2d124de75`, `a552341e4`).

## Feature-Branch / Long-Horizon Research Items (Tracked for #414/#418 and Hardware QA)

- **#253, #389, #424, #427–#431, #436, #443, #444, #496, #498, #499, #520, #521, #537**:
  These track either upstream Lavapipe/hardware pixel quirks (#389), long-horizon
  profiled performance investigations (#253, #444, #496), or unmerged wrapped-cone
  Phase A/B1 proof-architecture items (#424, #427–#431, #436, #443, #498, #499, #520,
  #521, #537) whose foundations are documented in
  `docs/features/surface-normal-sketches/IMPLEMENTATION-STATE-BRIEFING-2026-11-19.md`.
