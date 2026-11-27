// Shared ASWF OpenPBR Surface v1.1 mathematical equations (modeled on Blender Cycles
// `openpbr_util.h`, `bsdf_util.h`, `bsdf_oren_nayar.h`, and EEVEE `gpu_shader_material_open_pbr.bsl.hh`).
#ifndef OPENPBR_COMMON_GLSL
#define OPENPBR_COMMON_GLSL

const float OPENPBR_PI = 3.14159265358979323846;
const float OPENPBR_INV_PI = 0.31830988618379067154;
const float OPENPBR_EPS = 1e-5;

float openpbrSqr(float x) { return x * x; }

float openpbrF0FromIor(float ior) {
    float r = (ior - 1.0) / (ior + 1.0);
    return r * r;
}

float openpbrIorFromF0(float f0) {
    float f = sqrt(clamp(f0, 0.0, 0.9999));
    return (1.0 + f) / (1.0 - f);
}

// Exact unpolarized dielectric Fresnel reflectance.
float openpbrFresnelDielectric(float cosI, float eta) {
    float c = abs(cosI);
    float gSq = eta * eta - 1.0 + c * c;
    if (gSq <= 0.0) {
        return 1.0;
    }
    float g = sqrt(gSq);
    float A = (g - c) / (g + c);
    float B = (c * (g + c) - 1.0) / (c * (g - c) + 1.0);
    return 0.5 * A * A * (1.0 + B * B);
}

// d'Eon (v0.3.2 Eq. 11.15 / 11.12) analytical hemispherical Fresnel albedo.
float openpbrHemisphericalAlbedo(float eta) {
    float eta_ = (eta < 1.0) ? (1.0 / eta) : eta;
    float num = 10893.0 * eta_ - 1438.2;
    float den = -774.4 * eta_ * eta_ + 10212.0 * eta_ + 1.0;
    float albedo = log(max(num / den, 1e-7));
    if (eta < 1.0) {
        albedo = 1.0 - eta * eta * (1.0 - albedo);
    }
    return clamp(albedo, 0.0, 1.0);
}

// OpenPBR v1.1 Eq. (25-26): modulate dielectric IOR by specular_weight.
float openpbrModulateIor(float ior, float weight) {
    if (abs(weight - 1.0) < 1e-6) {
        return ior;
    }
    float modulated = openpbrIorFromF0(weight * openpbrF0FromIor(ior));
    return (ior < 1.0) ? (1.0 / modulated) : modulated;
}

// OpenPBR v1.1 Eq. (60, 74): effective specular IOR beneath coat with TIR compensation.
float openpbrEffectiveSpecularIor(float specIor, float specWeight, float coatIor, float coatWeight) {
    float ior = max(specIor, OPENPBR_EPS);
    if (coatWeight > 0.0) {
        float cIor = max(coatIor, OPENPBR_EPS);
        float tirRatio = (cIor > ior) ? (cIor / ior) : (ior / cIor);
        ior = mix(ior, tirRatio, coatWeight);
    }
    return openpbrModulateIor(ior, specWeight);
}

// OpenPBR v1.1 Eq. (73): roughening of base specular layer beneath rough coat.
float openpbrCoatRoughening(float specRoughness, float coatRoughness, float coatWeight) {
    if (coatWeight <= 0.0 || coatRoughness <= 0.0) {
        return specRoughness;
    }
    float r4 = openpbrSqr(openpbrSqr(specRoughness));
    float c4 = openpbrSqr(openpbrSqr(coatRoughness));
    float coated = min(1.0, pow(r4 + 2.0 * c4, 0.25));
    return mix(specRoughness, coated, coatWeight);
}

// Anisotropic microfacet roughness (alpha_x, alpha_y) from OpenPBR (roughness, anisotropy).
vec2 openpbrAnisotropicRoughness(float roughness, float anisotropy) {
    float r2 = max(roughness * roughness, 0.002);
    float factor = sqrt(1.0 - 0.9 * clamp(anisotropy, 0.0, 1.0));
    return vec2(min(1.0, r2 / factor), max(0.002, r2 * factor));
}

