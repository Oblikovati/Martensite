//! The GLSL text of the geometry stage, stipple lines and the lit programs.

/// The geometry stage viewport transform function.
pub(super) const GEOM_VIEWPORT_TRANSFORM: &str = concat!(
    "\nvec3 ViewPortTransform (vec4 theVec)",
    "\n{",
    "\n  vec3 aWinCoord = theVec.xyz / theVec.w;",
    "\n  aWinCoord    = aWinCoord * 0.5 + 0.5;",
    "\n  aWinCoord.xy = aWinCoord.xy * occViewport.zw + occViewport.xy;",
    "\n  return aWinCoord;",
    "\n}",
);

/// The geometry stage triangle heights for mesh edges.
pub(super) const GEOM_EDGE_HEIGHTS: &str = concat!(
    "\n  vec3 aSideA = ViewPortTransform (gl_in[2].gl_Position) - ViewPortTransform (gl_in[1].gl_Position);",
    "\n  vec3 aSideB = ViewPortTransform (gl_in[2].gl_Position) - ViewPortTransform (gl_in[0].gl_Position);",
    "\n  vec3 aSideC = ViewPortTransform (gl_in[1].gl_Position) - ViewPortTransform (gl_in[0].gl_Position);",
    "\n  float aQuadArea = abs (aSideB.x * aSideC.y - aSideB.y * aSideC.x);",
    "\n  vec3 aLenABC    = vec3 (length (aSideA), length (aSideB), length (aSideC));",
    "\n  vec3 aHeightABC = vec3 (aQuadArea) / aLenABC;",
    "\n  aHeightABC = max (aHeightABC, vec3 (10.0 * occLineWidth));",
    "\n  float aQuadModeHeightC = occIsQuadMode ? occLineWidth + 1.0 : 0.0;",
);

/// The stipple pattern axis selection.
pub(super) const STIPPLE_AXIS: &str = concat!(
    "\n  vec2 anAxis = vec2 (0.0, 1.0);",
    "\n  if (abs (dFdx (ScreenSpaceCoord.x)) - abs (dFdy (ScreenSpaceCoord.y)) > 0.001)",
    "\n  {",
    "\n    anAxis = vec2 (1.0, 0.0);",
    "\n  }",
    "\n  float aRotatePoint = dot (gl_FragCoord.xy, anAxis);",
);

/// The stipple pattern test with bitwise operations.
pub(super) const STIPPLE_BITWISE: &str = concat!(
    "\n  uint aBit = uint (floor (aRotatePoint / occStippleFactor + 0.5)) & 15U;",
    "\n  if ((uint (occStipplePattern) & (1U << aBit)) == 0U) discard;",
);

/// The stipple pattern test over a boolean array.
pub(super) const STIPPLE_ARRAY: &str = concat!(
    "\n  int aBit = int (mod (floor (aRotatePoint / occStippleFactor + 0.5), 16.0));",
    "\n  if (!occStipplePattern[aBit]) discard;",
);

/// The stipple fragment colour output.
pub(super) const STIPPLE_TAIL: &str = concat!(
    "\n  vec4 aColor = getFinalColor();",
    "\n  if (aColor.a <= 0.1) discard;",
    "\n  occSetFragColor (aColor);",
);

/// The stipple screen-space coordinate.
pub(super) const STIPPLE_VERT_END: &str = concat!(
    "\n  vec2 aPosition   = gl_Position.xy / gl_Position.w;",
    "\n  aPosition        = aPosition * 0.5 + 0.5;",
    "\n  ScreenSpaceCoord = aPosition.xy * occViewport.zw + occViewport.xy;",
);

