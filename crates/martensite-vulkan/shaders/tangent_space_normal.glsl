// the reference implementation resources/Shaders/TangentSpaceNormal.glsl, unchanged.
#include "glsl_normalize.glsl"
//! Calculates transformation from tangent space and apply it to value from normal map to get normal in object space
vec3 TangentSpaceNormal (in mat2 theDeltaUVMatrix,
                         in mat2x3 theDeltaVectorMatrix,
                         in vec3 theNormalMapValue,
                         in vec3 theNormal,
                         in bool theIsInverse)
{
  theNormalMapValue = glslNormalize(theNormalMapValue * 2.0 - vec3(1.0));
  // Inverse matrix
  theDeltaUVMatrix = mat2 (theDeltaUVMatrix[1][1], -theDeltaUVMatrix[0][1], -theDeltaUVMatrix[1][0], theDeltaUVMatrix[0][0]);
  theDeltaVectorMatrix = theDeltaVectorMatrix * theDeltaUVMatrix;
  // Gram-Schmidt orthogonalization
  theDeltaVectorMatrix[1] = glslNormalize(theDeltaVectorMatrix[1] - dot(theNormal, theDeltaVectorMatrix[1]) * theNormal);
  theDeltaVectorMatrix[0] = cross(theDeltaVectorMatrix[1], theNormal);
  float aDirection = theIsInverse ? -1.0 : 1.0;
  return mat3 (aDirection * theDeltaVectorMatrix[0], aDirection * theDeltaVectorMatrix[1], theNormal) * theNormalMapValue;
}
// The ray-tracing programs take only TangentSpaceNormal (GlView_Raytrace sources).
#ifndef MARTENSITE_TANGENT_SPACE_FUNCTION_ONLY
// The native Y derivative: top-left logical rasterization mirrors rows, as the flat normal does.
#ifdef MARTENSITE_TOP_LEFT_LOGICAL_COORDINATES
#define nativeDFdy(theValue) (-dFdy(theValue))
#else
#define nativeDFdy(theValue) dFdy(theValue)
#endif
// The getStdProgramPhong THE_HAS_TEXTURE_NORMAL block: a map texel with W > 0.5 replaces the
// interpolated normal; the black mock (W = 0) keeps it. Derivatives are taken in uniform control
// flow; their values equal the native ones taken inside the branch.
vec3 mappedSurfaceNormal(vec3 theNormal) {
  vec2 aTexCoord = materialMapCoord();
  vec4 aMapNormalValue = texture(surfaceNormalMap, aTexCoord);
  mat2 aDeltaUVMatrix = mat2 (dFdx(aTexCoord), nativeDFdy(aTexCoord));
  mat2x3 aDeltaVectorMatrix = mat2x3 (dFdx (worldPosition.xyz), nativeDFdy (worldPosition.xyz));
  if (aMapNormalValue.w <= 0.5) return theNormal;
  return TangentSpaceNormal (aDeltaUVMatrix, aDeltaVectorMatrix, aMapNormalValue.xyz, theNormal, !gl_FrontFacing);
}
#endif
