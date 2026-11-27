// WebGPU Core compute OpenPBR ray & progressive path tracer (#635).
// Traverses BVH + triangle + OpenPBR material storage buffers and writes Rgba16Float
// ping-pong accumulation textures without float32-filterable or fragment atomics.

struct RaytraceUniforms {
    eye_sample: vec4<f32>,
    ray_tl_bounces: vec4<f32>,
    ray_dx_width: vec4<f32>,
    ray_dy_height: vec4<f32>,
    background: vec4<f32>,
    counts: vec4<u32>,
};

struct RayTriangle {
    v0_mat: vec4<f32>,
    e1: vec4<f32>,
    e2: vec4<f32>,
    normal: vec4<f32>,
};

struct RayBvhNode {
    min_left: vec4<f32>,
    max_right: vec4<f32>,
};

struct RayMaterial {
    base_color: vec4<f32>,
    emissive_metallic: vec4<f32>,
    pbr_params: vec4<f32>,
};

struct RayLight {
    color_intensity: vec4<f32>,
    position_kind: vec4<f32>,
    direction_cutoff: vec4<f32>,
    attenuation: vec4<f32>,
};

@group(0) @binding(0) var<uniform> rt: RaytraceUniforms;
@group(0) @binding(1) var<storage, read> triangles: array<RayTriangle>;
@group(0) @binding(2) var<storage, read> bvh_nodes: array<RayBvhNode>;
@group(0) @binding(3) var<storage, read> materials: array<RayMaterial>;
@group(0) @binding(4) var<storage, read> lights: array<RayLight>;
@group(0) @binding(5) var prev_accum: texture_2d<f32>;
@group(0) @binding(6) var next_accum: texture_storage_2d<rgba16float, write>;

fn hash_u32(v_in: u32) -> u32 {
    var v = v_in;
    v = (v ^ 61u) ^ (v >> 16u);
    v = v * 9u;
    v = v ^ (v >> 4u);
    v = v * 0x27d4eb2du;
    v = v ^ (v >> 15u);
    return v;
}

fn rand01(seed: ptr<function, u32>) -> f32 {
    *seed = hash_u32(*seed);
    return f32(*seed & 0x00ffffffu) / 16777216.0;
}

fn intersect_aabb(ro: vec3<f32>, inv_d: vec3<f32>, bmin: vec3<f32>, bmax: vec3<f32>, t_max: f32) -> bool {
    let t0 = (bmin - ro) * inv_d;
    let t1 = (bmax - ro) * inv_d;
    let tmin = min(t0, t1);
    let tmax = max(t0, t1);
    let enter = max(max(tmin.x, tmin.y), max(tmin.z, 0.0));
    let exit = min(min(tmax.x, tmax.y), min(tmax.z, t_max));
    return exit >= enter;
}

struct HitInfo {
    t: f32,
    normal: vec3<f32>,
    mat_idx: u32,
};

fn trace_scene(ro: vec3<f32>, rd: vec3<f32>, t_max: f32) -> HitInfo {
    var best = HitInfo(t_max, vec3<f32>(0.0, 0.0, 1.0), 0xffffffffu);
    let tri_count = rt.counts.x;
    let node_count = rt.counts.y;
    let safe_rd = select(rd, vec3<f32>(1e-8), abs(rd) < vec3<f32>(1e-8));
    let inv_d = vec3<f32>(1.0) / safe_rd;

    if node_count > 0u {
        var stack: array<u32, 32>;
        var sp: i32 = 0;
        stack[0] = 0u;
        sp = 1;
        loop {
            if sp <= 0 {
                break;
            }
            sp = sp - 1;
            let node = bvh_nodes[stack[sp]];
            if !intersect_aabb(ro, inv_d, node.min_left.xyz, node.max_right.xyz, best.t) {
                continue;
            }
            let left_or_first = bitcast<u32>(node.min_left.w);
            let right_or_count = bitcast<u32>(node.max_right.w);
            if (right_or_count & 0x80000000u) != 0u {
                let count = right_or_count & 0x7fffffffu;
                for (var i: u32 = 0u; i < count; i = i + 1u) {
                    let tri = triangles[left_or_first + i];
                    let pvec = cross(rd, tri.e2.xyz);
                    let det = dot(tri.e1.xyz, pvec);
                    if abs(det) < 1e-7 {
                        continue;
                    }
                    let inv_det = 1.0 / det;
                    let tvec = ro - tri.v0_mat.xyz;
                    let u = dot(tvec, pvec) * inv_det;
                    if u < 0.0 || u > 1.0 {
                        continue;
                    }
                    let qvec = cross(tvec, tri.e1.xyz);
                    let v = dot(rd, qvec) * inv_det;
                    if v < 0.0 || u + v > 1.0 {
                        continue;
                    }
                    let t = dot(tri.e2.xyz, qvec) * inv_det;
                    if t > 1e-4 && t < best.t {
                        best.t = t;
                        best.normal = select(-tri.normal.xyz, tri.normal.xyz, dot(tri.normal.xyz, rd) < 0.0);
                        best.mat_idx = bitcast<u32>(tri.v0_mat.w);
                    }
                }
            } else if sp + 2 <= 32 {
                stack[sp] = left_or_first;
                stack[sp + 1] = right_or_count;
                sp = sp + 2;
            }
        }
        return best;
    }

    for (var i: u32 = 0u; i < tri_count; i = i + 1u) {
        let tri = triangles[i];
        let pvec = cross(rd, tri.e2.xyz);
        let det = dot(tri.e1.xyz, pvec);
        if abs(det) < 1e-7 {
            continue;
        }
        let inv_det = 1.0 / det;
        let tvec = ro - tri.v0_mat.xyz;
        let u = dot(tvec, pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            continue;
        }
        let qvec = cross(tvec, tri.e1.xyz);
        let v = dot(rd, qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            continue;
        }
        let t = dot(tri.e2.xyz, qvec) * inv_det;
        if t > 1e-4 && t < best.t {
            best.t = t;
            best.normal = select(-tri.normal.xyz, tri.normal.xyz, dot(tri.normal.xyz, rd) < 0.0);
            best.mat_idx = bitcast<u32>(tri.v0_mat.w);
        }
    }
    return best;
}