/// The Phong computeLighting header.
pub(super) const PHONG_LIGHTING_HEAD: &str = concat!(
    "\n",
    "\nvec4 computeLighting (in vec3 theNormal,",
    "\n                      in vec3 theView,",
    "\n                      in vec4 thePoint,",
    "\n                      in bool theIsFront)",
    "\n{",
    "\n  Ambient  = occLightAmbient.rgb;",
    "\n  Diffuse  = vec3 (0.0);",
    "\n  Specular = vec3 (0.0);",
    "\n  vec3 aPoint = thePoint.xyz / thePoint.w;",
);

/// The Phong material combination.
pub(super) const PHONG_LIGHTING_MATERIAL: &str = concat!(
    "\n  vec3 aMatAmbient  = occMaterial_Ambient(theIsFront);",
    "\n  vec4 aMatDiffuse  = occMaterial_Diffuse(theIsFront);",
    "\n  vec3 aMatSpecular = occMaterial_Specular(theIsFront);",
    "\n  vec4 aColor = vec4(Ambient * aMatAmbient + Diffuse * aMatDiffuse.rgb + Specular * aMatSpecular, aMatDiffuse.a);",
);

/// The Phong base colour texture factor.
pub(super) const PHONG_LIGHTING_TEXTURE: &str = concat!(
    "\n#if defined(THE_HAS_TEXTURE_COLOR) && defined(FRAGMENT_SHADER)",
    "\n  aColor *= occTexture2D(occSamplerBaseColor, TexCoord.st / TexCoord.w);",
    "\n#endif",
);

/// The Phong occlusion and emission tail.
pub(super) const PHONG_LIGHTING_TAIL: &str = concat!(
    "\n  occMaterialOcclusion(aColor.rgb, TexCoord.st / TexCoord.w);",
    "\n  vec3 aMatEmission = occMaterialEmission(theIsFront, TexCoord.st / TexCoord.w);",
    "\n  aColor.rgb += aMatEmission.rgb;",
    "\n  return aColor;",
    "\n}",
);

/// The PBR computeLighting header up to the base colour.
pub(super) const PBR_LIGHTING_HEAD: &str = concat!(
    "\n",
    "\nvec4 computeLighting (in vec3 theNormal,",
    "\n                      in vec3 theView,",
    "\n                      in vec4 thePoint,",
    "\n                      in bool theIsFront)",
    "\n{",
    "\n  DirectLighting = vec3(0.0);",
    "\n  BaseColor           = occMaterialBaseColor(theIsFront, TexCoord.st / TexCoord.w)",
);

/// The PBR material values.
pub(super) const PBR_LIGHTING_MATERIAL: &str = concat!(
    "\n  Emission            = occMaterialEmission(theIsFront, TexCoord.st / TexCoord.w);",
    "\n  Metallic            = occMaterialMetallic(theIsFront, TexCoord.st / TexCoord.w);",
    "\n  NormalizedRoughness = occMaterialRoughness(theIsFront, TexCoord.st / TexCoord.w);",
    "\n  Roughness = occRoughness (NormalizedRoughness);",
    "\n  IOR       = occPBRMaterial_IOR (theIsFront);",
    "\n  vec3 aPoint = thePoint.xyz / thePoint.w;",
);

/// The PBR indirect lighting tail.
pub(super) const PBR_LIGHTING_TAIL: &str = concat!(
    "\n  vec3 aColor = DirectLighting;",
    "\n  vec3 anIndirectLightingSpec = occPBRFresnel (BaseColor.rgb, Metallic, IOR);",
    "\n  vec2 aCoeff = occTexture2D (occEnvLUT, vec2(abs(dot(theView, theNormal)), NormalizedRoughness)).xy;",
    "\n  anIndirectLightingSpec *= aCoeff.x;",
    "\n  anIndirectLightingSpec += aCoeff.y;",
    "\n  anIndirectLightingSpec *= occTextureCubeLod (occSpecIBLMap, -reflect (theView, theNormal), NormalizedRoughness * float (occNbSpecIBLLevels - 1)).rgb;",
    "\n  vec3 aRefractionCoeff = 1.0 - occPBRFresnel (BaseColor.rgb, Metallic, NormalizedRoughness, IOR, abs(dot(theView, theNormal)));",
    "\n  aRefractionCoeff *= (1.0 - Metallic);",
    "\n  vec3 anIndirectLightingDiff = aRefractionCoeff * BaseColor.rgb * BaseColor.a;",
    "\n  anIndirectLightingDiff *= occDiffIBLMap (theNormal).rgb;",
    "\n  aColor += occLightAmbient.rgb * (anIndirectLightingDiff + anIndirectLightingSpec);",
    "\n  aColor += Emission;",
    "\n  occMaterialOcclusion(aColor, TexCoord.st / TexCoord.w);",
    "\n  return vec4 (aColor, mix(1.0, BaseColor.a, aRefractionCoeff.x));",
    "\n}",
);

