// Procedural RenderShaderGrid shader (#634).

struct GridUniforms {
    inv_view_proj: mat4x4<f32>,
    view_proj: mat4x4<f32>,
    grid_step: vec4<f32>,
    color_main: vec4<f32>,
    color_sub: vec4<f32>,
};

@group(0) @binding(0) var<uniform> grid_u: GridUniforms;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) near_pt: vec3<f32>,
    @location(1) far_pt: vec3<f32>,
};

fn unproject(ndc: vec3<f32>) -> vec3<f32> {
    let p = grid_u.inv_view_proj * vec4<f32>(ndc, 1.0);
    return p.xyz / max(abs(p.w), 1e-6);
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0),
    );
    let p = positions[vid];
    var out: VsOut;
    out.pos = vec4<f32>(p, 0.0, 1.0);
    out.near_pt = unproject(vec3<f32>(p, 0.0));
    out.far_pt = unproject(vec3<f32>(p, 1.0));
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let dir = in.far_pt - in.near_pt;
    let valid_dir = abs(dir.z) >= 1e-6;
    let safe_dz = select(1.0, dir.z, valid_dir);
    let t = -in.near_pt.z / safe_dz;
    let world_xy = in.near_pt.xy + t * dir.xy - grid_u.grid_step.zw;
    let step = max(grid_u.grid_step.xy, vec2<f32>(1e-4, 1e-4));
    let coord = world_xy / step;
    // Compute fwidth in uniform control flow before any discard.
    let deriv = max(fwidth(coord), vec2<f32>(1e-5, 1e-5));
    if !valid_dir || t < 0.0 || t > 1.0 {
        discard;
    }
    let line = abs(fract(coord - 0.5) - 0.5) / deriv;
    let alpha = 1.0 - min(min(line.x, line.y), 1.0);
    if alpha <= 0.02 {
        discard;
    }
    return vec4<f32>(grid_u.color_main.rgb, grid_u.color_main.a * alpha);
}
