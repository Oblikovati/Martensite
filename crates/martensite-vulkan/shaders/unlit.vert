#version 450
#extension GL_GOOGLE_include_directive : require
// Graphic3d cameras emit OpenGL clip depth. Without VK_EXT_depth_clip_control the stage remaps
// it to Vulkan's zero-to-one range; with it (negativeOneToOne) the viewport transform applies
// OpenGL's mapping, as the native view does (backend/depth_equivalence.rs).
layout(constant_id = 7) const bool kRemapClipDepth = true;
#include "clip_position.glsl"
#ifdef MARTENSITE_RASTER_WORLD_CLIPPING_V2
out gl_PerVertex {
    vec4 gl_Position;
    float gl_PointSize;
    float gl_ClipDistance[8];
};
#endif
#include "clipping.glsl"
#ifdef MARTENSITE_FLOAT64_WINDOW_DEPTH_V5
layout(location=11) out vec4 windowDepthRawGLClip;
#endif
layout(location=0) in vec4 position;
layout(location=1) in vec4 color;
layout(location=0) out vec4 vertexColor;
layout(push_constant) uniform Transform {
    mat4 clip;
#ifdef MARTENSITE_FACTORIZED_TRIANGLES_V2
    mat4 modelFromObject;
#endif
} transform;
void main() {
    // Polygon-point surfaces use the native default size; triangle fills ignore this output.
    gl_PointSize = 1.0;
#ifdef MARTENSITE_CLIPPING
    clippingWorldPosition = clipping.worldFromObject * position;
#if defined(MARTENSITE_RASTER_WORLD_CLIPPING_V2) && !defined(MARTENSITE_RASTER_CLIP_SPACE_V5)
    rasterWorldClipDistances(clippingWorldPosition);
#endif
#endif

#ifdef MARTENSITE_FACTORIZED_TRIANGLES_V2
    vec4 world = clipPosition(transform.modelFromObject, position);
    gl_Position = clipPosition(transform.clip, world);
#else
    gl_Position = clipPosition(transform.clip, position);
#endif
#ifdef MARTENSITE_FLOAT64_WINDOW_DEPTH_V5
    windowDepthRawGLClip = gl_Position;
#endif
#ifdef MARTENSITE_RASTER_CLIP_SPACE_V5
    rasterWorldClipDistances(gl_Position);
#endif
    if (kRemapClipDepth) gl_Position.z = 0.5 * (gl_Position.z + gl_Position.w);
    vertexColor = color;
}
