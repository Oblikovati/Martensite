// the reference implementation's LightShadow.glsl: directional and spot shadow-map attenuation (1.0 means no
// shadow) of the Phong-family and PBR fragment programs. The shadow maps of the view's casting
// lights are the layers of one array, in caster order; as natively, light `i` of the loop
// samples map `i` (occShadowMapSamplers[anIndex]). As in getStdProgramPhong, the vertex stage
// computes PosLightSpace = occShadowMapMatrices[i] * PositionWorld, for the first
// MARTENSITE_VERTEX_SHADOW_MAPS maps (the varying budget); further maps, and every map of the
// mesh-edge programs (whose geometry stage has no input locations left on AMD), transform the
// interpolated world position per fragment.
layout(set=6,binding=1,std430) readonly buffer ShadowMaps {
    // x: 1 / ShadowMapResolution, y: ShadowMapBias, z: the number of maps.
    vec4 sizeBias;
    mat4 matrices[];
} occShadowMaps;
#define MARTENSITE_VERTEX_SHADOW_MAPS 4
// Mesa's GLSL lowering of a matrix-vector product: columns left to right (see clip_position).
vec4 lightSpacePosition(uint theId, vec4 thePositionWorld) {
    mat4 aMatrix = occShadowMaps.matrices[theId];
    return ((aMatrix[0] * thePositionWorld.x + aMatrix[1] * thePositionWorld.y)
        + aMatrix[2] * thePositionWorld.z) + aMatrix[3] * thePositionWorld.w;
}
#ifndef MARTENSITE_FRAGMENT
#if defined(MARTENSITE_WINDOW_DEPTH_INTERFACE) || defined(MARTENSITE_NATIVE_GRID12_V4)
layout(location=7) out vec4 PosLightSpace[MARTENSITE_VERTEX_SHADOW_MAPS];
#else
layout(location=15) out vec4 PosLightSpace[MARTENSITE_VERTEX_SHADOW_MAPS];
#endif
void writeShadowPositions(vec4 thePositionWorld) {
    for (uint anIter = 0u; anIter < uint(MARTENSITE_VERTEX_SHADOW_MAPS); ++anIter) {
        PosLightSpace[anIter] = float(anIter) < occShadowMaps.sizeBias.z
            ? lightSpacePosition(anIter, thePositionWorld) : vec4(0.0);
    }
}
// Gouraud (per-vertex) lighting has no shadow maps, as natively.
float lightShadow(uint theId, RenderLight theLight, vec3 theNormal, vec4 thePositionWorld) { return 1.0; }
#else
#ifndef MARTENSITE_MESH_EDGES
layout(location=15) in vec4 PosLightSpace[MARTENSITE_VERTEX_SHADOW_MAPS];
#endif
layout(set=6,binding=0) uniform sampler2DArray occShadowMapSamplers;
const vec2 occPoissonDisk16[16] = vec2[](
 vec2(-0.94201624,-0.39906216), vec2( 0.94558609,-0.76890725), vec2(-0.09418410,-0.92938870), vec2( 0.34495938, 0.29387760),
 vec2(-0.91588581, 0.45771432), vec2(-0.81544232,-0.87912464), vec2(-0.38277543, 0.27676845), vec2( 0.97484398, 0.75648379),
 vec2( 0.44323325,-0.97511554), vec2( 0.53742981,-0.47373420), vec2(-0.26496911,-0.41893023), vec2( 0.79197514, 0.19090188),
 vec2(-0.24188840, 0.99706507), vec2(-0.81409955, 0.91437590), vec2( 0.19984126, 0.78641367), vec2( 0.14383161,-0.14100790)
);
float occLightShadow(uint theId, vec3 theNormal, vec4 thePositionWorld, vec3 theLightDir) {
#ifdef MARTENSITE_MESH_EDGES
    vec4 aPosLightSpace = lightSpacePosition(theId, thePositionWorld);
#else
    vec4 aPosLightSpace = theId < uint(MARTENSITE_VERTEX_SHADOW_MAPS)
        ? PosLightSpace[theId] : lightSpacePosition(theId, thePositionWorld);
#endif
    vec3 aProjCoords = (aPosLightSpace.xyz / aPosLightSpace.w);
    aProjCoords = aProjCoords * 0.5 + vec3(0.5);
    float aCurrentDepth = aProjCoords.z;
    if (aProjCoords.x < 0.0 || aProjCoords.x > 1.0
     || aProjCoords.y < 0.0 || aProjCoords.y > 1.0
     || aCurrentDepth > 1.0) {
        return 1.0;
    }
    vec2 aTexelSize = vec2(occShadowMaps.sizeBias.x);
    float aBias = max(occShadowMaps.sizeBias.y * (1.0 - dot(theNormal, theLightDir)), occShadowMaps.sizeBias.y * 0.1);
    float aShadow = 0.0;
    for (int aPosIter = 0; aPosIter < 16; ++aPosIter) {
        float aClosestDepth = texture(occShadowMapSamplers,
            vec3(aProjCoords.xy + occPoissonDisk16[aPosIter] * aTexelSize, float(theId))).r;
        aShadow += (aCurrentDepth - aBias) > aClosestDepth ? 1.0 : 0.0;
    }
    return 1.0 - aShadow / 16.0;
}
// The shadow term of source `theId`: directional (kind 1) and spot (kind 3) lights casting
// shadows (kind.y) with a map; occLight_Position is the direction of a directional light and
// the position of a spot light, as natively.
float lightShadow(uint theId, RenderLight theLight, vec3 theNormal, vec4 thePositionWorld) {
    if (theLight.kind.y == 0u || (theLight.kind.x != 1u && theLight.kind.x != 3u)
        || float(theId) >= occShadowMaps.sizeBias.z) {
        return 1.0;
    }
    return occLightShadow(theId, theNormal, thePositionWorld, theLight.position.xyz);
}
#endif
