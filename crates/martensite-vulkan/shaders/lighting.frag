#version 450
#extension GL_GOOGLE_include_directive : require
#ifdef MARTENSITE_UNSNAPPED_PHONG_V2
#extension GL_EXT_fragment_shader_barycentric : require
#endif
#if defined(MARTENSITE_TRIANGLE_GRID_COVERAGE_V2) || defined(MARTENSITE_WORLD_CLIP_GRID12_V1)
#extension GL_ARB_gpu_shader_int64 : require
#endif
#ifdef MARTENSITE_NATIVE_GRID12_V4
layout(depth_any) out float gl_FragDepth;
#endif
#define MARTENSITE_FRAGMENT
#include "lighting.glsl"
#include "clipping.glsl"
#ifdef MARTENSITE_VERTEX_OUTPUT_CAPTURE
layout(location=11) flat in uint capturedVertexIndex;
#endif
layout(location=0) in vec4 vertexColor;
layout(location=1) in vec3 worldNormal;
layout(location=2) in vec3 viewDirection;
layout(location=4) in vec4 worldPosition;
layout(location=5) in vec4 frontColor;
layout(location=6) in vec4 backColor;
#ifdef MARTENSITE_FIXED_LIGHTING
layout(location=9) flat in vec4 fixedFrontColor;
#endif
#ifdef MARTENSITE_FIXED_LIGHTING
layout(location=10) flat in vec4 fixedBackColor;
#endif
#if defined(MARTENSITE_OIT_WEIGHTED) || defined(MARTENSITE_OIT_PEELING)
#include "oit.glsl"
#else
#include "native_fragment_alpha.glsl"
layout(location=0) out vec4 fragmentColor;
#endif
#ifdef MARTENSITE_MARKER_LIGHTING
#define MARTENSITE_SPRITE_SET 2
#include "marker.glsl"
layout(push_constant) uniform Transform {
    mat4 clip;float size;uint width;uint height;uint policy;
#ifdef MARTENSITE_FIXED_LIGHTING
    float materialCutoff;
#endif
} transform;
#endif
#ifdef MARTENSITE_LINE_LIGHTING
layout(push_constant) uniform LineTransform { mat4 clip; vec2 viewport; uint pattern; uint factor; } transform;
#endif
#ifdef MARTENSITE_UNSNAPPED_PHONG_V2
#include "phong_interpolation_v2.glsl"
#ifdef MARTENSITE_UNSNAPPED_PHONG_V3
#include "phong_projected_axis_v3.glsl"
#endif
#endif
#ifdef MARTENSITE_TRIANGLE_GRID_COVERAGE_V2
#include "triangle_grid_coverage_v2.glsl"
#endif
#ifdef MARTENSITE_WORLD_CLIP_GRID12_V1
#include "world_clip_grid12_v1.glsl"
#endif
#include "lit_fragment.glsl"

#ifdef MARTENSITE_WINDOW_DEPTH_FRAGMENT
#ifdef MARTENSITE_FLOAT64_WINDOW_DEPTH_V5
#include "window_depth_float64_fragment.glsl"
#else
#include "window_depth_fragment.glsl"
#endif
#endif
void main() {
#ifdef MARTENSITE_WINDOW_DEPTH_FRAGMENT
    writeReferenceWindowDepth();
#endif
#ifdef MARTENSITE_TRIANGLE_GRID_COVERAGE_V2
#ifdef MARTENSITE_NATIVE_GRID12_V4
    if (triangleGridCulledV4()) discard;
#endif
    if (!triangleGridCoveredV2()) discard;
#ifdef MARTENSITE_NATIVE_GRID12_V4
    gl_FragDepth = triangleGridLogicalDepthV4();
#endif
#endif
#ifdef MARTENSITE_UNSNAPPED_PHONG_V3
    if (lighting.settings.x == 2u && phongExactProjectedAxis()) {
        fragmentColor = vec4(0.0);
        return;
    }
#endif
#ifndef MARTENSITE_FIXED_LIGHTING
    captureFacetNormal();
#endif
#if defined(MARTENSITE_OIT_WEIGHTED) || defined(MARTENSITE_OIT_PEELING)
    if(oitEarlyReturn())return;
    vec4 fragmentColor;
#endif
#ifdef MARTENSITE_CLIPPING
    discardClippedFragment();
#endif
// the reference implementation installs stippling only in its unlit shader. Nodal lit lines
    // intentionally retain solid coverage even when their retained aspect is dashed.
    fragmentColor = litFragmentColor();
#ifdef MARTENSITE_MARKER_LIGHTING
    fragmentColor = litMarkerColor(markerTexel(uvec2(transform.width, transform.height)), fragmentColor, transform.policy);
#ifdef MARTENSITE_FIXED_LIGHTING
    // GL_POINTS bypasses sprite alpha testing; sprite GL_GEQUAL accepts the exact cutoff.
    float cutoff=transform.policy==4u?transform.materialCutoff:0.1;
    if (fragmentColor.a < cutoff) discard;
#else
    if (fragmentColor.a <= 0.1) discard;
#endif
#endif
#if defined(MARTENSITE_OIT_WEIGHTED) || defined(MARTENSITE_OIT_PEELING)
    oitWriteColor(fragmentColor);
#else
    nativePostAlpha(fragmentColor.a);
#endif
}
