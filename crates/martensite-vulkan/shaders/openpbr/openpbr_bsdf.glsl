// Unified ASWF OpenPBR Surface v1.1 Path-Tracing BSDF, Sampling & Random-Walk SSS
// Shared identically by `pathtrace_hw.comp` (VK_KHR_ray_query) and `pathtrace_sw.comp` (BVH SSBO).
#ifndef OPENPBR_BSDF_GLSL
#define OPENPBR_BSDF_GLSL

#include "openpbr_types.glsl"
#include "openpbr_common.glsl"

// PCG32 deterministic PRNG for progressive path tracing.
uint pcgHash(inout uint state) {
    state = state * 747796405u + 2891336453u;
    uint word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word;
}

float rand01(inout uint rng) {
    return float(pcgHash(rng) & 0x00FFFFFFu) * (1.0 / 16777216.0);
}

vec2 rand2(inout uint rng) {
    return vec2(rand01(rng), rand01(rng));
}

// Cosine-weighted hemisphere sampling in local tangent frame (T, B, N).
vec3 sampleCosineHemisphere(vec2 u, vec3 T, vec3 B, vec3 N, out float pdf) {
    float r = sqrt(clamp(u.x, 0.0, 0.999999));
    float phi = 2.0 * OPENPBR_PI * u.y;
    float x = r * cos(phi);
    float y = r * sin(phi);
    float z = sqrt(max(0.0, 1.0 - u.x));
    pdf = z * OPENPBR_INV_PI;
    return normalize(x * T + y * B + z * N);
}

// Heitz (2018) anisotropic GGX VNDF half-vector sampler in local space.
vec3 sampleGgxVndfLocal(vec3 vLocal, vec2 alpha, vec2 u) {
    vec3 vh = normalize(vec3(alpha.x * vLocal.x, alpha.y * vLocal.y, max(vLocal.z, 1e-5)));
    float lensq = vh.x * vh.x + vh.y * vh.y;
    vec3 t1 = (lensq > 1e-12) ? (vec3(-vh.y, vh.x, 0.0) * inversesqrt(lensq)) : vec3(1.0, 0.0, 0.0);
    vec3 t2 = cross(vh, t1);
    float r = sqrt(clamp(u.x, 0.0, 0.999999));
    float phi = 2.0 * OPENPBR_PI * u.y;
    float p1 = r * cos(phi);
    float p2 = r * sin(phi);
    float s = 0.5 * (1.0 + vh.z);
    p2 = (1.0 - s) * sqrt(max(0.0, 1.0 - p1 * p1)) + s * p2;
    vec3 nh = p1 * t1 + p2 * t2 + sqrt(max(0.0, 1.0 - p1 * p1 - p2 * p2)) * vh;
    return normalize(vec3(alpha.x * nh.x, alpha.y * nh.y, max(0.0, nh.z)));
}

float ggxG1Local(vec3 wLocal, vec2 alpha) {
    float ax = max(alpha.x, 0.002);
    float ay = max(alpha.y, 0.002);
    float z = max(wLocal.z, 1e-5);
    return 2.0 * z / (z + length(vec3(ax * wLocal.x, ay * wLocal.y, z)));
}

float pdfGgxVndfReflect(vec3 vLocal, vec3 hLocal, vec2 alpha) {
    float D = openpbrGgxAnisoD(hLocal, alpha);
    float G1 = ggxG1Local(vLocal, alpha);
    return (D * G1) / max(4.0 * vLocal.z, 1e-6);
}

