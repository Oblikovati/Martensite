#version 450
#extension GL_EXT_demote_to_helper_invocation : require
#extension GL_KHR_shader_subgroup_quad : require
// Graphic3dShaderManager::getGridProgram fragment stage. The body is the native program;
// its uniforms come from one std430 block written by GlShaderGrid::SetUniforms order
// (crates/martensite-vulkan/src/shader_grid/uniforms.rs). OpenGL `discard` keeps the quad's derivatives
// for later fwidth calls; Vulkan `discard` terminates the invocation, so the implementation demotes.
// Top-left logical attachments (behavior version 2) rasterize a vertically mirrored image of
// the OpenGL window, so each 2x2 derivative quad is the mirror of the OpenGL quad (the frame
// already starts one attachment row down for odd heights, so the quads hold the same pixels).
// Mirroring the operand, deriving, and mirroring the result evaluates fwidth over the OpenGL
// lane order: the same pixels feed the same coarse/fine lanes as in the native driver.
layout(constant_id = 0) const bool kMirroredQuads = false;
float gridFwidth (float theValue)
{
  if (!kMirroredQuads) { return fwidth (theValue); }
  return subgroupQuadSwapVertical (fwidth (subgroupQuadSwapVertical (theValue)));
}
layout(location=0) in vec2 vNdc;
layout(location=0) out vec4 occFragColor;
layout(std430, set=0, binding=0) readonly buffer GridParams {
  mat4  occProjectionMatrix;
  mat4  occProjectionMatrixInverse;
  vec4  colorThickness;          // uColor, uThickness
  vec4  accentColorAngular;      // uAccentColor, uAccentAngularScale
  vec4  originScaleX;            // uPlaneOriginView, uScaleX
  vec4  refScaleY;               // uPlaneRefView, uScaleY
  vec4  xAngularScale;           // uPlaneXView, uAngularScale
  vec4  yAccentScaleX;           // uPlaneYView, uAccentScaleX
  vec4  nAccentScaleY;           // uPlaneNView, uAccentScaleY
  vec4  boundsTolerance;         // uBounds, uParallelTolerance
  vec4  localShifts;             // uLocalOriginShift, uAccentLocalOriginShift
  vec4  radialArc;               // uRadialOriginShift, uAccentRadialOriginShift, uArcRange
  ivec4 flags0;                  // uIsDrawAxis, uGridType, uDrawMode, uIsPerspective
  ivec4 flags1;                  // uIsZeroToOneDepth, uIsBackground, uIsBoundFade, uArcBounded
} grid;
#define occProjectionMatrix        grid.occProjectionMatrix
#define occProjectionMatrixInverse grid.occProjectionMatrixInverse
#define uColor                     grid.colorThickness.xyz
#define uThickness                 grid.colorThickness.w
#define uAccentColor               grid.accentColorAngular.xyz
#define uAccentAngularScale        grid.accentColorAngular.w
#define uPlaneOriginView           grid.originScaleX.xyz
#define uScaleX                    grid.originScaleX.w
#define uPlaneRefView              grid.refScaleY.xyz
#define uScaleY                    grid.refScaleY.w
#define uPlaneXView                grid.xAngularScale.xyz
#define uAngularScale              grid.xAngularScale.w
#define uPlaneYView                grid.yAccentScaleX.xyz
#define uAccentScaleX              grid.yAccentScaleX.w
#define uPlaneNView                grid.nAccentScaleY.xyz
#define uAccentScaleY              grid.nAccentScaleY.w
#define uBounds                    grid.boundsTolerance.xyz
#define uParallelTolerance         grid.boundsTolerance.w
#define uLocalOriginShift          grid.localShifts.xy
#define uAccentLocalOriginShift    grid.localShifts.zw
#define uRadialOriginShift         grid.radialArc.x
#define uAccentRadialOriginShift   grid.radialArc.y
#define uArcRange                  grid.radialArc.zw
#define uIsDrawAxis                grid.flags0.x
#define uGridType                  grid.flags0.y
#define uDrawMode                  grid.flags0.z
#define uIsPerspective             grid.flags0.w
#define uIsZeroToOneDepth          grid.flags1.x
#define uIsBackground              grid.flags1.y
#define uIsBoundFade               grid.flags1.z
#define uArcBounded                grid.flags1.w

