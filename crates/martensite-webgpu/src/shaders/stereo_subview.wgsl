// Stereo pair composition (anaglyph, interlaced, side-by-side) and subview blit pass (#635).

struct StereoSubviewUniforms {
    mode_flags: vec4<u32>, // [mode, flip_y, 0, 0]
    rect_uv: vec4<f32>,    // [dst_x, dst_y, dst_w, dst_h]
};

@group(0) @binding(0) var<uniform> comp_u: StereoSubviewUniforms;
@group(0) @binding(1) var left_tex: texture_2d<f32>;
@group(0) @binding(2) var right_tex: texture_2d<f32>;
@group(0) @binding(3) var linear_sampler: sampler;

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
    out.pos = vec4<f32>(p, 0.0, 1.0);
    out.uv = vec2<f32>(p.x * 0.5 + 0.5, 0.5 - p.y * 0.5);
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let mode = comp_u.mode_flags.x;
    var uv = in.uv;
    if comp_u.mode_flags.y != 0u {
        uv.y = 1.0 - uv.y;
    }
    let l = textureSample(left_tex, linear_sampler, uv);
    let r = textureSample(right_tex, linear_sampler, uv);
    let rx = (uv.x - comp_u.rect_uv.x) / max(comp_u.rect_uv.z, 1e-5);
    let ry = (uv.y - comp_u.rect_uv.y) / max(comp_u.rect_uv.w, 1e-5);
    let sub_sample = textureSample(left_tex, linear_sampler, clamp(vec2<f32>(rx, ry), vec2<f32>(0.0), vec2<f32>(1.0)));
    let half_uv = vec2<f32>(fract(uv.x * 2.0), uv.y);
    let ls = textureSample(left_tex, linear_sampler, half_uv);
    let rs = textureSample(right_tex, linear_sampler, half_uv);
    if mode == 0u {
        // Subview blit inside rect_uv
        if rx < 0.0 || rx > 1.0 || ry < 0.0 || ry > 1.0 {
            discard;
        }
        return sub_sample;
    }
    if mode == 1u {
        return vec4<f32>(l.r, r.g, r.b, 1.0); // Red-Cyan anaglyph
    }
    if mode == 2u {
        return vec4<f32>(r.r, l.g, r.b, 1.0); // Green-Magenta anaglyph
    }
    if mode == 3u {
        return vec4<f32>(l.r, l.g, r.b, 1.0); // Yellow-Blue anaglyph
    }
    if mode == 4u {
        let row = u32(in.pos.y) & 1u;
        return select(l, r, row == 1u); // Row interlaced
    }
    if mode == 5u {
        let col = u32(in.pos.x) & 1u;
        return select(l, r, col == 1u); // Column interlaced
    }
    // Side-by-side (mode 6)
    let is_right = uv.x >= 0.5;
    return select(ls, rs, is_right);
}