@compute @workgroup_size(8, 8, 1)
fn cs_main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let width = u32(rt.ray_dx_width.w + 0.5);
    let height = u32(rt.ray_dy_height.w + 0.5);
    if gid.x >= width || gid.y >= height {
        return;
    }
    let sample_idx = u32(rt.eye_sample.w + 0.5);
    var seed = gid.x * 1973u + gid.y * 9277u + sample_idx * 26699u + 1u;
    let jx = select(0.5, rand01(&seed), sample_idx > 0u);
    let jy = select(0.5, rand01(&seed), sample_idx > 0u);
    let u = (f32(gid.x) + jx) / max(f32(width), 1.0);
    let v = (f32(gid.y) + jy) / max(f32(height), 1.0);

    var ro = rt.eye_sample.xyz;
    var rd = normalize(rt.ray_tl_bounces.xyz + rt.ray_dx_width.xyz * u + rt.ray_dy_height.xyz * v);
    let max_bounces = max(u32(rt.ray_tl_bounces.w + 0.5), 1u);
    var throughput = vec3<f32>(1.0);
    var radiance = vec3<f32>(0.0);

    for (var bounce: u32 = 0u; bounce < max_bounces; bounce = bounce + 1u) {
        let hit = trace_scene(ro, rd, 1e20);
        if hit.mat_idx == 0xffffffffu {
            radiance += throughput * rt.background.rgb;
            break;
        }
        let mat = materials[hit.mat_idx];
        let p = ro + rd * hit.t;
        let n = normalize(hit.normal);
        radiance += throughput * mat.emissive_metallic.xyz;

        // Direct lighting with shadow rays
        let light_count = rt.counts.z;
        let metallic = clamp(mat.emissive_metallic.w, 0.0, 1.0);
        let roughness = clamp(mat.pbr_params.x, 0.04, 1.0);
        for (var li: u32 = 0u; li < light_count; li = li + 1u) {
            let light = lights[li];
            let kind = u32(light.position_kind.w + 0.5);
            var l = normalize(light.position_kind.xyz);
            var l_dist = 1e19;
            if kind != 0u {
                let d = light.position_kind.xyz - p;
                l_dist = length(d);
                l = d / max(l_dist, 1e-4);
            }
            let ndotl = max(dot(n, l), 0.0);
            if ndotl > 0.0 {
                let shadow_hit = trace_scene(p + n * 1e-3, l, l_dist);
                if shadow_hit.mat_idx == 0xffffffffu {
                    let h = normalize(l - rd);
                    let spec_p = pow(max(dot(n, h), 0.0), mix(64.0, 4.0, roughness));
                    let f0 = mix(vec3<f32>(0.04), mat.base_color.rgb, metallic);
                    let brdf = (1.0 - metallic) * mat.base_color.rgb / 3.14159265 + f0 * spec_p * 0.25;
                    radiance += throughput * light.color_intensity.xyz * light.color_intensity.w * brdf * ndotl;
                }
            }
        }
        // Ambient fill on primary bounce if no lights present
        if light_count == 0u && bounce == 0u {
            radiance += throughput * mat.base_color.rgb * (0.25 + 0.75 * max(dot(n, normalize(vec3<f32>(0.4, 0.6, 0.7))), 0.0));
            break;
        }
        // Specular/diffuse continuation bounce
        let refl = reflect(rd, n);
        let rx = rand01(&seed) * 2.0 - 1.0;
        let ry = rand01(&seed) * 2.0 - 1.0;
        let rz = rand01(&seed) * 2.0 - 1.0;
        let hemi = normalize(n + vec3<f32>(rx, ry, rz) * 0.85);
        rd = normalize(mix(refl, hemi, roughness));
        ro = p + n * 1e-3;
        throughput *= mat.base_color.rgb * mix(0.65, 0.92, metallic);
    }

    var accum = vec4<f32>(radiance, 1.0);
    if sample_idx > 0u {
        let prev = textureLoad(prev_accum, vec2<i32>(gid.xy), 0);
        let inv_n = 1.0 / f32(sample_idx + 1u);
        accum = vec4<f32>(mix(prev.rgb, radiance, inv_n), 1.0);
    }
    textureStore(next_accum, vec2<i32>(gid.xy), accum);
}
