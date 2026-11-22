# Vulkan renderer performance against OCCT's OpenGL renderer

`martensite-vulkan` replaces OCCT's `TKOpenGl` behind the same `V3d`/`Graphic3d` driver
interfaces. This note measures the two renderers on the same scenes, the way
[tkernel-occt-parity](../tkernel-occt-parity/README.md) measures `tkernel` against
`TKernel`. The improvement plan is in [PLAN.md](PLAN.md).

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

Without `--bench`, each program draws two frames per case, which checks that the cases
still work. The cases are:

| Group | Cases |
| --- | --- |
| Start-up | `startup_first_frame` |
| Frames (`frames.rs`) | `frame_empty`, `frame_mesh_1m`, `frame_structures_10k`, `frame_segments_1m`, `frame_points_1m`, `frame_text_1k`, `frame_transparent_1k`, `frame_msaa4_100k` |
| Interactive context (`ais.rs`) | `frame_ais_spheres_400`, `highlight_ais_spheres_400` (the scene of OCCT's `perf/vis/bug24623_1` at 20 x 20) |
| Transfers (`transfer.rs`) | `upload_mesh_100k`, `dump_1080p`, `resize_redraw` |
