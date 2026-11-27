// the reference implementation: world-space Phong family, including legacy intensity and cone quirks.
#include "native_dot.glsl"
#ifdef MARTENSITE_NUMERIC_LIGHT_DOT_V2
#include "numeric_light_dot.glsl"
#endif
struct RenderLight { uvec4 kind; vec4 color; vec4 position; vec4 direction; vec4 params; };
#ifdef MARTENSITE_PBR
#include "openpbr/openpbr_types.glsl"
layout(set=0,binding=0,std430) readonly buffer SurfaceLighting {
    mat4 worldFromObject;
    mat4 normalFromObject;
    vec4 eye;
    vec4 ambient;
    vec4 materials[8];
    uvec4 settings;
    vec4 tail[];
} lighting;
RenderLight loadRenderLight(uint index) {
    uint base = index * 5u;
    RenderLight l;
    l.kind = floatBitsToUint(lighting.tail[base]);
    l.color = lighting.tail[base + 1u];
    l.position = lighting.tail[base + 2u];
    l.direction = lighting.tail[base + 3u];
    l.params = lighting.tail[base + 4u];
    return l;
}
OpenPbrGpuMaterial loadOpenPbrMaterial(uint face) {
    uint base = lighting.settings.y * 5u + face * 16u;
    OpenPbrGpuMaterial m;
    m.base_color_weight = lighting.tail[base + 0u];
    m.base_spec_params = lighting.tail[base + 1u];
    m.specular_color_aniso = lighting.tail[base + 2u];
    m.specular_ior_trans = lighting.tail[base + 3u];
    m.transmission_color_disp = lighting.tail[base + 4u];
    m.transmission_scatter_abbe = lighting.tail[base + 5u];
    m.subsurface_color_weight = lighting.tail[base + 6u];
    m.subsurface_radius_scale = lighting.tail[base + 7u];
    m.coat_color_weight = lighting.tail[base + 8u];
    m.coat_params = lighting.tail[base + 9u];
    m.fuzz_color_weight = lighting.tail[base + 10u];
    m.fuzz_thin_film = lighting.tail[base + 11u];
    m.emission_color_lum = lighting.tail[base + 12u];
    m.geometry_meta = lighting.tail[base + 13u];
    m.uv_transform = lighting.tail[base + 14u];
    m.tex_indices = lighting.tail[base + 15u];
    return m;
}
#else
layout(set=0,binding=0,std430) readonly buffer SurfaceLighting {
    mat4 worldFromObject;
    mat4 normalFromObject;
    vec4 eye;
    vec4 ambient;
    vec4 materials[8];
    uvec4 settings;
    RenderLight sources[];
} lighting;
#endif
#ifdef MARTENSITE_INDEXED_DRAW_MATERIALS
#include "draw_material_table.glsl"
#else
#define materialCoefficient(slot) lighting.materials[slot]
#endif
float pointAttenuation(float distance, RenderLight light) {
    float range = light.direction.w;
    if (range == 0.0) return 1.0 / (light.params.x + light.params.y * distance);
    if (distance > range) return -1.0;
    float ratio = distance / range;
    ratio *= ratio;
    ratio *= ratio;
    return clamp(1.0 - ratio, 0.0, 1.0) / max(0.0001, distance * distance);
}
// GL 2.1 §2.14.1 points at infinity: a vertex V with w=0 sees every finite light along -V̂
// at infinite distance, so 1/(k0+k1*d) is 0 unless k1 is 0 (the reference implementation has no quadratic term).
float infiniteVertexAttenuation(RenderLight light) {
    if (light.direction.w != 0.0) return -1.0;
    return light.params.y > 0.0 ? 0.0 : 1.0 / light.params.x;
}
float spotAttenuation(vec3 direction, RenderLight light) {
    float cosine = nativeDot3(light.direction.xyz, -direction);
    // Native exact-axis spot lights are dark; retain this observable behavior.
    if (cosine >= 1.0 || cosine < cos(light.params.z)) return 0.0;
    return light.params.w > 0.0 ? pow(cosine, light.params.w * 128.0) : 1.0;
}
#if !defined(MARTENSITE_FIXED_LIGHTING) && !defined(MARTENSITE_ORTHOGRAPHIC_DIRECTIONAL_PHONG)
#include "light_shadow.glsl"
#else
// Fixed-function lighting has no shadow maps, as natively.
float lightShadow(uint theId, RenderLight theLight, vec3 theNormal, vec4 thePositionWorld) { return 1.0; }
#endif
#ifdef MARTENSITE_FIXED_LIGHTING
#include "fixed_lighting.glsl"
#endif
#ifdef MARTENSITE_PBR
#include "openpbr/openpbr_raster.glsl"
#endif
vec4 computeLighting(vec3 normal, vec3 view, vec4 world, bool front, vec4 vertexColor) {
#if defined(MARTENSITE_PBR)
    return computePbrLighting(normal, view, world, front, vertexColor);
#elif defined(MARTENSITE_FIXED_LIGHTING)
    return computeFixedLighting(normal,world,front,vertexColor);
#else
    uint face = front ? 0u : 4u;
    vec3 diffuse = vec3(0.0), specular = vec3(0.0);
    vec3 faceNormal = front ? normal : -normal;
    for (uint i=0u; i<lighting.settings.y; ++i) {
        RenderLight light = lighting.sources[i];
        vec3 direction = light.position.xyz;
        float attenuation = 1.0;
#ifndef MARTENSITE_ORTHOGRAPHIC_DIRECTIONAL_PHONG
        if (light.kind.x != 1u && world.w == 0.0) {
            direction = -glslNormalize(world.xyz);
            attenuation = infiniteVertexAttenuation(light);
            if (attenuation <= 0.0) continue;
            if (light.kind.x == 3u) attenuation *= spotAttenuation(direction, light);
        } else if (light.kind.x != 1u) {
            direction -= world.xyz / world.w;
            // length() as the native sqrt(dot(x, x)).
            float distance = sqrt(nativeDot3(direction, direction));
            attenuation = pointAttenuation(distance, light);
            if (attenuation <= 0.0) continue;
            direction /= distance;
            if (light.kind.x == 3u) attenuation *= spotAttenuation(direction, light);
        }
#endif
#ifdef MARTENSITE_NUMERIC_LIGHT_DOT_V2
        float ndotl = numericLightDotMaxZero(nativeDot3(faceNormal, direction));
        float ndoth = numericLightDotMaxZero(nativeDot3(faceNormal, glslNormalize(direction + view)));
#else
        float ndotl = max(0.0, nativeDot3(faceNormal, direction));
        float ndoth = max(0.0, nativeDot3(faceNormal, glslNormalize(direction + view)));
#endif
        float shine = ndotl > 0.0 ? pow(ndoth, materialCoefficient(face+2u).w) : 0.0;
        float shadow = lightShadow(i, light, normal, world);
        diffuse += light.color.rgb * ndotl * attenuation * shadow;
        specular += light.color.rgb * shine * attenuation * shadow;
    }
    vec4 result = vec4(lighting.ambient.rgb * materialCoefficient(face+3u).rgb
        + diffuse * materialCoefficient(face).rgb + specular * materialCoefficient(face+2u).rgb,
        materialCoefficient(face).a);
#ifndef MARTENSITE_ORTHOGRAPHIC_DIRECTIONAL_PHONG
    if (lighting.settings.z != 0u && (front || lighting.settings.w == 0u)) result *= vertexColor;
#endif
#if defined(MARTENSITE_FRAGMENT) && defined(MARTENSITE_SURFACE_FAMILY)
    // Native Phong modulates the lit terms before adding untextured emission; Gouraud modulates later.
    result *= surfaceTexel();
#endif
#ifdef MARTENSITE_MATERIAL_MAPS
    // occMaterialOcclusion, then occMaterialEmission with the emissive map.
    result.rgb *= occlusionMapTexel();
    result.rgb += materialCoefficient(face+1u).rgb * emissiveMapTexel();
#else
    result.rgb += materialCoefficient(face+1u).rgb;
#endif
    return result;
#endif
}
