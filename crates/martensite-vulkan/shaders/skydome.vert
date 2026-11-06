#version 450
// Graphic3dShaderManager::getBgSkydomeProgram vertex stage over the full-screen
// triangle of GlView::updateSkydomeBg. A positive viewport keeps the GL texel rows.
layout(location=0) out vec2 TexCoord;
const vec2 THE_SKYDOME_TRIANGLE[3] = vec2[] (vec2(-1.0, -1.0), vec2(3.0, -1.0), vec2(-1.0, 3.0));

void main()
{
  vec2 occVertex = THE_SKYDOME_TRIANGLE[gl_VertexIndex];
  gl_Position = vec4 (occVertex.xy, 0.0, 1.0);
  TexCoord    = 0.5 * gl_Position.xy + vec2 (0.5);
}
