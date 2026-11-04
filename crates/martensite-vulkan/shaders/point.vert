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
layout(location=0) out vec4 vertexColor;
#ifdef MARTENSITE_FIXED_GRID_SPRITE12
layout(location=10) noperspective out vec2 fixedGridSpriteCoordinate;
#endif
layout(push_constant) uniform Transform { mat4 clip; float size; uint width; uint height; uint policy;
#ifdef MARTENSITE_FIXED_GRID_POINT
    layout(offset=96) vec4 pointViewport;
    vec4 pointRasterViewport;
#endif
} transform;
#include "fixed_grid_point.glsl"
#ifdef MARTENSITE_VERTEX_OUTPUT_CAPTURE
layout(xfb_buffer=0, xfb_stride=20) out gl_PerVertex {
    layout(xfb_offset=0) vec4 gl_Position;
    float gl_PointSize;
    float gl_ClipDistance[];
    float gl_CullDistance[];
};
layout(location=11, xfb_buffer=0, xfb_offset=16, xfb_stride=20) flat out uint capturedVertexIndex;
#endif
void main() {
#ifdef MARTENSITE_VERTEX_OUTPUT_CAPTURE
    capturedVertexIndex = uint(gl_VertexIndex);
#endif
#ifdef MARTENSITE_CLIPPING
    clippingWorldPosition = clipping.worldFromObject * position;
#endif

    gl_Position = clipPosition(transform.clip, position);
    if (kRemapClipDepth) gl_Position.z = 0.5 * (gl_Position.z + gl_Position.w);
    gl_PointSize = transform.size;
    vertexColor = color;
#ifdef MARTENSITE_FIXED_GRID_POINT
    gl_Position = fixedGridPointCoverage(gl_Position, transform.size);
#ifdef MARTENSITE_FIXED_GRID_SPRITE12
    const vec2 spriteCorners[4] = vec2[4](vec2(0,0), vec2(1,0), vec2(0,1), vec2(1,1));
    fixedGridSpriteCoordinate = spriteCorners[gl_VertexIndex];
#endif
#endif
}
