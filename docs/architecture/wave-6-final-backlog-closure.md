# Wave 6 Final Backlog Closure Notes

This note records the resolution of the final 17 backlog issues after PR #617
(Waves 0–3) and PR #618 (Wave 3 remainders + Wave 4 + Wave 5).

## Implemented in Wave 6

- **#253 (Reduce repeated extrema grid work in polar projection corpus)**:
  `crates/martensite-tkgeombase/src/proj_lib/polar/` (`initial.rs`, `start_point.rs`,
  `walk.rs`) now retains one operation-local `ExtPS<'a>` on `Sampling<'a>`, reusing
  the precomputed `GenExtPS` surface grid across all global projection queries for
  the same surface and tolerances (re-initializing only when `first_extrema` reduces
  tolerances to `p_confusion()`). All 164 cases in `pl_polar_complete_family` pass
  with frozen fixture hashes unchanged in 111.62 s (down from >340 s unfinished).
- **#389 (Vulkan point UV palette tests lose requested size-1 colored points on pinned lavapipe)**:
  Reproduced locally on Mesa 26.1.6 `llvmpipe (LLVM 20.1.2, 256 bits)` (7 passed,
  3 failed) and isolated the root cause to stale cross-runtime entries in the host
  Mesa disk shader cache (`~/.cache/mesa_shader_cache_db`). With an isolated
  `MESA_SHADER_CACHE_DIR` (both cold and warm runs) or `MESA_SHADER_CACHE_DISABLE=1`,
  all 10 `point_uv_pixels_tests` pass deterministically (0.59–1.00 s). Updated
  `scripts/install/vulkan-validation.sh` to isolate `MESA_SHADER_CACHE_DIR` inside
  the pinned lavapipe directory for both local and CI runs.
- **#424 (Explicit volume GK integration spends excessive work on symmetric half-turn cone contributions)**:
  Added explicit finite work limits (`VolumeGKLimits`) and bounded public entry points
  (`BRepGProp::volume_properties_gk_bounded`, `BRepGProp::volume_properties_gk_plane_bounded`,
  `BRepGPropVinertGK::set_limits`, `BRepGPropTFunction::set_max_evaluations` /
  `set_max_inner_iterations`) in `martensite-tktopalgo`, applying bounded default caps
  (`max_t_evaluations: Some(12_000)`) so symmetric near-zero faces return the documented
  negative incomplete status instead of stalling CPU-bound. Added unit and half-turn
  conical sector regression tests verifying bounded termination, explicit exhaustion
  status, and closed-form volume match (`53.76 * pi / 0.7`) on the axial plane route.

## Verified Already Resolved on `main`

- **#496**: Parallel self-intersection checker and contiguous `IntSurfLineOn2S` storage
  merged in PR #577 (`perf/496-restore-parallel-self-check`, 14 commits).
- **#498**: Certified self-intersection execution selection
  (`WrappedConeExecutionSelection::DirectedCommonChartRequireCertifiedV1`) landed on `main`.
- **#499**: Conical cap construction no longer uses legacy `GeomBezierSurface::with_weights`.
- **#520**: Cone cancellation preserves admission/engine refusal before late cancellation
  (verified by `explicit_legacy_preserves_admission_failure_before_a_late_cancel`).
- **#521**: Cone coverage report preserves verified cap/side pcurve counts without the
  `edge_uses / 3` heuristic.
- **#537**: Common-chart Phase A (`PR #549`) and B1 (`PositiveXCommonChartV2`) replaced
  independent per-span midpoint charts on `main`.

## Superseded by Archived Prototype Replacement

- **#427, #428, #429, #430, #431, #436, #443, #444**:
  These tracked defects on the unmerged draft branch `feature/wrapped-polynomial-profiles`,
  which was archived to `archive/wrapped-polynomial-profiles-2026-11-13` and replaced by
  the approved common-chart rational wrapped-profile architecture now on `main` (see
  `docs/features/surface-normal-sketches/IMPLEMENTATION-STATE-BRIEFING-2026-11-19.md`).
