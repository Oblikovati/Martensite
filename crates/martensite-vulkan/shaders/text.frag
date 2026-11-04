#version 450
#extension GL_GOOGLE_include_directive : require
#define MARTENSITE_FRAGMENT
#include "clipping.glsl"
#include "native_fragment_alpha.glsl"
layout(location=0) in vec4 vertexColor;
layout(location=1) in vec2 textureUV;
layout(location=0) out vec4 outColor;
// GlFont binds an 8-bit alpha texture with bilinear filtering; the device sampler
// weights texels as the native texture unit does.
layout(set=0,binding=0) uniform sampler2D glyphs;
layout(push_constant) uniform Transform { mat4 clip; uint width; uint height; float cutoff; } transform;
void main() {
#ifdef MARTENSITE_CLIPPING
    discardClippedFragment();
#endif

    outColor = vertexColor * texture(glyphs, textureUV);
    if (outColor.a <= transform.cutoff) discard;
    nativePostAlpha(outColor.a);
}
