//! The GLSL text of the utility and background programs.

/// The blit vertex stage.
pub(super) const BLIT_VERT: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  TexCoord    = occVertex.zw;",
    "\n  gl_Position = vec4(occVertex.x, occVertex.y, 0.0, 1.0);",
    "\n}",
);

/// The multisampled blit fragment main.
pub(super) const BLIT_MSAA_FRAG_MAIN: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  ivec2 aSize  = textureSize (uColorSampler);",
    "\n  ivec2 anUV   = ivec2 (vec2 (aSize) * TexCoord);",
    "\n  gl_FragDepth = texelFetch (uDepthSampler, anUV, THE_NUM_SAMPLES / 2 - 1).r;",
    "\n",
    "\n  vec4 aColor = vec4 (0.0);",
    "\n  for (int aSample = 0; aSample < THE_NUM_SAMPLES; ++aSample)",
    "\n  {",
    "\n    vec4 aVal = texelFetch (uColorSampler, anUV, aSample);",
    "\n    aColor += aVal;",
    "\n  }",
    "\n  aColor /= float(THE_NUM_SAMPLES);",
    "\n#ifdef THE_SHIFT_sRGB",
    "\n  aColor.rgb = pow (aColor.rgb, vec3 (1.0 / 2.2));",
    "\n#endif",
    "\n  occSetFragColor (aColor);",
    "\n}",
);

/// The single-sample blit fragment main.
pub(super) const BLIT_FRAG_MAIN: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  gl_FragDepth = occTexture2D (uDepthSampler, TexCoord).r;",
    "\n  vec4  aColor = occTexture2D (uColorSampler, TexCoord);",
    "\n#ifdef THE_SHIFT_sRGB",
    "\n  aColor.rgb = pow (aColor.rgb, vec3 (1.0 / 2.2));",
    "\n#endif",
    "\n  occSetFragColor (aColor);",
    "\n}",
);

/// The blit fragment without depth output.
pub(super) const BLIT_FRAG_NO_DEPTH: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  occSetFragColor (occTexture2D (uColorSampler, TexCoord));",
    "\n}",
);

/// The OIT compositing vertex stage.
pub(super) const OIT_VERT: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  TexCoord    = occVertex.zw;",
    "\n  gl_Position = vec4 (occVertex.x, occVertex.y, 0.0, 1.0);",
    "\n}",
);

/// The OIT compositing fragment stage.
pub(super) const OIT_FRAG: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  vec4 aAccum   = occTexture2D (uAccumTexture,  TexCoord);",
    "\n  float aWeight = occTexture2D (uWeightTexture, TexCoord).r;",
    "\n  occSetFragColor (vec4 (aAccum.rgb / max (aWeight, 0.00001), aAccum.a));",
    "\n}",
);

/// The multisampled OIT compositing fragment stage.
pub(super) const OIT_MSAA_FRAG: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  ivec2 aTexel  = ivec2 (vec2 (textureSize (uAccumTexture)) * TexCoord);",
    "\n  vec4 aAccum   = texelFetch (uAccumTexture,  aTexel, gl_SampleID);",
    "\n  float aWeight = texelFetch (uWeightTexture, aTexel, gl_SampleID).r;",
    "\n  occSetFragColor (vec4 (aAccum.rgb / max (aWeight, 0.00001), aAccum.a));",
    "\n}",
);

/// The depth peeling vertex stage.
pub(super) const PEELING_VERT: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  gl_Position = vec4 (occVertex.x, occVertex.y, 0.0, 1.0);",
    "\n}",
);

/// The depth peeling flush fragment after the sample id.
pub(super) const PEELING_FLUSH_FRAG_TAIL: &str = concat!(
    "\n  ivec2 aFragCoord  = ivec2 (gl_FragCoord.xy);",
    "\n  vec4  aFrontColor = texelFetch (uDepthPeelingFrontColor, aFragCoord, THE_SAMPLE_ID);",
    "\n  vec4  aBackColor  = texelFetch (uDepthPeelingBackColor,  aFragCoord, THE_SAMPLE_ID);",
    "\n  float anAlphaMult = 1.0 - aFrontColor.a;",
    "\n  occFragColor = vec4 (aFrontColor.rgb + anAlphaMult * aBackColor.rgb, aFrontColor.a + aBackColor.a);",
    "\n}",
);

