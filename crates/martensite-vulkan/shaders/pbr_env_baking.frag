#version 450
#extension GL_GOOGLE_include_directive : require
// the reference implementation resources/Shaders/PBREnvBaking.fs, compiled once with THE_TO_BAKE_DIFFUSE (the
// 9x1 RGBA32F spherical-harmonics map) and once with THE_TO_BAKE_SPECULAR (one level of the
// prefiltered cube map). The source environment cube map is set 0, binding 0; the native
// uniforms are push constants. THE_TO_PACK_FLOAT (the RGBA8 fallback of devices that cannot
// render floats) is not compiled: Martensite requires a renderable RGBA32F format instead.
#include "environment_cube.glsl"
#include "pbr_env_baking.glsl"
#include "pbr_brdf.glsl"
#include "glsl_normalize.glsl"
#define PI_2     6.283185307
//! DeclarationsImpl.glsl occRoughness with MIN_ROUGHNESS 0.01.
float occRoughness (in float theNormalizedRoughness) { return theNormalizedRoughness * (1.0 - 0.01) + 0.01; }

layout(location=0) in vec3 ViewDirection; //!< direction of fetching from environment cubemap
layout(location=0) out vec4 occFragColor;
layout(set=0, binding=0) uniform samplerCube uEnvMap; //!< source of baking (environment cubemap)

//! Returns coordinates of point theNumber from Hammersley point set having size theSize.
vec2 hammersley (in int theNumber,
                 in int theSize)
{
  int aDenominator = 2;
  int aNumber = theNumber;
  float aVanDerCorput = 0.0;
  for (int i = 0; i < 32; ++i)
  {
    if (aNumber > 0)
    {
      aVanDerCorput += mod(float(aNumber), 2.0) / float(aDenominator);
      aNumber /= 2;
      aDenominator *= 2;
    }
  }
  return vec2(float(theNumber) / float(theSize), aVanDerCorput);
}

//! This function does importance sampling on hemisphere surface using GGX normal distribution function
//! in tangent space (positive z axis is surface normal direction).
vec3 importanceSample (in vec2  theHammersleyPoint,
                       in float theRoughness)
{
  float aPhi = PI_2 * theHammersleyPoint.x;
  theRoughness *= theRoughness;
  theRoughness *= theRoughness;
  float aCosTheta = sqrt((1.0 - theHammersleyPoint.y) / (1.0 + (theRoughness - 1.0) * theHammersleyPoint.y));
  float aSinTheta = sqrt(1.0 - aCosTheta * aCosTheta);
  return vec3(aSinTheta * cos(aPhi),
              aSinTheta * sin(aPhi),
              aCosTheta);
}

//! This function uniformly generates samples on whole sphere.
vec3 sphereUniformSample (in vec2 theHammersleyPoint)
{
  float aPhi = PI_2 * theHammersleyPoint.x;
  float aCosTheta = 2.0 * theHammersleyPoint.y - 1.0;
  float aSinTheta = sqrt(1.0 - aCosTheta * aCosTheta);
  return vec3(aSinTheta * cos(aPhi),
              aSinTheta * sin(aPhi),
              aCosTheta);
}

//! Transforms resulted sampled direction from tangent space to world space considering the surface normal.
vec3 fromTangentSpace (in vec3 theVector,
                       in vec3 theNormal)
{
  vec3 anUp = (abs(theNormal.z) < 0.999) ? vec3(0.0, 0.0, 1.0) : vec3(1.0, 0.0, 0.0);
  vec3 anX = glslNormalize(cross(anUp, theNormal));
  vec3 anY = cross(theNormal, anX);
  return anX * theVector.x + anY * theVector.y + theNormal * theVector.z;
}

#ifdef THE_TO_BAKE_DIFFUSE
// The squares 0.282095 * 0.282095, 0.488603 * 0.488603, 1.092548 * 1.092548,
// 0.315392 * 0.315392 and 0.546274 * 0.546274 as Mesa's GLSL front end folds them, in single
// precision; glslang folds constant products in double precision, which rounds three of them
// to a neighbouring float.
const float aSHBasisFuncCoeffs[9] = float[9]
(
  0.07957758,
  0.23873289,
  0.23873289,
  0.23873289,
  1.1936612,
  1.1936612,
  1.1936612,
  0.099472106,
  0.2984153
);
const float aSHCosCoeffs[9] = float[9]
(
  3.141593,
  2.094395,
  2.094395,
  2.094395,
  0.785398,
  0.785398,
  0.785398,
  0.785398,
  0.785398
);

