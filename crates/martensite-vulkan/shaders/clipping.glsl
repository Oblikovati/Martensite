// the reference implementation's Graphic3dShaderManager: union within each chain, intersection between chains.
#include "logical_visual_coordinates.glsl"
#include "native_dot.glsl"
#ifdef MARTENSITE_CLIPPING
struct ClipHalfspace { vec4 equation; uvec4 chain; };
layout(set=1,binding=0,std430) readonly buffer ClipVolume {
    mat4 worldFromObject;
    uvec4 count;
    uvec4 hatchRows[8];
    ClipHalfspace planes[];
} clipping;
#ifdef MARTENSITE_FRAGMENT
#ifdef MARTENSITE_WORLD_CLIP_GRID12_V1
#ifdef MARTENSITE_WORLD_CLIP_GRID12_V4
#ifdef MARTENSITE_WORLD_CLIP_GRID12_V4_COMPACT
layout(location=13) flat in vec4 clippingWorldPosition0;
layout(location=14) flat in vec4 clippingWorldPosition1;
layout(location=10, component=3) flat in float clippingWorldPosition2X;
layout(location=11, component=3) flat in float clippingWorldPosition2Y;
layout(location=12, component=3) flat in float clippingWorldPosition2Z;
layout(location=22, component=3) flat in float clippingWorldPosition2W;
#else
layout(location=28) flat in vec4 clippingWorldPositionGrid[3];
#endif
#else
layout(location=3) pervertexEXT in vec4 clippingWorldPositionGrid[];
#endif
bool worldClipGrid12PositionV1(out vec4 position);
#else
layout(location=3) in vec4 clippingWorldPosition;
#endif
void discardClippedFragment() {
    if (clipping.count.y != 0u) {
        uint x = uint(gl_FragCoord.x) & 31u;
        // Positive viewport attachments are read back bottom-up; this is already native window Y.
#ifdef MARTENSITE_TOP_LEFT_LOGICAL_COORDINATES
        uint y = uint(logicalVisualWindowY(gl_FragCoord.y, float(clipping.count.z))) & 31u;
#else
        uint y = uint(gl_FragCoord.y) & 31u;
#endif
        uint row = clipping.hatchRows[y / 4u][y % 4u];
        if ((row & (1u << x)) == 0u) discard;
    }
#ifdef MARTENSITE_WORLD_CLIP_GRID12_V1
    vec4 worldPosition;
    if (!worldClipGrid12PositionV1(worldPosition)) discard;
    vec3 point = worldPosition.xyz / worldPosition.w;
#elif defined(MARTENSITE_UNSNAPPED_PHONG_V2)
    // These published projected-grid modules retain their original local-copy emission.
    vec4 worldPosition = clippingWorldPosition;
    vec3 point = worldPosition.xyz / worldPosition.w;
#else
    // Preserve the pre-grid bytecode's direct input load for legacy replay (bc9d4e769).
    vec3 point = clippingWorldPosition.xyz / clippingWorldPosition.w;
#endif
    for (uint index = 0u; index < clipping.count.x;) {
        vec4 equation = clipping.planes[index].equation;
        uint remaining = clipping.planes[index].chain.x;
        if (nativeDot3(equation.xyz, point) + equation.w < 0.0) {
            if (remaining == 1u) discard;
            index += 1u;
        } else {
            index += remaining;
        }
    }
}
#else
layout(location=3) out vec4 clippingWorldPosition;
#if defined(MARTENSITE_RASTER_WORLD_CLIPPING_V2) || defined(MARTENSITE_RASTER_WORLD_CLIPPING_V3)
void rasterWorldClipDistances(vec4 objectPosition) {
    for (uint index = 0u; index < 8u; ++index) {
        float distance = 1.0;
        if (index < clipping.count.w) {
            vec4 equation = clipping.planes[index].equation;
#ifdef MARTENSITE_RASTER_CLIP_SPACE_V5
            // draw_pipe_clip.c dot4 over the clip position, in its summation order.
            distance = ((equation.x * objectPosition.x + equation.y * objectPosition.y)
                + equation.z * objectPosition.z) + equation.w * objectPosition.w;
#else
            distance = dot(equation.xyz, objectPosition.xyz) + equation.w * objectPosition.w;
#endif
        }
        gl_ClipDistance[index] = distance;
    }
}
#endif
#endif
#endif
