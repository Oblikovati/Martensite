#version 450
#extension GL_GOOGLE_include_directive : require
// Graphic3d cameras emit OpenGL clip depth. Without VK_EXT_depth_clip_control the stage remaps
// it to Vulkan's zero-to-one range; with it (negativeOneToOne) the viewport transform applies
// OpenGL's mapping, as the native view does (backend/depth_equivalence.rs).
layout(constant_id = 7) const bool kRemapClipDepth = true;
#include "clip_position.glsl"
#include "line_viewport_clip.glsl"
#ifdef MARTENSITE_RASTER_WORLD_CLIPPING_V2
out gl_PerVertex {
    vec4 gl_Position;
    float gl_PointSize;
    float gl_ClipDistance[8];
};
#endif
#include "lighting.glsl"
#include "clipping.glsl"
#include "vertex_output_xfb.glsl"
#ifdef MARTENSITE_FLOAT64_WINDOW_DEPTH_V5
layout(location=11) out vec4 windowDepthRawGLClip;
#endif
layout(location=0) in vec4 position;
layout(location=1) in vec4 color;
layout(location=2) in vec3 normal;
#ifdef MARTENSITE_FIXED_LIGHTING
layout(location=4) in vec4 fixedPosition;
layout(location=5) in vec3 fixedNormal;
layout(location=6) in vec4 fixedColor;
#endif
layout(push_constant) uniform Transform {
    mat4 clipFromObject;
#ifdef MARTENSITE_FACTORIZED_TRIANGLES_V2
    mat4 modelFromObject;
#endif
#ifdef MARTENSITE_LINE_LIGHTING
    vec2 viewport; uint pattern; uint factor;
#endif
#ifdef MARTENSITE_MARKER_LIGHTING
    float size; uint width; uint height; uint policy;
#endif
#ifdef MARTENSITE_FIXED_GRID_POINT
    layout(offset=96) vec4 pointViewport;
    vec4 pointRasterViewport;
#endif
} transform;
layout(location=0) out vec4 vertexColor;
#ifdef MARTENSITE_CLIPPED_FAN_DEPTH_V4
#ifndef MARTENSITE_FIXED_LIGHTING
#error clipped fan V4 requires the fixed-lighting interface
#endif
layout(location=12) out vec4 clippedDepthRawGl;
#endif
#ifdef MARTENSITE_VERTEX_OUTPUT_CAPTURE
layout(xfb_buffer=0, xfb_offset=16, xfb_stride=32)
#endif
layout(location=1) out vec3 worldNormal;
layout(location=2) out vec3 viewDirection;
layout(location=4) out vec4 worldPosition;
layout(location=5) out vec4 frontColor;
layout(location=6) out vec4 backColor;
#ifdef MARTENSITE_FIXED_LIGHTING
layout(location=9) flat out vec4 fixedFrontColor;
#endif
#ifdef MARTENSITE_FIXED_LIGHTING
layout(location=10) flat out vec4 fixedBackColor;
#endif
#if defined(MARTENSITE_UNSNAPPED_PHONG_V2) && !defined(MARTENSITE_NATIVE_GRID12_V4)
layout(location=7) out vec4 phongClip;
layout(location=8) out vec3 phongNormal;
layout(location=9) out vec4 phongPosition;
layout(location=10) out vec3 phongView;
layout(location=12) out vec4 phongColor;
#endif
#include "fixed_grid_point.glsl"
void main() {
#ifdef MARTENSITE_VERTEX_OUTPUT_CAPTURE
    capturedVertexIndex = uint(gl_VertexIndex);
#endif
#ifndef MARTENSITE_NATIVE_LINE_XY_CLIP
    // Marker lighting overrides this below; polygon-point surfaces retain the default.
    gl_PointSize = 1.0;
#endif
    // 6983300fb's line-only replay modules predate 7596e1404's point-size write.
    // Keep those qualified bytes reproducible; the clip variant is never a point pipeline.
#ifdef MARTENSITE_FACTORIZED_TRIANGLES_V2
    worldPosition = clipPosition(transform.modelFromObject, position);
#else
    worldPosition = lighting.worldFromObject * position;
#endif
#ifdef MARTENSITE_FIXED_LIGHTING
    worldNormal = lighting.settings.x>=3u?fixedLightingNormal(normal):glslNormalize((lighting.normalFromObject * vec4(normal, 0.0)).xyz);
#elif defined(MARTENSITE_FACTORIZED_TRIANGLES_V2)
    // Column by column, as Mesa's GLSL front end lowers the native normal matrix product.
    worldNormal = glslNormalize(((lighting.normalFromObject[1].xyz * normal.y
        + lighting.normalFromObject[0].xyz * normal.x) + lighting.normalFromObject[2].xyz * normal.z));
#else
    worldNormal = glslNormalize((lighting.normalFromObject * vec4(normal, 0.0)).xyz);
#endif
    viewDirection = lighting.eye.w != 0.0 ? glslNormalize(lighting.eye.xyz - worldPosition.xyz) : lighting.eye.xyz;
    vertexColor = color;
    frontColor = backColor = vec4(0.0);
#ifdef MARTENSITE_FIXED_LIGHTING
    if ((lighting.settings.x == 1u || lighting.settings.x>=3u)) {
#else
    if (lighting.settings.x == 1u) {
#endif
#ifdef MARTENSITE_FIXED_LIGHTING
        vec4 lightingPosition=lighting.worldFromObject*fixedPosition;
        vec3 lightingNormal=fixedLightingNormal(fixedNormal);
        frontColor=computeFixedLighting(lightingNormal,lightingPosition,true,fixedColor);
        backColor=computeFixedLighting(lightingNormal,lightingPosition,false,fixedColor);
#else
        frontColor = computeLighting(worldNormal, viewDirection, worldPosition, true, color);
        backColor = computeLighting(worldNormal, viewDirection, worldPosition, false, color);
#endif
    }
#ifdef MARTENSITE_FIXED_LIGHTING
    fixedFrontColor=frontColor; fixedBackColor=backColor;
#endif
#ifdef MARTENSITE_MARKER_LIGHTING
    gl_PointSize = transform.size;
#endif
#ifdef MARTENSITE_FACTORIZED_TRIANGLES_V2
    gl_Position = clipPosition(transform.clipFromObject, worldPosition);
#else
    gl_Position = clipPosition(transform.clipFromObject, position);
#endif
#ifdef MARTENSITE_NATIVE_LINE_XY_CLIP
    nativeLineViewportClip(gl_Position);
#endif
#ifdef MARTENSITE_FLOAT64_WINDOW_DEPTH_V5
    windowDepthRawGLClip = gl_Position;
#endif
#ifdef MARTENSITE_CLIPPED_FAN_DEPTH_V4
    // Preserve the actual computed value before Vulkan's optional depth-range remap.
    clippedDepthRawGl = gl_Position;
#endif
#ifdef MARTENSITE_RASTER_CLIP_SPACE_V5
    rasterWorldClipDistances(gl_Position);
#endif
    if (kRemapClipDepth) gl_Position.z = 0.5 * (gl_Position.z + gl_Position.w);
#ifndef MARTENSITE_FIXED_LIGHTING
    writeShadowPositions(worldPosition);
#endif
#ifdef MARTENSITE_CLIPPING
    clippingWorldPosition = clipping.worldFromObject * position;
#if defined(MARTENSITE_RASTER_WORLD_CLIPPING_V2) && !defined(MARTENSITE_RASTER_CLIP_SPACE_V5)
    rasterWorldClipDistances(clippingWorldPosition);
#endif
#endif
#ifdef MARTENSITE_FIXED_GRID_POINT
    gl_Position = fixedGridPointCoverage(gl_Position, transform.size);
#endif
#if defined(MARTENSITE_UNSNAPPED_PHONG_V2) && !defined(MARTENSITE_NATIVE_GRID12_V4)
    phongClip = gl_Position;
    phongNormal = worldNormal;
    phongPosition = worldPosition;
    phongView = viewDirection;
    phongColor = vertexColor;
#endif
}
