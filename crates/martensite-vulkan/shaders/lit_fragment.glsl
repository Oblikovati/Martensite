// The flat normal of the facet, captured by captureFacetNormal() at the start of main: Vulkan
// `discard` and an early `return` (clipping, OIT peeling) leave the quad's derivatives undefined
// on hardware (RADV), while native GL derives them from the whole quad.
vec3 facetNormal;
#ifndef MARTENSITE_CLIPPED_FAN_DEPTH_V4
void captureFacetNormal() {
    vec3 point = worldPosition.xyz / worldPosition.w;
    // Positive raster Y preserves native derivatives before canonical output row reversal.
#ifdef MARTENSITE_TOP_LEFT_LOGICAL_COORDINATES
    facetNormal = glslNormalize(cross(dFdx(point), -dFdy(point)));
#else
    facetNormal = glslNormalize(cross(dFdx(point), dFdy(point)));
#endif
}
#endif
vec4 litFragmentColor() {
    // the reference implementation line/marker shaders always evaluate the front material.
#if defined(MARTENSITE_LINE_LIGHTING) || defined(MARTENSITE_MARKER_LIGHTING)
    bool frontFacing = true;
#elif defined(MARTENSITE_NATIVE_GRID12_V4)
    bool frontFacing = triangleGridFrontFacingV4();
#else
    bool frontFacing = gl_FrontFacing;
#endif
#ifdef MARTENSITE_FIXED_LIGHTING
    vec4 result=lighting.settings.x==4u?(frontFacing?fixedFrontColor:fixedBackColor)
        :(frontFacing?frontColor:backColor);
    bool colored=lighting.settings.z!=0u&&(frontFacing||(lighting.settings.w&1u)==0u);
    if(!colored)result.a=lighting.materials[frontFacing?0u:4u].a;
    return result;
#else
    if (lighting.settings.x == 1u) {
        return frontFacing ? frontColor : backColor;
    }
#ifdef MARTENSITE_MATERIAL_MAPS
    vec3 normal = glslNormalize(mappedSurfaceNormal(worldNormal));
#else
    vec3 normal = glslNormalize(worldNormal);
#endif
    if (lighting.settings.x == 0u) {
#ifdef MARTENSITE_NATIVE_GRID12_V4
        normal = frontFacing ? facetNormal : -facetNormal;
#else
        normal = gl_FrontFacing ? facetNormal : -facetNormal;
#endif
        // The native flat program normalizes its derived normal twice: once when it derives it,
        // and again in getColor(), like every other normal.
        normal = glslNormalize(normal);
    }
#ifdef MARTENSITE_UNSNAPPED_PHONG_V2
    if (lighting.settings.x == 2u) {
#ifdef MARTENSITE_NATIVE_GRID12_V4
        // The native rasterizer's barycentrics come from the same 12-bit vertices as its coverage,
        // and its parameter interpolation is P0 + i * (P1 - P0) + j * (P2 - P0), each step fused.
        vec2 grid = triangleGridHardwareBarycentricV4();
        normal = glslNormalize(triangleGridInterpolate3V4(phongNormalAt(0), phongNormalAt(1), phongNormalAt(2), grid));
        vec3 view = glslNormalize(triangleGridInterpolate3V4(phongViewAt(0), phongViewAt(1), phongViewAt(2), grid));
        vec4 point = triangleGridInterpolate4V4(phongPosition[0], phongPosition[1], phongPosition[2], grid);
        vec4 color = triangleGridInterpolate4V4(phongColor[0], phongColor[1], phongColor[2], grid);
        return computeLighting(normal, view, point, frontFacing, color);
#else
        vec3 weights = phongProjectiveWeights();
        normal = glslNormalize(phongInterpolate3(phongNormalAt(0), phongNormalAt(1), phongNormalAt(2), weights));
        vec3 view = glslNormalize(phongInterpolate3(phongViewAt(0), phongViewAt(1), phongViewAt(2), weights));
        vec4 point = phongInterpolate4(phongPosition[0], phongPosition[1], phongPosition[2], weights);
        vec4 color = phongInterpolate4(phongColor[0], phongColor[1], phongColor[2], weights);
        return computeLighting(normal, view, point, frontFacing, color);
#endif
    }
#endif
    return computeLighting(normal, glslNormalize(viewDirection), worldPosition, frontFacing, vertexColor);
#endif
}

