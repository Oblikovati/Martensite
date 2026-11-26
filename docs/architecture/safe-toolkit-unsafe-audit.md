# Safe Toolkit and OS Soundness Audit

This document records the Wave 3 audit of `unsafe` usage across the Martensite workspace (`#83`–`#90`, `#95`, `#109`, `#171`, `#184`, `#573`).

## Workspace Unsafe Inventory

Only seven crates in `crates/` contain `unsafe` blocks. All other modelling, geometry, topology, boolean (`tkbo`), healing, fillet, offset, feature, HLR, meshing (`tkmesh`), and data-exchange toolkits (`tkdestep`, `tkdeiges`, `tkdeobj`, `tkdestl`, `tkdegltf`, `tkdeply`, `tkxcaf`) contain zero `unsafe` blocks and compile under `#![forbid(unsafe_code)]` or zero-unsafe enforcement.

| Crate | `unsafe` Matches | Primary Reason |
| --- | ---: | --- |
| `martensite-cmath` | 2 | Libm / C-math ABI seam |
| `martensite-tkbool` | 1 | Legacy `top_ope_brep_build` extension seam |
| `martensite-tkernel` | 2030 | OS FFI (`osd`), `SharedLibrary`, `NativeEnvironment`, and raw `NCollection` allocators |
| `martensite-tkmeshvs` | 1 | Legacy mesh visualization pointer seam |
| `martensite-tkservice` | 323 | FreeType, font manager, and platform windowing FFI |
| `martensite-tkv3d` | 13 | Anchored native const-char and view-source FFI seams |
| `martensite-vulkan` | 574 | Vulkan (`ash`) device/command FFI and isolated worker-process test texture env |

## Soundness Boundaries and Preconditions

1. **OS Dynamic Loading & Environment (`#171`, `#95`, `#109`, `#184`):**
   - Dynamic library symbol loading (`SharedLibrary`) and process environment mutation (`NativeEnvironment`) are confined to `martensite-tkernel::osd`.
   - Callers must guarantee single-threaded initialization or isolated worker processes before mutating process-wide environment variables.
2. **Vulkan Test Texture Environment (`#573`):**
   - `crates/martensite-vulkan/tests/support/dataset_shape.rs::select_predefined_textures` mutates `CSF_MDTVTexturesDirectory` only as the first line of an isolated case-runner worker process before the renderer spawns threads.
   - Future runner upgrades should inject `CSF_MDTVTexturesDirectory` via child-process `Command::env` in the case runner so the worker body requires zero `unsafe` blocks.
3. **Retained Allocation & Reset (`#104`):**
   - Safe retained-capacity slot reuse is provided in `martensite-tkmesh::retained_slot_pool::RetainedSlotPool<T>` using `Vec<Option<T>>` with zero `unsafe` code.
