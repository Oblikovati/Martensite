//! The GLSL text blocks shared by the standard programs (the anonymous-namespace constants).

/// `THE_VARY_TexCoord_Trsf`: the transformed texture coordinate in the vertex stage.
pub(super) const THE_VARY_TEX_COORD_TRSF: &str = concat!(
    "\n  float aRotSin = occTextureTrsf_RotationSin();",
    "\n  float aRotCos = occTextureTrsf_RotationCos();",
    "\n  vec2  aTex2   = vec2 (occTexCoord.x * aRotCos - occTexCoord.y * aRotSin,",
    "\n                        occTexCoord.x * aRotSin + occTexCoord.y * aRotCos);",
    "\n  aTex2 = (aTex2 + occTextureTrsf_Translation()) * occTextureTrsf_Scale();",
    "\n  TexCoord = vec4(aTex2, occTexCoord.zw);",
);

/// `THE_FUNC_transformNormal_view`: the normal from model to view space.
pub(super) const THE_FUNC_TRANSFORM_NORMAL_VIEW: &str = concat!(
    "\nvec3 transformNormal (in vec3 theNormal)",
    "\n{",
    "\n  vec4 aResult = occWorldViewMatrixInverseTranspose",
    "\n               * occModelWorldMatrixInverseTranspose",
    "\n               * vec4 (theNormal, 0.0);",
    "\n  return normalize (aResult.xyz);",
    "\n}",
);

/// `THE_FUNC_transformNormal_world`: the normal from model to world space (PBR).
pub(super) const THE_FUNC_TRANSFORM_NORMAL_WORLD: &str = concat!(
    "\nvec3 transformNormal (in vec3 theNormal)",
    "\n{",
    "\n  vec4 aResult = occModelWorldMatrixInverseTranspose",
    "\n               * vec4 (theNormal, 0.0);",
    "\n  return normalize (aResult.xyz);",
    "\n}",
);

/// `THE_FUNC_lightDef`: the Phong light accumulators.
pub(super) const THE_FUNC_LIGHT_DEF: &str =
    concat!("\nvec3 Ambient;", "\nvec3 Diffuse;", "\nvec3 Specular;",);

/// `THE_FUNC_PBR_lightDef`: the PBR light accumulators and material values.
pub(super) const THE_FUNC_PBR_LIGHT_DEF: &str = concat!(
    "\nvec3  DirectLighting;",
    "\nvec4  BaseColor;",
    "\nfloat Metallic;",
    "\nfloat NormalizedRoughness;",
    "\nfloat Roughness;",
    "\nvec3  Emission;",
    "\nfloat IOR;",
);

/// `THE_FUNC_directionalLightFirst`: the directional light with index 0.
pub(super) const THE_FUNC_DIRECTIONAL_LIGHT_FIRST: &str = concat!(
    "\nvoid directionalLightFirst (in vec3 theNormal,",
    "\n                            in vec3 theView,",
    "\n                            in bool theIsFront,",
    "\n                            in float theShadow)",
    "\n{",
    "\n  vec3 aLight = occLight_Position (0);",
    "\n",
    "\n  vec3 aHalf = normalize (aLight + theView);",
    "\n",
    "\n  vec3  aFaceSideNormal = theIsFront ? theNormal : -theNormal;",
    "\n  float aNdotL = max (0.0, dot (aFaceSideNormal, aLight));",
    "\n  float aNdotH = max (0.0, dot (aFaceSideNormal, aHalf ));",
    "\n",
    "\n  float aSpecl = 0.0;",
    "\n  if (aNdotL > 0.0)",
    "\n  {",
    "\n    aSpecl = pow (aNdotH, occMaterial_Shininess(theIsFront));",
    "\n  }",
    "\n",
    "\n  Diffuse  += occLight_Diffuse(0)  * aNdotL * theShadow;",
    "\n  Specular += occLight_Specular(0) * aSpecl * theShadow;",
    "\n}",
);

/// `THE_FUNC_cubemap_vector_transform`: the cubemap fetch direction.
pub(super) const THE_FUNC_CUBEMAP_VECTOR_TRANSFORM: &str = concat!(
    "\nvec3 cubemapVectorTransform (in vec3 theVector,",
    "\n                             in int  theYCoeff,",
    "\n                             in int  theZCoeff)",
    "\n{",
    "\n  theVector = theVector.yzx;",
    "\n  theVector.y *= float(theYCoeff);",
    "\n  theVector.z *= float(theZCoeff);",
    "\n  return theVector;",
    "\n}",
);

/// `THE_FRAG_CLIP_PLANES_N`: N clipping planes in the fragment stage.
pub(super) const THE_FRAG_CLIP_PLANES_N: &str = concat!(
    "\n  for (int aPlaneIter = 0; aPlaneIter < occClipPlaneCount; ++aPlaneIter)",
    "\n  {",
    "\n    vec4 aClipEquation = occClipPlaneEquations[aPlaneIter];",
    "\n    if (dot (aClipEquation.xyz, PositionWorld.xyz / PositionWorld.w) + aClipEquation.w < 0.0)",
    "\n    {",
    "\n      discard;",
    "\n    }",
    "\n  }",
);

