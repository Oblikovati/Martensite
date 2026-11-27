// Instanced screen-space quad expansion for large points and textured marker sprites (#634).

struct PointInstance {
    position: vec4<f32>,
    color: vec4<f32>,
};

struct PointUniforms {
    params: vec4<f32>, // [viewport_w, viewport_h, point_size_px, alpha_cutoff]
    flags: vec4<u32>,  // [has_sprite_texture, shape_kind, 0, 0]
};

@group(0) @binding(0) var<uniform> pt_u: PointUniforms;
@group(0) @binding(1) var<storage, read> points: array<PointInstance>;
@group(1) @binding(0) var sprite_tex: texture_2d<f32>;
@group(1) @binding(1) var sprite_sampler: sampler;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) vid: u32,
    @builtin(instance_index) iid: u32,
) -> VsOut {
    let pt = points[iid];
    var corners = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(-1.0, 1.0),
    );
    let c = corners[vid];
    let vp = max(pt_u.params.xy, vec2<f32>(1.0, 1.0));
    let half_px = max(pt_u.params.z, 1.0) * 0.5;
    let offset_ndc = c * half_px * (2.0 / vp);
    var out: VsOut;
    out.pos = vec4<f32>(pt.position.xy + offset_ndc * pt.position.w, pt.position.z, pt.position.w);
    out.color = pt.color;
    out.uv = vec2<f32>(c.x * 0.5 + 0.5, 0.5 - c.y * 0.5);
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let sample_rgba = textureSample(sprite_tex, sprite_sampler, in.uv);
    if pt_u.flags.x != 0u {
        let out_c = in.color * sample_rgba;
        if out_c.a < max(pt_u.params.w, 0.01) {
            discard;
        }
        return out_c;
    }
    return in.color;
}