// Beer-Lambert slab transmittance at incident angle cosThetaI.
vec3 openpbrSlabTransmittance(vec3 color, float cosThetaI, float ior) {
    float sin2 = max(0.0, 1.0 - cosThetaI * cosThetaI);
    float invCosT = ior / sqrt(max(ior * ior - sin2, OPENPBR_EPS));
    return pow(clamp(color, vec3(OPENPBR_EPS), vec3(1.0)), vec3(invCosT));
}

float openpbrThinGlassTransmissionRoughness(float roughness, float ior) {
    float eta = max(ior, 1.0);
    float scale = sqrt(max(0.0, 3.4 * (eta - 1.0) * openpbrSqr(eta - 0.5) / (eta * eta * eta)));
    return clamp(roughness * scale, 0.0, 1.0);
}

// OpenPBR v1.1 Eq. (64-70): coat internal-reflection darkening factor Delta.
vec3 openpbrCoatDarkening(
    float coatCosI,
    vec3 baseColor,
    float metalness,
    vec3 specColor,
    float specIor,
    float specRoughness,
    float transWeight,
    vec3 transColor,
    float sssWeight,
    vec3 sssColor,
    float coatWeight,
    float coatIor,
    float coatDarkening
) {
    if (coatWeight <= 0.0 || coatDarkening <= 0.0) {
        return vec3(1.0);
    }
    float F0 = openpbrF0FromIor(specIor);
    vec3 E_glossy = F0 * specColor;
    vec3 E_gd = E_glossy + (1.0 - E_glossy) * baseColor;
    vec3 E_opaque = mix(E_gd, sssColor, sssWeight);
    vec3 E_trans = E_glossy + (1.0 - F0) * transColor;
    vec3 E_diel = mix(E_opaque, E_trans, transWeight);
    vec3 E_b = mix(E_diel, baseColor, metalness);
    if (all(greaterThanEqual(E_b, vec3(0.99999)))) {
        return vec3(1.0);
    }
    float E_F = openpbrHemisphericalAlbedo(coatIor);
    float K_r = 1.0 - (1.0 - E_F) / (coatIor * coatIor);
    float K_s = openpbrFresnelDielectric(coatCosI, coatIor);
    float r_d = mix(1.0, specRoughness, F0);
    float r_b = mix(r_d, specRoughness, metalness);
    float K = mix(K_s, K_r, r_b);
    vec3 Delta = (1.0 - K) / max(vec3(1e-5), vec3(1.0) - E_b * K);
    return mix(vec3(1.0), Delta, coatWeight * coatDarkening);
}

// Conductor F82-tint Fresnel (OpenPBR v1.1 / Kutz et al.).
vec3 openpbrFresnelF82Tint(float cosI, vec3 F0, vec3 tint) {
    float s = clamp(1.0 - cosI, 0.0, 1.0);
    float s2 = s * s;
    float s5 = s2 * s2 * s;
    const float f = 6.0 / 7.0;
    const float f5 = f * f * f * f * f;
    const float invScale = 7.0 / (f5 * f);
    vec3 F_schlick_82 = mix(F0, vec3(1.0), f5);
    vec3 B = F_schlick_82 * invScale * (vec3(1.0) - tint);
    vec3 F_schlick = mix(F0, vec3(1.0), s5);
    return clamp(F_schlick - B * max(cosI, 0.0) * s5 * s, vec3(0.0), vec3(1.0));
}

// Average single-scattering F82 Fresnel for multi-scattering compensation.
vec3 openpbrFresnelF82Avg(vec3 F0, vec3 tint) {
    const float f = 6.0 / 7.0;
    const float f5 = f * f * f * f * f;
    const float invScale = 7.0 / (f5 * f);
    vec3 F_schlick_82 = mix(F0, vec3(1.0), f5);
    vec3 B = F_schlick_82 * invScale * (vec3(1.0) - tint);
    return clamp(mix(F0, vec3(1.0), 1.0 / 21.0) - B * (1.0 / 126.0), vec3(0.0), vec3(1.0));
}

