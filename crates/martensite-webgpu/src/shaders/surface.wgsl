// Unified Martensite WebGPU Core surface shader (#634).
// Supports Unlit, Blinn-Phong, OpenPBR Surface v1.1, arbitrary fragment clipping chains,
// barycentric mesh edges, Depth32Float PCF shadows, and WBOIT dual-target output.

override SHADING_MODE: u32 = 0u; // 0 = Unlit, 1 = Phong, 2 = OpenPBR v1.1
override USE_WBOIT: bool = false;

struct DrawUniforms {
    clip_matrix: mat4x4<f32>,
    world_matrix: mat4x4<f32>,
    normal_matrix: mat4x4<f32>,
    eye_position: vec4<f32>,
    base_color: vec4<f32>,
    specular_params: vec4<f32>,
    emissive_metallic: vec4<f32>,
    pbr_params: vec4<f32>,
    counts_flags: vec4<u32>,
    edge_color: vec4<f32>,
    viewport_params: vec4<f32>,
};

struct GpuLight {
    color_intensity: vec4<f32>,
    position_kind: vec4<f32>,
    direction_cutoff: vec4<f32>,
    attenuation: vec4<f32>,
};

struct SceneLightsAndClips {
    ambient: vec4<f32>,
    shadow_matrix: mat4x4<f32>,
    shadow_env: vec4<f32>,
    lights: array<GpuLight, 16>,
    clip_planes: array<vec4<f32>, 8>,
    clip_chains: array<vec4<u32>, 2>,
};

@group(0) @binding(0) var<uniform> draw_u: DrawUniforms;
@group(0) @binding(1) var<uniform> scene_u: SceneLightsAndClips;
@group(1) @binding(0) var surface_tex: texture_2d<f32>;
@group(1) @binding(1) var surface_sampler: sampler;
@group(1) @binding(2) var shadow_tex: texture_depth_2d;
@group(1) @binding(3) var shadow_sampler: sampler_comparison;

struct VsIn {
    @location(0) position: vec4<f32>,
    @location(1) color: vec4<f32>,
    @location(2) normal_edge: vec4<f32>,
    @location(3) uv_bary: vec4<f32>,
};

struct VsOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) color: vec4<f32>,
    @location(3) uv: vec2<f32>,
    @location(4) bary: vec3<f32>,
    @location(5) shadow_coord: vec4<f32>,
};

@vertex
fn vs_main(input: VsIn) -> VsOut {
    var out: VsOut;
    out.clip_pos = draw_u.clip_matrix * input.position;
    let wp = draw_u.world_matrix * input.position;
    out.world_pos = wp.xyz;
    let wn = (draw_u.normal_matrix * vec4<f32>(input.normal_edge.xyz, 0.0)).xyz;
    out.world_normal = wn;
    out.color = input.color;
    out.uv = input.uv_bary.xy;
    let b0 = input.uv_bary.z;
    let b1 = input.uv_bary.w;
    out.bary = vec3<f32>(b0, b1, max(0.0, 1.0 - b0 - b1));
    out.shadow_coord = scene_u.shadow_matrix * vec4<f32>(wp.xyz, 1.0);
    return out;
}

fn chain_id_at(idx: u32) -> u32 {
    let vec_i = idx / 4u;
    let comp = idx % 4u;
    let v = scene_u.clip_chains[vec_i];
    if comp == 0u { return v.x; }
    if comp == 1u { return v.y; }
    if comp == 2u { return v.z; }
    return v.w;
}

fn is_clipped(world_pos: vec3<f32>, clip_count: u32) -> bool {
    if clip_count == 0u {
        return false;
    }
    // Each distinct chain_id is an AND-chain (clipped only if every plane in the chain clips).
    // Across distinct chains, any clipped chain discards the fragment (OR of chains).
    var i: u32 = 0u;
    loop {
        if i >= clip_count {
            break;
        }
        let cid = chain_id_at(i);
        var chain_all_outside = true;
        var j = i;
        loop {
            if j >= clip_count || chain_id_at(j) != cid {
                break;
            }
            let eq = scene_u.clip_planes[j];
            if dot(eq.xyz, world_pos) + eq.w >= 0.0 {
                chain_all_outside = false;
            }
            j = j + 1u;
        }
        if chain_all_outside {
            return true;
        }
        i = j;
    }
    return false;
}

