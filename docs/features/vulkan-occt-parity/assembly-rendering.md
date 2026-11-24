# Assembly raster rendering

## Ordered draws and visibility

A connected part shares its geometry streams across the placements that reference it.
Each warm frame updates the placement matrices. Consecutive materials and index widths
retain their original segment order, including at equal depth. Where the device supports
compute and indirect counts, the GPU compacts cropped runs into bounded command ranges.
The CPU structure BVH still supplies the visibility flags, counts and picking inputs.
Uniformly mirrored batches retain their winding. Mixed raster state, inherited alpha or
stencil tests, multisampling and clip sequences use ordinary composition. The renderer
splits large ordered indirect runs at the device draw-count limit and declines a batch
if its transform table cannot fit in a storage buffer.

`RenderBehaviorConfiguration::with_placement_occlusion(true)` enables an extra depth
pass for eligible retained placements. It draws previously visible placements at the
current camera into a separate depth target. The maximum-depth pyramid marks a placement
occluded only when its conservative nearest depth is strictly behind the covered pyramid
cells. Cleared pixels, near-plane crossings and depth ties keep the draw. A camera jump,
viewport change, source edit or incompatible frame requires a fresh witness. The indexed
color pass retains the original draw order.

## Mesh detail by part

`AisShape::set_presentation_mesh_lod` attaches a `StdPrsMeshLod` approval to the
current shape. Each request specifies the tessellation deflections and an approved
two-way distance bound between the primary and alternate meshes in object space.
The presentation meshes private copies of the part. Changing the shape clears the approval.

The renderer selects the cheapest level whose projected deviation fits the pixel
budget, after ordinary structure culling. Connected occurrences make this choice
independently under their current transforms. Frames with authored levels use the
ordinary draw path so the decision is made again when the camera moves, including
for captures. Picking continues to use the original shape. Ineligible styles or an
unbounded projection keep the primary mesh.

## Mesh and material formats

The ordinary indexed path uses 16-bit indices where the largest local index fits.
Indexed draw tables store one copy of each exact material used by the run. The draw
selects its material while the run shares the view's lighting state.

`RenderMeshVertexBehavior::Quantized16OctV2` packs each of three position components
into 16 bits relative to the mesh bounds and encodes the normal with two 16-bit
octahedral coordinates. It uses 28 bytes per vertex in place of 44. The renderer
selects stock vertices for unsupported surfaces. The default behavior uses stock
binary32 positions and normals.

`RenderBehaviorConfiguration::with_meshlet_draws(true)` uses
`VK_EXT_mesh_shader` for qualifying single-sample stock Phong surfaces. Each meshlet
carries at most 64 vertices and 96 triangles, in source order. Its object-space
bounds provide a frustum test and its normal cone identifies candidates for back-face
rejection. A winding check accounts for viewport rounding before rejecting a meshlet
by facing. The renderer owns a bounded meshlet cache alongside its indexed geometry,
so enabling it can increase cold upload time and peak memory. Ordered indexed draws
remain available for devices and frames outside these requirements.

## Parallel recording and uploads

Eligible layers or draw ranges record into secondary command buffers on workers.
The primary buffer executes them in layer and source order. The frame owns the
secondary pools until GPU completion.

Large, cold geometry transfers use a separate transfer-capable queue on devices that
offer one. A semaphore orders graphics consumption after the upload. Sparse patches
and devices with only the graphics queue use the existing graphics upload path.

A mapped view can choose to limit geometry upload per redraw with
`RenderAssemblyStreaming::VisibleBudgetedV2`. The view first stages visible draws and
keeps their order. It reports how much geometry is ready, so the host can schedule
another redraw while uploads remain. Capture and incompatible composition use the
complete frame path. See [progressive uploads](assembly-streaming.md).

## Unsafe inventory

The Phase 4 and 5 inventory update records 55 additional blocks across 20 Vulkan backend
files. The reviewed calls cover Vulkan resource ownership, descriptor and pipeline setup,
recording, and queue operations for the retained image and assembly raster paths. No new
unsafe Rust functions, traits or implementations were required. The existing mapped readback
boundary checks the requested byte range and reads after GPU completion. Rendering modes
that intentionally change precision or progressive completeness are selected as
`Quantized16OctV2` and `VisibleBudgetedV2`.

## Measurements

Three alternating Vulkan and OCCT rounds ran on Linux x86-64 with Radeon 8060S
(RADV), X11, VSync off and CPUs 8–15. The workload checksums matched.

| Workload | Vulkan | OCCT | Vulkan speed |
| --- | ---: | ---: | ---: |
| 30,000 connected placements, 30 parts | 1.51 ms | 8.32 ms | 5.51× |
| 30,000 connected placements, 2,815 parts | 5.26 ms | 15.59 ms | 2.96× |
| 100,000 displayed structures | 14.30 ms | 41.15 ms | 2.88× |
| 10 million triangles, stock vertices | 1.80 ms | 1.63 ms | 0.90× |

The two connected cases reached about 662 and 190 FPS. The larger part count
uses a synthetic distribution of CAD shapes. Run that fixture by setting
`MARTENSITE_594_DISTRIBUTED_PARTS=1` on the usual `frame_connected_30k` parity
command. The stock mesh spent about 1.81 ms waiting for the GPU, with no warm geometry upload or readback. An earlier set
of three native stock/quantized pairs measured medians of 1.862 ms and 1.604 ms
with the same workload checksum. Peak RSS was about 1,000 MiB in both cases.

The separate peak RSS runs measured 276 MiB for Vulkan and 218 MiB for
OCCT on the 30-part connected fixture, and 1,116 MiB versus 366 MiB on
the 100,000-structure fixture. The latter is a peak for the entire process,
including scene construction and the renderer.

In a shorter direct Vulkan comparison, mesh shaders took 0.50 ms at one
million triangles and 2.59 ms at ten million. The stock indexed path took
0.34 ms and 1.92 ms. That sphere scene keeps most geometry in view. Mesh
shaders remain opt-in.

The quantized native fixture checks that an interior position at 0.5 becomes
the independently decoded binary32 value at code 32768. The packed capture
matches the stock capture authored at that value byte for byte, bounds the
depth change at 4×10⁻⁶, and leaves replay of the original stock capture exact.

The native Hi-Z fixture checks the GPU visibility mask and compares final RGBA
and depth with ordinary draws. In seven alternating warm capture samples at
95×79, culling 2.1 million rear triangles took 3.30 ms with Hi-Z and 4.69 ms
without it. At 8.4 million rear triangles, it took 11.52 ms with Hi-Z and
15.31 ms without it. An earlier 2.1-million-triangle trial showed similar
times with and without the extra pass. Hi-Z stays opt-in.

The scoped invalidation tests include eight simultaneous writers. Each registered
scope observed all 1,024 completed edits. All three scope tests passed under the
pinned Rust nightly's ThreadSanitizer in a read-only Ubuntu 24.04 container.