/// The Gouraud vertex main up to the side colours.
pub(super) const GOURAUD_VERT_MAIN: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  vec4 aPositionWorld = occModelWorldMatrix * occVertex;",
    "\n  vec3 aNormal        = transformNormal (occNormal);",
    "\n  vec3 aView;",
    "\n  if (occProjectionMatrix[3][3] == 1.0)",
    "\n  {",
    "\n    aView = (occWorldViewMatrixInverse * vec4(0.0, 0.0, 1.0, 0.0)).xyz;",
    "\n  }",
    "\n  else",
    "\n  {",
    "\n    vec3 anEye = (occWorldViewMatrixInverse * vec4(0.0, 0.0, 0.0, 1.0)).xyz;",
    "\n    aView = normalize (anEye - aPositionWorld.xyz);",
    "\n  }",
    "\n  FrontColor  = computeLighting (aNormal, aView, aPositionWorld, true);",
    "\n  BackColor   = computeLighting (aNormal, aView, aPositionWorld, false);",
);

/// The Phong normal map fragment code.
pub(super) const PHONG_NORMAL_MAP: &str = concat!(
    "\n#if defined(THE_HAS_TEXTURE_NORMAL)",
    "\n  vec2 aTexCoord = TexCoord.st / TexCoord.w;",
    "\n  vec4 aMapNormalValue = occTextureNormal(aTexCoord);",
    "\n  if (aMapNormalValue.w > 0.5)",
    "\n  {",
    "\n    mat2 aDeltaUVMatrix = mat2 (dFdx(aTexCoord), dFdy(aTexCoord));",
    "\n    mat2x3 aDeltaVectorMatrix = mat2x3 (dFdx (PositionWorld.xyz), dFdy (PositionWorld.xyz));",
    "\n    Normal = TangentSpaceNormal (aDeltaUVMatrix, aDeltaVectorMatrix, aMapNormalValue.xyz, Normal, !gl_FrontFacing);",
    "\n  }",
    "\n#endif",
);

/// The Phong light-space positions of shadow maps.
pub(super) const PHONG_SHADOW_VERT: &str = concat!(
    "\n  for (int aShadowIter = 0; aShadowIter < THE_NB_SHADOWMAPS; ++aShadowIter)",
    "\n  {",
    "\n    PosLightSpace[aShadowIter] = occShadowMapMatrices[aShadowIter] * PositionWorld;",
    "\n  }",
);

/// The Phong vertex main up to the view direction.
pub(super) const PHONG_VERT_MAIN: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  PositionWorld = occModelWorldMatrix * occVertex;",
    "\n  if (occProjectionMatrix[3][3] == 1.0)",
    "\n  {",
    "\n    View = (occWorldViewMatrixInverse * vec4(0.0, 0.0, 1.0, 0.0)).xyz;",
    "\n  }",
    "\n  else",
    "\n  {",
    "\n    vec3 anEye = (occWorldViewMatrixInverse * vec4(0.0, 0.0, 0.0, 1.0)).xyz;",
    "\n    View = normalize (anEye - PositionWorld.xyz);",
    "\n  }",
);