fn sample_shadow(shadow_coord: vec4<f32>) -> f32 {
    let safe_w = select(1.0, shadow_coord.w, shadow_coord.w > 0.0);
    let ndc = shadow_coord.xyz / safe_w;
    let uv = vec2<f32>(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5);
    let in_bounds = scene_u.shadow_env.y >= 0.5
        && shadow_coord.w > 0.0
        && uv.x >= 0.0
        && uv.x <= 1.0
        && uv.y >= 0.0
        && uv.y <= 1.0
        && ndc.z >= 0.0
        && ndc.z <= 1.0;
    let depth_ref = clamp(ndc.z - scene_u.shadow_env.x, 0.0, 1.0);
    let vis = textureSampleCompare(
        shadow_tex,
        shadow_sampler,
        clamp(uv, vec2<f32>(0.0), vec2<f32>(1.0)),
        depth_ref,
    );
    return select(1.0, vis, in_bounds);
}

fn ggx_d(ndoth: f32, alpha: f32) -> f32 {
    let a2 = alpha * alpha;
    let d = ndoth * ndoth * (a2 - 1.0) + 1.0;
    return a2 / max(3.14159265 * d * d, 1e-6);
}

fn ggx_g1(ndotv: f32, alpha: f32) -> f32 {
    let k = (alpha + 1.0) * (alpha + 1.0) * 0.125;
    return ndotv / max(ndotv * (1.0 - k) + k, 1e-6);
}

fn fresnel_schlick(cos_theta: f32, f0: vec3<f32>) -> vec3<f32> {
    let m = clamp(1.0 - cos_theta, 0.0, 1.0);
    let m2 = m * m;
    return f0 + (vec3<f32>(1.0) - f0) * (m2 * m2 * m);
}

fn eval_lit(
    world_pos: vec3<f32>,
    n_in: vec3<f32>,
    front_facing: bool,
    base_rgb: vec3<f32>,
    shadow: f32,
) -> vec3<f32> {
    var n = normalize(select(vec3<f32>(0.0, 0.0, 1.0), n_in, length(n_in) > 1e-6));
    if !front_facing {
        n = -n;
    }
    let v = select(
        normalize(draw_u.eye_position.xyz - world_pos),
        normalize(draw_u.eye_position.xyz),
        draw_u.eye_position.w == 0.0,
    );
    let ndotv = max(dot(n, v), 1e-4);
    let light_count = min(draw_u.counts_flags.y, 16u);
    var accum = draw_u.emissive_metallic.xyz;

    if SHADING_MODE == 1u {
        // Blinn-Phong
        accum += scene_u.ambient.xyz * base_rgb;
        let shininess = max(draw_u.specular_params.w, 1.0);
        for (var i: u32 = 0u; i < light_count; i = i + 1u) {
            let light = scene_u.lights[i];
            let kind = u32(light.position_kind.w + 0.5);
            var l = normalize(light.position_kind.xyz);
            var atten = light.color_intensity.w;
            if kind != 0u {
                let delta = light.position_kind.xyz - world_pos;
                let dist = max(length(delta), 1e-4);
                l = delta / dist;
                atten = atten / max(light.attenuation.x + light.attenuation.y * dist, 1e-4);
            }
            let ndotl = max(dot(n, l), 0.0);
            if ndotl > 0.0 {
                let h = normalize(l + v);
                let spec = pow(max(dot(n, h), 0.0), shininess);
                let vis = select(1.0, shadow, i == 0u);
                accum += vis * atten * light.color_intensity.xyz * (base_rgb * ndotl + draw_u.specular_params.xyz * spec);
            }
        }
        return accum;
    }

    // OpenPBR Surface v1.1 metallic-roughness + clearcoat
    let metallic = clamp(draw_u.emissive_metallic.w, 0.0, 1.0);
    let roughness = clamp(draw_u.pbr_params.x, 0.04, 1.0);
    let alpha = roughness * roughness;
    let coat_weight = clamp(draw_u.pbr_params.z, 0.0, 1.0);
    let coat_alpha = clamp(draw_u.pbr_params.w, 0.04, 1.0) * clamp(draw_u.pbr_params.w, 0.04, 1.0);
    let ior = max(draw_u.specular_params.w, 1.0);
    let f0_d = pow((ior - 1.0) / (ior + 1.0), 2.0);
    let f0 = mix(vec3<f32>(f0_d), base_rgb, metallic);
    let kd = (1.0 - metallic) * base_rgb / 3.14159265;

    // Ambient / IBL fill
    accum += scene_u.ambient.xyz * scene_u.shadow_env.z * mix(base_rgb, f0, 0.35 * metallic);

    for (var i: u32 = 0u; i < light_count; i = i + 1u) {
        let light = scene_u.lights[i];
        let kind = u32(light.position_kind.w + 0.5);
        var l = normalize(light.position_kind.xyz);
        var atten = light.color_intensity.w;
        if kind != 0u {
            let delta = light.position_kind.xyz - world_pos;
            let dist = max(length(delta), 1e-4);
            l = delta / dist;
            atten = atten / max(1.0 + light.attenuation.y * dist * dist, 1e-4);
        }
        let ndotl = max(dot(n, l), 0.0);
        if ndotl > 0.0 {
            let h = normalize(l + v);
            let ndoth = max(dot(n, h), 0.0);
            let vdoth = max(dot(v, h), 0.0);
            let d = ggx_d(ndoth, alpha);
            let g = ggx_g1(ndotv, alpha) * ggx_g1(ndotl, alpha);
            let f = fresnel_schlick(vdoth, f0);
            let spec = (d * g * f) / max(4.0 * ndotv * ndotl, 1e-5);
            var coat = vec3<f32>(0.0);
            if coat_weight > 0.001 {
                let cd = ggx_d(ndoth, coat_alpha);
                let cg = ggx_g1(ndotv, coat_alpha) * ggx_g1(ndotl, coat_alpha);
                let cf = fresnel_schlick(vdoth, vec3<f32>(0.04));
                coat = coat_weight * (cd * cg * cf) / max(4.0 * ndotv * ndotl, 1e-5);
            }
            let vis = select(1.0, shadow, i == 0u);
            accum += vis * atten * light.color_intensity.xyz * ((vec3<f32>(1.0) - f) * kd + spec + coat) * ndotl;
        }
    }
    return accum;
}

