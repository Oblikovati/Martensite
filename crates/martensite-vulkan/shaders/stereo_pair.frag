#version 450
// Graphic3dShaderManager::getStdProgramStereo fragment stage, one binary per
// program: MARTENSITE_STEREO_ANAGLYPH (with MARTENSITE_STEREO_LINEAR_TARGET when the target is
// not sRGB: the sRgb2linear / linear2sRgb pow pair of a context without sRGB rendering),
// _ROW_INTERLACED, _COLUMN_INTERLACED, _CHESS_BOARD, _SIDE_BY_SIDE, _OVER_UNDER, and otherwise
// the default red/cyan program (QuadBuffer, SoftPageFlip, OpenVR).
layout(set=0,binding=0) uniform sampler2D uLeftSampler;
layout(set=0,binding=1) uniform sampler2D uRightSampler;
layout(location=0) in vec2 TexCoord;
layout(location=0) out vec4 occFragColor;
#ifdef MARTENSITE_STEREO_ANAGLYPH
layout(push_constant) uniform StereoConstants {
  mat4 uMultL;
  mat4 uMultR;
} stereo;
#else
// uTexOffset, the native window size (y up) and whether the frame output is flipped.
layout(push_constant) uniform StereoConstants {
  vec2 uTexOffset;
  vec2 uViewSize;
  int uFlip;
} stereo;
#endif
// The eye images keep the capture rows top first; OpenGL texture coordinates start at the bottom.
vec4 occTexture2D(sampler2D theSampler, vec2 theCoord) {
  return texture(theSampler, vec2(theCoord.x, 1.0 - theCoord.y));
}
#define occSetFragColor(theColor) occFragColor = theColor
#ifndef MARTENSITE_STEREO_ANAGLYPH
// The native gl_FragCoord (window y up) of this fragment: the quad spans the viewport.
vec2 nativeFragCoord() {
  vec2 aCoord = floor(TexCoord * stereo.uViewSize) + vec2(0.5);
  if (stereo.uFlip != 0) aCoord.y = stereo.uViewSize.y - aCoord.y;
  return aCoord;
}
#endif

#if defined(MARTENSITE_STEREO_ANAGLYPH)
#ifdef MARTENSITE_STEREO_LINEAR_TARGET
#define sRgb2linear(theColor) pow(theColor, vec4(2.2, 2.2, 2.2, 1.0))
#define linear2sRgb(theColor) pow(theColor, 1.0 / vec4(2.2, 2.2, 2.2, 1.0))
#else
#define sRgb2linear(theColor) theColor
#define linear2sRgb(theColor) theColor
#endif
// Mesa's GLSL lowering of a matrix-vector product: columns left to right (see clip_position).
vec4 filtered(mat4 theMult, vec4 theColor) {
  return ((theMult[0] * theColor.x + theMult[1] * theColor.y) + theMult[2] * theColor.z) + theMult[3] * theColor.w;
}
void main()
{
  vec4 aColorL = occTexture2D (uLeftSampler,  TexCoord);
  vec4 aColorR = occTexture2D (uRightSampler, TexCoord);
  aColorL = sRgb2linear (aColorL);
  aColorR = sRgb2linear (aColorR);
  vec4 aColor = filtered (stereo.uMultR, aColorR) + filtered (stereo.uMultL, aColorL);
  occSetFragColor (linear2sRgb (aColor));
}
#elif defined(MARTENSITE_STEREO_ROW_INTERLACED)
void main()
{
  vec2 aTexCoordL = TexCoord - stereo.uTexOffset;
  vec2 aTexCoordR = TexCoord + stereo.uTexOffset;
  vec4 aColorL = occTexture2D (uLeftSampler,  aTexCoordL);
  vec4 aColorR = occTexture2D (uRightSampler, aTexCoordR);
  if (int (mod (nativeFragCoord().y - 1023.5, 2.0)) != 1)
  {
    occSetFragColor (aColorL);
  }
  else
  {
    occSetFragColor (aColorR);
  }
}
#elif defined(MARTENSITE_STEREO_COLUMN_INTERLACED)
void main()
{
  vec2 aTexCoordL = TexCoord - stereo.uTexOffset;
  vec2 aTexCoordR = TexCoord + stereo.uTexOffset;
  vec4 aColorL = occTexture2D (uLeftSampler,  aTexCoordL);
  vec4 aColorR = occTexture2D (uRightSampler, aTexCoordR);
  if (int (mod (nativeFragCoord().x - 1023.5, 2.0)) == 1)
  {
    occSetFragColor (aColorL);
  }
  else
  {
    occSetFragColor (aColorR);
  }
}
#elif defined(MARTENSITE_STEREO_CHESS_BOARD)
void main()
{
  vec2 aTexCoordL = TexCoord - stereo.uTexOffset;
  vec2 aTexCoordR = TexCoord + stereo.uTexOffset;
  vec4 aColorL = occTexture2D (uLeftSampler,  aTexCoordL);
  vec4 aColorR = occTexture2D (uRightSampler, aTexCoordR);
  vec2 aFragCoord = nativeFragCoord();
  bool isEvenX = int(mod(floor(aFragCoord.x - 1023.5), 2.0)) != 1;
  bool isEvenY = int(mod(floor(aFragCoord.y - 1023.5), 2.0)) == 1;
  if ((isEvenX && isEvenY) || (!isEvenX && !isEvenY))
  {
    occSetFragColor (aColorL);
  }
  else
  {
    occSetFragColor (aColorR);
  }
}
#elif defined(MARTENSITE_STEREO_SIDE_BY_SIDE)
void main()
{
  vec2 aTexCoord = vec2 (TexCoord.x * 2.0, TexCoord.y);
  if (TexCoord.x > 0.5)
  {
    aTexCoord.x -= 1.0;
  }
  vec4 aColorL = occTexture2D (uLeftSampler,  aTexCoord);
  vec4 aColorR = occTexture2D (uRightSampler, aTexCoord);
  if (TexCoord.x <= 0.5)
  {
    occSetFragColor (aColorL);
  }
  else
  {
    occSetFragColor (aColorR);
  }
}
#elif defined(MARTENSITE_STEREO_OVER_UNDER)
void main()
{
  vec2 aTexCoord = vec2 (TexCoord.x, TexCoord.y * 2.0);
  if (TexCoord.y > 0.5)
  {
    aTexCoord.y -= 1.0;
  }
  vec4 aColorL = occTexture2D (uLeftSampler,  aTexCoord);
  vec4 aColorR = occTexture2D (uRightSampler, aTexCoord);
  if (TexCoord.y <= 0.5)
  {
    occSetFragColor (aColorL);
  }
  else
  {
    occSetFragColor (aColorR);
  }
}
#else
void main()
{
  vec4 aColorL = occTexture2D (uLeftSampler,  TexCoord);
  vec4 aColorR = occTexture2D (uRightSampler, TexCoord);
  aColorL.b = 0.0;
  aColorL.g = 0.0;
  aColorR.r = 0.0;
  occSetFragColor (aColorL + aColorR);
}
#endif
