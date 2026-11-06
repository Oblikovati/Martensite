#version 450
#extension GL_GOOGLE_include_directive : require
#define MARTENSITE_FRAGMENT
#include "clipping.glsl"
layout(location=0) in vec4 vertexColor;
#if defined(MARTENSITE_OIT_WEIGHTED) || defined(MARTENSITE_OIT_PEELING)
#include "oit.glsl"
#else
#include "native_fragment_alpha.glsl"
layout(location=0) out vec4 outColor;
#endif
#ifdef MARTENSITE_WINDOW_DEPTH_FRAGMENT
#ifdef MARTENSITE_FLOAT64_WINDOW_DEPTH_V5
#include "window_depth_float64_fragment.glsl"
#else
#include "window_depth_fragment.glsl"
#endif
#endif
void main() {
#ifdef MARTENSITE_WINDOW_DEPTH_FRAGMENT
    writeReferenceWindowDepth();
#endif
#if defined(MARTENSITE_OIT_WEIGHTED) || defined(MARTENSITE_OIT_PEELING)
    if(oitEarlyReturn())return;
#endif
#ifdef MARTENSITE_CLIPPING
    discardClippedFragment();
#endif
#if defined(MARTENSITE_OIT_WEIGHTED) || defined(MARTENSITE_OIT_PEELING)
    oitWriteColor(vertexColor);
#else
 outColor = vertexColor;
    nativePostAlpha(outColor.a);
#endif
}