fn shade_fragment(in: VsOut, front_facing: bool) -> vec4<f32> {
    // Evaluate all implicit-derivative builtins in uniform control flow before any discard.
    let texel = textureSample(surface_tex, surface_sampler, in.uv);
    let shadow = sample_shadow(in.shadow_coord);
    let fw = fwidth(in.bary);
    if is_clipped(in.world_pos, draw_u.counts_flags.z) {
        discard;
    }
    let flags = draw_u.counts_flags.w;
    var base = select(in.color, draw_u.base_color * in.color, SHADING_MODE != 0u);
    if (flags & 1u) != 0u {
        base = base * texel;
    }
    if (flags & 4u) != 0u && base.a < draw_u.pbr_params.y {
        discard;
    }
    var rgb = base.rgb;
    if SHADING_MODE != 0u {
        rgb = eval_lit(in.world_pos, in.world_normal, front_facing, base.rgb, shadow);
    }
    if (flags & 2u) != 0u {
        let edge_dist = min(min(in.bary.x / max(fw.x, 1e-5), in.bary.y / max(fw.y, 1e-5)), in.bary.z / max(fw.z, 1e-5));
        let factor = 1.0 - smoothstep(0.0, max(draw_u.viewport_params.z, 1.0), edge_dist);
        rgb = mix(rgb, draw_u.edge_color.rgb, factor * draw_u.edge_color.a);
    }
    return vec4<f32>(rgb, base.a);
}

@fragment
fn fs_main(in: VsOut, @builtin(front_facing) front_facing: bool) -> @location(0) vec4<f32> {
    return shade_fragment(in, front_facing);
}

struct OitOut {
    @location(0) accum: vec4<f32>,
    @location(1) reveal: vec4<f32>,
};

@fragment
fn fs_wboit(in: VsOut, @builtin(front_facing) front_facing: bool) -> OitOut {
    let color = shade_fragment(in, front_facing);
    let a = clamp(color.a, 0.0, 1.0);
    let z = clamp(in.clip_pos.z, 0.0, 1.0);
    let weight = clamp(a * max(1e-2, 3e3 * pow(1.0 - z, 3.0)), 1e-2, 3e3);
    var out: OitOut;
    out.accum = vec4<f32>(color.rgb * a, a) * weight;
    out.reveal = vec4<f32>(a, 0.0, 0.0, 1.0);
    return out;
}
