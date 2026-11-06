#version 450
#extension GL_GOOGLE_include_directive : require
#define MARTENSITE_FRAGMENT
#define MARTENSITE_SURFACE_FAMILY
layout(push_constant) uniform Transform {
    mat4 clip;float cutoff;uint flags;float fixedPrimaryAlpha;
#ifdef MARTENSITE_MESH_EDGES
    layout(offset=80) vec4 edgeColor;
    layout(offset=96) float edgeWidth;
    layout(offset=100) float edgeFeather;
    layout(offset=104) vec2 viewport;
#endif
} transform;
layout(location=0) in vec4 vertexColor;
layout(location=7) in vec4 surfaceUV;
#ifdef MARTENSITE_MESH_EDGES
layout(location=8) in vec3 edgeDistance;
layout(location=11) flat in vec3 edgeLineA;
layout(location=12) flat in vec3 edgeLineB;
layout(location=13) flat in vec3 edgeLineC;
layout(location=14) flat in uint edgeLinesActive;
#endif
#ifdef MARTENSITE_SURFACE_TEXTURE
#ifdef MARTENSITE_LIGHTING
#define MARTENSITE_SURFACE_SET 2
#else
#define MARTENSITE_SURFACE_SET 0
#endif
layout(set=MARTENSITE_SURFACE_SET,binding=0) uniform sampler2D surfaceImage;
#endif
vec4 surfaceTexel() {
    // OpenGL incomplete texture sampling is black with alpha one; loading still failed.
    if ((transform.flags&16u)!=0u) return vec4(0.0,0.0,0.0,1.0);
#ifdef MARTENSITE_SURFACE_TEXTURE
#ifdef MARTENSITE_FIXED_RECIPROCAL_Q_V4
    // The driver's fixed-function texture stage multiplies by one reciprocal of q.
    vec4 texel=texture(surfaceImage,surfaceUV.st*(1.0/surfaceUV.w));
#else
    vec4 texel=texture(surfaceImage,surfaceUV.st/surfaceUV.w);
#endif
    // the reference implementation's ToRenderSRGB disables texture/output sRGB conversion in the FFP policy.
    if ((transform.flags&1024u)!=0u) texel.rgb=vec3(1.0);
    return texel;
#else
    return vec4(1.0);
#endif
}
#ifdef MARTENSITE_MATERIAL_MAPS
#include "material_maps.glsl"
#endif
#ifdef MARTENSITE_LIGHTING
#include "lighting.glsl"
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
#ifdef MARTENSITE_MATERIAL_MAPS
#include "tangent_space_normal.glsl"
#endif
#include "lit_fragment.glsl"
#endif
#include "clipping.glsl"
#if defined(MARTENSITE_OIT_WEIGHTED) || defined(MARTENSITE_OIT_PEELING)
#include "oit.glsl"
#else
#include "native_fragment_alpha.glsl"
layout(location=0) out vec4 outColor;
#endif
vec4 sampledSurfaceColor() {
    vec4 primary=vertexColor;
#if !defined(MARTENSITE_LIGHTING) && !defined(MARTENSITE_SURFACE_TEXTURE)
    // ADR 0021: the untextured auxiliary vec4 owns back RGBA, never sampler coordinates.
    if ((transform.flags&4096u)!=0u) return gl_FrontFacing?vertexColor:surfaceUV;
#endif
#ifdef MARTENSITE_LIGHTING
#ifdef MARTENSITE_FIXED_LIGHTING
    if(lighting.settings.x>=3u)primary=litFragmentColor();
#endif
#endif
    if ((transform.flags&2048u)!=0u) primary.a=transform.fixedPrimaryAlpha;
    if ((transform.flags&32u)!=0u) return primary*surfaceTexel();
    if ((transform.flags&128u)!=0u) return vec4(vertexColor.rgb,surfaceTexel().a);
    if ((transform.flags&64u)!=0u) return vec4(mix(primary.rgb,surfaceTexel().rgb,surfaceTexel().a),primary.a);
#ifdef MARTENSITE_LIGHTING
    vec4 result=litFragmentColor();
    if (lighting.settings.x==1u) result*=surfaceTexel();
    return result;
#else
    bool useVertex=(transform.flags&2u)!=0u&&(gl_FrontFacing||(transform.flags&4u)!=0u);
    return useVertex?vertexColor:((transform.flags&17u)!=0u?surfaceTexel():vertexColor);
#endif
}
#ifdef MARTENSITE_MESH_EDGES
// Distance to the geometry shader's homogeneous edge lines (vertices at infinity), in the same
// viewport frame: top-left logical rasterization mirrors the attachment row.
float homogeneousEdgeDistance() {
#ifdef MARTENSITE_TOP_LEFT_LOGICAL_COORDINATES
    vec3 point=vec3(gl_FragCoord.x,logicalVisualWindowY(gl_FragCoord.y,transform.viewport.y),1.0);
#else
    vec3 point=vec3(gl_FragCoord.xy,1.0);
#endif
    return min(min(abs(dot(edgeLineA,point)),abs(dot(edgeLineB,point))),abs(dot(edgeLineC,point)));
}
#endif
void main() {
#if defined(MARTENSITE_LIGHTING) && !defined(MARTENSITE_FIXED_LIGHTING)
    captureFacetNormal();
#endif
#if defined(MARTENSITE_OIT_WEIGHTED) || defined(MARTENSITE_OIT_PEELING)
    if(oitEarlyReturn())return;
    vec4 outColor;
#endif
#ifdef MARTENSITE_CLIPPING
    discardClippedFragment();
#endif
    outColor=sampledSurfaceColor();
#ifdef MARTENSITE_MESH_EDGES
    float distance=edgeLinesActive!=0u?homogeneousEdgeDistance():min(min(edgeDistance.x,edgeDistance.y),edgeDistance.z);
    float interior=smoothstep(transform.edgeWidth-transform.edgeFeather*0.5,transform.edgeWidth+transform.edgeFeather*0.5,distance);
    // mix() spelled `x + (y - x) * a`: Mesa's GLSL front end lowers the native mix that way, while
    // SPIR-V OpFMix reaches llvmpipe as `x * (1 - a) + y * a` (colored_quad.frag), which rounds a
    // fully interior `a = 1` differently.
    outColor=transform.edgeColor.a<0.0?vec4(outColor.rgb,1.0-interior):transform.edgeColor+(outColor-transform.edgeColor)*interior;
#endif
    // Surface masking uses native strict comparison; glyph cutoff remains <= in its independent shader.
    if ((transform.flags&8u)!=0u&&outColor.a<transform.cutoff) discard;
#if defined(MARTENSITE_OIT_WEIGHTED) || defined(MARTENSITE_OIT_PEELING)
    oitWriteColor(outColor);
#else
    nativePostAlpha(outColor.a);
#endif
}
