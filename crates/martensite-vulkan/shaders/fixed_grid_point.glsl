#ifdef MARTENSITE_FIXED_GRID_POINT
layout(constant_id = 8) const bool kFixedGridDepthClamp = false;
#if defined(MARTENSITE_FIXED_GRID_UNIT12) || defined(MARTENSITE_FIXED_GRID_SPRITE12)
int fixedGridUnit12FloorDivide(int value) {
    // SMod follows the positive divisor, so its negative-remainder correction never fires.
    // Arithmetic shift by twelve is floor division by this power-of-two lattice.
    return value >> 12;
}
#endif
vec4 fixedGridPointCoverage(vec4 center, float requestedSize) {
    bool invalid = any(isnan(center)) || any(isinf(center)) || center.w <= 0.0;
    invalid = invalid || any(greaterThan(abs(center.xy), vec2(center.w)));
    if (!kFixedGridDepthClamp) {
        invalid = invalid || center.z > center.w || center.z < (kRemapClipDepth ? 0.0 : -center.w);
    }
    if (invalid) return vec4(2.0, 2.0, 2.0, 1.0);
    vec2 halfViewport = transform.pointViewport.zw * 0.5;
    vec2 window = (center.xy / center.w) * halfViewport + (transform.pointViewport.xy + halfViewport);
    float reflection = 2.0 * transform.pointViewport.y + transform.pointViewport.w;
    if (transform.pointViewport.w < 0.0) window.y = reflection - window.y;
#ifdef MARTENSITE_FIXED_GRID_UNIT12
    ivec2 snapped = ivec2(roundEven(window * 4096.0));
    snapped.y -= 1;
    ivec2 minimumPixels = ivec2(
        fixedGridUnit12FloorDivide(snapped.x),
        fixedGridUnit12FloorDivide(snapped.y)
    );
    vec2 minimum = vec2(minimumPixels);
    float width = 1.0;
#elif defined(MARTENSITE_FIXED_GRID_SPRITE12)
    ivec2 snapped = ivec2(roundEven(window * 4096.0));
    int fixedWidth = max(256, (int(roundEven(requestedSize * 256.0)) + 127) & ~255);
    float width = float(fixedWidth / 256);
    int evenAdjustment = (fixedWidth / 256) % 2 == 0 ? 2048 : 0;
    ivec2 minimumPixels = ivec2(
        fixedGridUnit12FloorDivide(snapped.x + evenAdjustment),
        fixedGridUnit12FloorDivide(snapped.y + evenAdjustment)
    ) - ivec2(fixedWidth / 512);
    vec2 minimum = vec2(minimumPixels);
#else
    vec2 snapped = roundEven(window * 256.0);
    int fixedWidth = max(256, (int(roundEven(requestedSize * 256.0)) + 127) & ~255);
    float width = float(fixedWidth / 256);
    float evenAdjustment = mod(width, 2.0) == 0.0 ? 128.0 : 0.0;
    vec2 minimum = floor((snapped + evenAdjustment) / 256.0) - floor(width * 0.5);
#endif
    const vec2 corners[4] = vec2[4](vec2(0,0), vec2(1,0), vec2(0,1), vec2(1,1));
    vec2 corner = minimum + corners[gl_VertexIndex] * width;
    if (transform.pointViewport.w < 0.0) corner.y = reflection - corner.y;
    vec2 ndc = (corner - transform.pointRasterViewport.xy) / transform.pointRasterViewport.zw * 2.0 - 1.0;
    return vec4(ndc * center.w, center.z, center.w);
}
#endif
