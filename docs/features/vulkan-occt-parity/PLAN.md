# Plan: a Vulkan renderer for complex assemblies

Requirement (maintainer, 2026-11-22): `martensite-vulkan` must be at least as fast as OCCT's
`TKOpenGl` on every paired case, and much faster where large assemblies need it.
[README.md](README.md) has the measurements: today it is slower in 13 of 14 cases, by 8 to
1,550 times.

The renderer redoes on every frame work that OpenGL does once. The plan first makes the
frame cost proportional to what changed, which brings parity. Then it adds the techniques
that OpenGL does not have and that assemblies need: instancing, GPU culling and level of
detail.

## Constraints

- Pixels do not change in a structural phase. The recorded lavapipe images and the
  replayed requests of `martensite-vulkan` and `martensite-tkv3d` pass unchanged, with
  no validation message. A change that alters pixels or precision (compact vertex formats,
  moving CPU lowering into a shader) gets a new behaviour version, keeps the old one
  selectable, and changes a baseline only through a reviewed update (AGENTS.md, "Change
  and replacement").
- Public behaviour stays. `Graphic3dRenderView::capture` still returns the last frame
  or its error. Where the timing of an error has to change, for example a device error
  seen one frame later with frames in flight, that is a maintainer decision recorded
  before the phase.
- The old path is deleted in the same phase. A retained path does not ship beside the
  per-frame path it replaces.
- Unsafe code stays in `backend/`, with a `SAFETY` comment on every block. Views stay
  thread-confined; device resources belong to the device.
- Every phase reports the harness table (`cargo xtask occt-parity --toolkit vulkan
  --frames 10`) and the counters of phase 0, before and after.

## What to take from OCCT and from Oblikovati

OCCT's `TKOpenGl` is the parity reference. The Oblikovati viewport
(`Oblikovati/renderer` and `head/internal/native`) already draws 30,000-placement
assemblies and shows what works and what does not.

| Technique | OCCT `TKOpenGl` | Oblikovati viewport | This plan |
| --- | --- | --- | --- |
| Geometry stays on the GPU until it changes | yes: `OpenGl_PrimitiveArray` buffers, partial update from `Graphic3d_Buffer` invalidated ranges | yes, but one merged buffer for the whole scene in host-visible memory, keyed by content | phase 1: one device-local allocation per array, partial updates, sub-allocated |
| CPU frustum culling of structures | BVH (`Graphic3d_BvhCStructureSet`) | BVH over placement bounds, plus a cull of placements under one pixel | kept (already ported); phase 5 adds GPU culling per instance |
| Render targets and readback | targets kept, no readback unless asked | targets per frame slot, readback only on request | phase 2 |
| Frames in flight | driver-managed | 2 slots, linked by semaphores, one batched submit | phase 2 |
| Immediate layer for hover highlight | `redrawImmediate` over a copy of the main image | none: overlays and selection are baked into the merged buffer, so a change re-uploads everything | phase 4, keeping highlight out of geometry |
| State caching and sorting | `OpenGl_Workspace` aspect cache | draws sorted by pipeline | phase 3 |
| Pipeline cache | GLSL program manager | none | phase 3 |
| Instancing of repeated parts | instanced structures for connected presentations | per-instance matrices on vertex binding 1, one mesh per unique component (ADR-0038) | phase 5 |
| Indirect, GPU-driven draws | no | no | phase 3 (multi-draw) and phase 5 (GPU culling) |
| Occlusion culling, LOD, meshlets | no | no | phase 5 |
| Compact vertices | no | no: 64-byte vertices repeat the material in each vertex | phase 5 |
| Wide lines | GL line width | expanded to quads in the vertex shader, so vertices do not depend on the camera | phase 1 keeps camera-independent geometry retained |
| Tests that a static orbit uploads nothing | no | upload counters and an allocation test | phase 0 |

## Phase 0: measurement and guards

No behaviour change. Status: done, except the host setting of item 3. The README section
"Phase 0 baseline" is the table that phase 1 starts from.

1. Frame counters and phase timings on `Graphic3dRenderView`: device buffers and
   images created, bytes uploaded, bytes read back, pipelines created and descriptor sets
   allocated per frame, and CPU time for extraction, lowering, upload, recording,
   submission and waiting. The harness and the tests read them; OCCT's
   `Graphic3d_FrameStatsTimer` values stay as they are.

   Done as `Graphic3dRenderView::last_frame_work` and
   `Graphic3dRenderDriver::device_work`, which return `RenderDeviceWork` (README,
   "Device work counters"). Reading the `Graphic3d` arrays and the CPU lowering are one
   pass today, so they share the `scene` phase; phase 1 splits them when lowering is
   retained. The bench prints the counters per operation and the xtask shows them as a
   second table.
2. Assembly cases in the harness, each with its C++ twin:

   | Case | Scene |
   | --- | --- |
   | `frame_connected_30k` | 30,000 `AisConnectedInteractive` instances of 30 parts |
   | `frame_structures_100k` | per-draw cost at assembly scale |
   | `frame_mesh_10m` | GPU vertex throughput |
   | `select_10k` | a selection change in a large scene; must not re-upload geometry |
   | `close_10k` | structure removal, which is quadratic today |

   Done, each with its C++ twin and an equal checksum (README, "Harness").

3. A CPU profiler. Allow `perf` on the measuring host (`kernel.perf_event_paranoid`
   at most 1); `callgrind` instruction counts remain the portable fallback.

   The README ("Profiling") gives both procedures. The setting needs root, and this host
   still has 2, so `perf record` fails here until it is lowered.

## Phase 1: geometry stays on the GPU

Target: `frame_mesh_1m`, `frame_segments_1m`, `frame_points_1m`,
`frame_ais_spheres_400` and `upload_mesh_100k` reach parity. These are the 95 to 908
times cases, and the profile puts all their CPU time here.

1. GPU geometry store in `backend/`.
   - Each `Graphic3d` attribute or index buffer gets a device-local allocation, keyed by
     buffer identity and revision.
   - Allocations come from large memory blocks through a sub-allocator. Today each draw
     calls `vkAllocateMemory` twice per frame.
   - Uploads go through a staging ring, as one batched transfer per frame.
   - A mutable buffer uploads only its invalidated range (`Graphic3dAttribBuffer` already
     has the range and the flag).
   - Memory is released when the last frame that used it has retired.
2. Lowering is retained per group. The lowered geometry (hatching, guard bands, flat
   sources, quad corners) is cached and keyed by the group, aspect and array revisions.
   - The camera-dependent lowerings (projected dashes, viewport-clipped lines, wide-line
     replicas) stay per frame, for the draws that use them only.
   - Each one is then a candidate for a shader under its existing behaviour switch.
3. Bulk accessors.
   - Indices and attributes are read as typed slices of the `tkservice` buffers, instead
     of one trait call per index (37% of a mesh frame today).
   - Encoding is a slice copy, instead of an iterator per `f32`.
4. Per frame, only per-frame data: transforms, clip planes, lights and materials, in
   push constants and a uniform ring.

Acceptance:

- The five cases are within the harness tolerance of OCCT.
- A static orbit uploads 0 geometry bytes and creates 0 buffers (phase 0 counters, as a
  test).
- The existing suites pass with unchanged images.

## Phase 2: a frame pipeline without stalls

Target: `frame_empty`, `dump_1080p`, `resize_redraw` and `frame_msaa4_100k` reach parity.
The fixed frame cost falls from 0.5 ms to OCCT's 0.07 ms.

1. Persistent targets per view: colour, depth, stencil, MSAA and OIT images are created
   again only when the size or the sample count changes.
2. Capture on demand. The last frame's images are kept, and `capture` reads them back
   when called. The steps that run on the host copy today then move:
   - render-scale blit, subview composition and flipped output become GPU passes;
   - or, for capture only, they are applied at capture time.

   Captures stay identical to today's.
3. Two or three frames in flight.
   - Each frame slot has its own command pool, descriptor pool and uniform ring.
   - A fence is waited on only when its slot is reused.
   - A timeline semaphore orders uploads before rendering.
   - A window frame renders into, or blits on the GPU to, the swapchain image, with no host
     round trip.
4. Present modes stay as they are: immediate or mailbox without vertical sync, FIFO
   with it.

Acceptance:

- No readback in a static orbit (counter test).
- The four cases at parity.
- The error-timing decision recorded.

## Phase 3: draw submission at scale

Target: `frame_structures_10k`, `frame_transparent_1k` and `frame_text_1k` reach parity.
`frame_structures_10k` then goes from 388 ms to below OCCT's 2.1 ms; the aim is 0.5 ms.

1. Constant-time structure registry in the driver. `remove_structure` scans a
   `Vec<WeakHandle>` for each removal; a slot map makes closing a 10,000-structure view
   take microseconds, not 3.9 billion instructions.
2. Persistent draw records per group: pipeline, descriptor set and push-constant block
   are prepared once. A frame gathers the records that survive culling and sorts them by
   pipeline and material.
3. Pipeline cache. A `VkPipelineCache` saved on disk, and variants keyed by a hash,
   built once. `VK_EXT_graphics_pipeline_library` (available on RADV) shortens the first
   frame.
4. Multi-draw indirect for static draws in shared buffers:
   - `vkCmdDrawIndexedIndirectCount`, with per-draw data (transform and material index)
     in a storage buffer indexed by draw or instance index;
   - descriptor indexing for material and texture tables;
   - feature-gated, with ordinary binds where the device lacks it.
5. Text is retained: the glyph atlas and the label quads are keyed by text and aspect
   revision. Screen-space labels expand in the vertex shader, so a frame does no layout.

Acceptance: the three cases at parity; `close_10k` linear.

## Phase 4: the immediate layer

Target: `highlight_ais_spheres_400` reaches parity. It goes from 174 ms to below 0.11 ms
per hover pair, the 1,550 times case.

1. The main-layer colour and depth are kept per view. `redraw_immediate` draws only
   the immediate layers over a GPU copy of them, as OCCT's `redrawImmediate` does. A
   main-layer change invalidates the copy.
2. Highlight and selection are per-draw state (a uniform or material index), never
   lowered or uploaded again. This avoids the Oblikovati defect, where a selection change
   re-uploads the whole assembly.

Acceptance: the case at parity; `select_10k` uploads no geometry.

## Phase 5: faster than OpenGL for assemblies

Target: `frame_connected_30k` and `frame_structures_100k` at least twice as fast as OCCT,
and `frame_mesh_10m` limited only by the GPU.

1. Instancing. A connected presentation (`AisConnectedInteractive`, assembly
   occurrences) draws one resident mesh with an instance buffer of transforms.
   - The normal matrix handles non-uniform scale; mirrored transforms flip the winding.
   - There is one draw per mesh and pipeline, with an instance count or through multi-draw.
   - Memory grows with unique parts, not occurrences: Oblikovati's 30,000-placement
     fixture has 2,815 unique meshes.
2. GPU culling. A compute pass tests instance bounds against the frustum and a
   projected-size threshold, and writes compacted indirect commands and their count.
   - The CPU structure BVH stays for structure-level culling and for picking.
3. Occlusion culling. Two-phase Hi-Z culling:
   - first draw what was visible last frame;
   - then build a depth pyramid and test the rest against it.

   This is where dense assemblies (engines, cabinets) gain most.
4. Level of detail. Discrete meshes per part from coarser tessellation deflections,
   chosen by screen-space error, after the small-feature cull.
5. Meshlets. `VK_EXT_mesh_shader` (available on RADV), with frustum, cone and
   back-face culling per meshlet.
   - Behind a capability check, with multi-draw as the fallback.
6. Compact vertices.
   - Positions quantized to part bounds and octahedral normals.
   - 16-bit indices for small meshes.
   - A material index instead of per-vertex material.

   These change precision, so they get a new behaviour version and reviewed baselines.
7. Parallel CPU work.
   - Secondary command buffers recorded per layer or bucket on worker threads.
   - Uploads on a transfer queue.
   - Streaming, so a large assembly shows its first frame early and fills in.

Acceptance:

- The three targets above.
- On this host, a 30,000-placement assembly orbits at 120 frames per second or more.

## Phase 6: ray tracing (later)

- One BLAS per unique mesh, with refit and compaction.
- A TLAS with the real instance transforms.
- Accumulation and tone mapping on the GPU.

These are the Oblikovati lessons: there, ray tracing re-tessellates every placement each
frame and round-trips each sample through the CPU.

## Order and expected effect

```text
phase 0 ──► phase 1 ──► phase 2 ──┬──► phase 3 ──┐
                                  └──► phase 4 ──┴──► phase 5 ──► phase 6
```

| After | Cases expected at parity |
| --- | --- |
| Phase 1 | mesh, segments, points, AIS spheres, upload (5) |
| Phase 2 | + empty, dump, resize, MSAA (9) |
| Phases 3 and 4 | + structures, transparent, text, highlight (all 13 slower cases) |
| Phase 5 | the assembly cases faster than OCCT |

Phases 3 and 4 do not depend on each other and can run in parallel lanes. Each phase is
one or more pull requests to the standing release candidate branch, and each pull request
reports its harness table.

## Outside Martensite

OCCT on Linux does not turn vertical sync off: `OpenGl_Context::SetSwapInterval` passes 0
to `glXSwapIntervalSGI`, which rejects it (README, "Vertical sync in OCCT"). The fix is to
call `glXSwapIntervalEXT` for every value, not only -1. It can be reported upstream.
