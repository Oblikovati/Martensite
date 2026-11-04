#version 450
#extension GL_GOOGLE_include_directive : require
#define MARTENSITE_FRAGMENT
#include "clipping.glsl"
#include "native_fragment_alpha.glsl"
#include "logical_visual_coordinates.glsl"
layout(location=0) in vec4 vertexColor;
layout(location=1) in vec2 screenSpaceCoord;
layout(location=0) out vec4 fragmentColor;
layout(push_constant) uniform LineTransform { mat4 clip; vec2 viewport; uint pattern; uint factor; } transform;
void main() {
#ifdef MARTENSITE_CLIPPING
    discardClippedFragment();
#endif

#ifndef MARTENSITE_FIXED_LINE_STIPPLE
    // the reference implementation's Graphic3dShaderManager uses the dominant screen derivative,
    // not a stipple counter restarted at each segment. Raster Y now stays native.
#ifdef MARTENSITE_TOP_LEFT_LOGICAL_COORDINATES
    float coordinate = logicalVisualWindowY(gl_FragCoord.y, transform.viewport.y);
#else
    float coordinate = gl_FragCoord.y;
#endif
    uint axis = transform.factor >> 16;
    if (axis == 1u || (axis == 0u && abs(dFdx(screenSpaceCoord.x)) - abs(dFdy(screenSpaceCoord.y)) > 0.001))
        coordinate = gl_FragCoord.x;
    uint bit = uint(floor(coordinate / float(transform.factor & 65535u) + 0.5)) & 15u;
    if ((transform.pattern & (1u << bit)) == 0u) discard;
#endif
    fragmentColor = vertexColor;
    nativePostAlpha(fragmentColor.a);
}
