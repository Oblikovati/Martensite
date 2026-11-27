// WGSL compute placement frustum culling writing DrawIndexedIndirect args (#634).

struct PlacementInput {
    world_col0: vec4<f32>,
    world_col1: vec4<f32>,
    world_col2: vec4<f32>,
    world_col3: vec4<f32>,
    aabb_min: vec4<f32>,
    aabb_max: vec4<f32>,
    index_count: u32,
    first_index: u32,
    base_vertex: i32,
    instance_id: u32,
};

struct IndirectArgs {
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    base_vertex: i32,
    first_instance: u32,
};

struct CullUniforms {
    view_proj: mat4x4<f32>,
    count: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
};

@group(0) @binding(0) var<uniform> cull_u: CullUniforms;
@group(0) @binding(1) var<storage, read> placements: array<PlacementInput>;
@group(0) @binding(2) var<storage, read_write> indirect_out: array<IndirectArgs>;

@compute @workgroup_size(64, 1, 1)
fn cs_cull(@builtin(global_invocation_id) gid: vec3<u32>) {
    let idx = gid.x;
    if idx >= cull_u.count {
        return;
    }
    let item = placements[idx];
    let world = mat4x4<f32>(item.world_col0, item.world_col1, item.world_col2, item.world_col3);
    let mvp = cull_u.view_proj * world;
    var visible = false;
    for (var corner: u32 = 0u; corner < 8u; corner = corner + 1u) {
        let cx = select(item.aabb_min.x, item.aabb_max.x, (corner & 1u) != 0u);
        let cy = select(item.aabb_min.y, item.aabb_max.y, (corner & 2u) != 0u);
        let cz = select(item.aabb_min.z, item.aabb_max.z, (corner & 4u) != 0u);
        let clip = mvp * vec4<f32>(cx, cy, cz, 1.0);
        if clip.w > 0.0
            && clip.x >= -clip.w && clip.x <= clip.w
            && clip.y >= -clip.w && clip.y <= clip.w
            && clip.z >= 0.0 && clip.z <= clip.w
        {
            visible = true;
            break;
        }
    }
    indirect_out[idx] = IndirectArgs(
        item.index_count,
        select(0u, 1u, visible),
        item.first_index,
        item.base_vertex,
        item.instance_id,
    );
}