// Airy thin-film interference iridescence (Belcour & Barla 2017, spectral Fourier LUT).
vec3 openpbrThinFilmIridescence(
    float cosTheta1,
    float filmThicknessNm,
    float filmIor,
    vec3 baseF0,
    float baseIor,
    bool isConductor,
    sampler2D thinFilmLut
) {
    if (filmThicknessNm <= 1.0) {
        return isConductor ? baseF0 : vec3(openpbrFresnelDielectric(cosTheta1, baseIor));
    }
    float sin2Theta2 = (1.0 - cosTheta1 * cosTheta1) / (filmIor * filmIor);
    if (sin2Theta2 >= 1.0) {
        return vec3(1.0);
    }
    float cosTheta2 = sqrt(max(1.0 - sin2Theta2, 0.0));
    // Air-film interface reflectance R12 and phase shift (pi when filmIor > 1).
    float R12 = openpbrFresnelDielectric(cosTheta1, filmIor);
    float T121 = 1.0 - R12;
    // Film-substrate reflectance R23.
    vec3 R23 = isConductor
        ? openpbrFresnelF82Tint(cosTheta2, baseF0, vec3(1.0))
        : vec3(openpbrFresnelDielectric(cosTheta2, baseIor / filmIor));
    vec3 R123 = clamp(R12 * R23, vec3(0.0), vec3(0.9999));
    vec3 r123 = sqrt(R123);
    vec3 Rs = (T121 * T121) * R23 / max(vec3(1e-5), vec3(1.0) - R123);
    // DC term C0 + first 2 Airy harmonics using precomputed CIE XYZ Fourier sensitivity LUT.
    vec3 C0 = vec3(R12) + Rs;
    float opd = 2.0 * filmIor * filmThicknessNm * cosTheta2;
    vec3 phaseSign = isConductor ? vec3(1.0) : ((baseIor > filmIor) ? vec3(1.0) : vec3(-1.0));
    vec3 I = C0;
    vec3 Cm = Rs - vec3(T121);
    for (int m = 1; m <= 2; ++m) {
        Cm *= r123;
        float u = clamp((float(m) * opd) / 3200.0, 0.0, 1.0);
        vec3 S = texture(thinFilmLut, vec2(u, 0.5)).rgb * phaseSign;
        I += 2.0 * Cm * S;
    }
    return clamp(I, vec3(0.0), vec3(1.0));
}

// Anisotropic GGX NDF D(h) in local tangent space (T, B, N).
float openpbrGgxAnisoD(vec3 hLocal, vec2 alpha) {
    float ax = max(alpha.x, 0.002);
    float ay = max(alpha.y, 0.002);
    float d = openpbrSqr(hLocal.x / ax) + openpbrSqr(hLocal.y / ay) + openpbrSqr(hLocal.z);
    return 1.0 / max(OPENPBR_PI * ax * ay * d * d, 1e-8);
}

// Height-correlated Smith G2 / (4 * nv * nl) visibility term for anisotropic GGX.
float openpbrGgxAnisoV(vec3 vLocal, vec3 lLocal, vec2 alpha) {
    float ax = max(alpha.x, 0.002);
    float ay = max(alpha.y, 0.002);
    float lambdaV = lLocal.z * length(vec3(ax * vLocal.x, ay * vLocal.y, vLocal.z));
    float lambdaL = vLocal.z * length(vec3(ax * lLocal.x, ay * lLocal.y, lLocal.z));
    return 0.5 / max(lambdaV + lambdaL, 1e-7);
}

