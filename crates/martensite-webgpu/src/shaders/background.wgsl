// Fullscreen background shader: corner gradient, 2D image, or procedural skydome (#634).

struct BackgroundUniforms {
    corner_tl: vec4<f32>,
    corner_tr: vec4<f32>,
    corner_bl: vec4<f32>,
    corner_br: vec4<f32>,
    mode_sun: vec4<f32>,
};

@group(0) @binding(0) var<uniform> bg: BackgroundUniforms;
@group(0) @binding(1) var bg_tex: texture_2d<f32>;
@group(0) @binding(2) var bg_sampler: sampler;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0),
    );
    let p = positions[vid];
    var out: VsOut;
    out.pos = vec4<f32>(p, 0.999999, 1.0);
    out.uv = vec2<f32>(p.x * 0.5 + 0.5, 0.5 - p.y * 0.5);
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let mode = u32(bg.mode_sun.x + 0.5);
    let img = textureSample(bg_tex, bg_sampler, in.uv);
    if mode == 1u {
        return img;
    }
    if mode == 2u {
        let horizon = clamp(1.0 - in.uv.y, 0.0, 1.0);
        let zenith = vec3<f32>(0.18, 0.38, 0.72);
        let mist = vec3<f32>(0.76, 0.84, 0.94);
        let sky = mix(mist, zenith, pow(horizon, 0.65));
        return vec4<f32>(sky, 1.0);
    }
    let top = mix(bg.corner_tl, bg.corner_tr, in.uv.x);
    let bot = mix(bg.corner_bl, bg.corner_br, in.uv.x);
    return mix(top, bot, in.uv.y);
}
