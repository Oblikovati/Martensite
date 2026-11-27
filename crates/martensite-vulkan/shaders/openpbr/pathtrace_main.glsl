// Shared OpenPBR Surface v1.1 Compute Path Tracer body.
// Included by `pathtrace_hw.comp` (with `OPENPBR_HW_RAY_QUERY` defined)
// and `pathtrace_sw.comp` (software BVH SSBO traversal).
layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

#include "openpbr_bsdf.glsl"

struct PtVertex {
    vec4 pos_u;     // xyz = position, w = u
    vec4 norm_v;    // xyz = normal, w = v
};

struct PtTriangle {
    uvec4 idx_mat;  // xyz = vertex indices, w = material index
};

struct PtBvhNode {
    vec4 bmin_left; // xyz = aabb min, w = bitcast(left_or_first)
    vec4 bmax_cnt;  // xyz = aabb max, w = bitcast(right_or_count; >=0 leaf count, <0 internal)
};

struct PtLight {
    uvec4 kind;     // x: 1=dir, 2=point, 3=spot
    vec4 color;     // rgb=color, a=intensity
    vec4 position;  // xyz=pos (or dir for directional)
    vec4 direction; // xyz=spot dir, w=range
    vec4 params;    // x=const, y=linear, z=angle, w=exponent
};

layout(std140, set = 0, binding = 0) uniform PathtraceCameraUbo {
    vec4 eyeOrigin;         // xyz=origin, w=apertureRadius
    vec4 eyeView;           // xyz=forward, w=focalDist
    vec4 eyeRight;          // xyz=right * scaleX, w=tanHalfFovX
    vec4 eyeUp;             // xyz=up * scaleY, w=tanHalfFovY
    mat4 viewProj;
    mat4 prevViewProj;
    vec4 backTop;           // rgb=top gradient
    vec4 backBottom;        // rgb=bottom gradient
    vec4 ambient;           // rgb=ambient factor, w=sceneEpsilon
    uvec4 counts;           // x=width, y=height, z=lightCount, w=triangleCount
    uvec4 settings;         // x=maxBounces, y=frameSeed, z=accumFrames, w=flags
    vec4 misc;              // x=radianceClamp, y=envIntensity, z=nearPlane, w=farPlane
} ubo;

layout(std430, set = 0, binding = 1) readonly buffer VertexBuf { PtVertex vertices[]; };
layout(std430, set = 0, binding = 2) readonly buffer TriangleBuf { PtTriangle triangles[]; };
layout(std430, set = 0, binding = 3) readonly buffer MaterialBuf { OpenPbrGpuMaterial materials[]; };
layout(std430, set = 0, binding = 4) readonly buffer LightBuf { PtLight lights[]; };
layout(std430, set = 0, binding = 5) readonly buffer BvhBuf { PtBvhNode bvhNodes[]; };

#ifdef OPENPBR_HW_RAY_QUERY
layout(set = 0, binding = 6) uniform accelerationStructureEXT topLevelAS;
#endif

// Primary-hit G-buffer & radiance outputs for SVGF spatiotemporal denoiser / direct accumulation.
layout(set = 1, binding = 0, rgba16f) writeonly uniform image2D outRadiance;
layout(set = 1, binding = 1, rgba16f) writeonly uniform image2D outAlbedoRoughness;
layout(set = 1, binding = 2, rgba32f) writeonly uniform image2D outNormalDepth;
layout(set = 1, binding = 3, rgba32f) writeonly uniform image2D outMotionId;
layout(set = 1, binding = 4, rgba16f) writeonly uniform image2D outEmission;

// OpenPBR LUTs & HDR environment (same Set 4 layout bound at Set 2 in compute).
layout(set = 2, binding = 0) uniform sampler2D openpbrGgxLut;
layout(set = 2, binding = 1) uniform sampler2D pbrDiffuseSH;
layout(set = 2, binding = 2) uniform samplerCube pbrSpecularMap;
layout(set = 2, binding = 3) uniform sampler3D openpbrDielectricLut;
layout(set = 2, binding = 4) uniform sampler2D openpbrSheenLut;
layout(set = 2, binding = 5) uniform sampler2D openpbrThinFilmLut;

struct HitInfo {
    bool hit;
    float t;
    uint triIndex;
    uint matIndex;
    vec2 bary;
    vec3 pos;
    vec3 geomNormal;
    vec3 shadNormal;
    vec2 uv;
    bool backfacing;
};