// Multi-scattering GGX compensation factor (Turquin / Kulla-Conty) using GGX LUT (E, E_avg).
vec3 openpbrGgxMultiscatterWeight(vec3 Favg, vec2 ggxLutSample) {
    float E = max(ggxLutSample.x, 0.02);
    float Eavg = clamp(ggxLutSample.y, 0.02, 0.999);
    vec3 Fms = (Favg * Favg * Eavg) / max(vec3(1e-5), vec3(1.0) - Favg * (1.0 - Eavg));
    return vec3(1.0) + Fms * ((1.0 - E) / E);
}

// Maps dielectric IOR to 3D dielectric LUT Z coordinate (matches `openpbr_luts.rs`).
float openpbrDielectricIorToLutZ(float ior) {
    float eta = (ior < 1.0) ? (1.0 / max(ior, 1e-4)) : ior;
    float s = clamp(sqrt(openpbrF0FromIor(eta)), 0.0, 0.85) / 0.85;
    return (ior < 1.0) ? (0.5 * (1.0 - s)) : (0.5 * (1.0 + s));
}

// Energy-preserving Oren-Nayar (EON) BRDF (Fujii + OpenPBR multi-scatter compensation).
float openpbrOrenNayarG(float cosTheta) {
    if (cosTheta < 1e-6) {
        return (OPENPBR_PI * 0.5 - 2.0 / 3.0) - cosTheta;
    }
    float sinTheta = sqrt(max(0.0, 1.0 - cosTheta * cosTheta));
    float theta = acos(clamp(cosTheta, -1.0, 1.0));
    return sinTheta * (theta - 2.0 / 3.0 - sinTheta * cosTheta)
         + (2.0 / 3.0) * (sinTheta / cosTheta) * (1.0 - sinTheta * sinTheta * sinTheta);
}

vec3 openpbrOrenNayarEon(vec3 albedo, float roughness, float nv, float nl, float vl) {
    float sigma = clamp(roughness, 0.0, 1.0);
    if (sigma < 1e-5) {
        return albedo * OPENPBR_INV_PI;
    }
    float a = 1.0 / (OPENPBR_PI + sigma * (OPENPBR_PI * 0.5 - 2.0 / 3.0));
    float b = sigma * a;
    float t = vl - nl * nv;
    if (t > 0.0) {
        t /= max(max(nl, nv), 1e-7);
    }
    float single = a + b * t;
    float Eavg = a * OPENPBR_PI + ((2.0 * OPENPBR_PI - 5.6) / 3.0) * b;
    float Ev = a * OPENPBR_PI + b * openpbrOrenNayarG(max(nv, 0.0));
    float El = a * OPENPBR_PI + b * openpbrOrenNayarG(max(nl, 0.0));
    vec3 rho = clamp(albedo, vec3(0.0), vec3(1.0));
    vec3 Ems = (rho * rho * OPENPBR_INV_PI) * (Eavg / max(1.0 - Eavg, 1e-6))
             / max(vec3(1e-6), vec3(1.0) - rho * (1.0 - Eavg));
    return rho * single + Ems * (1.0 - Ev) * (1.0 - El);
}

// Charlie microfiber sheen BRDF (Estevez & Kulla 2017).
float openpbrCharlieD(float cosH, float roughness) {
    float alpha = max(roughness * roughness, 0.01);
    float invR = 1.0 / alpha;
    float sin2 = max(0.0, 1.0 - cosH * cosH);
    return (2.0 + invR) * pow(sin2, 0.5 * invR) / (2.0 * OPENPBR_PI);
}

float openpbrCharlieL(float x, float r) {
    float w = openpbrSqr(1.0 - r);
    float a = mix(25.3245, 21.5473, w);
    float b = mix(3.32435, 3.82987, w);
    float c = mix(0.16801, 0.19823, w);
    float d = mix(-1.27393, -1.97760, w);
    float e = mix(-4.85967, -4.32054, w);
    return a / (1.0 + b * pow(max(x, 1e-4), c)) + d * x + e;
}

