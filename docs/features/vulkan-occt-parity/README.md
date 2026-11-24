# Vulkan renderer performance against OCCT's OpenGL renderer

`martensite-vulkan` replaces OCCT's `TKOpenGl` behind the same `V3d`/`Graphic3d` driver
interfaces. This note measures the two renderers on the same scenes, the way
[tkernel-occt-parity](../tkernel-occt-parity/README.md) measures `tkernel` against
`TKernel`. The improvement plan is in [PLAN.md](PLAN.md); the work is tracked by #594.

Reviewed tree: `release/next` at `172f5b1f` (`martensite-vulkan` 0.7.2). OCCT:
`master` at `4dbfdcc4` (V8_0_1-88), built with the Visualization module, `-O3 -DNDEBUG`.

## Result

The Vulkan renderer is slower than OpenGL in 13 of 14 cases, by 8 to 1,550 times. Only
start-up is faster. A scene that OpenGL redraws in a third of a millisecond takes Vulkan
between 90 and 230 ms, so an interactive assembly view is not possible today.

| Case | Scene | OpenGL ms | Vulkan ms | Ratio |
| --- | --- | ---: | ---: | ---: |
| `highlight_ais_spheres_400` | two hover detections with dynamic highlight | 0.112 | 174.0 | 1,553 |
| `frame_segments_1m` | 1,000,000 line segments | 0.254 | 230.2 | 908 |
| `frame_mesh_1m` | one 1,000,000-triangle mesh | 0.317 | 175.7 | 555 |
| `frame_ais_spheres_400` | 400 shaded `AIS_Shape` spheres, 496,800 triangles | 0.167 | 86.4 | 517 |
| `frame_points_1m` | 1,000,000 coloured points | 0.285 | 128.3 | 451 |
| `frame_text_1k` | 1,000 text labels | 0.724 | 138.8 | 192 |
| `frame_structures_10k` | 10,000 boxes, one structure each | 2.146 | 387.8 | 181 |
| `upload_mesh_100k` | display, draw and remove a 102,000-triangle structure | 0.313 | 29.8 | 95 |
| `frame_transparent_1k` | 1,000 half-transparent boxes | 0.312 | 27.4 | 88 |
| `frame_msaa4_100k` | 102,000 triangles, 4 samples per pixel | 0.241 | 17.2 | 71 |
| `dump_1080p` | `ToPixMap` at 1920 x 1080 | 0.434 | 23.6 | 54 |
| `resize_redraw` | resize the window and redraw | 0.415 | 17.8 | 43 |
| `frame_empty` | background only | 0.071 | 0.582 | 8.2 |
| `startup_first_frame` | window, driver, view and first frame | 33.3 | 16.8 | 0.51 |

Times are per operation: one frame, or one detection pair, upload, dump, resize or
start-up. Both sides report the same checksum in every case: the structure, element,
triangle, line and point counts of the renderer's own frame statistics, read from one
counted frame at the start position. They draw the same scenes, including the
tessellation of the 400 spheres by each kernel (496,800 triangles on both sides).

## Phase 0 baseline

The phase 0 run adds the assembly cases and the device work of each Rust case. It used the
settings of the result above (10 frames per timed run, median of 7 runs, one round, CPUs
8-15), and all 19 checksums agree. Phase 1 compares against this table.

| Case | OpenGL ms | Vulkan ms | Ratio |
| --- | ---: | ---: | ---: |
| `frame_empty` | 0.062 | 0.551 | 8.92 |
| `frame_mesh_1m` | 0.309 | 175 | 568 |
| `frame_mesh_10m` | 1.77 | 1849 | 1,042 |
| `frame_segments_1m` | 0.269 | 230 | 853 |
| `frame_points_1m` | 0.264 | 121 | 461 |
| `frame_text_1k` | 0.691 | 141 | 205 |
| `frame_transparent_1k` | 0.311 | 27.7 | 89 |
| `frame_msaa4_100k` | 0.236 | 17.8 | 75 |
| `frame_structures_10k` | 2.24 | 377 | 168 |
| `frame_structures_100k` | 43.8 | 4870 | 111 |
| `frame_ais_spheres_400` | 0.161 | 86.6 | 536 |
| `frame_connected_30k` | 9.83 | 2007 | 204 |
| `highlight_ais_spheres_400` | 0.106 | 174 | 1,642 |
| `select_10k` | 4.14 | 386 | 93 |
| `close_10k` | 9.58 | 894 | 93 |
| `upload_mesh_100k` | 0.308 | 29.7 | 96 |
| `dump_1080p` | 0.436 | 22.7 | 52 |
| `resize_redraw` | 0.391 | 17.5 | 45 |
| `startup_first_frame` | 33.3 | 13.6 | 0.41 |

The device work of one Vulkan operation, from the work column. "Outside phases" is redraw
time that no phase holds; the release of the frame's buffers and images happens there.

