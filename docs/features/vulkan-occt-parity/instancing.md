# Phase 5, step 1: instanced connected presentations

Status: steps 1, 1b, 3a to 3d and the retained placement path in step 4 are implemented.
Step 2 was dropped. Part of phase 5 in
[PLAN.md](PLAN.md) (#594).

## Measurement

`frame_connected_30k` draws 30 parts under 30,000 placements (`AisConnectedInteractive`).
On this host it takes about 154 ms per orbit frame in Rust and 10.2 ms in OCCT (15.2x).
The work counters per frame:

| Phase | ms |
| --- | ---: |
| Scene (culling, lowering, frame build) | 91 to 97 |
| Upload | 25 to 34 |
| Recording | 4 to 8 |

- The frame uploads 11,250 KiB, which is 384 bytes of draw constants for each of the
  30,000 draws, and creates about 43 buffers and 44 descriptor sets. It uploads no geometry.
- A perf profile of 10 frames puts about 60% of the samples in `append_structure`, and
  most of those in `lower_in_structure`. A group that is lowered twice in one pass is marked
  `RecordSlot::Instanced` (graphic_group_record.rs) and keeps no record, because a record
  is keyed on the exact model matrix. So every placement lowers its part's groups again on
  every frame.
- About 7% of the samples were in `update_culling`. A later check showed that the layer BVH
  is built once, at the first frame: the share came from that build in a 15-frame profile.
- OCCT draws each placement's groups with one `glDrawElements` each and sets the placement
  transform as a uniform. It makes 30,000 draw calls per frame at about 330 ns each.

The phase 5 target is twice OCCT's speed, about 5 ms per frame. That leaves about 170 ns
per placement for culling, frame build, upload and recording together, so per-draw
constants and per-draw recording do not fit. The frame needs real instancing.

## Steps

1. **Instance records.** A group keeps one record in its own space. The key holds the model
   matrix's class instead of its bits: the sign of its determinant (mirroring) and whether it
   is the identity (the implicit model stack). A replay under another placement computes
   every field that depends on the model again: the clip transform, the projection factors,
   the lighting world and normal matrices, `mirrored`, and the model stack it leaves for
   the next group. `RecordCamera::certifies` grows into a check of all of them, and the
   `draw-record-verification` feature proves each replay against a fresh lowering. This
   removes the lowering, which is most of the scene time.
2. **Layer BVH (dropped).** The layer already keeps its BVH until a structure is added,
   removed or moved, as OCCT's `OpenGl_BVHClipPrimitiveSet` does. The culling cost that
   remains is the traversal and the culled flags, about 5% of the frame.
3. **Draw tables (ordered indirect draws).** Decided by the maintainer on 2026-11-24: the draw
   order stays the same, so that captures stay identical to the bit (option 2 of the review;
   sorting the draws by part, then instancing them, could change pixels where two opaque
   surfaces have the same depth). A run of consecutive plain lit surface draws that differ only
   in their placement and geometry range records as one `vkCmdDrawIndexedIndirect`. Each draw
   of the run reads the matrices it would push or bind from a per-frame table at its first
   instance. Sub-steps:
   - **3a, shader.** `lighting.vert` reads its clip, world and normal matrices through
     `DRAW_*_FROM_OBJECT` macros. With `MARTENSITE_DRAW_TABLE` (`draw_table.glsl`) they read
     the storage buffer at set 7, entry `gl_InstanceIndex`; without it they expand to the push
     constant and the lighting block as before. Compiling the old and the new source with 98
     define sets gives identical bytes for every set that compiles, so no committed module
     changes. `lighting.draw-table.vert.spv` is the plain precomposed twin; fixed lighting,
     factorized projection, markers and lines are refused at compile time.
   - **3b, pipeline.** A `draw_table` flag of `PipelineVariant` builds the twin: the same
     fragment stage, the twin vertex stage when the selected one is the plain `lighting.vert`,
     and the table layout at set 7 (free: window depth status, its other user, is refused).
     The device enables `multiDrawIndirect` and `drawIndirectFirstInstance` when present;
     without them no draw takes a table.
   - **3c, runs at upload.** After the draws of a frame upload, consecutive kept uploads of the
     plain pipeline whose recording state is equal (pipeline, bindings, buffers, presentation,
     stencil plan entries) form a run. The run writes its entries (the push-constant clip matrix
     and the lighting block's world and normal matrices) and its indirect commands to the frame
     pages, and its draws share one lighting block without placement. A run of one draw stays
     an ordinary draw.
   - **3d, record.** The record loop records the first draw of a run with its state commands and
     one indirect draw for the run, and skips the other draws of the run.
   - **3e, frame objects.** Measure, then remove the per-draw objects that remain (the uploaded
     draw records, the scene's replayed draws) for the draws of a run.
4. **Measure, then continue with the plan.** GPU culling (plan item 2) reuses the per-frame
   placement buffer and writes the indirect commands.

Each step keeps its own tests: unit tests for the key and the placement fields, the
existing capture baselines unchanged, and a new capture of a mirrored and a scaled
placement against a fresh lowering.

## Results

| State | ms per frame | Scene share of samples |
| --- | ---: | ---: |
| Before step 1 | 154 | about 60% |
| Step 1: instance records | 65 | about 45% |
| Step 1b: direct replay of connected structures | 34 | about 10% |
| Step 3: draw tables, shared lighting blocks | 22 | about 35% |

Step 1 is as described above. The lighting world and normal matrices moved out of the shared
`LightingBody` into `RenderLighting`, so that a replay sets them without a copy of the body.
A record keeps the model class (`mirrored`, implicit stack). The model stack that a group leaves
for the next group (`FlipCarry::ordinary`) is computed for each placement, and a record is kept
only when the lowered stack equals it. The `draw-record-verification` run of the integration
targets passes.

Step 1b: `replayed_groups` also serves a connected structure. It replays the groups of its
references first, deepest first, then its own groups, in the order and with the model stack of
`CappingScene::draws`.

The frame is now about 4 times slower than OCCT's 9.7 ms. The profile of the frame:
per-draw upload of constants 14%, command recording 7%, frame launch and release 7%, culling 5%.
These are per-draw costs, which step 3 removes.

Step 3: runs form for draws of the plain lit pipeline (`lighting.vert`, precomposed projection,
no clipping, OIT, window depth or line state) of at least 8 consecutive draws with equal
recording state. A kept draw whose lighting block differs from the previous one only in the
world and normal matrices borrows that binding; a borrowed draw that no run draws binds its
own block again (`repair_borrowed_lighting`), so every draw outside a run records as before.
On this host the frame records in 0.14 ms instead of 5 ms, uploads 6.2 MB instead of
11.2 MB, allocates 9 descriptor sets instead of 44 and keeps the image checksum. The unit test
`a_run_captures_the_bytes_its_draws_capture_one_by_one` compares a run with the same draws
recorded one by one, byte for byte, and the full suite passes with runs active.

The harness, on a busy host: `frame_connected_30k` 22.9 ms against OCCT's 13.7 ms (OCCT
measured 9.7 to 13.7 ms in this session), `frame_structures_100k` 43.1 ms against 39.0 ms
(scene 42.5 ms), `frame_mesh_10m` 1.81 ms against 1.71 ms. The remaining per-frame costs of
`frame_connected_30k`: culling about 5 ms (marking each of the 30,000 structures culled or
not), the replay of the scene about 3 ms, the per-draw upload about 7 ms.

### After step 3

`frame_structures_100k` first reached approximately 20.5 ms against OCCT's 42 ms.
Group records and fused batches now subscribe to the sources they read. They reuse the
validated input while its scoped revision holds.
`frame_connected_30k` is then 20.4 ms against OCCT's 9.5 ms. Every remaining cost is per
placement and memory bound: marking 30,000 structures culled or not (about 5 ms), the replay of
30,000 group records into frame draws (model product, normal matrix, draw copy), the upload of
30,000 kept draws and the release of the frame's per-draw objects. Twice OCCT's speed leaves
about 160 ns per placement for all of it.

## Step 4: retained placement frames

A view retains the draw templates and their world and normal matrices when the main layer can
be drawn as one ordered indirect run. On a camera-only redraw, it builds one frame draw from
the saved placements and recomputes their clip matrices and the camera-dependent lighting.
The CPU still checks the structure BVH and updates the culled flags and structure statistics.
A retained draw range maps each structure to the placements it covers. On a cropped frame,
the GPU rejects placements that the CPU excluded and checks the remaining mesh bounds
against the frustum. Within each segment the compute pass writes surviving commands and
their count in the original order. Single-sample frames can also omit a subpixel box when
conservative bounds show that it cannot reach a pixel center. A device without the optional
compute path takes the full scene route when cropping changes the visible set. Changes to the view, layer notices, connections, and revised graphic inputs can
also invalidate the batch. Before the seed frame, the view subscribes to its layers,
structures, connections, groups, aspects, buffers, and per-view affinity masks. A write or
retirement advances the revisions of its subscribers. Other views keep their own revisions.

The native check establishes that a warm batch was used before each edit. It compares the
fallback and ordinary frames by RGBA bytes and depth bits after transform, material, color,
per-view visibility, and erase changes. It also zooms in and out to check culling transitions
and visible structure counts.

```sh
MARTENSITE_594_VERIFY_PLACEMENT_EDITS=1 cargo bench --locked -p martensite-vulkan \
  --bench occt_parity -- --filter frame_connected_30k --frames 30
```

### Native measurements

Three paired runs on Linux x86-64 with Radeon 8060S graphics (RADV), X11, VSync off,
and both renderers pinned to CPUs 8–15. These measurements used the worktree based on
`997afd39` after scoped invalidation and before GPU visibility and secondary command recording.
The connected case timed 30 redraws per sample. The other cases timed three redraws per
sample. The primitive-count checksums matched OCCT in all three cases.

| Case | Vulkan median (ms) | OCCT median (ms) |
| --- | ---: | ---: |
| `frame_connected_30k` | 1.54 | 8.78 |
| `frame_structures_100k` | 15.69 | 39.71 |
| `frame_mesh_10m` | 1.99 | 1.54 |

The connected case averaged roughly 650 redraws per second from the median frame time.
Its redraw builds the scene in 0.03 ms and uploads roughly 6 MiB of draw-table and command
entries. The 100,000-structure case builds the scene in 14.93 ms. When the BVH root passes
the fully-inside test and size culling is disabled, the layer marks every leaf reached
directly. The same benchmark before that change spent 20.20 ms building the scene.

The mesh benchmark reported 1.92 ms of GPU wait per operation, 0.01 ms of CPU scene time,
and no warm geometry upload or readback. The parity harness still reports the mesh case
as slower than OCCT.

The TKService, TKV3D, and Vulkan library and toolkit suites ran 5,082 tests. Of those,
5,080 passed and two Vulkan graduated-trihedron image checks failed. Both failures also
occurred on the clean baseline and reported the same image checksums and pixel deviations.
Nextest skipped 31 tests. All seven active graduated-trihedron tests passed when rerun
with the repository's `scripts/install/renderer-fontconfig.conf`, which the build driver
sets for renderer tests.
