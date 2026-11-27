//! Flat world-space scene packing and binary BVH builder shared by both
//! `pathtrace_hw.comp` (`VK_KHR_ray_query` TLAS) and `pathtrace_sw.comp` (software BVH SSBO).
//!
//! Triangles are reordered into BVH leaf order before buffer upload so hardware `gl_PrimitiveID`
//! and software BVH leaf indices address the exact same `PtTriangle` and `PtVertex` records.
use super::lights::RaytraceLights;
use super::triangle_set::RaytraceTriangleSet;
use crate::openpbr_material::OpenPbrGpuMaterial;
use martensite_tkernel::ncollection::{Mat4, Vec4};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Vertex layout matching `struct PtVertex` in `shaders/openpbr/pathtrace_main.glsl` (32 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PtVertex {
    pub pos_u: [f32; 4],
    pub norm_v: [f32; 4],
}

/// Triangle layout matching `struct PtTriangle` in `shaders/openpbr/pathtrace_main.glsl` (16 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PtTriangle {
    pub idx_mat: [u32; 4],
}

/// Binary BVH node matching `struct PtBvhNode` in `shaders/openpbr/pathtrace_main.glsl` (32 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PtBvhNode {
    pub bmin_left: [f32; 4],
    pub bmax_cnt: [f32; 4],
}

/// Analytic light layout matching `struct PtLight` in `shaders/openpbr/pathtrace_main.glsl` (80 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PtLight {
    pub kind: [u32; 4],
    pub color: [f32; 4],
    pub position: [f32; 4],
    pub direction: [f32; 4],
    pub params: [f32; 4],
}

/// Camera & frame settings UBO matching `PathtraceCameraUbo` in `pathtrace_main.glsl` (272 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PathtraceCameraUbo {
    pub eye_origin: [f32; 4],
    pub eye_view: [f32; 4],
    pub eye_right: [f32; 4],
    pub eye_up: [f32; 4],
    pub view_proj: [f32; 16],
    pub prev_view_proj: [f32; 16],
    pub back_top: [f32; 4],
    pub back_bottom: [f32; 4],
    pub ambient: [f32; 4],
    pub counts: [u32; 4],
    pub settings: [u32; 4],
    pub misc: [f32; 4],
}

/// Flattened world-space scene ready for upload to both HW RT (BLAS/TLAS) and SW BVH compute.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PtSceneData {
    pub vertices: Vec<PtVertex>,
    pub triangles: Vec<PtTriangle>,
    pub bvh_nodes: Vec<PtBvhNode>,
    pub materials: Vec<OpenPbrGpuMaterial>,
    pub lights: Vec<PtLight>,
    /// Stable content hash of world-space positions + triangle indices for `BlasCache`.
    pub geometry_hash: u64,
}

impl PtSceneData {
    /// Flattens `sets` into world-space vertices/triangles, builds the binary BVH, and packs lights.
    pub fn build(
        sets: &[RaytraceTriangleSet],
        materials: Vec<OpenPbrGpuMaterial>,
        lights: &RaytraceLights,
    ) -> Self {
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();
        for set in sets {
            append_world_set(set, &mut vertices, &mut triangles);
        }
        let bvh_nodes = build_binary_bvh(&vertices, &mut triangles);
        let geometry_hash = hash_geometry(&vertices, &triangles);
        let pt_lights = pack_lights(lights);
        Self {
            vertices,
            triangles,
            bvh_nodes,
            materials,
            lights: pt_lights,
            geometry_hash,
        }
    }

    /// Packs `[f32; 4]` positions (`stride = 16`) for `BottomLevelAccel::build_triangles`.
    pub fn blas_position_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.vertices.len() * 16);
        for v in &self.vertices {
            for f in v.pos_u {
                out.extend_from_slice(&f.to_ne_bytes());
            }
        }
        out
    }

    /// Packs `u32` triangle indices (`3 * triangles.len()`) for `BottomLevelAccel::build_triangles`.
    pub fn blas_index_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.triangles.len() * 12);
        for t in &self.triangles {
            for idx in &t.idx_mat[..3] {
                out.extend_from_slice(&idx.to_ne_bytes());
            }
        }
        out
    }
}

