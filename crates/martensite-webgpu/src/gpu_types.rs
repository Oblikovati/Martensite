//! `bytemuck`-verified GPU buffer layouts strictly obeying WebGPU Core 16/256-byte alignment (#634, #635).
use bytemuck::{Pod, Zeroable};
use martensite_render_core::{PrimitiveDraw, RenderClipping, RenderLightSet, RenderLighting};
use martensite_tkservice::graphic3d::Graphic3dTypeOfLightSource;

/// Maximum lights evaluated per surface draw in the WebGPU Core uniform block.
pub const MAX_WEBGPU_LIGHTS: usize = 16;
/// Maximum clipping half-spaces evaluated per surface draw in fragment shader.
pub const MAX_WEBGPU_CLIP_PLANES: usize = 8;

/// Unified 48-byte vertex layout shared by unlit, Phong, OpenPBR, and mesh-edge pipelines.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct GpuSurfaceVertex {
    /// Object-space position (`xyz`) and homogeneous weight (`w`).
    pub position: [f32; 4],
    /// Linear RGBA vertex color.
    pub color: [f32; 4],
    /// Object-space normal (`xyz`) and barycentric edge mask (`w`, bits 0..2).
    pub normal_edge: [f32; 4],
    /// Texture coordinates (`xy`) and barycentric coordinates (`zw`).
    pub uv_bary: [f32; 4],
}

/// Per-draw uniform block (256-byte aligned for dynamic uniform offsets).
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct GpuDrawUniforms {
    /// Column-major clip transform (`projection * view * model`).
    pub clip_matrix: [[f32; 4]; 4],
    /// Column-major world matrix.
    pub world_matrix: [[f32; 4]; 4],
    /// Column-major inverse-transpose normal matrix.
    pub normal_matrix: [[f32; 4]; 4],
    /// Eye position in world space (`w == 0` marks orthographic eye direction).
    pub eye_position: [f32; 4],
    /// Base RGBA color / albedo multiplier.
    pub base_color: [f32; 4],
    /// Specular RGB + shininess (Phong) or specular weight/IOR (OpenPBR).
    pub specular_params: [f32; 4],
    /// Emissive RGB + metallic factor.
    pub emissive_metallic: [f32; 4],
    /// `[roughness, alpha_cutoff, coat_weight, coat_roughness]`.
    pub pbr_params: [f32; 4],
    /// `[shading_mode, light_count, clip_count, flags]` as `u32`.
    pub counts_flags: [u32; 4],
    /// Mesh edge overlay RGBA color.
    pub edge_color: [f32; 4],
    /// `[viewport_w, viewport_h, edge_width, alpha_scale]`.
    pub viewport_params: [f32; 4],
}

impl Default for GpuDrawUniforms {
    fn default() -> Self {
        Self {
            clip_matrix: identity_mat4(),
            world_matrix: identity_mat4(),
            normal_matrix: identity_mat4(),
            eye_position: [0.0, 0.0, 1.0, 0.0],
            base_color: [1.0, 1.0, 1.0, 1.0],
            specular_params: [0.2, 0.2, 0.2, 32.0],
            emissive_metallic: [0.0, 0.0, 0.0, 0.0],
            pbr_params: [0.5, 0.0, 0.0, 0.0],
            counts_flags: [0, 0, 0, 0],
            edge_color: [0.0, 0.0, 0.0, 1.0],
            viewport_params: [64.0, 64.0, 1.0, 1.0],
        }
    }
}

/// One GPU light record (64 bytes, 16-byte aligned).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct GpuLight {
    /// Light RGB radiance (`xyz`) and intensity (`w`).
    pub color_intensity: [f32; 4],
    /// Position (point/spot) or normalized direction (directional), plus `kind` in `w`.
    pub position_kind: [f32; 4],
    /// Spot direction (`xyz`) and spot cutoff cosine (`w`).
    pub direction_cutoff: [f32; 4],
    /// `[const_atten, linear_atten, spot_exponent, range]`.
    pub attenuation: [f32; 4],
}

