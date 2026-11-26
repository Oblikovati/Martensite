# Kernel Thread-Local and Global Mutable State Inventory (#62)

This document classifies every `thread_local!` and global mutable state site across
Martensite crates. New `thread_local!` or `static mut` declarations in `crates/*/src/`
outside the explicit allowlist are rejected by `scripts/build/src/global_state_guard.rs`.

## Eliminated in Wave 1 (#62)

- `crates/martensite-tkfillet/src/blend_func/const_rad.rs` (`SCRATCH`): moved to per-instance
  `ConstRad::scratch`.
- `crates/martensite-tkfillet/src/blend_func/evol_rad.rs` (`SCRATCH`): moved to per-instance
  `EvolRad::scratch`.

## Remaining Allowlisted Sites (Tracked for Deletion or Runtime-Confined)

| File | Category | Rationale / Tracking |
| --- | --- | --- |
| `crates/martensite-tkernel/src/standard/error_handler/handler_stack.rs` | Runtime-confined | OCCT `Standard_ErrorHandler` per-thread jump stack (thread-confined by contract `#61`). |
| `crates/martensite-tkernel/src/ncollection/packed_map/storage/allocation/observation.rs` | Test-only trace | Allocation trace hook used only by unit tests. |
| `crates/martensite-tkbool/src/top_ope_brep_build/*.rs`, `top_ope_brep_tool/*.rs`, `top_ope_brep_ds/*.rs` (9 sites) | Legacy `TopOpeBRep` | Legacy Boolean engine slated for full deletion after `#44`/`#45`/`#46`. |
| `crates/martensite-tkxsbase/src/if_select/work_session/mod.rs` | Exchange session | Legacy STEP/IGES `XSControl_WorkSession` (scoped for M7 `#57`). |
| `crates/martensite-tkdestep/src/step_selections/{selects,select_instances}.rs` | Exchange session | Legacy STEP selection context (scoped for M7 `#57`). |
| `crates/martensite-vulkan/src/backend/draw_table.rs` | Renderer scratch | Thread-local Vulkan draw-table buffer (non-kernel viewport crate). |
| `crates/martensite-boolean-cases/src/support/{process,heal_commands}.rs` | Test harness | Non-kernel test runner support. |
