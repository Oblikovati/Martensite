#ifndef MARTENSITE_GLSL_NORMALIZE
#define MARTENSITE_GLSL_NORMALIZE
// normalize() as Mesa's GLSL front end lowers the built-in (builtin_functions.cpp: x * rsq(dot)).
// SPIR-V's GLSL.std.450 Normalize is lowered as x / sqrt(dot) instead, which rounds the native
// programs' directions one ulp apart (the skydome and PBR-baking controls of 2026-10-28:
// our SPIR-V in Mesa OpenGL equals the native faces only with this spelling).
vec3 glslNormalize (in vec3 theVec)
{
  return theVec * inversesqrt (dot (theVec, theVec));
}
#endif
