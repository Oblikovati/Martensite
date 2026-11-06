#version 450
#extension GL_GOOGLE_include_directive : require
// Graphic3d cameras emit OpenGL clip depth. Without VK_EXT_depth_clip_control the stage remaps
// it to Vulkan's zero-to-one range; with it (negativeOneToOne) the viewport transform applies
// OpenGL's mapping, as the native view does (backend/depth_equivalence.rs).
layout(constant_id = 7) const bool kRemapClipDepth = true;
#include "clip_position.glsl"
#ifdef MARTENSITE_LIGHTING
#include "lighting.glsl"
#endif
#include "clipping.glsl"
layout(location=0) in vec4 position;
layout(location=1) in vec4 color;
layout(location=2) in vec4 uv;
#ifdef MARTENSITE_LIGHTING
layout(location=3) in vec3 normal;
#ifdef MARTENSITE_FIXED_LIGHTING
layout(location=4) in vec4 fixedPosition;
layout(location=5) in vec3 fixedNormal;
layout(location=6) in vec4 fixedColor;
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
#endif
layout(location=0) out vec4 vertexColor;
layout(location=7) out vec4 surfaceUV;
#ifdef MARTENSITE_SPHERE_MAP
#include "sphere_map.glsl"
#endif
#if defined(MARTENSITE_UV_POINT) && defined(MARTENSITE_SURFACE_TEXTURE)
#ifdef MARTENSITE_LIGHTING
layout(set=2,binding=0) uniform sampler2D pointBaseColor;
#else
layout(set=0,binding=0) uniform sampler2D pointBaseColor;
#endif
#endif
layout(push_constant) uniform Transform {mat4 clip;float cutoff;uint flags;
#ifdef MARTENSITE_UV_POINT
    float primaryAlpha;float pointSize;
#endif
#ifdef MARTENSITE_FIXED_GRID_POINT
    layout(offset=96) vec4 pointViewport;
    vec4 pointRasterViewport;
#endif
} transform;
#include "fixed_grid_point.glsl"
void main() {
    // Polygon-point surfaces use the native default size; triangle fills ignore this output.
#ifdef MARTENSITE_UV_POINT
    gl_PointSize = transform.pointSize;
#else
    gl_PointSize = 1.0;
#endif
#if defined(MARTENSITE_UV_POINT) && defined(MARTENSITE_SURFACE_TEXTURE)
    vec4 pointColor=(transform.flags&8192u)!=0u?color:textureLod(pointBaseColor,uv.xy,0.0);
#define MARTENSITE_VERTEX_BASE pointColor
#else
#define MARTENSITE_VERTEX_BASE color
#endif
    vertexColor=MARTENSITE_VERTEX_BASE; surfaceUV=uv;
#ifdef MARTENSITE_SPHERE_MAP
    surfaceUV=sphereMapTexCoord(position,normal);
#endif
#ifdef MARTENSITE_LIGHTING
    worldPosition=lighting.worldFromObject*position;
#ifdef MARTENSITE_FIXED_LIGHTING
    worldNormal=lighting.settings.x>=3u?fixedLightingNormal(normal):glslNormalize((lighting.normalFromObject*vec4(normal,0.0)).xyz);
#else
    worldNormal=glslNormalize((lighting.normalFromObject*vec4(normal,0.0)).xyz);
#endif
    viewDirection=lighting.eye.w!=0.0?glslNormalize(lighting.eye.xyz-worldPosition.xyz):lighting.eye.xyz;
    frontColor=backColor=vec4(0.0);
#ifdef MARTENSITE_FIXED_LIGHTING
    if ((lighting.settings.x==1u || lighting.settings.x>=3u)) {
#else
    if (lighting.settings.x==1u) {
#endif
#ifdef MARTENSITE_FIXED_LIGHTING
        vec4 lightingPosition=lighting.worldFromObject*fixedPosition;
        vec3 lightingNormal=fixedLightingNormal(fixedNormal);
        frontColor=computeFixedLighting(lightingNormal,lightingPosition,true,fixedColor);
        backColor=computeFixedLighting(lightingNormal,lightingPosition,false,fixedColor);
#else
        frontColor=computeLighting(worldNormal,viewDirection,worldPosition,true,MARTENSITE_VERTEX_BASE);
        backColor=computeLighting(worldNormal,viewDirection,worldPosition,false,MARTENSITE_VERTEX_BASE);
#endif
    }
#ifdef MARTENSITE_FIXED_LIGHTING
    fixedFrontColor=frontColor; fixedBackColor=backColor;
#else
    writeShadowPositions(worldPosition);
#endif
#endif
    gl_Position=clipPosition(transform.clip,position);
    if (kRemapClipDepth) gl_Position.z = 0.5 * (gl_Position.z + gl_Position.w);
#ifdef MARTENSITE_CLIPPING
    clippingWorldPosition=clipping.worldFromObject*position;
#endif
#ifdef MARTENSITE_FIXED_GRID_POINT
    gl_Position = fixedGridPointCoverage(gl_Position, transform.pointSize);
#endif
}