/// Scene light + clipping storage/uniform block (16-byte aligned).
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct GpuSceneLightsAndClips {
    /// Ambient RGB radiance + unused `w`.
    pub ambient: [f32; 4],
    /// Shadow light-space view-projection matrix.
    pub shadow_matrix: [[f32; 4]; 4],
    /// `[shadow_bias, shadow_enabled, env_intensity, padding]`.
    pub shadow_env: [f32; 4],
    /// Up to `MAX_WEBGPU_LIGHTS` lights.
    pub lights: [GpuLight; MAX_WEBGPU_LIGHTS],
    /// Up to `MAX_WEBGPU_CLIP_PLANES` half-space equations `(a, b, c, d)` where `dot(p, n) + d < 0` clips.
    pub clip_planes: [[f32; 4]; MAX_WEBGPU_CLIP_PLANES],
    /// Chain IDs for AND/OR clipping chains (`chain_id` per plane; 0..MAX_WEBGPU_CLIP_PLANES).
    pub clip_chains: [u32; MAX_WEBGPU_CLIP_PLANES],
}

impl Default for GpuSceneLightsAndClips {
    fn default() -> Self {
        Self {
            ambient: [0.2, 0.2, 0.2, 1.0],
            shadow_matrix: identity_mat4(),
            shadow_env: [0.002, 0.0, 1.0, 0.0],
            lights: [GpuLight::default(); MAX_WEBGPU_LIGHTS],
            clip_planes: [[0.0; 4]; MAX_WEBGPU_CLIP_PLANES],
            clip_chains: [0; MAX_WEBGPU_CLIP_PLANES],
        }
    }
}

/// Segment record for instanced screen-space wide / stippled line expansion (`wide_line.wgsl`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct GpuLineSegment {
    /// Clip-space start endpoint.
    pub p0: [f32; 4],
    /// Clip-space end endpoint.
    pub p1: [f32; 4],
    /// Start linear RGBA color.
    pub c0: [f32; 4],
    /// End linear RGBA color.
    pub c1: [f32; 4],
}

/// Uniform block for instanced wide/stippled lines.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct GpuLineUniforms {
    /// `[viewport_w, viewport_h, line_width_px, stipple_factor]`.
    pub params: [f32; 4],
    /// `[stipple_pattern, pad, pad, pad]`.
    pub stipple: [u32; 4],
}

/// Point record for instanced screen-space point/marker expansion (`point_sprite.wgsl`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct GpuPointInstance {
    /// Clip-space center position.
    pub position: [f32; 4],
    /// Linear RGBA color.
    pub color: [f32; 4],
}

/// Uniform block for instanced point markers.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct GpuPointUniforms {
    /// `[viewport_w, viewport_h, point_size_px, alpha_cutoff]`.
    pub params: [f32; 4],
    /// `[has_sprite_texture, shape_kind, pad, pad]`.
    pub flags: [u32; 4],
}

/// Background uniform block (`background.wgsl`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct GpuBackgroundUniforms {
    /// Top-left corner linear RGBA.
    pub corner_tl: [f32; 4],
    /// Top-right corner linear RGBA.
    pub corner_tr: [f32; 4],
    /// Bottom-left corner linear RGBA.
    pub corner_bl: [f32; 4],
    /// Bottom-right corner linear RGBA.
    pub corner_br: [f32; 4],
    /// `[mode (0=quad, 1=image, 2=skydome), sun_dir.x, sun_dir.y, sun_dir.z]`.
    pub mode_sun: [f32; 4],
}

/// Shader grid uniform block (`grid.wgsl`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct GpuGridUniforms {
    /// Inverse view-projection matrix to unproject screen pixels to world rays.
    pub inv_view_proj: [[f32; 4]; 4],
    /// View-projection matrix for grid depth output.
    pub view_proj: [[f32; 4]; 4],
    /// `[step_x, step_y, origin_x, origin_y]`.
    pub grid_step: [f32; 4],
    /// Primary grid line RGBA.
    pub color_main: [f32; 4],
    /// Secondary grid line RGBA.
    pub color_sub: [f32; 4],
}

/// Stereo / subview composition uniform block (`stereo_subview.wgsl`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct GpuStereoSubviewUniforms {
    /// `[mode: 0=subview_blit, 1=red_cyan, 2=green_magenta, 3=yellow_blue, 4=row_interlaced, 5=col_interlaced, 6=side_by_side, flip_y, pad, pad]`.
    pub mode_flags: [u32; 4],
    /// `[dst_x, dst_y, dst_w, dst_h]` in normalized `[0, 1]` UV space.
    pub rect_uv: [f32; 4],
}

