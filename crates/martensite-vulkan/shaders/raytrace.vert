#version 450
// the reference implementation resources/Shaders/RaytraceBase.vs over the two triangles of
// GlView::myRaytraceScreenQuad (glDrawArrays(GL_TRIANGLES, 0, 6)), without vertex buffers.
// NDC stays in the Graphic3d (OpenGL) convention; the frame viewport orients it.
const vec4 THE_SCREEN_QUAD[6] = vec4[] (
  vec4(-1.0, -1.0, 0.0, 1.0), vec4(-1.0,  1.0, 0.0, 1.0), vec4( 1.0,  1.0, 0.0, 1.0),
  vec4( 1.0,  1.0, 0.0, 1.0), vec4( 1.0, -1.0, 0.0, 1.0), vec4(-1.0, -1.0, 0.0, 1.0));

//! Normalized pixel coordinates.
layout(location=0) out vec2 vPixel;

void main (void)
{
  vec4 occVertex = THE_SCREEN_QUAD[gl_VertexIndex];
  vPixel = vec2 ((occVertex.x + 1.f) * 0.5f,
                 (occVertex.y + 1.f) * 0.5f);

  gl_Position = occVertex;
}
