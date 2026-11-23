# Retained render list (phase 3, item 4)

Maintainer decision, 2026-11-23: start the retained render list to bring
`frame_transparent_1k` and `frame_text_1k` to OCCT parity.

## Problem

After the phase 3 records, a transparent frame of 1,000 boxes costs about
1.4 µs of CPU time per draw, and OCCT's `TKOpenGl` costs 0.37 µs. The frame
counters split it into 0.9 ms of scene assembly, 0.4 ms of upload and 0.09 ms
of command recording. No step is large: each draw goes through a structure
snapshot, the capping scene, a group record check, a new `PrimitiveDraw`,
about twenty upload admission checks, a lighting encoding, a geometry lookup
and a new `UploadedDraw`. OCCT keeps its GPU objects and only sets uniforms
and issues the draw.

To reach parity, a draw that did not change may cost about 0.3 µs:

| Step | Budget per draw |
| --- | ---: |
| Check that the structure and group did not change | 0.10 µs |
| Put the camera fields on the kept draw | 0.03 µs |
| Write the per-frame data (push constants, lighting block) | 0.07 µs |
| Record the commands | 0.09 µs |

## Design

The render list is a set of validated caches, one per level. The existing
lowering path stays the only producer of draws: a cache keeps what the
lowering made, and a frame that finds the cache valid skips the lowering
and the per-frame work of that level. There is no second renderer beside
the first, so the plan's rule that a retained path replaces the per-frame
path holds per level.

1. **Structure records.** `Graphic3dRenderStructure` keeps the program
   that its groups gave last time, with the structure inputs it was built
   from. A structure whose inputs and group records are unchanged appends
   the kept draws with the camera fields of this frame. This skips the
   structure snapshot, the capping scene and the per-group record inputs.
2. **Group and label records.** These exist (phase 3 items 2 and 5). A
   structure record asks each group record only whether its sources still
   hold.
3. **Retained uploads.** A draw body keeps the `UploadedDraw` that its last
   upload produced in each frame slot, with the inputs of that upload. A
   draw with the same body and inputs re-encodes only its push constants
   and its lighting block. This skips the admission checks, the pipeline
   selection, the geometry lookup and the binding preparation.
4. **Shared camera block, then multi-draw indirect.** The view, eye and
   lights are the same for every draw of a pass. Moving them into one
   per-frame block, and the per-draw data into a storage buffer, lets
   draws of one pipeline share one indirect draw. The shader arithmetic
   stays the same, so pixels do not change.

Each level certifies what it keeps, as the group records do: it keeps
nothing for a structure or draw outside the ordinary class (transform
persistence, clip planes, highlight, flipping, user programs, textures with
live contents), and those draws take the lowering path every frame.

## Verification

The `draw-record-verification` feature checks every level: on a hit it
also runs the lowering path and panics on any difference. The recorded
lavapipe images and the replayed requests must pass unchanged, with and
without the feature.

## Steps

Each step is a commit with its measurements:

1. Structure records.
2. Retained uploads.
3. The shared camera block.
4. Multi-draw indirect for the ordinary pipelines.

Acceptance is the phase 3 acceptance: `frame_transparent_1k` and
`frame_text_1k` at parity, no other case slower, and peak memory within
the rule (up to 10% more only with at least a 50% speedup).