float gridLine1d (float theCoord, float theShift, float theScale, float theThickness)
{
  float aCoord = (theCoord - theShift) * theScale;
  float aPixelWidth = gridFwidth (aCoord);
  if (!(aPixelWidth >= 0.0)) { return 0.0; }
  float aDist  = abs (fract (aCoord + 0.5) - 0.5);
  float aWidth = max (aPixelWidth, theThickness);
  if (aWidth == 0.0) { return 0.0; }
  return 1.0 - min (aDist / aWidth, 1.0);
}
float axisLine1d (float theCoord, float theThickness)
{
  float aWidth = max (gridFwidth (theCoord), theThickness);
  if (aWidth == 0.0) { return 0.0; }
  return 1.0 - min (abs (theCoord) / aWidth, 1.0);
}
vec4 gridLines2d (vec2 theUV, vec2 theAxisUV, vec3 theColor, vec2 theScale,
                  vec2 theShift, bool theIsDrawAxis, float theThickness)
{
  float aAlphaX = gridLine1d (theUV.x, theShift.x, theScale.x, theThickness);
  float aAlphaY = gridLine1d (theUV.y, theShift.y, theScale.y, theThickness);
  float aAlpha  = uDrawMode == 1 ? aAlphaX * aAlphaY : max (aAlphaX, aAlphaY);
  vec4  aColor  = vec4 (theColor, aAlpha);
  if (uIsDrawAxis != 0 && theIsDrawAxis && uDrawMode != 1)
  {
    float anAxisX = axisLine1d (theAxisUV.y, theThickness);
    float anAxisY = axisLine1d (theAxisUV.x, theThickness);
    if      (anAxisX > 0.0 && anAxisY > 0.0) { aColor = vec4 (0.0, 0.0, 1.0, max (aColor.a, max (anAxisX, anAxisY))); }
    else if (anAxisX > 0.0)                  { aColor = vec4 (1.0, 0.0, 0.0, max (aColor.a, anAxisX)); }
    else if (anAxisY > 0.0)                  { aColor = vec4 (0.0, 1.0, 0.0, max (aColor.a, anAxisY)); }
  }
  return aColor;
}
vec4 gridLines1d (float theCoord, float theShift, float theScale, vec3 theColor, float theThickness)
{
  return vec4 (theColor, gridLine1d (theCoord, theShift, theScale, theThickness));
}
vec4 overlayGrid (vec4 theBase, vec4 theAccent)
{
  return theAccent.a >= theBase.a ? theAccent : theBase;
}
// Column by column, as Mesa's GLSL front end lowers the native product; SPIR-V OpMatrixTimesVector
// is lowered last column first, which rounds the view point and so the line phase differently.
vec4 gridColumns (mat4 theMat, vec4 theVec)
{
  return ((theMat[0] * theVec.x + theMat[1] * theVec.y) + theMat[2] * theVec.z) + theMat[3] * theVec.w;
}
vec3 unprojectView (float theX, float theY, float theZ)
{
  vec4 aView = gridColumns (occProjectionMatrixInverse, vec4 (theX, theY, theZ, 1.0));
  return aView.xyz / aView.w;
}
float gridNdcNear()
{
  return uIsZeroToOneDepth != 0 ? 0.0 : -1.0;
}
float gridDepthFromNdc (float theNdcZ)
{
  return uIsZeroToOneDepth != 0 ? theNdcZ : (theNdcZ * 0.5 + 0.5);
}
float fragmentDepthFromView (vec3 theViewPnt)
{
  vec4 aClip = gridColumns (occProjectionMatrix, vec4 (theViewPnt, 1.0));
  float aNdcZ = aClip.z / aClip.w;
  return gridDepthFromNdc (aNdcZ);
}
bool intersectPlaneView (vec3 theNearPoint, vec3 theFarPoint, out vec3 theHit)
{
  vec3 aRayOrigin = uIsPerspective != 0 ? vec3 (0.0) : theNearPoint;
  vec3 aRayTarget = theFarPoint;
  vec3 aDir = aRayTarget - aRayOrigin;
  float aDirLen = length (aDir);
  if (aDirLen == 0.0) { return false; }
  float aDenomN = dot (uPlaneNView, aDir / aDirLen);
  if (abs (aDenomN) <= uParallelTolerance) { return false; }
  float aDenom = dot (uPlaneNView, aDir);
  float aT = dot (uPlaneNView, uPlaneOriginView - aRayOrigin) / aDenom;
  if (uIsBackground == 0 && uIsPerspective != 0 && aT < 0.0)
  {
    theHit = theNearPoint;
    return false;
  }
  theHit = aRayOrigin + aT * aDir;
  return true;
}

