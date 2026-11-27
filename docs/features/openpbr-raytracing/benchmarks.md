# OpenPBR Surface v1.1 & Vulkan Ray Tracing Benchmarks

Measured on Linux x86_64 (`AMD Ryzen AI Max+ 395 w/ Radeon 8060S`, Mesa RADV 26.1.6 `STRIX_HALO` with `VK_KHR_ray_query` + `VK_KHR_acceleration_structure`, and Mesa lavapipe LLVM 20.1.2 256-bit) against **Blender 4.0.2** (`EEVEE` OpenGL GPU and `Cycles` CPU Embree 32-thread) on two equivalent scenes with exact triangle parity.

## Reproduction

```bash
# Martensite release benchmark (AMD RADV)
cargo run --release -p martensite-vulkan --example openpbr_raytrace_bench

# Martensite release benchmark (Mesa lavapipe CPU fallback)
VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json \
  cargo run --release -p martensite-vulkan --example openpbr_raytrace_bench
```

## Test Scenes & Methodology

Both renderers run on the same machine with 45-degree perspective camera, `max_bounces = 4`, AgX display transform, and persistent acceleration structures (`BlasCache` in Martensite; `use_persistent_data = True` in Blender, `write_still = False`).

1. **`car_paint_torus` (2,592 triangles exact in both):** Single CAD torus (`R = 2.0`, `r = 0.65`) with metallic clear-coat car paint (`base_color = (0.85, 0.12, 0.15)`, `metalness = 0.85`, `roughness = 0.25`, `coat_weight = 1.0`, `coat_roughness = 0.04`).
2. **`showcase_5mat` (6,476 triangles exact in both):** Five CAD primitives over a neutral ground plane: car-paint torus, brushed gold metal sphere (`metalness = 1.0`, `roughness = 0.18`), solid refractive glass sphere (`IOR = 1.52`, `transmission = 1.0`, `roughness = 0.03`), matte terracotta cylinder (`roughness = 0.75`), and microfiber-sheen velvet sphere (`fuzz_weight = 0.9`, `fuzz_roughness = 0.45`).

Methodology notes:
- Martensite path-tracing `16 spp` times measure 16 accumulated frames (`view.redraw()`) followed by a single host readback (`capture_color`), matching Blender's `16 samples` single-frame render call.
- System Blender 4.0.2 ships with Cycles CPU (Embree, 32 threads) and EEVEE OpenGL GPU; no ROCm/HIP Cycles runtime is installed on the host. Comparing Martensite against both Blender EEVEE (same 8060S GPU) and Blender Cycles CPU (both GPU-vs-CPU and lavapipe-CPU-vs-Cycles-CPU) keeps the comparison transparent.
- Blender's `bpy.ops.render.render(write_still=False)` includes Cycles/EEVEE render-layer compositing and color management; Martensite renders directly to its Vulkan swapchain/offscreen image with GPU AgX tone mapping.

## Scene 1: `car_paint_torus` (2,592 triangles, 4 bounces, AgX)

| Renderer & Backend | 512x512 (1 spp) | 512x512 (16 spp) | 1024x1024 (1 spp) | 1024x1024 (16 spp) | Est. Throughput (1024x1024) |
|---|---:|---:|---:|---:|---:|
| Martensite OpenPBR Raster (RADV 8060S) | 0.24 ms | - | 1.33 ms | - | - |
| Blender 4.0.2 EEVEE (OpenGL 8060S) | 6.61 ms | 9.20 ms (TAA) | 12.12 ms | - | - |
| Martensite PT HW `VK_KHR_ray_query` Raw (RADV 8060S) | 0.81 ms | 12.99 ms | 1.49 ms | 23.79 ms | 2,115.7 Mrays/s |
| Martensite PT HW `VK_KHR_ray_query` + 3-pass SVGF (RADV 8060S) | 1.37 ms | 21.96 ms | 2.71 ms | 43.37 ms | 1,160.6 Mrays/s |
| Martensite PT SW BVH Raw (RADV 8060S) | 1.66 ms | 26.52 ms | 3.33 ms | 53.33 ms | 943.7 Mrays/s |
| Martensite PT SW BVH + 3-pass SVGF (RADV 8060S) | 2.12 ms | 34.00 ms | 4.78 ms | 76.51 ms | 657.9 Mrays/s |
| Martensite PT HW-emulated Raw (Mesa lavapipe CPU) | 9.67 ms | 154.70 ms | - | - | 81.3 Mrays/s (512x512) |
| Martensite PT SW BVH Raw (Mesa lavapipe CPU) | 13.99 ms | 223.87 ms | - | - | 56.2 Mrays/s (512x512) |
| Blender 4.0.2 Cycles CPU (Embree 32T, Raw) | 9.71 ms | 117.17 ms | 44.68 ms | 570.68 ms | - |

