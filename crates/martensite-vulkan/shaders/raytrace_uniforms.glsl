// The loose uniforms RaytraceBase.fs, RaytraceRender.fs and RaytraceSmooth.fs as
// one std140 block (GlView::setUniformState); the byte offsets are mirrored by
// src/raytrace/uniforms.rs. Each vec3 shares its 16-byte slot with the scalar after it.
layout(set=0, binding=0, std140) uniform RaytraceUniforms {
  mat4 uUnviewMat;          //   0
  mat4 uViewMat;            //  64
  vec4 uGlobalAmbient;      // 128
  vec4 uBackColorTop;       // 144
  vec4 uBackColorBot;       // 160
  vec3 uOriginLT; int uWinSizeX;          // 176
  vec3 uOriginLB; int uWinSizeY;          // 192
  vec3 uOriginRT; int uLightCount;        // 208
  vec3 uOriginRB; int uShadowsEnabled;    // 224
  vec3 uDirectLT; int uReflectEnabled;    // 240
  vec3 uDirectLB; int uEnvMapEnabled;     // 256
  vec3 uDirectRT; int uEnvMapForBack;     // 272
  vec3 uDirectRB; float uSceneRadius;     // 288
  vec3 uEyeOrig; float uSceneEpsilon;     // 304
  vec3 uEyeView; float uApertureRadius;   // 320
  vec3 uEyeVert; float uFocalPlaneDist;   // 336
  vec3 uEyeSide; float uOffsetY;          // 352
  vec2 uEyeSize; vec2 occReserved368;     // 368 (uFsaaOffset is a smoothing push constant)
  ivec2 uTileSize; int uYCoeff; int uZCoeff;                                       // 384
#ifdef ADAPTIVE_SAMPLING
  // uFrameRndSeed changes between the passes of a frame: a push constant (raytrace_render.frag).
  int occBlockFrameRndSeed;
#else
  int uFrameRndSeed;
#endif
  int uBlockedRngEnabled; int uAccumSamples; float uMaxRadiance; // 400
};