/// The stereo vertex stage.
pub(super) const STEREO_VERT: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  TexCoord    = occVertex.zw;",
    "\n  gl_Position = vec4(occVertex.x, occVertex.y, 0.0, 1.0);",
    "\n}",
);

/// The anaglyph colour space without conversion.
pub(super) const STEREO_ANAGLYPH_LINEAR: &str = concat!(
    "\n#define sRgb2linear(theColor) theColor",
    "\n#define linear2sRgb(theColor) theColor",
);

/// The anaglyph sRGB conversion.
pub(super) const STEREO_ANAGLYPH_SRGB: &str = concat!(
    "\n#define sRgb2linear(theColor) pow(theColor, vec4(2.2, 2.2, 2.2, 1.0))",
    "\n#define linear2sRgb(theColor) pow(theColor, 1.0 / vec4(2.2, 2.2, 2.2, 1.0))",
);

/// The anaglyph fragment main.
pub(super) const STEREO_ANAGLYPH_MAIN: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  vec4 aColorL = occTexture2D (uLeftSampler,  TexCoord);",
    "\n  vec4 aColorR = occTexture2D (uRightSampler, TexCoord);",
    "\n  aColorL = sRgb2linear (aColorL);",
    "\n  aColorR = sRgb2linear (aColorR);",
    "\n  vec4 aColor = uMultR * aColorR + uMultL * aColorL;",
    "\n  occSetFragColor (linear2sRgb (aColor));",
    "\n}",
);

/// The row-interlaced fragment stage.
pub(super) const STEREO_ROW_FRAG: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  vec2 aTexCoordL = TexCoord - uTexOffset;",
    "\n  vec2 aTexCoordR = TexCoord + uTexOffset;",
    "\n  vec4 aColorL = occTexture2D (uLeftSampler,  aTexCoordL);",
    "\n  vec4 aColorR = occTexture2D (uRightSampler, aTexCoordR);",
    "\n  if (int (mod (gl_FragCoord.y - 1023.5, 2.0)) != 1)",
    "\n  {",
    "\n    occSetFragColor (aColorL);",
    "\n  }",
    "\n  else",
    "\n  {",
    "\n    occSetFragColor (aColorR);",
    "\n  }",
    "\n}",
);

/// The column-interlaced fragment stage.
pub(super) const STEREO_COLUMN_FRAG: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  vec2 aTexCoordL = TexCoord - uTexOffset;",
    "\n  vec2 aTexCoordR = TexCoord + uTexOffset;",
    "\n  vec4 aColorL = occTexture2D (uLeftSampler,  aTexCoordL);",
    "\n  vec4 aColorR = occTexture2D (uRightSampler, aTexCoordR);",
    "\n  if (int (mod (gl_FragCoord.x - 1023.5, 2.0)) == 1)",
    "\n  {",
    "\n    occSetFragColor (aColorL);",
    "\n  }",
    "\n  else",
    "\n  {",
    "\n    occSetFragColor (aColorR);",
    "\n  }",
    "\n}",
);

/// The chessboard fragment stage.
pub(super) const STEREO_CHESS_FRAG: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  vec2 aTexCoordL = TexCoord - uTexOffset;",
    "\n  vec2 aTexCoordR = TexCoord + uTexOffset;",
    "\n  vec4 aColorL = occTexture2D (uLeftSampler,  aTexCoordL);",
    "\n  vec4 aColorR = occTexture2D (uRightSampler, aTexCoordR);",
    "\n  bool isEvenX = int(mod(floor(gl_FragCoord.x - 1023.5), 2.0)) != 1;",
    "\n  bool isEvenY = int(mod(floor(gl_FragCoord.y - 1023.5), 2.0)) == 1;",
    "\n  if ((isEvenX && isEvenY) || (!isEvenX && !isEvenY))",
    "\n  {",
    "\n    occSetFragColor (aColorL);",
    "\n  }",
    "\n  else",
    "\n  {",
    "\n    occSetFragColor (aColorR);",
    "\n  }",
    "\n}",
);