fn append_world_set(
    set: &RaytraceTriangleSet,
    vertices: &mut Vec<PtVertex>,
    triangles: &mut Vec<PtTriangle>,
) {
    let base_vertex = vertices.len() as u32;
    let inv_t = set.inverse_transform();
    for (i, pos) in set.vertices.iter().enumerate() {
        let wp = set.transform * Vec4::new(pos.x, pos.y, pos.z, 1.0);
        let n = set.normals.get(i).copied().unwrap_or([0.0, 0.0, 1.0]);
        let wn = transform_normal(&inv_t, n);
        let uv = set.tex_coords.get(i).copied().unwrap_or([0.0, 0.0]);
        vertices.push(PtVertex {
            pos_u: [wp.x, wp.y, wp.z, uv[0]],
            norm_v: [wn[0], wn[1], wn[2], uv[1]],
        });
    }
    for tri in &set.elements {
        triangles.push(PtTriangle {
            idx_mat: [
                base_vertex + tri[0].max(0) as u32,
                base_vertex + tri[1].max(0) as u32,
                base_vertex + tri[2].max(0) as u32,
                tri[3].max(0) as u32,
            ],
        });
    }
}

fn transform_normal(inv: &Mat4<f32>, n: [f32; 3]) -> [f32; 3] {
    let m = inv.data();
    let x = m[0] * n[0] + m[1] * n[1] + m[2] * n[2];
    let y = m[4] * n[0] + m[5] * n[1] + m[6] * n[2];
    let z = m[8] * n[0] + m[9] * n[1] + m[10] * n[2];
    let len = (x * x + y * y + z * z).sqrt();
    if len > 1e-8 {
        [x / len, y / len, z / len]
    } else {
        [0.0, 0.0, 1.0]
    }
}

fn pack_lights(lights: &RaytraceLights) -> Vec<PtLight> {
    lights
        .sources
        .iter()
        .map(|[emission, position]| {
            let is_point = position[3] > 0.5;
            PtLight {
                kind: [if is_point { 2 } else { 1 }, 0, 0, 0],
                color: [emission[0], emission[1], emission[2], 1.0],
                position: [position[0], position[1], position[2], position[3]],
                direction: [0.0, 0.0, 0.0, 0.0],
                params: [1.0, 0.0, 0.0, 0.0],
            }
        })
        .collect()
}

fn hash_geometry(vertices: &[PtVertex], triangles: &[PtTriangle]) -> u64 {
    let mut h = DefaultHasher::new();
    vertices.len().hash(&mut h);
    triangles.len().hash(&mut h);
    for v in vertices {
        for f in v.pos_u {
            f.to_bits().hash(&mut h);
        }
    }
    for t in triangles {
        t.idx_mat[..3].hash(&mut h);
    }
    h.finish()
}

/// Builds a compact binary BVH over `triangles` (reordering `triangles` in-place to leaf order).
pub fn build_binary_bvh(vertices: &[PtVertex], triangles: &mut [PtTriangle]) -> Vec<PtBvhNode> {
    if triangles.is_empty() {
        return vec![PtBvhNode::default()];
    }
    let mut nodes = Vec::with_capacity(triangles.len() * 2);
    nodes.push(PtBvhNode::default());
    build_node(vertices, triangles, &mut nodes, 0, 0, triangles.len());
    nodes
}

fn triangle_bounds(vertices: &[PtVertex], tri: &PtTriangle) -> ([f32; 3], [f32; 3], [f32; 3]) {
    let p0 = vertices[tri.idx_mat[0] as usize].pos_u;
    let p1 = vertices[tri.idx_mat[1] as usize].pos_u;
    let p2 = vertices[tri.idx_mat[2] as usize].pos_u;
    let bmin = [
        p0[0].min(p1[0]).min(p2[0]),
        p0[1].min(p1[1]).min(p2[1]),
        p0[2].min(p1[2]).min(p2[2]),
    ];
    let bmax = [
        p0[0].max(p1[0]).max(p2[0]),
        p0[1].max(p1[1]).max(p2[1]),
        p0[2].max(p1[2]).max(p2[2]),
    ];
    let centroid = [
        (bmin[0] + bmax[0]) * 0.5,
        (bmin[1] + bmax[1]) * 0.5,
        (bmin[2] + bmax[2]) * 0.5,
    ];
    (bmin, bmax, centroid)
}

