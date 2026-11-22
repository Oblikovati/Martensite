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

No behaviour change. Status: done (#594). The README section "Phase 0 baseline" is the
table that phase 1 starts from.

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

   The README ("Profiling") gives both procedures and the first `perf` profile. The
   setting needs root; the maintainer lowered it to 1 on the measuring host.

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

Phase 1 also takes items 2 and 3 of phase 2. With the geometry on the device, the mesh,
point and segment frames spend about 0.1 ms on the CPU. The rest of their time is the
wait for the GPU and the readback of the image. OCCT draws ten frames and waits once, so
parity needs a redraw that returns without waiting and a capture that reads on demand.
Both are done.

Before batching, the 400-sphere orbit took 1.204 ms per frame on the Radeon 8060S
host, against 0.167 ms for OCCT. At two fixed orbit angles, replaying the same
extracted frame took 0.396 to 0.406 ms. Reusing the prepared GPU bindings as well
took 0.245 to 0.270 ms.

At those angles, all 400 draws shared the same transform and lighting inputs apart
from their geometry streams. A probe concatenated the vertices, normals and indices
in draw order and used the existing directional Phong shader. The resulting draw
kept all 496,800 triangles and exactly matched every color and depth byte. It took
0.103 to 0.110 ms per frame after the merged geometry was prepared.

A moving-orbit probe built the batch once and reused its geometry for the next 47
redraws. Color and depth matched the unbatched renderer at four sampled angles.
A live material edit made the batch ineligible. Hiding the edited object rebuilt
the batch, and both edits matched the unbatched images. Building the live scene
and checking the assembled draws still took 1.061 ms per frame. The full probe
took 1.204 ms, including 0.003 ms for GPU draw preparation.

Refreshing the shared transform and lights while holding the scene inputs fixed took
0.108 ms for the moving orbit. A retained implementation must validate live inputs
before lowering the draws, preserve mutable ranges and resource lifetimes, and
replace the corresponding per-draw device allocations when admitting a batch.

On that scene, a scan that captured fresh buffer stamps and copied the layer
priority maps took about 81 µs in a short diagnostic. Checking the saved order in
the existing maps and comparing saved geometry stamps took 32 µs. The saved-stamp
check rejected direct layout writes and index or vertex edits. This timing covers
buffer and order checks with representative structure, group and aspect reads.
This measurement preceded the checks for live material values, visibility,
layer settings and clipping.

Warm culling took 42 µs on its own. Borrowing the BVH and reducing the per-leaf
locking brought it to 29 µs in a probe that increased peak process memory in the
sphere and text comparisons. The two traversals leave little room in the earlier
0.175 ms parity budget. Reusing the culling traversal for the live batch check can
avoid reading the same structures twice.

A fused probe encoded each original geometry stream into staging ranges and retained
the resulting vertex and index buffers after submission. The ordinary and streamed
sphere orbits took 1.287 and 0.181 ms per frame on the same host, with OCCT at
0.187 ms. Peak process memory through the timed windows was 1.0% lower, and the
whole-process peak rose 0.02% (32 KiB). Color and depth matched at four camera angles
and after material and visibility edits forced ordinary rendering. An abandoned
upload kept its source streams available for retry.

The native redraw path now batches compatible opaque directional Phong draws.
The ordinary frame supplies the draw order and geometry streams. Batch admission
checks the layer span and the recorded vertex, index, normal and material inputs.
On warm redraws, culling and the live input checks decide whether to reuse those
buffers. A changed input uses the ordinary path. Mutable buffers keep the existing
partial upload path. Batch buffers share the device geometry budget with the
ordinary cache, and submitted frames keep their buffer owners until retirement.

The native sphere check matched ordinary Vulkan color and depth at four camera
angles. It also matched ordinary rendering after material, visibility, background,
priority and connection edits, during immediate redraw, and after a group was
destroyed. Warm orbit frames created zero buffers and uploaded zero geometry
bytes. The TkService and Vulkan suites and doctests passed 33,871 tests,
with 40 ignored. That total includes 30,366 passes across the other 125
registered Vulkan integration targets and three test-enabled examples.
Default all-targets Clippy passed for both crates.

The equal-window comparison on the Radeon 8060S used the same executable for
the default and batch-disabled Vulkan runs. Each renderer used ten frames per
orbit window. The upload setting timed six display, redraw and removal
operations per window. The first attempt timed only two uploads, and its GPU
wait varied enough to put the case outside the tolerance. The six-operation
rerun passed in all five pairs.

| Case | Ordinary Vulkan (ms) | Default Vulkan (ms) | OCCT (ms) | Default peak RSS vs ordinary |
| --- | ---: | ---: | ---: | ---: |
| `frame_ais_spheres_400` | 1.253 | 0.171 | 0.182 | −0.31% |
| `frame_mesh_1m` | 0.305 | 0.301 | 0.306 | −0.05% |
| `frame_segments_1m` | 0.237 | 0.247 | 0.270 | +0.04% |
| `frame_points_1m` | 0.270 | 0.270 | 0.283 | +0.01% |
| `upload_mesh_100k` | 0.141 | 0.136 | 0.277 | −0.22% |

The rendering counts matched in every run. The median for each case was below
OCCT's. The point and segment scenes went through the same single-draw path in
both Vulkan modes. Their peak RSS ranges overlapped.

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

Error timing, decided by the maintainer on 2026-11-22: an ordinary redraw does not wait
for its GPU work. Errors that Vulkan returns at once (acquire, submit, present,
validation) stay immediate. An error that the GPU reports later, for example a lost
device, shows at the next `capture`, at the next redraw that reuses the frame slot, or
at an explicit wait. `capture` reads the frame back on demand and returns the same pixels
as before.

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
