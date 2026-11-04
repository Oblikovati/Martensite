// Martensite per-pixel Blinn-Phong sample program, fragment stage: clip planes, then ambient,
// diffuse and specular terms of every directional, positional and spot light.

THE_SHADER_IN vec3 View;
THE_SHADER_IN vec3 Normal;
THE_SHADER_IN vec4 Position;
THE_SHADER_IN vec4 PositionWorld;

vec3 SumDiffuse;
vec3 SumSpecular;

// Adds one light arriving along the unit vector theToLight, scaled by theAttenuation.
void addLight (in int theId, in vec3 theToLight, in vec3 theNormal, in vec3 theEye, in float theAttenuation)
{
  vec3  aHalfway = normalize (theToLight + theEye);
  vec3  aSide    = gl_FrontFacing ? theNormal : -theNormal;
  float aLambert = max (0.0, dot (aSide, theToLight));
  float aGloss   = max (0.0, dot (aSide, aHalfway));
  float aShine   = 0.0;
  if (aLambert > 0.0)
  {
    aShine = pow (aGloss, occMaterial_Shininess (gl_FrontFacing));
  }
  SumDiffuse  += occLight_Diffuse  (theId).rgb * aLambert * theAttenuation;
  SumSpecular += occLight_Specular (theId).rgb * aShine   * theAttenuation;
}

void directional (in int theId, in vec3 theNormal, in vec3 theEye)
{
  vec3 aToLight = normalize (occLight_Position (theId).xyz);
  if (!occLight_IsHeadlight (theId))
  {
    aToLight = vec3 (occWorldViewMatrix * vec4 (aToLight, 0.0));
  }
  addLight (theId, aToLight, theNormal, theEye, 1.0);
}

// The unit vector from thePoint to the light and its distance.
vec4 towardsLight (in int theId, in vec3 thePoint)
{
  vec3 aLight = occLight_Position (theId).xyz;
  if (!occLight_IsHeadlight (theId))
  {
    aLight = vec3 (occWorldViewMatrix * vec4 (aLight, 1.0));
  }
  aLight -= thePoint;
  float aDistance = length (aLight);
  return vec4 (aLight * (1.0 / aDistance), aDistance);
}

float distanceFalloff (in int theId, in float theDistance)
{
  return 1.0 / (occLight_ConstAttenuation (theId) + occLight_LinearAttenuation (theId) * theDistance);
}

void positional (in int theId, in vec3 theNormal, in vec3 theEye, in vec3 thePoint)
{
  vec4 aToLight = towardsLight (theId, thePoint);
  addLight (theId, aToLight.xyz, theNormal, theEye, distanceFalloff (theId, aToLight.w));
}

void spot (in int theId, in vec3 theNormal, in vec3 theEye, in vec3 thePoint)
{
  vec4 aToLight = towardsLight (theId, thePoint);
  vec3 anAxis   = occLight_SpotDirection (theId).xyz;
  if (!occLight_IsHeadlight (theId))
  {
    anAxis = vec3 (occWorldViewMatrix * vec4 (anAxis, 0.0));
  }
  anAxis = normalize (anAxis);
  float aCone = dot (anAxis, -aToLight.xyz);
  if (aCone >= 1.0 || aCone < cos (occLight_SpotCutOff (theId)))
  {
    return;
  }
  float aFalloff  = distanceFalloff (theId, aToLight.w);
  float aExponent = occLight_SpotExponent (theId);
  if (aExponent > 0.0)
  {
    aFalloff *= pow (aCone, aExponent * 128.0);
  }
  addLight (theId, aToLight.xyz, theNormal, theEye, aFalloff);
}

vec4 shade (in vec3 theNormal, in vec3 theEye, in vec4 thePosition)
{
  SumDiffuse  = vec3 (0.0);
  SumSpecular = vec3 (0.0);
  vec3 aPoint = thePosition.xyz / thePosition.w;
  for (int anId = 0; anId < occLightSourcesCount; ++anId)
  {
    int aKind = occLight_Type (anId);
    if (aKind == OccLightType_Direct)     { directional (anId, theNormal, theEye); }
    else if (aKind == OccLightType_Point) { positional  (anId, theNormal, theEye, aPoint); }
    else if (aKind == OccLightType_Spot)  { spot        (anId, theNormal, theEye, aPoint); }
  }
  vec4 aDiffuse = occMaterial_Diffuse (gl_FrontFacing);
  vec3 aColor = occLightAmbient.rgb * occMaterial_Ambient (gl_FrontFacing).rgb
              + SumDiffuse          * aDiffuse.rgb
              + SumSpecular         * occMaterial_Specular (gl_FrontFacing).rgb
                                    + occMaterial_Emission (gl_FrontFacing).rgb;
  return vec4 (aColor, aDiffuse.a);
}

void main()
{
  for (int aPlane = 0; aPlane < occClipPlaneCount; ++aPlane)
  {
    vec4 anEquation = occClipPlaneEquations[aPlane];
    if (dot (anEquation.xyz, PositionWorld.xyz / PositionWorld.w) + anEquation.w < 0.0)
    {
      discard;
    }
  }
  occSetFragColor (shade (normalize (Normal), normalize (View), Position));
}
