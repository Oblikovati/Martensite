# Vulkan renderer

`martensite-vulkan` is the Vulkan 1.3 implementation of the graphic driver interfaces of
`martensite-tkservice`. It draws retained structures in windows and offscreen targets:
fixed and programmable lighting, PBR, order-independent transparency, shadow maps, text,
clipping and capping, stereo, ray and path tracing, and application shader programs.

| Area | Where |
| --- | --- |
| Driver, views, render layers, frame statistics and dumps | `driver.rs`, `render_view*`, `view_*`, `frame_stats.rs` |
| Presentation elements and text | `render_primitive_array.rs`, `render_text*`, `graduated_trihedron/` |
| Device, memory, pipelines, passes and submission | `backend/` |
| Ray and path tracing | `raytrace/`, `backend/raytrace_*` |
| Application shader programs | `user_program/` |
| Shader sources and their SPIR-V binaries | `shaders/` |

## Behaviour versions

Raster behaviours that can change output are public, selectable versions:
`RenderWorldClippingBehavior`, the line clipping, depth bias, thin line, thin axis, triangle
coverage, window depth and light dot behaviours, and the text and Phong interpolation
policies. Earlier versions stay selectable for replay. Tests pick one per worker process with
a `MARTENSITE_CASE_*` environment variable; the library never reads these variables.

## Unsafe code

The crate calls the Vulkan API through `ash`, so `backend` opts in to `unsafe` code under the
exception recorded in `AGENTS.md`. Every block has a `SAFETY` comment; clippy enforces it.

## Verification

Run the suite on Mesa lavapipe with the pinned validation layer. Every validated process must
report no error, warning or performance warning:

```bash
source ~/.local/share/martensite/env.sh
export VK_DRIVER_FILES=/usr/share/vulkan/icd.d/lvp_icd.json \
    GALLIUM_OVERRIDE_CPU_CAPS=sse2 LP_NATIVE_VECTOR_WIDTH=128 LIBGL_ALWAYS_SOFTWARE=1 \
    MARTENSITE_VULKAN_DEVICE=0
cargo nextest run -p martensite-vulkan
```

`scripts/install/vulkan-validation.sh` provides the layer, and `install-linux.sh` runs it on
Ubuntu 24.04. CI uses `.github/actions/setup-renderer-tests`. The recorded software images
need the SSE2, 128-bit rasterizer settings above.

The `*_regressions` test targets run regression cases in isolated worker processes under their
group's time budget and grade the case report. Cases that need boolean, fillet, offset or
feature toolkits are not here yet because those toolkits are not in this repository.

## Assets

`tests/fixtures` holds recorded reference images and depth buffers (about 130 MB). `shaders`
holds 36 MB, mostly generated SPIR-V variants embedded with `include_bytes!`. The scripts
that generated the variants are not in this repository, so a new variant is compiled from its
source with `glslangValidator`.
