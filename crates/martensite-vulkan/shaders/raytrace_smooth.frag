#version 450
#extension GL_GOOGLE_include_directive : require
#ifdef USE_TEXTURES
  #extension GL_EXT_nonuniform_qualifier : require
#endif
// GlView::initRaytraceResources program 1: RaytraceBase.fs and
// RaytraceSmooth.fs after the generateShaderPrefix string (see raytrace_render.frag).
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
// The per-pass uniforms of runRaytrace's smoothing loop.
layout(push_constant) uniform RaytraceSmoothConstants {
  vec2 uFsaaOffset;
  int uSamples;
};
layout(location=0) in vec2 vPixel;
vec4 occFragCoord() {
  return vec4 (floor (vPixel * vec2 (uWinSizeX, uWinSizeY)) + vec2 (0.5), gl_FragCoord.zw);
}
#include "glsl_builtins.glsl"
#include "raytrace_base.glsl"
vec4 occFsaaFetch (int theDx, int theDy);
#include "raytrace_smooth.glsl"
// texelFetch outside the image reads zero, as the native drivers return.
vec4 occFsaaFetch (int theDx, int theDy)
{
  ivec2 aTexel = ivec2 (gl_FragCoord.xy) + ivec2 (theDx, theDy);
  ivec2 aSize = textureSize (uFSAAInputTexture, 0);
  bool anInside = all (greaterThanEqual (aTexel, ivec2 (0))) && all (lessThan (aTexel, aSize));
  return anInside ? texelFetch (uFSAAInputTexture, aTexel, 0) : vec4 (0.0);
}
