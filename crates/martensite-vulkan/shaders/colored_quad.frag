#version 450
#extension GL_GOOGLE_include_directive : require
// Graphic3dShaderManager::getColoredQuadProgram fragment stage.
#include "colored_quad.glsl"
layout(location=0) in vec2 TexCoord;
layout(location=0) out vec4 occFragColor;

// mix() is spelled `x + (y - x) * a`: Mesa's GLSL front end lowers the native mix that way,
// while SPIR-V OpFMix reaches llvmpipe as `x * (1 - a) + y * a` (one-level steps otherwise).
void main()
{
  vec3 c1 = quad.uColor1.rgb + (quad.uColor2.rgb - quad.uColor1.rgb) * TexCoord.x;
  occFragColor = vec4 (quad.uColor2.rgb + (c1 - quad.uColor2.rgb) * TexCoord.y, 1.0);
}
