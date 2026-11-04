#version 450
#extension GL_GOOGLE_include_directive : require
// Graphic3dShaderManager::getBgCubeMapProgram vertex stage over the box of
// GlBackgroundArray::createCubeMapArray (8 corners, 36 indices), drawn without buffers.
// NDC stays in the Graphic3d (OpenGL) convention; the frame viewport orients it.
#include "environment_cube.glsl"
#include "clip_position.glsl"
layout(push_constant) uniform BackgroundCubeMapConstants {
  mat4 occProjectionWorldView; //!< the float product occProjectionMatrix * occWorldViewMatrix
  int uYCoeff;
  int uZCoeff;
} cube;
layout(location=0) out vec3 ViewDirection;
const vec3 THE_BOX_CORNERS[8] = vec3[] (
  vec3(-1.0, -1.0,  1.0), vec3( 1.0, -1.0,  1.0), vec3(-1.0,  1.0,  1.0), vec3( 1.0,  1.0,  1.0),
  vec3(-1.0, -1.0, -1.0), vec3( 1.0, -1.0, -1.0), vec3(-1.0,  1.0, -1.0), vec3( 1.0,  1.0, -1.0));
const int THE_BOX_TRIS[36] = int[] (
  0, 1, 2, 2, 1, 3, // top face
  1, 5, 7, 1, 7, 3, // right face
  0, 6, 4, 0, 2, 6, // left face
  4, 6, 5, 6, 7, 5, // bottom face
  0, 5, 1, 0, 4, 5, // front face
  2, 7, 6, 2, 3, 7  // back face
);

void main()
{
  vec4 occVertex = vec4 (THE_BOX_CORNERS[THE_BOX_TRIS[gl_VertexIndex]], 1.0);
  ViewDirection = cubemapVectorTransform (occVertex.xyz, cube.uYCoeff, cube.uZCoeff);
  vec4 aPos = clipPosition (cube.occProjectionWorldView, vec4(occVertex.xyz, 1.0));
  // setting Z to W ensures that final Z will be 1.0 after perspective division, (w/w=1))
  gl_Position = aPos.xyww;
}
