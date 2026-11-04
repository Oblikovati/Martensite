// Conservative fragments are filtered against exact snapped sample centers; no geometry is rewritten.
#include "triangle_grid_math_v2.glsl"

bool triangleGridTopLeftV2(i64vec2 a, i64vec2 b) {
    int64_t dx = b.x - a.x;
    int64_t dy = b.y - a.y;
    return phongViewport.coverageOrigin == 1u
        ? dy < 0 || (dy == 0 && dx > 0)
        : dy < 0 || (dy == 0 && dx < 0);
}

bool triangleGridAcceptEdgeV2(i64vec2 a, i64vec2 b, i64vec2 point) {
    int64_t value = triangleGridEdgeV2(a, b, point);
    return value > 0 || (value == 0 && triangleGridTopLeftV2(a, b));
}

int64_t triangleGridAreaV2() {
    i64vec2 a = triangleGridPointV2(phongClip[0]);
    i64vec2 b = triangleGridPointV2(phongClip[1]);
    i64vec2 c = triangleGridPointV2(phongClip[2]);
    return triangleGridEdgeV2(a, b, c);
}
#ifdef MARTENSITE_NATIVE_GRID12_V4
vec2 triangleGridHardwareBarycentricV4() {
    i64vec2 a = triangleGridPointV2(phongClip[0]);
    i64vec2 b = triangleGridPointV2(phongClip[1]);
    i64vec2 c = triangleGridPointV2(phongClip[2]);
    vec3 barycentric;
    i64vec2 samplePoint = triangleGridWindowPointV2(gl_FragCoord.xy);
    triangleGridBarycentricV2(a, b, c, samplePoint, barycentric);
    vec3 weights = barycentric / vec3(phongClip[0].w, phongClip[1].w, phongClip[2].w);
    float total = (weights.x + weights.y) + weights.z;
    return weights.yz / total;
}
vec3 triangleGridInterpolate3V4(vec3 a, vec3 b, vec3 c, vec2 grid) {
    return fma(vec3(grid.y), c - a, fma(vec3(grid.x), b - a, a));
}
vec4 triangleGridInterpolate4V4(vec4 a, vec4 b, vec4 c, vec2 grid) {
    return fma(vec4(grid.y), c - a, fma(vec4(grid.x), b - a, a));
}
bool triangleGridFrontFacingV4() {
    bool logicalCcw = triangleGridAreaV2() < 0;
    return logicalCcw == (phongViewport.coverageFrontFaceCcw != 0u);
}
bool triangleGridCulledV4() {
    bool front = triangleGridFrontFacingV4();
    if (phongViewport.coverageCullMode == 1u) return !front;
    return phongViewport.coverageCullMode == 2u && front;
}
#endif
bool triangleGridCoveredV2() {
    i64vec2 a = triangleGridPointV2(phongClip[0]);
    i64vec2 b = triangleGridPointV2(phongClip[1]);
    i64vec2 c = triangleGridPointV2(phongClip[2]);
    int64_t area = triangleGridEdgeV2(a, b, c);
    if (area == 0) return false;
    if (area < 0) {
        i64vec2 swapPoint = b;
        b = c;
        c = swapPoint;
    }
    i64vec2 samplePoint = i64vec2(
        triangleGridCoordinateV2(gl_FragCoord.x),
        triangleGridCoordinateV2(gl_FragCoord.y));
    return triangleGridAcceptEdgeV2(a, b, samplePoint)
        && triangleGridAcceptEdgeV2(b, c, samplePoint)
        && triangleGridAcceptEdgeV2(c, a, samplePoint);
}

#ifdef MARTENSITE_NATIVE_GRID12_V4
#include "window_depth_plane.glsl"
vec2 triangleGridDepthGradientV4(i64vec2 grid[3], float depth[3]) {
    i64vec2 first = grid[1] - grid[0];
    i64vec2 second = grid[2] - grid[0];
    int64_t area = first.x * second.y - second.x * first.y;
    if (area == 0) discard;
    precise vec2 firstFloat = vec2(first);
    precise vec2 secondFloat = vec2(second);
    precise vec2 numerator = vec2(
        (depth[1] - depth[0]) * secondFloat.y - (depth[2] - depth[0]) * firstFloat.y,
        firstFloat.x * (depth[2] - depth[0]) - secondFloat.x * (depth[1] - depth[0]));
    precise vec2 gradient = numerator / float(area) * 4096.0;
    return gradient;
}

float triangleGridClipDepthV4(vec4 clip) {
    precise float depth = clip.z / clip.w;
    return phongViewport.depthNegativeOneToOne != 0u ? depth * 0.5 + 0.5 : depth;
}

float triangleGridLogicalDepthV4() {
    i64vec2 grid[3] = i64vec2[3](
        triangleGridPointV2(phongClip[0]),
        triangleGridPointV2(phongClip[1]),
        triangleGridPointV2(phongClip[2]));
    float depth[3] = float[3](
        triangleGridClipDepthV4(phongClip[0]),
        triangleGridClipDepthV4(phongClip[1]),
        triangleGridClipDepthV4(phongClip[2]));
    WindowDepthPlane plane = WindowDepthPlane(vec3(0.0, 0.0, depth[0]),
                                               triangleGridDepthGradientV4(grid, depth));
    float depthSpan = phongViewport.depthMaximum - phongViewport.depthMinimum;
    plane.origin.z = phongViewport.depthMinimum + plane.origin.z * depthSpan;
    plane.gradient *= depthSpan;
    i64vec2 sampleGrid = i64vec2(
        triangleGridCoordinateV2(gl_FragCoord.x),
        triangleGridCoordinateV2(gl_FragCoord.y));
    vec2 relativeSample = vec2(sampleGrid - grid[0]) / 4096.0;
    float result = windowDepthBiasedSample(plane, relativeSample,
        phongViewport.depthBiasFactor, phongViewport.depthBiasUnits,
        phongViewport.depthBiasUnitScale);
    if (isnan(result) || isinf(result)) discard;
    // V4 defines saturation at the viewport range; without depth clamp Vulkan leaves
    // out-of-range fragment depth undefined, so this makes the owned behavior deterministic.
    return clamp(result, phongViewport.depthMinimum, phongViewport.depthMaximum);
}
#endif