## Scene 2: `showcase_5mat` (6,476 triangles, 4 bounces, AgX)

| Renderer & Backend | 512x512 (1 spp) | 512x512 (16 spp) | 1024x1024 (1 spp) | 1024x1024 (16 spp) | Est. Throughput (1024x1024) |
|---|---:|---:|---:|---:|---:|
| Martensite OpenPBR Raster (RADV 8060S) | 0.17 ms | - | 1.36 ms | - | - |
| Blender 4.0.2 EEVEE (OpenGL 8060S) | 7.67 ms | 11.24 ms (TAA) | 13.22 ms | - | - |
| Martensite PT HW `VK_KHR_ray_query` Raw (RADV 8060S) | 2.01 ms | 32.14 ms | 3.08 ms | 49.28 ms | 1,021.4 Mrays/s |
| Martensite PT HW `VK_KHR_ray_query` + 3-pass SVGF (RADV 8060S) | 2.51 ms | 40.09 ms | 4.30 ms | 68.79 ms | 731.6 Mrays/s |
| Martensite PT SW BVH Raw (RADV 8060S) | 3.32 ms | 53.06 ms | 5.06 ms | 80.94 ms | 621.8 Mrays/s |
| Martensite PT SW BVH + 3-pass SVGF (RADV 8060S) | 2.99 ms | 47.84 ms | 6.15 ms | 98.43 ms | 511.3 Mrays/s |
| Martensite PT HW-emulated Raw (Mesa lavapipe CPU) | 8.54 ms | 136.63 ms | - | - | 92.1 Mrays/s (512x512) |
| Martensite PT SW BVH Raw (Mesa lavapipe CPU) | 13.37 ms | 213.96 ms | - | - | 58.8 Mrays/s (512x512) |
| Blender 4.0.2 Cycles CPU (Embree 32T, Raw) | 19.27 ms | 348.66 ms | 97.04 ms | 1,264.20 ms | - |

## Standalone GPU HDR Tone Mapping (`1024x1024`, AgX)

- AMD Radeon 8060S (RADV): **5.88 ms/frame** (including host `f32 -> f16` staging upload + RGBA8 readback)
- Mesa lavapipe (LLVM 20.1.2, 256-bit): **14.23 ms/frame**

## Summary

- **Hardware RT speedup over software BVH:** `VK_KHR_ray_query` runs **2.24x faster** on `car_paint_torus` (`1.49 ms` vs `3.33 ms` at `1024x1024`) and **1.64x faster** on `showcase_5mat` (`3.08 ms` vs `5.06 ms` at `1024x1024`).
- **SVGF denoiser overhead:** The 3-pass spatiotemporal + 3-iteration A-trous denoiser adds **0.50-0.56 ms** at `512x512` and **1.22 ms** at `1024x1024` on RADV, sustaining over **230 FPS** denoised path tracing at `1024x1024` on the 5-material CAD showcase.
- **Comparison against Blender 4.0.2:**
  - Real-time GPU rasterization: Martensite OpenPBR Vulkan (`1.33-1.36 ms` at `1024x1024`) runs **9.1x-9.7x faster** than Blender EEVEE (`12.12-13.22 ms`), reflecting a direct forward CAD pipeline without EEVEE's screen-space reflection and probe passes.
  - Path tracing (GPU vs Cycles CPU): Martensite HW `VK_KHR_ray_query` (`23.79-49.28 ms` for 16 spp at `1024x1024`) runs **24.0x-25.6x faster** than Blender Cycles CPU (`570.68-1,264.20 ms`).
  - Path tracing (CPU vs CPU at `512x512`, 16 spp): Martensite on Mesa lavapipe (`136.63-154.70 ms`) is **2.55x faster** than Cycles CPU on the 5-material glass/velvet showcase (`348.66 ms`) and within **1.32x** on the single car-paint torus (`117.17 ms`).
