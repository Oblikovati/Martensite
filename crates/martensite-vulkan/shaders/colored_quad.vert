#version 450
#extension GL_GOOGLE_include_directive : require
// Graphic3dShaderManager::getColoredQuadProgram vertex stage over the corner quad of
// GlBackgroundArray::createGradientArray (4 vertices, indices 0 1 3 1 2 3), drawn
// without buffers. NDC stays in the Graphic3d (OpenGL) convention; the frame viewport orients it.
#include "colored_quad.glsl"
#include "clip_position.glsl"
layout(location=0) out vec2 TexCoord;
const int THE_FS_QUAD_TRIS[6] = int[] (0, 1, 3, 1, 2, 3);
const vec2 THE_CORNER_UVS[4] = vec2[] (vec2(1.0, 0.0), vec2(1.0, 1.0), vec2(0.0, 1.0), vec2(0.0, 0.0));

void main()
{
  int aVertex = THE_FS_QUAD_TRIS[gl_VertexIndex];
  // aVertices of createGradientArray: (W, 0), (W, H), (0, H), (0, 0).
  vec2 aPos = vec2(aVertex < 2 ? quad.viewSize.x : 0.0, (aVertex == 1 || aVertex == 2) ? quad.viewSize.y : 0.0);
  // cyclically move highlighted corner depending on the gradient type
  TexCoord = THE_CORNER_UVS[(aVertex + quad.corner) % 4];
  gl_Position = clipPosition(quad.occProjection, vec4(aPos, 0.0, 1.0));
}