const float GRID_TWO_PI = 6.28318530718;
float gridNormalizeAngle (float theAngle)
{
  float anAngle = mod (theAngle, GRID_TWO_PI);
  if (anAngle < 0.0) { anAngle += GRID_TWO_PI; }
  return anAngle;
}
float gridPositiveAngleSpan (float theStart, float theEnd)
{
  float aSpan = mod (theEnd - theStart, GRID_TWO_PI);
  if (aSpan < 0.0) { aSpan += GRID_TWO_PI; }
  if (abs (aSpan) <= uParallelTolerance && abs (theEnd - theStart) >= GRID_TWO_PI - uParallelTolerance)
  {
    return GRID_TWO_PI;
  }
  return aSpan;
}
bool gridAngleInArc (float theAngle)
{
  float aSpan = gridPositiveAngleSpan (uArcRange.x, uArcRange.y);
  if (aSpan >= GRID_TWO_PI - uParallelTolerance) { return true; }
  float aDelta = gridNormalizeAngle (theAngle) - gridNormalizeAngle (uArcRange.x);
  if (aDelta < 0.0) { aDelta += GRID_TWO_PI; }
  return aDelta <= aSpan + uParallelTolerance;
}

void main()
{
  // Intersect in view space: the camera is the numerical origin, so zoom and
  // world-coordinate magnitude do not destabilize the line phase.
  vec3 aNearPoint = unprojectView (vNdc.x, vNdc.y, gridNdcNear());
  vec3 aFarPoint  = unprojectView (vNdc.x, vNdc.y, 1.0);
  vec3 aHit;
  if (!intersectPlaneView (aNearPoint, aFarPoint, aHit)) { demote; }
  float aFragDepth = fragmentDepthFromView (aHit);
  if (uIsBackground == 0)
  {
    if (aFragDepth < 0.0) { demote; }
    gl_FragDepth = min (aFragDepth, 1.0);
  }
  else
  {
    gl_FragDepth = 1.0;
  }
  vec3 aLocalAbs3  = aHit - uPlaneOriginView;
  vec2 aLocal      = vec2 (dot (aLocalAbs3, uPlaneXView), dot (aLocalAbs3, uPlaneYView));
  vec3 aLocalGrid3 = aHit - uPlaneRefView;
  vec2 aLocalGrid  = vec2 (dot (aLocalGrid3, uPlaneXView), dot (aLocalGrid3, uPlaneYView));
  // Bounded work area. Rectangular, radial and angular clipping can be mixed.
  float aR = length (aLocal);
  float aA = atan (aLocal.y, aLocal.x);
  float aBoundFade = 1.0;
  if (uBounds.z > 0.0)
  {
    float aFwBR = gridFwidth (aR);
    if (aR > uBounds.z) { demote; }
    if (uIsBoundFade != 0)
    {
      aBoundFade *= 1.0 - smoothstep (uBounds.z - aFwBR, uBounds.z, aR);
    }
  }
  if (uArcBounded != 0 && !gridAngleInArc (aA)) { demote; }
  if (uBounds.x > 0.0)
  {
    // Keep the bounded area geometrically strict; fwidth is used only for the
    // optional fade inside the boundary, never to expand the valid area.
    float aFwBX = gridFwidth (abs (aLocal.x));
    if (abs (aLocal.x) > uBounds.x) { demote; }
    if (uIsBoundFade != 0)
    {
      aBoundFade *= 1.0 - smoothstep (uBounds.x - aFwBX, uBounds.x, abs (aLocal.x));
    }
  }
  if (uBounds.y > 0.0)
  {
    float aFwBY = gridFwidth (abs (aLocal.y));
    if (abs (aLocal.y) > uBounds.y) { demote; }
    if (uIsBoundFade != 0)
    {
      aBoundFade *= 1.0 - smoothstep (uBounds.y - aFwBY, uBounds.y, abs (aLocal.y));
    }
  }
  // Grid coordinates depend on uGridType: 0=rectangular (X/Y), 1=circular (radius/angle).
  vec2 aGridUv;
  vec2 aScale;
  vec2 aAxisUv;
  if (uGridType == 1)
  {
    aGridUv = vec2 (aR, aA);
    aScale  = vec2 (uScaleX, uAngularScale);
    aAxisUv = aLocal;
  }
  else
  {
    aGridUv = aLocalGrid;
    aScale  = vec2 (uScaleX, uScaleY);
    aAxisUv = aLocal;
  }
  vec2 aGridShift = uGridType == 1 ? vec2 (uRadialOriginShift, 0.0) : vec2 (0.0);
  vec4 aColor = gridLines2d (aGridUv, aAxisUv, uColor, aScale, aGridShift, uGridType == 0, uThickness);
  if (uDrawMode != 1)
  {
    if (uGridType == 0)
    {
      if (uAccentScaleX > 0.0)
      {
        aColor = overlayGrid (aColor, gridLines1d (aLocal.x, uAccentLocalOriginShift.x, uAccentScaleX, uAccentColor, uThickness));
      }
      if (uAccentScaleY > 0.0)
      {
        aColor = overlayGrid (aColor, gridLines1d (aLocal.y, uAccentLocalOriginShift.y, uAccentScaleY, uAccentColor, uThickness));
      }
    }
    else
    {
      if (uAccentScaleX > 0.0)
      {
        aColor = overlayGrid (aColor, gridLines1d (aR, uAccentRadialOriginShift, uAccentScaleX, uAccentColor, uThickness));
      }
      if (uAccentAngularScale > 0.0)
      {
        aColor = overlayGrid (aColor, gridLines1d (aA, 0.0, uAccentAngularScale, uAccentColor, uThickness));
      }
    }
    // For circular grid, paint X/Y axis lines explicitly using plane-local coords.
    if (uIsDrawAxis != 0 && uGridType == 1)
    {
      float anAxisX = axisLine1d (aAxisUv.y, uThickness);
      float anAxisY = axisLine1d (aAxisUv.x, uThickness);
      if (anAxisX > 0.0 && anAxisY > 0.0)
      {
        aColor = vec4 (0.0, 0.0, 1.0, 1.0);
      }
      else if (anAxisX > 0.0)
      {
        aColor = vec4 (1.0, 0.0, 0.0, max (aColor.a, anAxisX));
      }
      else if (anAxisY > 0.0)
      {
        aColor = vec4 (0.0, 1.0, 0.0, max (aColor.a, anAxisY));
      }
    }
  }
  if (aColor.a == 0.0) { demote; }
  aColor.a *= aBoundFade;
  occFragColor = aColor;
}