#ifndef OPENPBR_HW_RAY_QUERY
bool intersectAabb(vec3 origin, vec3 invDir, vec3 bmin, vec3 bmax, float tMax, out float tNear) {
    vec3 t0 = (bmin - origin) * invDir;
    vec3 t1 = (bmax - origin) * invDir;
    vec3 tmin = min(t0, t1);
    vec3 tmax = max(t0, t1);
    tNear = max(max(tmin.x, tmin.y), max(tmin.z, 0.0));
    float tFar = min(min(tmax.x, tmax.y), min(tmax.z, tMax));
    return tFar >= tNear;
}

bool intersectTriangle(vec3 ro, vec3 rd, uint triIdx, float tMin, float tMax, out float tHit, out vec2 bary) {
    uvec3 idx = triangles[triIdx].idx_mat.xyz;
    vec3 v0 = vertices[idx.x].pos_u.xyz;
    vec3 v1 = vertices[idx.y].pos_u.xyz;
    vec3 v2 = vertices[idx.z].pos_u.xyz;
    vec3 e1 = v1 - v0;
    vec3 e2 = v2 - v0;
    vec3 p = cross(rd, e2);
    float det = dot(e1, p);
    if (abs(det) < 1e-8) return false;
    float invDet = 1.0 / det;
    vec3 s = ro - v0;
    float u = dot(s, p) * invDet;
    if (u < 0.0 || u > 1.0) return false;
    vec3 q = cross(s, e1);
    float v = dot(rd, q) * invDet;
    if (v < 0.0 || (u + v) > 1.0) return false;
    float t = dot(e2, q) * invDet;
    if (t <= tMin || t >= tMax) return false;
    tHit = t;
    bary = vec2(u, v);
    return true;
}
#endif

HitInfo traceScene(vec3 ro, vec3 rd, float tMin, float tMax) {
    HitInfo info;
    info.hit = false;
    info.t = tMax;
    info.triIndex = 0u;
    info.matIndex = 0u;
    info.bary = vec2(0.0);

#ifdef OPENPBR_HW_RAY_QUERY
    if (ubo.counts.w == 0u) {
        return info;
    }
    rayQueryEXT rq;
    rayQueryInitializeEXT(rq, topLevelAS, gl_RayFlagsOpaqueEXT, 0xFFu, ro, tMin, rd, tMax);
    while (rayQueryProceedEXT(rq)) {
        if (rayQueryGetIntersectionTypeEXT(rq, false) == gl_RayQueryCandidateIntersectionTriangleEXT) {
            rayQueryConfirmIntersectionEXT(rq);
        }
    }
    if (rayQueryGetIntersectionTypeEXT(rq, true) == gl_RayQueryCommittedIntersectionTriangleEXT) {
        info.hit = true;
        info.t = rayQueryGetIntersectionTEXT(rq, true);
        info.triIndex = uint(rayQueryGetIntersectionPrimitiveIndexEXT(rq, true));
        info.bary = rayQueryGetIntersectionBarycentricsEXT(rq, true);
    }
#else
    if (ubo.counts.w == 0u) {
        return info;
    }
    vec3 invDir = 1.0 / mix(rd, vec3(1e-8), lessThan(abs(rd), vec3(1e-8)));
    int stack[64];
    int sp = 0;
    stack[sp++] = 0;
    while (sp > 0) {
        int nodeIdx = stack[--sp];
        PtBvhNode node = bvhNodes[nodeIdx];
        float tNear;
        if (!intersectAabb(ro, invDir, node.bmin_left.xyz, node.bmax_cnt.xyz, info.t, tNear)) {
            continue;
        }
        int a = floatBitsToInt(node.bmin_left.w);
        int b = floatBitsToInt(node.bmax_cnt.w);
        if (b > 0) {
            for (int i = 0; i < b; ++i) {
                float tHit;
                vec2 bary;
                uint triIdx = uint(a + i);
                if (intersectTriangle(ro, rd, triIdx, tMin, info.t, tHit, bary)) {
                    info.hit = true;
                    info.t = tHit;
                    info.triIndex = triIdx;
                    info.bary = bary;
                }
            }
        } else {
            int right = -b;
            if (sp < 62) {
                stack[sp++] = right;
                stack[sp++] = a;
            }
        }
    }
#endif

    if (info.hit) {
        uvec4 tm = triangles[info.triIndex].idx_mat;
        info.matIndex = tm.w;
        PtVertex v0 = vertices[tm.x];
        PtVertex v1 = vertices[tm.y];
        PtVertex v2 = vertices[tm.z];
        float w0 = 1.0 - info.bary.x - info.bary.y;
        info.pos = ro + rd * info.t;
        vec3 gn = normalize(cross(v1.pos_u.xyz - v0.pos_u.xyz, v2.pos_u.xyz - v0.pos_u.xyz));
        vec3 sn = normalize(v0.norm_v.xyz * w0 + v1.norm_v.xyz * info.bary.x + v2.norm_v.xyz * info.bary.y);
        if (dot(sn, sn) < 1e-5) sn = gn;
        info.backfacing = dot(gn, rd) > 0.0;
        info.geomNormal = info.backfacing ? -gn : gn;
        info.shadNormal = (dot(sn, info.geomNormal) < 0.0) ? -sn : sn;
        info.uv = vec2(v0.pos_u.w, v0.norm_v.w) * w0
                + vec2(v1.pos_u.w, v1.norm_v.w) * info.bary.x
                + vec2(v2.pos_u.w, v2.norm_v.w) * info.bary.y;
    }
    return info;
}