fn build_node(
    vertices: &[PtVertex],
    triangles: &mut [PtTriangle],
    nodes: &mut Vec<PtBvhNode>,
    node_idx: usize,
    start: usize,
    count: usize,
) {
    let mut bmin = [f32::INFINITY; 3];
    let mut bmax = [f32::NEG_INFINITY; 3];
    let mut cmin = [f32::INFINITY; 3];
    let mut cmax = [f32::NEG_INFINITY; 3];
    for tri in &triangles[start..start + count] {
        let (tmin, tmax, c) = triangle_bounds(vertices, tri);
        for a in 0..3 {
            bmin[a] = bmin[a].min(tmin[a]);
            bmax[a] = bmax[a].max(tmax[a]);
            cmin[a] = cmin[a].min(c[a]);
            cmax[a] = cmax[a].max(c[a]);
        }
    }
    for a in 0..3 {
        bmin[a] -= 1e-5;
        bmax[a] += 1e-5;
    }
    if count <= 4 {
        nodes[node_idx] = PtBvhNode {
            bmin_left: [bmin[0], bmin[1], bmin[2], f32::from_bits(start as u32)],
            bmax_cnt: [bmax[0], bmax[1], bmax[2], f32::from_bits(count as u32)],
        };
        return;
    }
    let extent = [cmax[0] - cmin[0], cmax[1] - cmin[1], cmax[2] - cmin[2]];
    let axis = if extent[0] >= extent[1] && extent[0] >= extent[2] {
        0
    } else if extent[1] >= extent[2] {
        1
    } else {
        2
    };
    let mid = (cmin[axis] + cmax[axis]) * 0.5;
    let slice = &mut triangles[start..start + count];
    let mut split = partition_triangles(vertices, slice, axis, mid);
    if split == 0 || split == count {
        split = count / 2;
    }
    let left_idx = nodes.len();
    let right_idx = left_idx + 1;
    nodes.push(PtBvhNode::default());
    nodes.push(PtBvhNode::default());
    nodes[node_idx] = PtBvhNode {
        bmin_left: [bmin[0], bmin[1], bmin[2], f32::from_bits(left_idx as u32)],
        bmax_cnt: [
            bmax[0],
            bmax[1],
            bmax[2],
            f32::from_bits((-(right_idx as i32)) as u32),
        ],
    };
    build_node(vertices, triangles, nodes, left_idx, start, split);
    build_node(
        vertices,
        triangles,
        nodes,
        right_idx,
        start + split,
        count - split,
    );
}

fn partition_triangles(
    vertices: &[PtVertex],
    slice: &mut [PtTriangle],
    axis: usize,
    mid: f32,
) -> usize {
    let mut i = 0;
    let mut j = slice.len();
    while i < j {
        let (_, _, c) = triangle_bounds(vertices, &slice[i]);
        if c[axis] < mid {
            i += 1;
        } else {
            j -= 1;
            slice.swap(i, j);
        }
    }
    i
}

pub fn encode_vertices_bytes(vertices: &[PtVertex]) -> Vec<u8> {
    let mut out = Vec::with_capacity(vertices.len().max(1) * 32);
    if vertices.is_empty() {
        out.resize(32, 0);
        return out;
    }
    for v in vertices {
        for f in v.pos_u.iter().chain(v.norm_v.iter()) {
            out.extend_from_slice(&f.to_ne_bytes());
        }
    }
    out
}

pub fn encode_triangles_bytes(triangles: &[PtTriangle]) -> Vec<u8> {
    let mut out = Vec::with_capacity(triangles.len().max(1) * 16);
    if triangles.is_empty() {
        out.resize(16, 0);
        return out;
    }
    for t in triangles {
        for u in t.idx_mat {
            out.extend_from_slice(&u.to_ne_bytes());
        }
    }
    out
}

