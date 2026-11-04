#include "triangle_grid_math_v2.glsl"

bool worldClipGrid12PositionV1(out vec4 position) {
    i64vec2 a = triangleGridPointV2(phongClip[0]);
    i64vec2 b = triangleGridPointV2(phongClip[1]);
    i64vec2 c = triangleGridPointV2(phongClip[2]);
    i64vec2 samplePoint = triangleGridWindowPointV2(gl_FragCoord.xy);
    vec3 barycentric;
    if (!triangleGridBarycentricV2(a, b, c, samplePoint, barycentric)) return false;
    vec3 clipW = vec3(phongClip[0].w, phongClip[1].w, phongClip[2].w);
    if (any(isnan(clipW)) || any(isinf(clipW)) || any(lessThan(clipW, vec3(4.7019774e-38)))
        || any(greaterThan(clipW, vec3(5.3169119e36)))) return false;
    vec3 reciprocalW = vec3(1.0) / clipW;
    if (any(isnan(reciprocalW)) || any(isinf(reciprocalW))
        || any(lessThan(reciprocalW, vec3(1.88079096e-37)))) return false;
    float total = ((barycentric.x * reciprocalW.x + barycentric.y * reciprocalW.y)
        + barycentric.z * reciprocalW.z);
    if (isnan(total) || isinf(total) || total < 4.7019774e-38) return false;
#ifdef MARTENSITE_WORLD_CLIP_GRID12_V4_COMPACT
    vec4 clipWorld0 = clippingWorldPosition0;
    vec4 clipWorld1 = clippingWorldPosition1;
    vec4 clipWorld2 = vec4(clippingWorldPosition2X, clippingWorldPosition2Y,
        clippingWorldPosition2Z, clippingWorldPosition2W);
    vec4 aWorld = clipWorld0 / clipW.x;
    vec4 bWorld = clipWorld1 / clipW.y;
    vec4 cWorld = clipWorld2 / clipW.z;
#else
    vec4 aWorld = clippingWorldPositionGrid[0] / clipW.x;
    vec4 bWorld = clippingWorldPositionGrid[1] / clipW.y;
    vec4 cWorld = clippingWorldPositionGrid[2] / clipW.z;
#endif
    position = ((barycentric.x * aWorld + barycentric.y * bWorld)
        + barycentric.z * cWorld) / total;
    return !any(isnan(position)) && !any(isinf(position)) && position.w > 0.0;
}
