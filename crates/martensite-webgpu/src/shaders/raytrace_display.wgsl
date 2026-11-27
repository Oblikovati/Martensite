// Display pass copying Rgba16Float compute raytrace accumulation to the scene color target (#635).

@group(0) @binding(0) var accum_tex: texture_2d<f32>;

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
    let hdr = textureLoad(accum_tex, coord, 0);
    return vec4<f32>(clamp(hdr.rgb, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
}
