#version 450
#extension GL_GOOGLE_include_directive : require
#ifdef USE_TEXTURES
  #extension GL_EXT_nonuniform_qualifier : require
#endif
#ifdef ADAPTIVE_SAMPLING_ATOMIC
  #extension GL_EXT_shader_atomic_float : require
#endif
// GlView::initRaytraceResources program 0: RaytraceBase.fs, PathtraceBase.fs
// and RaytraceRender.fs after the generateShaderPrefix string. PATH_TRACING,
// ADAPTIVE_SAMPLING[_ATOMIC] and USE_TEXTURES select binaries; the other prefix entries are
// constants here.
layout(constant_id = 0) const int STACK_SIZE = 10;
layout(constant_id = 1) const int NB_BOUNCES = 3;
layout(constant_id = 2) const bool THE_ZERO_TO_ONE_DEPTH = false;
layout(constant_id = 3) const bool TRANSPARENT_SHADOWS = false;
layout(constant_id = 4) const bool THE_SHIFT_sRGB = false;
layout(constant_id = 5) const bool TWO_SIDED_BXDF = false;
layout(constant_id = 6) const bool TONE_MAPPING_FILMIC = false;
layout(constant_id = 7) const bool IGNORE_NORMAL_MAP = false;
layout(constant_id = 8) const bool DEPTH_OF_FIELD = false;
layout(constant_id = 9) const bool BACKGROUND_CUBEMAP = false;
#include "raytrace_uniforms.glsl"
layout(location=0) in vec2 vPixel;
#ifdef ADAPTIVE_SAMPLING
// runPathtrace sets uFrameRndSeed before each pass of a frame, and the atomic variant extends
// the viewport over the offset tiles (OffsetTilesViewport); occViewport is that viewport.
layout(push_constant) uniform AdaptivePass {
  int uFrameRndSeed;
  int occPassReserved;
  ivec2 occViewport;
};
#endif
// The native gl_FragCoord (window y up): the quad spans the viewport of uWinSizeX x uWinSizeY,
// or occViewport with adaptive sampling.
#ifdef ADAPTIVE_SAMPLING
  #define occViewportSize vec2 (occViewport)
#else
  #define occViewportSize vec2 (uWinSizeX, uWinSizeY)
#endif
vec4 occFragCoord() {
  return vec4 (floor (vPixel * occViewportSize) + vec2 (0.5), gl_FragCoord.zw);
}
#include "glsl_builtins.glsl"
#include "raytrace_base.glsl"
#define MARTENSITE_TANGENT_SPACE_FUNCTION_ONLY
#include "tangent_space_normal.glsl"
#include "pathtrace_base.glsl"
vec4 occAccumFetch ();
#include "raytrace_render.glsl"
#if defined(PATH_TRACING) && !defined(ADAPTIVE_SAMPLING)
vec4 occAccumFetch () { return texelFetch (uAccumTexture, ivec2 (gl_FragCoord.xy), 0); }
#endif
