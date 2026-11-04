#ifndef MARTENSITE_TRIANGLE_GRID_MATH_V2_GLSL
#define MARTENSITE_TRIANGLE_GRID_MATH_V2_GLSL
// Coverage and explicit clipping interpolation share this exact nearest-even coordinate grid.
int64_t triangleGridCoordinateV2(float coordinate) {
    return int64_t(roundEven(coordinate * 4096.0));
}

i64vec2 triangleGridWindowPointV2(vec2 window) {
    return i64vec2(triangleGridCoordinateV2(window.x), triangleGridCoordinateV2(window.y));
}

i64vec2 triangleGridPointV2(vec4 clip) {
#if defined(MARTENSITE_WORLD_CLIP_GRID12_V1) || defined(MARTENSITE_UNSNAPPED_PHONG_V3)
    return triangleGridWindowPointV2(phongWindowPoint(clip));
#else
    vec2 window = phongWindowPoint(clip);
    return i64vec2(triangleGridCoordinateV2(window.x), triangleGridCoordinateV2(window.y));
#endif
}

int64_t triangleGridEdgeV2(i64vec2 a, i64vec2 b, i64vec2 point) {
    return (b.x - a.x) * (point.y - a.y) - (b.y - a.y) * (point.x - a.x);
}

bool triangleGridBarycentricV2(
    i64vec2 a, i64vec2 b, i64vec2 c, i64vec2 samplePoint, out vec3 barycentric
) {
    int64_t area = triangleGridEdgeV2(a, b, c);
    if (area == int64_t(0)) return false;
    vec3 edges = vec3(
        float(triangleGridEdgeV2(b, c, samplePoint)),
        float(triangleGridEdgeV2(c, a, samplePoint)),
        float(triangleGridEdgeV2(a, b, samplePoint)));
    barycentric = edges / float(area);
    return !any(isnan(barycentric)) && !any(isinf(barycentric))
        && !any(lessThan(barycentric, vec3(0.0)));
}
#endif
