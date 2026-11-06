// Push constants of Graphic3dShaderManager::getColoredQuadProgram: the
// GlBackgroundArray chart (occProjectionMatrix; world-view and model-world are identity),
// uColor1/uColor2 (QuantityColor::Rgb of the gradient colors), the view size of
// createGradientArray and the highlighted corner (fill method - AspectGradientFillMethod_Corner1).
layout(push_constant) uniform ColoredQuadConstants {
  mat4 occProjection;
  vec4 uColor1;
  vec4 uColor2;
  vec2 viewSize;
  int corner;
} quad;
