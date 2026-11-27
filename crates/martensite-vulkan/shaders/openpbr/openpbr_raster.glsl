// ASWF OpenPBR Surface v1.1 Real-Time Rasterization Evaluator (modeled on Blender EEVEE
// `gpu_shader_material_open_pbr.bsl.hh`), included by `lighting.glsl` under `MARTENSITE_PBR`.
//
// Set 4 bindings (OpenPBR IBL + LUTs):
//   0: openpbrGgxLut (2D RG16F)
//   1: pbrDiffuseSH (2D 9x1 RGBA32F)
//   2: pbrSpecularMap (Cube RGBA16F HDR)
//   3: openpbrDielectricLut (3D RG16F)
//   4: openpbrSheenLut (2D R16F)
//   5: openpbrThinFilmLut (2D RGBA16F)
#ifndef OPENPBR_RASTER_GLSL
#define OPENPBR_RASTER_GLSL

layout(set=4, binding=0) uniform sampler2D openpbrGgxLut;
layout(set=4, binding=1) uniform sampler2D pbrDiffuseSH;
layout(set=4, binding=2) uniform samplerCube pbrSpecularMap;
layout(set=4, binding=3) uniform sampler3D openpbrDielectricLut;
layout(set=4, binding=4) uniform sampler2D openpbrSheenLut;
layout(set=4, binding=5) uniform sampler2D openpbrThinFilmLut;

#include "openpbr_common.glsl"

float openpbrPointAttenuation(float dist, float range) {
    if (range == 0.0) {
        return 1.0 / max(0.0001, dist * dist);
    }
    if (dist > range) {
        return -1.0;
    }
    float ratio = dist / range;
    ratio *= ratio;
    ratio *= ratio;
    return clamp(1.0 - ratio, 0.0, 1.0) / max(0.0001, dist * dist);
}

float openpbrSpotAttenuation(RenderLight light, vec3 lightDir, float atten) {
    float cosA = dot(light.direction.xyz, -lightDir);
    float rel = 2.0 * acos(clamp(cosA, -1.0, 1.0)) / max(light.params.z, 1e-5);
    if (cosA >= 1.0 || rel > 1.0) {
        return -1.0;
    }
    float expVal = light.params.w;
    if ((1.0 - rel) <= expVal) {
        float offset = cos(0.5 * light.params.z);
        float scale = 1.0 / max(0.001, cos(0.5 * light.params.z * (1.0 - expVal)) - offset);
        float f = clamp(cosA * scale - offset * scale, 0.0, 1.0);
        atten *= f * f;
    }
    return atten;
}

vec3 openpbrSampleDiffuseSH(vec3 N) {
    vec3 sh[9];
    for (int i = 0; i < 9; ++i) {
        sh[i] = texture(pbrDiffuseSH, vec2((float(i) + 0.5) / 9.0, 0.5)).rgb;
    }
    vec3 irradiance = sh[0]
        + sh[1] * N.x
        + sh[2] * N.y
        + sh[3] * N.z
        + sh[4] * N.x * N.z
        + sh[5] * N.y * N.z
        + sh[6] * N.x * N.y
        + sh[7] * (3.0 * N.z * N.z - 1.0)
        + sh[8] * (N.x * N.x - N.y * N.y);
    return max(irradiance, vec3(0.0));
}

vec3 openpbrSampleSpecularIbl(vec3 R, float roughness) {
    float maxLod = float(max(textureQueryLevels(pbrSpecularMap) - 1, 0));
    return max(textureLod(pbrSpecularMap, -R, roughness * maxLod).rgb, vec3(0.0));
}

