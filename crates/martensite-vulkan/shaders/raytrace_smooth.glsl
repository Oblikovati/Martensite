// the reference implementation resources/Shaders/RaytraceSmooth.fs for Vulkan (see raytrace_base.glsl). The
// neighbourhood is fetched in attachment rows (occFsaaFetch): the luminance test is symmetric
// in the row offset, so mirrored rows decide alike; the equivalence and the rays use window rows.
//! Input ray-traced image.
layout(set=1, binding=0) uniform sampler2D uFSAAInputTexture;

//! Number of accumulated FSAA samples.
// uniform int uSamples: member of RaytraceSmoothConstants.

//! Output pixel color.
layout(location=0) out vec4 OutColor;

#define LUM_DIFFERENCE 0.085f

// =======================================================================
// function : main
// purpose  :
// =======================================================================
void main (void)
{
#ifndef PATH_TRACING

  int aPixelX = int (occFragCoord().x);
  int aPixelY = int (occFragCoord().y);

  // Adjust FLIPTRI pattern used for adaptive FSAA
  float anOffsetX = mix (uFsaaOffset.x, -uFsaaOffset.x, float (aPixelX % 2));
  float anOffsetY = mix (uFsaaOffset.y, -uFsaaOffset.y, float (aPixelY % 2));

  vec4 aClr0 = occFsaaFetch (0, 0);
  vec4 aClr1 = occFsaaFetch (0, -1);
  vec4 aClr2 = occFsaaFetch (0, 1);

  vec4 aClr3 = occFsaaFetch (1, 0);
  vec4 aClr4 = occFsaaFetch (1, -1);
  vec4 aClr5 = occFsaaFetch (1, 1);

  vec4 aClr6 = occFsaaFetch (-1, 0);
  vec4 aClr7 = occFsaaFetch (-1, -1);
  vec4 aClr8 = occFsaaFetch (-1, 1);

  float aLum = dot (LUMA, aClr0.xyz);

  bool aRender = abs (aClr1.w - aClr0.w) > LUM_DIFFERENCE ||
                 abs (aClr2.w - aClr0.w) > LUM_DIFFERENCE ||
                 abs (aClr3.w - aClr0.w) > LUM_DIFFERENCE ||
                 abs (aClr4.w - aClr0.w) > LUM_DIFFERENCE ||
                 abs (aClr5.w - aClr0.w) > LUM_DIFFERENCE ||
                 abs (aClr6.w - aClr0.w) > LUM_DIFFERENCE ||
                 abs (aClr7.w - aClr0.w) > LUM_DIFFERENCE ||
                 abs (aClr8.w - aClr0.w) > LUM_DIFFERENCE;

  if (!aRender)
  {
    aRender = abs (dot (LUMA, aClr1.xyz) - aLum) > LUM_DIFFERENCE ||
              abs (dot (LUMA, aClr2.xyz) - aLum) > LUM_DIFFERENCE ||
              abs (dot (LUMA, aClr3.xyz) - aLum) > LUM_DIFFERENCE ||
              abs (dot (LUMA, aClr4.xyz) - aLum) > LUM_DIFFERENCE ||
              abs (dot (LUMA, aClr5.xyz) - aLum) > LUM_DIFFERENCE ||
              abs (dot (LUMA, aClr6.xyz) - aLum) > LUM_DIFFERENCE ||
              abs (dot (LUMA, aClr7.xyz) - aLum) > LUM_DIFFERENCE ||
              abs (dot (LUMA, aClr8.xyz) - aLum) > LUM_DIFFERENCE;
  }

  vec4 aColor = aClr0;

  if (aRender)
  {
    SRay aRay = GenerateRay (vPixel + vec2 (anOffsetX, anOffsetY));

    vec3 aInvDirect = 1.f / max (abs (aRay.Direct), SMALL);

    aInvDirect = vec3 (aRay.Direct.x < 0.f ? -aInvDirect.x : aInvDirect.x,
                       aRay.Direct.y < 0.f ? -aInvDirect.y : aInvDirect.y,
                       aRay.Direct.z < 0.f ? -aInvDirect.z : aInvDirect.z);

    aColor = mix (aClr0, clamp (Radiance (aRay, aInvDirect), 0.0, 1.0), 1.0 / float(uSamples));
  }

  OutColor = aColor;

#endif
}
