#version 450
#extension GL_GOOGLE_include_directive : require
layout(constant_id = 7) const bool kRemapClipDepth = true;
#include "clip_position.glsl"
layout(location=0) in vec4 position;
layout(location=1) in vec4 color;
layout(location=2) in vec2 uv;
layout(location=0) out vec4 vertexColor;
layout(location=1, xfb_buffer=0, xfb_offset=16, xfb_stride=28) out vec2 textureUV;
layout(xfb_buffer=0, xfb_stride=28) out gl_PerVertex {
    layout(xfb_offset=0) vec4 gl_Position;
    float gl_PointSize;
    float gl_ClipDistance[];
    float gl_CullDistance[];
};
layout(location=11, xfb_buffer=0, xfb_offset=24, xfb_stride=28) flat out uint capturedVertexIndex;
layout(push_constant) uniform Transform { mat4 clip; uint width; uint height; float cutoff; } transform;
void main() {
    gl_Position = clipPosition(transform.clip, position);
    if (kRemapClipDepth) gl_Position.z = 0.5 * (gl_Position.z + gl_Position.w);
    vertexColor = color;
    textureUV = uv;
    capturedVertexIndex = uint(gl_VertexIndex);
}