| Case | Buffers | Sets | Submits | Upload MiB | Scene ms | Upload ms | Record ms | Wait ms | Outside phases ms |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `frame_empty` | 2 | 0 | 1 | 0.0 | 0.023 | 0.005 | 0.009 | 0.152 | 0.049 |
| `frame_mesh_1m` | 5 | 1 | 1 | 32.5 | 148 | 21.4 | 0.075 | 0.977 | 3.94 |
| `frame_mesh_10m` | 5 | 1 | 1 | 324.5 | 1584 | 236 | 0.086 | 4.24 | 23.3 |
| `frame_segments_1m` | 4 | 0 | 1 | 68.7 | 164 | 60.0 | 0.073 | 1.25 | 4.15 |
| `frame_points_1m` | 5 | 1 | 1 | 34.3 | 88.9 | 30.3 | 0.076 | 1.08 | 0.390 |
| `frame_text_1k` | 3,002 | 1,000 | 1,001 | 10.4 | 32.2 | 62.5 | 0.433 | 28.9 | 8.91 |
| `frame_transparent_1k` | 3,002 | 1,000 | 1 | 1.5 | 8.33 | 9.53 | 0.191 | 0.279 | 8.01 |
| `frame_msaa4_100k` | 5 | 1 | 1 | 3.3 | 14.6 | 1.55 | 0.052 | 0.801 | 0.205 |
| `frame_structures_10k` | 30,002 | 10,000 | 1 | 15.1 | 89.6 | 107 | 18.0 | 2.48 | 149 |
| `frame_structures_100k` | 300,002 | 100,000 | 1 | 151.1 | 1017 | 952 | 1138 | 6.54 | 1574 |
| `frame_ais_spheres_400` | 1,202 | 400 | 1 | 16.6 | 72.2 | 10.1 | 0.127 | 0.488 | 2.81 |
| `frame_connected_30k` | 90,002 | 30,000 | 1 | 226.9 | 963 | 404 | 124 | 12.9 | 470 |
| `highlight_ais_spheres_400` | 2,408 | 800 | 2 | 33.2 | 145 | 20.4 | 0.247 | 1.05 | 5.66 |
| `select_10k` | 30,004 | 10,000 | 1 | 15.1 | 91.4 | 109 | 17.8 | 2.54 | 151 |
| `upload_mesh_100k` | 5 | 1 | 1 | 3.3 | 14.2 | 1.36 | 0.043 | 0.735 | 0.180 |
| `dump_1080p` | 5 | 1 | 1 | 3.3 | 14.5 | 1.76 | 0.058 | 0.603 | 0.233 |
| `resize_redraw` | 5 | 1 | 1 | 3.3 | 14.3 | 1.32 | 0.039 | 1.04 | 0.086 |

The counters show causes that the first run could not separate:

- Text makes one image and one submission per label on every frame: 1,002 images and 1,001
  submissions per frame of `frame_text_1k`, with 29 ms spent waiting on them.
- Each draw creates three buffers and one descriptor set on every frame.
- Recording grows faster than the draw count: 18 ms for 10,000 structures, 1,138 ms for
  100,000.
- A selection change uploads the whole scene again: 15 MiB and 30,004 buffers per
  operation of `select_10k`.
- Removing 10,000 structures takes 894 ms, against 9.6 ms for OpenGL, and closing the view
  of `frame_structures_100k` takes about 84 s.

## Method