bool traceShadowOccluded(vec3 ro, vec3 rd, float tMin, float tMax) {
#ifdef OPENPBR_HW_RAY_QUERY
    if (ubo.counts.w == 0u) {
        return false;
    }
    rayQueryEXT rq;
    rayQueryInitializeEXT(rq, topLevelAS, gl_RayFlagsTerminateOnFirstHitEXT | gl_RayFlagsOpaqueEXT, 0xFFu, ro, tMin, rd, tMax);
    rayQueryProceedEXT(rq);
    return rayQueryGetIntersectionTypeEXT(rq, true) != gl_RayQueryCommittedIntersectionNoneEXT;
#else
    HitInfo h = traceScene(ro, rd, tMin, tMax);
    return h.hit;
#endif
}

vec3 sampleBackground(vec3 rd, vec2 uv) {
    bool useEnvBg = (ubo.settings.w & 1u) != 0u;
    if (useEnvBg) {
        return max(textureLod(pbrSpecularMap, -rd, 0.0).rgb * ubo.misc.y, vec3(0.0));
    }
    return mix(ubo.backBottom.rgb, ubo.backTop.rgb, clamp(1.0 - uv.y, 0.0, 1.0));
}

// Solid Random-Walk BSSRDF inside the mesh volume (bounded 16 steps).
bool randomWalkSubsurface(
    inout vec3 pos,
    inout vec3 normal,
    vec3 radius,
    float g,
    float eps,
    inout vec3 throughput,
    inout uint rng
) {
    vec3 mfp = max(radius, vec3(1e-4));
    vec3 sigmaT = 1.0 / mfp;
    int ch = int(min(floor(rand01(rng) * 3.0), 2.0));
    vec3 walkPos = pos - normal * (eps * 4.0);
    vec3 walkDir = sampleHenyeyGreenstein(-normal, g, rand2(rng));
    for (int step = 0; step < 16; ++step) {
        float dist = -log(max(rand01(rng), 1e-6)) / sigmaT[ch];
        HitInfo exitHit = traceScene(walkPos, walkDir, eps, dist);
        if (exitHit.hit) {
            vec3 tr = exp(-sigmaT * exitHit.t);
            float pdf = (tr.r + tr.g + tr.b) / 3.0;
            throughput *= tr / max(pdf, 1e-5);
            pos = exitHit.pos;
            normal = exitHit.shadNormal;
            return true;
        }
        walkPos += walkDir * dist;
        walkDir = sampleHenyeyGreenstein(walkDir, g, rand2(rng));
    }
    return false;
}