float openpbrCharlieLambda(float cosTheta, float r) {
    return (cosTheta < 0.5)
        ? exp(openpbrCharlieL(cosTheta, r))
        : exp(2.0 * openpbrCharlieL(0.5, r) - openpbrCharlieL(1.0 - cosTheta, r));
}

float openpbrSheenBrdf(float nv, float nl, float nh, float roughness) {
    float r = clamp(roughness, 0.05, 1.0);
    float D = openpbrCharlieD(nh, r);
    float V = 1.0 / max(4.0 * (nl + nv - nl * nv) * (1.0 + openpbrCharlieLambda(nv, r) + openpbrCharlieLambda(nl, r)), 1e-5);
    return D * V;
}

// HDR Display Tone Mapping: 0 = Disabled, 1 = Filmic, 2 = AgX (Blender), 3 = ACES (Hill fit).
vec3 openpbrTonemapAgX(vec3 x) {
    x = max(x, vec3(0.0));
    const mat3 mIn = mat3(
        0.84247905, 0.04232824, 0.04237565,
        0.07843360, 0.87846863, 0.07843360,
        0.07922375, 0.07916613, 0.87914294
    );
    const mat3 mOut = mat3(
         1.19687940, -0.05289685, -0.05297163,
        -0.09802088,  1.15190310, -0.09804345,
        -0.09902975, -0.09896118,  1.15107370
    );
    vec3 v = mIn * x;
    const float minEv = -12.47393;
    const float maxEv = 4.026069;
    vec3 u = (clamp(log2(max(v, vec3(1e-10))), vec3(minEv), vec3(maxEv)) - minEv) / (maxEv - minEv);
    vec3 u2 = u * u;
    vec3 u4 = u2 * u2;
    vec3 sig = 15.5 * u4 * u2 - 40.14 * u4 * u + 31.96 * u4 - 6.868 * u2 * u + 0.4298 * u2 + 0.1191 * u - 0.00232;
    return clamp(mOut * sig, vec3(0.0), vec3(1.0));
}

vec3 openpbrTonemapAces(vec3 x) {
    x = max(x, vec3(0.0));
    const mat3 mIn = mat3(
        0.59719, 0.07600, 0.02840,
        0.35458, 0.90834, 0.13383,
        0.04823, 0.01566, 0.83777
    );
    const mat3 mOut = mat3(
         1.60475, -0.10208, -0.00327,
        -0.53108,  1.10813, -0.07276,
        -0.07367, -0.00605,  1.07602
    );
    vec3 v = mIn * x;
    vec3 a = v * (v + 0.0245786) - 0.000090537;
    vec3 b = v * (0.983729 * v + 0.4329510) + 0.238081;
    return clamp(mOut * (a / b), vec3(0.0), vec3(1.0));
}

vec3 openpbrTonemapFilmic(vec3 x, float whitePoint) {
    x = max(x, vec3(0.0));
    const float A = 0.22, B = 0.30, C = 0.10, D = 0.20, E = 0.01, F = 0.30;
    vec3 num = x * (A * x + C * B) + D * E;
    vec3 den = x * (A * x + B) + D * F;
    float w = max(whitePoint, 1e-4);
    float wVal = ((w * (A * w + C * B) + D * E) / (w * (A * w + B) + D * F)) - E / F;
    return clamp((num / den - E / F) / max(wVal, 1e-5), vec3(0.0), vec3(1.0));
}

vec3 openpbrApplyToneMapping(vec3 hdr, uint method, float exposure, float whitePoint) {
    vec3 exposed = hdr * exp2(exposure);
    if (method == 1u) {
        return openpbrTonemapFilmic(exposed, whitePoint);
    } else if (method == 2u) {
        return openpbrTonemapAgX(exposed);
    } else if (method == 3u) {
        return openpbrTonemapAces(exposed);
    }
    return clamp(exposed, vec3(0.0), vec3(1.0));
}

#endif