pub fn encode_bvh_bytes(nodes: &[PtBvhNode]) -> Vec<u8> {
    let mut out = Vec::with_capacity(nodes.len().max(1) * 32);
    if nodes.is_empty() {
        out.resize(32, 0);
        return out;
    }
    for n in nodes {
        for f in n.bmin_left.iter().chain(n.bmax_cnt.iter()) {
            out.extend_from_slice(&f.to_ne_bytes());
        }
    }
    out
}

pub fn encode_materials_bytes(materials: &[OpenPbrGpuMaterial]) -> Vec<u8> {
    let mut out = Vec::with_capacity(materials.len().max(1) * 256);
    if materials.is_empty() {
        out.extend_from_slice(&OpenPbrGpuMaterial::default().to_bytes());
        return out;
    }
    for m in materials {
        out.extend_from_slice(&m.to_bytes());
    }
    out
}

pub fn encode_lights_bytes(lights: &[PtLight]) -> Vec<u8> {
    let mut out = Vec::with_capacity(lights.len().max(1) * 80);
    if lights.is_empty() {
        out.resize(80, 0);
        return out;
    }
    for l in lights {
        for u in l.kind {
            out.extend_from_slice(&u.to_ne_bytes());
        }
        for f in l
            .color
            .iter()
            .chain(l.position.iter())
            .chain(l.direction.iter())
            .chain(l.params.iter())
        {
            out.extend_from_slice(&f.to_ne_bytes());
        }
    }
    out
}

pub fn encode_camera_ubo_bytes(ubo: &PathtraceCameraUbo) -> [u8; 288] {
    fn push_f(out: &mut [u8], pos: &mut usize, slice: &[f32]) {
        for &f in slice {
            out[*pos..*pos + 4].copy_from_slice(&f.to_ne_bytes());
            *pos += 4;
        }
    }
    fn push_u(out: &mut [u8], pos: &mut usize, slice: &[u32]) {
        for &u in slice {
            out[*pos..*pos + 4].copy_from_slice(&u.to_ne_bytes());
            *pos += 4;
        }
    }
    let mut out = [0u8; 288];
    let mut pos = 0;
    push_f(&mut out, &mut pos, &ubo.eye_origin);
    push_f(&mut out, &mut pos, &ubo.eye_view);
    push_f(&mut out, &mut pos, &ubo.eye_right);
    push_f(&mut out, &mut pos, &ubo.eye_up);
    push_f(&mut out, &mut pos, &ubo.view_proj);
    push_f(&mut out, &mut pos, &ubo.prev_view_proj);
    push_f(&mut out, &mut pos, &ubo.back_top);
    push_f(&mut out, &mut pos, &ubo.back_bottom);
    push_f(&mut out, &mut pos, &ubo.ambient);
    push_u(&mut out, &mut pos, &ubo.counts);
    push_u(&mut out, &mut pos, &ubo.settings);
    push_f(&mut out, &mut pos, &ubo.misc);
    debug_assert_eq!(pos, 288);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_bvh_covers_all_triangles_and_encodes_leaf_and_internal_nodes() {
        let mut verts = Vec::new();
        let mut tris = Vec::new();
        for i in 0..20u32 {
            let x = i as f32 * 2.0;
            let b = verts.len() as u32;
            verts.push(PtVertex {
                pos_u: [x, 0.0, 0.0, 0.0],
                norm_v: [0.0, 0.0, 1.0, 0.0],
            });
            verts.push(PtVertex {
                pos_u: [x + 1.0, 0.0, 0.0, 1.0],
                norm_v: [0.0, 0.0, 1.0, 0.0],
            });
            verts.push(PtVertex {
                pos_u: [x, 1.0, 0.0, 0.0],
                norm_v: [0.0, 0.0, 1.0, 1.0],
            });
            tris.push(PtTriangle {
                idx_mat: [b, b + 1, b + 2, 0],
            });
        }
        let nodes = build_binary_bvh(&verts, &mut tris);
        assert!(nodes.len() > 1);
        let mut covered = 0usize;
        for n in &nodes {
            let cnt = n.bmax_cnt[3].to_bits() as i32;
            if cnt > 0 {
                covered += cnt as usize;
            }
        }
        assert_eq!(covered, 20);
        assert_eq!(
            encode_camera_ubo_bytes(&PathtraceCameraUbo::default()).len(),
            288
        );
    }
}
