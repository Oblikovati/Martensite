#ifndef MARTENSITE_VIEWPORT_GRID_GLSL
#define MARTENSITE_VIEWPORT_GRID_GLSL
// Callers admit finite coordinates, nonoverflowing scaling and explicit 8/10/12 precision.
// The grid is an owned contract, never an inferred device-minimum property.
vec2 viewportGridNearestEven(vec2 coordinate, uint fractionalBits) {
    float scale = float(1u << fractionalBits);
    return roundEven(coordinate * scale) / scale;
}
#endif