// Henyey-Greenstein phase function sampler (volume scattering & random-walk SSS).
vec3 sampleHenyeyGreenstein(vec3 dir, float g, vec2 u) {
    float cosTheta;
    if (abs(g) < 1e-3) {
        cosTheta = 1.0 - 2.0 * u.x;
    } else {
        float sqrTerm = (1.0 - g * g) / max(1.0 - g + 2.0 * g * u.x, 1e-5);
        cosTheta = (1.0 + g * g - sqrTerm * sqrTerm) / (2.0 * g);
    }
    float sinTheta = sqrt(max(0.0, 1.0 - cosTheta * cosTheta));
    float phi = 2.0 * OPENPBR_PI * u.y;
    vec3 T, B;
    openpbrBuildOrthonormalBasis(dir, T, B);
    return normalize(sinTheta * cos(phi) * T + sinTheta * sin(phi) * B + cosTheta * dir);
}

// Abbe-number chromatic dispersion IOR offset for hero RGB wavelength channel (0=R, 1=G, 2=B).
// Matches Cycles `bsdf_glass_ior` Cauchy dispersion model around Fraunhofer d-line (587.6 nm).
float openpbrDispersedIor(float ior, float dispScale, float abbe, int channel) {
    if (dispScale <= 0.0 || abbe < 1.0) {
        return ior;
    }
    // Normalized wavelength offset relative to d-line: R=-0.5, G=0.0, B=+0.5.
    float delta = (channel == 0) ? -0.5 : ((channel == 2) ? 0.5 : 0.0);
    float spread = dispScale * (ior - 1.0) / max(abbe, 1.0);
    return max(1.001, ior + delta * spread);
}

struct OpenPbrSampleResult {
    vec3 wo;
    vec3 weight;       // f(wi, wo) * |cos_o| / pdf
    float pdf;
    bool isDelta;
    bool transmitted;
    bool enterSubsurface;
};

