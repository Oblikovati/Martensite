// the reference implementation metallic-roughness shading: resources/Shaders/PBR*.glsl, PointLightAttenuation.glsl,
// DeclarationsImpl.glsl (occDiffIBLMap, occRoughness) and the PBR branch of
// Graphic3dShaderManager::stdComputeLighting, in the same arithmetic order. The PBR material
// replaces the common records: materials[face + 0..2] hold base colour, emission/IOR and the
// (reserved, roughness, metallic, reserved) parameters of GlMaterialPBR.
layout(set=4,binding=0) uniform sampler2D pbrEnvLut;
layout(set=4,binding=1) uniform sampler2D pbrDiffuseSH;
layout(set=4,binding=2) uniform samplerCube pbrSpecularMap;
struct PbrSurface { vec4 baseColor; float metallic; float normalizedRoughness; float roughness; vec3 emission; float ior; };
#include "pbr_brdf.glsl"
float occPBRGeometry(in float theCosV, in float theCosL, in float theRoughness) {
    float k = theRoughness + 1.0;
    k *= 0.125 * k;
    float g1 = 1.0;
    g1 /= theCosV * (1.0 - k) + k;
    float g2 = 1.0;
    g2 /= theCosL * (1.0 - k) + k;
    return g1 * g2;
}
vec3 occPBRFresnel(in vec3 theBaseColor, in float theMetallic, in float theIOR) {
    theIOR = (1.0 - theIOR) / (1.0 + theIOR);
    theIOR *= theIOR;
    vec3 f0 = vec3(theIOR);
    f0 = mix(f0, theBaseColor.rgb, theMetallic);
    return f0;
}
vec3 occPBRFresnel(in vec3 theBaseColor, in float theMetallic, in float theIOR, in float theCosVH) {
    vec3 f0 = occPBRFresnel(theBaseColor, theMetallic, theIOR);
    theCosVH = 1.0 - theCosVH;
    theCosVH *= theCosVH;
    theCosVH *= theCosVH * theCosVH * theCosVH * theCosVH;
    return f0 + (vec3(1.0) - f0) * theCosVH;
}
vec3 occPBRFresnel(in vec3 theBaseColor, in float theMetallic, in float theRoughness, in float theIOR, in float theCosV) {
    vec3 f0 = occPBRFresnel(theBaseColor, theMetallic, theIOR);
    theCosV = 1.0 - theCosV;
    theCosV *= theCosV;
    theCosV *= theCosV * theCosV * theCosV * theCosV;
    return f0 + (max(vec3(1.0 - theRoughness), f0) - f0) * theCosV;
}
vec3 occPBRCookTorrance(in vec3 theView, in vec3 theLight, in vec3 theNormal, in vec3 theBaseColor,
                        in float theMetallic, in float theRoughness, in float theIOR) {
    vec3 aHalf = normalize(theView + theLight);
    float aCosV = max(dot(theView, theNormal), 0.0);
    float aCosL = max(dot(theLight, theNormal), 0.0);
    float aCosH = max(dot(aHalf, theNormal), 0.0);
    float aCosVH = max(dot(aHalf, theView), 0.0);
    vec3 aCookTorrance = occPBRDistribution(aCosH, theRoughness)
                       * occPBRGeometry(aCosV, aCosL, theRoughness)
                       * occPBRFresnel(theBaseColor, theMetallic, theIOR, aCosVH);
    aCookTorrance /= 4.0;
    return aCookTorrance;
}
vec3 occPBRIllumination(in vec3 theView, in vec3 theLight, in vec3 theNormal, in PbrSurface theSurface,
                        in vec3 theLightColor, in float theLightIntensity) {
    vec3 aHalf = normalize(theView + theLight);
    float aCosVH = max(dot(theView, aHalf), 0.0);
    vec3 aFresnel = occPBRFresnel(theSurface.baseColor.rgb, theSurface.metallic, theSurface.ior, aCosVH);
    vec3 aSpecular = occPBRCookTorrance(theView, theLight, theNormal, theSurface.baseColor.rgb,
                                        theSurface.metallic, theSurface.roughness, theSurface.ior);
    vec3 aDiffuse = vec3(1.0) - aFresnel;
    aDiffuse *= 1.0 - theSurface.metallic;
    aDiffuse *= INV_PI;
    aDiffuse *= theSurface.baseColor.rgb;
    aDiffuse = mix(vec3(0.0), aDiffuse, theSurface.baseColor.a);
    return (aDiffuse + aSpecular) * theLightColor * theLightIntensity * max(0.0, dot(theLight, theNormal));
}
float occRangedPointLightAttenuation(in float theDistance, in float theRange) {
    if (theDistance <= theRange) {
        float aResult = theDistance / theRange;
        aResult *= aResult;
        aResult *= aResult;
        aResult = 1.0 - aResult;
        aResult = clamp(aResult, 0.0, 1.0);
        aResult /= max(0.0001, theDistance * theDistance);
        return aResult;
    }
    return -1.0;
}
float occPointLightAttenuation(in float theDistance, in float theRange) {
    if (theRange == 0.0) return 1.0 / max(0.0001, theDistance * theDistance);
    return occRangedPointLightAttenuation(theDistance, theRange);
}
vec3 occDiffIBLMap(in vec3 theNormal) {
    vec3 aSHCoeffs[9];
    for (int i = 0; i < 9; ++i) {
        aSHCoeffs[i] = texture(pbrDiffuseSH, vec2((float(i) + 0.5) / 9.0, 0.0)).rgb;
    }
    return aSHCoeffs[0]
         + aSHCoeffs[1] * theNormal.x
         + aSHCoeffs[2] * theNormal.y
         + aSHCoeffs[3] * theNormal.z
         + aSHCoeffs[4] * theNormal.x * theNormal.z
         + aSHCoeffs[5] * theNormal.y * theNormal.z
         + aSHCoeffs[6] * theNormal.x * theNormal.y
         + aSHCoeffs[7] * (3.0 * theNormal.z * theNormal.z - 1.0)
         + aSHCoeffs[8] * (theNormal.x * theNormal.x - theNormal.y * theNormal.y);
}
// PBRSpotLight.glsl angular attenuation; a negative result skips the source.
float pbrSpotAttenuation(in RenderLight theLight, in vec3 theDirection, in float theAttenuation) {
    vec3 aSpotDir = theLight.direction.xyz;
    float aCosA = dot(aSpotDir, -theDirection);
    float aRelativeAngle = 2.0 * acos(aCosA) / theLight.params.z;
    if (aCosA >= 1.0 || aRelativeAngle > 1.0) return -1.0;
    float anExponent = theLight.params.w;
    if ((1.0 - aRelativeAngle) <= anExponent) {
        float anAngularAttenuationOffset = cos(0.5 * theLight.params.z);
        float anAngularAttenuationScale = 1.0 / max(0.001, cos(0.5 * theLight.params.z * (1.0 - anExponent)) - anAngularAttenuationOffset);
        anAngularAttenuationOffset *= -anAngularAttenuationScale;
        float anAngularAttenuantion = clamp(aCosA * anAngularAttenuationScale + anAngularAttenuationOffset, 0.0, 1.0);
        theAttenuation *= anAngularAttenuantion * anAngularAttenuantion;
    }
    return theAttenuation;
}
// One occDirectionalLight / occPointLight / occSpotLight term of the native light loop.
vec3 pbrDirectLight(in RenderLight theLight, in vec3 theNormal, in vec3 theView, in vec3 thePoint,
                    in bool theIsFront, in PbrSurface theSurface) {
    vec3 aLight = theLight.position.xyz;
    float anIntensity = theLight.color.a;
    if (theLight.kind.x != 1u) {
        aLight = theLight.position.xyz - thePoint;
        float aDist = length(aLight);
        float anAtten = occPointLightAttenuation(aDist, theLight.direction.w);
        if (anAtten <= 0.0) return vec3(0.0);
        aLight /= aDist;
        if (theLight.kind.x == 3u) anAtten = pbrSpotAttenuation(theLight, aLight, anAtten);
        if (anAtten < 0.0) return vec3(0.0);
        anIntensity *= anAtten;
    }
    theNormal = theIsFront ? theNormal : -theNormal;
    return occPBRIllumination(theView, aLight, theNormal, theSurface, theLight.color.rgb, anIntensity);
}
PbrSurface pbrSurface(in bool theIsFront, in vec4 theVertexColor) {
    uint face = theIsFront ? 0u : 4u;
    PbrSurface aSurface;
    aSurface.baseColor = lighting.materials[face];
#if defined(MARTENSITE_FRAGMENT) && defined(MARTENSITE_SURFACE_FAMILY)
    aSurface.baseColor *= surfaceTexel();
#endif
    if (lighting.settings.z != 0u) aSurface.baseColor *= (theIsFront || (lighting.settings.w & 1u) == 0u) ? theVertexColor : vec4(1.0);
    aSurface.emission = lighting.materials[face + 1u].rgb;
    aSurface.metallic = lighting.materials[face + 2u].b;
    aSurface.normalizedRoughness = lighting.materials[face + 2u].g;
#ifdef MARTENSITE_MATERIAL_MAPS
    aSurface.emission *= emissiveMapTexel();
    vec2 aMetallicRoughness = metallicRoughnessMapTexel();
    aSurface.metallic *= aMetallicRoughness.x;
    aSurface.normalizedRoughness *= aMetallicRoughness.y;
#endif
    aSurface.roughness = aSurface.normalizedRoughness * (1.0 - 0.01) + 0.01;
    aSurface.ior = lighting.materials[face + 1u].w;
    return aSurface;
}
vec4 computePbrLighting(vec3 theNormal, vec3 theView, vec4 thePoint, bool theIsFront, vec4 theVertexColor) {
    PbrSurface aSurface = pbrSurface(theIsFront, theVertexColor);
    vec3 aPoint = thePoint.xyz / thePoint.w;
    vec3 aColor = vec3(0.0);
    for (uint i = 0u; i < lighting.settings.y; ++i) {
        aColor += pbrDirectLight(lighting.sources[i], theNormal, theView, aPoint, theIsFront, aSurface)
            * lightShadow(i, lighting.sources[i], theNormal, thePoint);
    }
    vec3 anIndirectLightingSpec = occPBRFresnel(aSurface.baseColor.rgb, aSurface.metallic, aSurface.ior);
    vec2 aCoeff = texture(pbrEnvLut, vec2(abs(dot(theView, theNormal)), aSurface.normalizedRoughness)).xy;
    anIndirectLightingSpec *= aCoeff.x;
    anIndirectLightingSpec += aCoeff.y;
    anIndirectLightingSpec *= textureLod(pbrSpecularMap, -reflect(theView, theNormal),
        aSurface.normalizedRoughness * float(textureQueryLevels(pbrSpecularMap) - 1)).rgb;
    vec3 aRefractionCoeff = 1.0 - occPBRFresnel(aSurface.baseColor.rgb, aSurface.metallic,
        aSurface.normalizedRoughness, aSurface.ior, abs(dot(theView, theNormal)));
    aRefractionCoeff *= (1.0 - aSurface.metallic);
    vec3 anIndirectLightingDiff = aRefractionCoeff * aSurface.baseColor.rgb * aSurface.baseColor.a;
    anIndirectLightingDiff *= occDiffIBLMap(theNormal).rgb;
    aColor += lighting.ambient.rgb * (anIndirectLightingDiff + anIndirectLightingSpec);
    aColor += aSurface.emission;
#ifdef MARTENSITE_MATERIAL_MAPS
    aColor *= occlusionMapTexel();
#endif
    return vec4(aColor, mix(1.0, aSurface.baseColor.a, aRefractionCoeff.x));
}
