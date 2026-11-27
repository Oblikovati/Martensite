// Weighted Blended OIT fullscreen composite pass (#634).
// Reads Rgba16Float accumulation and R16Float revealage textures (WebGPU Core).

@group(0) @binding(0) var accum_tex: texture_2d<f32>;
@group(0) @binding(1) var reveal_tex: texture_2d<f32>;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0),
    );
    var out: VsOut;
    out.pos = vec4<f32>(positions[vid], 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let coord = vec2<i32>(in.pos.xy);
    let accum = textureLoad(accum_tex, coord, 0);
    let reveal = textureLoad(reveal_tex, coord, 0).r;
    if reveal >= 0.9999 {
        discard;
    }
    let avg = accum.rgb / max(accum.a, 1e-4);
    return vec4<f32>(avg, 1.0 - reveal);
}