/// The side-by-side fragment stage.
pub(super) const STEREO_SIDE_FRAG: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  vec2 aTexCoord = vec2 (TexCoord.x * 2.0, TexCoord.y);",
    "\n  if (TexCoord.x > 0.5)",
    "\n  {",
    "\n    aTexCoord.x -= 1.0;",
    "\n  }",
    "\n  vec4 aColorL = occTexture2D (uLeftSampler,  aTexCoord);",
    "\n  vec4 aColorR = occTexture2D (uRightSampler, aTexCoord);",
    "\n  if (TexCoord.x <= 0.5)",
    "\n  {",
    "\n    occSetFragColor (aColorL);",
    "\n  }",
    "\n  else",
    "\n  {",
    "\n    occSetFragColor (aColorR);",
    "\n  }",
    "\n}",
);

/// The over-under fragment stage.
pub(super) const STEREO_OVER_FRAG: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  vec2 aTexCoord = vec2 (TexCoord.x, TexCoord.y * 2.0);",
    "\n  if (TexCoord.y > 0.5)",
    "\n  {",
    "\n    aTexCoord.y -= 1.0;",
    "\n  }",
    "\n  vec4 aColorL = occTexture2D (uLeftSampler,  aTexCoord);",
    "\n  vec4 aColorR = occTexture2D (uRightSampler, aTexCoord);",
    "\n  if (TexCoord.y <= 0.5)",
    "\n  {",
    "\n    occSetFragColor (aColorL);",
    "\n  }",
    "\n  else",
    "\n  {",
    "\n    occSetFragColor (aColorR);",
    "\n  }",
    "\n}",
);

/// The default stereo fragment stage.
pub(super) const STEREO_DEFAULT_FRAG: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  vec4 aColorL = occTexture2D (uLeftSampler,  TexCoord);",
    "\n  vec4 aColorR = occTexture2D (uRightSampler, TexCoord);",
    "\n  aColorL.b = 0.0;",
    "\n  aColorL.g = 0.0;",
    "\n  aColorR.r = 0.0;",
    "\n  occSetFragColor (aColorL + aColorR);",
    "\n}",
);

/// The bounding box vertex stage.
pub(super) const BNDBOX_VERT: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  vec4 aCenter = vec4(occVertex.xyz * occBBoxSize + occBBoxCenter, 1.0);",
    "\n  vec4 aPos    = vec4(occVertex.xyz * occBBoxSize + occBBoxCenter, 1.0);",
    "\n  gl_Position = occProjectionMatrix * occWorldViewMatrix * occModelWorldMatrix * aPos;",
    "\n}",
);

/// The bounding box fragment stage.
pub(super) const BNDBOX_FRAG: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  occSetFragColor (occColor);",
    "\n}",
);

/// The background cubemap vertex main.
pub(super) const CUBEMAP_VERT_MAIN: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  ViewDirection = cubemapVectorTransform (occVertex.xyz, uYCoeff, uZCoeff);",
    "\n  vec4 aPos = occProjectionMatrix * occWorldViewMatrix * vec4(occVertex.xyz, 1.0);",
    "\n  gl_Position = aPos.xyww;",
    "\n}",
);

/// The background cubemap fragment up to the colour.
pub(super) const CUBEMAP_FRAG_HEAD: &str = concat!(
    "\n#define occEnvCubemap occSampler0",
    "\nvoid main()",
    "\n{",
    "\n  occSetFragColor (vec4(occTextureCube (occEnvCubemap, ViewDirection).rgb, 1.0));",
);

/// The skydome vertex stage.
pub(super) const SKYDOME_VERT: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  gl_Position = vec4 (occVertex.xy, 0.0, 1.0);",
    "\n  TexCoord    = 0.5 * gl_Position.xy + vec2 (0.5);",
    "\n}",
);

/// The coloured quad vertex stage.
pub(super) const QUAD_VERT: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  TexCoord    = occTexCoord.st;",
    "\n  gl_Position = occProjectionMatrix * occWorldViewMatrix * occModelWorldMatrix * occVertex;",
    "\n}",
);

/// The coloured quad fragment stage.
pub(super) const QUAD_FRAG: &str = concat!(
    "\nvoid main()",
    "\n{",
    "\n  vec3 c1 = mix (uColor1, uColor2, TexCoord.x);",
    "\n  occSetFragColor (vec4 (mix (uColor2, c1, TexCoord.y), 1.0));",
    "\n}",
);