// Evaluates OpenPBR v1.1 reflection BSDF f(wi, wo) * cos(theta_o) and combined sampling PDF for MIS.
vec3 openpbrEvalBsdf(
    OpenPbrParams p,
    vec3 N, vec3 T, vec3 B,
    vec3 wi, vec3 wo,
    sampler2D ggxLut,
    sampler2D sheenLut,
    sampler2D thinFilmLut,
    out float pdfOut
) {
    pdfOut = 0.0;
    float nv = dot(N, wi);
    float nl = dot(N, wo);
    if (nv <= 1e-5) {
        return vec3(0.0);
    }

    float effSpecIor = openpbrEffectiveSpecularIor(p.specular_ior, p.specular_weight, p.coat_ior, p.coat_weight);
    float effSpecRough = openpbrCoatRoughening(p.specular_roughness, p.coat_roughness, p.coat_weight);
    vec2 specAlpha = openpbrAnisotropicRoughness(effSpecRough, p.specular_anisotropy);
    vec2 coatAlpha = openpbrAnisotropicRoughness(p.coat_roughness, p.coat_anisotropy);

    float sheenAl = (p.fuzz_weight > 0.0)
        ? p.fuzz_weight * texture(sheenLut, vec2(nv, clamp(p.fuzz_roughness, 0.05, 1.0))).r
        : 0.0;
    vec3 fuzzAtten = vec3(clamp(1.0 - sheenAl, 0.0, 1.0));
    float coatRefl = (p.coat_weight > 0.0) ? openpbrFresnelDielectric(nv, p.coat_ior) : 0.0;
    vec3 coatTint = (p.coat_weight > 0.0) ? openpbrSlabTransmittance(p.coat_color, nv, p.coat_ior) : vec3(1.0);
    vec3 coatTrans = clamp(vec3(1.0) - (vec3(1.0) - coatTint * (1.0 - coatRefl)) * p.coat_weight, vec3(0.0), vec3(1.0));
    vec3 darkening = openpbrCoatDarkening(
        nv, p.base_color, p.metalness, p.specular_color, effSpecIor, effSpecRough,
        p.transmission_weight, p.transmission_color, p.subsurface_weight, p.subsurface_color,
        p.coat_weight, p.coat_ior, p.coat_darkening
    );

    // Thin-walled diffuse transmission when wo is on the opposite hemisphere.
    if (nl <= 0.0) {
        if (p.thin_walled && p.subsurface_weight > 0.0 && p.metalness < 1.0 && p.transmission_weight < 1.0) {
            float nlBack = max(-nl, 0.0);
            float fwdW = clamp(0.5 * (1.0 + p.subsurface_aniso), 0.0, 1.0);
            vec3 w = fuzzAtten * coatTrans * darkening * (1.0 - p.metalness) * (1.0 - p.transmission_weight);
            pdfOut = nlBack * OPENPBR_INV_PI * p.subsurface_weight * fwdW;
            return w * p.subsurface_weight * fwdW * p.subsurface_color * OPENPBR_INV_PI * nlBack;
        }
        return vec3(0.0);
    }

    vec3 H = normalize(wi + wo);
    float nh = max(dot(N, H), 1e-5);
    float vh = max(dot(wi, H), 1e-5);
    vec3 vLocal = vec3(dot(wi, T), dot(wi, B), nv);
    vec3 lLocal = vec3(dot(wo, T), dot(wo, B), nl);
    vec3 hLocal = vec3(dot(H, T), dot(H, B), nh);
    vec2 ggxLutV = texture(ggxLut, vec2(nv, effSpecRough)).rg;

    vec3 eval = vec3(0.0);
    vec3 w = vec3(1.0);

    // Lobe selection probabilities (matching openpbrSampleBsdf).
    float pFuzz = p.fuzz_weight * sheenAl;
    float pCoat = (1.0 - pFuzz) * p.coat_weight * coatRefl;
    float pBase = max(1.0 - pFuzz - pCoat, 0.0);
    float pMetal = pBase * p.metalness;
    float pDiel = pBase * (1.0 - p.metalness);
    float fDielV = openpbrFresnelDielectric(nv, effSpecIor);
    float pSpec = pDiel * (p.transmission_weight * fDielV + (1.0 - p.transmission_weight) * fDielV);
    float pDiff = pDiel * (1.0 - p.transmission_weight) * (1.0 - fDielV);

    if (p.fuzz_weight > 0.0) {
        eval += w * p.fuzz_weight * p.fuzz_color * openpbrSheenBrdf(nv, nl, nh, p.fuzz_roughness);
        pdfOut += pFuzz * (nl * OPENPBR_INV_PI);
        w *= fuzzAtten;
    }

    if (p.coat_weight > 0.0) {
        float Fc = openpbrFresnelDielectric(vh, p.coat_ior);
        float Dc = openpbrGgxAnisoD(hLocal, coatAlpha);
        float Vc = openpbrGgxAnisoV(vLocal, lLocal, coatAlpha);
        eval += w * p.coat_weight * (Fc * Dc * Vc);
        pdfOut += pCoat * pdfGgxVndfReflect(vLocal, hLocal, coatAlpha);
        w *= coatTrans;
    }

    w *= darkening;
    float Ds = openpbrGgxAnisoD(hLocal, specAlpha);
    float Vs = openpbrGgxAnisoV(vLocal, lLocal, specAlpha);
    float pdfSpecVndf = pdfGgxVndfReflect(vLocal, hLocal, specAlpha);

    if (p.metalness > 0.0) {
        vec3 Fm = openpbrFresnelF82Tint(vh, p.base_color, p.specular_color);
        if (p.thin_film_weight > 0.0 && p.thin_film_thickness_nm > 1.0) {
            vec3 Ff = openpbrThinFilmIridescence(vh, p.thin_film_thickness_nm, p.thin_film_ior, p.base_color, effSpecIor, true, thinFilmLut);
            Fm = mix(Fm, Ff, p.thin_film_weight);
        }
        vec3 Favg = openpbrFresnelF82Avg(p.base_color, p.specular_color);
        vec3 ms = openpbrGgxMultiscatterWeight(Favg, ggxLutV);
        eval += w * p.metalness * (Fm * ms * Ds * Vs);
        pdfOut += pMetal * pdfSpecVndf;
        w *= (1.0 - p.metalness);
    }

    if (p.metalness < 1.0) {
        vec3 Fd = vec3(openpbrFresnelDielectric(vh, effSpecIor));
        if (p.thin_film_weight > 0.0 && p.thin_film_thickness_nm > 1.0) {
            vec3 Ff = openpbrThinFilmIridescence(vh, p.thin_film_thickness_nm, p.thin_film_ior, vec3(openpbrF0FromIor(effSpecIor)), effSpecIor, false, thinFilmLut);
            Fd = mix(Fd, Ff, p.thin_film_weight);
        }
        float FavgD = openpbrHemisphericalAlbedo(effSpecIor);
        vec3 msD = openpbrGgxMultiscatterWeight(vec3(FavgD), ggxLutV);
        eval += w * (Fd * p.specular_color * msD * Ds * Vs);
        pdfOut += pSpec * pdfSpecVndf;

        vec3 baseW = w * (1.0 - p.transmission_weight) * max(vec3(0.0), vec3(1.0) - Fd);
        float vl = dot(wi, wo);
        if (p.subsurface_weight > 0.0 && p.thin_walled) {
            float backW = clamp(0.5 * (1.0 - p.subsurface_aniso), 0.0, 1.0);
            eval += baseW * p.subsurface_weight * backW * openpbrOrenNayarEon(p.subsurface_color, p.diffuse_roughness, nv, nl, vl);
            baseW *= (1.0 - p.subsurface_weight);
        } else if (p.subsurface_weight > 0.0) {
            baseW *= (1.0 - p.subsurface_weight);
        }
        eval += baseW * openpbrOrenNayarEon(p.base_color, p.diffuse_roughness, nv, nl, vl);
        pdfOut += pDiff * (nl * OPENPBR_INV_PI);
    }

    return eval * nl;
}

