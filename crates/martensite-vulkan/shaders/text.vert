#version 450
#extension GL_GOOGLE_include_directive : require
// Graphic3d cameras emit OpenGL clip depth. Without VK_EXT_depth_clip_control the stage remaps
// it to Vulkan's zero-to-one range; with it (negativeOneToOne) the viewport transform applies
// OpenGL's mapping, as the native view does (backend/depth_equivalence.rs).
layout(constant_id = 7) const bool kRemapClipDepth = true;
#include "clip_position.glsl"
#include "clipping.glsl"
layout(location=0) in vec4 position;
layout(location=1) in vec4 color;
layout(location=2) in vec2 uv;
layout(location=0) out vec4 vertexColor;
layout(location=1) out vec2 textureUV;
layout(push_constant) uniform Transform { mat4 clip; uint width; uint height; float cutoff; } transform;
void main() {
#ifdef MARTENSITE_CLIPPING
    clippingWorldPosition = clipping.worldFromObject * position;
#endif

    gl_Position = clipPosition(transform.clip, position);
    if (kRemapClipDepth) gl_Position.z = 0.5 * (gl_Position.z + gl_Position.w);
    vertexColor = color;
    textureUV = uv;
}