void main() {
    uvec2 px = gl_GlobalInvocationID.xy;
    if (px.x >= ubo.counts.x || px.y >= ubo.counts.y) {
        return;
    }

    uint rng = (px.y * ubo.counts.x + px.x) * 1973u + ubo.settings.y * 9277u + 26699u;
    vec2 jitter = rand2(rng);
    vec2 uv = (vec2(px) + jitter) / vec2(ubo.counts.xy);
    vec2 ndc = vec2(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0);

    vec3 ro;
    vec3 rd;
    if ((ubo.settings.w & 2u) != 0u) {
        // Orthographic projection: parallel rays displaced across the near plane.
        ro = ubo.eyeOrigin.xyz + ubo.eyeRight.xyz * ndc.x + ubo.eyeUp.xyz * ndc.y;
        rd = normalize(ubo.eyeView.xyz);
    } else {
        // Perspective projection: pinhole origin with optional thin-lens depth of field.
        ro = ubo.eyeOrigin.xyz;
        rd = normalize(ubo.eyeView.xyz + ubo.eyeRight.xyz * ndc.x + ubo.eyeUp.xyz * ndc.y);
        if (ubo.eyeOrigin.w > 0.0) {
            vec2 disk = rand2(rng);
            float r = ubo.eyeOrigin.w * sqrt(disk.x);
            float phi = 2.0 * OPENPBR_PI * disk.y;
            vec3 lensOffset = normalize(ubo.eyeRight.xyz) * (r * cos(phi)) + normalize(ubo.eyeUp.xyz) * (r * sin(phi));
            vec3 focalPoint = ro + rd * ubo.eyeView.w;
            ro += lensOffset;
            rd = normalize(focalPoint - ro);
        }
    }

    float eps = max(ubo.ambient.w, 1e-5);
    uint maxBounces = max(ubo.settings.x, 1u);
    int heroChannel = int(pcgHash(rng) % 3u);

    vec3 radiance = vec3(0.0);
    vec3 throughput = vec3(1.0);
    vec3 primaryEmission = vec3(0.0);
    vec3 primaryAlbedo = vec3(1.0);
    vec3 primaryNormal = vec3(0.0, 0.0, 1.0);
    float primaryDepth = 1.0;
    float primaryRough = 1.0;
    float primaryHit = 0.0;
    vec2 motionVec = vec2(0.0);
    float primaryId = 0.0;

    bool insideMedium = false;
    vec3 mediumSigmaA = vec3(0.0);
    vec3 mediumSigmaS = vec3(0.0);
    float mediumAniso = 0.0;

    for (uint bounce = 0u; bounce <= maxBounces; ++bounce) {
        HitInfo hit = traceScene(ro, rd, eps, 1e30);
        if (!hit.hit) {
            vec3 bg = sampleBackground(rd, uv);
            if (bounce == 0u) {
                primaryEmission = bg;
            } else {
                radiance += throughput * bg;
            }
            break;
        }

        // Apply Beer-Lambert absorption & homogeneous volume scatter inside refractive solid.
        if (insideMedium) {
            vec3 sigmaT = mediumSigmaA + mediumSigmaS;
            float maxSigma = max(max(sigmaT.r, sigmaT.g), sigmaT.b);
            if (maxSigma > 1e-5) {
                float scatterDist = (length(mediumSigmaS) > 1e-5)
                    ? (-log(max(rand01(rng), 1e-6)) / maxSigma)
                    : 1e30;
                if (scatterDist < hit.t) {
                    vec3 tr = exp(-sigmaT * scatterDist);
                    throughput *= tr * (mediumSigmaS / maxSigma);
                    ro = ro + rd * scatterDist;
                    rd = sampleHenyeyGreenstein(rd, mediumAniso, rand2(rng));
                    continue;
                }
                throughput *= exp(-mediumSigmaA * hit.t);
            }
        }

        OpenPbrParams p = openpbrUnpack(materials[hit.matIndex]);

        // Stochastic geometry opacity pass-through.
        if (p.opacity < 1.0 && rand01(rng) > p.opacity) {
            ro = hit.pos + rd * (eps * 2.0);
            continue;
        }

        vec3 N = hit.shadNormal;
        vec3 T, B;
        openpbrBuildOrthonormalBasis(N, T, B);
        vec3 wi = -rd;

        if (bounce == 0u) {
            primaryHit = 1.0;
            primaryNormal = N;
            vec4 clipCur = ubo.viewProj * vec4(hit.pos, 1.0);
            vec4 clipPrev = ubo.prevViewProj * vec4(hit.pos, 1.0);
            primaryDepth = clamp(clipCur.z / max(clipCur.w, 1e-5), 0.0, 1.0);
            vec2 uvCur = (clipCur.xy / max(clipCur.w, 1e-5)) * vec2(0.5, -0.5) + 0.5;
            vec2 uvPrev = (clipPrev.xy / max(clipPrev.w, 1e-5)) * vec2(0.5, -0.5) + 0.5;
            motionVec = uvPrev - uvCur;
            primaryId = float(hit.matIndex + 1u);
            primaryRough = p.specular_roughness;
            primaryAlbedo = max(mix(p.base_color, vec3(1.0), p.transmission_weight), vec3(0.05));
            float nv0 = clamp(dot(N, wi), 1e-4, 1.0);
            float sh0 = (p.fuzz_weight > 0.0) ? p.fuzz_weight * texture(openpbrSheenLut, vec2(nv0, clamp(p.fuzz_roughness, 0.05, 1.0))).r : 0.0;
            float cr0 = (p.coat_weight > 0.0) ? openpbrFresnelDielectric(nv0, p.coat_ior) : 0.0;
            vec3 ct0 = (p.coat_weight > 0.0) ? openpbrSlabTransmittance(p.coat_color, nv0, p.coat_ior) : vec3(1.0);
            vec3 emAtten = vec3(1.0 - sh0) * clamp(vec3(1.0) - (vec3(1.0) - ct0 * (1.0 - cr0)) * p.coat_weight, vec3(0.0), vec3(1.0));
            primaryEmission = emAtten * p.emission_radiance;
        } else if ((p.lobe_flags & OPENPBR_LOBE_EMISSION) != 0u) {
            radiance += throughput * p.emission_radiance;
        }

        // Next Event Estimation (NEE) over analytic light sources.
        for (uint li = 0u; li < ubo.counts.z; ++li) {
            PtLight light = lights[li];
            vec3 L = light.position.xyz;
            float dist = 1e30;
            float atten = light.color.a;
            if (light.kind.x != 1u) {
                L = light.position.xyz - hit.pos;
                dist = length(L);
                L /= max(dist, 1e-6);
                atten /= max(dist * dist, 1e-4);
                if (light.direction.w > 0.0 && dist > light.direction.w) continue;
            }
            float nl = dot(N, L);
            if (nl <= 0.0 && !(p.thin_walled && p.subsurface_weight > 0.0)) continue;
            vec3 shadowOrigin = hit.pos + hit.geomNormal * (nl >= 0.0 ? eps * 2.0 : -eps * 2.0);
            if (!traceShadowOccluded(shadowOrigin, L, eps, dist - eps * 4.0)) {
                float bsdfPdf;
                vec3 f = openpbrEvalBsdf(p, N, T, B, wi, L, openpbrGgxLut, openpbrSheenLut, openpbrThinFilmLut, bsdfPdf);
                radiance += throughput * f * light.color.rgb * atten;
            }
        }

        if (bounce == maxBounces) {
            break;
        }

        // Sample OpenPBR BSDF for next bounce.
        OpenPbrSampleResult s = openpbrSampleBsdf(
            p, N, T, B, wi, hit.backfacing, heroChannel,
            openpbrGgxLut, openpbrSheenLut, openpbrThinFilmLut, rng
        );
        if (max(max(s.weight.r, s.weight.g), s.weight.b) <= 1e-6) {
            break;
        }
        throughput *= s.weight;

        if (s.enterSubsurface) {
            vec3 exitPos = hit.pos;
            vec3 exitN = N;
            if (!randomWalkSubsurface(exitPos, exitN, p.subsurface_radius, p.subsurface_aniso, eps, throughput, rng)) {
                break;
            }
            vec3 eT, eB;
            openpbrBuildOrthonormalBasis(exitN, eT, eB);
            float exitPdf;
            rd = sampleCosineHemisphere(rand2(rng), eT, eB, exitN, exitPdf);
            ro = exitPos + exitN * (eps * 2.0);
        } else {
            if (s.transmitted && !p.thin_walled) {
                insideMedium = !hit.backfacing;
                if (insideMedium && p.transmission_depth > 0.0) {
                    mediumSigmaA = -log(max(p.transmission_color, vec3(1e-5))) / p.transmission_depth;
                    mediumSigmaS = p.transmission_scatter;
                    mediumAniso = p.transmission_scatter_aniso;
                } else {
                    mediumSigmaA = vec3(0.0);
                    mediumSigmaS = vec3(0.0);
                }
            }
            ro = hit.pos + hit.geomNormal * (s.transmitted ? (-eps * 2.0) : (eps * 2.0));
            rd = s.wo;
        }

        // Russian roulette after bounce 2.
        if (bounce >= 2u) {
            float survive = clamp(max(throughput.r, max(throughput.g, throughput.b)), 0.05, 0.95);
            if (rand01(rng) > survive) {
                break;
            }
            throughput /= survive;
        }
    }

    // Firefly clamping + demodulation for SVGF denoiser.
    float clampVal = max(ubo.misc.x, 1.0);
    radiance = min(max(radiance, vec3(0.0)), vec3(clampVal));
    vec3 demod = (primaryHit > 0.5) ? (radiance / primaryAlbedo) : radiance;

    imageStore(outRadiance, ivec2(px), vec4(demod, primaryHit));
    imageStore(outAlbedoRoughness, ivec2(px), vec4(primaryAlbedo, primaryRough));
    imageStore(outNormalDepth, ivec2(px), vec4(primaryNormal, primaryDepth));
    imageStore(outMotionId, ivec2(px), vec4(motionVec, primaryId, 0.0));
    imageStore(outEmission, ivec2(px), vec4(primaryEmission, primaryHit));
}