/// Compute raytracer camera & accumulation uniform block (`raytrace_render.wgsl`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct GpuRaytraceUniforms {
    /// Camera eye world position (`xyz`) + sample index (`w`).
    pub eye_sample: [f32; 4],
    /// Top-left frustum ray direction (`xyz`) + max bounces (`w`).
    pub ray_tl_bounces: [f32; 4],
    /// Horizontal frustum span vector (`xyz`) + image width (`w`).
    pub ray_dx_width: [f32; 4],
    /// Vertical frustum span vector (`xyz`) + image height (`w`).
    pub ray_dy_height: [f32; 4],
    /// Clear / background sky RGBA.
    pub background: [f32; 4],
    /// `[triangle_count, bvh_node_count, light_count, flags]`.
    pub counts: [u32; 4],
}

/// Compute raytracer triangle packed in 64 bytes (3 vertices + material index).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct GpuRayTriangle {
    /// Vertex 0 world position (`xyz`) + material index as bits (`w`).
    pub v0_mat: [f32; 4],
    /// Edge `v1 - v0` (`xyz`) + unused (`w`).
    pub e1: [f32; 4],
    /// Edge `v2 - v0` (`xyz`) + unused (`w`).
    pub e2: [f32; 4],
    /// Geometric face normal (`xyz`) + unused (`w`).
    pub normal: [f32; 4],
}

/// Compute raytracer BVH node (32 bytes, WebGPU Core storage buffer).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct GpuRayBvhNode {
    /// AABB min (`xyz`) + `left_or_first` (`u32` bits in `w`).
    pub min_left: [f32; 4],
    /// AABB max (`xyz`) + `count_or_right` (`u32` bits in `w`; `> 0` leaf triangle count).
    pub max_right: [f32; 4],
}

/// Compute raytracer OpenPBR material record (48 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct GpuRayMaterial {
    /// Base albedo RGB + alpha (`w`).
    pub base_color: [f32; 4],
    /// Emissive RGB + metallic (`w`).
    pub emissive_metallic: [f32; 4],
    /// `[roughness, ior, coat_weight, coat_roughness]`.
    pub pbr_params: [f32; 4],
}

/// Convert a flat 16-float column-major array to `[[f32; 4]; 4]`.
pub fn mat4_from_slice(m: [f32; 16]) -> [[f32; 4]; 4] {
    [
        [m[0], m[1], m[2], m[3]],
        [m[4], m[5], m[6], m[7]],
        [m[8], m[9], m[10], m[11]],
        [m[12], m[13], m[14], m[15]],
    ]
}

/// Column-major 4×4 identity matrix.
pub const fn identity_mat4() -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

/// Pack a `PrimitiveDraw`'s vertices and indices into `GpuSurfaceVertex` records.
pub fn pack_surface_vertices(draw: &PrimitiveDraw) -> (Vec<GpuSurfaceVertex>, Vec<u32>) {
    let normals: Option<&[[f32; 3]]> = draw.lighting.as_ref().map(|l| &l.normals[..]);
    let text_uvs: Option<&[[f32; 2]]> = draw.textured_style.as_ref().map(|t| &t.uv[..]);
    let surf_uvs: Option<&[[f32; 4]]> = draw.surface_texture.as_ref().map(|t| &t.uv[..]);
    let uv_at = |idx: usize| -> [f32; 2] {
        if let Some(us) = text_uvs {
            return us.get(idx).copied().unwrap_or([0.0, 0.0]);
        }
        if let Some(us) = surf_uvs {
            let u = us.get(idx).copied().unwrap_or([0.0, 0.0, 0.0, 1.0]);
            return [u[0], u[1]];
        }
        [0.0, 0.0]
    };
    let has_edges = draw.mesh_edges.is_some() || draw.polygon_edge_style.is_some();

    if has_edges && draw.indices.len() >= 3 {
        // Expand triangles so every triangle carries exact barycentric coordinates for
        // zero-geometry-shader wireframe / mesh-edge rendering.
        let mut verts = Vec::with_capacity(draw.indices.len());
        let mut inds = Vec::with_capacity(draw.indices.len());
        let barys = [[1.0_f32, 0.0], [0.0, 1.0], [0.0, 0.0]];
        for (tri_idx, tri) in draw.indices.as_chunks::<3>().0.iter().enumerate() {
            for (corner, &vi) in tri.iter().enumerate() {
                let v = draw.vertices[vi as usize];
                let n = normals
                    .and_then(|ns| ns.get(vi as usize))
                    .copied()
                    .unwrap_or([0.0, 0.0, 1.0]);
                let uv = uv_at(vi as usize);
                verts.push(GpuSurfaceVertex {
                    position: [v.position[0], v.position[1], v.position[2], v.weight],
                    color: v.color,
                    normal_edge: [n[0], n[1], n[2], 7.0],
                    uv_bary: [uv[0], uv[1], barys[corner][0], barys[corner][1]],
                });
                inds.push((tri_idx * 3 + corner) as u32);
            }
        }
        return (verts, inds);
    }

    let verts = draw
        .vertices
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let n = normals
                .and_then(|ns| ns.get(i))
                .copied()
                .unwrap_or([0.0, 0.0, 1.0]);
            let uv = uv_at(i);
            GpuSurfaceVertex {
                position: [v.position[0], v.position[1], v.position[2], v.weight],
                color: v.color,
                normal_edge: [n[0], n[1], n[2], 0.0],
                uv_bary: [uv[0], uv[1], 0.0, 0.0],
            }
        })
        .collect();
    (verts, draw.indices.to_vec())
}