/// `THE_FRAG_CLIP_CHAINS_N`: chains of clipping planes in the fragment stage.
pub(super) const THE_FRAG_CLIP_CHAINS_N: &str = concat!(
    "\n  for (int aPlaneIter = 0; aPlaneIter < occClipPlaneCount;)",
    "\n  {",
    "\n    vec4 aClipEquation = occClipPlaneEquations[aPlaneIter];",
    "\n    if (dot (aClipEquation.xyz, PositionWorld.xyz / PositionWorld.w) + aClipEquation.w < 0.0)",
    "\n    {",
    "\n      if (occClipPlaneChains[aPlaneIter] == 1)",
    "\n      {",
    "\n        discard;",
    "\n      }",
    "\n      aPlaneIter += 1;",
    "\n    }",
    "\n    else",
    "\n    {",
    "\n      aPlaneIter += occClipPlaneChains[aPlaneIter];",
    "\n    }",
    "\n  }",
);

/// `THE_FRAG_CLIP_PLANES_1`: one clipping plane in the fragment stage.
pub(super) const THE_FRAG_CLIP_PLANES_1: &str = concat!(
    "\n  vec4 aClipEquation0 = occClipPlaneEquations[0];",
    "\n  if (dot (aClipEquation0.xyz, PositionWorld.xyz / PositionWorld.w) + aClipEquation0.w < 0.0)",
    "\n  {",
    "\n    discard;",
    "\n  }",
);

/// `THE_FRAG_CLIP_PLANES_2`: two clipping planes in the fragment stage.
pub(super) const THE_FRAG_CLIP_PLANES_2: &str = concat!(
    "\n  vec4 aClipEquation0 = occClipPlaneEquations[0];",
    "\n  vec4 aClipEquation1 = occClipPlaneEquations[1];",
    "\n  if (dot (aClipEquation0.xyz, PositionWorld.xyz / PositionWorld.w) + aClipEquation0.w < 0.0",
    "\n   || dot (aClipEquation1.xyz, PositionWorld.xyz / PositionWorld.w) + aClipEquation1.w < 0.0)",
    "\n  {",
    "\n    discard;",
    "\n  }",
);

/// `THE_FRAG_CLIP_CHAINS_2`: a chain of two clipping planes in the fragment stage.
pub(super) const THE_FRAG_CLIP_CHAINS_2: &str = concat!(
    "\n  vec4 aClipEquation0 = occClipPlaneEquations[0];",
    "\n  vec4 aClipEquation1 = occClipPlaneEquations[1];",
    "\n  if (dot (aClipEquation0.xyz, PositionWorld.xyz / PositionWorld.w) + aClipEquation0.w < 0.0",
    "\n   && dot (aClipEquation1.xyz, PositionWorld.xyz / PositionWorld.w) + aClipEquation1.w < 0.0)",
    "\n  {",
    "\n    discard;",
    "\n  }",
);

/// `THE_FRAG_WIREFRAME_COLOR`: the wireframe colour of mesh edges.
pub(super) const THE_FRAG_WIREFRAME_COLOR: &str = concat!(
    "\nvec4 getFinalColor(void)",
    "\n{",
    "\n  float aDistance = min (min (EdgeDistance[0], EdgeDistance[1]), EdgeDistance[2]);",
    "\n  bool isHollow = occWireframeColor.a < 0.0;",
    "\n  float aMixVal = smoothstep (occLineWidth - occLineFeather * 0.5, occLineWidth + occLineFeather * 0.5, aDistance);",
    "\n  vec4 aMixColor = isHollow",
    "\n                 ? vec4 (getColor().rgb, 1.0 - aMixVal)",
    "\n                 : mix (occWireframeColor, getColor(), aMixVal);",
    "\n  return aMixColor;",
    "\n}",
);

/// `THE_VERT_gl_Position`: the vertex stage output position.
pub(super) const THE_VERT_GL_POSITION: &str =
    "\n  gl_Position = occProjectionMatrix * occWorldViewMatrix * occModelWorldMatrix * occVertex;";

/// `THE_VERT_gl_Position_OUTLINE`: the position displaced along the normal for outlines.
pub(super) const THE_VERT_GL_POSITION_OUTLINE: &str = concat!(
    "\n  float anOutlineDisp = occOrthoScale > 0.0 ? occOrthoScale : gl_Position.w;",
    "\n  vec4  anOutlinePos  = occVertex + vec4 (occNormal * (occSilhouetteThickness * anOutlineDisp), 0.0);",
    "\n  gl_Position = occProjectionMatrix * occWorldViewMatrix * occModelWorldMatrix * anOutlinePos;",
);
