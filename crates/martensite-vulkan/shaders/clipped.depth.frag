#version 450
#extension GL_GOOGLE_include_directive : require
#extension GL_ARB_gpu_shader_int64 : require
#define MARTENSITE_FRAGMENT
#define MARTENSITE_FIXED_LIGHTING 1
#define MARTENSITE_CLIPPED_FAN_DEPTH_V4 1
#include "lighting.glsl"
#include "native_fragment_alpha.glsl"
layout(location=0) in vec4 vertexColor;
layout(location=1) in vec4 clippingWorldPosition;
layout(location=2) in vec4 centerFrontColor;
layout(location=3) in vec4 centerBackColor;
#define frontColor interpolateAtOffset(centerFrontColor,vec2(0.0))
#define backColor interpolateAtOffset(centerBackColor,vec2(0.0))
#define fixedFrontColor frontColor
#define fixedBackColor backColor
#include "lit_fragment.glsl"
#include "clipped_depth_sample.glsl"
layout(location=0) out vec4 fragmentColor;
void main() {
    if(lighting.settings.x!=3u) {clippedDepthFault(256u);discard;}
    if(!clippedDepthWriteSample()) discard;
    fragmentColor=litFragmentColor();
    nativePostAlpha(fragmentColor.a);
}
