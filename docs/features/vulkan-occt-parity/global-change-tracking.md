# Global change tracking (phase 3, option b)

Status: decided on 2026-11-23. The maintainer chose option C. Nothing in this note is
implemented; it records the measurements and the options for a later decision.

## Problem

After the parallel scene assembly, `frame_text_1k` takes 0.62 ms (OCCT: 0.74 ms), but
`frame_transparent_1k` still takes 0.59 ms against OCCT's 0.37 ms. Its frame is spread over
many small per-draw costs, each about 40 ns per draw:

| Step | Time | Main costs |
| --- | ---: | --- |
| Scene | 0.21 ms | culling update (10% of the frame), layer assembly, capping pass |
| Upload | 0.17 ms | lighting bind and encoding (12%), push constants (5%) |
| Settle | about 0.07 ms | GPU owner lists (8%), geometry retirement checks |
| Record | 0.06 ms | draw commands |

Removing any one of these saves at most 0.03 ms. The benchmark redraws a scene that does not
change, and every frame repeats the culling, the assembly and the uploads of the frame before.
OCCT does the same work in its own frame, but each of its steps costs less.

## Idea

A frame whose inputs are all unchanged since the previous frame of the same view reuses that
frame's culling result, layer programs and uploaded draws, and records its command buffer again.
The expected static frame is record, submit and settle: about 0.15 ms.

The hard part is the proof that nothing changed. Today only aspects and texts have a write
revision (`RevisedState`). The draw records prove each group by comparing its inputs, which is
O(draws) per frame. A frame-level proof must be O(1): one counter that advances on every write
that can change a frame.

## What the counter must cover

- About 86 state cells in `martensite-tkservice/src/graphic3d` (`SharedState`, `Mutex`,
  `RwLock`, atomics): structures, groups, layers, cameras, lights, clip planes, textures, shader
  programs, view settings and others.
- Buffers that change in place (vertex and index arrays), and the Vulkan driver's own view
  state.
- Inputs that change with time (media textures, animated shaders): a frame that reads one of
  them never reuses.
- The window size, the frame extent and the sample count, which the frame key holds directly.

## Options

### A. One process-wide edit epoch

Each graphic3d state cell becomes a `RevisedState`, and each ended write also advances one
process-wide `AtomicU64`. A view keeps the epoch it last drew with, and reuses its frame when
the epoch and its own frame key are unchanged.

- Complete by construction for every wrapped cell. A test fails on a graphic3d state cell that
  is not wrapped.
- Conservative: an edit anywhere (another view, another document) invalidates every view. The
  result is always correct, only slower.
- It conflicts with the rule "keep no thread-local or global mutable algorithm state". The epoch
  holds no algorithm state, and a reused frame must equal a fresh one, so results do not depend
  on it. But it is global mutable state, and it needs a recorded exception.

### B. One epoch per scene, linked from each cell

Each cell holds a link to the epoch of the scene that owns it. Shared objects (one aspects
object used by structures of two viewers) need a link per owner, kept up to date as objects
attach and detach.

- Follows the rule.
- Much larger, and a missed attach or detach gives a stale frame. That class of error is hard
  to find.

### C. No frame reuse

Keep `frame_transparent_1k` at 0.59 ms (1.6 times OCCT), record the gap, and move to phase 4.

## Verification (A or B)

- Under `draw-record-verification`, every reused frame is also built fresh, and its layer
  programs and uploaded draws must be equal, as for the other retained levels.
- All library tests and suites run in both modes.
- A regression test edits each kind of input between two frames and requires a fresh frame.

## Cost and versioning

Option A changes about 40 files in `martensite-tkservice`, adds public API (`revision()`
accessors and the epoch) and needs a minor version of that crate, which needs approval.
