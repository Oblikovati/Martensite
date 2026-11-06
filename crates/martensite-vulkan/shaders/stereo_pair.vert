#version 450
// Graphic3dShaderManager::getStdProgramStereo vertex stage over the quad of
// GlView::initBlitQuad, drawn as a 4-vertex strip without buffers. NDC stays in the
// Graphic3d (OpenGL) convention; the frame viewport orients it.
layout(location=0) out vec2 TexCoord;
const vec4 THE_BLIT_QUAD[4] = vec4[] (
  vec4( 1.0, -1.0, 1.0, 0.0), vec4( 1.0, 1.0, 1.0, 1.0),
  vec4(-1.0, -1.0, 0.0, 0.0), vec4(-1.0, 1.0, 0.0, 1.0));

void main()
{
  vec4 occVertex = THE_BLIT_QUAD[gl_VertexIndex];
  TexCoord    = occVertex.zw;
  gl_Position = vec4(occVertex.x, occVertex.y, 0.0, 1.0);
}