// Importance-samples the OpenPBR v1.1 slab stack (Fuzz, Coat, Metal, Transmission, Specular, SSS, Diffuse).
OpenPbrSampleResult openpbrSampleBsdf(
    OpenPbrParams p,
    vec3 N, vec3 T, vec3 B,
    vec3 wi,
    bool backfacing,
    int heroChannel,
    sampler2D ggxLut,
    sampler2D sheenLut,
    sampler2D thinFilmLut,
    inout uint rng
) {
    OpenPbrSampleResult res;
    res.wo = N;
    res.weight = vec3(0.0);
    res.pdf = 0.0;
    res.isDelta = false;
    res.transmitted = false;
    res.enterSubsurface = false;

    float nv = clamp(dot(N, wi), 1e-4, 1.0);
    float dispIor = openpbrDispersedIor(p.specular_ior, p.dispersion_scale, p.dispersion_abbe, heroChannel);
    float effSpecIor = openpbrEffectiveSpecularIor(dispIor, p.specular_weight, p.coat_ior, p.coat_weight);
    float eta = (backfacing && !p.thin_walled) ? (1.0 / max(effSpecIor, 1e-4)) : effSpecIor;
    float effSpecRough = openpbrCoatRoughening(p.specular_roughness, p.coat_roughness, p.coat_weight);
    vec2 specAlpha = openpbrAnisotropicRoughness(effSpecRough, p.specular_anisotropy);
    vec2 coatAlpha = openpbrAnisotropicRoughness(p.coat_roughness, p.coat_anisotropy);
    vec2 ggxLutV = texture(ggxLut, vec2(nv, effSpecRough)).rg;

    float sheenAl = (p.fuzz_weight > 0.0)
        ? p.fuzz_weight * texture(sheenLut, vec2(nv, clamp(p.fuzz_roughness, 0.05, 1.0))).r
        : 0.0;
    vec3 fuzzAtten = vec3(clamp(1.0 - sheenAl, 0.0, 1.0));
    float coatRefl = (p.coat_weight > 0.0) ? openpbrFresnelDielectric(nv, p.coat_ior) : 0.0;
    vec3 coatTint = (p.coat_weight > 0.0) ? openpbrSlabTransmittance(p.coat_color, nv, p.coat_ior) : vec3(1.0);
    vec3 coatTrans = clamp(vec3(1.0) - (vec3(1.0) - coatTint * (1.0 - coatRefl)) * p.coat_weight, vec3(0.0), vec3(1.0));
    vec3 darkening = openpbrCoatDarkening(
        nv, p.base_color, p.metalness, p.specular_color, effSpecIor, effSpecRough,
        p.transmission_weight, p.transmission_color, p.subsurface_weight, p.subsurface_color,
        p.coat_weight, p.coat_ior, p.coat_darkening
    );

    vec3 vLocal = vec3(dot(wi, T), dot(wi, B), nv);

    float pFuzz = p.fuzz_weight * sheenAl;
    float pCoat = (1.0 - pFuzz) * p.coat_weight * coatRefl;
    float pBase = max(1.0 - pFuzz - pCoat, 0.0);
    float pMetal = pBase * p.metalness;
    float pDiel = pBase * (1.0 - p.metalness);
    float fDiel = openpbrFresnelDielectric(nv, eta);
    float pTrans = pDiel * p.transmission_weight * (1.0 - fDiel);
    float pSpec = pDiel * fDiel;
    float pOpaque = pDiel * (1.0 - p.transmission_weight) * (1.0 - fDiel);
    float pSss = pOpaque * p.subsurface_weight;
    float pDiff = pOpaque * (1.0 - p.subsurface_weight);

    float totalProb = pFuzz + pCoat + pMetal + pSpec + pTrans + pSss + pDiff;
    if (totalProb <= 1e-6) {
        return res;
    }
    float xi = rand01(rng) * totalProb;
    vec2 u = rand2(rng);

    // 1. Fuzz (Charlie sheen).
    if (xi < pFuzz) {
        float pdf;
        res.wo = sampleCosineHemisphere(u, T, B, N, pdf);
        vec3 H = normalize(wi + res.wo);
        float nl = max(dot(N, res.wo), 1e-5);
        float nh = max(dot(N, H), 1e-5);
        float brdf = openpbrSheenBrdf(nv, nl, nh, p.fuzz_roughness);
        res.pdf = max(pdf * (pFuzz / totalProb), 1e-6);
        res.weight = (p.fuzz_weight * p.fuzz_color * brdf * nl) / res.pdf;
        return res;
    }
    xi -= pFuzz;

    // 2. Clear-coat GGX.
    if (xi < pCoat) {
        vec3 hLocal = sampleGgxVndfLocal(vLocal, coatAlpha, u);
        vec3 lLocal = reflect(-vLocal, hLocal);
        if (lLocal.z <= 1e-5) return res;
        res.wo = normalize(lLocal.x * T + lLocal.y * B + lLocal.z * N);
        float vh = max(dot(vLocal, hLocal), 1e-5);
        float Fc = openpbrFresnelDielectric(vh, p.coat_ior);
        float G2OverG1 = ggxG1Local(lLocal, coatAlpha);
        res.pdf = max(pdfGgxVndfReflect(vLocal, hLocal, coatAlpha) * (pCoat / totalProb), 1e-6);
        res.weight = fuzzAtten * p.coat_weight * Fc * G2OverG1 / max(pCoat / totalProb, 1e-6);
        res.isDelta = (p.coat_roughness < 0.01);
        return res;
    }
    xi -= pCoat;

    vec3 baseLayerAtten = fuzzAtten * coatTrans * darkening;

    // 3. Metallic conductor (multi-scatter F82-tint + Airy thin-film).
    if (xi < pMetal) {
        vec3 hLocal = sampleGgxVndfLocal(vLocal, specAlpha, u);
        vec3 lLocal = reflect(-vLocal, hLocal);
        if (lLocal.z <= 1e-5) return res;
        res.wo = normalize(lLocal.x * T + lLocal.y * B + lLocal.z * N);
        float vh = max(dot(vLocal, hLocal), 1e-5);
        vec3 Fm = openpbrFresnelF82Tint(vh, p.base_color, p.specular_color);
        if (p.thin_film_weight > 0.0 && p.thin_film_thickness_nm > 1.0) {
            vec3 Ff = openpbrThinFilmIridescence(vh, p.thin_film_thickness_nm, p.thin_film_ior, p.base_color, effSpecIor, true, thinFilmLut);
            Fm = mix(Fm, Ff, p.thin_film_weight);
        }
        vec3 Favg = openpbrFresnelF82Avg(p.base_color, p.specular_color);
        vec3 ms = openpbrGgxMultiscatterWeight(Favg, ggxLutV);
        float G2OverG1 = ggxG1Local(lLocal, specAlpha);
        res.pdf = max(pdfGgxVndfReflect(vLocal, hLocal, specAlpha) * (pMetal / totalProb), 1e-6);
        res.weight = baseLayerAtten * p.metalness * Fm * ms * G2OverG1 / max(pMetal / totalProb, 1e-6);
        res.isDelta = (effSpecRough < 0.01);
        return res;
    }
    xi -= pMetal;

    // 4. Dielectric specular reflection.
    if (xi < pSpec) {
        vec3 hLocal = sampleGgxVndfLocal(vLocal, specAlpha, u);
        vec3 lLocal = reflect(-vLocal, hLocal);
        if (lLocal.z <= 1e-5) return res;
        res.wo = normalize(lLocal.x * T + lLocal.y * B + lLocal.z * N);
        float vh = max(dot(vLocal, hLocal), 1e-5);
        vec3 Fd = vec3(openpbrFresnelDielectric(vh, eta));
        if (p.thin_film_weight > 0.0 && p.thin_film_thickness_nm > 1.0) {
            vec3 Ff = openpbrThinFilmIridescence(vh, p.thin_film_thickness_nm, p.thin_film_ior, vec3(openpbrF0FromIor(effSpecIor)), effSpecIor, false, thinFilmLut);
            Fd = mix(Fd, Ff, p.thin_film_weight);
        }
        float FavgD = openpbrHemisphericalAlbedo(effSpecIor);
        vec3 msD = openpbrGgxMultiscatterWeight(vec3(FavgD), ggxLutV);
        float G2OverG1 = ggxG1Local(lLocal, specAlpha);
        res.pdf = max(pdfGgxVndfReflect(vLocal, hLocal, specAlpha) * (pSpec / totalProb), 1e-6);
        res.weight = baseLayerAtten * (1.0 - p.metalness) * Fd * p.specular_color * msD * G2OverG1 / max(pSpec / totalProb, 1e-6);
        res.isDelta = (effSpecRough < 0.01);
        return res;
    }
    xi -= pSpec;

    // 5. Dielectric transmission (thin-walled slab or solid refractive glass with dispersion).
    if (xi < pTrans) {
        if (p.thin_walled) {
            float tRough = openpbrThinGlassTransmissionRoughness(effSpecRough, effSpecIor);
            vec2 tAlpha = openpbrAnisotropicRoughness(tRough, p.specular_anisotropy);
            vec3 hLocal = sampleGgxVndfLocal(vLocal, tAlpha, u);
            vec3 rLocal = reflect(-vLocal, hLocal);
            vec3 tLocal = vec3(rLocal.x, rLocal.y, -abs(rLocal.z));
            res.wo = normalize(tLocal.x * T + tLocal.y * B + tLocal.z * N);
            vec3 tint = openpbrSlabTransmittance(p.transmission_color, nv, effSpecIor);
            float prob = max(pTrans / totalProb, 1e-6);
            res.pdf = 1.0;
            res.isDelta = (tRough < 0.02);
            res.transmitted = true;
            res.weight = baseLayerAtten * (1.0 - p.metalness) * p.transmission_weight * (1.0 - fDiel) * tint / prob;
            return res;
        }
        vec3 hLocal = sampleGgxVndfLocal(vLocal, specAlpha, u);
        float invEta = 1.0 / max(eta, 1e-4);
        vec3 tLocal = refract(-vLocal, hLocal, invEta);
        if (dot(tLocal, tLocal) < 1e-6) {
            // Total internal reflection fallback.
            vec3 rLocal = reflect(-vLocal, hLocal);
            res.wo = normalize(rLocal.x * T + rLocal.y * B + rLocal.z * N);
            res.transmitted = false;
        } else {
            res.wo = normalize(tLocal.x * T + tLocal.y * B + tLocal.z * N);
            res.transmitted = true;
        }
        // On-surface tint when depth == 0; volume Beer-Lambert handles depth > 0 inside the ray loop.
        vec3 surfTint = (p.transmission_depth <= 0.0 && !backfacing) ? p.transmission_color : vec3(1.0);
        // Hero-wavelength spectral weight when chromatic dispersion is active.
        vec3 dispWeight = vec3(1.0);
        if (p.dispersion_scale > 0.0) {
            dispWeight = vec3(0.0);
            dispWeight[heroChannel] = 3.0;
        }
        float prob = max(pTrans / totalProb, 1e-6);
        res.pdf = 1.0;
        res.isDelta = (effSpecRough < 0.02);
        res.weight = baseLayerAtten * (1.0 - p.metalness) * p.transmission_weight * (1.0 - fDiel) * surfTint * dispWeight / prob;
        return res;
    }
    xi -= pTrans;

    // 6. Subsurface scattering (thin-walled translucent/diffuse split or solid random-walk BSSRDF).
    if (xi < pSss) {
        float prob = max(pSss / totalProb, 1e-6);
        if (p.thin_walled) {
            float fwdProb = clamp(0.5 * (1.0 + p.subsurface_aniso), 0.0, 1.0);
            float pdf;
            if (rand01(rng) < fwdProb) {
                res.wo = sampleCosineHemisphere(u, T, B, -N, pdf);
                res.transmitted = true;
            } else {
                res.wo = sampleCosineHemisphere(u, T, B, N, pdf);
            }
            res.pdf = max(pdf * prob, 1e-6);
            res.weight = baseLayerAtten * (1.0 - p.metalness) * (1.0 - p.transmission_weight) * (1.0 - fDiel) * p.subsurface_weight * p.subsurface_color / prob;
            return res;
        }
        // Solid random-walk SSS entry: signal the integrator to walk inside the mesh.
        res.enterSubsurface = true;
        res.transmitted = true;
        res.wo = -N;
        res.pdf = prob;
        res.weight = baseLayerAtten * (1.0 - p.metalness) * (1.0 - p.transmission_weight) * (1.0 - fDiel) * p.subsurface_weight * p.subsurface_color / prob;
        return res;
    }

    // 7. Diffuse base (Lambertian or Energy-Preserving Oren-Nayar).
    {
        float pdf;
        res.wo = sampleCosineHemisphere(u, T, B, N, pdf);
        float nl = max(dot(N, res.wo), 1e-5);
        float vl = dot(wi, res.wo);
        vec3 brdf = openpbrOrenNayarEon(p.base_color, p.diffuse_roughness, nv, nl, vl);
        float prob = max(pDiff / totalProb, 1e-6);
        res.pdf = max(pdf * prob, 1e-6);
        res.weight = baseLayerAtten * (1.0 - p.metalness) * (1.0 - p.transmission_weight) * (1.0 - fDiel) * (1.0 - p.subsurface_weight) * (brdf * nl) / res.pdf;
        return res;
    }
}

#endif