/// Pack draw uniforms and scene lights/clips for `draw`.
pub fn pack_draw_state(
    draw: &PrimitiveDraw,
    extent: [u32; 2],
    has_texture: bool,
    has_shadow: bool,
    shadow_matrix: [[f32; 4]; 4],
) -> (GpuDrawUniforms, GpuSceneLightsAndClips) {
    let mut u = GpuDrawUniforms {
        clip_matrix: mat4_from_slice(draw.clip_transform),
        viewport_params: [extent[0] as f32, extent[1] as f32, 1.0, 1.0],
        ..Default::default()
    };
    let mut scene = GpuSceneLightsAndClips {
        shadow_matrix,
        shadow_env: [0.002, if has_shadow { 1.0 } else { 0.0 }, 1.0, 0.0],
        ..Default::default()
    };

    let mut flags = 0u32;
    if has_texture {
        flags |= 1 << 0;
    }
    if let Some(ref edges) = draw.mesh_edges {
        flags |= 1 << 1;
        u.edge_color = edges.color;
        u.viewport_params[2] = edges.width.max(1.0);
    } else if draw.polygon_edge_style.is_some() {
        flags |= 1 << 1;
        u.edge_color = [0.0, 0.0, 0.0, 1.0];
        u.viewport_params[2] = 1.0;
    }
    if let Some(cutoff) = draw.surface_cutoff {
        u.pbr_params[1] = cutoff;
        flags |= 1 << 2;
    } else if let Some(ref tex) = draw.textured_style {
        u.pbr_params[1] = tex.alpha_cutoff;
        if tex.alpha_cutoff > 0.0 {
            flags |= 1 << 2;
        }
    }

    if let Some(ref lighting) = draw.lighting {
        pack_lighting_into(lighting, &mut u, &mut scene);
    }
    if let Some(ref clipping) = draw.clipping {
        u.counts_flags[2] = pack_clipping_into(clipping, &mut scene);
    }
    u.counts_flags[3] = flags;
    (u, scene)
}

fn pack_lighting_into(
    lighting: &RenderLighting,
    u: &mut GpuDrawUniforms,
    scene: &mut GpuSceneLightsAndClips,
) {
    u.world_matrix = mat4_from_slice(lighting.world);
    u.normal_matrix = mat4_from_slice(lighting.normal);
    u.eye_position = lighting.eye;
    let mode = if lighting.pbr { 2u32 } else { 1u32 };
    u.counts_flags[0] = mode;
    if lighting.pbr {
        let openpbr = &lighting.material.openpbr[0];
        let opacity = openpbr.geometry_meta[0].clamp(0.0, 1.0);
        let weight = openpbr.base_color_weight[3];
        u.base_color = [
            openpbr.base_color_weight[0] * weight,
            openpbr.base_color_weight[1] * weight,
            openpbr.base_color_weight[2] * weight,
            opacity,
        ];
        let lum = openpbr.emission_color_lum[3];
        u.emissive_metallic = [
            openpbr.emission_color_lum[0] * lum,
            openpbr.emission_color_lum[1] * lum,
            openpbr.emission_color_lum[2] * lum,
            openpbr.base_spec_params[0],
        ];
        u.pbr_params[0] = openpbr.base_spec_params[3].clamp(0.04, 1.0);
        u.pbr_params[2] = openpbr.coat_color_weight[3];
        u.pbr_params[3] = openpbr.coat_params[0].clamp(0.04, 1.0);
        u.specular_params = [
            openpbr.specular_color_aniso[0],
            openpbr.specular_color_aniso[1],
            openpbr.specular_color_aniso[2],
            openpbr.specular_ior_trans[0].max(1.0),
        ];
    } else {
        let front = &lighting.material.common[0];
        u.base_color = front.diffuse;
        u.specular_params = [
            front.specular_shininess[0],
            front.specular_shininess[1],
            front.specular_shininess[2],
            front.specular_shininess[3].max(1.0),
        ];
        u.emissive_metallic = [front.emission[0], front.emission[1], front.emission[2], 0.0];
    }
    u.counts_flags[1] = pack_lights(&lighting.lights, scene);
}