//! Bakes diffuse IBL map's spherical harmonics coefficients.
vec3 bakeDiffuseSH()
{
  int anId = int(gl_FragCoord.x);
  float aCoef;
  aCoef = aSHCosCoeffs[anId] * aSHBasisFuncCoeffs[anId];
  vec3 aRes = vec3 (0.0);
  for (int aSampleIter = 0; aSampleIter < bake.uSamplesNum; ++aSampleIter)
  {
    vec2 aHammersleyPoint = hammersley (aSampleIter, bake.uSamplesNum);
    vec3 aDir = sphereUniformSample (aHammersleyPoint);

    vec3 aVal = texture (uEnvMap, cubemapVectorTransform (aDir, bake.uYCoeff, bake.uZCoeff)).rgb;
    float aFunc[9];
    aFunc[0] = 1.0;

    aFunc[1] = aDir.x;
    aFunc[2] = aDir.y;
    aFunc[3] = aDir.z;

    aFunc[4] = aDir.x * aDir.z;
    aFunc[5] = aDir.y * aDir.z;
    aFunc[6] = aDir.x * aDir.y;

    aFunc[7] = 3.0 * aDir.z * aDir.z - 1.0;
    aFunc[8] = aDir.x * aDir.x - aDir.y * aDir.y;

    aRes += aVal * aFunc[anId];
  }

  return 4.0 * aRes * aCoef / float(bake.uSamplesNum);
}
#endif

#ifdef THE_TO_BAKE_SPECULAR
//! Computes a single sample for specular IBL map.
vec4 specularMapSample (in vec3  theNormal,
                        in float theRoughness,
                        in int   theNumber,
                        in int   theSize)
{
  vec2 aHammersleyPoint = hammersley (theNumber, theSize);
  vec3 aHalf = importanceSample (aHammersleyPoint, occRoughness (theRoughness));
  float aHdotV = aHalf.z;
  aHalf = fromTangentSpace (aHalf, theNormal);
  vec3  aLight = -reflect (theNormal, aHalf);
  float aNdotL = dot (aLight, theNormal);
  if (aNdotL > 0.0)
  {
    float aSolidAngleSample = 1.0 / (float(theSize) * (occPBRDistribution (aHdotV, theRoughness) * 0.25 + 0.0001) + 0.0001);
    float aLod = (theRoughness == 0.0) ? 0.0 : 0.5 * log2 (aSolidAngleSample / bake.uEnvSolidAngleSource);
    return vec4 (textureLod (uEnvMap, aLight, aLod).rgb * aNdotL, aNdotL);
  }
  return vec4 (0.0);
}

//! Bakes specular IBL map.
vec3 bakeSpecularMap (in vec3  theNormal,
                      in float theRoughness)
{
  vec4 aResult = vec4(0.0);
  if (theRoughness == 0.0)
  {
    aResult = specularMapSample (theNormal, theRoughness, 0, 1);
  }
  else
  {
    for (int aSampleIter = 0; aSampleIter < bake.uSamplesNum; ++aSampleIter)
    {
      aResult += specularMapSample (theNormal, theRoughness, aSampleIter, bake.uSamplesNum);
    }
  }
  return aResult.xyz / aResult.w;
}
#endif

void main()
{
  vec3 aViewDirection = glslNormalize (ViewDirection);
#ifdef THE_TO_BAKE_DIFFUSE
  vec4 aRes = vec4 (bakeDiffuseSH(), 1.0);
  occFragColor = aRes;
#else
  float aRoughness = float(bake.uCurrentLevel) / float(bake.occNbSpecIBLLevels - 1);
  occFragColor = vec4 (bakeSpecularMap (aViewDirection, aRoughness), 1.0);
#endif
}
