// Instanced screen-space quad expansion for wide and stippled CAD lines (#634).
// Works identically across Chromium, Firefox, and macOS Safari without wide_lines.

struct LineSegment {
    p0: vec4<f32>,
    p1: vec4<f32>,
    c0: vec4<f32>,
    c1: vec4<f32>,
};

struct LineUniforms {
    params: vec4<f32>,   // [viewport_w, viewport_h, width_px, stipple_factor]
    stipple: vec4<u32>,  // [pattern_u16, 0, 0, 0]
};

@group(0) @binding(0) var<uniform> line_u: LineUniforms;
@group(0) @binding(1) var<storage, read> segments: array<LineSegment>;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) dist_px: f32,
};

@vertex
fn vs_main(
    @builtin(vertex_index) vid: u32,
    @builtin(instance_index) iid: u32,
) -> VsOut {
    let seg = segments[iid];
    var corners = array<vec2<f32>, 6>(
        vec2<f32>(0.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(0.0, -1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(0.0, 1.0),
    );
    let corner = corners[vid];
    let ndc0 = seg.p0.xy / max(abs(seg.p0.w), 1e-6);
    let ndc1 = seg.p1.xy / max(abs(seg.p1.w), 1e-6);
    let vp = max(line_u.params.xy, vec2<f32>(1.0, 1.0));
    let s0 = ndc0 * vp * 0.5;
    let s1 = ndc1 * vp * 0.5;
    let delta = s1 - s0;
    let seg_len = max(length(delta), 1e-4);
    let dir = delta / seg_len;
    let perp = vec2<f32>(-dir.y, dir.x);
    let half_w = max(line_u.params.z, 1.0) * 0.5;

    let clip_base = mix(seg.p0, seg.p1, corner.x);
    let offset_ndc = (perp * (corner.y * half_w)) * (2.0 / vp);
    var out: VsOut;
    out.pos = vec4<f32>(clip_base.xy + offset_ndc * clip_base.w, clip_base.z, clip_base.w);
    out.color = mix(seg.c0, seg.c1, corner.x);
    out.dist_px = corner.x * seg_len;
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let pattern = line_u.stipple.x & 0xffffu;
    if pattern != 0xffffu {
        let factor = max(line_u.params.w, 1.0);
        let bit = u32(floor(in.dist_px / factor)) % 16u;
        if ((pattern >> bit) & 1u) == 0u {
            discard;
        }
    }
    return in.color;
}