// Evaluates OpenPBR v1.1 direct lighting for one analytic light source (directional, point, or spot).
vec3 openpbrEvaluateDirectLight(
    vec3 V, vec3 L, vec3 N, vec3 T, vec3 B,
    OpenPbrParams p,
    float effSpecIor,
    float effSpecRough,
    vec2 specAlpha,
    vec2 coatAlpha,
    vec3 fuzzAtten,
    vec3 coatTrans,
    vec3 darkening,
    vec2 ggxLutV,
    vec3 lightRadiance
) {
    float nl = dot(N, L);
    float nv = max(dot(N, V), 1e-4);
    vec3 contrib = vec3(0.0);

    // Thin-walled subsurface backlit transmission (L on opposite hemisphere).
    if (nl <= 0.0) {
        if (p.thin_walled && p.subsurface_weight > 0.0 && p.metalness < 1.0 && p.transmission_weight < 1.0) {
            float nlBack = max(-nl, 0.0);
            float fwdWeight = clamp(0.5 * (1.0 + p.subsurface_aniso), 0.0, 1.0);
            vec3 layerWeight = fuzzAtten * coatTrans * darkening * (1.0 - p.metalness) * (1.0 - p.transmission_weight);
            contrib += layerWeight * p.subsurface_weight * fwdWeight * p.subsurface_color * OPENPBR_INV_PI * nlBack * lightRadiance;
        }
        return contrib;
    }

    vec3 H = normalize(V + L);
    float nh = max(dot(N, H), 1e-5);
    float vh = max(dot(V, H), 1e-5);
    vec3 vLocal = vec3(dot(V, T), dot(V, B), nv);
    vec3 lLocal = vec3(dot(L, T), dot(L, B), nl);
    vec3 hLocal = vec3(dot(H, T), dot(H, B), nh);

    vec3 weight = vec3(1.0);

    // 1. Fuzz (Charlie microfiber sheen).
    if (p.fuzz_weight > 0.0) {
        float sheen = openpbrSheenBrdf(nv, nl, nh, p.fuzz_roughness);
        contrib += weight * p.fuzz_weight * p.fuzz_color * sheen;
        weight *= fuzzAtten;
    }

    // 2. Clear-coat dielectric GGX.
    if (p.coat_weight > 0.0) {
        float Fc = openpbrFresnelDielectric(vh, p.coat_ior);
        float Dc = openpbrGgxAnisoD(hLocal, coatAlpha);
        float Vc = openpbrGgxAnisoV(vLocal, lLocal, coatAlpha);
        contrib += weight * p.coat_weight * (Fc * Dc * Vc);
        weight *= coatTrans;
    }

    // 3. Coat darkening (Eq. 64-70).
    weight *= darkening;

    // 4. Metallic conductor (F82-tint + Airy thin-film + multi-scatter anisotropic GGX).
    float Ds = openpbrGgxAnisoD(hLocal, specAlpha);
    float Vs = openpbrGgxAnisoV(vLocal, lLocal, specAlpha);
    if (p.metalness > 0.0) {
        vec3 Fm = openpbrFresnelF82Tint(vh, p.base_color, p.specular_color);
        if (p.thin_film_weight > 0.0 && p.thin_film_thickness_nm > 1.0) {
            vec3 Ffilm = openpbrThinFilmIridescence(vh, p.thin_film_thickness_nm, p.thin_film_ior, p.base_color, effSpecIor, true, openpbrThinFilmLut);
            Fm = mix(Fm, Ffilm, p.thin_film_weight);
        }
        vec3 Favg = openpbrFresnelF82Avg(p.base_color, p.specular_color);
        vec3 ms = openpbrGgxMultiscatterWeight(Favg, ggxLutV);
        contrib += weight * p.metalness * (Fm * ms * Ds * Vs);
        weight *= (1.0 - p.metalness);
    }

    // 5. Dielectric specular (modulated IOR + specular_color + thin-film) over transmission/SSS/diffuse.
    if (p.metalness < 1.0) {
        float FdScalar = openpbrFresnelDielectric(vh, effSpecIor);
        vec3 Fd = vec3(FdScalar);
        if (p.thin_film_weight > 0.0 && p.thin_film_thickness_nm > 1.0) {
            vec3 Ffilm = openpbrThinFilmIridescence(vh, p.thin_film_thickness_nm, p.thin_film_ior, vec3(openpbrF0FromIor(effSpecIor)), effSpecIor, false, openpbrThinFilmLut);
            Fd = mix(Fd, Ffilm, p.thin_film_weight);
        }
        float FavgD = openpbrHemisphericalAlbedo(effSpecIor);
        vec3 msD = openpbrGgxMultiscatterWeight(vec3(FavgD), ggxLutV);
        contrib += weight * (Fd * p.specular_color * msD * Ds * Vs);

        // Refracted energy entering opaque base beneath dielectric specular.
        vec3 baseWeight = weight * (1.0 - p.transmission_weight) * max(vec3(0.0), vec3(1.0) - Fd);
        float vl = dot(V, L);
        if (p.subsurface_weight > 0.0) {
            if (p.thin_walled) {
                float backWeight = clamp(0.5 * (1.0 - p.subsurface_aniso), 0.0, 1.0);
                vec3 sssBrdf = openpbrOrenNayarEon(p.subsurface_color, p.diffuse_roughness, nv, nl, vl);
                contrib += baseWeight * p.subsurface_weight * backWeight * sssBrdf;
            } else {
                // Real-time Burley/wrap subsurface approximation for solid SSS.
                float wrap = clamp(length(p.subsurface_radius) * 0.25, 0.05, 0.5);
                float nlWrap = max(0.0, (nl + wrap) / ((1.0 + wrap) * (1.0 + wrap)));
                contrib += baseWeight * p.subsurface_weight * p.subsurface_color * OPENPBR_INV_PI * (nlWrap / max(nl, 1e-4));
            }
            baseWeight *= (1.0 - p.subsurface_weight);
        }
        vec3 diffBrdf = openpbrOrenNayarEon(p.base_color, p.diffuse_roughness, nv, nl, vl);
        contrib += baseWeight * diffBrdf;
    }

    return contrib * nl * lightRadiance;
}