fn pack_lights(lights: &RenderLightSet, scene: &mut GpuSceneLightsAndClips) -> u32 {
    scene.ambient = [lights.ambient[0], lights.ambient[1], lights.ambient[2], 1.0];
    let mut count = 0usize;
    for src in lights.sources.iter() {
        if count >= MAX_WEBGPU_LIGHTS {
            break;
        }
        let kind_f = match src.kind {
            k if k == Graphic3dTypeOfLightSource::Directional as u32 => 0.0,
            k if k == Graphic3dTypeOfLightSource::Positional as u32 => 1.0,
            k if k == Graphic3dTypeOfLightSource::Spot as u32 => 2.0,
            _ => continue,
        };
        scene.lights[count] = GpuLight {
            color_intensity: [
                src.color[0],
                src.color[1],
                src.color[2],
                src.color[3].max(1.0),
            ],
            position_kind: [src.position[0], src.position[1], src.position[2], kind_f],
            direction_cutoff: [
                src.direction[0],
                src.direction[1],
                src.direction[2],
                src.params[2],
            ],
            attenuation: [
                src.params[0],
                src.params[1],
                src.params[3],
                src.direction[3],
            ],
        };
        count += 1;
    }
    count as u32
}

fn pack_clipping_into(clipping: &RenderClipping, scene: &mut GpuSceneLightsAndClips) -> u32 {
    let mut count = 0usize;
    for (chain_idx, chain) in clipping.chains().iter().enumerate() {
        for plane in chain {
            if count >= MAX_WEBGPU_CLIP_PLANES {
                return count as u32;
            }
            scene.clip_planes[count] = [plane[0], plane[1], plane[2], plane[3]];
            scene.clip_chains[count] = (chain_idx + 1) as u32;
            count += 1;
        }
    }
    count as u32
}

/// Placement culling uniform block matching `placement_cull.wgsl`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct GpuCullUniforms {
    /// Column-major camera view-projection matrix.
    pub view_proj: [[f32; 4]; 4],
    /// Number of placements in the input storage buffer.
    pub count: u32,
    /// Padding.
    pub _pad: [u32; 3],
}

/// Per-placement culling record matching `placement_cull.wgsl`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct GpuPlacementInput {
    /// World transform column 0.
    pub world_col0: [f32; 4],
    /// World transform column 1.
    pub world_col1: [f32; 4],
    /// World transform column 2.
    pub world_col2: [f32; 4],
    /// World transform column 3.
    pub world_col3: [f32; 4],
    /// Local AABB minimum (`xyz`).
    pub aabb_min: [f32; 4],
    /// Local AABB maximum (`xyz`).
    pub aabb_max: [f32; 4],
    /// Draw index count.
    pub index_count: u32,
    /// Draw first index.
    pub first_index: u32,
    /// Draw base vertex.
    pub base_vertex: i32,
    /// Instance identifier.
    pub instance_id: u32,
}

/// Indirect indexed draw command matching WebGPU `DrawIndexedIndirectArgs` (20 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct GpuDrawIndexedIndirect {
    /// Index count.
    pub index_count: u32,
    /// Instance count (`0` when culled, `1` when visible).
    pub instance_count: u32,
    /// First index.
    pub first_index: u32,
    /// Base vertex.
    pub base_vertex: i32,
    /// First instance.
    pub first_instance: u32,
}
