#version 450
#extension GL_GOOGLE_include_directive : require
// Graphic3d cameras emit OpenGL clip depth. Without VK_EXT_depth_clip_control the stage remaps
// it to Vulkan's zero-to-one range; with it (negativeOneToOne) the viewport transform applies
// OpenGL's mapping, as the native view does (backend/depth_equivalence.rs).
layout(constant_id = 7) const bool kRemapClipDepth = true;
#include "clip_position.glsl"
#include "line_viewport_clip.glsl"
#ifdef MARTENSITE_RASTER_WORLD_CLIPPING_V3
out gl_PerVertex {
    vec4 gl_Position;
    float gl_PointSize;
    float gl_ClipDistance[8];
};
#endif
#include "clipping.glsl"
layout(location=0) in vec4 position;
layout(location=1) in vec4 color;
layout(location=0) out vec4 vertexColor;
layout(location=1) out vec2 screenSpaceCoord;
layout(push_constant) uniform LineTransform { mat4 clip; vec2 viewport; uint pattern; uint factor; } transform;
void main() {
#ifdef MARTENSITE_CLIPPING
    clippingWorldPosition = clipping.worldFromObject * position;
#ifdef MARTENSITE_RASTER_WORLD_CLIPPING_V3
    rasterWorldClipDistances(clippingWorldPosition);
#endif
#endif

    gl_Position = clipPosition(transform.clip, position);
    screenSpaceCoord = (gl_Position.xy / gl_Position.w * 0.5 + 0.5) * transform.viewport;
#ifdef MARTENSITE_NATIVE_LINE_XY_CLIP
    nativeLineViewportClip(gl_Position);
#endif
    if (kRemapClipDepth) gl_Position.z = 0.5 * (gl_Position.z + gl_Position.w);
    vertexColor = color;
}
