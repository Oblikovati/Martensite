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

## Validation admission

Validated startup checks required extensions in the global and selected-layer
catalogs, and refuses a validation layer older than 1.4.313, before instance
creation. See [validation instance admission](../validation-instance-admission/README.md)
for typed failure names and the remaining ARM64 runtime requirement.

## Unsafe code

The crate calls the Vulkan API through `ash`, so `backend` opts in to `unsafe` code under the
exception recorded in `AGENTS.md`. Every block has a `SAFETY` comment; clippy enforces it.

## Verification

Run the suite on Mesa lavapipe with the pinned validation layer. Every validated process must
report no error, warning or performance warning:

```bash
source ~/.local/share/martensite/env.sh
export FONTCONFIG_FILE="$PWD/scripts/install/renderer-fontconfig.conf" \
    VK_DRIVER_FILES="$MARTENSITE_LAVAPIPE_ICD" \
    GALLIUM_OVERRIDE_CPU_CAPS=sse2 LP_NATIVE_VECTOR_WIDTH=128 LIBGL_ALWAYS_SOFTWARE=1 \
    MARTENSITE_VULKAN_DEVICE=0
cargo nextest run -p martensite-vulkan
```

`scripts/install/vulkan-validation.sh` provides the layer and the driver, and `install-linux.sh`
runs it on Ubuntu 24.04. CI uses `.github/actions/setup-renderer-tests`. The recorded software
images are exact only on the pinned Mesa 26.1.6 lavapipe with LLVM 20.1.2 and the SSE2, 128-bit
rasterizer settings above. Other Mesa versions differ by a few levels at some pixels, and report
4 instead of 8 maximum samples, so these tests fail there.

Text in the recorded images comes from the system fonts that `install-linux.sh` installs.
`renderer-fontconfig.conf` limits the tests to system fonts, because a font in a user's own folder
can change which one an alias resolves to: a local Helvetica replaces FreeSans for "Arial", and the
text tests then fail.

Lavapipe's output also differs between CPU vendors at some pixels, for the same driver build and
settings, and the recorded images come from AMD CPUs. `tests/support/reference_cpu.rs` keeps the
affected image checks exact on AMD. On other vendors a mismatch prints a notice instead of failing,
and the rest of each test still applies. A CI job can land on either vendor, so only runs on AMD
exercise those checks.

The `*_regressions` test targets run regression cases in isolated worker processes under their
group's time budget and grade the case report. Cases that need boolean, fillet, offset or
feature toolkits are not here yet because those toolkits are not in this repository.

## Assets

`tests/fixtures` holds recorded reference images and depth buffers (about 130 MB). `shaders`
holds 36 MB, mostly generated SPIR-V variants embedded with `include_bytes!`. The scripts
that generated the variants are not in this repository, so a new variant is compiled from its
source with `glslangValidator`.
