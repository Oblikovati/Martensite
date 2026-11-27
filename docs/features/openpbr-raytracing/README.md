# OpenPBR Surface v1.1 and Vulkan hardware ray tracing

Martensite renders physically based materials with the Academy Software Foundation (ASWF) OpenPBR Surface v1.1 specification across both real-time rasterization and a unified Vulkan compute path tracer.

## Material model (`martensite-tkservice`)

`OpenPbrMaterial` in `crates/martensite-tkservice/src/graphic3d/openpbr_material.rs` implements all nine OpenPBR Surface v1.1 parameter groups:

1. `OpenPbrBase`: base weight, linear RGB color, diffuse roughness (energy-preserving Oren-Nayar / EON), and conductor metalness.
2. `OpenPbrSpecular`: dielectric specular weight, F0 tint color, GGX roughness, roughness anisotropy, and index of refraction (IOR).
3. `OpenPbrTransmission`: dielectric transmission weight, Beer-Lambert absorption color and depth, homogeneous volume scatter color and anisotropy, and chromatic dispersion scale with Abbe number.
4. `OpenPbrSubsurface`: subsurface scattering weight, color, mean free path radius, per-channel radius scale, and phase anisotropy.
5. `OpenPbrCoat`: clear-coat weight, Beer-Lambert transmittance tint color, GGX roughness and anisotropy, coat IOR, and internal-reflection coat darkening.
6. `OpenPbrFuzz`: Charlie microfiber sheen weight, color, and roughness.
7. `OpenPbrThinFilm`: Airy spectral thin-film iridescence weight, thickness in micrometers, and film IOR.
8. `OpenPbrEmission`: emission luminance and linear RGB color.
9. `OpenPbrGeometry`: opacity, thin-walled slab toggle, and normal/tangent scale factors.

Every field is validated by `openpbr_validate.rs` for finiteness and domain bounds, returning `Graphic3d_MaterialDefinitionError` with the offending value and expected range. All 26 `Graphic3dNameOfMaterial` presets map to calibrated OpenPBR v1.1 parameters in `material_specs.rs`. `Graphic3dTextureUnit` and `Graphic3dTextureSetBits` cover texture units 0 through 12 for base color, emissive, occlusion, normal, metallic-roughness, specular color, specular roughness, coat weight/roughness, coat normal, transmission, subsurface color, fuzz color, and anisotropy tangent maps.

## Unified GPU representation and LUTs (`martensite-vulkan`)

Rasterization and compute path tracing read the same 256-byte `std430` `OpenPbrGpuMaterial` struct (`crates/martensite-vulkan/src/openpbr_material.rs` and `crates/martensite-vulkan/shaders/openpbr/openpbr_types.glsl`). Active slab bitmasks (`lobe_flags`) allow shaders to skip disabled layers without branching overhead on unused terms.

`crates/martensite-vulkan/src/openpbr_luts.rs` generates four deterministic half-float lookup tables uploaded to descriptor set 4 (and bound at set 2 in compute):

- `openpbrGgxLut`: `64x64` `R16G16_SFLOAT` directional and hemispherical GGX albedo `(E, E_avg)` for Turquin / Kulla-Conty multi-scattering compensation.
- `openpbrDielectricLut`: `32x32x32` `R16G16_SFLOAT` 3D dielectric reflectance and transmittance over `(cos_theta, roughness, mapped_ior)`.
- `openpbrSheenLut`: `32x32` `R16_SFLOAT` Charlie microfiber directional albedo.
- `openpbrThinFilmLut`: `64x1` `R16G16B16A16_SFLOAT` CIE 1931 XYZ Fourier sensitivity table for Airy thin-film harmonics.

`openpbr_env_cdf.rs` constructs a 2D HDR environment Vose alias table for importance sampling, and `pbr_environment/bake.rs` bakes specular prefiltered cube maps into `R16G16B16A16_SFLOAT` HDR attachments alongside 9-band spherical-harmonic diffuse irradiance.

## Real-time rasterization and shader build (`xtask shaders`)

`shaders/openpbr/openpbr_raster.glsl` evaluates all nine OpenPBR slabs in physical stacking order (fuzz over coat, coat darkening Eq. 64-70, coat roughening Eq. 73, coat TIR IOR modulation Eq. 60/74, F82-tinted metallic GGX, Airy thin-film iridescence, dielectric specular over transmission/SSS/EON diffuse, and attenuated emission) with direct analytic lights, shadow maps, and HDR IBL.