OpenPbrParams openpbrSurfaceParams(bool isFront, vec4 vertexColor) {
    uint face = isFront ? 0u : 1u;
    OpenPbrParams p = openpbrUnpack(loadOpenPbrMaterial(face));
#if defined(MARTENSITE_FRAGMENT) && defined(MARTENSITE_SURFACE_FAMILY)
    vec4 baseTex = surfaceTexel();
    p.base_color *= baseTex.rgb;
    p.opacity *= baseTex.a;
#endif
    if (lighting.settings.z != 0u && (isFront || (lighting.settings.w & 1u) == 0u)) {
        p.base_color *= vertexColor.rgb;
        p.opacity *= vertexColor.a;
    }
#ifdef MARTENSITE_MATERIAL_MAPS
    p.emission_radiance *= emissiveMapTexel();
    vec2 mr = metallicRoughnessMapTexel();
    p.metalness = clamp(p.metalness * mr.x, 0.0, 1.0);
    p.specular_roughness = clamp(p.specular_roughness * mr.y, 0.0, 1.0);
#endif
    return p;
}

vec4 computePbrLighting(vec3 normal, vec3 view, vec4 worldPoint, bool isFront, vec4 vertexColor) {
    OpenPbrParams p = openpbrSurfaceParams(isFront, vertexColor);
    vec3 N = isFront ? normal : -normal;
    vec3 V = normalize(view);
    vec3 T, B;
    openpbrBuildOrthonormalBasis(N, T, B);
    float nv = clamp(dot(N, V), 1e-4, 1.0);

    float effSpecIor = openpbrEffectiveSpecularIor(p.specular_ior, p.specular_weight, p.coat_ior, p.coat_weight);
    float effSpecRough = openpbrCoatRoughening(p.specular_roughness, p.coat_roughness, p.coat_weight);
    vec2 specAlpha = openpbrAnisotropicRoughness(effSpecRough, p.specular_anisotropy);
    vec2 coatAlpha = openpbrAnisotropicRoughness(p.coat_roughness, p.coat_anisotropy);

    // Precompute layer attenuations matching OpenPBR slab order.
    float sheenAl = (p.fuzz_weight > 0.0)
        ? p.fuzz_weight * texture(openpbrSheenLut, vec2(nv, clamp(p.fuzz_roughness, 0.05, 1.0))).r
        : 0.0;
    vec3 fuzzAtten = vec3(clamp(1.0 - sheenAl, 0.0, 1.0));

    float coatRefl = 0.0;
    vec3 coatTrans = vec3(1.0);
    if (p.coat_weight > 0.0) {
        float coatZ = openpbrDielectricIorToLutZ(p.coat_ior);
        coatRefl = texture(openpbrDielectricLut, vec3(nv, p.coat_roughness, coatZ)).r;
        vec3 coatTint = openpbrSlabTransmittance(p.coat_color, nv, p.coat_ior);
        coatTrans = clamp(vec3(1.0) - (vec3(1.0) - coatTint * (1.0 - coatRefl)) * p.coat_weight, vec3(0.0), vec3(1.0));
    }

    vec3 darkening = openpbrCoatDarkening(
        nv, p.base_color, p.metalness, p.specular_color, effSpecIor, effSpecRough,
        p.transmission_weight, p.transmission_color, p.subsurface_weight, p.subsurface_color,
        p.coat_weight, p.coat_ior, p.coat_darkening
    );

    vec2 ggxLutV = texture(openpbrGgxLut, vec2(nv, effSpecRough)).rg;
    vec3 point = worldPoint.xyz / worldPoint.w;

    // Direct analytic lights (with shadow map attenuation).
    vec3 color = vec3(0.0);
    for (uint i = 0u; i < lighting.settings.y; ++i) {
        RenderLight light = loadRenderLight(i);
        vec3 L = light.position.xyz;
        float intensity = light.color.a;
        if (light.kind.x != 1u) {
            L = light.position.xyz - point;
            float dist = length(L);
            float atten = openpbrPointAttenuation(dist, light.direction.w);
            if (atten <= 0.0) continue;
            L /= max(dist, 1e-6);
            if (light.kind.x == 3u) atten = openpbrSpotAttenuation(light, L, atten);
            if (atten < 0.0) continue;
            intensity *= atten;
        }
        float shadow = lightShadow(i, light, normal, worldPoint);
        color += openpbrEvaluateDirectLight(
            V, L, N, T, B, p, effSpecIor, effSpecRough, specAlpha, coatAlpha,
            fuzzAtten, coatTrans, darkening, ggxLutV, light.color.rgb * intensity * shadow
        );
    }

    // Indirect HDR IBL (Diffuse SH + Prefiltered HDR Specular Cube + Multi-scatter LUTs).
    vec3 R = reflect(-V, N);
    vec3 diffIbl = openpbrSampleDiffuseSH(N);
    vec3 ibl = vec3(0.0);
    vec3 iblWeight = vec3(1.0);

    if (p.fuzz_weight > 0.0) {
        ibl += iblWeight * p.fuzz_color * sheenAl * diffIbl;
        iblWeight *= fuzzAtten;
    }
    if (p.coat_weight > 0.0) {
        ibl += iblWeight * p.coat_weight * coatRefl * openpbrSampleSpecularIbl(R, p.coat_roughness);
        iblWeight *= coatTrans;
    }
    // Emission sits beneath fuzz & coat, above coat darkening.
    vec3 emission = iblWeight * p.emission_radiance;
    iblWeight *= darkening;

    vec3 specIbl = openpbrSampleSpecularIbl(R, effSpecRough);
    if (p.metalness > 0.0) {
        vec3 F0 = p.base_color;
        vec3 Fm = openpbrFresnelF82Tint(nv, F0, p.specular_color);
        if (p.thin_film_weight > 0.0 && p.thin_film_thickness_nm > 1.0) {
            vec3 Ffilm = openpbrThinFilmIridescence(nv, p.thin_film_thickness_nm, p.thin_film_ior, F0, effSpecIor, true, openpbrThinFilmLut);
            Fm = mix(Fm, Ffilm, p.thin_film_weight);
        }
        vec3 Favg = openpbrFresnelF82Avg(F0, p.specular_color);
        vec3 Fms = openpbrGgxMultiscatterWeight(Favg, ggxLutV);
        ibl += iblWeight * p.metalness * (Fm * ggxLutV.x * Fms) * specIbl;
        iblWeight *= (1.0 - p.metalness);
    }

    float outAlpha = p.opacity;
    if (p.metalness < 1.0) {
        float dielZ = openpbrDielectricIorToLutZ(effSpecIor);
        vec2 dielLut = texture(openpbrDielectricLut, vec3(nv, effSpecRough, dielZ)).rg;
        vec3 specRefl = vec3(dielLut.r);
        if (p.thin_film_weight > 0.0 && p.thin_film_thickness_nm > 1.0) {
            vec3 Ffilm = openpbrThinFilmIridescence(nv, p.thin_film_thickness_nm, p.thin_film_ior, vec3(openpbrF0FromIor(effSpecIor)), effSpecIor, false, openpbrThinFilmLut);
            specRefl = mix(specRefl, Ffilm * ggxLutV.x, p.thin_film_weight);
        }

        if (p.transmission_weight > 0.0) {
            vec3 transTint = p.thin_walled
                ? openpbrSlabTransmittance(p.transmission_color, nv, effSpecIor)
                : p.transmission_color;
            float transRough = p.thin_walled
                ? openpbrThinGlassTransmissionRoughness(effSpecRough, effSpecIor)
                : effSpecRough;
            vec3 refractDir = p.thin_walled ? (-V) : refract(-V, N, 1.0 / max(effSpecIor, 1.0001));
            if (dot(refractDir, refractDir) < 1e-5) refractDir = R;
            vec3 transIbl = openpbrSampleSpecularIbl(refractDir, transRough);
            ibl += iblWeight * p.transmission_weight * (specRefl * p.specular_color * specIbl + dielLut.g * transTint * transIbl);
            iblWeight *= (1.0 - p.transmission_weight);
        }

        ibl += iblWeight * (specRefl * p.specular_color) * specIbl;
        vec3 diffWeight = iblWeight * max(vec3(0.0), vec3(1.0) - specRefl);
        if (p.subsurface_weight > 0.0) {
            if (p.thin_walled) {
                float backW = clamp(0.5 * (1.0 - p.subsurface_aniso), 0.0, 1.0);
                float fwdW = clamp(0.5 * (1.0 + p.subsurface_aniso), 0.0, 1.0);
                ibl += diffWeight * p.subsurface_weight * p.subsurface_color * (backW * diffIbl + fwdW * openpbrSampleDiffuseSH(-N));
            } else {
                ibl += diffWeight * p.subsurface_weight * p.subsurface_color * (0.75 * diffIbl + 0.25 * openpbrSampleDiffuseSH(-N));
            }
            diffWeight *= (1.0 - p.subsurface_weight);
        }
        ibl += diffWeight * p.base_color * diffIbl;
        outAlpha = mix(1.0, p.opacity, clamp(1.0 - specRefl.r, 0.0, 1.0));
    }

    color += lighting.ambient.rgb * ibl + emission;
#ifdef MARTENSITE_MATERIAL_MAPS
    color *= occlusionMapTexel();
#endif
    return vec4(max(color, vec3(0.0)), outAlpha);
}

#endif
