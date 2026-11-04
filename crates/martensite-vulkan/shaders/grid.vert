#version 450
// Graphic3dShaderManager::getGridProgram vertex stage: one clip-space triangle covering the
// viewport. NDC stays in the Graphic3d (OpenGL) convention; the frame viewport orients it.
layout(location=0) out vec2 vNdc;
const vec2 gridPlane[3] = vec2[] (vec2(-1.0, -1.0), vec2(3.0, -1.0), vec2(-1.0, 3.0));
void main()
{
  vec2 aVertex = gridPlane[gl_VertexIndex];
  vNdc        = aVertex;
  gl_Position = vec4 (aVertex, 0.0, 1.0);
}