`cargo xtask shaders` compiles all 246 SPIR-V shader targets (18 OpenPBR raster fragment families and 6 compute shaders) with `glslangValidator -V --target-env vulkan1.3`. `cargo xtask shaders --check` verifies that checked-in `.spv` binaries match the GLSL sources.

## Hardware ray tracing, compute path tracer, SVGF, and tone mapping

`crates/martensite-vulkan/src/backend/hw_raytrace/` probes `VK_KHR_deferred_host_operations`, `VK_KHR_acceleration_structure`, `VK_KHR_ray_query`, and Vulkan 1.2 `bufferDeviceAddress`. When supported, logical device creation enables these extensions and builds compacted BLAS handles (cached by geometry digest in `BlasCache` via `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR`) and per-frame TLAS instances.

`Graphic3dRayTracingBackend` selects the execution path:

- `AutoPreferHardware`: runs `pathtrace_hw.comp` (`rayQueryEXT`) on hardware RT devices and falls back to `pathtrace_sw.comp` (binary BVH SSBO traversal over the exact same `PtVertex`, `PtTriangle`, `OpenPbrGpuMaterial`, and `PtLight` buffers) on software or non-RT Vulkan devices such as Mesa lavapipe.
- `HardwareRayQuery`: requires `VK_KHR_ray_query` and returns `RenderPortabilityFailure::MissingDeviceFeature { feature: "VK_KHR_ray_query" }` when the device lacks it.
- `SoftwareComputeBvh`: forces the portable software-BVH compute shader on any Vulkan 1.3 device.

Both compute shaders include `shaders/openpbr/pathtrace_main.glsl` and `shaders/openpbr/openpbr_bsdf.glsl`, implementing VNDF GGX sampling, EON diffuse, Charlie fuzz, solid and thin-walled glass transmission with Beer-Lambert absorption, homogeneous volume scattering, Abbe chromatic dispersion, and bounded 16-step random-walk subsurface scattering.

Primary-hit G-buffers (demodulated radiance, albedo/roughness, normal/depth, screen-space motion vectors, material ID, and emission/background) feed a 3-pass compute SVGF denoiser when `denoiser_enabled` is true:

1. `svgf_reproject.comp`: temporal reprojection with ID, normal, and depth disocclusion checks plus 3x3 spatial variance fallback for short histories.
2. `svgf_atrous.comp`: configurable multi-scale 5x5 B3-spline edge-stopping A-trous wavelet iterations (`denoiser_atrous_iterations`, clamped 0..=5).
3. `svgf_compose_tonemap.comp`: albedo remodulation, emission/background composition, unbiased progressive accumulation when denoising is disabled, and HDR tone mapping.

`Graphic3dToneMappingMethod` supports `Disabled`, `Filmic`, `AgX` (Blender AgX), and `Aces` (Hill ACES fit). `VulkanRenderer::tonemap_hdr` also exposes `tonemap.comp` directly for standalone HDR-to-LDR GPU tone mapping.

## Verification

`crates/martensite-vulkan/tests/openpbr_and_raytrace.rs` verifies:

- Physical equation invariants in `openpbr_math` (coat darkening, coat roughening, coat IOR modulation, F82 grazing tint, EON diffuse, and thin-glass roughness remapping).
- White-furnace energy conservation across white dielectric, smooth/rough white conductor, and coated white materials.
- All 10 canonical OpenPBR materials rendered in both rasterization and compute path tracing.
- Hardware `VK_KHR_ray_query` vs software compute BVH radiance parity on RT hardware, and explicit `MissingDeviceFeature` refusal on non-RT devices (verified on Mesa lavapipe).
- SVGF variance reduction without NaN/Inf artifacts.
- GPU HDR tone-mapping monotonicity across `Disabled`, `Filmic`, `AgX`, and `Aces`.
- Byte-exact determinism for fixed-seed path-traced captures and multi-threaded safety of LUT and environment CDF generation.
- Zero `VK_LAYER_KHRONOS_validation` errors or warnings across all tests.

Measured BLAS build/compaction, hardware vs software path-tracing throughput, and SVGF timings are recorded in [benchmarks.md](benchmarks.md).
