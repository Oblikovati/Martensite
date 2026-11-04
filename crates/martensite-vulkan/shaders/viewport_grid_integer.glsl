#ifndef MARTENSITE_VIEWPORT_GRID_INTEGER_GLSL
#define MARTENSITE_VIEWPORT_GRID_INTEGER_GLSL
#extension GL_ARB_gpu_shader_int64 : require
#include "viewport_grid.glsl"
// Admission bounds scaled coordinates to 24 bits, so differences/products fit signed 64 bits.
bool viewportGridTriangleEmpty(vec2 window[3], uint fractionalBits) {
    float scale = float(1u << fractionalBits);
    i64vec2 first = i64vec2(roundEven(window[0] * scale));
    i64vec2 second = i64vec2(roundEven(window[1] * scale));
    i64vec2 third = i64vec2(roundEven(window[2] * scale));
    i64vec2 a = second - first;
    i64vec2 b = third - first;
    return a.x * b.y - b.x * a.y == int64_t(0);
}
#endif
