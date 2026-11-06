#ifndef MARTENSITE_GLSL_BUILTINS
#define MARTENSITE_GLSL_BUILTINS
#include "glsl_normalize.glsl"
// mix() and reflect() as Mesa's GLSL front end lowers the built-ins of a native program. SPIR-V's
// FMix and Reflect reach llvmpipe in other forms (x * (1 - a) + y * a, colored_quad.frag), and the
// ray tracer's coordinates round apart from native through them.
float glslMix (float x, float y, float a) { return x + (y - x) * a; }
vec2  glslMix (vec2 x, vec2 y, float a) { return x + (y - x) * a; }
vec3  glslMix (vec3 x, vec3 y, float a) { return x + (y - x) * a; }
vec4  glslMix (vec4 x, vec4 y, float a) { return x + (y - x) * a; }
vec2  glslMix (vec2 x, vec2 y, vec2 a) { return x + (y - x) * a; }
vec3  glslMix (vec3 x, vec3 y, vec3 a) { return x + (y - x) * a; }
vec4  glslMix (vec4 x, vec4 y, vec4 a) { return x + (y - x) * a; }
// The boolean mix is a selection, in either front end.
float glslMix (float x, float y, bool a) { return mix (x, y, a); }
vec3  glslMix (vec3 x, vec3 y, bool a) { return a ? y : x; }
vec3  glslReflect (vec3 theIncident, vec3 theNormal)
{
  return theIncident - 2.0 * (dot (theNormal, theIncident) * theNormal);
}
#define mix glslMix
#define normalize glslNormalize
#define reflect glslReflect
#endif