The harness pairs a Rust bench target of `martensite-vulkan` with C++ twins that build the
same scenes through OCCT's `V3d` on `TKOpenGl`
([harness](#harness)). Both programs:

- draw into a mapped, unmanaged 1280 x 720 X11 window on the host's display, created the
  same way (`Xw_Window`/`XwWindow` with override-redirect);
- turn vertical sync off and light the view with the viewer's default lights;
- orbit an orthographic camera one degree per frame, so that every redraw renders the
  whole scene again;
- end each timed run only when the GPU has finished: OpenGL calls `glFinish`, and a
  Vulkan redraw already waits for its frame;
- report the median of 7 timed runs.

The run above used 10 frames (operations) per timed run, one round, both programs pinned
to CPUs 8-15. The machine was not idle (load average about 11 from other work), which
adds noise to both sides; it cannot explain ratios of 40 to 1,500. A first run with the
default 100 frames per timed run agreed on every checksum but was stopped before its
table was printed, because the Vulkan side needed about 16 minutes per round.

Host: AMD Radeon 8060S (Strix Halo, integrated, 16 GB), Mesa 26.1.6 (RADV for Vulkan,
radeonsi for OpenGL 4.6), Linux 7.2.15, X display through Xwayland.

### Vertical sync in OCCT

`OpenGl_Caps::swapInterval = 0` does not turn vertical sync off under GLX.
`OpenGl_Context::SetSwapInterval` passes values other than -1 to `glXSwapIntervalSGI`, which
rejects 0, so OCCT stayed at 60 Hz (16.67 ms for an empty frame). The C++ twins call
`glXSwapIntervalEXT(display, window, 0)` on each window, as OCCT intends; the empty frame
then takes 0.063 ms, the same as with Mesa's `vblank_mode=0`. The defect is in OCCT: an
OCCT application on Linux cannot turn vertical sync off this way and stays at the display rate.

## Causes

`perf` cannot open events on this host, so the profiles below are `callgrind` instruction
counts of the smoke runs (`valgrind --tool=callgrind ... --filter <case>`). Time spent
waiting for the GPU is not in them. The scaling across the cases gives the same picture:

| Cost | Vulkan | OpenGL | From |
| --- | ---: | ---: | --- |
| Fixed cost of a frame | about 0.5 ms | about 0.07 ms | `frame_empty` |
| Each draw (structure or group array) | 27-39 us | about 0.2 us | `frame_structures_10k`, `frame_transparent_1k` |
| Each vertex, every frame | 115-350 ns | 0 (resident on the GPU) | `frame_mesh_1m`, `frame_segments_1m`, `frame_points_1m` |
| Each text label, every frame | about 140 us | about 0.7 us | `frame_text_1k` |
| A hover highlight | a full redraw (about 87 ms) | about 56 us | `highlight_ais_spheres_400` against `frame_ais_spheres_400` |

They follow from how a Vulkan frame is built. Every redraw:

1. Extracts the scene again. `view_scene::frame_for_view` walks the layers and builds a
   new `RenderFrame` whose `PrimitiveDraw`s each own a `Vec<RenderVertex>` and a
   `Vec<u32>` copied from the `Graphic3d` arrays, with the CPU lowering (line guard bands,
   wide lines, flat sources, text quads) done again.
2. Uploads all of it again. `FrameResources::new` (`backend/frame.rs`) encodes every
   draw to bytes and creates new vertex and index buffers for it (`frame_draw_upload.rs`),
   and new colour, depth and readback targets for the frame.
3. Waits and reads the frame back. `render_current` returns a `FrameCapture`: the
   submission waits on its fence, then copies the colour and depth images to host memory
   (`FrameResources::capture`), also for a window frame. Render scale, subview composition
   and flipped output are then applied to that host copy.
4. Has no immediate layer. `redraw_immediate` calls the same `try_redraw`, so a hover
   highlight renders the whole scene again.

OpenGL does none of this per frame: `OpenGl_PrimitiveArray` keeps its vertex buffers on
the GPU until the array changes, the structure BVH culls on the CPU, state changes are
cached in `OpenGl_Workspace`, nothing is read back unless asked, and `redrawImmediate`
draws the immediate layers over a copy of the last main-layer image.

Nothing in the crate retains geometry between frames today. Only textures have retained
caches (`retained_*_texture*`).

### Profiles

`frame_mesh_1m`, about 1.28 billion instructions per frame:

| Phase | Share of a redraw | Where |
| --- | ---: | --- |
| Scene extraction and lowering | 75% | `frame_for_view` → `Graphic3dDrawGroup::append_geometry` → `PrimitiveSubmission::draws` |
| of which: reading indices one by one through a trait object | 37% | `PrimitiveSubmission::read_indices` → `Graphic3dIndexBufferMembers::index` (about 110 instructions per index) |
| of which: positions, normals, lighting | 35% | `PrimitiveSubmission::{positions, normals}`, `retained_lighting::apply_surface_lighting` |
| Encoding and upload | 25% | `FrameSetup::upload` → `encode_vertex_sources` (each `f32` through an iterator chain into a new `Vec<u8>`) |

`frame_structures_10k`, about 1.56 billion instructions per frame (156,000 per structure):
extraction and lowering per group take 0.87 billion, encoding and upload (two new buffers
per draw) 0.36 billion, and `malloc`, `free` and `memcpy` together about 9% of the
program. Removing the 10,000 structures when the view closes took 3.9 billion
instructions, more than three frames: `Graphic3dRenderDriver::remove_structure` scans a
`Vec<WeakHandle>` with `retain` for every removal, so closing or reloading a scene is
quadratic in its structure count.

## Geometry residency measurements

The paired runs used the same AMD Radeon 8060S, OCCT installation, CPUs 8–15 and
10-frame setting as Phase 0. Scene checksums agreed. All times are milliseconds per
operation.

| Case | Phase 0 Vulkan | With retained geometry | Paired OCCT |
| --- | ---: | ---: | ---: |
| `frame_mesh_1m` | 175 | 1.118 | 0.309 |
| `frame_segments_1m` | 230 | 1.075 | 0.273 |
| `frame_points_1m` | 121 | 1.012 | 0.301 |
| `frame_ais_spheres_400` | 86.6 | 4.158 | 0.228 |
| `upload_mesh_100k` | 29.7 | 0.861 | 0.490 |

The four orbit cases created zero geometry buffers and uploaded zero geometry bytes in
their timed frames. The presentation case included its initial upload and then reused
the same source array across new groups. In an intermediate run, checking the array
and reading its indices took 90% of sampled user-space CPU cycles. The presentation
case took 16.9 ms then. It fell to 0.861 ms after index validation began scanning
the bytes under one read lock and reusing the result for the same revision and extent.

The million-segment scene step fell from 9.51 ms to 0.05 ms after the guard-band
check began using the retained bounds. The bounds enclose all projected segments
under the same float operations as the clipping path. When that enclosure cannot
prove the whole draw stays below the clipping threshold, the renderer checks each
segment. In the 400-sphere case, sharing descriptor pools reduced the upload step
from 3.10 ms to 0.57 ms.

Peak process RSS was lower in each paired measurement, with reductions from 0.5%
to 21%. The million-triangle, line and point frames still spend 0.8–0.9 ms on fence
waits and readback. Phase 2 addresses that cost. The 400-sphere frame spends 1.89 ms on
scene extraction, which Phase 3 addresses with retained draw records.

The live-view tests cover static orbiting, sampler and UV edits, and a mutable
unlit triangle. Editing one vertex in that triangle uploaded 32 bytes. The
renderer copied the unchanged vertex records on the GPU and reused the index buffer.

## Attachment reuse and text residency measurements

Two live views of different sizes reuse their Vulkan attachments after warmup without
creating images. The `frame_empty` benchmark still copies 7,200 KiB to the host each
frame. On the same host and CPUs 8–15, three alternating 28-frame runs measured
0.581 ms before attachment pooling, 0.765 ms with pooling alone, and 0.519 ms after
choosing the depth format once per capture. The capture tests check that both depth
formats retain the scalar decoder's bits.

The 1,000-label case used the same release harness in three alternating 8-frame runs
with matching checksums. Median Vulkan time fell from 149.53 ms to 5.17 ms and peak RSS
from 156 MiB to 122 MiB. OCCT took 0.73 ms. The warm Vulkan frame creates zero glyph
images and makes one submission, down from 1,000 images and 1,001 submissions.

Retaining the glyph vertex, index and UV streams removed a further 2,000 buffer
creations and 718.75 KiB of uploads per warm frame. A new set of three alternating runs
with matching checksums measured 5.37 ms before that change and 4.07 ms after, with peak
RSS 1,000 KiB lower. OCCT took 0.72 ms. A live orbit test compares the complete RGBA
and depth against the same view using an uncached formatter.

On the startup case, the same executable took 45.59 ms with an empty Martensite
pipeline cache and 28.92 ms with a warm one. These are medians of seven paired process
launches with Mesa's own shader cache disabled. Each launch successfully rendered its
first frame. The driver supplied the cache bytes, and the live-device test verifies
that a new device with the same cache identity can reload them.

## Lighting binding measurements

In the stock sphere orbit, all 400 lit draws in a frame use the same packed
lighting bytes. Materials and transforms still participate in that byte
comparison. A bounded cache retains one candidate per frame and shares it across
matching draws. Other inputs use the existing frame storage allocation.

Five pairs with reuse enabled and disabled in the same release executable
measured 3.01 ms without reuse and 2.84 ms with it. Peak RSS was 153680 and
153640 KiB, respectively. Descriptor allocation fell from 400 to about one
per frame. Three pairs of the transparent 1,000-draw scene measured 5.87 ms
without reuse and 5.42 ms with it, with peak RSS 121620 and 121740 KiB. All
paired scene checksums matched.

## Camera snapshot measurements

Vulkan checks whether a placement's world-view matrix matches the camera bit for bit
and whether the camera has unit axial scale. It reads the two values under one lock
using `Graphic3dCamera::orientation_with_axial_scale_f`. It previously copied the
whole camera to make the same check.

Against `983bbd2a` on the same host and CPUs 8–15, the connected 30,000-part
frame improved from 170.035 to 162.471 ms across seven alternating pairs. Scene
extraction fell from 110.353 to 105.715 ms and peak process RSS from 310,364 to
310,068 KiB. This comparison used the Vulkan benchmark on both sides. A separate
10-frame run against OCCT measured 167.082 ms for Vulkan and 11.408 ms for OCCT,
so the assembly parity gate remains open.

## Immediate redraw measurements

The first immediate redraw establishes the main color and depth at the boundary
before overlays. Warm immediate redraws reuse those images and prepare only the
overlays. The attachment pool owns the retained images and can reclaim them when
it reaches its existing budget. A view with a recycled ticket renders the main
pass again. Native unlit and shaded AIS tests compare restored RGBA and depth
against a full redraw.

Five alternating release runs on CPUs 8–15, with 18 operations each, reduced the
400-sphere hover pair from 6.82 ms to 4.15 ms and median peak RSS from 155176 to
154420 KiB. Ordinary 400-sphere orbiting took 3.38 ms before and 3.34 ms after.
Each workload kept its checksum. The hover pair now allocates zero descriptor
sets in warm redraws, down from 800. It still walks the scene twice, reads back
14,400 KiB and uploads 2.28 KiB of overlay geometry. OCCT took 0.11 ms per hover
pair in the separate paired 8-operation run.

At 1280 × 720, the source and destination attachments total 16,039,952 bytes in
a warm immediate redraw. Three same-binary comparisons against full redraws
reported the same driver estimate of 69,427,200 bytes in use and the same ten
native memory allocations for the whole scene. This is a result for the measured
host and extent.

The stock unlit wire highlights now send their color in the draw's push
constants. Warm hover and selection both reuse the underlying vertex and index
buffers. The live shader check compares color and depth against the same draw
with its highlight color baked into the vertices.

Five paired 64-frame runs removed four geometry-buffer allocations and 2.28 KiB
of uploads from each warm hover pair. Median time was 4.24 ms before and 4.23 ms
after, with about 4% fewer user-space instructions over the process. In three
paired four-frame `select_10k` runs, geometry uploads fell from 864 bytes to zero
and median time fell from 81.43 ms to 80.30 ms. Both cases kept their checksums
and peak RSS was within 1 MiB in every hover pair and within 2 MiB in every
selection pair.

## Immediate layer elision

Phase 4 (PLAN.md) makes an immediate redraw skip the main layers while its view has had
no main-layer notice and its in-place state still matches. The redraw then culls, builds
and uploads only the immediate layers, restores the kept main colour and depth, and draws
the immediate layers over them. Before, each hover built all 400 main draws and compared
them with the kept pass, which cost about 2.5 million instructions per hover pair. OCCT's
`redrawImmediate` spends about 65,000.

The same profile showed five more per-frame costs, which this phase removes:

| Cost | Before | After |
| --- | --- | --- |
| Geometry retirement after each frame | 512 entries checked | twice the frame's uploads, in a frame that restores the main image |
| Device limits and OIT formats | format queries in each frame | read once per device |
| Presentable-object accessors in picking | all fields and the presentation list copied, 236 times per hover pair | one field read under the lock |
| Camera inversion in a selecting frustum | both matrices inverted for each of 10 points | inverted once |
| Frustum box test | 21 million instructions | 15 million (OCCT: 11 million) |

`cargo xtask occt-parity --toolkit vulkan --cpus 8-15`, same host and OCCT build:

| Run | Vulkan | OCCT | Ratio |
| --- | ---: | ---: | ---: |
| Before phase 4, `--frames 10` | 0.609 ms | 0.102 ms | 5.96 |
| `--frames 10` | 0.141 ms | 0.112 ms | 1.26 |
| `--frames 100`, highlight case only | 0.160 ms | 0.122 ms | 1.31 |

Each hover pair keeps its checksum and uploads no geometry. The two redraws take about
0.07 ms together; an empty Vulkan frame takes about 0.033 ms on this host. The rest of the
gap is picking in `martensite-tkv3d`, which runs about 270,000 instructions per hover pair
against OCCT's 170,000.

The other cases kept their time within run-to-run noise. `upload_mesh_100k` varies
between 0.11 and 0.77 ms from one run to the next on this host.

## Frames in flight and capture on demand

A redraw now submits its frame and returns. Up to two frames run on the GPU while the host
prepares the next one, and a frame waits only for the fence of the frame two before it.
`capture` reads the last frame back the first time it is called, so a redraw copies nothing
to host memory. The maintainer chose this error timing on 2026-11-22 (PLAN.md, phase 2):
a GPU fault that comes after the redraw returned shows at the next `capture`, at the next
redraw that needs the frame's slot, or at `Graphic3dRenderDriver::finish_frames`. The bench
ends each timed run with `finish_frames`, as the C++ twin ends it with `glFinish`.

One run of each program per case, 10 frames, CPUs 8-15, same session and OCCT build;
milliseconds per operation:

| Case | Vulkan before | Vulkan after | OCCT | Ratio before | Ratio after |
| --- | ---: | ---: | ---: | ---: | ---: |
| `close_10k` | 47.872 | 58.660 | 9.525 | 5.09 | 6.16 |
| `dump_1080p` | 6.751 | 6.001 | 0.431 | 7.23 | 13.93 |
| `frame_ais_spheres_400` | 3.520 | 2.293 | 0.177 | 21.01 | 12.96 |
| `frame_connected_30k` | 170.905 | 174.868 | 9.021 | 19.76 | 19.38 |
| `frame_empty` | 0.498 | 0.088 | 0.073 | 6.73 | 1.21 |
| `frame_mesh_10m` | 3.944 | 3.597 | 1.877 | 2.07 | 1.92 |
| `frame_mesh_1m` | 0.906 | 0.377 | 0.310 | 2.89 | 1.22 |
| `frame_msaa4_100k` | 0.656 | 0.176 | 0.245 | 2.65 | 0.72 |
| `frame_points_1m` | 0.753 | 0.313 | 0.300 | 2.68 | 1.04 |
| `frame_segments_1m` | 0.897 | 0.361 | 0.272 | 3.14 | 1.33 |
| `frame_structures_100k` | 696.322 | 695.898 | 35.782 | 17.43 | 19.45 |
| `frame_structures_10k` | 59.851 | 56.449 | 2.032 | 28.15 | 27.77 |
| `frame_text_1k` | 3.586 | 2.585 | 0.743 | 4.86 | 3.48 |
| `frame_transparent_1k` | 5.027 | 3.892 | 0.326 | 15.73 | 11.94 |
| `highlight_ais_spheres_400` | 4.928 | 3.652 | 0.099 | 44.58 | 36.98 |
| `resize_redraw` | 1.289 | 1.003 | 0.424 | 3.21 | 2.37 |
| `select_10k` | 77.672 | 78.119 | 4.055 | 22.75 | 19.26 |
| `startup_first_frame` | 15.921 | 25.631 | 33.528 | 0.48 | 0.76 |
| `upload_mesh_100k` | 1.702 | 0.961 | 0.310 | 3.38 | 3.10 |

The `close_10k` and `startup_first_frame` rows come from a run before the residency fix
below. Paired runs of the same binaries after it, alternating the order:
`close_10k` takes the same time as before with less peak memory, and the first frame goes
from 27.2 to 24.7 ms. `frame_structures_10k` keeps its time but peaks 7.5 MB higher
(232.8 to 240.4 MB, 3.3%), because the frame in flight keeps its 10,000 draw records.
Dropping those records at launch brings the peak back to 232.6 MB in a throwaway test, so
the next change keeps only the GPU owners of a launched frame.

A submitted frame shares its uploads with later frames before its fence signals. The
residency store still retires as many entries at settle as the frame uploaded, shared
uploads included; without that count the budget fell to its floor of 512 and `close_10k`
took 23% longer with 50 MB more.

The mesh, point and segment frames now spend about 0.1 ms on the CPU and wait for the GPU
for the rest. Points reach parity. The 10-million-triangle frame still takes 1.9 times as
long as OCCT, so the GPU work per triangle is the next limit for the mesh and segment cases.
Each lit vertex has 44 bytes against OCCT's 24, because the uniform colour is in every
vertex. The 400-sphere case is bound by the CPU (1.2 ms of scene extraction and 0.4 ms of
upload per frame).

A view's first two redraws each create one set of attachments and one upload page, one for
each frame in flight; later redraws create none.

## Phase 2 window and readback measurements

`cargo xtask occt-parity --toolkit vulkan --cpus 8-15 --rounds 3`, one case per run, with
the default 100 frames:

| Case | Before phase 2 | After | OCCT | Ratio |
| --- | ---: | ---: | ---: | ---: |
| `frame_empty` | 0.116 ms | 0.052 ms | 0.068 ms | 0.76 |
| `frame_msaa4_100k` | 0.144 ms | 0.116 ms | 0.315 ms | 0.37 |
| `dump_1080p` | 12.8 ms | 0.456 ms | 0.436 ms | 1.05 |
| `resize_redraw` | 5.7 ms | 1.08 ms | 0.51 ms | 2.10 |

The "before" column is the phase 1 executable on CPU 14. Peak memory stayed the same or
fell in every case.

A minimal Vulkan program that only acquires, clears and presents set the floor for the
window cases. On the benchmark's override-redirect window, XWayland releases presented
images late, so with the minimum image count plus one every acquire waited about 20 µs.
One more image without vertical sync removed the wait; FIFO keeps the old count.

The other gains came from X server requests that OCCT's redraw does not make or that cost
more here. Mesa's WSI threads share the application's X connection, so a synchronous
request from the main thread also waits for them. The surface queries, the window size and
mapped state, and the window size that auto Z-fit reads through `MinMaxValues` (16 µs per
frame on CPUs 8-15) now come from values the view keeps between resizes. The host no
longer waits on an acquire fence either.

A colour dump reads only the colour attachment and reuses its buffers. An RGB dump packs
the rows with a compute pass (`shaders/rgb_pack.comp`) and copies them straight into the
image, which halves the dump time again after the CPU conversion hit memory limits.

Resize stays outside parity on X11 (see PLAN.md). Recreating the swapchain alone costs more
than OCCT's whole resize.

## Phase 3 draw submission measurements

Phase 3 keeps what a frame lowered and uploaded, and reuses it while its inputs are
unchanged ([retained-render-list.md](retained-render-list.md)):

- A group keeps its draws, and a structure keeps its program, while the group's commands,
  aspects, transform, camera parts and pass are unchanged.
- A text label keeps its glyph draws while its text revision, aspects revision, font and
  resolution are unchanged.
- A plain surface draw keeps its pipeline and its resident geometry, and binds again only
  the bindings that follow the view.
- Layers of 256 structures or more, and runs of 256 scene text labels or more, replay on
  `rayon` worker threads. Any structure or label that must lower again lowers on the
  calling thread, so lowering and user callbacks stay on the view's thread.
- Culling borrows the layer's BVH tree and updates each structure under one lock.

The `draw-record-verification` feature runs the lowering path beside each reuse and fails
on any difference. All library tests and suites pass with and without it.

`cargo xtask occt-parity --toolkit vulkan --cpus 8-15 --frames 10`, one round, against the
same OCCT build. "Phase 3 start" is the base of this work on the same host:

| Case | Phase 3 start | Vulkan | OCCT | Ratio |
| --- | ---: | ---: | ---: | ---: |
| `frame_structures_10k` | 1.674 ms | 1.811 ms | 1.911 ms | 0.95 |
| `frame_text_1k` | 2.029 ms | 0.675 ms | 0.739 ms | 0.91 |
| `frame_transparent_1k` | 3.02 ms | 0.654 ms | 0.329 ms | 1.99 |

Against the phase 3 start, `select_10k` went from 49.179 to 14.349 ms and
`highlight_ais_spheres_400` from 1.867 to 0.562 ms. The other cases kept their
time within run-to-run noise. Peak memory grew by 6 to 7% in the text, transparent and
selection cases, which are 69 to 78% faster, and fell by 6 to 14% in the large structure
cases.

`frame_transparent_1k` stays outside parity. Its frame costs about 40 ns per draw in each
of culling, assembly, upload and settling, and no single step can close the gap. Reusing a
whole unchanged frame would close it, but needs a change counter over every scene input.
The maintainer chose on 2026-11-23 not to build it
([global-change-tracking.md](global-change-tracking.md)).

## Harness

`cargo xtask occt-parity --toolkit vulkan --occt <prefix> --cpus 8-15` compares the cases
of `crates/martensite-vulkan/benches/occt_parity/` with their C++ twins in its `occt/`
directory. It works like the `tkernel` harness (build both, run them in turn, compare
medians and checksums, write `target/occt-parity/vulkan/report.json`, fail when a Rust
case is slower beyond `--tolerance`), with these differences:

- `<prefix>` must be an OCCT install with the Visualization module (`TKService`,
  `TKV3d`, `TKOpenGl`). The twins also link Xlib and GLX.
- Both programs need an X11 display and a GPU. The Rust bench compiles on every platform
  but runs only on Linux.
- `--frames <n>` sets the frames (operations) of one timed run; the default is 100, and 10
  is enough while the gaps are this large.
- The C++ program is linked with an RPATH (`--disable-new-dtags`) because OCCT's
  libraries carry no run path: a RUNPATH would serve only the program's direct
  dependencies, and `TKV3d` could not find `TKGeomAlgo`.
- The Rust program adds a fourth column to each case line: the device work of the timed
  operations, per operation (next section). The xtask prints it as a second table and
  stores it in `report.json` as `rust_work`; the C++ twins print no such column.

Without `--bench`, each program draws two frames per case, which checks that the cases
still work. The cases are:

| Group | Cases |
| --- | --- |
| Start-up | `startup_first_frame` |
| Frames (`frames.rs`) | `frame_empty`, `frame_mesh_1m`, `frame_structures_10k`, `frame_segments_1m`, `frame_points_1m`, `frame_text_1k`, `frame_transparent_1k`, `frame_msaa4_100k`, `frame_structures_100k`, `frame_mesh_10m` |
| Interactive context (`ais.rs`) | `frame_ais_spheres_400`, `highlight_ais_spheres_400` (the scene of OCCT's `perf/vis/bug24623_1` at 20 x 20), `select_10k`, `frame_connected_30k` |
| Transfers (`transfer.rs`) | `upload_mesh_100k`, `dump_1080p`, `resize_redraw`, `close_10k` |

The assembly cases of phase 0 ([PLAN.md](PLAN.md)):

| Case | Scene and operation |
| --- | --- |
| `frame_structures_100k` | 100,000 boxes on a 400 x 250 grid, one structure each: the per-draw cost at assembly scale |
| `frame_mesh_10m` | one 10,000,000-triangle sphere: vertex throughput |
| `frame_connected_30k` | 30,000 `AIS_ConnectedInteractive` occurrences of 30 parts (10 boxes, 10 cylinders, 10 cones) on a 200 x 150 grid, shaded and not selectable; 4,420,000 triangles |
| `select_10k` | 10,000 selectable shaded `AIS_Shape` boxes; one operation selects the other of two boxes and updates the viewer |
| `close_10k` | one operation erases, removes and releases the 10,000 box structures of `frame_structures_10k`; building and drawing them is not timed |

The three orbit cases time a tenth of `--frames` per run, because a Vulkan frame of these
scenes takes seconds. `close_10k` builds its scene again before each timed run
(`timed_rebuilds`), so its removed count, and with it its checksum, grows with the number of
runs; both sides count the same.

## Device work counters

`Graphic3dRenderView::last_frame_work()` returns the `RenderDeviceWork` of the view's last
redraw, and `Graphic3dRenderDriver::device_work()` returns the totals of the device since it
was created. The counts come from the backend sites that create or move something:

| Counter | Counted where |
| --- | --- |
| `buffers_created` | `DeviceBuffer::new`, the only `vkCreateBuffer` call |
| `images_created` | `bind_device_image`, which binds every image right after it is created |
| `memory_allocations` | the memory arena: new shared blocks and dedicated allocations |
| `pipelines_created` | the one `vkCreateGraphicsPipelines` call |
| `descriptor_sets_allocated` | each of the 13 `vkAllocateDescriptorSets` calls |
| `submissions` | the one `vkQueueSubmit2` call |
| `uploaded_bytes`, `read_back_bytes` | `DeviceBuffer::write` and `DeviceBuffer::read`, the only host copies |

`times` splits the CPU time of redraws into phases that exclude each other: a submission
made while frame resources are prepared counts as submission, not as upload.

| Phase | What it holds |
| --- | --- |
| `scene` | walking the layers into the frame's draws: reading the `Graphic3d` arrays and the CPU lowering, which are one pass today |
| `upload` | creating and filling the frame's buffers, images and descriptor sets |
| `recording` | recording command buffers |
| `submission` | ending and submitting command buffers, and presenting |
| `waiting` | the fence of the oldest frame in flight when the flight is full, and swapchain image acquisition |
| `readback` | copying a frame to host memory when `capture` first asks for it, and the host-copy steps (render-scale depth blit, subview blits, flipped output); a redraw itself reads nothing back |
| `redraw` | the whole redraw; the time in no phase (frame statistics, resource release) is the difference |

The counts are device-wide differences over a redraw, so a redraw that another thread runs
on another view of the same driver at the same time adds to them. Injected backends count
nothing.

## Profiling

`perf record` needs `kernel.perf_event_paranoid` at 1 or lower. At 2, the default here,
`perf stat` works but `perf record` cannot open its events. To lower it until the next
boot:

```sh
sudo sysctl kernel.perf_event_paranoid=1
```

Then profile one case of the release bench (`cargo bench -p martensite-vulkan --bench
occt_parity --no-run` prints the executable):

```sh
perf record -g -o target/occt-parity/vulkan/frame_mesh_1m.perf -- \
  target/release/deps/occt_parity-<hash> --bench --frames 10 --filter frame_mesh_1m
perf report -i target/occt-parity/vulkan/frame_mesh_1m.perf
```

On this host, a `perf` profile of `frame_mesh_1m` (`--frames 2`, the whole program with its
set-up) puts 33% of the samples in `Graphic3dIndexBufferMembers::index` called through
`dyn`, 13% in `Graphic3dBufferMembers::read_at::<f32>` and 9% in the `read_indices` loop
around them. Reading the arrays one element at a time is the first target of phase 1 (bulk
accessors).

Without `perf`, `callgrind` counts instructions, without the time spent waiting for the GPU:

```sh
valgrind --tool=callgrind --callgrind-out-file=target/occt-parity/vulkan/frame_mesh_1m.cg \
  target/release/deps/occt_parity-<hash> --filter frame_mesh_1m
callgrind_annotate target/occt-parity/vulkan/frame_mesh_1m.cg
```

## Presentation thread

A window redraw now records its frame and hands it to a presentation thread, one per
renderer, which submits and presents it with the same calls the redraw made before. The
maintainer accepted on 2026-11-24 (PLAN.md, phase 2) that a submit or present error of such
a frame shows at the next redraw, `finish_frames`, capture or close. Acquire errors stay
immediate. Frames with GUI layers, window depth status or ray-tracing feedback are still
submitted and presented by the redraw itself.

Mesa's `vkQueuePresentKHR` made 11 `ioctl` calls per frame and `vkQueueSubmit2` 4, all
on the redraw thread; radeonsi makes the matching calls on its own `cs0` thread. Alternating
runs of 300 frames of `highlight_ais_spheres_400`, CPUs 8-15, median µs per hover pair:

| Host load | Before | Present call only on the thread | Submit and present on the thread | OCCT |
| --- | ---: | ---: | ---: | ---: |
| 2 to 4 | 130 | 121 | 107 | 113 |
| 11 to 13 | 166 | | 129 | 130 |
