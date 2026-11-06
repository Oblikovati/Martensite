#version 450
#extension GL_GOOGLE_include_directive : require
layout(constant_id = 7) const bool kRemapClipDepth = true;
#include "clip_position.glsl"
#include "clipping.glsl"
layout(location=0) in vec4 position;
layout(location=1) in vec4 color;
layout(location=2) in vec2 uv;
layout(location=0) out vec4 vertexColor;
layout(location=1) out vec2 textureUV;
#ifdef MARTENSITE_TEXT_FLAT_CORNERS_V4
layout(location=3) in vec4 textCornerPosition[3];
layout(location=6) in vec2 textCornerUv[3];
layout(location=4) flat out vec4 textGridClip[3];
layout(location=7) flat out vec2 textGridUv[3];
#else
layout(location=4) out vec4 textGridClip;
layout(location=5) out vec2 textGridUv;
#endif
layout(push_constant) uniform Transform { mat4 clip; uint width; uint height; float cutoff; } transform;
void main() {
#ifdef MARTENSITE_CLIPPING
    clippingWorldPosition = clipping.worldFromObject * position;
#endif
    gl_Position = clipPosition(transform.clip, position);
    if (kRemapClipDepth) gl_Position.z = 0.5 * (gl_Position.z + gl_Position.w);
    vertexColor = color;
    textureUV = uv;
#ifdef MARTENSITE_TEXT_FLAT_CORNERS_V4
    // Identical corner payloads make the transport independent of the provoking vertex.
    for (int corner = 0; corner < 3; ++corner) {
        textGridClip[corner] = clipPosition(transform.clip, textCornerPosition[corner]);
        if (kRemapClipDepth) textGridClip[corner].z = 0.5 * (textGridClip[corner].z + textGridClip[corner].w);
        textGridUv[corner] = textCornerUv[corner];
    }
#else
    textGridClip = gl_Position;
    textGridUv = uv;
#endif
}
